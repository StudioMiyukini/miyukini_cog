//! Module de gestion de compatibilité d'EverBuddy

/// @id: everbuddy_compatibility_level
/// @role: data
/// @layer: core
/// @human: Niveau de compatibilité entre versions.
/// @do: represent_compatibility_level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatibilityLevel {
    /// @id: everbuddy_compatibility_level_compatible
    /// @role: data
    /// @layer: core
    /// @human: Versions compatibles.
    /// @do: represent_compatible_level
    /// @depends: everbuddy_compatibility_level
    Compatible,
    /// @id: everbuddy_compatibility_level_incompatible
    /// @role: data
    /// @layer: core
    /// @human: Versions incompatibles.
    /// @do: represent_incompatible_level
    /// @depends: everbuddy_compatibility_level
    Incompatible,
}

/// @id: everbuddy_compatibility
/// @role: data
/// @layer: core
/// @human: Résultat de vérification de compatibilité.
/// @do: represent_compatibility_result
#[derive(Debug, Clone)]
pub struct Compatibility {
    /// @id: everbuddy_compatibility_level_field
    /// @role: data
    /// @layer: core
    /// @human: Niveau de compatibilité.
    /// @do: store_compatibility_level
    /// @depends: everbuddy_compatibility
    pub level: CompatibilityLevel,
}

/// @id: everbuddy_compatibility_checker_trait
/// @role: infrastructure
/// @layer: core
/// @human: Trait de vérification de compatibilité.
/// @do: define_compatibility_checker_contract
pub trait CompatibilityChecker {
    /// @id: everbuddy_compatibility_checker_check
    /// @role: accessor
    /// @layer: core
    /// @human: Vérifie la compatibilité entre deux versions.
    /// @do: check_version_compatibility
    /// @depends: everbuddy_compatibility_checker_trait
    fn check(&self, version1: &str, version2: &str) -> Compatibility;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: everbuddy_compatibility_test_levels
    /// @role: test
    /// @layer: core
    /// @human: Test des niveaux de compatibilité.
    /// @do: verify_compatibility_levels
    /// @depends: everbuddy_compatibility_level
    #[test]
    fn test_compatibility_levels() {
        assert_eq!(
            CompatibilityLevel::Compatible,
            CompatibilityLevel::Compatible
        );
    }
}
