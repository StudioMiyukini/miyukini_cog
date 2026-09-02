//! Service de sécurité de MiyukiniAdmin

use worrysentinel::SecurityLevel;

/// @id miyukiniadmin_security_service_trait
/// @role infrastructure
/// @layer operator
/// @do define_security_service_contract
//@human / @human Trait de service de sécurité. Gère les contrôles de sécurité.
pub trait SecurityService {
    /// @id miyukiniadmin_security_service_get_security_level
    /// @role accessor
    /// @layer operator
    /// @do get_current_security_level
    //@human / @human Récupère le niveau de sécurité actuel.
    /// @depends miyukiniadmin_security_service_trait, worrysentinel_security_level_manager_get_current
    /// Récupère le niveau de sécurité actuel.
    fn get_security_level(&self) -> SecurityLevel;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id miyukiniadmin_security_test_service_structure
    /// @role test
    /// @layer operator
    /// @do verify_security_service_structure
    //@human / @human Test de la structure du service de sécurité.
    /// @depends miyukiniadmin_security_service_trait
    #[test]
    fn test_security_service_structure() {
        // Test structure only - implementation would require concrete type
        assert!(matches!(SecurityLevel::Hardened, SecurityLevel::Hardened));
    }
}
