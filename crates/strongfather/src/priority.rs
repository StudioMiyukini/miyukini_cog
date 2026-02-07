//! # StrongFather — Gestion des Priorités
//!
//! Ce module définit la gestion des priorités pour les intentions dans StrongFather.
//!
//! ## Références
//!
//! - [Policy Engine Contract](../../../docs/core/StrongFather/contracts/policy/StrongFather%20-%20Policy%20Engine%20Contract.md)
//! - [Documentation Fondatrice](../../../docs/core/StrongFather/foundation/StrongFather%20-%20Documentation%20Fondatrice.md)

use crate::intent::Intent;
use crate::policy::PolicyResult;

/// @id: strongfather_priority_value
/// @role: domain_model
/// @layer: core
/// @human: Valeur de priorité relative d'une intention. Ordre d'importance par rapport à d'autres intentions.
/// @do: represent_priority_value
/// Valeur de priorité
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Priority {
    /// @id: strongfather_priority_value_level
    /// @role: domain_model
    /// @layer: core
    /// @human: Niveau de priorité (0 = minimum, valeurs plus élevées = plus prioritaire).
    /// @do: store_priority_level
    pub value: u32,
}

impl Priority {
    /// @id: strongfather_priority_new
    /// @role: constructor
    /// @layer: core
    /// @human: Crée une nouvelle priorité avec une valeur donnée.
    /// @do: create_priority
    /// @depends: strongfather_priority_value
    #[must_use] 
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    /// @id: strongfather_priority_minimum
    /// @role: constant
    /// @layer: core
    /// @human: Priorité minimale (0).
    /// @do: provide_minimum_priority
    /// @depends: strongfather_priority_value
    pub const MINIMUM: Priority = Priority { value: 0 };

    /// @id: strongfather_priority_maximum
    /// @role: constant
    /// @layer: core
    /// @human: Priorité maximale (u32::MAX).
    /// @do: provide_maximum_priority
    /// @depends: strongfather_priority_value
    pub const MAXIMUM: Priority = Priority {
        value: u32::MAX,
    };
}

impl Default for Priority {
    /// @id: strongfather_priority_default
    /// @role: constructor
    /// @layer: core
    /// @human: Priorité par défaut (0).
    /// @do: provide_default_priority
    fn default() -> Self {
        Self::MINIMUM
    }
}

/// @id: strongfather_priority_calculator
/// @role: service
/// @layer: core
/// @human: Calculateur de priorité. Détermine la priorité relative d'une intention selon les politiques.
/// @do: calculate_intent_priority
/// Calculateur de priorité
pub struct PriorityCalculator;

impl PriorityCalculator {
    /// @id: strongfather_priority_calculator_new
    /// @role: constructor
    /// @layer: core
    /// @human: Crée un nouveau calculateur de priorité.
    /// @do: create_priority_calculator
    /// @depends: strongfather_priority_calculator
    #[must_use] 
    pub fn new() -> Self {
        Self
    }

    /// @id: strongfather_priority_calculator_calculate
    /// @role: service
    /// @layer: core
    /// @human: Calcule la priorité d'une intention selon les politiques de priorité applicables et la priorité demandée.
    /// @do: calculate_priority_from_policies
    /// @depends: strongfather_priority_calculator_new
    #[must_use] 
    pub fn calculate(
        &self,
        intent: &Intent,
        policy_results: &[PolicyResult],
    ) -> Priority {
        // R-PRIO-3 : La priorité finale est déterminée par StrongFather, pas par l'appelant
        // La priorité demandée est indicative, mais les politiques de priorité peuvent la modifier

        let mut calculated_priority = intent.requested_priority.unwrap_or(0);

        // Appliquer les politiques de priorité
        for result in policy_results {
            if let PolicyResult::Priority { value, .. } = result {
                // Les politiques de priorité peuvent augmenter ou modifier la priorité
                // Pour simplifier, on prend la valeur maximale entre la priorité demandée
                // et les priorités établies par les politiques
                calculated_priority = calculated_priority.max(*value);
            }
        }

        Priority::new(calculated_priority)
    }

    /// @id: strongfather_priority_calculator_compare
    /// @role: service
    /// @layer: core
    /// @human: Compare deux priorités pour déterminer l'ordre relatif.
    /// @do: compare_priorities
    /// @depends: strongfather_priority_calculator_new
    #[must_use] 
    pub fn compare(&self, a: &Priority, b: &Priority) -> std::cmp::Ordering {
        a.value.cmp(&b.value)
    }
}

impl Default for PriorityCalculator {
    /// @id: strongfather_priority_calculator_default
    /// @role: constructor
    /// @layer: core
    /// @human: Crée un calculateur de priorité par défaut.
    /// @do: create_default_priority_calculator
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::{ActionType, CallContext, Intent, IntentData};
    use crate::policy::PolicyResult;
    use std::collections::HashMap;

    /// @id: strongfather_priority_test_create
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'une priorité.
    /// @do: test_priority_creation
    #[test]
    fn test_create_priority() {
        let priority = Priority::new(10);
        assert_eq!(priority.value, 10);
    }

    /// @id: strongfather_priority_test_default
    /// @role: test
    /// @layer: core
    /// @human: Test de la priorité par défaut.
    /// @do: test_default_priority
    #[test]
    fn test_default_priority() {
        let priority = Priority::default();
        assert_eq!(priority.value, 0);
    }

    /// @id: strongfather_priority_test_calculate_with_requested
    /// @role: test
    /// @layer: core
    /// @human: Test de calcul de priorité avec priorité demandée.
    /// @do: test_calculate_with_requested_priority
    #[test]
    fn test_calculate_with_requested_priority() {
        let calculator = PriorityCalculator::new();

        let context = CallContext {
            caller_identity: "user123".to_string(),
            origin: "product_a".to_string(),
            instance: "instance_1".to_string(),
        };

        let intent = Intent::new(
            "intent_001".to_string(),
            ActionType::Creation,
            "entity_123".to_string(),
            context,
            IntentData {
                content: HashMap::new(),
            },
        )
        .with_priority(5);

        let policy_results = vec![];

        let priority = calculator.calculate(&intent, &policy_results);
        assert_eq!(priority.value, 5);
    }

    /// @id: strongfather_priority_test_calculate_with_policies
    /// @role: test
    /// @layer: core
    /// @human: Test de calcul de priorité avec politiques de priorité.
    /// @do: test_calculate_with_priority_policies
    #[test]
    fn test_calculate_with_priority_policies() {
        let calculator = PriorityCalculator::new();

        let context = CallContext {
            caller_identity: "user123".to_string(),
            origin: "product_a".to_string(),
            instance: "instance_1".to_string(),
        };

        let intent = Intent::new(
            "intent_002".to_string(),
            ActionType::Modification,
            "entity_456".to_string(),
            context,
            IntentData {
                content: HashMap::new(),
            },
        )
        .with_priority(5);

        let policy_results = vec![PolicyResult::Priority {
            policy_id: "POL-PRIORITY".to_string(),
            value: 10,
        }];

        let priority = calculator.calculate(&intent, &policy_results);
        // La priorité établie par la politique (10) prime sur la priorité demandée (5)
        assert_eq!(priority.value, 10);
    }

    /// @id: strongfather_priority_test_compare
    /// @role: test
    /// @layer: core
    /// @human: Test de comparaison de priorités.
    /// @do: test_priority_comparison
    #[test]
    fn test_compare_priorities() {
        let calculator = PriorityCalculator::new();

        let priority_a = Priority::new(5);
        let priority_b = Priority::new(10);

        assert_eq!(
            calculator.compare(&priority_a, &priority_b),
            std::cmp::Ordering::Less
        );
        assert_eq!(
            calculator.compare(&priority_b, &priority_a),
            std::cmp::Ordering::Greater
        );
        assert_eq!(
            calculator.compare(&priority_a, &priority_a),
            std::cmp::Ordering::Equal
        );
    }
}
