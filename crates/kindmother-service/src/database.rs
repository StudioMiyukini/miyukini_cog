//! Gestion des bases de données SQLite via KindMother Service.
//!
//! @id     kindmother_service_database
//! @do     manage_sqlite_databases_isolated
//@human ! @layer  core
//!
//! Ce module gère les bases de données SQLite de manière isolée.
//! KindMother Service est le seul processus autorisé à accéder aux fichiers DB (K-13).
//! La sécurité repose sur :
//! - Isolation de processus (seul KM accède aux fichiers)
//! - Contrôle d'accès via arbitrage
//! - **Chiffrement par défaut** (SQLCipher, feature `db-encryption` par défaut, K-06) :
//!   une base en clair est refusée ; le mode en clair n'existe qu'en build de debug
//!   avec `KINDMOTHER_DEV_PLAINTEXT=1` (fail-closed, ignoré en release)
//! - Une clé par base issue d'un secret aléatoire propre à la base (K-07,
//!   `kindmother_db_key::DatabaseKeyStore`)
//! - Écritures **typées** (`typed`) avec cycle de vie et journal d'audit chaîné hors des
//!   bases opérateurs (`audit`, K-08)
//! - Migrations de schéma versionnées et non destructives (C-41)
//! - Politique SQL appliquée par l'autorisateur SQLite (`sql_guard`, K-05) sur le
//!   chemin SQL brut **transitoire** ; les lectures passent par une connexion ouverte
//!   en `SQLITE_OPEN_READ_ONLY` avec `PRAGMA query_only = ON`, une seule instruction
//!   par requête.
//! - **Effacement physique** (oubli, droit à l'effacement) : `PRAGMA secure_delete = ON`
//!   sur **chaque** connexion ouverte par KindMother (bases opérateurs en lecture-écriture
//!   et en lecture seule, journal d'audit), relu après réglage ; s'il n'est pas relu actif,
//!   l'ouverture est refusée (fail-closed). SQLite met alors à zéro le contenu supprimé
//!   (cellules, pages de débordement, pages libres) au lieu de le laisser dans les pages
//!   libérées. En mode WAL, les anciennes images de page restent dans le `-wal` jusqu'au
//!   point de contrôle, d'où un `PRAGMA wal_checkpoint(TRUNCATE)` (pages remises à zéro
//!   recopiées dans le fichier principal, `-wal` ramené à zéro octet) :
//!   - après tout lot typé contenant une intention d'**effacement** explicite
//!     (`TypedWriteIntent::forget`, ou `.erasure()` sur `Update`/`Upsert` qui écrase une
//!     valeur à oublier) — **toujours**, même sans ligne touchée — avec une entrée d'audit
//!     `effacement` par intention (empreinte de la clé, jamais la valeur) ;
//!   - après tout lot typé dont un `Delete` ordinaire a supprimé des lignes (A-020) ;
//!   - sur le chemin SQL brut transitoire, après **toute** instruction ou transaction qui a
//!     modifié des lignes, d'après le compteur de SQLite (`total_changes`, déclencheurs et
//!     `REPLACE` compris) et non d'après le texte SQL ;
//!   - après toute migration de schéma appliquée.
//!
//!   Les `Insert`/`Update`/`Upsert` typés ordinaires n'en déclenchent pas (coût évité) :
//!   une mise à jour ordinaire n'est pas un oubli et ne promet pas la disparition de
//!   l'ancienne valeur ; qui veut l'oublier le dit par `.erasure()`.
//! - **Reprise des bases antérieures** (A-020) : les pages libérées avant l'activation de
//!   `secure_delete` gardent leur contenu. À la première ouverture en lecture-écriture par
//!   cette version, chaque base existante est réécrite par `VACUUM` (sous `secure_delete`)
//!   puis `wal_checkpoint(TRUNCATE)`, après contrôle de l'espace disque libre (taille de
//!   la base + `-wal` + marge) ; le marqueur `a020_secure_vacuum = 1` est alors posé dans
//!   `_kindmother_meta` (une seule fois par base) et l'opération est journalisée
//!   (`effacement` / `secure-vacuum`). Échec (VACUUM, point de contrôle incomplet,
//!   espace insuffisant, audit) ⇒ base refusée à l'ouverture, marqueur non posé,
//!   nouvelle tentative à l'ouverture suivante (fail-closed). Une base neuve reçoit le
//!   marqueur sans VACUUM.

use rusqlite::types::Value;
use rusqlite::{params_from_iter, Connection, OpenFlags, OptionalExtension, Statement};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tracing::{error, info, warn};

use kindmother::intent::{
    check_migrations, IntentLifecycle, IntentOutcome, IntentState, SchemaMigration, TypedSelect,
    TypedWriteIntent, MAX_INTENTS_PER_BATCH,
};

use crate::audit::{sha256_hex, AuditJournal, AuditKind, AuditRecord};
use crate::errors::ServiceError;
use crate::sql_guard::{map_prepare_error, prepare_single, AuthorizerGuard, SqlPolicy};
use crate::typed;

/// Nombre maximal d'opérations dans une transaction (C-13).
pub const MAX_TRANSACTION_OPS: usize = 1000;

/// Variable d'environnement autorisant des bases **en clair** (build de debug uniquement).
pub const ENV_DEV_PLAINTEXT: &str = "KINDMOTHER_DEV_PLAINTEXT";

/// En-tête d'un fichier SQLite non chiffré.
pub const SQLITE_PLAINTEXT_HEADER: &[u8; 16] = b"SQLite format 3\0";

/// Clé de `_kindmother_meta` portant la version du schéma de l'opérateur.
const APP_SCHEMA_VERSION_KEY: &str = "app_schema_version";

/// Sous-répertoire des sauvegardes avant migration.
const BACKUP_DIR: &str = "_backups";

/// Clé de `_kindmother_meta` : VACUUM de reprise A-020 fait (pages libérées avant
/// `secure_delete` réécrites). Valeur [`SECURE_VACUUM_DONE`].
pub const SECURE_VACUUM_KEY: &str = "a020_secure_vacuum";

/// Valeur du marqueur [`SECURE_VACUUM_KEY`].
pub const SECURE_VACUUM_DONE: &str = "1";

/// Marge d'espace libre exigée en plus de la taille de la base (et du `-wal`) pour le
/// VACUUM de reprise : en mode WAL, VACUUM écrit toute la base dans le `-wal` avant le
/// point de contrôle (la taille sur disque double temporairement).
pub const VACUUM_SPACE_MARGIN: u64 = 4 * 1024 * 1024;

/// Opérateur inscrit au journal pour les opérations internes de KindMother.
const SYSTEM_OPERATOR: &str = "kindmother";

/// Mode de stockage.
enum StorageMode {
    /// SQLCipher, une clé par base (défaut).
    #[cfg(feature = "db-encryption")]
    Encrypted(kindmother_db_key::DatabaseKeyStore),
    /// En clair : développement uniquement (debug + `KINDMOTHER_DEV_PLAINTEXT=1`).
    /// Jamais construit quand `db-encryption` est active.
    #[cfg_attr(feature = "db-encryption", allow(dead_code))]
    PlaintextDev,
}

/// Vrai si le fichier commence par l'en-tête SQLite en clair.
pub fn is_plaintext_sqlite(path: &Path) -> bool {
    use std::io::Read;
    let mut header = [0u8; 16];
    std::fs::File::open(path)
        .and_then(|mut f| f.read_exact(&mut header))
        .is_ok_and(|()| &header == SQLITE_PLAINTEXT_HEADER)
}

/// Mode en clair demandé et permis ? (debug uniquement ; en release, refus bruyant.)
#[cfg(not(feature = "db-encryption"))]
fn plaintext_dev_permitted() -> bool {
    let requested = std::env::var(ENV_DEV_PLAINTEXT).is_ok_and(|v| v == "1");
    if cfg!(debug_assertions) {
        if requested {
            warn!(
                "!!! KindMother: PLAINTEXT DATABASES ({ENV_DEV_PLAINTEXT}=1, debug build). \
                 DEVELOPMENT ONLY — never in production (K-06) !!!"
            );
        }
        requested
    } else {
        if requested {
            error!(
                "!!! KindMother: {ENV_DEV_PLAINTEXT}=1 is IGNORED in release builds: \
                 plaintext databases are refused (K-06) !!!"
            );
        }
        false
    }
}

/// Active l'effacement physique sur `conn` et le relit (fail-closed).
///
/// À appeler sur **chaque** connexion ouverte par KindMother, après la clé SQLCipher.
/// Erreur si la valeur relue n'est pas `1` (ON) : la connexion ne doit pas être utilisée.
pub(crate) fn enforce_secure_delete(conn: &Connection) -> Result<(), ServiceError> {
    conn.pragma_update(None, "secure_delete", "ON")
        .map_err(|e| ServiceError::Database(format!("Cannot enable secure_delete: {e}")))?;
    let active: i64 = conn
        .query_row("PRAGMA secure_delete", [], |r| r.get(0))
        .map_err(|e| ServiceError::Database(format!("Cannot read back secure_delete: {e}")))?;
    if active != 1 {
        error!("secure_delete read back as {active} (expected 1): connection refused");
        return Err(ServiceError::Database(format!(
            "secure_delete not active (read back {active}): connection refused"
        )));
    }
    Ok(())
}

/// Point de contrôle `TRUNCATE` après un effacement.
///
/// Recopie dans le fichier principal les pages remises à zéro par `secure_delete` et
/// ramène le `-wal` à zéro octet (les anciennes images de page qu'il contenait
/// disparaissent). Retourne `false` si le point de contrôle n'a pas pu être complet
/// (lecteur actif au-delà de `busy_timeout`, erreur) : l'effacement logique reste acquis,
/// le résidu sera éliminé au prochain effacement réussi.
fn truncate_wal_after_erasure(conn: &Connection, database: &str) -> bool {
    let result = conn.query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |r| {
        Ok((r.get::<_, i64>(0)?, r.get::<_, i64>(1)?, r.get::<_, i64>(2)?))
    });
    match result {
        Ok((0, _, _)) => true,
        Ok((busy, log, done)) => {
            error!(
                "Database '{database}': WAL checkpoint after erasure incomplete \
                 (busy={busy}, log={log}, checkpointed={done}): erased pages may remain in -wal"
            );
            false
        }
        Err(e) => {
            error!("Database '{database}': WAL checkpoint after erasure failed: {e}");
            false
        }
    }
}

/// Mention ajoutée au détail d'audit quand le point de contrôle d'effacement a échoué.
const WAL_RESIDUE_NOTE: &str = "; WAL checkpoint after erasure incomplete";

/// Motif rendu à l'opérateur quand un effacement typé a été appliqué mais que le point de
/// contrôle n'a pas pu être complet.
const ERASURE_RESIDUE_REASON: &str = "erasure applied but WAL checkpoint incomplete: \
     erased page images may remain in -wal until the next successful erasure";

/// Espace nécessaire au VACUUM de reprise.
pub fn vacuum_space_needed(db_len: u64, wal_len: u64) -> u64 {
    db_len.saturating_add(wal_len).saturating_add(VACUUM_SPACE_MARGIN)
}

/// Refus propre si l'espace libre ne suffit pas au VACUUM de reprise.
pub fn check_vacuum_space(database: &str, free: u64, needed: u64) -> Result<(), ServiceError> {
    if free < needed {
        error!(
            "Database '{database}': A-020 VACUUM needs {needed} free bytes, only {free} \
             available: database refused until space is freed"
        );
        return Err(ServiceError::QuotaExceeded(format!(
            "database '{database}' refused: A-020 secure VACUUM needs {needed} free bytes \
             in the data directory, {free} available — free disk space and reopen"
        )));
    }
    Ok(())
}

/// Espace libre (octets) pour un utilisateur non privilégié sur le système de fichiers
/// de `dir`.
#[cfg(unix)]
fn free_space(dir: &Path) -> Result<u64, ServiceError> {
    rustix::fs::statvfs(dir)
        .map(|s| s.f_bavail.saturating_mul(s.f_frsize))
        .map_err(|e| ServiceError::Internal(format!("Cannot read free disk space: {e}")))
}

#[cfg(not(unix))]
fn free_space(_dir: &Path) -> Result<u64, ServiceError> {
    warn!("Free disk space not checked on this platform before the A-020 VACUUM");
    Ok(u64::MAX)
}

fn file_len(path: &Path) -> u64 {
    std::fs::metadata(path).map(|m| m.len()).unwrap_or(0)
}

fn wal_file(db_path: &Path) -> PathBuf {
    PathBuf::from(format!("{}-wal", db_path.display()))
}

fn read_meta(conn: &Connection, key: &str) -> Result<Option<String>, ServiceError> {
    conn.query_row("SELECT value FROM _kindmother_meta WHERE key = ?1", [key], |r| r.get(0))
        .optional()
        .map_err(|e| ServiceError::Database(format!("Cannot read metadata '{key}': {e}")))
}

fn write_meta(conn: &Connection, key: &str, value: &str) -> Result<(), ServiceError> {
    conn.execute(
        "INSERT INTO _kindmother_meta (key, value, updated_at) VALUES (?1, ?2, datetime('now'))
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
        rusqlite::params![key, value],
    )
    .map(|_| ())
    .map_err(|e| ServiceError::Database(format!("Cannot write metadata '{key}': {e}")))
}

/// Résultat de l'application d'un lot : (lignes, rowid) par intention, ou (indice, motif).
type Applied = Result<Vec<(u64, Option<i64>)>, (usize, String)>;

/// Lignes JSON d'une lecture.
pub type Rows = Vec<HashMap<String, serde_json::Value>>;

/// Gestionnaire de bases de données isolées.
pub struct EncryptedDatabase {
    /// Répertoire racine des données
    data_dir: PathBuf,
    /// Chiffrement ou mode en clair de développement
    mode: StorageMode,
    /// Journal d'audit hors des bases opérateurs
    journal: AuditJournal,
    /// Cache des connexions ouvertes (protégé par mutex async)
    connections: Arc<RwLock<HashMap<String, Arc<Mutex<Connection>>>>>,
    /// Cache des connexions en lecture seule (chemin Query)
    ro_connections: Arc<RwLock<HashMap<String, Arc<Mutex<Connection>>>>>,
}

impl EncryptedDatabase {
    /// Crée un nouveau gestionnaire de bases de données.
    ///
    /// Fail-closed (K-06) : sans la feature `db-encryption`, refuse de démarrer sauf en
    /// build de debug avec `KINDMOTHER_DEV_PLAINTEXT=1`.
    pub fn new(data_dir: impl AsRef<Path>) -> Result<Self, ServiceError> {
        let data_dir = data_dir.as_ref().to_path_buf();
        std::fs::create_dir_all(&data_dir)
            .map_err(|e| ServiceError::Internal(format!("Cannot create data dir: {}", e)))?;

        // Définir des permissions restrictives sur le répertoire
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(0o700);
            std::fs::set_permissions(&data_dir, perms)
                .map_err(|e| ServiceError::Internal(format!("Cannot set permissions: {}", e)))?;
        }

        let mode = Self::storage_mode(&data_dir)?;
        let journal_key: Option<zeroize::Zeroizing<String>> = match &mode {
            #[cfg(feature = "db-encryption")]
            StorageMode::Encrypted(store) => {
                let exists = data_dir
                    .join(crate::audit::AUDIT_DIR)
                    .join(crate::audit::AUDIT_FILE)
                    .exists();
                Some(
                    store
                        .pragma_key(crate::audit::AUDIT_KEY_NAME, exists)
                        .map_err(|e| ServiceError::Encryption(e.0))?
                        .0,
                )
            }
            StorageMode::PlaintextDev => None,
        };
        let journal = AuditJournal::open(&data_dir, journal_key.as_deref().map(String::as_str))?;
        info!(
            "Audit journal ready ({} entries, chain verified) at {:?}",
            journal.verify()?,
            journal.path()
        );

        Ok(Self {
            data_dir,
            mode,
            journal,
            connections: Arc::new(RwLock::new(HashMap::new())),
            ro_connections: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    #[cfg(feature = "db-encryption")]
    fn storage_mode(data_dir: &Path) -> Result<StorageMode, ServiceError> {
        if std::env::var(ENV_DEV_PLAINTEXT).is_ok_and(|v| v == "1") {
            warn!("{ENV_DEV_PLAINTEXT}=1 ignored: this build encrypts databases (db-encryption)");
        }
        let store = kindmother_db_key::DatabaseKeyStore::open(data_dir)
            .map_err(|e| ServiceError::Encryption(e.0))?;
        Ok(StorageMode::Encrypted(store))
    }

    #[cfg(not(feature = "db-encryption"))]
    fn storage_mode(_data_dir: &Path) -> Result<StorageMode, ServiceError> {
        if plaintext_dev_permitted() {
            Ok(StorageMode::PlaintextDev)
        } else {
            error!(
                "KindMother built without db-encryption: refusing to start with plaintext \
                 databases (K-06). Build with the default features, or for development only \
                 use a debug build with {ENV_DEV_PLAINTEXT}=1."
            );
            Err(ServiceError::Encryption(
                "plaintext storage refused (build without db-encryption)".to_string(),
            ))
        }
    }

    /// Vrai si les bases sont chiffrées.
    pub fn is_encrypted(&self) -> bool {
        !matches!(self.mode, StorageMode::PlaintextDev)
    }

    /// Journal d'audit.
    pub fn journal(&self) -> &AuditJournal {
        &self.journal
    }

    /// Chemin du fichier d'une base.
    pub fn database_path(&self, database: &str) -> PathBuf {
        self.data_dir.join(format!("{}.db", database))
    }

    /// Applique la clé (mode chiffré) et vérifie qu'elle ouvre la base.
    fn apply_key(&self, conn: &Connection, database: &str, existed: bool) -> Result<(), ServiceError> {
        match &self.mode {
            #[cfg(feature = "db-encryption")]
            StorageMode::Encrypted(store) => {
                if existed && is_plaintext_sqlite(&self.database_path(database)) {
                    error!(
                        "Database '{database}' is a PLAINTEXT SQLite file: refused (K-06). \
                         Import it into an encrypted database."
                    );
                    return Err(ServiceError::Encryption(format!(
                        "plaintext database '{database}' refused"
                    )));
                }
                let (key, scheme) = store
                    .pragma_key(database, existed)
                    .map_err(|e| ServiceError::Encryption(e.0))?;
                if scheme == kindmother_db_key::KeyScheme::V1Legacy {
                    warn!("Database '{database}' uses the legacy v1 key (rekey to v2 pending, K-07)");
                }
                conn.pragma_update(None, "key", key.as_str())
                    .map_err(|e| ServiceError::Encryption(format!("Cannot set encryption key: {e}")))?;
            }
            StorageMode::PlaintextDev => {
                let _ = (database, existed);
            }
        }
        conn.query_row("SELECT count(*) FROM sqlite_master", [], |r| r.get::<_, i64>(0))
            .map_err(|e| ServiceError::Encryption(format!("Cannot read database '{database}' (wrong key?): {e}")))?;
        Ok(())
    }

    /// Ouvre ou récupère une connexion à une base de données.
    pub async fn get_connection(
        &self,
        database: &str,
    ) -> Result<Arc<Mutex<Connection>>, ServiceError> {
        {
            let connections = self.connections.read().await;
            if let Some(conn) = connections.get(database) {
                return Ok(conn.clone());
            }
        }

        let conn = self.open_database(database)?;
        let conn = Arc::new(Mutex::new(conn));

        let mut connections = self.connections.write().await;
        Ok(connections
            .entry(database.to_string())
            .or_insert(conn)
            .clone())
    }

    /// Ouvre ou récupère une connexion en lecture seule à une base de données.
    ///
    /// La base est d'abord ouverte en lecture-écriture (création et tables système),
    /// puis une seconde connexion est ouverte avec `SQLITE_OPEN_READ_ONLY`.
    pub async fn get_ro_connection(
        &self,
        database: &str,
    ) -> Result<Arc<Mutex<Connection>>, ServiceError> {
        {
            let connections = self.ro_connections.read().await;
            if let Some(conn) = connections.get(database) {
                return Ok(conn.clone());
            }
        }

        // Garantit l'existence du fichier et du schéma système.
        self.get_connection(database).await?;

        let conn = self.open_database_read_only(database)?;
        let conn = Arc::new(Mutex::new(conn));
        let mut connections = self.ro_connections.write().await;
        Ok(connections
            .entry(database.to_string())
            .or_insert(conn)
            .clone())
    }

    /// Ouvre une base existante en lecture seule.
    fn open_database_read_only(&self, database: &str) -> Result<Connection, ServiceError> {
        if !Self::is_valid_database_name(database) {
            return Err(ServiceError::Validation(format!(
                "Invalid database name: {}",
                database
            )));
        }
        let db_path = self.database_path(database);

        let conn = Connection::open_with_flags(
            &db_path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .map_err(|e| ServiceError::Database(format!("Cannot open database read-only: {}", e)))?;

        self.apply_key(&conn, database, true)?;
        enforce_secure_delete(&conn)?;

        conn.execute_batch(
            r#"
            PRAGMA query_only = ON;
            PRAGMA busy_timeout = 5000;
            "#,
        )
        .map_err(|e| ServiceError::Database(format!("Cannot configure database: {}", e)))?;

        Ok(conn)
    }

    /// Ouvre une base de données.
    fn open_database(&self, database: &str) -> Result<Connection, ServiceError> {
        if !Self::is_valid_database_name(database) {
            return Err(ServiceError::Validation(format!(
                "Invalid database name: {}",
                database
            )));
        }

        let db_path = self.database_path(database);
        // Base antérieure seulement si non vide : un fichier vide vient d'être
        // créé (par cette ouverture ou une autre concurrente) et ne doit jamais
        // déclencher le repli sur la clé v1.
        let existed = std::fs::metadata(&db_path).is_ok_and(|m| m.len() > 0);

        let conn = Connection::open_with_flags(
            &db_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .map_err(|e| ServiceError::Database(format!("Cannot open database: {}", e)))?;

        self.apply_key(&conn, database, existed)?;
        enforce_secure_delete(&conn)?;

        conn.execute_batch(
            r#"
            PRAGMA foreign_keys = ON;
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            PRAGMA busy_timeout = 5000;
            "#,
        )
        .map_err(|e| ServiceError::Database(format!("Cannot configure database: {}", e)))?;

        self.ensure_system_tables(&conn)?;
        self.ensure_secure_vacuum(&conn, database, existed)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(0o600);
            let _ = std::fs::set_permissions(&db_path, perms);
        }

        Ok(conn)
    }

    /// Valide un nom de base de données : `[a-z0-9_-]`, 1 à 64 caractères, sans `-`/`_`
    /// initial. Minuscules seulement : `Foo` et `foo` ne doivent pas désigner deux bases
    /// distinctes qui se confondraient sur un système de fichiers insensible à la casse.
    fn is_valid_database_name(name: &str) -> bool {
        !name.is_empty()
            && name.len() <= 64
            && name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
            && !name.starts_with('-')
            && !name.starts_with('_')
    }

    /// Assure que les tables système existent.
    ///
    /// L'audit n'est plus dans la base de l'opérateur (K-08) : `_kindmother_audit`
    /// n'est plus créée ni écrite (une table héritée reste protégée par l'autorisateur).
    fn ensure_system_tables(&self, conn: &Connection) -> Result<(), ServiceError> {
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS _kindmother_meta (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            INSERT OR IGNORE INTO _kindmother_meta (key, value)
            VALUES ('schema_version', '1');
            "#,
        )
        .map_err(|e| ServiceError::Database(format!("Cannot create system tables: {}", e)))?;

        Ok(())
    }

    /// Reprise A-020 : VACUUM unique d'une base antérieure (voir l'en-tête du module).
    fn ensure_secure_vacuum(
        &self,
        conn: &Connection,
        database: &str,
        existed: bool,
    ) -> Result<(), ServiceError> {
        if read_meta(conn, SECURE_VACUUM_KEY)?.as_deref() == Some(SECURE_VACUUM_DONE) {
            return Ok(());
        }
        if !existed {
            // Base neuve : elle n'a jamais été écrite sans `secure_delete`.
            return write_meta(conn, SECURE_VACUUM_KEY, SECURE_VACUUM_DONE);
        }
        let result = self.secure_vacuum(conn, database);
        let (verdict, detail) = match &result {
            Ok(detail) => ("appliquee", detail.clone()),
            Err(e) => (refusal_or_failure(e), e.to_string()),
        };
        // Reprise non journalisée = reprise non faite (marqueur non posé).
        self.journal.append(&AuditRecord {
            kind: AuditKind::Effacement,
            intent_id: None,
            operator_id: SYSTEM_OPERATOR.to_string(),
            database: database.to_string(),
            operation: "secure-vacuum".to_string(),
            table: None,
            verdict: verdict.to_string(),
            decision_digest: None,
            params_hash: sha256_hex(SECURE_VACUUM_KEY.as_bytes()),
            detail,
        })?;
        result?;
        write_meta(conn, SECURE_VACUUM_KEY, SECURE_VACUUM_DONE)?;
        // Le marqueur seul est dans le `-wal` : point de contrôle de confort.
        let _ = truncate_wal_after_erasure(conn, database);
        Ok(())
    }

    /// VACUUM sous `secure_delete` puis `wal_checkpoint(TRUNCATE)` complet, après contrôle
    /// de l'espace libre. Retourne le détail d'audit.
    fn secure_vacuum(&self, conn: &Connection, database: &str) -> Result<String, ServiceError> {
        let db_path = self.database_path(database);
        let before = file_len(&db_path);
        let needed = vacuum_space_needed(before, file_len(&wal_file(&db_path)));
        check_vacuum_space(database, free_space(&self.data_dir)?, needed)?;
        warn!("Database '{database}': A-020 secure VACUUM of a pre-existing database ({before} bytes)");
        let started = std::time::Instant::now();
        conn.execute_batch("VACUUM").map_err(|e| {
            error!("Database '{database}': A-020 VACUUM failed: {e}");
            ServiceError::Database(format!(
                "database '{database}' refused: A-020 secure VACUUM failed ({e}); \
                 retried at next open"
            ))
        })?;
        if !truncate_wal_after_erasure(conn, database) {
            return Err(ServiceError::Database(format!(
                "database '{database}' refused: WAL checkpoint after the A-020 VACUUM is \
                 incomplete (active reader?); retried at next open"
            )));
        }
        let detail = format!(
            "a020 vacuum: {before} -> {} bytes in {} ms",
            file_len(&db_path),
            started.elapsed().as_millis()
        );
        info!("Database '{database}': {detail}");
        Ok(detail)
    }

    // ------------------------------------------------------------------
    // Chemin SQL brut — TRANSITOIRE (lot C) : à retirer quand tous les services
    // seront passés aux intentions typées, lectures typées et migrations.
    // ------------------------------------------------------------------

    /// Exécute une requête en lecture seule (une seule instruction). **Transitoire.**
    ///
    /// Trois barrières : connexion `SQLITE_OPEN_READ_ONLY` + `query_only`, autorisateur
    /// en politique lecture seule, et `Statement::readonly()`. Lecture journalisée.
    pub async fn query(
        &self,
        database: &str,
        operator_id: &str,
        sql: &str,
        params: Vec<String>,
    ) -> Result<Rows, ServiceError> {
        let params_hash = raw_params_hash(sql, &params);
        let result = self.query_inner(database, sql, params).await;
        let (verdict, detail) = match &result {
            Ok(rows) => ("autorisee", format!("{} rows", rows.len())),
            Err(e) => (refusal_or_failure(e), e.to_string()),
        };
        // Lecture non journalisée = lecture refusée (fail-closed).
        self.journal.append(&AuditRecord {
            kind: AuditKind::RawRead,
            intent_id: None,
            operator_id: operator_id.to_string(),
            database: database.to_string(),
            operation: "query".to_string(),
            table: None,
            verdict: verdict.to_string(),
            decision_digest: None,
            params_hash,
            detail,
        })?;
        result
    }

    async fn query_inner(
        &self,
        database: &str,
        sql: &str,
        params: Vec<String>,
    ) -> Result<Rows, ServiceError> {
        let conn = self.get_ro_connection(database).await?;
        let conn = conn.lock().await;

        let _guard = AuthorizerGuard::install(&conn, SqlPolicy::READ_ONLY);
        let mut stmt = prepare_single(&conn, sql)?;
        if !stmt.readonly() {
            return Err(ServiceError::PermissionDenied(
                "Query path accepts read-only statements only".to_string(),
            ));
        }
        let values: Vec<Value> = params.into_iter().map(Value::Text).collect();
        collect_rows(&mut stmt, values)
    }

    /// Exécute une instruction d'écriture sous la politique `policy`. **Transitoire.**
    pub async fn execute(
        &self,
        database: &str,
        operator_id: &str,
        policy: SqlPolicy,
        sql: &str,
        params: Vec<String>,
        intent: &str,
    ) -> Result<(u64, Option<i64>), ServiceError> {
        let params_hash = raw_params_hash(sql, &params);
        let mut wal_clean = true;
        let result = async {
            let conn = self.get_connection(database).await?;
            let conn = conn.lock().await;
            let changes_before = conn.total_changes();
            let rows_affected = {
                let _guard = AuthorizerGuard::install(&conn, policy);
                let mut stmt = prepare_single(&conn, sql)?;
                stmt.execute(params_from_iter(params))
                    .map_err(|e| ServiceError::Database(format!("Execute failed: {}", e)))?
            };
            let last_id = conn.last_insert_rowid();
            // Décision prise par SQLite (lignes modifiées, déclencheurs et REPLACE
            // compris), jamais par le texte SQL.
            if conn.total_changes() != changes_before {
                wal_clean = truncate_wal_after_erasure(&conn, database);
            }
            Ok::<_, ServiceError>((rows_affected as u64, Some(last_id)))
        }
        .await;
        let note = if wal_clean { "" } else { WAL_RESIDUE_NOTE };
        self.audit_raw_write(database, operator_id, "execute", intent, params_hash, &result.as_ref().map(|r| (r.0, note)))?;
        result
    }

    /// Exécute une transaction (chaque opération : une instruction, sous `policy`). **Transitoire.**
    pub async fn transaction(
        &self,
        database: &str,
        operator_id: &str,
        policy: SqlPolicy,
        operations: Vec<(String, Vec<String>)>,
        intent: &str,
    ) -> Result<Vec<u64>, ServiceError> {
        if operations.len() > MAX_TRANSACTION_OPS {
            return Err(ServiceError::Validation(format!(
                "Too many operations in transaction (max {MAX_TRANSACTION_OPS})"
            )));
        }
        let mut canonical = Vec::new();
        for (sql, params) in &operations {
            canonical.extend_from_slice(raw_params_hash(sql, params).as_bytes());
        }
        let params_hash = sha256_hex(&canonical);

        let mut wal_clean = true;
        let result = async {
            let conn = self.get_connection(database).await?;
            let mut conn = conn.lock().await;
            let changes_before = conn.total_changes();
            let tx = conn
                .transaction()
                .map_err(|e| ServiceError::Database(format!("Cannot begin transaction: {}", e)))?;
            let mut results = Vec::new();
            for (sql, params) in operations {
                // L'autorisateur n'est actif que pendant l'opération de l'opérateur
                // (BEGIN / COMMIT sont émis par le service). Rollback via drop de tx.
                let _guard = AuthorizerGuard::install(&tx, policy);
                let mut stmt = prepare_single(&tx, &sql)?;
                let rows = stmt
                    .execute(params_from_iter(params))
                    .map_err(|e| ServiceError::Database(format!("Transaction failed: {}", e)))?;
                results.push(rows as u64);
            }
            tx.commit()
                .map_err(|e| ServiceError::Database(format!("Cannot commit: {}", e)))?;
            if conn.total_changes() != changes_before {
                wal_clean = truncate_wal_after_erasure(&conn, database);
            }
            Ok::<_, ServiceError>(results)
        }
        .await;
        let note = if wal_clean { "" } else { WAL_RESIDUE_NOTE };
        let total = result.as_ref().map(|r| (r.iter().sum::<u64>(), note));
        self.audit_raw_write(database, operator_id, "transaction", intent, params_hash, &total)?;
        result
    }

    fn audit_raw_write(
        &self,
        database: &str,
        operator_id: &str,
        operation: &str,
        intent: &str,
        params_hash: String,
        result: &Result<(u64, &str), &ServiceError>,
    ) -> Result<(), ServiceError> {
        let (verdict, detail) = match result {
            Ok((rows, note)) => ("appliquee", format!("{intent}: {rows} rows{note}")),
            Err(e) => (refusal_or_failure(e), format!("{intent}: {e}")),
        };
        self.journal
            .append(&AuditRecord {
                kind: AuditKind::RawWrite,
                intent_id: None,
                operator_id: operator_id.to_string(),
                database: database.to_string(),
                operation: operation.to_string(),
                table: None,
                verdict: verdict.to_string(),
                decision_digest: None,
                params_hash,
                detail,
            })
            .map(|_| ())
            .inspect_err(|e| error!("AUDIT FAILURE after raw write on '{database}': {e}"))
    }

    // ------------------------------------------------------------------
    // Chemin typé (lot C)
    // ------------------------------------------------------------------

    /// Soumet un lot d'intentions typées, appliqué atomiquement.
    ///
    /// Cycle de vie de chaque intention : `Créée → Validée → Appliquée → Archivée`,
    /// ou `Refusée`/`Échouée → Archivée`. Une intention refusée ou échouée annule tout
    /// le lot. L'arbitrage (base, table, mode) est fait par l'appelant ;
    /// `refusals` porte ses refus éventuels (motif par intention).
    pub async fn submit_intents(
        &self,
        database: &str,
        operator_id: &str,
        intents: Vec<TypedWriteIntent>,
        refusals: Vec<Option<String>>,
    ) -> Vec<IntentOutcome> {
        let mut lifecycles: Vec<IntentLifecycle> =
            intents.iter().map(|_| IntentLifecycle::new()).collect();
        let mut reasons: Vec<Option<String>> = refusals;
        reasons.resize(intents.len(), None);

        if intents.is_empty() || intents.len() > MAX_INTENTS_PER_BATCH {
            return Vec::new();
        }
        // Refus d'arbitrage : aucune base n'est ouverte (ni créée) pour un refus.
        if reasons.iter().any(Option::is_some) {
            return self.finish_refused(database, operator_id, &intents, lifecycles, reasons);
        }

        let conn = match self.get_connection(database).await {
            Ok(c) => c,
            Err(e) => {
                let reason = e.to_string();
                for r in &mut reasons {
                    r.get_or_insert_with(|| reason.clone());
                }
                return self.finish_refused(database, operator_id, &intents, lifecycles, reasons);
            }
        };
        let mut conn = conn.lock().await;

        // 1. Validation contre le schéma (aucune écriture).
        let mut statements: Vec<(String, Vec<Value>)> = Vec::with_capacity(intents.len());
        for (i, intent) in intents.iter().enumerate() {
            if reasons[i].is_some() {
                continue;
            }
            match typed::build_write(&conn, intent) {
                Ok(stmt) => {
                    // Défense en profondeur : préparation sous l'autorisateur.
                    let _guard = AuthorizerGuard::install(&conn, SqlPolicy::DATA_WRITE);
                    match conn.prepare(&stmt.0) {
                        Ok(_) => statements.push(stmt),
                        Err(e) => reasons[i] = Some(map_prepare_error(&e).to_string()),
                    }
                }
                Err(reason) => reasons[i] = Some(reason),
            }
        }
        if reasons.iter().any(Option::is_some) {
            drop(conn);
            return self.finish_refused(database, operator_id, &intents, lifecycles, reasons);
        }
        for lc in &mut lifecycles {
            let _ = lc.transition(IntentState::Validee);
        }

        // 2. Journal en écriture anticipée (fail-closed) + non-réutilisation des identifiants.
        let validated: Vec<AuditRecord> = intents
            .iter()
            .map(|i| intent_record(i, operator_id, database, IntentState::Validee, i.purpose.clone()))
            .collect();
        if let Err(e) = self.journal.append_all(&validated, true) {
            let reason = e.to_string();
            drop(conn);
            let reasons = intents.iter().map(|_| Some(reason.clone())).collect();
            return self.finish_refused(database, operator_id, &intents, lifecycles, reasons);
        }

        // 3. Application atomique.
        let applied: Applied = (|| {
            let tx = conn
                .transaction()
                .map_err(|e| (0, format!("Cannot begin transaction: {e}")))?;
            let mut out = Vec::with_capacity(statements.len());
            for (i, (sql, params)) in statements.iter().enumerate() {
                let _guard = AuthorizerGuard::install(&tx, SqlPolicy::DATA_WRITE);
                let rows = tx
                    .prepare(sql)
                    .and_then(|mut s| s.execute(params_from_iter(params.iter())))
                    .map_err(|e| (i, e.to_string()))?;
                out.push((rows as u64, Some(tx.last_insert_rowid())));
            }
            tx.commit().map_err(|e| (0, format!("Cannot commit: {e}")))?;
            Ok(out)
        })();
        // Effacement physique : un lot contenant une intention d'effacement explicite
        // (toujours), ou dont un `Delete` a supprimé des lignes, est suivi d'un point de
        // contrôle TRUNCATE (pages remises à zéro recopiées, `-wal` vidé). Les autres lots
        // n'en paient pas le coût.
        let erased = applied.as_ref().is_ok_and(|results| {
            intents.iter().zip(results).any(|(i, (rows, _))| {
                i.erasure || (i.operation == kindmother::IntentOperation::Delete && *rows > 0)
            })
        });
        let wal_clean = !erased || truncate_wal_after_erasure(&conn, database);
        drop(conn);

        // 4. Issue, journal terminal, archivage.
        let mut outcomes = Vec::with_capacity(intents.len());
        let mut records = Vec::with_capacity(intents.len());
        let mut erasures = Vec::new();
        for (i, (intent, mut lc)) in intents.iter().zip(lifecycles).enumerate() {
            let (state, rows, last_id, reason) = match &applied {
                Ok(results) => {
                    let reason = if intent.erasure {
                        erasures.push(erasure_record(intent, operator_id, database, results[i].0, wal_clean));
                        (!wal_clean).then(|| ERASURE_RESIDUE_REASON.to_string())
                    } else {
                        None
                    };
                    (IntentState::Appliquee, results[i].0, results[i].1, reason)
                }
                Err((failed, msg)) => {
                    let reason = if *failed == i {
                        msg.clone()
                    } else {
                        format!("batch rolled back (intent #{failed} failed)")
                    };
                    (IntentState::Echouee, 0, None, Some(reason))
                }
            };
            let _ = lc.transition(state);
            let note = if wal_clean { "" } else { WAL_RESIDUE_NOTE };
            let detail = match (state, &reason) {
                (IntentState::Appliquee, _) | (_, None) => format!("{rows} rows{note}"),
                (_, Some(reason)) => reason.clone(),
            };
            records.push(intent_record(intent, operator_id, database, state, detail));
            outcomes.push((lc, IntentOutcome {
                intent_id: intent.intent_id.clone(),
                state,
                archived: false,
                rows_affected: rows,
                last_insert_id: last_id.filter(|_| intent.operation == kindmother::IntentOperation::Insert),
                reason,
            }));
        }
        records.extend(erasures);
        let archived = match self.journal.append_all(&records, false) {
            Ok(()) => true,
            Err(e) => {
                error!("AUDIT FAILURE after typed write on '{database}' (outcome not archived): {e}");
                false
            }
        };
        outcomes
            .into_iter()
            .map(|(mut lc, mut o)| {
                if archived && lc.transition(IntentState::Archivee).is_ok() {
                    o.archived = true;
                }
                o
            })
            .collect()
    }

    /// Refuse tout le lot : journalise chaque refus, archive, retourne les issues.
    fn finish_refused(
        &self,
        database: &str,
        operator_id: &str,
        intents: &[TypedWriteIntent],
        lifecycles: Vec<IntentLifecycle>,
        reasons: Vec<Option<String>>,
    ) -> Vec<IntentOutcome> {
        let first = reasons
            .iter()
            .position(Option::is_some)
            .unwrap_or_default();
        let mut records = Vec::with_capacity(intents.len());
        let mut outcomes = Vec::with_capacity(intents.len());
        for ((intent, mut lc), reason) in intents.iter().zip(lifecycles).zip(reasons) {
            let reason =
                reason.unwrap_or_else(|| format!("batch refused (intent #{first} refused)"));
            let _ = lc.transition(IntentState::Refusee);
            records.push(intent_record(
                intent,
                operator_id,
                database,
                IntentState::Refusee,
                reason.clone(),
            ));
            outcomes.push((lc, IntentOutcome {
                intent_id: intent.intent_id.clone(),
                state: IntentState::Refusee,
                archived: false,
                rows_affected: 0,
                last_insert_id: None,
                reason: Some(reason),
            }));
        }
        let archived = match self.journal.append_all(&records, false) {
            Ok(()) => true,
            Err(e) => {
                error!("AUDIT FAILURE while recording refusals on '{database}': {e}");
                false
            }
        };
        outcomes
            .into_iter()
            .map(|(mut lc, mut o)| {
                if archived && lc.transition(IntentState::Archivee).is_ok() {
                    o.archived = true;
                }
                o
            })
            .collect()
    }

    /// Lecture typée (connexion lecture seule, autorisateur lecture seule, journalisée).
    pub async fn select(
        &self,
        database: &str,
        operator_id: &str,
        query: &TypedSelect,
    ) -> Result<Rows, ServiceError> {
        let mut canonical = Vec::new();
        for (column, value) in &query.filter {
            kindmother::intent::push_len_prefixed(&mut canonical, column.as_bytes());
            value.canonical_bytes(&mut canonical);
        }
        let result = async {
            let conn = self.get_ro_connection(database).await?;
            let conn = conn.lock().await;
            let (sql, params) =
                typed::build_select(&conn, query).map_err(ServiceError::Validation)?;
            let _guard = AuthorizerGuard::install(&conn, SqlPolicy::READ_ONLY);
            let mut stmt = conn.prepare(&sql).map_err(|e| map_prepare_error(&e))?;
            collect_rows(&mut stmt, params)
        }
        .await;
        let (verdict, detail) = match &result {
            Ok(rows) => ("autorisee", format!("{} rows", rows.len())),
            Err(e) => (refusal_or_failure(e), e.to_string()),
        };
        self.journal.append(&AuditRecord {
            kind: AuditKind::Select,
            intent_id: None,
            operator_id: operator_id.to_string(),
            database: database.to_string(),
            operation: "select".to_string(),
            table: Some(query.table.clone()),
            verdict: verdict.to_string(),
            decision_digest: None,
            params_hash: sha256_hex(&canonical),
            detail,
        })?;
        result
    }

    /// Version du schéma de l'opérateur (0 si aucune migration).
    pub async fn schema_version(&self, database: &str) -> Result<u32, ServiceError> {
        let conn = self.get_connection(database).await?;
        let conn = conn.lock().await;
        read_schema_version(&conn)
    }

    /// Applique les migrations versionnées manquantes (non destructives, atomiques).
    ///
    /// Refus (`Validation` / `PermissionDenied`) : suite non contiguë, base plus
    /// récente que les migrations connues, instruction hors politique `MIGRATION`
    /// (retrait de table ou de lignes, `ATTACH`, `PRAGMA`…), ou disparition d'une
    /// table ou d'une colonne. Avant de migrer une base qui contient des tables, une
    /// copie de sauvegarde est faite dans `<data_dir>/_backups`.
    pub async fn apply_migrations(
        &self,
        database: &str,
        operator_id: &str,
        migrations: &[SchemaMigration],
    ) -> Result<MigrationReport, ServiceError> {
        let mut canonical = Vec::new();
        for m in migrations {
            canonical.extend_from_slice(&m.version.to_be_bytes());
            for s in &m.statements {
                kindmother::intent::push_len_prefixed(&mut canonical, s.as_bytes());
            }
        }
        let result = self.apply_migrations_inner(database, migrations).await;
        let (verdict, detail) = match &result {
            Ok(r) => (
                "appliquee",
                format!("v{} -> v{}", r.version_before, r.version_after),
            ),
            Err(e) => (refusal_or_failure(e), e.to_string()),
        };
        self.journal.append(&AuditRecord {
            kind: AuditKind::Migration,
            intent_id: None,
            operator_id: operator_id.to_string(),
            database: database.to_string(),
            operation: "migrate".to_string(),
            table: None,
            verdict: verdict.to_string(),
            decision_digest: None,
            params_hash: sha256_hex(&canonical),
            detail,
        })?;
        result
    }

    async fn apply_migrations_inner(
        &self,
        database: &str,
        migrations: &[SchemaMigration],
    ) -> Result<MigrationReport, ServiceError> {
        check_migrations(migrations).map_err(ServiceError::Validation)?;
        let latest = u32::try_from(migrations.len())
            .map_err(|_| ServiceError::Validation("too many migrations".to_string()))?;

        let conn = self.get_connection(database).await?;
        let mut conn = conn.lock().await;
        let before = read_schema_version(&conn)?;
        if before > latest {
            return Err(ServiceError::Validation(format!(
                "database '{database}' is at schema v{before}, newer than the code (v{latest}): refused"
            )));
        }
        if before == latest {
            return Ok(MigrationReport {
                version_before: before,
                version_after: before,
                backup: None,
            });
        }

        let snapshot = typed::schema_snapshot(&conn)?;
        let backup = if snapshot.is_empty() {
            None
        } else {
            Some(self.backup_database(&conn, database, before)?)
        };

        let tx = conn
            .transaction()
            .map_err(|e| ServiceError::Database(format!("Cannot begin migration: {e}")))?;
        for m in &migrations[before as usize..] {
            for script in &m.statements {
                // Une entrée peut contenir plusieurs instructions (ex. `schema.sql` avec
                // déclencheurs) : chacune est préparée sous la politique MIGRATION.
                let _guard = AuthorizerGuard::install(&tx, SqlPolicy::MIGRATION);
                let mut batch = rusqlite::Batch::new(&tx, script);
                let mut count = 0_usize;
                while let Some(mut stmt) = batch.next().map_err(|e| map_prepare_error(&e))? {
                    stmt.raw_execute().map_err(|e| {
                        ServiceError::Database(format!("Migration v{} failed: {e}", m.version))
                    })?;
                    count += 1;
                }
                if count == 0 {
                    return Err(ServiceError::Validation(format!(
                        "migration v{} contains an empty statement",
                        m.version
                    )));
                }
            }
            info!("Database '{database}': migration v{} ({})", m.version, m.description);
        }
        let after_snapshot = typed::schema_snapshot(&tx)?;
        typed::check_non_destructive(&snapshot, &after_snapshot).map_err(|reason| {
            ServiceError::PermissionDenied(format!("non-destructive rule: {reason}"))
        })?;
        tx.execute(
            "INSERT INTO _kindmother_meta (key, value, updated_at) VALUES (?1, ?2, datetime('now'))
             ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
            rusqlite::params![APP_SCHEMA_VERSION_KEY, latest.to_string()],
        )
        .map_err(|e| ServiceError::Database(format!("Cannot record schema version: {e}")))?;
        tx.commit()
            .map_err(|e| ServiceError::Database(format!("Cannot commit migration: {e}")))?;
        // Une migration peut écraser des valeurs (rattrapage) ou retirer des index : leurs
        // anciennes images de page ne restent pas dans le `-wal`.
        let _ = truncate_wal_after_erasure(&conn, database);

        Ok(MigrationReport {
            version_before: before,
            version_after: latest,
            backup: backup.map(|p| p.display().to_string()),
        })
    }

    /// Copie de sauvegarde (base + WAL après point de contrôle) avant migration.
    fn backup_database(
        &self,
        conn: &Connection,
        database: &str,
        version: u32,
    ) -> Result<PathBuf, ServiceError> {
        let _ = conn.query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |_| Ok(()));
        let dir = self.data_dir.join(BACKUP_DIR);
        std::fs::create_dir_all(&dir)
            .map_err(|e| ServiceError::Internal(format!("Cannot create backup dir: {e}")))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700));
        }
        let stamp = chrono::Utc::now().format("%Y%m%dT%H%M%S%.f");
        let target = dir.join(format!("{database}.v{version}-{stamp}.bak"));
        let source = self.database_path(database);
        std::fs::copy(&source, &target)
            .map_err(|e| ServiceError::Internal(format!("Backup before migration failed: {e}")))?;
        let wal = PathBuf::from(format!("{}-wal", source.display()));
        if wal.exists() {
            std::fs::copy(&wal, PathBuf::from(format!("{}-wal", target.display())))
                .map_err(|e| ServiceError::Internal(format!("Backup of WAL failed: {e}")))?;
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&target, std::fs::Permissions::from_mode(0o600));
        }
        info!("Database '{database}' backed up to {:?} before migration", target);
        Ok(target)
    }

    /// Obtient les informations sur une base de données.
    pub async fn get_info(&self, database: &str) -> Result<DatabaseInfo, ServiceError> {
        let db_path = self.database_path(database);

        if !db_path.exists() {
            return Err(ServiceError::DatabaseNotFound(database.to_string()));
        }

        let size_bytes = std::fs::metadata(&db_path).map(|m| m.len()).unwrap_or(0);

        let conn = self.get_connection(database).await?;
        let conn = conn.lock().await;

        let table_count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name NOT LIKE '_kindmother_%'",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        Ok(DatabaseInfo {
            database: database.to_string(),
            size_bytes,
            table_count: table_count as u32,
            encrypted: self.is_encrypted(),
        })
    }
}

/// Rapport d'application de migrations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationReport {
    /// Version avant.
    pub version_before: u32,
    /// Version après.
    pub version_after: u32,
    /// Copie de sauvegarde créée.
    pub backup: Option<String>,
}

fn read_schema_version(conn: &Connection) -> Result<u32, ServiceError> {
    let value: Option<String> = conn
        .query_row(
            "SELECT value FROM _kindmother_meta WHERE key = ?1",
            [APP_SCHEMA_VERSION_KEY],
            |r| r.get(0),
        )
        .optional()
        .map_err(|e| ServiceError::Database(format!("Cannot read schema version: {e}")))?;
    match value {
        None => Ok(0),
        Some(v) => v
            .parse()
            .map_err(|_| ServiceError::Database(format!("Invalid schema version: {v:?}"))),
    }
}

/// Verdict d'audit d'une erreur : refus de contrôle ou échec technique.
fn refusal_or_failure(err: &ServiceError) -> &'static str {
    match err {
        ServiceError::PermissionDenied(_)
        | ServiceError::Validation(_)
        | ServiceError::UnknownOperator(_) => "refusee",
        _ => "echouee",
    }
}

/// Empreinte du SQL brut et de ses paramètres.
fn raw_params_hash(sql: &str, params: &[String]) -> String {
    let mut buf = Vec::new();
    kindmother::intent::push_len_prefixed(&mut buf, sql.as_bytes());
    for p in params {
        kindmother::intent::push_len_prefixed(&mut buf, p.as_bytes());
    }
    sha256_hex(&buf)
}

/// Enregistrement d'audit d'une intention.
fn intent_record(
    intent: &TypedWriteIntent,
    operator_id: &str,
    database: &str,
    state: IntentState,
    detail: String,
) -> AuditRecord {
    AuditRecord {
        kind: AuditKind::Intent,
        intent_id: Some(intent.intent_id.clone()),
        operator_id: operator_id.to_string(),
        database: database.to_string(),
        operation: intent.operation.as_str().to_string(),
        table: Some(intent.table.clone()),
        verdict: state.as_str().to_string(),
        decision_digest: intent.decision_digest.clone(),
        params_hash: sha256_hex(&intent.canonical_params()),
        detail,
    }
}

/// Entrée d'audit `effacement` d'une intention d'oubli appliquée : empreinte de la clé,
/// nombre de lignes et état du point de contrôle — jamais la valeur.
fn erasure_record(
    intent: &TypedWriteIntent,
    operator_id: &str,
    database: &str,
    rows: u64,
    wal_clean: bool,
) -> AuditRecord {
    let checkpoint = if wal_clean { "ok" } else { "incomplete" };
    AuditRecord {
        kind: AuditKind::Effacement,
        intent_id: Some(intent.intent_id.clone()),
        operator_id: operator_id.to_string(),
        database: database.to_string(),
        operation: intent.operation.as_str().to_string(),
        table: Some(intent.table.clone()),
        verdict: if wal_clean { "effacee" } else { "residu_wal" }.to_string(),
        decision_digest: intent.decision_digest.clone(),
        params_hash: sha256_hex(&intent.canonical_params()),
        detail: format!("{rows} rows; secure_delete=on; wal_checkpoint(truncate)={checkpoint}"),
    }
}

/// Exécute une instruction préparée et convertit les lignes en JSON.
fn collect_rows(stmt: &mut Statement<'_>, params: Vec<Value>) -> Result<Rows, ServiceError> {
    let column_names: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();
    let rows_iter = stmt
        .query_map(params_from_iter(params), |row| {
            let mut map = HashMap::new();
            for (i, name) in column_names.iter().enumerate() {
                let value = match row.get_ref(i) {
                    Ok(rusqlite::types::ValueRef::Null) => serde_json::Value::Null,
                    Ok(rusqlite::types::ValueRef::Integer(i)) => serde_json::Value::Number(i.into()),
                    Ok(rusqlite::types::ValueRef::Real(f)) => serde_json::Number::from_f64(f)
                        .map(serde_json::Value::Number)
                        .unwrap_or(serde_json::Value::Null),
                    Ok(rusqlite::types::ValueRef::Text(s)) => {
                        serde_json::Value::String(String::from_utf8_lossy(s).to_string())
                    }
                    Ok(rusqlite::types::ValueRef::Blob(b)) => {
                        serde_json::Value::String(base64_encode(b))
                    }
                    Err(_) => serde_json::Value::Null,
                };
                map.insert(name.clone(), value);
            }
            Ok(map)
        })
        .map_err(|e| ServiceError::Database(format!("Query failed: {}", e)))?;

    let mut results = Vec::new();
    for row in rows_iter {
        results.push(row.map_err(|e| ServiceError::Database(format!("Row fetch failed: {}", e)))?);
    }
    Ok(results)
}

/// Informations sur une base de données.
pub struct DatabaseInfo {
    /// Nom de la base
    pub database: String,
    /// Taille en bytes
    pub size_bytes: u64,
    /// Nombre de tables
    pub table_count: u32,
    /// Base chiffrée
    pub encrypted: bool,
}

/// Encode en base64.
fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();

    for chunk in data.chunks(3) {
        let mut n = (chunk[0] as u32) << 16;
        if chunk.len() > 1 {
            n |= (chunk[1] as u32) << 8;
        }
        if chunk.len() > 2 {
            n |= chunk[2] as u32;
        }

        result.push(ALPHABET[(n >> 18) as usize & 63] as char);
        result.push(ALPHABET[(n >> 12) as usize & 63] as char);

        if chunk.len() > 1 {
            result.push(ALPHABET[(n >> 6) as usize & 63] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(ALPHABET[n as usize & 63] as char);
        } else {
            result.push('=');
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::{
        check_vacuum_space, enforce_secure_delete, vacuum_space_needed, EncryptedDatabase,
        VACUUM_SPACE_MARGIN,
    };

    #[test]
    fn secure_delete_force_et_relu_sur_une_connexion() {
        let c = rusqlite::Connection::open_in_memory().expect("mem");
        c.pragma_update(None, "secure_delete", "OFF").expect("off");
        enforce_secure_delete(&c).expect("on");
        let v: i64 = c.query_row("PRAGMA secure_delete", [], |r| r.get(0)).expect("read");
        assert_eq!(v, 1);
    }

    #[test]
    fn espace_disque_du_vacuum_de_reprise() {
        let needed = vacuum_space_needed(10_000, 500);
        assert_eq!(needed, 10_500 + VACUUM_SPACE_MARGIN);
        assert!(check_vacuum_space("b", needed, needed).is_ok());
        let err = check_vacuum_space("b", needed - 1, needed).expect_err("refus");
        assert!(matches!(err, crate::errors::ServiceError::QuotaExceeded(_)), "{err}");
        assert!(err.to_string().contains("free disk space"), "{err}");
        assert_eq!(vacuum_space_needed(u64::MAX, 1), u64::MAX);
    }

    #[test]
    fn nom_de_base_minuscules_chiffres_tiret_souligne_acceptes() {
        for nom in ["miyucloud", "jay_rdv", "maia-memoire", "a1", "0base"] {
            assert!(EncryptedDatabase::is_valid_database_name(nom), "{nom}");
        }
    }

    #[test]
    fn nom_de_base_avec_majuscule_refuse() {
        for nom in ["MiyuCloud", "A", "jayRdv", "BASE"] {
            assert!(!EncryptedDatabase::is_valid_database_name(nom), "{nom}");
        }
    }

    #[test]
    fn nom_de_base_hors_alphabet_ou_mal_forme_refuse() {
        let trop_long = "a".repeat(65);
        for nom in ["", "-base", "_base", "a.b", "a/b", "../x", "é", trop_long.as_str()] {
            assert!(!EncryptedDatabase::is_valid_database_name(nom), "{nom}");
        }
        assert!(EncryptedDatabase::is_valid_database_name(&"a".repeat(64)));
    }
}
