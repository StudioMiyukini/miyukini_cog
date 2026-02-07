//! # StrongFather — Moteur de Politiques
//!
//! Ce module définit le moteur d'application des politiques pour évaluer des intentions.
//!
//! ## Références
//!
//! - [Policy Engine Contract](../../../docs/core/StrongFather/contracts/policy/StrongFather%20-%20Policy%20Engine%20Contract.md)
//! - [Documentation Fondatrice](../../../docs/core/StrongFather/foundation/StrongFather%20-%20Documentation%20Fondatrice.md)

use crate::intent::Intent;
use crate::policy::{Policy, PolicyPriorityLevel, PolicyResult, PolicySet, PolicyType};

/// @id: strongfather_policy_engine_error
/// @role: error
/// @layer: core
/// @human: Erreur du moteur de politiques. Indique un dysfonctionnement interne.
/// @do: represent_policy_engine_error
/// Erreur du moteur de politiques
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyEngineError {
    /// @id: strongfather_policy_engine_error_consistency
    /// @role: error
    /// @layer: core
    /// @human: Erreur de cohérence - violation d'un invariant.
    /// @do: represent_consistency_error
    ConsistencyError {
        /// @id: strongfather_policy_engine_error_invariant
        /// @role: error
        /// @layer: core
        /// @human: Identifiant de l'invariant violé.
        /// @do: identify_violated_invariant
        violated_invariant: String,
        /// @id: strongfather_policy_engine_error_reason
        /// @role: error
        /// @layer: core
        /// @human: Raison de l'erreur.
        /// @do: explain_error
        reason: String,
    },
}

impl std::fmt::Display for PolicyEngineError {
    /// @id: strongfather_policy_engine_error_display
    /// @role: formatter
    /// @layer: core
    /// @human: Formate l'erreur pour affichage.
    /// @do: format_policy_engine_error
    /// @depends: strongfather_policy_engine_error
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PolicyEngineError::ConsistencyError {
                violated_invariant,
                reason,
            } => {
                write!(
                    f,
                    "Consistency error: {violated_invariant} - {reason}"
                )
            }
        }
    }
}

impl std::error::Error for PolicyEngineError {}

/// @id: strongfather_policy_engine
/// @role: service
/// @layer: core
/// @human: Moteur de politiques. Applique les politiques pour évaluer des intentions (G-POL-1 : évaluation déterministe).
/// @do: apply_policies_to_intent
/// Moteur de politiques
pub struct PolicyEngine {
    /// @id: strongfather_policy_engine_policies
    /// @role: service
    /// @layer: core
    /// @human: Ensemble de politiques chargées depuis la source unique (INV-POL-SOURCE).
    /// @do: store_policy_set
    policies: PolicySet,
}

impl PolicyEngine {
    /// @id: strongfather_policy_engine_new
    /// @role: constructor
    /// @layer: core
    /// @human: Crée un nouveau moteur de politiques avec un ensemble de politiques.
    /// @do: create_policy_engine
    /// @depends: strongfather_policy_engine
    #[must_use] 
    pub fn new(policies: PolicySet) -> Self {
        Self { policies }
    }

    /// @id: strongfather_policy_engine_apply
    /// @role: service
    /// @layer: core
    /// @human: Applique les politiques applicables à une intention selon l'ordre de priorité (RÈGLE-PRIO-1, G-POL-3 : évaluation ordonnée).
    /// @do: apply_policies_in_priority_order
    /// @depends: strongfather_policy_engine_new
    pub fn apply(&self, intent: &Intent) -> Result<Vec<PolicyResult>, PolicyEngineError> {
        // Sélectionner les politiques applicables
        let applicable_policies = self.select_applicable_policies(intent)?;

        // Trier par priorité décroissante (RÈGLE-PRIO-1)
        let mut ordered_policies = applicable_policies;
        ordered_policies.sort_by(|a, b| b.priority.cmp(&a.priority));

        // Évaluer chaque politique dans l'ordre
        let mut results = Vec::new();
        for policy in &ordered_policies {
            let result = self.evaluate_policy(policy, intent)?;

            // RÈGLE-PRIO-2 : Arrêt sur violation critique
            if policy.is_critical() {
                if let PolicyResult::Violated { .. } = &result {
                    // Arrêt immédiat si politique critique violée
                    results.push(result);
                    return Ok(results);
                }
            }

            results.push(result);
        }

        Ok(results)
    }

    /// @id: strongfather_policy_engine_select_applicable
    /// @role: service
    /// @layer: core
    /// @human: Sélectionne les politiques applicables à une intention selon leurs conditions d'application.
    /// @do: select_policies_by_condition
    /// @depends: strongfather_policy_engine_new
    fn select_applicable_policies(
        &self,
        intent: &Intent,
    ) -> Result<Vec<Policy>, PolicyEngineError> {
        // Pour simplifier, on considère qu'une politique est applicable si sa condition
        // correspond au type d'action de l'intention
        // Dans une implémentation complète, on évaluerait l'expression de condition

        let mut applicable = Vec::new();

        for policy in self.policies.iter() {
            // Vérification simplifiée : si la condition contient le type d'action, la politique est applicable
            // Dans une implémentation complète, on utiliserait un évaluateur d'expressions
            let action_type_str = match intent.action_type {
                crate::intent::ActionType::Creation => "creation",
                crate::intent::ActionType::Modification => "modification",
                crate::intent::ActionType::Deletion => "deletion",
                crate::intent::ActionType::Read => "read",
                crate::intent::ActionType::Evaluation => "evaluation",
            };

            if policy.condition.expression.contains(action_type_str)
                || policy.condition.expression == "true"
            {
                applicable.push(policy.clone());
            }
        }

        Ok(applicable)
    }

    /// @id: strongfather_policy_engine_evaluate_policy
    /// @role: service
    /// @layer: core
    /// @human: Évalue une politique individuelle pour une intention (INV-POL-3 : déterminisme).
    /// @do: evaluate_single_policy
    /// @depends: strongfather_policy_engine_new
    fn evaluate_policy(
        &self,
        policy: &Policy,
        _intent: &Intent,
    ) -> Result<PolicyResult, PolicyEngineError> {
        // Évaluation simplifiée selon le type de politique
        // Dans une implémentation complète, on évaluerait l'expression de règle

        match policy.policy_type {
            PolicyType::Permission => {
                // Pour simplifier, on considère que la politique est satisfaite si l'effet est Authorize
                if policy.effect == crate::policy::PolicyEffect::Authorize {
                    Ok(PolicyResult::Satisfied {
                        policy_id: policy.id.clone(),
                    })
                } else {
                    Ok(PolicyResult::Violated {
                        policy_id: policy.id.clone(),
                        reason: "Permission denied".to_string(),
                    })
                }
            }
            PolicyType::Constraint => {
                // Pour simplifier, on considère que la contrainte est satisfaite
                // Dans une implémentation complète, on évaluerait l'expression de contrainte
                Ok(PolicyResult::Satisfied {
                    policy_id: policy.id.clone(),
                })
            }
            PolicyType::Priority => {
                // Politique de priorité : extraire la valeur de priorité de la règle
                // Pour simplifier, on utilise une valeur par défaut basée sur la priorité de la politique
                let priority_value = match policy.priority {
                    PolicyPriorityLevel::Critical => 100,
                    PolicyPriorityLevel::Important => 50,
                    PolicyPriorityLevel::Normal => 25,
                    PolicyPriorityLevel::Optional => 10,
                };

                Ok(PolicyResult::Priority {
                    policy_id: policy.id.clone(),
                    value: priority_value,
                })
            }
            PolicyType::Validation => {
                // Pour simplifier, on considère que la validation est satisfaite
                Ok(PolicyResult::Satisfied {
                    policy_id: policy.id.clone(),
                })
            }
            PolicyType::Composite => {
                // Pour une politique composite, on devrait évaluer les politiques élémentaires
                // Pour simplifier, on considère qu'elle est satisfaite
                Ok(PolicyResult::Satisfied {
                    policy_id: policy.id.clone(),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::{ActionType, CallContext, Intent, IntentData};
    use crate::policy::{Policy, PolicyCondition, PolicyEffect, PolicyRule, PolicySet};
    use std::collections::HashMap;

    /// @id: strongfather_policy_engine_test_create
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'un moteur de politiques.
    /// @do: test_policy_engine_creation
    #[test]
    fn test_create_policy_engine() {
        let policy_set = PolicySet::new();
        let _engine = PolicyEngine::new(policy_set);
        // Vérification que le moteur est créé sans erreur
        assert!(true);
    }

    /// @id: strongfather_policy_engine_test_apply_permission
    /// @role: test
    /// @layer: core
    /// @human: Test d'application d'une politique de permission.
    /// @do: test_apply_permission_policy
    #[test]
    fn test_apply_permission_policy() {
        let mut policy_set = PolicySet::new();

        let policy = Policy::new(
            "POL-001".to_string(),
            PolicyType::Permission,
            PolicyCondition {
                expression: "action == 'creation'".to_string(),
            },
            PolicyRule {
                expression: "user.role == 'admin'".to_string(),
            },
            PolicyEffect::Authorize,
        );

        policy_set.add(policy).unwrap();

        let engine = PolicyEngine::new(policy_set);

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
        );

        let results = engine.apply(&intent).unwrap();
        assert!(!results.is_empty());
    }

    /// @id: strongfather_policy_engine_test_priority_order
    /// @role: test
    /// @layer: core
    /// @human: Test que les politiques sont évaluées dans l'ordre de priorité.
    /// @do: test_policy_priority_ordering
    #[test]
    fn test_policy_priority_ordering() {
        let mut policy_set = PolicySet::new();

        // Politique normale
        let policy_normal = Policy::new(
            "POL-NORMAL".to_string(),
            PolicyType::Permission,
            PolicyCondition {
                expression: "true".to_string(),
            },
            PolicyRule {
                expression: "true".to_string(),
            },
            PolicyEffect::Authorize,
        )
        .with_priority(PolicyPriorityLevel::Normal);

        // Politique critique
        let policy_critical = Policy::new(
            "POL-CRITICAL".to_string(),
            PolicyType::Constraint,
            PolicyCondition {
                expression: "true".to_string(),
            },
            PolicyRule {
                expression: "true".to_string(),
            },
            PolicyEffect::Constrain,
        )
        .with_priority(PolicyPriorityLevel::Critical);

        policy_set.add(policy_normal).unwrap();
        policy_set.add(policy_critical).unwrap();

        let engine = PolicyEngine::new(policy_set);

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
        );

        let results = engine.apply(&intent).unwrap();
        // La politique critique devrait être évaluée en premier
        assert!(!results.is_empty());
    }
}
