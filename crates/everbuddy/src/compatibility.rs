//! Module de gestion de compatibilité d'EverBuddy

/// @id everbuddy_compatibility_level
/// @role data
/// @layer core
/// @do represent_compatibility_level
//@human / @human Niveau de compatibilité entre versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatibilityLevel {
    /// @id everbuddy_compatibility_level_compatible
    /// @role data
    /// @layer core
    /// @do represent_compatible_level
    //@human / @human Versions compatibles.
    /// @depends everbuddy_compatibility_level
    Compatible,
    /// @id everbuddy_compatibility_level_incompatible
    /// @role data
    /// @layer core
    /// @do represent_incompatible_level
    //@human / @human Versions incompatibles.
    /// @depends everbuddy_compatibility_level
    Incompatible,
}

/// @id everbuddy_compatibility
/// @role data
/// @layer core
/// @do represent_compatibility_result
//@human / @human Résultat de vérification de compatibilité.
#[derive(Debug, Clone)]
pub struct Compatibility {
    /// @id everbuddy_compatibility_level_field
    /// @role data
    /// @layer core
    /// @do store_compatibility_level
    //@human / @human Niveau de compatibilité.
    /// @depends everbuddy_compatibility
    pub level: CompatibilityLevel,
}

/// @id everbuddy_compatibility_checker_trait
/// @role infrastructure
/// @layer core
/// @do define_compatibility_checker_contract
//@human / @human Trait de vérification de compatibilité.
pub trait CompatibilityChecker {
    /// @id everbuddy_compatibility_checker_check
    /// @role accessor
    /// @layer core
    /// @do check_version_compatibility
    //@human / @human Vérifie la compatibilité entre deux versions.
    /// @depends everbuddy_compatibility_checker_trait
    fn check(&self, version1: &str, version2: &str) -> Compatibility;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id everbuddy_compatibility_test_levels
    /// @role test
    /// @layer core
    /// @do verify_compatibility_levels
    //@human / @human Test des niveaux de compatibilité.
    /// @depends everbuddy_compatibility_level
    #[test]
    fn test_compatibility_levels() {
        assert_eq!(
            CompatibilityLevel::Compatible,
            CompatibilityLevel::Compatible
        );
    }
}
