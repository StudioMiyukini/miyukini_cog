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
}
