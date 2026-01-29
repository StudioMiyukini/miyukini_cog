//! Module API CoreData de KindMother
//!
//! Ce module expose les 10 opérations de la CoreDataAPI :
//! - read, list, query (lecture)
//! - submitWriteIntent, submitBatchWriteIntent (écriture)
//! - sync, requestSync (synchronisation)
//! - inspect, health, metrics (inspection)

use crate::state::InstanceIdentity;

/// @id: kindmother_api_write_intent
/// @role: data
/// @layer: core
/// @human: Intention d'écriture avant validation et application.
/// @do: represent_write_intent
/// Intention d'écriture.
#[derive(Debug, Clone)]
pub struct WriteIntent {
    /// @id: kindmother_api_write_intent_id
    /// @role: data
    /// @layer: core
    /// @human: Identifiant unique de l'intention.
    /// @do: store_intent_id
    /// @depends: kindmother_api_write_intent
    pub id: String,
    /// @id: kindmother_api_write_intent_entity_id
    /// @role: data
    /// @layer: core
    /// @human: Identifiant de l'entité concernée.
    /// @do: store_entity_id
    /// @depends: kindmother_api_write_intent
    pub entity_id: String,
    /// @id: kindmother_api_write_intent_operation
    /// @role: data
    /// @layer: core
    /// @human: Type d'opération (création, modification, suppression).
    /// @do: store_operation_type
    /// @depends: kindmother_api_write_intent
    pub operation: WriteOperation,
    /// @id: kindmother_api_write_intent_data
    /// @role: data
    /// @layer: core
    /// @human: Données de l'entité (si création ou modification).
    /// @do: store_entity_data
    /// @depends: kindmother_api_write_intent
    pub data: Option<Vec<u8>>,
}

/// @id: kindmother_api_write_operation
/// @role: data
/// @layer: core
/// @human: Type d'opération d'écriture.
/// @do: represent_write_operation
/// Type d'opération d'écriture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteOperation {
    /// @id: kindmother_api_write_operation_create
    /// @role: data
    /// @layer: core
    /// @human: Création d'une nouvelle entité.
    /// @do: represent_create_operation
    /// @depends: kindmother_api_write_operation
    Create,
    /// @id: kindmother_api_write_operation_update
    /// @role: data
    /// @layer: core
    /// @human: Modification d'une entité existante.
    /// @do: represent_update_operation
    /// @depends: kindmother_api_write_operation
    Update,
    /// @id: kindmother_api_write_operation_delete
    /// @role: data
    /// @layer: core
    /// @human: Suppression d'une entité.
    /// @do: represent_delete_operation
    /// @depends: kindmother_api_write_operation
    Delete,
}

/// @id: kindmother_api_coredata
/// @role: infrastructure
/// @layer: core
/// @human: Trait de la CoreDataAPI. Expose les 10 opérations de KindMother.
/// @do: define_coredata_api
/// Trait de la CoreDataAPI.
pub trait CoreDataAPI {
    /// @id: kindmother_api_read
    /// @role: accessor
    /// @layer: core
    /// @human: Lit une entité unique par son identifiant.
    /// @do: read_entity_by_id
    /// @depends: kindmother_api_coredata
    /// Lit une entité unique.
    fn read(
        &self,
        instance: &InstanceIdentity,
        entity_id: &str,
    ) -> Result<Option<Vec<u8>>, APIError>;

    /// @id: kindmother_api_submit_write_intent
    /// @role: mutator
    /// @layer: core
    /// @human: Soumet une intention d'écriture pour validation et application.
    /// @do: submit_write_intent
    /// @depends: kindmother_api_coredata
    /// Soumet une intention d'écriture.
    fn submit_write_intent(
        &mut self,
        instance: &InstanceIdentity,
        intent: WriteIntent,
    ) -> Result<(), APIError>;
}

/// @id: kindmother_api_error
/// @role: error
/// @layer: core
/// @human: Erreur de l'API CoreData.
/// @do: represent_api_error
/// Erreur de l'API CoreData.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum APIError {
    /// @id: kindmother_api_error_not_found
    /// @role: error
    /// @layer: core
    /// @human: Entité non trouvée.
    /// @do: represent_not_found_error
    /// @depends: kindmother_api_error
    NotFound,
    /// @id: kindmother_api_error_permission_denied
    /// @role: error
    /// @layer: core
    /// @human: Permission refusée.
    /// @do: represent_permission_error
    /// @depends: kindmother_api_error
    PermissionDenied(String),
    /// @id: kindmother_api_error_invalid_intent
    /// @role: error
    /// @layer: core
    /// @human: Intention invalide.
    /// @do: represent_invalid_intent_error
    /// @depends: kindmother_api_error
    InvalidIntent(String),
}

impl std::fmt::Display for APIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            APIError::NotFound => write!(f, "Entity not found"),
            APIError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            APIError::InvalidIntent(msg) => write!(f, "Invalid intent: {}", msg),
        }
    }
}

impl std::error::Error for APIError {}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: kindmother_api_test_write_intent_creation
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'une intention d'écriture.
    /// @do: verify_write_intent_creation
    /// @depends: kindmother_api_write_intent
    #[test]
    fn test_write_intent_creation() {
        let intent = WriteIntent {
            id: "intent-1".to_string(),
            entity_id: "entity-1".to_string(),
            operation: WriteOperation::Create,
            data: Some(b"test data".to_vec()),
        };
        assert_eq!(intent.id, "intent-1");
        assert_eq!(intent.operation, WriteOperation::Create);
    }
}
