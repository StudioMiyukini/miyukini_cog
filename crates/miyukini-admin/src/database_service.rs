//! Service de gestion de base de données de MiyukiniAdmin

use kindmother::InstanceIdentity;

/// @id: miyukiniadmin_database_service_trait
/// @role: infrastructure
/// @layer: operator
/// @human: Trait de service de gestion de base de données.
/// @do: define_database_service_contract
pub trait DatabaseService {
    /// @id: miyukiniadmin_database_service_get_instance_info
    /// @role: accessor
    /// @layer: operator
    /// @human: Récupère les informations sur une instance.
    /// @do: get_instance_information
    /// @depends: miyukiniadmin_database_service_trait
    /// Récupère les informations sur une instance.
    fn get_instance_info(&self, instance: &InstanceIdentity) -> InstanceInfo;
}

/// @id: miyukiniadmin_database_instance_info
/// @role: data
/// @layer: operator
/// @human: Informations sur une instance de base de données.
/// @do: represent_instance_info
#[derive(Debug, Clone)]
pub struct InstanceInfo {
    /// @id: miyukiniadmin_database_instance_info_identity
    /// @role: data
    /// @layer: operator
    /// @human: Identité de l'instance.
    /// @do: store_instance_identity
    /// @depends: miyukiniadmin_database_instance_info
    pub identity: InstanceIdentity,
}

#[cfg(test)]
mod tests {
    use super::*;
    use kindmother::state::{InstanceState, InstanceType};

    /// @id: miyukiniadmin_database_test_instance_info
    /// @role: test
    /// @layer: operator
    /// @human: Test de création d'informations d'instance.
    /// @do: verify_instance_info_creation
    /// @depends: miyukiniadmin_database_instance_info
    #[test]
    fn test_instance_info() {
        let instance = InstanceState::new(InstanceType::Mother);
        let info = InstanceInfo {
            identity: instance.identity,
        };
        assert!(info.identity.is_mother());
    }
}
