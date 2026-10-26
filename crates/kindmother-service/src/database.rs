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
//! - **Sauvegardes** (A-022 §5, module `sauvegardes`) : copies d'avant migration et
//!   d'avant restauration dans `_backups/`, chiffrées par SQLCipher avec la clé de la
//!   base, gardées 30 jours puis effacées de façon sûre (au démarrage et
//!   périodiquement) ; toute restauration réapplique les oublis du journal avant de
//!   rendre la base disponible (fail-closed).

use rusqlite::types::Value;
use rusqlite::{params_from_iter, Connection, OpenFlags, OptionalExtension, Statement};
use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tracing::{error, info, warn};

use kindmother::intent::{
    check_migrations, IntentLifecycle, IntentOutcome, IntentState, SchemaMigration, TypedSelect,
    TypedWriteIntent, MAX_INTENTS_PER_BATCH,
};

use crate::audit::{
    sha256_hex, AuditJournal, AuditKind, AuditRecord, CreationsIntention, IdentiteOubli, OubliRejouable,
    MARQUE_CREATIONS, MARQUE_REJEU,
};
use crate::identite::{self, ColonnesIdentite};
use crate::errors::ServiceError;
use crate::sauvegardes::{
    analyser_nom, date_effective, effacer_fichier_sur, empreinte_sauvegarde, fichiers_de,
    lignes_du_detail, ARejouer, EntreeRejeu, RetentionSauvegardes, BACKUP_DIR, RESTORE_DIR,
};
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

/// Copie `src` vers `dst` (créé, 0600) et la synchronise.
fn copier_prive(src: &Path, dst: &Path) -> Result<(), ServiceError> {
    let echec = |e: std::io::Error| {
        ServiceError::Internal(format!("Cannot copy {} to {}: {e}", src.display(), dst.display()))
    };
    let mut entree = std::fs::File::open(src).map_err(echec)?;
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut sortie = options.open(dst).map_err(echec)?;
    std::io::copy(&mut entree, &mut sortie).map_err(echec)?;
    sortie.sync_all().map_err(echec)
}

/// Connexion de remplacement pendant la bascule d'une restauration : base vide en
/// mémoire, en lecture seule (toute opération d'un appelant en attente échoue).
fn connexion_fermee() -> Result<Connection, ServiceError> {
    let c = Connection::open_in_memory()
        .map_err(|e| ServiceError::Internal(format!("Cannot open placeholder connection: {e}")))?;
    c.execute_batch("PRAGMA query_only = ON;")
        .map_err(|e| ServiceError::Internal(format!("Cannot configure placeholder connection: {e}")))?;
    Ok(c)
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

/// Application d'une intention : lignes, rowid, identités normalisées des lignes
/// effacées (intention d'effacement) et des lignes créées (A-025).
struct Applique {
    rows: u64,
    last_id: Option<i64>,
    effacees: Option<Vec<String>>,
    creees: BTreeSet<String>,
}

/// Résultat de l'application d'un lot, ou (indice, motif).
type Applied = Result<Vec<Applique>, (usize, String)>;

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
    /// Durée de conservation des sauvegardes (A-022 §5)
    retention: RetentionSauvegardes,
    /// Exclusion mutuelle des opérations sur `_backups/` (copie, purge, restauration).
    /// Ordre des verrous : connexion de la base, puis celui-ci.
    sauvegardes_verrou: std::sync::Mutex<()>,
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

        // Configuration invalide de la conservation ⇒ refus de démarrer (fail-closed).
        let retention = RetentionSauvegardes::depuis_env()?;
        let db = Self {
            data_dir,
            mode,
            journal,
            connections: Arc::new(RwLock::new(HashMap::new())),
            ro_connections: Arc::new(RwLock::new(HashMap::new())),
            retention,
            sauvegardes_verrou: std::sync::Mutex::new(()),
        };
        // Purge des sauvegardes expirées au démarrage (puis périodique, côté serveur).
        match db.purger_sauvegardes_expirees() {
            Ok(r) if !r.echecs.is_empty() => {
                error!("Backup purge at startup: {} failure(s): {:?}", r.echecs.len(), r.echecs);
            }
            Ok(r) => info!(
                "Backup retention {} days: {} backup(s) kept, {} purged",
                db.retention.jours(),
                r.conservees,
                r.purgees.len()
            ),
            Err(e) => error!("Backup purge at startup failed: {e}"),
        }
        Ok(db)
    }

    /// Durée de conservation des sauvegardes.
    pub fn retention_sauvegardes(&self) -> RetentionSauvegardes {
        self.retention
    }

    /// Répertoire des sauvegardes.
    pub fn backup_dir(&self) -> PathBuf {
        self.data_dir.join(BACKUP_DIR)
    }

    fn lock_sauvegardes(&self) -> Result<std::sync::MutexGuard<'_, ()>, ServiceError> {
        self.sauvegardes_verrou
            .lock()
            .map_err(|_| ServiceError::Internal("backup lock poisoned".to_string()))
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
        self.apply_key_at(conn, database, &self.database_path(database), existed)
    }

    /// Comme [`Self::apply_key`] pour un fichier de la base `database` situé en `path`
    /// (copie en cours de restauration).
    fn apply_key_at(
        &self,
        conn: &Connection,
        database: &str,
        path: &Path,
        existed: bool,
    ) -> Result<(), ServiceError> {
        match &self.mode {
            #[cfg(feature = "db-encryption")]
            StorageMode::Encrypted(store) => {
                if existed && is_plaintext_sqlite(path) {
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
                let _ = (database, path, existed);
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

        // Garantit l'existence du fichier et du schéma système. Ouverture et mise en
        // cache sous le verrou de la connexion d'écriture : une restauration (qui le
        // tient) ne peut pas laisser en cache une lecture ouverte sur l'ancien fichier.
        let rw = self.get_connection(database).await?;
        let _rw = rw.lock().await;
        if let Some(conn) = self.ro_connections.read().await.get(database) {
            return Ok(conn.clone());
        }

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

        // 1. Validation contre le schéma (aucune écriture). Un effacement exige une
        //    identité normalisée de ses lignes (clé primaire ou unique déclarée, A-025).
        let mut statements: Vec<(String, Vec<Value>)> = Vec::with_capacity(intents.len());
        let mut identites_cols: Vec<Option<ColonnesIdentite>> = vec![None; intents.len()];
        for (i, intent) in intents.iter().enumerate() {
            if reasons[i].is_some() {
                continue;
            }
            match typed::build_write(&conn, intent) {
                Ok(stmt) => {
                    let utile = intent.erasure
                        || matches!(
                            intent.operation,
                            kindmother::IntentOperation::Insert | kindmother::IntentOperation::Upsert
                        );
                    if utile {
                        match identite::colonnes_identite(&conn, &intent.table) {
                            Ok(Some(c)) => identites_cols[i] = Some(c),
                            Ok(None) if intent.erasure => {
                                reasons[i] = Some(format!(
                                    "erasure refused on {}: no declared primary key nor NOT NULL unique key \
                                     (A-025: erased rows need a normalized identity to be replayed on restore)",
                                    intent.table
                                ));
                                continue;
                            }
                            Ok(None) => {}
                            Err(e) => {
                                reasons[i] = Some(e);
                                continue;
                            }
                        }
                    }
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
                let intent = &intents[i];
                let cols = identites_cols[i].as_ref();
                // Lignes désignées par la clé de l'intention, identifiées par la base.
                let par_cle = |c: &ColonnesIdentite| {
                    let mut p = Vec::new();
                    let filtre = typed::where_clause(&intent.key, &mut p);
                    identite::identites(&tx, &intent.table, &c.colonnes, &filtre, &p).map_err(|e| (i, e))
                };
                let upsert = intent.operation == kindmother::IntentOperation::Upsert;
                let avant = match cols {
                    Some(c) if intent.erasure || upsert => Some(par_cle(c)?),
                    _ => None,
                };
                let rows = {
                    let _guard = AuthorizerGuard::install(&tx, SqlPolicy::DATA_WRITE);
                    tx.prepare(sql)
                        .and_then(|mut s| s.execute(params_from_iter(params.iter())))
                        .map_err(|e| (i, e.to_string()))? as u64
                };
                let rowid = tx.last_insert_rowid();
                let effacees = if intent.erasure {
                    let avant = avant.as_deref().unwrap_or_default();
                    if avant.iter().any(|l| l.contient_null) {
                        return Err((i, "erasure refused: an erased row has a NULL key column \
                                        (identity not determinable, A-025)".to_string()));
                    }
                    let ids: BTreeSet<String> = avant.iter().map(|l| l.identite.clone()).collect();
                    let exacte = ids.len() == avant.len()
                        && (upsert || ids.len() as u64 == rows);
                    if !exacte {
                        return Err((i, format!(
                            "erasure refused: {rows} row(s) affected, {} identified (A-025)",
                            ids.len()
                        )));
                    }
                    Some(ids.into_iter().collect())
                } else {
                    None
                };
                let creees: BTreeSet<String> = match (intent.operation, cols) {
                    (kindmother::IntentOperation::Insert, Some(c)) if rows == 1 => {
                        identite::identite_inseree(&tx, &intent.table, c, &intent.values, rowid)
                            .into_iter()
                            .collect()
                    }
                    (kindmother::IntentOperation::Upsert, Some(c)) => {
                        let avant: BTreeSet<&str> =
                            avant.iter().flatten().map(|l| l.identite.as_str()).collect();
                        par_cle(c)?
                            .into_iter()
                            .filter(|l| !l.contient_null && !avant.contains(l.identite.as_str()))
                            .map(|l| l.identite)
                            .collect()
                    }
                    _ => BTreeSet::new(),
                };
                out.push(Applique { rows, last_id: Some(rowid), effacees, creees });
            }
            tx.commit().map_err(|e| (0, format!("Cannot commit: {e}")))?;
            Ok(out)
        })();
        // Effacement physique : un lot contenant une intention d'effacement explicite
        // (toujours), ou dont un `Delete` a supprimé des lignes, est suivi d'un point de
        // contrôle TRUNCATE (pages remises à zéro recopiées, `-wal` vidé). Les autres lots
        // n'en paient pas le coût.
        let erased = applied.as_ref().is_ok_and(|results| {
            intents.iter().zip(results).any(|(i, a)| {
                i.erasure || (i.operation == kindmother::IntentOperation::Delete && a.rows > 0)
            })
        });
        let wal_clean = !erased || truncate_wal_after_erasure(&conn, database);
        drop(conn);

        // 4. Issue, journal terminal, archivage.
        let mut outcomes = Vec::with_capacity(intents.len());
        let mut records = Vec::with_capacity(intents.len());
        let mut erasures: Vec<(AuditRecord, OubliRejouable)> = Vec::new();
        let mut creations: Vec<(usize, CreationsIntention)> = Vec::new();
        let mut marques_creation: Vec<Option<String>> = vec![None; intents.len()];
        for (i, (intent, mut lc)) in intents.iter().zip(lifecycles).enumerate() {
            let (state, rows, last_id, reason) = match &applied {
                Ok(results) => {
                    let a = &results[i];
                    if !a.creees.is_empty() {
                        let c = CreationsIntention {
                            database: database.to_string(),
                            table: intent.table.to_ascii_lowercase(),
                            identites: a.creees.clone(),
                        };
                        marques_creation[i] = Some(format!("; {MARQUE_CREATIONS}{}", c.empreinte()));
                        creations.push((i, c));
                    }
                    let reason = if intent.erasure {
                        let mut oubli = OubliRejouable::depuis_intention(intent, database);
                        if let (Some(ids), Some(c)) = (&a.effacees, &identites_cols[i]) {
                            oubli = oubli.avec_identite(IdentiteOubli {
                                colonnes: c.colonnes.clone(),
                                types: c.types.clone(),
                                lignes: ids.len() as u64,
                                identites: ids.clone(),
                            });
                        }
                        match erasure_record(intent, operator_id, database, a.rows, wal_clean, &oubli) {
                            Ok(rec) => erasures.push((rec, oubli)),
                            Err(e) => error!("Cannot build erasure record on '{database}': {e}"),
                        }
                        (!wal_clean).then(|| ERASURE_RESIDUE_REASON.to_string())
                    } else {
                        None
                    };
                    (IntentState::Appliquee, a.rows, a.last_id, reason)
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
            let creees = marques_creation[i].as_deref().unwrap_or_default();
            let detail = match (state, &reason) {
                (IntentState::Appliquee, _) | (_, None) => format!("{rows} rows{note}{creees}"),
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
        let base = records.len();
        let mut oublis = Vec::with_capacity(erasures.len());
        for (i, (rec, oubli)) in erasures.into_iter().enumerate() {
            records.push(rec);
            oublis.push((base + i, oubli));
        }
        let archived = match self.journal.append_all_avec_registres(&records, false, &oublis, &creations) {
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
            let _verrou = self.lock_sauvegardes()?;
            Some(self.backup_database_locked(&conn, database, before, "avant migration")?)
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

    // ------------------------------------------------------------------
    // Sauvegardes (A-022 §5) — voir le module `sauvegardes`.
    // ------------------------------------------------------------------

    /// Copie de sauvegarde (base, et `-wal` s'il n'est pas vide après le point de
    /// contrôle) dans `_backups/`, sous le verrou des sauvegardes (tenu par l'appelant).
    ///
    /// La copie est le fichier SQLCipher : chiffrée avec la clé de la base. Elle est
    /// vérifiée non lisible en clair (mode chiffré), synchronisée, puis inscrite au
    /// journal (`migration` / `sauvegarde`, empreinte du contenu). Tout échec ⇒ copie
    /// effacée de façon sûre et erreur (l'opération qui l'exigeait est refusée).
    fn backup_database_locked(
        &self,
        conn: &Connection,
        database: &str,
        version: u32,
        motif: &str,
    ) -> Result<PathBuf, ServiceError> {
        let _ = conn.query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |_| Ok(()));
        let dir = self.backup_dir();
        crate::sauvegardes::dossier_prive(&dir)?;
        let nom = crate::sauvegardes::nom_sauvegarde(database, version, chrono::Utc::now());
        let target = dir.join(&nom);
        let source = self.database_path(database);
        let result = (|| -> Result<(), ServiceError> {
            copier_prive(&source, &target)?;
            let wal = wal_file(&source);
            let avec_wal = file_len(&wal) > 0;
            if avec_wal {
                copier_prive(&wal, &wal_file(&target))?;
            }
            let chiffrement = if self.is_encrypted() {
                if is_plaintext_sqlite(&target) {
                    error!("Backup of '{database}' is readable in plaintext: refused and erased");
                    return Err(ServiceError::Encryption(format!(
                        "backup of '{database}' is not encrypted: refused"
                    )));
                }
                "sqlcipher-cle-de-la-base"
            } else {
                warn!("Backup of '{database}' in PLAINTEXT (development mode only)");
                "clair-developpement"
            };
            crate::sauvegardes::sync_dossier(&dir)?;
            self.journal.append(&AuditRecord {
                kind: AuditKind::Migration,
                intent_id: None,
                operator_id: SYSTEM_OPERATOR.to_string(),
                database: database.to_string(),
                operation: "sauvegarde".to_string(),
                table: None,
                verdict: "appliquee".to_string(),
                decision_digest: None,
                params_hash: empreinte_sauvegarde(&target)?,
                detail: format!(
                    "fichier={nom}; motif={motif}; chiffrement={chiffrement}; wal={}; retention={} j",
                    u8::from(avec_wal),
                    self.retention.jours()
                ),
            })?;
            Ok(())
        })();
        if let Err(e) = result {
            for f in fichiers_de(&target) {
                if let Err(err) = effacer_fichier_sur(&f) {
                    error!("Cannot erase failed backup {}: {err}", f.display());
                }
            }
            return Err(e);
        }
        info!("Database '{database}' backed up to {:?} ({motif})", target);
        Ok(target)
    }

    /// Sauvegardes présentes pour `database` (noms de fichiers, de la plus ancienne à la
    /// plus récente).
    pub fn lister_sauvegardes(&self, database: &str) -> Result<Vec<String>, ServiceError> {
        let mut out: Vec<(chrono::DateTime<chrono::Utc>, String)> = Vec::new();
        let entries = match std::fs::read_dir(self.backup_dir()) {
            Ok(e) => e,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(e) => return Err(ServiceError::Internal(format!("Cannot list backups: {e}"))),
        };
        for entry in entries {
            let entry = entry.map_err(|e| ServiceError::Internal(format!("Cannot list backups: {e}")))?;
            let Some(nom) = entry.file_name().to_str().map(str::to_string) else {
                continue;
            };
            if let Some(n) = analyser_nom(&nom).filter(|n| n.database == database) {
                out.push((n.horodatage, nom));
            }
        }
        out.sort();
        Ok(out.into_iter().map(|(_, n)| n).collect())
    }

    /// Purge les sauvegardes plus âgées que la durée de conservation (effacement sûr,
    /// une entrée d'audit `effacement` / `purge-sauvegarde` par sauvegarde).
    pub fn purger_sauvegardes_expirees(&self) -> Result<RapportPurge, ServiceError> {
        self.purger_sauvegardes_au(chrono::Utc::now())
    }

    /// Comme [`Self::purger_sauvegardes_expirees`], l'âge étant mesuré à `maintenant`.
    pub fn purger_sauvegardes_au(
        &self,
        maintenant: chrono::DateTime<chrono::Utc>,
    ) -> Result<RapportPurge, ServiceError> {
        let _verrou = self.lock_sauvegardes()?;
        let dir = self.backup_dir();
        let mut rapport = RapportPurge::default();
        let entries = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(rapport),
            Err(e) => return Err(ServiceError::Internal(format!("Cannot list backups: {e}"))),
        };
        // Une sauvegarde = son fichier principal et ses annexes `-wal` / `-shm`.
        let mut groupes = std::collections::BTreeSet::new();
        for entry in entries {
            let entry = entry.map_err(|e| ServiceError::Internal(format!("Cannot list backups: {e}")))?;
            let Some(nom) = entry.file_name().to_str().map(str::to_string) else {
                warn!("Backup directory: non UTF-8 file name ignored");
                continue;
            };
            let principal = nom
                .strip_suffix("-wal")
                .or_else(|| nom.strip_suffix("-shm"))
                .unwrap_or(&nom)
                .to_string();
            groupes.insert(principal);
        }
        for principal in groupes {
            let chemin = dir.join(&principal);
            if std::fs::symlink_metadata(&chemin).is_ok_and(|m| m.is_dir()) {
                continue;
            }
            let analyse = analyser_nom(&principal);
            let modifie = fichiers_de(&chemin)
                .iter()
                .filter_map(|f| std::fs::symlink_metadata(f).ok())
                .filter_map(|m| m.modified().ok())
                .map(chrono::DateTime::<chrono::Utc>::from)
                .min();
            let Some(date) = date_effective(analyse.as_ref(), modifie) else {
                continue;
            };
            let age = maintenant.signed_duration_since(date);
            if age <= self.retention.duree() {
                rapport.conservees += 1;
                continue;
            }
            let mut effaces = Vec::new();
            let mut erreur: Option<String> = None;
            for (f, etiquette) in fichiers_de(&chemin).iter().zip(["bak", "wal", "shm"]) {
                match effacer_fichier_sur(f) {
                    Ok(true) => effaces.push(etiquette),
                    Ok(false) => {}
                    Err(e) => {
                        erreur.get_or_insert_with(|| e.to_string());
                    }
                }
            }
            if let Err(e) = crate::sauvegardes::sync_dossier(&dir) {
                erreur.get_or_insert_with(|| e.to_string());
            }
            let detail = format!(
                "fichier={principal}; age={} j; retention={} j; effaces={}{}",
                age.num_days(),
                self.retention.jours(),
                effaces.join(","),
                erreur.as_deref().map(|e| format!("; erreur={e}")).unwrap_or_default()
            );
            // Purge non journalisée = erreur : la purge s'arrête (reprise au tour suivant).
            self.journal.append(&AuditRecord {
                kind: AuditKind::Effacement,
                intent_id: None,
                operator_id: SYSTEM_OPERATOR.to_string(),
                database: analyse.map_or_else(|| BACKUP_DIR.to_string(), |n| n.database),
                operation: "purge-sauvegarde".to_string(),
                table: None,
                verdict: if erreur.is_none() { "effacee" } else { "echouee" }.to_string(),
                decision_digest: None,
                params_hash: sha256_hex(principal.as_bytes()),
                detail,
            })?;
            match erreur {
                None => {
                    info!("Backup {principal} purged ({} days old)", age.num_days());
                    rapport.purgees.push(principal);
                }
                Some(e) => {
                    error!("Backup {principal} expired but not erased: {e}");
                    rapport.echecs.push(format!("{principal}: {e}"));
                }
            }
        }
        Ok(rapport)
    }

    /// **Restaure** la sauvegarde `fichier` (nom dans `_backups/`) de `database`.
    ///
    /// Avant que la base ne soit rendue disponible : journal d'audit vérifié (chaîne
    /// complète), empreinte de la sauvegarde comparée à celle inscrite, copie ouverte
    /// avec la clé de la base et contrôlée, **oublis rejoués** (toutes les entrées
    /// `effacement` d'intentions postérieures à la sauvegarde ; toutes celles de la base
    /// si la sauvegarde n'est pas inscrite), point de contrôle `TRUNCATE` complet,
    /// journalisation (`effacement` / `rejeu` par oubli, `migration` / `restauration`).
    /// L'état remplacé est d'abord sauvegardé (`avant restauration`). Les connexions en
    /// cache sont remplacées sous leur verrou : aucune opération ne voit d'état
    /// intermédiaire. Tout échec avant la bascule laisse la base courante inchangée ;
    /// un échec après la bascule laisse la base **indisponible** (jamais la copie non
    /// purgée de ses oublis).
    pub async fn restaurer_sauvegarde(
        &self,
        database: &str,
        fichier: &str,
        operator_id: &str,
    ) -> Result<RapportRestauration, ServiceError> {
        if !Self::is_valid_database_name(database) {
            return Err(ServiceError::Validation(format!("Invalid database name: {database}")));
        }
        let conn_arc = self.get_connection(database).await?;
        let mut conn = conn_arc.lock().await;
        let ro_arc = self.ro_connections.read().await.get(database).cloned();
        let mut ro = match &ro_arc {
            Some(a) => Some(a.lock().await),
            None => None,
        };
        let result =
            self.restaurer_verrouille(database, fichier, operator_id, &mut conn, ro.as_deref_mut());
        if let Err((e, fermee)) = &result {
            if let Err(err) = self.journal.append(&AuditRecord {
                kind: AuditKind::Migration,
                intent_id: None,
                operator_id: operator_id.to_string(),
                database: database.to_string(),
                operation: "restauration".to_string(),
                table: None,
                verdict: refusal_or_failure(e).to_string(),
                decision_digest: None,
                params_hash: sha256_hex(fichier.as_bytes()),
                detail: format!("fichier={fichier}; base_indisponible={}; {e}", u8::from(*fermee)),
            }) {
                error!("AUDIT FAILURE while recording a refused restoration of '{database}': {err}");
            }
        }
        drop(ro);
        drop(conn);
        match result {
            Ok(rapport) => Ok(rapport),
            Err((e, fermee)) => {
                if fermee {
                    // Base indisponible : la prochaine ouverture repart du fichier.
                    self.connections.write().await.remove(database);
                    self.ro_connections.write().await.remove(database);
                    error!("Database '{database}' unavailable after a failed restoration: {e}");
                }
                Err(e)
            }
        }
    }

    /// Corps de la restauration, connexions verrouillées. `Err((e, true))` : la base
    /// courante a été fermée (indisponible).
    fn restaurer_verrouille(
        &self,
        database: &str,
        fichier: &str,
        operator_id: &str,
        conn: &mut Connection,
        mut ro: Option<&mut Connection>,
    ) -> Result<RapportRestauration, (ServiceError, bool)> {
        let ouverte = |e: ServiceError| (e, false);
        let _verrou = self.lock_sauvegardes().map_err(ouverte)?;

        // 1. Journal : chaîne complète, sinon aucun oubli n'est sûr de pouvoir être rejoué.
        let entries = self.journal.verified_entries().map_err(|e| {
            ouverte(ServiceError::PermissionDenied(format!(
                "restoration refused: erasure journal unreadable or broken ({e})"
            )))
        })?;

        // 2. Sauvegarde : nom de cette base, fichier ordinaire de `_backups/`.
        if analyser_nom(fichier).is_none_or(|n| n.database != database) {
            return Err(ouverte(ServiceError::Validation(format!(
                "not a backup of '{database}': {fichier:?}"
            ))));
        }
        let chemin = self.backup_dir().join(fichier);
        match std::fs::symlink_metadata(&chemin) {
            Ok(m) if m.is_file() => {}
            Ok(_) => {
                return Err(ouverte(ServiceError::PermissionDenied(format!(
                    "backup {fichier} is not a regular file"
                ))))
            }
            Err(_) => return Err(ouverte(ServiceError::Validation(format!("backup not found: {fichier}")))),
        }
        let empreinte = empreinte_sauvegarde(&chemin).map_err(ouverte)?;
        let champ = format!("fichier={fichier}");
        let inscrite = entries.iter().rev().find(|e| {
            e.record.kind == AuditKind::Migration
                && e.record.operation == "sauvegarde"
                && e.record.database == database
                && e.record.detail.split("; ").any(|p| p == champ)
        });
        let depuis = match inscrite {
            Some(e) if e.record.params_hash == empreinte => e.seq,
            Some(e) => {
                return Err(ouverte(ServiceError::PermissionDenied(format!(
                    "restoration refused: backup {fichier} differs from the copy recorded in entry #{}",
                    e.seq
                ))))
            }
            None => {
                warn!(
                    "Backup {fichier} not recorded in the journal (older version): \
                     replaying every erasure of '{database}'"
                );
                0
            }
        };

        // 3. Oublis à rejouer (ordre du journal).
        let mut a_rejouer: Vec<(&crate::audit::AuditEntry, Option<OubliRejouable>)> = Vec::new();
        for e in entries.iter().filter(|e| {
            e.seq > depuis
                && e.record.kind == AuditKind::Effacement
                && e.record.database == database
                && matches!(e.record.operation.as_str(), "delete" | "update" | "upsert")
        }) {
            let marque = e.record.detail.split("; ").find_map(|p| p.strip_prefix(MARQUE_REJEU));
            let oubli = match marque {
                Some(attendue) => {
                    let oubli = self.journal.oubli(e.seq).map_err(ouverte)?.ok_or_else(|| {
                        ouverte(ServiceError::PermissionDenied(format!(
                            "restoration refused: erasure registry entry #{} missing",
                            e.seq
                        )))
                    })?;
                    let conforme = oubli.empreinte().map_err(ouverte)? == attendue
                        && oubli.database == database
                        && e.record.table.as_deref() == Some(oubli.table.as_str());
                    if !conforme {
                        return Err(ouverte(ServiceError::PermissionDenied(format!(
                            "restoration refused: erasure registry entry #{} altered",
                            e.seq
                        ))));
                    }
                    Some(oubli)
                }
                None if e.record.operation == "delete" => None,
                None => {
                    return Err(ouverte(ServiceError::PermissionDenied(format!(
                        "restoration refused: erasure #{} ({}) predates the erasure registry \
                         and cannot be replayed",
                        e.seq, e.record.operation
                    ))))
                }
            };
            a_rejouer.push((e, oubli));
        }

        // 4. Copie de l'état remplacé.
        let version = read_schema_version(conn).map_err(ouverte)?;
        let avant = self
            .backup_database_locked(conn, database, version, "avant restauration")
            .map_err(ouverte)?;

        // 5. Préparation hors de portée des opérateurs, oublis rejoués.
        let dossier = self.data_dir.join(RESTORE_DIR);
        crate::sauvegardes::dossier_prive(&dossier).map_err(ouverte)?;
        let staging = dossier.join(format!("{database}.db"));
        let effacer_staging = || {
            for f in fichiers_de(&staging) {
                if let Err(err) = effacer_fichier_sur(&f) {
                    error!("Cannot erase restoration staging {}: {err}", f.display());
                }
            }
        };
        effacer_staging();
        let prep = (|| -> Result<Vec<(i64, crate::sauvegardes::BilanRejeu)>, ServiceError> {
            copier_prive(&chemin, &staging)?;
            let wal = wal_file(&chemin);
            if file_len(&wal) > 0 {
                copier_prive(&wal, &wal_file(&staging))?;
            }
            let mut sc = Connection::open_with_flags(
                &staging,
                OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX,
            )
            .map_err(|e| ServiceError::Database(format!("Cannot open backup copy: {e}")))?;
            self.apply_key_at(&sc, database, &staging, true)?;
            enforce_secure_delete(&sc)?;
            sc.execute_batch("PRAGMA journal_mode = WAL; PRAGMA busy_timeout = 5000;")
                .map_err(|e| ServiceError::Database(format!("Cannot configure backup copy: {e}")))?;
            let controle: String = sc
                .query_row("PRAGMA quick_check", [], |r| r.get(0))
                .map_err(|e| ServiceError::Database(format!("Backup check failed: {e}")))?;
            if controle != "ok" {
                return Err(ServiceError::Database(format!("backup {fichier} is damaged: {controle}")));
            }
            let tx = sc
                .transaction()
                .map_err(|e| ServiceError::Database(format!("Cannot begin replay: {e}")))?;
            // Preuve qu'une ligne oubliée absente de la copie a été créée après elle (A-025) :
            // création inscrite au registre des créations, couverte par la chaîne
            // (`creations=` dans le détail de l'entrée terminale de l'intention), à une
            // entrée postérieure à la sauvegarde **inscrite** et antérieure à l'oubli.
            let preuve = |table: &str, identite: &str, avant: i64| -> Result<bool, String> {
                if inscrite.is_none() {
                    return Ok(false);
                }
                let seqs = self
                    .journal
                    .entrees_de_creation(database, table, identite)
                    .map_err(|e| e.to_string())?;
                for seq in seqs.into_iter().filter(|s| *s > depuis && *s < avant) {
                    let entree = entries
                        .binary_search_by_key(&seq, |e| e.seq)
                        .ok()
                        .and_then(|i| entries.get(i))
                        .ok_or_else(|| format!("creation registry entry #{seq} has no journal entry"))?;
                    let attendue = entree
                        .record
                        .detail
                        .split("; ")
                        .find_map(|p| p.strip_prefix(MARQUE_CREATIONS))
                        .ok_or_else(|| format!("creation registry entry #{seq} not covered by the journal"))?;
                    let c = self
                        .journal
                        .creations(seq)
                        .map_err(|e| e.to_string())?
                        .ok_or_else(|| format!("creation registry entry #{seq} missing"))?;
                    let conforme = entree.record.kind == AuditKind::Intent
                        && entree.record.verdict == IntentState::Appliquee.as_str()
                        && entree.record.database == database
                        && entree.record.table.as_deref().is_some_and(|t| t.eq_ignore_ascii_case(table))
                        && c.database == database
                        && c.table.eq_ignore_ascii_case(table)
                        && c.empreinte() == attendue;
                    if !conforme {
                        return Err(format!("creation registry entry #{seq} altered"));
                    }
                    if c.identites.contains(identite) {
                        return Ok(true);
                    }
                }
                Ok(false)
            };
            let mut rejoues = Vec::with_capacity(a_rejouer.len());
            for (e, oubli) in &a_rejouer {
                let cible = EntreeRejeu {
                    oubli: match oubli {
                        Some(o) => ARejouer::Registre(o),
                        None => ARejouer::Ancien {
                            table: e.record.table.as_deref().unwrap_or_default(),
                            params_hash: &e.record.params_hash,
                        },
                    },
                    seq: e.seq,
                    lignes_journal: lignes_du_detail(&e.record.detail),
                };
                let bilan = crate::sauvegardes::rejouer(&tx, &cible, &preuve).map_err(|m| {
                    ServiceError::PermissionDenied(format!(
                        "restoration refused: erasure #{} cannot be replayed ({m})",
                        e.seq
                    ))
                })?;
                rejoues.push((e.seq, bilan));
            }
            tx.commit()
                .map_err(|e| ServiceError::Database(format!("Cannot commit replay: {e}")))?;
            if !truncate_wal_after_erasure(&sc, database) {
                return Err(ServiceError::Database(
                    "WAL checkpoint after replay incomplete: restoration refused".to_string(),
                ));
            }
            drop(sc);
            // Dernière connexion fermée : `-wal`/`-shm` retirés par SQLite ; reste éventuel effacé.
            for f in &fichiers_de(&staging)[1..] {
                effacer_fichier_sur(f)?;
            }
            Ok(rejoues)
        })();
        let rejoues = match prep {
            Ok(r) => r,
            Err(e) => {
                effacer_staging();
                return Err(ouverte(e));
            }
        };
        let lignes: u64 = rejoues.iter().map(|(_, b)| b.lignes).sum();
        let posterieures: u64 = rejoues.iter().map(|(_, b)| b.posterieures).sum();

        // 6. Journal avant toute mise à disposition (fail-closed).
        let mut records: Vec<AuditRecord> = a_rejouer
            .iter()
            .zip(&rejoues)
            .map(|((e, _), (seq, b))| AuditRecord {
                kind: AuditKind::Effacement,
                intent_id: None,
                operator_id: operator_id.to_string(),
                database: database.to_string(),
                operation: "rejeu".to_string(),
                table: e.record.table.clone(),
                verdict: "effacee".to_string(),
                decision_digest: None,
                params_hash: e.record.params_hash.clone(),
                detail: format!(
                    "restauration de {fichier}; entree #{seq}; {} rows; retrouvees={}; posterieures={}; filet={}",
                    b.lignes, b.retrouvees, b.posterieures, b.filet
                ),
            })
            .collect();
        let nom_avant = avant
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default()
            .to_string();
        records.push(AuditRecord {
            kind: AuditKind::Migration,
            intent_id: None,
            operator_id: operator_id.to_string(),
            database: database.to_string(),
            operation: "restauration".to_string(),
            table: None,
            verdict: "appliquee".to_string(),
            decision_digest: None,
            params_hash: empreinte,
            detail: format!(
                "fichier={fichier}; inscrite={}; depuis=#{depuis}; oublis_rejoues={}; lignes={lignes}; \
                 sauvegarde_avant={nom_avant}",
                u8::from(inscrite.is_some()),
                rejoues.len()
            ),
        });
        if let Err(e) = self.journal.append_all(&records, false) {
            effacer_staging();
            return Err(ouverte(e));
        }

        // 7. Bascule : fermeture des connexions (remplacées sous leur verrou), échange des
        //    fichiers, ouverture de la base restaurée.
        let (attente_rw, attente_ro) = match (connexion_fermee(), connexion_fermee()) {
            (Ok(a), Ok(b)) => (a, b),
            (Err(e), _) | (_, Err(e)) => {
                effacer_staging();
                return Err(ouverte(e));
            }
        };
        if let Some(ro) = ro.as_deref_mut() {
            drop(std::mem::replace(ro, attente_ro));
        }
        drop(std::mem::replace(conn, attente_rw));
        let fermee = |e: ServiceError| (e, true);
        let live = self.database_path(database);
        let ancien = dossier.join(format!("{database}.ancien"));
        for f in fichiers_de(&ancien) {
            effacer_fichier_sur(&f).map_err(fermee)?;
        }
        if let Err(e) = std::fs::rename(&live, &ancien) {
            effacer_staging();
            return Err(fermee(ServiceError::Internal(format!("Cannot move the replaced database: {e}"))));
        }
        // Annexes de l'ancienne base (normalement retirées à la fermeture) : jamais
        // appliquées à la base restaurée.
        for f in &fichiers_de(&live)[1..] {
            effacer_fichier_sur(f).map_err(fermee)?;
        }
        if let Err(e) = std::fs::rename(&staging, &live) {
            let retour = std::fs::rename(&ancien, &live);
            error!("Cannot install the restored copy of '{database}': {e} (rollback: {retour:?})");
            effacer_staging();
            return Err(fermee(ServiceError::Internal(format!("Cannot install the restored copy: {e}"))));
        }
        crate::sauvegardes::sync_dossier(&self.data_dir).map_err(fermee)?;
        for f in fichiers_de(&ancien) {
            // Son contenu est dans la sauvegarde « avant restauration ».
            if let Err(e) = effacer_fichier_sur(&f) {
                error!("Cannot erase the replaced database file {}: {e}", f.display());
            }
        }
        *conn = self.open_database(database).map_err(fermee)?;
        if let Some(ro) = ro {
            *ro = self.open_database_read_only(database).map_err(fermee)?;
        }
        warn!(
            "Database '{database}' restored from {fichier}: {} erasure(s) replayed, {lignes} row(s)",
            rejoues.len()
        );
        Ok(RapportRestauration {
            sauvegarde: fichier.to_string(),
            inscrite: inscrite.is_some(),
            depuis_entree: depuis,
            oublis_rejoues: rejoues.len(),
            lignes_effacees: lignes,
            lignes_posterieures: posterieures,
            sauvegarde_avant_restauration: nom_avant,
        })
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

/// Rapport d'une purge des sauvegardes expirées.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RapportPurge {
    /// Sauvegardes conservées (plus jeunes que la durée de conservation).
    pub conservees: usize,
    /// Sauvegardes effacées (noms de fichiers).
    pub purgees: Vec<String>,
    /// Sauvegardes expirées dont l'effacement a échoué (nom et motif) ; retentées à la
    /// purge suivante.
    pub echecs: Vec<String>,
}

/// Rapport d'une restauration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RapportRestauration {
    /// Sauvegarde restaurée (nom de fichier dans `_backups/`).
    pub sauvegarde: String,
    /// La sauvegarde était inscrite au journal et son contenu y correspondait. Faux
    /// pour une sauvegarde antérieure à l'inscription : tous les oublis de la base sont
    /// alors rejoués.
    pub inscrite: bool,
    /// Numéro de l'entrée de journal de la sauvegarde (0 si non inscrite) : les oublis
    /// d'entrées postérieures ont été rejoués.
    pub depuis_entree: i64,
    /// Oublis rejoués (entrées `effacement` postérieures à la sauvegarde).
    pub oublis_rejoues: usize,
    /// Lignes effacées ou caviardées par le rejeu.
    pub lignes_effacees: u64,
    /// Lignes oubliées absentes de la sauvegarde, prouvées créées après elle (registre
    /// des créations, A-025).
    pub lignes_posterieures: u64,
    /// Copie de l'état remplacé (dans `_backups/`, soumise à la même conservation).
    pub sauvegarde_avant_restauration: String,
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
    oubli: &OubliRejouable,
) -> Result<AuditRecord, ServiceError> {
    let checkpoint = if wal_clean { "ok" } else { "incomplete" };
    Ok(AuditRecord {
        kind: AuditKind::Effacement,
        intent_id: Some(intent.intent_id.clone()),
        operator_id: operator_id.to_string(),
        database: database.to_string(),
        operation: intent.operation.as_str().to_string(),
        table: Some(intent.table.clone()),
        verdict: if wal_clean { "effacee" } else { "residu_wal" }.to_string(),
        decision_digest: intent.decision_digest.clone(),
        params_hash: sha256_hex(&intent.canonical_params()),
        detail: format!(
            "{rows} rows; secure_delete=on; wal_checkpoint(truncate)={checkpoint}; {MARQUE_REJEU}{}",
            oubli.empreinte()?
        ),
    })
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
