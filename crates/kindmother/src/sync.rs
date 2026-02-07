//! Module de synchronisation de KindMother
//!
//! Ce module gère la synchronisation entre instances (DB Mère et DB Filles).
//! Il détecte les deltas, résout les conflits, et propage les changements.

use crate::state::InstanceIdentity;

/// @id: kindmother_sync_delta
/// @role: data
/// @layer: core
/// @human: Delta représentant les différences entre deux états pour la synchronisation.
/// @do: represent_sync_delta
/// Delta de synchronisation.
#[derive(Debug, Clone)]
pub struct SyncDelta {
    /// @id: kindmother_sync_delta_entity_id
    /// @role: data
    /// @layer: core
    /// @human: Identifiant de l'entité modifiée.
    /// @do: store_entity_id
    /// @depends: kindmother_sync_delta
    pub entity_id: String,
    /// @id: kindmother_sync_delta_operation
    /// @role: data
    /// @layer: core
    /// @human: Type d'opération (création, modification, suppression).
    /// @do: store_operation_type
    /// @depends: kindmother_sync_delta
    pub operation: DeltaOperation,
    /// @id: kindmother_sync_delta_data
    /// @role: data
    /// @layer: core
    /// @human: Données de l'entité (si création ou modification).
    /// @do: store_entity_data
    /// @depends: kindmother_sync_delta
    pub data: Option<Vec<u8>>,
}

/// @id: kindmother_sync_delta_operation
/// @role: data
/// @layer: core
/// @human: Type d'opération dans un delta de synchronisation.
/// @do: represent_delta_operation
/// Type d'opération dans un delta.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeltaOperation {
    /// @id: kindmother_sync_delta_operation_create
    /// @role: data
    /// @layer: core
    /// @human: Création d'une nouvelle entité.
    /// @do: represent_create_operation
    /// @depends: kindmother_sync_delta_operation
    Create,
    /// @id: kindmother_sync_delta_operation_update
    /// @role: data
    /// @layer: core
    /// @human: Modification d'une entité existante.
    /// @do: represent_update_operation
    /// @depends: kindmother_sync_delta_operation
    Update,
    /// @id: kindmother_sync_delta_operation_delete
    /// @role: data
    /// @layer: core
    /// @human: Suppression d'une entité.
    /// @do: represent_delete_operation
    /// @depends: kindmother_sync_delta_operation
    Delete,
}

/// @id: kindmother_sync_trait
/// @role: infrastructure
/// @layer: core
/// @human: Trait de synchronisation entre instances. Gère la détection des deltas et la résolution de conflits.
/// @do: define_sync_contract
/// Trait de synchronisation.
pub trait Sync {
    /// @id: kindmother_sync_compute_delta
    /// @role: infrastructure
    /// @layer: core
    /// @human: Calcule les deltas entre deux instances depuis le dernier point de synchronisation.
    /// @do: compute_sync_deltas
    /// @depends: kindmother_sync_trait
    /// Calcule les deltas entre deux instances.
    ///
    /// # Arguments
    ///
    /// * `source` - Instance source
    /// * `target` - Instance cible
    ///
    /// # Returns
    ///
    /// Liste des deltas à appliquer.
    fn compute_delta(
        &self,
        source: &InstanceIdentity,
        target: &InstanceIdentity,
    ) -> Result<Vec<SyncDelta>, SyncError>;

    /// @id: kindmother_sync_apply_delta
    /// @role: mutator
    /// @layer: core
    /// @human: Applique un delta à une instance cible.
    /// @do: apply_sync_delta
    /// @depends: kindmother_sync_trait
    /// Applique un delta à une instance cible.
    ///
    /// # Arguments
    ///
    /// * `target` - Instance cible
    /// * `delta` - Delta à appliquer
    ///
    /// # Returns
    ///
    /// Résultat de l'opération (succès ou erreur).
    fn apply_delta(
        &mut self,
        target: &InstanceIdentity,
        delta: &SyncDelta,
    ) -> Result<(), SyncError>;
}

/// @id: kindmother_sync_error
/// @role: error
/// @layer: core
/// @human: Erreur de synchronisation.
/// @do: represent_sync_error
/// Erreur de synchronisation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncError {
    /// @id: kindmother_sync_error_conflict
    /// @role: error
    /// @layer: core
    /// @human: Conflit détecté lors de la synchronisation.
    /// @do: represent_conflict_error
    /// @depends: kindmother_sync_error
    Conflict(String),
    /// @id: kindmother_sync_error_invalid_delta
    /// @role: error
    /// @layer: core
    /// @human: Delta invalide détecté.
    /// @do: represent_invalid_delta_error
    /// @depends: kindmother_sync_error
    InvalidDelta(String),
    /// @id: kindmother_sync_error_io
    /// @role: error
    /// @layer: core
    /// @human: Erreur d'entrée/sortie lors de la synchronisation.
    /// @do: represent_io_error
    /// @depends: kindmother_sync_error
    Io(String),
}

impl std::fmt::Display for SyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncError::Conflict(msg) => write!(f, "Sync conflict: {msg}"),
            SyncError::InvalidDelta(msg) => write!(f, "Invalid delta: {msg}"),
            SyncError::Io(msg) => write!(f, "IO error: {msg}"),
        }
    }
}

impl std::error::Error for SyncError {}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: kindmother_sync_test_delta_creation
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'un delta.
    /// @do: verify_delta_creation
    /// @depends: kindmother_sync_delta
    #[test]
    fn test_delta_creation() {
        let delta = SyncDelta {
            entity_id: "test-entity".to_string(),
            operation: DeltaOperation::Create,
            data: Some(b"test data".to_vec()),
        };
        assert_eq!(delta.entity_id, "test-entity");
        assert_eq!(delta.operation, DeltaOperation::Create);
    }
}
