//! Service de gestion de base de données de MiyukiniAdmin

use kindmother::InstanceIdentity;

/// @id miyukiniadmin_database_service_trait
/// @role infrastructure
/// @layer operator
/// @do define_database_service_contract
//@human / @human Trait de service de gestion de base de données.
pub trait DatabaseService {
    /// @id miyukiniadmin_database_service_get_instance_info
    /// @role accessor
    /// @layer operator
    /// @do get_instance_information
    //@human / @human Récupère les informations sur une instance.
    /// @depends miyukiniadmin_database_service_trait
    /// Récupère les informations sur une instance.
    fn get_instance_info(&self, instance: &InstanceIdentity) -> InstanceInfo;
}

/// @id miyukiniadmin_database_instance_info
/// @role data
/// @layer operator
/// @do represent_instance_info
//@human / @human Informations sur une instance de base de données.
#[derive(Debug, Clone)]
pub struct InstanceInfo {
    /// @id miyukiniadmin_database_instance_info_identity
    /// @role data
    /// @layer operator
    /// @do store_instance_identity
    //@human / @human Identité de l'instance.
    /// @depends miyukiniadmin_database_instance_info
    pub identity: InstanceIdentity,
}

#[cfg(test)]
mod tests {
    use super::*;
    use kindmother::state::{InstanceState, InstanceType};

    /// @id miyukiniadmin_database_test_instance_info
    /// @role test
    /// @layer operator
    /// @do verify_instance_info_creation
    //@human / @human Test de création d'informations d'instance.
    /// @depends miyukiniadmin_database_instance_info
    #[test]
    fn test_instance_info() {
        let instance = InstanceState::new(InstanceType::Mother);
        let info = InstanceInfo {
            identity: instance.identity,
        };
        assert!(info.identity.is_mother());
    }
}
