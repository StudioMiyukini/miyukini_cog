//! `auth_op` — sessions web et jetons applicatifs WebDAV.
//!
//! En PR-2 (P3.a), les sessions et app-passwords sont **en mémoire**.
//! La persistance via `storage_kit` (tables `sessions` et `app_passwords`)
//! arrivera en PR-5 quand l'UI sera connectée.
//!
//! Le module fournit déjà les types et l'API qui survivront au branchement
//! KindMother — seul le RwLock interne sera remplacé par des requêtes SQL.

use std::collections::HashMap;
use std::sync::RwLock;

use crate::kits::crypto_kit::Key32;
use crate::kits::token_kit::{self, TokenKitError};

/// Représentation d'une session web active.
#[derive(Debug, Clone)]
pub struct Session {
    /// UUID de la session (cookie côté navigateur).
    pub id: String,
    /// Identité COG.
    pub user_id: String,
    /// Timestamp Unix de création.
    pub created_at: i64,
    /// Timestamp Unix d'expiration.
    pub expires_at: i64,
}

/// Métadonnée d'un app-password WebDAV (sans le raw token).
#[derive(Debug, Clone)]
pub struct AppPasswordRecord {
    /// UUID de l'app-password.
    pub id: String,
    /// Identité COG du propriétaire.
    pub user_id: String,
    /// Nom affichable (ex: "rclone", "Duplicati").
    pub name: String,
    /// SHA-256 hex du token brut.
    pub token_hash: String,
    /// Scopes JSON sérialisé (ex: `["webdav","backup_api"]`).
    pub scopes: Vec<String>,
    /// Timestamp de création.
    pub created_at: i64,
    /// Dernière utilisation observée.
    pub last_used_at: Option<i64>,
    /// Timestamp de révocation (None = actif).
    pub revoked_at: Option<i64>,
}

/// Erreurs de l'Opérateur Auth.
#[derive(Debug, thiserror::Error)]
pub enum AuthOpError {
    /// Session invalide ou expirée.
    #[error("session invalide ou expirée")]
    InvalidSession,
    /// App-password introuvable.
    #[error("app-password introuvable : {0}")]
    AppPasswordNotFound(String),
    /// App-password révoqué.
    #[error("app-password révoqué : {0}")]
    AppPasswordRevoked(String),
    /// Erreur du Kit token.
    #[error("token : {0}")]
    Token(#[from] TokenKitError),
    /// Aucun app-password ne correspond au token fourni.
    #[error("authentification refusée")]
    AuthDenied,
}

/// Opérateur Auth.
pub struct AuthOp {
    sessions: RwLock<HashMap<String, Session>>,
    app_passwords: RwLock<HashMap<String, AppPasswordRecord>>, // clé = token_hash
    token_secret: Key32,
}

impl AuthOp {
    /// Construit l'Opérateur. `token_secret` est typiquement dérivé via
    /// `crypto_kit::derive_key_hkdf` avec le contexte `b"jaycloud_token_kit_v1"`.
    #[must_use]
    pub fn new(token_secret: Key32) -> Self {
        Self {
            sessions: RwLock::new(HashMap::new()),
            app_passwords: RwLock::new(HashMap::new()),
            token_secret,
        }
    }

    // ─── Sessions ─────────────────────────────────────────────────────────

    /// Crée une session web. `now_unix` permet d'injecter une horloge pour les tests.
    pub fn create_session(&self, user_id: &str, ttl_seconds: i64, now_unix: i64) -> Session {
        let session = Session {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            created_at: now_unix,
            expires_at: now_unix + ttl_seconds,
        };
        self.sessions
            .write()
            .unwrap()
            .insert(session.id.clone(), session.clone());
        session
    }

    /// Vérifie une session par son ID cookie. `now_unix` doit être l'horloge courante.
    pub fn verify_session(&self, session_id: &str, now_unix: i64) -> Result<Session, AuthOpError> {
        let sessions = self.sessions.read().unwrap();
        let s = sessions
            .get(session_id)
            .cloned()
            .ok_or(AuthOpError::InvalidSession)?;
        if now_unix >= s.expires_at {
            return Err(AuthOpError::InvalidSession);
        }
        Ok(s)
    }

    /// Supprime une session (logout).
    pub fn delete_session(&self, session_id: &str) -> bool {
        self.sessions.write().unwrap().remove(session_id).is_some()
    }

    // ─── App-passwords ─────────────────────────────────────────────────────

    /// Crée un app-password. Renvoie le **raw token** (à afficher une fois
    /// à l'utilisateur) et le `AppPasswordRecord` persisté.
    pub fn create_app_password(
        &self,
        user_id: &str,
        name: &str,
        scopes: Vec<String>,
        now_unix: i64,
    ) -> (String, AppPasswordRecord) {
        let (raw_token, hash) = token_kit::generate(&self.token_secret);
        let record = AppPasswordRecord {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            name: name.to_string(),
            token_hash: hash.clone(),
            scopes,
            created_at: now_unix,
            last_used_at: None,
            revoked_at: None,
        };
        self.app_passwords
            .write()
            .unwrap()
            .insert(hash, record.clone());
        (raw_token, record)
    }

    /// Liste les app-passwords d'un utilisateur (révoqués inclus).
    pub fn list_app_passwords(&self, user_id: &str) -> Vec<AppPasswordRecord> {
        let passwords = self.app_passwords.read().unwrap();
        let mut out: Vec<_> = passwords
            .values()
            .filter(|p| p.user_id == user_id)
            .cloned()
            .collect();
        out.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        out
    }

    /// Révoque un app-password par son nom (premier match pour un user_id donné).
    pub fn revoke_app_password(
        &self,
        user_id: &str,
        name: &str,
        now_unix: i64,
    ) -> Result<(), AuthOpError> {
        let mut passwords = self.app_passwords.write().unwrap();
        let found = passwords
            .values_mut()
            .find(|p| p.user_id == user_id && p.name == name && p.revoked_at.is_none());
        match found {
            Some(p) => {
                p.revoked_at = Some(now_unix);
                Ok(())
            }
            None => Err(AuthOpError::AppPasswordNotFound(name.to_string())),
        }
    }

    /// Vérifie un raw token (HTTP Basic Auth WebDAV). Renvoie le record
    /// associé s'il est valide et non révoqué.
    ///
    /// Met à jour `last_used_at`.
    pub fn verify_app_password(
        &self,
        raw_token: &str,
        now_unix: i64,
    ) -> Result<AppPasswordRecord, AuthOpError> {
        // 1. Vérifie l'intégrité interne (préfixe + checksum HMAC).
        token_kit::verify_integrity(raw_token, &self.token_secret)?;

        // 2. Hash + lookup.
        let hash = token_kit::hash_token(raw_token);
        let mut passwords = self.app_passwords.write().unwrap();
        let record = passwords
            .get_mut(&hash)
            .ok_or(AuthOpError::AuthDenied)?;
        if record.revoked_at.is_some() {
            return Err(AuthOpError::AppPasswordRevoked(record.name.clone()));
        }
        record.last_used_at = Some(now_unix);
        Ok(record.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn op() -> AuthOp {
        AuthOp::new(Key32::generate())
    }

    // ── Sessions ────────────────────────────────────────────────────────

    #[test]
    fn create_and_verify_session() {
        let auth = op();
        let s = auth.create_session("user_42", 60, 1000);
        let v = auth.verify_session(&s.id, 1010).unwrap();
        assert_eq!(v.user_id, "user_42");
    }

    #[test]
    fn expired_session_rejected() {
        let auth = op();
        let s = auth.create_session("user_42", 60, 1000);
        let r = auth.verify_session(&s.id, 1061);
        assert!(matches!(r, Err(AuthOpError::InvalidSession)));
    }

    #[test]
    fn unknown_session_rejected() {
        let auth = op();
        let r = auth.verify_session("ghost", 1000);
        assert!(matches!(r, Err(AuthOpError::InvalidSession)));
    }

    #[test]
    fn delete_session_removes_it() {
        let auth = op();
        let s = auth.create_session("u", 60, 1000);
        assert!(auth.delete_session(&s.id));
        assert!(!auth.delete_session(&s.id)); // déjà supprimée
    }

    // ── App-passwords ────────────────────────────────────────────────────

    #[test]
    fn create_app_password_returns_raw_and_record() {
        let auth = op();
        let (raw, record) = auth.create_app_password(
            "user_42",
            "rclone",
            vec!["webdav".into()],
            1000,
        );
        assert!(raw.starts_with("mws-jc-"));
        assert_eq!(record.name, "rclone");
        assert_eq!(record.user_id, "user_42");
        assert!(record.revoked_at.is_none());
    }

    #[test]
    fn verify_app_password_succeeds_and_updates_last_used() {
        let auth = op();
        let (raw, _) = auth.create_app_password("u", "rclone", vec![], 1000);
        let r = auth.verify_app_password(&raw, 2000).unwrap();
        assert_eq!(r.last_used_at, Some(2000));
    }

    #[test]
    fn verify_forged_token_fails() {
        let auth = op();
        let r = auth.verify_app_password("mws-jc-aaaa-bbbb", 1000);
        // Format invalide OU checksum invalide.
        assert!(r.is_err());
    }

    #[test]
    fn verify_random_string_fails() {
        let auth = op();
        let r = auth.verify_app_password("nope", 1000);
        assert!(matches!(r, Err(AuthOpError::Token(_))));
    }

    #[test]
    fn revoke_then_verify_fails() {
        let auth = op();
        let (raw, _) = auth.create_app_password("u", "rclone", vec![], 1000);
        auth.revoke_app_password("u", "rclone", 2000).unwrap();
        let r = auth.verify_app_password(&raw, 3000);
        assert!(matches!(r, Err(AuthOpError::AppPasswordRevoked(_))));
    }

    #[test]
    fn revoke_unknown_fails() {
        let auth = op();
        let r = auth.revoke_app_password("u", "ghost", 1000);
        assert!(matches!(r, Err(AuthOpError::AppPasswordNotFound(_))));
    }

    #[test]
    fn list_filters_by_user() {
        let auth = op();
        auth.create_app_password("alice", "a1", vec![], 1000);
        auth.create_app_password("alice", "a2", vec![], 2000);
        auth.create_app_password("bob", "b1", vec![], 1500);

        let alice = auth.list_app_passwords("alice");
        assert_eq!(alice.len(), 2);
        let bob = auth.list_app_passwords("bob");
        assert_eq!(bob.len(), 1);
    }

    #[test]
    fn list_is_sorted_by_creation() {
        let auth = op();
        auth.create_app_password("u", "third", vec![], 3000);
        auth.create_app_password("u", "first", vec![], 1000);
        auth.create_app_password("u", "second", vec![], 2000);

        let list = auth.list_app_passwords("u");
        let names: Vec<_> = list.iter().map(|p| p.name.clone()).collect();
        assert_eq!(names, vec!["first", "second", "third"]);
    }
}
