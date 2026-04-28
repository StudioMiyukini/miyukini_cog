//! Gestion des sessions authentifiées CentralRemote.
//!
//! @id: central_remote_session @do: manage_remote_auth_sessions
//! @role: security @layer: toolkit
//! @human: Tokens HMAC-SHA256 avec expiration pour les sessions remote.

use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

/// Durée de validité d'un token remote (1 heure).
const SESSION_TTL: Duration = Duration::from_secs(3600);

/// Session remote authentifiée.
#[derive(Debug, Clone)]
pub struct RemoteSession {
    pub token: String,
    pub profile_id: String,
    pub display_name: String,
    pub created_at: u64,
    pub expires_at: u64,
}

/// Gestionnaire de sessions remote.
pub struct SessionManager {
    secret: Vec<u8>,
    sessions: Mutex<HashMap<String, RemoteSession>>,
}

impl SessionManager {
    /// Crée un gestionnaire avec un secret HMAC.
    pub fn new(secret: &[u8]) -> Self {
        Self {
            secret: secret.to_vec(),
            sessions: Mutex::new(HashMap::new()),
        }
    }

    /// Crée un gestionnaire avec un secret aléatoire.
    pub fn with_random_secret() -> Self {
        Self::new(Uuid::new_v4().as_bytes())
    }

    /// Crée une session pour un profil authentifié. Retourne le token.
    pub fn create_session(&self, profile_id: &str, display_name: &str) -> String {
        let now = now_secs();
        let raw_token = format!("{}.{}.{}", profile_id, now, Uuid::new_v4());
        let token = self.sign_token(&raw_token);

        let session = RemoteSession {
            token: token.clone(),
            profile_id: profile_id.to_string(),
            display_name: display_name.to_string(),
            created_at: now,
            expires_at: now + SESSION_TTL.as_secs(),
        };

        if let Ok(mut sessions) = self.sessions.lock() {
            // Nettoyage des sessions expirées.
            sessions.retain(|_, s| s.expires_at > now);
            sessions.insert(token.clone(), session);
        }

        token
    }

    /// Valide un token et retourne la session associée.
    pub fn validate_token(&self, token: &str) -> Option<RemoteSession> {
        let sessions = self.sessions.lock().ok()?;
        let session = sessions.get(token)?;
        let now = now_secs();
        if session.expires_at <= now {
            return None;
        }
        Some(session.clone())
    }

    /// Révoque une session.
    pub fn revoke_session(&self, token: &str) {
        if let Ok(mut sessions) = self.sessions.lock() {
            sessions.remove(token);
        }
    }

    /// Nombre de sessions actives.
    pub fn active_count(&self) -> usize {
        let now = now_secs();
        self.sessions
            .lock()
            .map(|s| s.values().filter(|sess| sess.expires_at > now).count())
            .unwrap_or(0)
    }

    fn sign_token(&self, data: &str) -> String {
        let mut mac =
            HmacSha256::new_from_slice(&self.secret).expect("HMAC accepts any key length");
        mac.update(data.as_bytes());
        let sig = hex::encode(mac.finalize().into_bytes());
        format!("{data}.{sig}")
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_create_and_validate() {
        let mgr = SessionManager::with_random_secret();
        let token = mgr.create_session("prof-1", "Alice");
        let session = mgr.validate_token(&token).expect("session should be valid");
        assert_eq!(session.profile_id, "prof-1");
        assert_eq!(session.display_name, "Alice");
    }

    #[test]
    fn invalid_token_rejected() {
        let mgr = SessionManager::with_random_secret();
        assert!(mgr.validate_token("bogus-token").is_none());
    }

    #[test]
    fn revoked_session_rejected() {
        let mgr = SessionManager::with_random_secret();
        let token = mgr.create_session("prof-1", "Alice");
        mgr.revoke_session(&token);
        assert!(mgr.validate_token(&token).is_none());
    }
}
