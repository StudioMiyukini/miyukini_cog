//! @id     origin_web_forum_auth
//! @do     definir_max_failed_attempts
//! @role   securite
//! @layer  app
//! @human  origin — module « web forum auth ».

//! Auth unifiée Miyukini Central — profils Central via Origin.
//!
//! Copie serveur des profils Central (email, password_hash, pseudonyme) pour
//! validation des connexions forum. Hachage Argon2id exclusivement : tout
//! hash qui n'est pas au format `$argon2id$` est refusé à l'écriture et à la
//! vérification (le format SHA-256 legacy n'est plus accepté).
//!
//! Sécurité renforcée (fail-closed) :
//! - Rate limiting par IP (via couche réseau) et par compte (lockout).
//! - Verrouillage temporaire après N tentatives échouées.
//! - `/api/auth/forum/sync` : canal machine-à-machine authentifié par
//!   HMAC-SHA256 (corps + chemin + horodatage ±5 min + nonce anti-rejeu),
//!   secret partagé `MIYUKINI_FORUM_SYNC_SECRET`. Secret absent → route 503.
//! - `/api/auth/security/info` retirée du routage public (aucun mécanisme
//!   de session admin n'est partagé avec le serveur web ; voir admin/).
//! - Session tokens opaques (HMAC-SHA256, comparaison en temps constant).

use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use serde::{Deserialize, Serialize};
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Mutex;
use tracing::{debug, warn};

// ---------------------------------------------------------------------------
// Constantes sécurité
// ---------------------------------------------------------------------------

/// Nombre max de tentatives échouées avant verrouillage du compte.
const MAX_FAILED_ATTEMPTS: u32 = 5;
/// Durée de verrouillage en secondes (15 min).
const LOCKOUT_DURATION_SECS: u64 = 900;
/// Durée de validité d'un session token (1 heure).
const SESSION_TOKEN_VALIDITY_SECS: u64 = 3600;
/// Fenêtre de tolérance de l'horodatage des requêtes de sync (± 5 min).
const SYNC_TIMESTAMP_WINDOW_SECS: u64 = 300;
/// Nombre max de nonces mémorisés (protection mémoire).
const SYNC_NONCE_CACHE_MAX: usize = 10_000;
/// Variable d'environnement portant le secret partagé de sync Central → Origin.
pub const SYNC_SECRET_ENV: &str = "MIYUKINI_FORUM_SYNC_SECRET";
/// Longueur minimale du secret de sync (octets).
const SYNC_SECRET_MIN_LEN: usize = 32;

type HmacSha256 = Hmac<Sha256>;

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
    /// Secret partagé de sync Central → Origin (None = route sync désactivée).
    sync_secret: Option<Vec<u8>>,
    /// Nonces de sync déjà vus (nonce -> expiration epoch), anti-rejeu.
    sync_nonces: Mutex<HashMap<String, u64>>,
}

/// Issue de l'authentification d'une requête de sync.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyncAuthError {
    /// Aucun secret configuré : route désactivée (503).
    Disabled,
    /// En-têtes manquants / signature invalide / horodatage hors fenêtre / rejeu (401).
    Unauthorized,
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
            sync_secret: load_sync_secret_from_env(),
            sync_nonces: Mutex::new(HashMap::new()),
        };
        if store.sync_secret.is_none() {
            warn!(
                "{} absent ou trop court (< {} octets) : /api/auth/forum/sync désactivé",
                SYNC_SECRET_ENV, SYNC_SECRET_MIN_LEN
            );
        }
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
                last_login_at INTEGER,
                failed_attempts INTEGER NOT NULL DEFAULT 0
            );
            CREATE UNIQUE INDEX IF NOT EXISTS idx_forum_profiles_email ON forum_profiles(email);",
        )
        .map_err(|e| e.to_string())?;
        // Migration : ajouter les nouvelles colonnes si absentes.
        for col in &[
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

    /// Un hash est acceptable uniquement s'il est un hash PHC `$argon2id$` valide.
    #[must_use]
    pub fn is_valid_argon2id_hash(hash: &str) -> bool {
        hash.starts_with("$argon2id$") && PasswordHash::new(hash).is_ok()
    }

    /// Vérifie un mot de passe contre un hash stocké (Argon2id uniquement).
    /// Tout autre format (dont SHA-256 legacy) est refusé et journalisé.
    fn verify_password(password: &str, stored_hash: &str) -> bool {
        if !stored_hash.starts_with("$argon2id$") {
            warn!("Hash non argon2id refusé à la vérification (format legacy ?) : re-sync requis");
            return false;
        }
        match PasswordHash::new(stored_hash) {
            Ok(parsed) => argon2::Argon2::default()
                .verify_password(password.as_bytes(), &parsed)
                .is_ok(),
            Err(_) => false,
        }
    }

    // ----- Sécurité : session tokens -----

    fn session_mac(&self, central_id: &str, expires_at: u64) -> HmacSha256 {
        let mut mac = <HmacSha256 as Mac>::new_from_slice(&self.hmac_key)
            .expect("HMAC accepte toute longueur de clé");
        mac.update(b"miyukini-forum-session-v1\n");
        mac.update(&(central_id.len() as u64).to_be_bytes());
        mac.update(central_id.as_bytes());
        mac.update(&expires_at.to_be_bytes());
        mac
    }

    /// Génère un session token opaque (HMAC-SHA256 du central_id + expiration).
    fn generate_session_token(&self, central_id: &str) -> (String, u64) {
        let expires_at = epoch_secs() + SESSION_TOKEN_VALIDITY_SECS;
        let tag = self.session_mac(central_id, expires_at).finalize().into_bytes();
        (hex::encode(tag), expires_at)
    }

    /// Vérifie un session token (HMAC, comparaison en temps constant).
    pub fn verify_session_token(
        &self,
        central_id: &str,
        token: &str,
        expires_at: u64,
    ) -> bool {
        if epoch_secs() > expires_at {
            return false;
        }
        let Ok(provided) = hex::decode(token) else {
            return false;
        };
        self.session_mac(central_id, expires_at)
            .verify_slice(&provided)
            .is_ok()
    }

    // ----- Sécurité : canal de sync authentifié -----

    /// Active (ou remplace) le secret de sync (utile pour la configuration et les tests).
    #[must_use]
    pub fn with_sync_secret(mut self, secret: &[u8]) -> Self {
        self.sync_secret = if secret.len() >= SYNC_SECRET_MIN_LEN {
            Some(secret.to_vec())
        } else {
            None
        };
        self
    }

    /// Indique si le canal de sync est activé.
    #[must_use]
    pub fn sync_enabled(&self) -> bool {
        self.sync_secret.is_some()
    }

    fn sync_mac(secret: &[u8], path: &str, timestamp: u64, nonce: &str, body: &[u8]) -> HmacSha256 {
        let mut mac = <HmacSha256 as Mac>::new_from_slice(secret)
            .expect("HMAC accepte toute longueur de clé");
        mac.update(b"v1\n");
        mac.update(path.as_bytes());
        mac.update(b"\n");
        mac.update(timestamp.to_string().as_bytes());
        mac.update(b"\n");
        mac.update(nonce.as_bytes());
        mac.update(b"\n");
        mac.update(body);
        mac
    }

    /// Calcule la signature attendue : HMAC-SHA256(secret, "v1\n" path "\n" ts "\n" nonce "\n" body), en hex.
    #[must_use]
    pub fn compute_sync_signature(
        secret: &[u8],
        path: &str,
        timestamp: u64,
        nonce: &str,
        body: &[u8],
    ) -> String {
        hex::encode(Self::sync_mac(secret, path, timestamp, nonce, body).finalize().into_bytes())
    }

    /// Authentifie une requête de sync (signature, fenêtre temporelle, nonce non rejoué).
    pub fn authenticate_sync(
        &self,
        path: &str,
        body: &[u8],
        timestamp: Option<&str>,
        nonce: Option<&str>,
        signature: Option<&str>,
    ) -> Result<(), SyncAuthError> {
        let secret = self.sync_secret.as_ref().ok_or(SyncAuthError::Disabled)?;
        let (Some(ts_str), Some(nonce), Some(sig)) = (timestamp, nonce, signature) else {
            return Err(SyncAuthError::Unauthorized);
        };
        let ts: u64 = ts_str.trim().parse().map_err(|_| SyncAuthError::Unauthorized)?;
        let nonce = nonce.trim();
        if nonce.len() < 8
            || nonce.len() > 128
            || !nonce
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
        {
            return Err(SyncAuthError::Unauthorized);
        }
        let provided = hex::decode(sig.trim()).map_err(|_| SyncAuthError::Unauthorized)?;

        // 1. Signature (avant toute écriture dans le cache de nonces).
        let mac = Self::sync_mac(secret, path, ts, nonce, body);
        if mac.verify_slice(&provided).is_err() {
            warn!("Sync forum : signature invalide");
            return Err(SyncAuthError::Unauthorized);
        }

        // 2. Fenêtre temporelle.
        let now = epoch_secs();
        if now.abs_diff(ts) > SYNC_TIMESTAMP_WINDOW_SECS {
            warn!("Sync forum : horodatage hors fenêtre");
            return Err(SyncAuthError::Unauthorized);
        }

        // 3. Anti-rejeu.
        let mut cache = self.sync_nonces.lock().unwrap_or_else(|e| e.into_inner());
        cache.retain(|_, exp| *exp > now);
        if cache.contains_key(nonce) {
            warn!("Sync forum : rejeu de nonce refusé");
            return Err(SyncAuthError::Unauthorized);
        }
        if cache.len() >= SYNC_NONCE_CACHE_MAX {
            warn!("Sync forum : cache de nonces saturé");
            return Err(SyncAuthError::Unauthorized);
        }
        cache.insert(nonce.to_string(), now + 2 * SYNC_TIMESTAMP_WINDOW_SECS);
        Ok(())
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
    /// Argon2id uniquement (le SHA256 legacy est refusé).
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
                "SELECT central_id, email, pseudonyme, password_hash
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
            ))
        });
        match row {
            Ok((profile, stored_hash)) => {
                // Vérifier le verrouillage
                if self.is_account_locked(&profile.id) {
                    warn!("Login attempt on locked account: {}", profile.id);
                    return Ok(None);
                }
                if Self::verify_password(password, &stored_hash) {
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
    ///
    /// Privé au module : seul `handle_api` l'appelle, après `authenticate_sync`.
    /// Refuse tout hash qui n'est pas `$argon2id$`.
    fn sync_profile(
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
        if !Self::is_valid_argon2id_hash(password_hash) {
            warn!(
                "Sync refusée : hash non argon2id pour central_id={}",
                central_id
            );
            return Err("password_hash doit être au format $argon2id$".to_string());
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

    /// Retourne les informations de sécurité d'un profil (pour audit).
    /// Non exposé par le routage public (réservé à un usage interne/admin).
    #[allow(dead_code)]
    pub fn get_security_info(&self, central_id: &str) -> Result<Option<serde_json::Value>, String> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| "store lock poisoned".to_string())?;
        let mut stmt = conn
            .prepare(
                "SELECT email, last_login_at, failed_attempts,
                        CASE WHEN password_hash LIKE '$argon2%' THEN 'argon2id' ELSE 'sha256_legacy' END as hash_algo
                 FROM forum_profiles WHERE central_id = ?1",
            )
            .map_err(|e| e.to_string())?;
        let row = stmt.query_row(rusqlite::params![central_id], |row| {
            Ok(serde_json::json!({
                "central_id": central_id,
                "email": row.get::<_, String>(0)?,
                "last_login_at": row.get::<_, Option<i64>>(1)?,
                "failed_attempts": row.get::<_, i64>(2)?,
                "hash_algorithm": row.get::<_, String>(3)?,
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
    /// Hash du mot de passe (Argon2id `$argon2id$` obligatoire).
    pub password_hash: String,
    pub pseudonyme: Option<String>,
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
pub fn handle_api(
    path: &str,
    body: &[u8],
    headers: &[String],
    store: &ForumAuthStore,
) -> super::server::RouteResponse {
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
        // Authentification machine-à-machine obligatoire (fail-closed).
        match store.authenticate_sync(
            path_clean,
            body,
            header_value(headers, "x-miyu-timestamp").as_deref(),
            header_value(headers, "x-miyu-nonce").as_deref(),
            header_value(headers, "x-miyu-signature").as_deref(),
        ) {
            Ok(()) => {}
            Err(SyncAuthError::Disabled) => {
                return json_response(503, r#"{"ok":false,"error":"sync_disabled"}"#);
            }
            Err(SyncAuthError::Unauthorized) => {
                return json_response(401, r#"{"ok":false,"error":"unauthorized"}"#);
            }
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
                warn!("Sync forum refusée : {}", e);
                return json_response(400, r#"{"ok":false,"error":"sync_rejected"}"#);
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

    // --- POST /api/auth/security/info : retiré du routage public ---
    // Divulguait email / état de compte sans authentification. Aucune session
    // admin n'est partagée avec le serveur web : la route est supprimée
    // (404 via le fallback ci-dessous). `get_security_info` reste disponible
    // pour un futur appel interne authentifié.

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
        503 => "503 Service Unavailable",
        _ => "500 Internal Server Error",
    };
    super::server::RouteResponse::Normal {
        status: status_str.to_string(),
        content_type: "application/json; charset=utf-8".to_string(),
        body: body.to_string(),
    }
}

/// Charge le secret de sync depuis l'environnement (None si absent/trop court).
fn load_sync_secret_from_env() -> Option<Vec<u8>> {
    let v = std::env::var(SYNC_SECRET_ENV).ok()?;
    let v = v.trim();
    if v.len() >= SYNC_SECRET_MIN_LEN {
        Some(v.as_bytes().to_vec())
    } else {
        None
    }
}

/// Valeur d'un en-tête HTTP (nom insensible à la casse).
fn header_value(headers: &[String], name: &str) -> Option<String> {
    for line in headers {
        if let Some((k, v)) = line.split_once(':') {
            if k.trim().eq_ignore_ascii_case(name) {
                return Some(v.trim().to_string());
            }
        }
    }
    None
}

/// Comparaison de secrets en temps constant, indépendante de la longueur
/// (les deux entrées sont d'abord condensées en SHA-256).
pub(crate) fn secrets_equal(a: &str, b: &str) -> bool {
    let ha = Sha256::digest(a.as_bytes());
    let hb = Sha256::digest(b.as_bytes());
    constant_time_eq(&ha, &hb)
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

#[cfg(test)]
mod tests {
    use super::*;

    const SECRET: &[u8] = b"0123456789abcdef0123456789abcdef-secret";

    fn temp_store() -> ForumAuthStore {
        let path = std::env::temp_dir().join(format!("forum_auth_test_{}.db", uuid::Uuid::new_v4()));
        ForumAuthStore::open(path).expect("store").with_sync_secret(SECRET)
    }

    fn sync_body(central_id: &str, email: &str, hash: &str) -> String {
        serde_json::json!({"central_id": central_id, "email": email, "password_hash": hash})
            .to_string()
    }

    fn signed_headers(body: &str, ts: u64, nonce: &str) -> Vec<String> {
        let sig = ForumAuthStore::compute_sync_signature(
            SECRET,
            "/api/auth/forum/sync",
            ts,
            nonce,
            body.as_bytes(),
        );
        vec![
            format!("X-Miyu-Timestamp: {ts}\r\n"),
            format!("X-Miyu-Nonce: {nonce}\r\n"),
            format!("X-Miyu-Signature: {sig}\r\n"),
        ]
    }

    fn status_of(r: &super::super::server::RouteResponse) -> String {
        match r {
            super::super::server::RouteResponse::Normal { status, .. } => status.clone(),
            _ => String::new(),
        }
    }

    #[test]
    fn sync_sans_signature_refusee() {
        let store = temp_store();
        let hash = ForumAuthStore::hash_password_argon2("pw").unwrap();
        let body = sync_body("c1", "a@b.c", &hash);
        let r = handle_api("/api/auth/forum/sync", body.as_bytes(), &[], &store);
        assert!(status_of(&r).starts_with("401"));
        assert!(store.validate("a@b.c", "pw").unwrap().is_none());
    }

    #[test]
    fn sync_sans_secret_configure_renvoie_503() {
        let path = std::env::temp_dir().join(format!("forum_auth_test_{}.db", uuid::Uuid::new_v4()));
        let store = ForumAuthStore::open(path).unwrap().with_sync_secret(b"");
        let r = handle_api("/api/auth/forum/sync", b"{}", &[], &store);
        assert!(status_of(&r).starts_with("503"));
    }

    #[test]
    fn sync_signature_valide_acceptee_puis_rejeu_refuse() {
        let store = temp_store();
        let hash = ForumAuthStore::hash_password_argon2("pw").unwrap();
        let body = sync_body("c1", "a@b.c", &hash);
        let h = signed_headers(&body, epoch_secs(), "nonce-abcdef01");
        let r = handle_api("/api/auth/forum/sync", body.as_bytes(), &h, &store);
        assert!(status_of(&r).starts_with("200"), "{}", status_of(&r));
        assert!(store.validate("a@b.c", "pw").unwrap().is_some());
        // Rejeu exact.
        let r2 = handle_api("/api/auth/forum/sync", body.as_bytes(), &h, &store);
        assert!(status_of(&r2).starts_with("401"));
    }

    #[test]
    fn sync_corps_altere_ou_horodatage_perime_refuse() {
        let store = temp_store();
        let hash = ForumAuthStore::hash_password_argon2("pw").unwrap();
        let body = sync_body("c1", "a@b.c", &hash);
        let h = signed_headers(&body, epoch_secs(), "nonce-abcdef02");
        let tampered = sync_body("c1", "evil@b.c", &hash);
        let r = handle_api("/api/auth/forum/sync", tampered.as_bytes(), &h, &store);
        assert!(status_of(&r).starts_with("401"));
        let old = signed_headers(&body, epoch_secs() - 3600, "nonce-abcdef03");
        let r = handle_api("/api/auth/forum/sync", body.as_bytes(), &old, &store);
        assert!(status_of(&r).starts_with("401"));
    }

    #[test]
    fn sync_hash_non_argon2_refuse() {
        let store = temp_store();
        let legacy = format!("{:x}", Sha256::digest(b"pw"));
        let body = sync_body("c1", "a@b.c", &legacy);
        let h = signed_headers(&body, epoch_secs(), "nonce-abcdef04");
        let r = handle_api("/api/auth/forum/sync", body.as_bytes(), &h, &store);
        assert!(status_of(&r).starts_with("400"));
        assert!(!ForumAuthStore::is_valid_argon2id_hash("$argon2i$v=19$m=1,t=1,p=1$YQ$YQ"));
        assert!(!ForumAuthStore::verify_password("pw", &legacy));
    }

    #[test]
    fn security_info_retire_du_routage() {
        let store = temp_store();
        let r = handle_api("/api/auth/security/info", br#"{"central_id":"c1"}"#, &[], &store);
        assert!(status_of(&r).starts_with("404"));
    }

    #[test]
    fn session_token_hmac_roundtrip() {
        let store = temp_store();
        let (tok, exp) = store.generate_session_token("c1");
        assert!(store.verify_session_token("c1", &tok, exp));
        assert!(!store.verify_session_token("c2", &tok, exp));
        assert!(!store.verify_session_token("c1", &tok, exp + 1));
        assert!(!store.verify_session_token("c1", "zz", exp));
    }
}
