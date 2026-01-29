//! Module de niveaux de confiance de BorderGuard

/// @id: borderguard_trust_level
/// @role: data
/// @layer: core
/// @human: Niveau de confiance pour une entité ou une interaction.
/// @do: represent_trust_level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TrustLevel {
    /// @id: borderguard_trust_level_trusted
    /// @role: data
    /// @layer: core
    /// @human: Entité de confiance - vérifiée et approuvée.
    /// @do: represent_trusted_level
    /// @depends: borderguard_trust_level
    Trusted,
    /// @id: borderguard_trust_level_verified
    /// @role: data
    /// @layer: core
    /// @human: Entité vérifiée - authentifiée mais pas encore approuvée.
    /// @do: represent_verified_level
    /// @depends: borderguard_trust_level
    Verified,
    /// @id: borderguard_trust_level_unknown
    /// @role: data
    /// @layer: core
    /// @human: Entité inconnue - pas encore vérifiée.
    /// @do: represent_unknown_level
    /// @depends: borderguard_trust_level
    Unknown,
    /// @id: borderguard_trust_level_hostile
    /// @role: data
    /// @layer: core
    /// @human: Entité hostile - identifiée comme menaçante.
    /// @do: represent_hostile_level
    /// @depends: borderguard_trust_level
    Hostile,
}

/// @id: borderguard_trust_level_classifier
/// @role: infrastructure
/// @layer: core
/// @human: Trait de classification des niveaux de confiance.
/// @do: define_trust_classification_contract
pub trait TrustLevelClassifier {
    /// @id: borderguard_trust_level_classifier_classify
    /// @role: infrastructure
    /// @layer: core
    /// @human: Classifie le niveau de confiance d'une entité.
    /// @do: classify_trust_level
    /// @depends: borderguard_trust_level_classifier
    fn classify(&self, entity_id: &str) -> TrustLevel;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: borderguard_trust_level_test_ordering
    /// @role: test
    /// @layer: core
    /// @human: Test de l'ordre des niveaux de confiance.
    /// @do: verify_trust_level_ordering
    /// @depends: borderguard_trust_level
    #[test]
    fn test_trust_level_ordering() {
        assert!(TrustLevel::Trusted > TrustLevel::Verified);
        assert!(TrustLevel::Verified > TrustLevel::Unknown);
        assert!(TrustLevel::Unknown > TrustLevel::Hostile);
    }
}
