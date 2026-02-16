//! Module d'allocation de ressources de LogisticsSteward

use crate::resource::ResourceType;

/// @id: logisticssteward_allocation
/// @role: data
/// @layer: core
/// @human: Allocation d'une ressource à un composant.
/// @do: represent_resource_allocation
#[derive(Debug, Clone)]
pub struct Allocation {
    /// @id: logisticssteward_allocation_resource_type
    /// @role: data
    /// @layer: core
    /// @human: Type de ressource allouée.
    /// @do: store_resource_type
    /// @depends: logisticssteward_allocation
    pub resource_type: ResourceType,
    /// @id: logisticssteward_allocation_component_id
    /// @role: data
    /// @layer: core
    /// @human: Identifiant du composant auquel la ressource est allouée.
    /// @do: store_component_id
    /// @depends: logisticssteward_allocation
    pub component_id: String,
    /// @id: logisticssteward_allocation_amount
    /// @role: data
    /// @layer: core
    /// @human: Quantité allouée.
    /// @do: store_allocation_amount
    /// @depends: logisticssteward_allocation
    pub amount: u64,
}

/// @id: logisticssteward_allocation_manager_trait
/// @role: infrastructure
/// @layer: core
/// @human: Trait de gestion des allocations.
/// @do: define_allocation_manager_contract
pub trait AllocationManager {
    /// @id: logisticssteward_allocation_manager_allocate
    /// @role: mutator
    /// @layer: core
    /// @human: Alloue une ressource à un composant.
    /// @do: allocate_resource
    /// @depends: logisticssteward_allocation_manager_trait
    fn allocate(
        &mut self,
        resource_type: ResourceType,
        component_id: &str,
        amount: u64,
    ) -> Result<Allocation, AllocationError>;
}

/// @id: logisticssteward_allocation_error
/// @role: error
/// @layer: core
/// @human: Erreur d'allocation.
/// @do: represent_allocation_error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AllocationError {
    /// @id: logisticssteward_allocation_error_insufficient
    /// @role: error
    /// @layer: core
    /// @human: Ressources insuffisantes.
    /// @do: represent_insufficient_error
    /// @depends: logisticssteward_allocation_error
    InsufficientResources,
}

impl std::fmt::Display for AllocationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AllocationError::InsufficientResources => write!(f, "Insufficient resources"),
        }
    }
}

impl std::error::Error for AllocationError {}

/// Gestionnaire par défaut : suit les capacités disponibles et les allocations (en mémoire).
#[derive(Debug, Default)]
pub struct DefaultAllocationManager {
    available: std::collections::HashMap<ResourceType, u64>,
    allocations: Vec<Allocation>,
}

impl DefaultAllocationManager {
    /// Crée un gestionnaire vide.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Définit la capacité disponible pour un type de ressource.
    pub fn set_available(&mut self, resource_type: ResourceType, amount: u64) {
        self.available.insert(resource_type, amount);
    }
}

impl AllocationManager for DefaultAllocationManager {
    fn allocate(
        &mut self,
        resource_type: ResourceType,
        component_id: &str,
        amount: u64,
    ) -> Result<Allocation, AllocationError> {
        let avail = self.available.entry(resource_type).or_insert(0);
        if *avail < amount {
            return Err(AllocationError::InsufficientResources);
        }
        *avail -= amount;
        let allocation = Allocation {
            resource_type,
            component_id: component_id.to_string(),
            amount,
        };
        self.allocations.push(allocation.clone());
        Ok(allocation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: logisticssteward_allocation_test_creation
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'une allocation.
    /// @do: verify_allocation_creation
    /// @depends: logisticssteward_allocation
    #[test]
    fn test_allocation_creation() {
        let allocation = Allocation {
            resource_type: ResourceType::Memory,
            component_id: "comp-1".to_string(),
            amount: 512,
        };
        assert_eq!(allocation.amount, 512);
    }

    #[test]
    fn test_default_allocation_manager() {
        let mut mgr = DefaultAllocationManager::new();
        mgr.set_available(ResourceType::Memory, 1024);
        let a = mgr.allocate(ResourceType::Memory, "c1", 512).unwrap();
        assert_eq!(a.amount, 512);
        assert_eq!(a.component_id, "c1");
        assert!(mgr.allocate(ResourceType::Memory, "c2", 1024).is_err());
    }
}
