//! Module de gestion de ressources de LogisticsSteward

/// @id logisticssteward_resource_type
/// @role data
/// @layer core
/// @do represent_resource_type
//@human / @human Type de ressource système.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceType {
    /// @id logisticssteward_resource_type_memory
    /// @role data
    /// @layer core
    /// @do represent_memory_resource
    //@human / @human Ressource mémoire.
    /// @depends logisticssteward_resource_type
    Memory,
    /// @id logisticssteward_resource_type_cpu
    /// @role data
    /// @layer core
    /// @do represent_cpu_resource
    //@human / @human Ressource CPU.
    /// @depends logisticssteward_resource_type
    Cpu,
    /// @id logisticssteward_resource_type_disk
    /// @role data
    /// @layer core
    /// @do represent_disk_resource
    //@human / @human Ressource disque.
    /// @depends logisticssteward_resource_type
    Disk,
}

/// @id logisticssteward_resource
/// @role data
/// @layer core
/// @do represent_resource
//@human / @human Ressource système avec son type et sa capacité.
#[derive(Debug, Clone)]
pub struct Resource {
    /// @id logisticssteward_resource_type_field
    /// @role data
    /// @layer core
    /// @do store_resource_type
    //@human / @human Type de ressource.
    /// @depends logisticssteward_resource
    pub resource_type: ResourceType,
    /// @id logisticssteward_resource_capacity
    /// @role data
    /// @layer core
    /// @do store_resource_capacity
    //@human / @human Capacité totale de la ressource.
    /// @depends logisticssteward_resource
    pub capacity: u64,
}

/// @id logisticssteward_resource_manager_trait
/// @role infrastructure
/// @layer core
/// @do define_resource_manager_contract
//@human / @human Trait de gestion des ressources.
pub trait ResourceManager {
    /// @id logisticssteward_resource_manager_get
    /// @role accessor
    /// @layer core
    /// @do get_resource_by_type
    //@human / @human Récupère une ressource par son type.
    /// @depends logisticssteward_resource_manager_trait
    fn get(&self, resource_type: ResourceType) -> Option<&Resource>;
}

/// Gestionnaire par défaut : registre en mémoire.
#[derive(Debug, Default)]
pub struct DefaultResourceManager {
    resources: std::collections::HashMap<ResourceType, Resource>,
}

impl DefaultResourceManager {
    /// Crée un gestionnaire vide.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Enregistre une ressource.
    pub fn register(&mut self, resource: Resource) {
        self.resources.insert(resource.resource_type, resource);
    }
}

impl ResourceManager for DefaultResourceManager {
    fn get(&self, resource_type: ResourceType) -> Option<&Resource> {
        self.resources.get(&resource_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id logisticssteward_resource_test_creation
    /// @role test
    /// @layer core
    /// @do verify_resource_creation
    //@human / @human Test de création d'une ressource.
    /// @depends logisticssteward_resource
    #[test]
    fn test_resource_creation() {
        let resource = Resource {
            resource_type: ResourceType::Memory,
            capacity: 1024,
        };
        assert_eq!(resource.resource_type, ResourceType::Memory);
        assert_eq!(resource.capacity, 1024);
    }

    #[test]
    fn test_default_resource_manager() {
        let mut mgr = DefaultResourceManager::new();
        mgr.register(Resource {
            resource_type: ResourceType::Memory,
            capacity: 1024,
        });
        assert!(mgr.get(ResourceType::Memory).is_some());
        assert_eq!(mgr.get(ResourceType::Memory).unwrap().capacity, 1024);
    }
}
