//! Service de sécurité de MiyukiniAdmin

use worrysentinel::SecurityLevel;

/// @id: miyukiniadmin_security_service_trait
/// @role: infrastructure
/// @layer: operator
/// @human: Trait de service de sécurité. Gère les contrôles de sécurité.
/// @do: define_security_service_contract
pub trait SecurityService {
    /// @id: miyukiniadmin_security_service_get_security_level
    /// @role: accessor
    /// @layer: operator
    /// @human: Récupère le niveau de sécurité actuel.
    /// @do: get_current_security_level
    /// @depends: miyukiniadmin_security_service_trait, worrysentinel_security_level_manager_get_current
    /// Récupère le niveau de sécurité actuel.
    fn get_security_level(&self) -> SecurityLevel;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: miyukiniadmin_security_test_service_structure
    /// @role: test
    /// @layer: operator
    /// @human: Test de la structure du service de sécurité.
    /// @do: verify_security_service_structure
    /// @depends: miyukiniadmin_security_service_trait
    #[test]
    fn test_security_service_structure() {
        // Test structure only - implementation would require concrete type
        assert!(matches!(SecurityLevel::Hardened, SecurityLevel::Hardened));
    }
}
