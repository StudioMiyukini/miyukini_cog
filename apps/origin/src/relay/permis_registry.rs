//! @id: origin.relay.permis_registry
//! @do: store_and_validate_permis_issued_by_relay
//! @role: security
//! @layer: domain
//! @human: Registre des permis délivrés par le Relay (conformité R-011).
//!
//! Permet au Tracker de vérifier qu'un permis_id présenté dans une ANNOUNCE
//! a bien été délivré par le Relay (conformité R-011 / contrôle de présence).

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::debug;

/// Registre des permis valides (permis_id → date d'expiration).
#[derive(Debug, Clone)]
pub struct PermisRegistry {
    entries: Arc<RwLock<HashMap<String, DateTime<Utc>>>>,
}

impl PermisRegistry {
    /// Crée un nouveau registre vide.
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Enregistre un permis délivré par le Relay (appelé après REGISTER_OK).
    pub async fn register(&self, permis_id: String, expires_at: DateTime<Utc>) {
        let mut entries = self.entries.write().await;
        entries.insert(permis_id.clone(), expires_at);
        debug!("Permis registered: {} (expires {:?})", permis_id, expires_at);
    }

    /// Vérifie si un permis est valide (enregistré et non expiré).
    pub async fn is_valid(&self, permis_id: &str) -> bool {
        let entries = self.entries.read().await;
        match entries.get(permis_id) {
            Some(expires_at) => Utc::now() < *expires_at,
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
        entries.retain(|_, exp| *exp > now);
        before - entries.len()
    }
}

impl Default for PermisRegistry {
    fn default() -> Self {
        Self::new()
    }
}
