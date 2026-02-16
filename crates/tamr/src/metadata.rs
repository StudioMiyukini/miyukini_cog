//! Module de gestion de métadonnées de TAMR

use std::collections::HashMap;

/// @id: tamr_metadata
/// @role: data
/// @layer: core
/// @human: Métadonnées associées à une entité.
/// @do: represent_metadata
#[derive(Debug, Clone)]
pub struct Metadata {
    /// @id: tamr_metadata_entity_id
    /// @role: data
    /// @layer: core
    /// @human: Identifiant de l'entité.
    /// @do: store_entity_id
    /// @depends: tamr_metadata
    pub entity_id: String,
    /// @id: tamr_metadata_values
    /// @role: data
    /// @layer: core
    /// @human: Valeurs des métadonnées (clé -> valeur).
    /// @do: store_metadata_values
    /// @depends: tamr_metadata
    pub values: HashMap<String, String>,
}

/// @id: tamr_metadata_manager_trait
/// @role: infrastructure
/// @layer: core
/// @human: Trait de gestion des métadonnées.
/// @do: define_metadata_manager_contract
pub trait MetadataManager {
    /// @id: tamr_metadata_manager_get
    /// @role: accessor
    /// @layer: core
    /// @human: Récupère les métadonnées d'une entité.
    /// @do: get_entity_metadata
    /// @depends: tamr_metadata_manager_trait
    fn get(&self, entity_id: &str) -> Option<&Metadata>;
}

/// Gestionnaire par défaut : registre en mémoire.
#[derive(Debug, Default)]
pub struct DefaultMetadataManager {
    by_entity: HashMap<String, Metadata>,
}

impl DefaultMetadataManager {
    /// Crée un gestionnaire vide.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Enregistre les métadonnées d'une entité.
    pub fn register(&mut self, metadata: Metadata) {
        let id = metadata.entity_id.clone();
        self.by_entity.insert(id, metadata);
    }
}

impl MetadataManager for DefaultMetadataManager {
    fn get(&self, entity_id: &str) -> Option<&Metadata> {
        self.by_entity.get(entity_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: tamr_metadata_test_creation
    /// @role: test
    /// @layer: core
    /// @human: Test de création de métadonnées.
    /// @do: verify_metadata_creation
    /// @depends: tamr_metadata
    #[test]
    fn test_metadata_creation() {
        let metadata = Metadata {
            entity_id: "entity-1".to_string(),
            values: HashMap::new(),
        };
        assert_eq!(metadata.entity_id, "entity-1");
    }

    #[test]
    fn test_default_metadata_manager() {
        let mut mgr = DefaultMetadataManager::new();
        mgr.register(Metadata {
            entity_id: "e1".to_string(),
            values: HashMap::new(),
        });
        assert!(mgr.get("e1").is_some());
        assert!(mgr.get("e2").is_none());
    }
}
