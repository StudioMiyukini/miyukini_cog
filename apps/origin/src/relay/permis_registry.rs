//! @id     origin.relay.permis_registry
//! @do     store_and_validate_permis_issued_by_relay
//! @role   security
//! @layer  domain
//! @human  Registre des permis délivrés par le Relay (conformité R-011).
//!
//! Permet au Tracker de vérifier qu'un permis_id présenté dans une ANNOUNCE
//! a bien été délivré par le Relay (conformité R-011 / contrôle de présence).

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;

/// Nombre maximal de permis simultanés (C-13).
pub const MAX_PERMIS: usize = 1_000_000;

/// Entrée du registre : permis lié à un `cog_id` authentifié (O-01).
#[derive(Debug, Clone)]
struct EntreePermis {
    cog_id: String,
    expires_at: DateTime<Utc>,
}

/// Registre des permis valides (`permis_id` → `cog_id` authentifié + expiration).
#[derive(Debug, Clone)]
pub struct PermisRegistry {
    entries: Arc<RwLock<HashMap<String, EntreePermis>>>,
}

impl PermisRegistry {
    /// Crée un nouveau registre vide.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Enregistre un permis délivré par le Relay au `cog_id` dont l'identité a été prouvée.
    pub async fn register(&self, permis_id: String, cog_id: String, expires_at: DateTime<Utc>) {
        let mut entries = self.entries.write().await;
        if entries.len() >= MAX_PERMIS {
            let now = Utc::now();
            entries.retain(|_, e| e.expires_at > now);
        }
        debug!(
            "Permis registered: {} for {} (expires {:?})",
            permis_id, cog_id, expires_at
        );
        entries.insert(permis_id, EntreePermis { cog_id, expires_at });
    }

    /// Vérifie qu'un permis est valide (enregistré, non expiré) **et** lié à `cog_id`.
    pub async fn is_valid_for(&self, permis_id: &str, cog_id: &str) -> bool {
        let entries = self.entries.read().await;
        match entries.get(permis_id) {
            Some(e) => Utc::now() < e.expires_at && e.cog_id == cog_id,
            None => false,
        }
    }

    /// Retire un permis du registre (ex. session fermée côté Relay).
    pub async fn revoke(&self, permis_id: &str) {
        let mut entries = self.entries.write().await;
        entries.remove(permis_id);
        debug!("Permis revoked: {}", permis_id);
    }

    /// Nettoie les entrées expirées (à appeler périodiquement).
    pub async fn cleanup_expired(&self) -> usize {
        let now = Utc::now();
        let mut entries = self.entries.write().await;
        let before = entries.len();
        entries.retain(|_, e| e.expires_at > now);
        before - entries.len()
    }
}

impl Default for PermisRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn permis_lie_au_cog_id() {
        let r = PermisRegistry::new();
        let exp = Utc::now() + chrono::Duration::hours(1);
        r.register("p1".into(), "cogA".into(), exp).await;
        assert!(r.is_valid_for("p1", "cogA").await);
        // Permis valide mais présenté pour un autre cog_id : refusé.
        assert!(!r.is_valid_for("p1", "cogB").await);
        assert!(!r.is_valid_for("inconnu", "cogA").await);
        r.revoke("p1").await;
        assert!(!r.is_valid_for("p1", "cogA").await);
    }

    #[tokio::test]
    async fn permis_expire_refuse() {
        let r = PermisRegistry::new();
        r.register("p".into(), "c".into(), Utc::now() - chrono::Duration::seconds(1))
            .await;
        assert!(!r.is_valid_for("p", "c").await);
        assert_eq!(r.cleanup_expired().await, 1);
    }
}
