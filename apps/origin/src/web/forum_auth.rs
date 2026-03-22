//! Auth unifiée Miyukini Central — profils Central + JayXpose via Origin.
//!
//! Copie serveur des profils Central (email, password_hash, pseudonyme) pour
//! validation des connexions forum ET JayXpose. Hachage Argon2id (migration
//! progressive depuis SHA256).
//!
//! Sécurité renforcée :
//! - Rate limiting par IP (via couche réseau) et par compte (lockout).
//! - Verrouillage temporaire après N tentatives échouées.
//! - Migration automatique SHA256 → Argon2id au login réussi.
//! - Session tokens opaques (HMAC-SHA256) pour les appels authentifiés.

use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use tracing::{debug, info, warn};

// ---------------------------------------------------------------------------
// Constantes sécurité
// ---------------------------------------------------------------------------

/// Nombre max de tentatives échouées avant verrouillage du compte.
const MAX_FAILED_ATTEMPTS: u32 = 5;
/// Durée de verrouillage en secondes (15 min).
const LOCKOUT_DURATION_SECS: u64 = 900;
/// Durée de validité d'un session token (1 heure).
const SESSION_TOKEN_VALIDITY_SECS: u64 = 3600;

// ---------------------------------------------------------------------------
// Types publics
// ---------------------------------------------------------------------------

/// Profil forum / Central (exposé après validation).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumProfile {
    /// ID profil Central (UUID).
    pub id: String,
    pub email: String,
    pub pseudonyme: Option<String>,
}

/// Résultat d'une validation avec session token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthValidationResult {
    pub profile: ForumProfile,
    /// Token de session opaque (hex, HMAC-SHA256).
    pub session_token: String,
    /// Timestamp d'expiration du token (epoch seconds).
    pub expires_at: u64,
    /// Indique si un profil JayXpose est lié.
    pub has_jayxpose_profile: bool,
}

/// État de verrouillage d'un compte.
#[derive(Debug, Clone, Default)]
struct AccountLockState {
    failed_attempts: u32,
    locked_until: Option<u64>,
}

// ---------------------------------------------------------------------------
// Store
// ---------------------------------------------------------------------------

/// Stockage SQLite des profils synchronisés depuis Central + sécurité.
pub struct ForumAuthStore {
    conn: Mutex<rusqlite::Connection>,
    /// État de verrouillage par central_id (en mémoire, reset au redémarrage).
    lock_state: Mutex<HashMap<String, AccountLockState>>,
    /// Clé HMAC pour la génération des session tokens.
    hmac_key: [u8; 32],
}

impl ForumAuthStore {
    /// Ouvre ou crée la base au chemin donné.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, String> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let conn = rusqlite::Connection::open(path).map_err(|e| e.to_string())?;
        // Générer une clé HMAC aléatoire au démarrage (tokens invalidés au restart = sécurité).
        let mut hmac_key = [0u8; 32];
        use rand::RngCore;
        rand::thread_rng().fill_bytes(&mut hmac_key);
        let store = Self {
            conn: Mutex::new(conn),
            lock_state: Mutex::new(HashMap::new()),
            hmac_key,
        };
        store.init_schema().map_err(|e| e.to_string())?;
        Ok(store)
    }

    fn init_schema(&self) -> Result<(), String> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| "store lock poisoned".to_string())?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS forum_profiles (
                central_id TEXT PRIMARY KEY,
                email TEXT NOT NULL,
                password_hash TEXT NOT NULL,
                pseudonyme TEXT,
                updated_at INTEGER NOT NULL,
                has_jayxpose INTEGER NOT NULL DEFAULT 0,
                last_login_at INTEGER,
                failed_attempts INTEGER NOT NULL DEFAULT 0
            );
            CREATE UNIQUE INDEX IF NOT EXISTS idx_forum_profiles_email ON forum_profiles(email);",
        )
        .map_err(|e| e.to_string())?;
        // Migration : ajouter les nouvelles colonnes si absentes.
        for col in &[
            ("has_jayxpose", "INTEGER NOT NULL DEFAULT 0"),
            ("last_login_at", "INTEGER"),
            ("failed_attempts", "INTEGER NOT NULL DEFAULT 0"),
        ] {
            let sql = format!(
                "ALTER TABLE forum_profiles ADD COLUMN {} {}",
                col.0, col.1
            );
            if let Err(e) = conn.execute(&sql, []) {
                if !e.to_string().contains("duplicate column name") {
                    return Err(e.to_string());
                }
            }
        }
        Ok(())
    }

    // ----- Sécurité : hashing -----

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
            info!(
                "Upgraded password hash to Argon2id for central_id={}",
                central_id
            );
        }
    }

    // ----- Sécurité : session tokens -----

    /// Génère un session token opaque (HMAC-SHA256 du central_id + timestamp).
    fn generate_session_token(&self, central_id: &str) -> (String, u64) {
        let now = epoch_secs();
        let expires_at = now + SESSION_TOKEN_VALIDITY_SECS;
        let mut mac = Sha256::new();
        mac.update(&self.hmac_key);
        mac.update(central_id.as_bytes());
        mac.update(&expires_at.to_be_bytes());
        let token = format!("{:x}", mac.finalize());
        (token, expires_at)
    }

    /// Vérifie un session token.
    pub fn verify_session_token(
        &self,
        central_id: &str,
        token: &str,
        expires_at: u64,
    ) -> bool {
        if epoch_secs() > expires_at {
            return false;
        }
        let mut mac = Sha256::new();
        mac.update(&self.hmac_key);
        mac.update(central_id.as_bytes());
        mac.update(&expires_at.to_be_bytes());
        let expected = format!("{:x}", mac.finalize());
        constant_time_eq(token.as_bytes(), expected.as_bytes())
    }

    // ----- Sécurité : account lockout -----

    /// Vérifie si un compte est verrouillé.
    fn is_account_locked(&self, central_id: &str) -> bool {
        let lock = self.lock_state.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(state) = lock.get(central_id) {
            if let Some(until) = state.locked_until {
                if epoch_secs() < until {
                    return true;
                }
            }
        }
        false
    }

    /// Enregistre une tentative échouée. Verrouille le compte si le seuil est atteint.
    fn record_failed_attempt(&self, central_id: &str) {
        let mut lock = self.lock_state.lock().unwrap_or_else(|e| e.into_inner());
        let state = lock.entry(central_id.to_string()).or_default();
        state.failed_attempts += 1;
        if state.failed_attempts >= MAX_FAILED_ATTEMPTS {
            state.locked_until = Some(epoch_secs() + LOCKOUT_DURATION_SECS);
            warn!(
                "Account locked for central_id={} ({} failed attempts)",
                central_id, state.failed_attempts
            );
        }
    }

    /// Réinitialise le compteur d'échecs après un login réussi.
    fn clear_failed_attempts(&self, central_id: &str) {
        let mut lock = self.lock_state.lock().unwrap_or_else(|e| e.into_inner());
        lock.remove(central_id);
    }

    // ----- Opérations publiques -----

    /// Valide email + mot de passe ; retourne le profil + session token si OK.
    /// Supporte Argon2id et SHA256 legacy (migration progressive).
    /// Applique le verrouillage de compte.
    pub fn validate(
        &self,
        email: &str,
        password: &str,
    ) -> Result<Option<AuthValidationResult>, String> {
        let email = email.trim().to_lowercase();
        if email.is_empty() || password.is_empty() {
            return Ok(None);
        }
        let conn = self
            .conn
            .lock()
            .map_err(|_| "store lock poisoned".to_string())?;
        let mut stmt = conn
            .prepare(
                "SELECT central_id, email, pseudonyme, password_hash, has_jayxpose
                 FROM forum_profiles WHERE email = ?1",
            )
            .map_err(|e| e.to_string())?;
        let row = stmt.query_row(rusqlite::params![email], |row| {
            Ok((
                ForumProfile {
                    id: row.get::<_, String>(0)?,
                    email: row.get::<_, String>(1)?,
                    pseudonyme: row.get::<_, Option<String>>(2)?,
                },
                row.get::<_, String>(3)?,
                row.get::<_, i64>(4).unwrap_or(0) != 0,
            ))
        });
        match row {
            Ok((profile, stored_hash, has_jayxpose)) => {
                // Vérifier le verrouillage
                if self.is_account_locked(&profile.id) {
                    warn!("Login attempt on locked account: {}", profile.id);
                    return Ok(None);
                }
                if Self::verify_password(password, &stored_hash) {
                    Self::upgrade_hash_if_legacy(&conn, &profile.id, password, &stored_hash);
                    // Mettre à jour last_login et reset failed_attempts en base
                    let now = epoch_secs() as i64;
                    let _ = conn.execute(
                        "UPDATE forum_profiles SET last_login_at = ?1, failed_attempts = 0 WHERE central_id = ?2",
                        rusqlite::params![now, profile.id],
                    );
                    self.clear_failed_attempts(&profile.id);
                    let (token, expires_at) = self.generate_session_token(&profile.id);
                    Ok(Some(AuthValidationResult {
                        profile,
                        session_token: token,
                        expires_at,
                        has_jayxpose_profile: has_jayxpose,
                    }))
                } else {
                    self.record_failed_attempt(&profile.id);
                    // Incrémenter en base aussi
                    let _ = conn.execute(
                        "UPDATE forum_profiles SET failed_attempts = failed_attempts + 1 WHERE central_id = ?1",
                        rusqlite::params![profile.id],
                    );
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
        let now = epoch_secs() as i64;
        let conn = self
            .conn
            .lock()
            .map_err(|_| "store lock poisoned".to_string())?;
        conn.execute(
            "INSERT INTO forum_profiles (central_id, email, password_hash, pseudonyme, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(central_id) DO UPDATE SET
               email = excluded.email,
               password_hash = excluded.password_hash,
               pseudonyme = excluded.pseudonyme,
               updated_at = excluded.updated_at",
            rusqlite::params![central_id, email, password_hash, pseudonyme, now],
        )
        .map_err(|e| e.to_string())?;
        debug!("Forum profile synced: central_id={}", central_id);
        Ok(())
    }

    /// Marque un profil Central comme ayant un profil JayXpose lié.
    pub fn set_jayxpose_linked(
        &self,
        central_id: &str,
        linked: bool,
    ) -> Result<(), String> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| "store lock poisoned".to_string())?;
        conn.execute(
            "UPDATE forum_profiles SET has_jayxpose = ?1 WHERE central_id = ?2",
            rusqlite::params![if linked { 1i64 } else { 0i64 }, central_id],
        )
        .map_err(|e| e.to_string())?;
        debug!(
            "JayXpose link updated: central_id={}, linked={}",
            central_id, linked
        );
        Ok(())
    }

    /// Sync un profil Central + liaison JayXpose en une seule opération.
    pub fn sync_profile_with_jayxpose(
        &self,
        central_id: &str,
        email: &str,
        password_hash: &str,
        pseudonyme: Option<&str>,
        has_jayxpose: bool,
    ) -> Result<(), String> {
        let email = email.trim().to_lowercase();
        if email.is_empty() || central_id.is_empty() {
            return Err("central_id et email requis".to_string());
        }
        let now = epoch_secs() as i64;
        let jx = if has_jayxpose { 1i64 } else { 0i64 };
        let conn = self
            .conn
            .lock()
            .map_err(|_| "store lock poisoned".to_string())?;
        conn.execute(
            "INSERT INTO forum_profiles (central_id, email, password_hash, pseudonyme, updated_at, has_jayxpose)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(central_id) DO UPDATE SET
               email = excluded.email,
               password_hash = excluded.password_hash,
               pseudonyme = excluded.pseudonyme,
               updated_at = excluded.updated_at,
               has_jayxpose = excluded.has_jayxpose",
            rusqlite::params![central_id, email, password_hash, pseudonyme, now, jx],
        )
        .map_err(|e| e.to_string())?;
        debug!(
            "Profile synced with jayxpose flag: central_id={}, has_jayxpose={}",
            central_id, has_jayxpose
        );
        Ok(())
    }

    /// Retourne les informations de sécurité d'un profil (pour audit).
    pub fn get_security_info(&self, central_id: &str) -> Result<Option<serde_json::Value>, String> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| "store lock poisoned".to_string())?;
        let mut stmt = conn
            .prepare(
                "SELECT email, has_jayxpose, last_login_at, failed_attempts,
                        CASE WHEN password_hash LIKE '$argon2%' THEN 'argon2id' ELSE 'sha256_legacy' END as hash_algo
                 FROM forum_profiles WHERE central_id = ?1",
            )
            .map_err(|e| e.to_string())?;
        let row = stmt.query_row(rusqlite::params![central_id], |row| {
            Ok(serde_json::json!({
                "central_id": central_id,
                "email": row.get::<_, String>(0)?,
                "has_jayxpose": row.get::<_, i64>(1)? != 0,
                "last_login_at": row.get::<_, Option<i64>>(2)?,
                "failed_attempts": row.get::<_, i64>(3)?,
                "hash_algorithm": row.get::<_, String>(4)?,
                "account_locked": false, // Mis à jour ci-dessous
            }))
        });
        match row {
            Ok(mut info) => {
                let locked = self.is_account_locked(central_id);
                if let Some(obj) = info.as_object_mut() {
                    obj.insert("account_locked".to_string(), serde_json::json!(locked));
                }
                Ok(Some(info))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }
}

// ---------------------------------------------------------------------------
// Requêtes API
// ---------------------------------------------------------------------------

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

/// Corps JSON pour POST /api/auth/profile/sync (sync complet avec JayXpose)
#[derive(Debug, Deserialize)]
pub struct ProfileSyncRequest {
    pub central_id: String,
    pub email: String,
    pub password_hash: String,
    pub pseudonyme: Option<String>,
    /// Indique si un profil JayXpose est lié côté Central.
    #[serde(default)]
    pub has_jayxpose: bool,
}

/// Corps JSON pour POST /api/auth/session/verify
#[derive(Debug, Deserialize)]
pub struct SessionVerifyRequest {
    pub central_id: String,
    pub session_token: String,
    pub expires_at: u64,
}

/// Corps JSON pour GET /api/auth/security/{central_id}
#[derive(Debug, Deserialize)]
pub struct SecurityInfoRequest {
    pub central_id: String,
}

/// Traite une requête API auth (POST body déjà lu).
pub fn handle_api(path: &str, body: &[u8], store: &ForumAuthStore) -> super::server::RouteResponse {
    let path_clean = path.trim_end_matches('/');
    let body_str = match std::str::from_utf8(body) {
        Ok(s) => s.trim(),
        Err(_) => {
            return json_response(400, r#"{"ok":false,"error":"body must be UTF-8"}"#);
        }
    };

    // --- POST /api/auth/forum/validate (+ /api/auth/validate alias) ---
    if path_clean == "/api/auth/forum/validate" || path_clean == "/api/auth/validate" {
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
            Ok(Some(result)) => {
                let json = serde_json::json!({
                    "ok": true,
                    "profile": {
                        "id": result.profile.id,
                        "email": result.profile.email,
                        "pseudonyme": result.profile.pseudonyme
                    },
                    "session_token": result.session_token,
                    "expires_at": result.expires_at,
                    "has_jayxpose_profile": result.has_jayxpose_profile
                });
                return json_response(200, &serde_json::to_string(&json).unwrap_or_default());
            }
            Ok(None) => {
                return json_response(401, r#"{"ok":false,"error":"invalid_credentials"}"#);
            }
            Err(e) => {
                warn!("Auth validate error: {}", e);
                return json_response(500, r#"{"ok":false,"error":"internal"}"#);
            }
        }
    }

    // --- POST /api/auth/forum/sync ---
    if path_clean == "/api/auth/forum/sync" {
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

    // --- POST /api/auth/profile/sync (sync complet Central + JayXpose) ---
    if path_clean == "/api/auth/profile/sync" {
        if body_str.len() > 4096 {
            return json_response(400, r#"{"ok":false,"error":"payload too large"}"#);
        }
        let req: ProfileSyncRequest = match serde_json::from_str(body_str) {
            Ok(r) => r,
            Err(_) => {
                return json_response(400, r#"{"ok":false,"error":"invalid json"}"#);
            }
        };
        match store.sync_profile_with_jayxpose(
            &req.central_id,
            &req.email,
            &req.password_hash,
            req.pseudonyme.as_deref(),
            req.has_jayxpose,
        ) {
            Ok(()) => {
                return json_response(200, r#"{"ok":true}"#);
            }
            Err(e) => {
                return json_response(400, &format!(r#"{{"ok":false,"error":"{}"}}"#, e));
            }
        }
    }

    // --- POST /api/auth/session/verify ---
    if path_clean == "/api/auth/session/verify" {
        if body_str.len() > 4096 {
            return json_response(400, r#"{"ok":false,"error":"payload too large"}"#);
        }
        let req: SessionVerifyRequest = match serde_json::from_str(body_str) {
            Ok(r) => r,
            Err(_) => {
                return json_response(400, r#"{"ok":false,"error":"invalid json"}"#);
            }
        };
        let valid = store.verify_session_token(&req.central_id, &req.session_token, req.expires_at);
        let json = serde_json::json!({ "ok": true, "valid": valid });
        return json_response(200, &serde_json::to_string(&json).unwrap_or_default());
    }

    // --- POST /api/auth/security/info ---
    if path_clean == "/api/auth/security/info" {
        if body_str.len() > 4096 {
            return json_response(400, r#"{"ok":false,"error":"payload too large"}"#);
        }
        let req: SecurityInfoRequest = match serde_json::from_str(body_str) {
            Ok(r) => r,
            Err(_) => {
                return json_response(400, r#"{"ok":false,"error":"invalid json"}"#);
            }
        };
        match store.get_security_info(&req.central_id) {
            Ok(Some(info)) => {
                let json = serde_json::json!({ "ok": true, "security": info });
                return json_response(200, &serde_json::to_string(&json).unwrap_or_default());
            }
            Ok(None) => {
                return json_response(404, r#"{"ok":false,"error":"profile_not_found"}"#);
            }
            Err(e) => {
                return json_response(500, &format!(r#"{{"ok":false,"error":"{}"}}"#, e));
            }
        }
    }

    json_response(404, r#"{"ok":false,"error":"not_found"}"#)
}

// ---------------------------------------------------------------------------
// Utilitaires
// ---------------------------------------------------------------------------

fn json_response(status: u16, body: &str) -> super::server::RouteResponse {
    let status_str = match status {
        200 => "200 OK",
        400 => "400 Bad Request",
        401 => "401 Unauthorized",
        404 => "404 Not Found",
        429 => "429 Too Many Requests",
        500 => "500 Internal Server Error",
        _ => "500 Internal Server Error",
    };
    super::server::RouteResponse::Normal {
        status: status_str.to_string(),
        content_type: "application/json; charset=utf-8".to_string(),
        body: body.to_string(),
    }
}

fn epoch_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Comparaison en temps constant pour éviter les timing attacks.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}
