//! Cellule d'administration MiyuCloud (gouvernance COG).
//!
//! @id: miyucloud_admin_cell
//! @do: provide_admin_metadata_for_miyucloud
//! @role: governance
//! @layer: infra
//!
//! Metadonnees d'administration du service MiyuCloud, conformes au
//! pattern `AdminCell` du workspace Miyukini.

use serde::{Deserialize, Serialize};

/// Metadonnees d'administration du service MiyuCloud.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiyucloudAdminCell {
    /// Identifiant unique du service (genere au premier lancement).
    pub service_id: String,
    /// Version du service.
    pub version: String,
    /// Date de creation de la cellule (ISO 8601).
    pub created_at: String,
    /// Statut actuel du service.
    pub status: ServiceStatus,
}

/// Statut du service MiyuCloud.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceStatus {
    /// Service initialise mais pas encore demarre.
    Initialized,
    /// Service en cours d'execution.
    Running,
    /// Service arrete proprement.
    Stopped,
    /// Service en erreur.
    Error,
}

impl MiyucloudAdminCell {
    /// Cree une nouvelle cellule d'administration.
    pub fn new(service_id: String) -> Self {
        Self {
            service_id,
            version: env!("CARGO_PKG_VERSION").to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            status: ServiceStatus::Initialized,
        }
    }
}

impl Default for MiyucloudAdminCell {
    fn default() -> Self {
        Self::new(uuid::Uuid::new_v4().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admin_cell_default_is_initialized() {
        let cell = MiyucloudAdminCell::default();
        assert_eq!(cell.status, ServiceStatus::Initialized);
        assert!(!cell.service_id.is_empty());
        assert!(!cell.created_at.is_empty());
    }

    #[test]
    fn admin_cell_serialize_roundtrip() {
        let cell = MiyucloudAdminCell::default();
        let json = serde_json::to_string(&cell).unwrap();
        let restored: MiyucloudAdminCell = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.service_id, cell.service_id);
        assert_eq!(restored.status, ServiceStatus::Initialized);
    }
}
