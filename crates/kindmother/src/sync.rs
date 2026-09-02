//! Module de synchronisation de KindMother
//!
//! Ce module gère la synchronisation entre instances (DB Mère et DB Filles).
//! Il détecte les deltas, résout les conflits, et propage les changements.

use crate::state::InstanceIdentity;

/// @id kindmother_sync_delta
/// @role data
/// @layer core
/// @do represent_sync_delta
//@human / @human Delta représentant les différences entre deux états pour la synchronisation.
/// Delta de synchronisation.
#[derive(Debug, Clone)]
pub struct SyncDelta {
    /// @id kindmother_sync_delta_entity_id
    /// @role data
    /// @layer core
    /// @do store_entity_id
    //@human / @human Identifiant de l'entité modifiée.
    /// @depends kindmother_sync_delta
    pub entity_id: String,
    /// @id kindmother_sync_delta_operation
    /// @role data
    /// @layer core
    /// @do store_operation_type
    //@human / @human Type d'opération (création, modification, suppression).
    /// @depends kindmother_sync_delta
    pub operation: DeltaOperation,
    /// @id kindmother_sync_delta_data
    /// @role data
    /// @layer core
    /// @do store_entity_data
    //@human / @human Données de l'entité (si création ou modification).
    /// @depends kindmother_sync_delta
    pub data: Option<Vec<u8>>,
}

/// @id kindmother_sync_delta_operation_delta_operation_2
/// @role data
/// @layer core
/// @do represent_delta_operation
//@human / @human Type d'opération dans un delta de synchronisation.
/// Type d'opération dans un delta.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeltaOperation {
    /// @id kindmother_sync_delta_operation_create
    /// @role data
    /// @layer core
    /// @do represent_create_operation
    //@human / @human Création d'une nouvelle entité.
    /// @depends kindmother_sync_delta_operation
    Create,
    /// @id kindmother_sync_delta_operation_update
    /// @role data
    /// @layer core
    /// @do represent_update_operation
    //@human / @human Modification d'une entité existante.
    /// @depends kindmother_sync_delta_operation
    Update,
    /// @id kindmother_sync_delta_operation_delete
    /// @role data
    /// @layer core
    /// @do represent_delete_operation
    //@human / @human Suppression d'une entité.
    /// @depends kindmother_sync_delta_operation
    Delete,
}

/// @id kindmother_sync_trait
/// @role infrastructure
/// @layer core
/// @do define_sync_contract
//@human / @human Trait de synchronisation entre instances. Gère la détection des deltas et la résolution de conflits.
/// Trait de synchronisation.
pub trait Sync {
    /// @id kindmother_sync_compute_delta
    /// @role infrastructure
    /// @layer core
    /// @do compute_sync_deltas
    //@human / @human Calcule les deltas entre deux instances depuis le dernier point de synchronisation.
    /// @depends kindmother_sync_trait
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

    /// @id kindmother_sync_apply_delta
    /// @role mutator
    /// @layer core
    /// @do apply_sync_delta
    //@human / @human Applique un delta à une instance cible.
    /// @depends kindmother_sync_trait
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

/// @id kindmother_sync_error
/// @role error
/// @layer core
/// @do represent_sync_error
//@human / @human Erreur de synchronisation.
/// Erreur de synchronisation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyncError {
    /// @id kindmother_sync_error_conflict
    /// @role error
    /// @layer core
    /// @do represent_conflict_error
    //@human / @human Conflit détecté lors de la synchronisation.
    /// @depends kindmother_sync_error
    Conflict(String),
    /// @id kindmother_sync_error_invalid_delta
    /// @role error
    /// @layer core
    /// @do represent_invalid_delta_error
    //@human / @human Delta invalide détecté.
    /// @depends kindmother_sync_error
    InvalidDelta(String),
    /// @id kindmother_sync_error_io
    /// @role error
    /// @layer core
    /// @do represent_io_error
    //@human / @human Erreur d'entrée/sortie lors de la synchronisation.
    /// @depends kindmother_sync_error
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

    /// @id kindmother_sync_test_delta_creation
    /// @role test
    /// @layer core
    /// @do verify_delta_creation
    //@human / @human Test de création d'un delta.
    /// @depends kindmother_sync_delta
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
