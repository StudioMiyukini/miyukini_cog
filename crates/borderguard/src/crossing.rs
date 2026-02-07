//! Module de règles de franchissement de BorderGuard

use crate::trust_level::TrustLevel;

/// @id: borderguard_crossing_rule
/// @role: data
/// @layer: core
/// @human: Règle de franchissement d'une frontière.
/// @do: represent_crossing_rule
#[derive(Debug, Clone)]
pub struct CrossingRule {
    /// @id: borderguard_crossing_rule_boundary_id
    /// @role: data
    /// @layer: core
    /// @human: Identifiant de la frontière concernée.
    /// @do: store_boundary_id
    /// @depends: borderguard_crossing_rule
    pub boundary_id: String,
    /// @id: borderguard_crossing_rule_min_trust_level
    /// @role: data
    /// @layer: core
    /// @human: Niveau de confiance minimum requis pour franchir.
    /// @do: store_min_trust_level
    /// @depends: borderguard_crossing_rule
    pub min_trust_level: TrustLevel,
    /// @id: borderguard_crossing_rule_allowed
    /// @role: data
    /// @layer: core
    /// @human: Indique si le franchissement est autorisé avec ce niveau de confiance.
    /// @do: store_allowed_flag
    /// @depends: borderguard_crossing_rule
    pub allowed: bool,
}

impl CrossingRule {
    /// @id: borderguard_crossing_rule_new
    /// @role: infrastructure
    /// @layer: core
    /// @human: Crée une nouvelle règle de franchissement.
    /// @do: create_crossing_rule
    /// @depends: borderguard_crossing_rule
    #[must_use] 
    pub fn new(boundary_id: String, min_trust_level: TrustLevel, allowed: bool) -> Self {
        Self {
            boundary_id,
            min_trust_level,
            allowed,
        }
    }
}

/// @id: borderguard_crossing_rules
/// @role: infrastructure
/// @layer: core
/// @human: Collection de règles de franchissement.
/// @do: represent_crossing_rules_collection
#[derive(Debug, Default)]
pub struct CrossingRules {
    /// @id: borderguard_crossing_rules_rules
    /// @role: data
    /// @layer: core
    /// @human: Liste des règles de franchissement.
    /// @do: store_crossing_rules
    /// @depends: borderguard_crossing_rules
    pub rules: Vec<CrossingRule>,
}

impl CrossingRules {
    /// @id: borderguard_crossing_rules_add
    /// @role: mutator
    /// @layer: core
    /// @human: Ajoute une règle de franchissement.
    /// @do: add_crossing_rule
    /// @depends: borderguard_crossing_rules
    pub fn add(&mut self, rule: CrossingRule) {
        self.rules.push(rule);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: borderguard_crossing_test_rule_creation
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'une règle de franchissement.
    /// @do: verify_crossing_rule_creation
    /// @depends: borderguard_crossing_rule_new
    #[test]
    fn test_crossing_rule_creation() {
        let rule = CrossingRule::new(
            "boundary-1".to_string(),
            TrustLevel::Verified,
            true,
        );
        assert_eq!(rule.boundary_id, "boundary-1");
        assert_eq!(rule.min_trust_level, TrustLevel::Verified);
        assert!(rule.allowed);
    }
}
