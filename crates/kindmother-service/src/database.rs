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
        let existed = db_path.exists();

        let conn = Connection::open_with_flags(
            &db_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .map_err(|e| ServiceError::Database(format!("Cannot open database: {}", e)))?;

        self.apply_key(&conn, database, existed)?;

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

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(0o600);
            let _ = std::fs::set_permissions(&db_path, perms);
        }

        Ok(conn)
    }

    /// Valide un nom de base de données.
    fn is_valid_database_name(name: &str) -> bool {
        !name.is_empty()
            && name.len() <= 64
            && name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
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
        let result = async {
            let conn = self.get_connection(database).await?;
            let conn = conn.lock().await;
            let rows_affected = {
                let _guard = AuthorizerGuard::install(&conn, policy);
                let mut stmt = prepare_single(&conn, sql)?;
                stmt.execute(params_from_iter(params))
                    .map_err(|e| ServiceError::Database(format!("Execute failed: {}", e)))?
            };
            Ok::<_, ServiceError>((rows_affected as u64, Some(conn.last_insert_rowid())))
        }
        .await;
        self.audit_raw_write(database, operator_id, "execute", intent, params_hash, &result.as_ref().map(|r| r.0))?;
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

        let result = async {
            let conn = self.get_connection(database).await?;
            let mut conn = conn.lock().await;
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
            Ok::<_, ServiceError>(results)
        }
        .await;
        let total = result.as_ref().map(|r| r.iter().sum::<u64>());
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
        result: &Result<u64, &ServiceError>,
    ) -> Result<(), ServiceError> {
        let (verdict, detail) = match result {
            Ok(rows) => ("appliquee", format!("{intent}: {rows} rows")),
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
        drop(conn);

        // 4. Issue, journal terminal, archivage.
        let mut outcomes = Vec::with_capacity(intents.len());
        let mut records = Vec::with_capacity(intents.len());
        for (i, (intent, mut lc)) in intents.iter().zip(lifecycles).enumerate() {
            let (state, rows, last_id, reason) = match &applied {
                Ok(results) => (IntentState::Appliquee, results[i].0, results[i].1, None),
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
            let detail = reason.clone().unwrap_or_else(|| format!("{rows} rows"));
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
