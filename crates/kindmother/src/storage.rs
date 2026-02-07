//! Module d'abstraction de la persistance de KindMother
//!
//! Ce module fournit une abstraction complète de la persistance. SQLite est utilisé en interne
//! mais n'est jamais exposé aux adaptateurs ou aux modules.

use crate::state::InstanceIdentity;

/// @id: kindmother_storage_trait
/// @role: infrastructure
/// @layer: core
/// @human: Trait d'abstraction de la persistance. Masque les détails d'implémentation (SQLite) et fournit une interface conceptuelle.
/// @do: define_storage_contract
/// Trait d'abstraction de la persistance.
///
/// Ce trait définit l'interface conceptuelle pour la persistance sans exposer SQLite.
/// Les adaptateurs utilisent cette interface pour stocker et récupérer des données.
pub trait Storage {
    /// @id: kindmother_storage_read
    /// @role: accessor
    /// @layer: core
    /// @human: Lit une entité depuis la persistance par son identifiant.
    /// @do: read_entity_by_id
    /// @depends: kindmother_storage_trait
    /// Lit une entité depuis la persistance.
    ///
    /// # Arguments
    ///
    /// * `instance` - Identité de l'instance
    /// * `entity_id` - Identifiant de l'entité à lire
    ///
    /// # Returns
    ///
    /// Les données de l'entité si elle existe, None sinon.
    fn read(&self, instance: &InstanceIdentity, entity_id: &str) -> Option<Vec<u8>>;

    /// @id: kindmother_storage_write
    /// @role: mutator
    /// @layer: core
    /// @human: Écrit une entité dans la persistance.
    /// @do: write_entity
    /// @depends: kindmother_storage_trait
    /// Écrit une entité dans la persistance.
    ///
    /// # Arguments
    ///
    /// * `instance` - Identité de l'instance
    /// * `entity_id` - Identifiant de l'entité
    /// * `data` - Données de l'entité à écrire
    ///
    /// # Returns
    ///
    /// Résultat de l'opération (succès ou erreur).
    fn write(
        &mut self,
        instance: &InstanceIdentity,
        entity_id: &str,
        data: &[u8],
    ) -> Result<(), StorageError>;

    /// @id: kindmother_storage_delete
    /// @role: mutator
    /// @layer: core
    /// @human: Supprime une entité de la persistance.
    /// @do: delete_entity
    /// @depends: kindmother_storage_trait
    /// Supprime une entité de la persistance.
    ///
    /// # Arguments
    ///
    /// * `instance` - Identité de l'instance
    /// * `entity_id` - Identifiant de l'entité à supprimer
    ///
    /// # Returns
    ///
    /// Résultat de l'opération (succès ou erreur).
    fn delete(&mut self, instance: &InstanceIdentity, entity_id: &str) -> Result<(), StorageError>;
}

/// @id: kindmother_storage_error
/// @role: error
/// @layer: core
/// @human: Erreur de persistance. Masque les détails d'implémentation SQLite.
/// @do: represent_storage_error
/// Erreur de persistance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageError {
    /// @id: kindmother_storage_error_not_found
    /// @role: error
    /// @layer: core
    /// @human: Entité non trouvée dans la persistance.
    /// @do: represent_not_found_error
    /// @depends: kindmother_storage_error
    NotFound,
    /// @id: kindmother_storage_error_corruption
    /// @role: error
    /// @layer: core
    /// @human: Corruption détectée dans la persistance.
    /// @do: represent_corruption_error
    /// @depends: kindmother_storage_error
    Corruption,
    /// @id: kindmother_storage_error_io
    /// @role: error
    /// @layer: core
    /// @human: Erreur d'entrée/sortie lors de l'accès à la persistance.
    /// @do: represent_io_error
    /// @depends: kindmother_storage_error
    Io(String),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::NotFound => write!(f, "Entity not found"),
            StorageError::Corruption => write!(f, "Storage corruption detected"),
            StorageError::Io(msg) => write!(f, "IO error: {msg}"),
        }
    }
}

impl std::error::Error for StorageError {}

/// @id: kindmother_storage_memory
/// @role: infrastructure
/// @layer: core
/// @human: Implémentation mémoire de Storage pour les tests. Utilise une HashMap en mémoire.
/// @do: provide_memory_storage
/// Implémentation mémoire de Storage pour les tests.
#[derive(Debug, Default)]
pub struct MemoryStorage {
    /// @id: kindmother_storage_memory_data
    /// @role: data
    /// @layer: core
    /// @human: Données stockées en mémoire (instance_id -> entity_id -> data).
    /// @do: store_memory_data
    /// @depends: kindmother_storage_memory
    data: std::collections::HashMap<String, std::collections::HashMap<String, Vec<u8>>>,
}

impl MemoryStorage {
    /// @id: kindmother_storage_memory_new
    /// @role: infrastructure
    /// @layer: core
    /// @human: Crée un nouveau stockage mémoire.
    /// @do: create_memory_storage
    /// @depends: kindmother_storage_memory
    /// Crée un nouveau stockage mémoire.
    #[must_use] 
    pub fn new() -> Self {
        Self::default()
    }
}

impl Storage for MemoryStorage {
    fn read(&self, instance: &InstanceIdentity, entity_id: &str) -> Option<Vec<u8>> {
        let instance_key = format!("{}", instance.id);
        self.data
            .get(&instance_key)?
            .get(entity_id)
            .cloned()
    }

    fn write(
        &mut self,
        instance: &InstanceIdentity,
        entity_id: &str,
        data: &[u8],
    ) -> Result<(), StorageError> {
        let instance_key = format!("{}", instance.id);
        self.data
            .entry(instance_key)
            .or_default()
            .insert(entity_id.to_string(), data.to_vec());
        Ok(())
    }

    fn delete(&mut self, instance: &InstanceIdentity, entity_id: &str) -> Result<(), StorageError> {
        let instance_key = format!("{}", instance.id);
        if let Some(instance_data) = self.data.get_mut(&instance_key) {
            instance_data.remove(entity_id);
            Ok(())
        } else {
            Err(StorageError::NotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{InstanceState, InstanceType};

    /// @id: kindmother_storage_test_read_write
    /// @role: test
    /// @layer: core
    /// @human: Test lecture/écriture dans le stockage mémoire.
    /// @do: verify_read_write_operations
    /// @depends: kindmother_storage_memory_new, kindmother_storage_write, kindmother_storage_read
    #[test]
    fn test_read_write() {
        let mut storage = MemoryStorage::new();
        let instance = InstanceState::new(InstanceType::Mother);
        let entity_id = "test-entity";
        let data = b"test data";

        storage
            .write(&instance.identity, entity_id, data)
            .unwrap();
        let read_data = storage.read(&instance.identity, entity_id).unwrap();
        assert_eq!(read_data, data);
    }

    /// @id: kindmother_storage_test_delete
    /// @role: test
    /// @layer: core
    /// @human: Test suppression dans le stockage mémoire.
    /// @do: verify_delete_operation
    /// @depends: kindmother_storage_memory_new, kindmother_storage_write, kindmother_storage_delete
    #[test]
    fn test_delete() {
        let mut storage = MemoryStorage::new();
        let instance = InstanceState::new(InstanceType::Mother);
        let entity_id = "test-entity";
        let data = b"test data";

        storage
            .write(&instance.identity, entity_id, data)
            .unwrap();
        storage.delete(&instance.identity, entity_id).unwrap();
        assert!(storage.read(&instance.identity, entity_id).is_none());
    }
}
