//! Module de niveaux de confiance de BorderGuard

/// @id: borderguard_trust_level
/// @role: data
/// @layer: core
/// @human: Niveau de confiance pour une entité ou une interaction.
/// @do: represent_trust_level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl PartialOrd for TrustLevel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TrustLevel {
    /// Ordre de confiance : Trusted > Verified > Unknown > Hostile.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        match (self, other) {
            (TrustLevel::Trusted, TrustLevel::Trusted) => Ordering::Equal,
            (TrustLevel::Trusted, _) => Ordering::Greater,
            (_, TrustLevel::Trusted) => Ordering::Less,
            (TrustLevel::Verified, TrustLevel::Verified) => Ordering::Equal,
            (TrustLevel::Verified, _) => Ordering::Greater,
            (_, TrustLevel::Verified) => Ordering::Less,
            (TrustLevel::Unknown, TrustLevel::Unknown) => Ordering::Equal,
            (TrustLevel::Unknown, _) => Ordering::Greater,
            (_, TrustLevel::Unknown) => Ordering::Less,
            (TrustLevel::Hostile, TrustLevel::Hostile) => Ordering::Equal,
        }
    }
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

/// Implémentation concrète du classificateur : registre explicite, défaut Unknown (INV-BG-4).
#[derive(Debug, Default)]
pub struct DefaultTrustLevelClassifier {
    levels: std::collections::HashMap<String, TrustLevel>,
}

impl DefaultTrustLevelClassifier {
    /// Crée un classificateur vide (toutes les entités seront Unknown).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Enregistre le niveau de confiance pour une entité.
    pub fn register(&mut self, entity_id: String, level: TrustLevel) {
        self.levels.insert(entity_id, level);
    }

    /// Retourne le niveau pour une entité, ou Unknown si non enregistrée.
    #[must_use]
    pub fn classify_entity(&self, entity_id: &str) -> TrustLevel {
        self.levels
            .get(entity_id)
            .copied()
            .unwrap_or(TrustLevel::Unknown)
    }
}

impl TrustLevelClassifier for DefaultTrustLevelClassifier {
    fn classify(&self, entity_id: &str) -> TrustLevel {
        self.classify_entity(entity_id)
    }
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

    /// @id: borderguard_classifier_test_default_unknown
    /// @role: test
    /// @layer: core
    /// @human: Entité non enregistrée retourne Unknown (INV-BG-4).
    #[test]
    fn test_classifier_default_unknown() {
        let classifier = DefaultTrustLevelClassifier::new();
        assert_eq!(classifier.classify("any-entity"), TrustLevel::Unknown);
    }

    /// @id: borderguard_classifier_test_registered
    /// @role: test
    /// @layer: core
    /// @human: Entité enregistrée retourne le niveau défini.
    #[test]
    fn test_classifier_registered() {
        let mut classifier = DefaultTrustLevelClassifier::new();
        classifier.register("e1".to_string(), TrustLevel::Trusted);
        classifier.register("e2".to_string(), TrustLevel::Hostile);
        assert_eq!(classifier.classify("e1"), TrustLevel::Trusted);
        assert_eq!(classifier.classify("e2"), TrustLevel::Hostile);
        assert_eq!(classifier.classify("e3"), TrustLevel::Unknown);
    }
}
