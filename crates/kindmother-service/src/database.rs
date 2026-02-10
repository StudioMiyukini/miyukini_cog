//! Gestion des bases de données SQLite via KindMother Service.
//!
//! @id: kindmother_service_database
//! @do: manage_sqlite_databases_isolated
//! @layer: core
//!
//! Ce module gère les bases de données SQLite de manière isolée.
//! KindMother Service est le seul processus autorisé à accéder aux fichiers DB.
//! La sécurité repose sur :
//! - Isolation de processus (seul KM accède aux fichiers)
//! - Contrôle d'accès via arbitrage
//! - Audit complet des opérations
//! - Permissions fichiers restrictives

use rusqlite::{params, Connection, OpenFlags, params_from_iter};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

use crate::errors::ServiceError;

/// Gestionnaire de bases de données isolées.
pub struct EncryptedDatabase {
    /// Répertoire racine des données
    data_dir: PathBuf,
    /// Cache des connexions ouvertes (protégé par mutex async)
    connections: Arc<RwLock<HashMap<String, Arc<Mutex<Connection>>>>>,
}

impl EncryptedDatabase {
    /// Crée un nouveau gestionnaire de bases de données.
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
        
        Ok(Self {
            data_dir,
            connections: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Ouvre ou récupère une connexion à une base de données.
    pub async fn get_connection(&self, database: &str) -> Result<Arc<Mutex<Connection>>, ServiceError> {
        // Vérifier le cache
        {
            let connections = self.connections.read().await;
            if let Some(conn) = connections.get(database) {
                return Ok(conn.clone());
            }
        }

        // Ouvrir la base de données
        let conn = self.open_database(database)?;
        let conn = Arc::new(Mutex::new(conn));

        // Mettre en cache
        {
            let mut connections = self.connections.write().await;
            connections.insert(database.to_string(), conn.clone());
        }

        Ok(conn)
    }

    /// Ouvre une base de données.
    fn open_database(&self, database: &str) -> Result<Connection, ServiceError> {
        // Valider le nom de la base
        if !Self::is_valid_database_name(database) {
            return Err(ServiceError::Validation(format!(
                "Invalid database name: {}",
                database
            )));
        }

        let db_path = self.data_dir.join(format!("{}.db", database));
        
        // Ouvrir avec des flags restrictifs
        let conn = Connection::open_with_flags(
            &db_path,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .map_err(|e| ServiceError::Database(format!("Cannot open database: {}", e)))?;

        // Configuration de sécurité
        conn.execute_batch(
            r#"
            PRAGMA foreign_keys = ON;
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            PRAGMA busy_timeout = 5000;
            "#,
        )
        .map_err(|e| ServiceError::Database(format!("Cannot configure database: {}", e)))?;

        // Initialiser les tables système
        self.ensure_system_tables(&conn)?;

        // Définir les permissions du fichier DB
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
    fn ensure_system_tables(&self, conn: &Connection) -> Result<(), ServiceError> {
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS _kindmother_meta (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            );

            CREATE TABLE IF NOT EXISTS _kindmother_audit (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL DEFAULT (datetime('now')),
                operator_id TEXT NOT NULL,
                operation TEXT NOT NULL,
                intent TEXT,
                sql_hash TEXT,
                rows_affected INTEGER,
                success INTEGER NOT NULL
            );

            INSERT OR IGNORE INTO _kindmother_meta (key, value) 
            VALUES ('schema_version', '1');
            "#,
        )
        .map_err(|e| ServiceError::Database(format!("Cannot create system tables: {}", e)))?;

        Ok(())
    }

    /// Exécute une requête SELECT.
    pub async fn query(
        &self,
        database: &str,
        sql: &str,
        params: Vec<String>,
    ) -> Result<Vec<HashMap<String, serde_json::Value>>, ServiceError> {
        let conn = self.get_connection(database).await?;
        let conn = conn.lock().await;

        let mut stmt = conn
            .prepare(sql)
            .map_err(|e| ServiceError::Database(format!("Query prepare failed: {}", e)))?;

        let column_names: Vec<String> = stmt
            .column_names()
            .iter()
            .map(|s| s.to_string())
            .collect();

        let rows_iter = stmt
            .query_map(params_from_iter(params), |row| {
                let mut map = HashMap::new();
                for (i, name) in column_names.iter().enumerate() {
                    let value = match row.get_ref(i) {
                        Ok(rusqlite::types::ValueRef::Null) => serde_json::Value::Null,
                        Ok(rusqlite::types::ValueRef::Integer(i)) => {
                            serde_json::Value::Number(i.into())
                        }
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

    /// Exécute une requête d'écriture (INSERT, UPDATE, DELETE).
    pub async fn execute(
        &self,
        database: &str,
        operator_id: &str,
        sql: &str,
        params: Vec<String>,
        intent: &str,
    ) -> Result<(u64, Option<i64>), ServiceError> {
        let conn = self.get_connection(database).await?;
        let conn = conn.lock().await;

        let rows_affected = conn
            .execute(sql, params_from_iter(params))
            .map_err(|e| ServiceError::Database(format!("Execute failed: {}", e)))?;

        let last_insert_id = conn.last_insert_rowid();

        // Audit
        self.audit_operation(&conn, operator_id, "execute", intent, sql, rows_affected as u64, true)?;

        Ok((rows_affected as u64, Some(last_insert_id)))
    }

    /// Exécute une transaction.
    pub async fn transaction(
        &self,
        database: &str,
        operator_id: &str,
        operations: Vec<(String, Vec<String>)>,
        intent: &str,
    ) -> Result<Vec<u64>, ServiceError> {
        let conn = self.get_connection(database).await?;
        let mut conn = conn.lock().await;

        // Démarrer la transaction
        let tx = conn
            .transaction()
            .map_err(|e| ServiceError::Database(format!("Cannot begin transaction: {}", e)))?;

        let mut results = Vec::new();

        for (sql, params) in &operations {
            match tx.execute(sql, params_from_iter(params.clone())) {
                Ok(rows) => results.push(rows as u64),
                Err(e) => {
                    // Rollback automatique via drop de tx
                    return Err(ServiceError::Database(format!("Transaction failed: {}", e)));
                }
            }
        }

        // Commit
        tx.commit()
            .map_err(|e| ServiceError::Database(format!("Cannot commit: {}", e)))?;

        // Audit
        let total_affected: u64 = results.iter().sum();
        self.audit_operation(&conn, operator_id, "transaction", intent, "COMMIT", total_affected, true)?;

        Ok(results)
    }

    /// Enregistre une opération dans l'audit.
    fn audit_operation(
        &self,
        conn: &Connection,
        operator_id: &str,
        operation: &str,
        intent: &str,
        sql: &str,
        rows_affected: u64,
        success: bool,
    ) -> Result<(), ServiceError> {
        use sha2::{Sha256, Digest};
        let sql_hash = format!("{:x}", Sha256::digest(sql.as_bytes()));

        conn.execute(
            r#"
            INSERT INTO _kindmother_audit 
            (operator_id, operation, intent, sql_hash, rows_affected, success)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
            params![
                operator_id,
                operation,
                intent,
                sql_hash,
                rows_affected as i64,
                if success { 1 } else { 0 },
            ],
        )
        .map_err(|e| ServiceError::Database(format!("Audit failed: {}", e)))?;

        Ok(())
    }

    /// Obtient les informations sur une base de données.
    pub async fn get_info(&self, database: &str) -> Result<DatabaseInfo, ServiceError> {
        let db_path = self.data_dir.join(format!("{}.db", database));

        if !db_path.exists() {
            return Err(ServiceError::DatabaseNotFound(database.to_string()));
        }

        let size_bytes = std::fs::metadata(&db_path).map(|m| m.len()).unwrap_or(0);

        let conn = self.get_connection(database).await?;
        let conn = conn.lock().await;

        // Compter les tables
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
            encrypted: false, // SQLite standard n'est pas chiffré
        })
    }
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
