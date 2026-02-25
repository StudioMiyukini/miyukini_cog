//! Auth forum unifiée avec Miyukini Central.
//!
//! Copie serveur des profils Central (email, password_hash, pseudonyme) pour
//! validation des connexions forum. Hachage Argon2id (migration progressive depuis SHA256).

use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::Path;
use std::sync::Mutex;
use tracing::{debug, info, warn};

/// Profil forum (exposé après validation).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumProfile {
    /// ID profil Central (UUID).
    pub id: String,
    pub email: String,
    pub pseudonyme: Option<String>,
}

/// Stockage SQLite des profils synchronisés depuis Central.
pub struct ForumAuthStore {
    conn: Mutex<rusqlite::Connection>,
}

impl ForumAuthStore {
    /// Ouvre ou crée la base au chemin donné.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, String> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let conn = rusqlite::Connection::open(path).map_err(|e| e.to_string())?;
        let store = Self {
            conn: Mutex::new(conn),
        };
        store.init_schema().map_err(|e| e.to_string())?;
        Ok(store)
    }

    fn init_schema(&self) -> Result<(), String> {
        let conn = self.conn.lock().map_err(|_| "store lock poisoned".to_string())?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS forum_profiles (
                central_id TEXT PRIMARY KEY,
                email TEXT NOT NULL,
                password_hash TEXT NOT NULL,
                pseudonyme TEXT,
                updated_at INTEGER NOT NULL
            );
            CREATE UNIQUE INDEX IF NOT EXISTS idx_forum_profiles_email ON forum_profiles(email);",
        ).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Hash mot de passe avec Argon2id.
    fn hash_password_argon2(password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut rand::thread_rng());
        let argon2 = argon2::Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|e| format!("argon2 hash error: {}", e))
    }

    /// Vérifie un mot de passe contre un hash stocké (Argon2id ou SHA256 legacy).
    fn verify_password(password: &str, stored_hash: &str) -> bool {
        if stored_hash.starts_with("$argon2") {
            if let Ok(parsed) = PasswordHash::new(stored_hash) {
                return argon2::Argon2::default()
                    .verify_password(password.as_bytes(), &parsed)
                    .is_ok();
            }
            false
        } else {
            let mut h = Sha256::new();
            h.update(password.as_bytes());
            let sha_hex = format!("{:x}", h.finalize());
            sha_hex == stored_hash
        }
    }

    /// Migre un hash SHA256 legacy vers Argon2id (écriture en base).
    fn upgrade_hash_if_legacy(
        conn: &rusqlite::Connection,
        central_id: &str,
        password: &str,
        stored_hash: &str,
    ) {
        if stored_hash.starts_with("$argon2") {
            return;
        }
        if let Ok(new_hash) = Self::hash_password_argon2(password) {
            let _ = conn.execute(
                "UPDATE forum_profiles SET password_hash = ?1 WHERE central_id = ?2",
                rusqlite::params![new_hash, central_id],
            );
            info!("Upgraded password hash to Argon2id for central_id={}", central_id);
        }
    }

    /// Valide email + mot de passe ; retourne le profil si OK.
    /// Supporte Argon2id et SHA256 legacy (migration progressive).
    pub fn validate(&self, email: &str, password: &str) -> Result<Option<ForumProfile>, String> {
        let email = email.trim().to_lowercase();
        if email.is_empty() || password.is_empty() {
            return Ok(None);
        }
        let conn = self.conn.lock().map_err(|_| "store lock poisoned".to_string())?;
        let mut stmt = conn.prepare(
            "SELECT central_id, email, pseudonyme, password_hash FROM forum_profiles WHERE email = ?1",
        ).map_err(|e| e.to_string())?;
        let row = stmt.query_row(rusqlite::params![email], |row| {
            Ok((
                ForumProfile {
                    id: row.get::<_, String>(0)?,
                    email: row.get::<_, String>(1)?,
                    pseudonyme: row.get::<_, Option<String>>(2)?,
                },
                row.get::<_, String>(3)?,
            ))
        });
        match row {
            Ok((profile, stored_hash)) => {
                if Self::verify_password(password, &stored_hash) {
                    Self::upgrade_hash_if_legacy(&conn, &profile.id, password, &stored_hash);
                    Ok(Some(profile))
                } else {
                    Ok(None)
                }
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }

    /// Met à jour ou insère un profil (sync depuis Central).
    pub fn sync_profile(
        &self,
        central_id: &str,
        email: &str,
        password_hash: &str,
        pseudonyme: Option<&str>,
    ) -> Result<(), String> {
        let email = email.trim().to_lowercase();
        if email.is_empty() || central_id.is_empty() {
            return Err("central_id et email requis".to_string());
        }
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        let conn = self.conn.lock().map_err(|_| "store lock poisoned".to_string())?;
        conn.execute(
            "INSERT INTO forum_profiles (central_id, email, password_hash, pseudonyme, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(central_id) DO UPDATE SET
               email = excluded.email,
               password_hash = excluded.password_hash,
               pseudonyme = excluded.pseudonyme,
               updated_at = excluded.updated_at",
            rusqlite::params![central_id, email, password_hash, pseudonyme, now],
        ).map_err(|e| e.to_string())?;
        debug!("Forum profile synced: central_id={}", central_id);
        Ok(())
    }
}

/// Corps JSON pour POST /api/auth/forum/validate
#[derive(Debug, Deserialize)]
pub struct ValidateRequest {
    pub email: String,
    pub password: String,
}

/// Corps JSON pour POST /api/auth/forum/sync (appelé par Central ou outil de sync)
#[derive(Debug, Deserialize)]
pub struct SyncRequest {
    pub central_id: String,
    pub email: String,
    /// Hash du mot de passe (Argon2id ou SHA256 legacy depuis Central).
    pub password_hash: String,
    pub pseudonyme: Option<String>,
}

/// Traite une requête API auth forum (POST body déjà lu).
pub fn handle_api(
    path: &str,
    body: &[u8],
    store: &ForumAuthStore,
) -> super::server::RouteResponse {
    let path_clean = path.trim_end_matches('/');
    let body_str = match std::str::from_utf8(body) {
        Ok(s) => s.trim(),
        Err(_) => {
            return json_response(400, r#"{"ok":false,"error":"body must be UTF-8"}"#);
        }
    };

    if path_clean == "/api/auth/forum/validate" || path_clean == "/api/auth/forum/validate/" {
        if body_str.len() > 4096 {
            return json_response(400, r#"{"ok":false,"error":"payload too large"}"#);
        }
        let req: ValidateRequest = match serde_json::from_str(body_str) {
            Ok(r) => r,
            Err(_) => {
                return json_response(400, r#"{"ok":false,"error":"invalid json"}"#);
            }
        };
        match store.validate(&req.email, &req.password) {
            Ok(Some(profile)) => {
                let json = serde_json::json!({
                    "ok": true,
                    "profile": {
                        "id": profile.id,
                        "email": profile.email,
                        "pseudonyme": profile.pseudonyme
                    }
                });
                return json_response(200, &serde_json::to_string(&json).unwrap_or_default());
            }
            Ok(None) => {
                return json_response(401, r#"{"ok":false,"error":"invalid_credentials"}"#);
            }
            Err(e) => {
                warn!("Forum auth validate error: {}", e);
                return json_response(500, r#"{"ok":false,"error":"internal"}"#);
            }
        }
    }

    if path_clean == "/api/auth/forum/sync" || path_clean == "/api/auth/forum/sync/" {
        if body_str.len() > 4096 {
            return json_response(400, r#"{"ok":false,"error":"payload too large"}"#);
        }
        let req: SyncRequest = match serde_json::from_str(body_str) {
            Ok(r) => r,
            Err(_) => {
                return json_response(400, r#"{"ok":false,"error":"invalid json"}"#);
            }
        };
        match store.sync_profile(
            &req.central_id,
            &req.email,
            &req.password_hash,
            req.pseudonyme.as_deref(),
        ) {
            Ok(()) => {
                return json_response(200, r#"{"ok":true}"#);
            }
            Err(e) => {
                return json_response(400, &format!(r#"{{"ok":false,"error":"{}"}}"#, e));
            }
        }
    }

    json_response(404, r#"{"ok":false,"error":"not_found"}"#)
}

fn json_response(status: u16, body: &str) -> super::server::RouteResponse {
    let status_str = match status {
        200 => "200 OK",
        400 => "400 Bad Request",
        401 => "401 Unauthorized",
        404 => "404 Not Found",
        500 => "500 Internal Server Error",
        _ => "500 Internal Server Error",
    };
    super::server::RouteResponse::Normal {
        status: status_str.to_string(),
        content_type: "application/json; charset=utf-8".to_string(),
        body: body.to_string(),
    }
}
