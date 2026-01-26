//! Calculateur de priorité pour StrongFather
//!
//! Ce module implémente le calculateur de priorité qui établit la priorité relative
//! d'une intention si toutes les politiques sont satisfaites.
//! Conforme au contrat Architecture & Flows (section 3.5) et Policy Engine Contract (section 3.3).
//!
//! **Invariants respectés :**
//! - INV-AUTH-3 : Pas d'autorité sur le temps (priorité relative, pas temporelle)
//!
//! **Caractéristiques :**
//! - Activé uniquement si toutes les politiques sont satisfaites
//! - Calcule la priorité à partir des politiques de type Priority avec effet Prioritize
//! - Prend en compte la priorité demandée par l'intention (indicative, non contraignante)
//! - Combine les priorités selon les règles de cumul (RÈGLE-PRIO-3)

use crate::error::SFError;
use crate::intent::Intent;
use crate::policy_engine::PolicyResult;
use crate::result_composer::ComposedResult;
use crate::types::Priority;

/// Calculateur de priorité pour StrongFather
///
/// Établit la priorité relative d'une intention si toutes les politiques sont satisfaites.
/// Conforme au contrat Architecture & Flows (section 3.5) et Policy Engine Contract (section 3.3).
///
/// **Responsabilités :**
/// - Appliquer les politiques de priorité
/// - Calculer la priorité relative
/// - Fournir la priorité à la décision
///
/// **Activation :**
/// - Activé uniquement si toutes les politiques sont satisfaites (ComposedResult::AllSatisfied)
///
/// **Règles appliquées :**
/// - RÈGLE-PRIO-3 : Cumul des politiques (les priorités sont combinées)
/// - INV-AUTH-3 : Priorité relative, pas temporelle
///
/// **Caractéristiques :**
/// - Déterministe : Pour une intention et des politiques données, la priorité est toujours la même
/// - Conceptuelle : La priorité est relative, pas temporelle
/// - Indicative : La priorité demandée par l'intention est indicative, non contraignante
#[derive(Debug, Clone)]
pub struct PriorityCalculator;

impl PriorityCalculator {
    /// Crée un nouveau calculateur de priorité
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::priority::PriorityCalculator;
    ///
    /// let calculator = PriorityCalculator::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Calcule la priorité d'une intention à partir du résultat composé
    ///
    /// La priorité est calculée uniquement si toutes les politiques sont satisfaites.
    /// Si le résultat n'est pas AllSatisfied, une erreur est retournée.
    ///
    /// **Processus de calcul :**
    /// 1. Vérifie que toutes les politiques sont satisfaites (AllSatisfied)
    /// 2. Extrait les politiques de type Priority avec effet Prioritize depuis les résultats
    /// 3. Combine les niveaux de priorité (maximum des niveaux)
    /// 4. Prend en compte la priorité demandée par l'intention (indicative)
    /// 5. Retourne la priorité calculée
    ///
    /// **Règles de combinaison :**
    /// - Si plusieurs politiques de priorité sont présentes, le maximum est utilisé (RÈGLE-PRIO-3)
    /// - La priorité demandée par l'intention est indicative et peut être ignorée
    /// - Si aucune politique de priorité n'est présente, la priorité par défaut est utilisée
    ///
    /// Conforme à :
    /// - Architecture & Flows (section 3.5)
    /// - Policy Engine Contract (section 3.3)
    /// - INV-AUTH-3 (priorité relative, pas temporelle)
    ///
    /// # Arguments
    /// * `intent` - Intention pour laquelle calculer la priorité
    /// * `result` - Résultat composé de l'évaluation des politiques (contient les policy_results)
    ///
    /// # Retour
    /// Priorité calculée, ou erreur si le résultat n'est pas AllSatisfied ou si le calcul échoue.
    ///
    /// # Erreurs
    /// - `ConsistencyError` : Si le résultat n'est pas AllSatisfied (priorité calculée uniquement si toutes politiques satisfaites)
    /// - `StructuralError` : Si une incohérence est détectée dans le calcul
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::priority::PriorityCalculator;
    /// use miyukini_strongfather::result_composer::ComposedResult;
    /// use miyukini_strongfather::intent::{Intent, CallContext, IntentData};
    /// use miyukini_strongfather::types::ActionType;
    /// use miyukini_strongfather::policy_engine::PolicyResult;
    ///
    /// let calculator = PriorityCalculator::new();
    /// let context = CallContext::new(
    ///     "user123".to_string(),
    ///     "product".to_string(),
    ///     "instance1".to_string(),
    /// );
    /// let intent = Intent::new(
    ///     "intent-001".to_string(),
    ///     ActionType::Creation,
    ///     "document".to_string(),
    ///     context,
    ///     IntentData::empty(),
    /// );
    /// let result = ComposedResult::AllSatisfied {
    ///     policy_results: Vec::new(),
    /// };
    /// let priority = calculator.calculate(&intent, &result);
    /// ```
    pub fn calculate(
        &self,
        intent: &Intent,
        result: &ComposedResult,
    ) -> Result<Priority, SFError> {
        // Vérification : la priorité est calculée uniquement si toutes les politiques sont satisfaites
        if !result.is_all_satisfied() {
            return Err(SFError::consistency(
                "La priorité ne peut être calculée que si toutes les politiques sont satisfaites",
                "PriorityCalculator::calculate",
            ));
        }

        // Étape 1 : Extraire les résultats de politiques depuis le ComposedResult
        let policy_results = match result {
            ComposedResult::AllSatisfied { policy_results } => policy_results,
            _ => {
                // Ne devrait jamais arriver car on a vérifié is_all_satisfied() ci-dessus
                return Err(SFError::consistency(
                    "État incohérent : résultat non AllSatisfied après vérification",
                    "PriorityCalculator::calculate",
                ));
            }
        };

        // Étape 2 : Extraire les politiques de priorité avec effet Prioritize
        let priority_policies = self.extract_priority_policies(policy_results);

        // Étape 2 : Calculer la priorité à partir des politiques
        let calculated_priority = if priority_policies.is_empty() {
            // Aucune politique de priorité : utiliser la priorité par défaut ou celle demandée
            self.default_priority(intent)
        } else {
            // Combiner les priorités : prendre le maximum (RÈGLE-PRIO-3 : cumul des politiques)
            let max_level = priority_policies
                .iter()
                .max()
                .copied()
                .unwrap_or(0);

            Priority::new(max_level)
        };

        // Étape 3 : Prendre en compte la priorité demandée par l'intention (indicative)
        let final_priority = if let Some(requested) = intent.requested_priority() {
            // La priorité demandée est indicative, non contraignante
            // On prend le maximum entre la priorité calculée et la priorité demandée
            // pour respecter les deux sources d'information
            let max_value = calculated_priority.value().max(requested.value());
            Priority::new(max_value)
        } else {
            calculated_priority
        };

        Ok(final_priority)
    }

    /// Extrait les niveaux de priorité des politiques de type Priority avec effet Prioritize
    ///
    /// # Arguments
    /// * `policy_results` - Résultats des politiques évaluées
    ///
    /// # Retour
    /// Vecteur des niveaux de priorité extraits
    fn extract_priority_policies(&self, policy_results: &[PolicyResult]) -> Vec<u8> {
        policy_results
            .iter()
            .filter_map(|result| {
                // Vérifier que la politique est de type Priority
                // Note : Le type de politique n'est pas directement accessible depuis PolicyResult,
                // donc on se base uniquement sur l'effet Prioritize
                match &result.effect {
                    crate::types::PolicyEffect::Prioritize { level } => Some(*level),
                    _ => None,
                }
            })
            .collect()
    }

    /// Détermine la priorité par défaut
    ///
    /// Utilise la priorité demandée par l'intention si présente, sinon la priorité minimale.
    ///
    /// # Arguments
    /// * `intent` - Intention pour laquelle déterminer la priorité par défaut
    ///
    /// # Retour
    /// Priorité par défaut
    fn default_priority(&self, intent: &Intent) -> Priority {
        // Si l'intention a une priorité demandée, l'utiliser
        // Sinon, utiliser la priorité minimale (0)
        intent
            .requested_priority()
            .unwrap_or_else(|| Priority::min())
    }
}

impl Default for PriorityCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::{CallContext, IntentData};
    use crate::policy_engine::PolicyResult;
    use crate::types::{ActionType, PolicyEffect, PolicyPriority};

    fn create_test_intent() -> Intent {
        let context = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        Intent::new(
            "intent-001".to_string(),
            ActionType::Creation,
            "document".to_string(),
            context,
            IntentData::empty(),
        )
    }

    fn create_priority_result(level: u8) -> PolicyResult {
        PolicyResult::satisfied(
            "POL-PRIO-001".to_string(),
            PolicyEffect::Prioritize { level },
            PolicyPriority::new(100),
        )
    }

    #[test]
    fn test_priority_calculator_new() {
        let calculator = PriorityCalculator::new();
        assert!(matches!(calculator, PriorityCalculator));
    }

    #[test]
    fn test_priority_calculator_default() {
        let calculator = PriorityCalculator::default();
        assert!(matches!(calculator, PriorityCalculator));
    }

    #[test]
    fn test_calculate_all_satisfied_with_priority_policy() {
        let calculator = PriorityCalculator::new();
        let intent = create_test_intent();

        let policy_results = vec![create_priority_result(150)];
        let result = ComposedResult::AllSatisfied { policy_results };

        let priority = calculator.calculate(&intent, &result).unwrap();
        assert_eq!(priority.value(), 150);
    }

    #[test]
    fn test_calculate_all_satisfied_multiple_priority_policies() {
        let calculator = PriorityCalculator::new();
        let intent = create_test_intent();

        let policy_results = vec![
            create_priority_result(100),
            create_priority_result(150),
            create_priority_result(75),
        ];
        let result = ComposedResult::AllSatisfied { policy_results };

        let priority = calculator.calculate(&intent, &result).unwrap();
        // Le maximum doit être utilisé (RÈGLE-PRIO-3)
        assert_eq!(priority.value(), 150);
    }

    #[test]
    fn test_calculate_all_satisfied_no_priority_policy() {
        let calculator = PriorityCalculator::new();
        let intent = create_test_intent();

        // Aucune politique de priorité
        let policy_results = vec![PolicyResult::satisfied(
            "POL-001".to_string(),
            PolicyEffect::Allow,
            PolicyPriority::new(100),
        )];
        let result = ComposedResult::AllSatisfied { policy_results };

        let priority = calculator.calculate(&intent, &result).unwrap();
        // Priorité par défaut (minimale) car aucune politique de priorité
        assert_eq!(priority.value(), 0);
    }

    #[test]
    fn test_calculate_all_satisfied_with_requested_priority() {
        let calculator = PriorityCalculator::new();
        let context = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let intent = Intent::with_priority(
            "intent-002".to_string(),
            ActionType::Modification,
            "document".to_string(),
            context,
            IntentData::empty(),
            Priority::new(200),
        );

        let policy_results = vec![create_priority_result(150)];
        let result = ComposedResult::AllSatisfied { policy_results };

        let priority = calculator.calculate(&intent, &result).unwrap();
        // Le maximum entre la priorité calculée (150) et la priorité demandée (200)
        assert_eq!(priority.value(), 200);
    }

    #[test]
    fn test_calculate_violated_should_fail() {
        let calculator = PriorityCalculator::new();
        let intent = create_test_intent();

        let policy_results = vec![PolicyResult::not_satisfied(
            "POL-001".to_string(),
            PolicyEffect::Deny,
            PolicyPriority::new(100),
        )];
        let result = ComposedResult::Violated {
            policy_results,
            policies: vec!["POL-001".to_string()],
        };

        let error = calculator.calculate(&intent, &result).unwrap_err();
        assert!(matches!(error, SFError::ConsistencyError { .. }));
        assert!(error
            .description()
            .contains("toutes les politiques sont satisfaites"));
    }

    #[test]
    fn test_calculate_ambiguous_should_fail() {
        let calculator = PriorityCalculator::new();
        let intent = create_test_intent();

        let policy_results = vec![PolicyResult::indeterminate(
            "POL-001".to_string(),
            PolicyEffect::Require,
            PolicyPriority::new(100),
        )];
        let result = ComposedResult::Ambiguous {
            policy_results,
            missing: vec!["info1".to_string()],
        };

        let error = calculator.calculate(&intent, &result).unwrap_err();
        assert!(matches!(error, SFError::ConsistencyError { .. }));
    }

    #[test]
    fn test_calculate_deferred_should_fail() {
        let calculator = PriorityCalculator::new();
        let intent = create_test_intent();

        let policy_results = vec![];
        let result = ComposedResult::Deferred {
            policy_results,
            reason: crate::types::DeferralReason::ContextRequired {
                description: "Context required".to_string(),
            },
            context_required: vec!["context1".to_string()],
        };

        let error = calculator.calculate(&intent, &result).unwrap_err();
        assert!(matches!(error, SFError::ConsistencyError { .. }));
    }

    #[test]
    fn test_extract_priority_policies() {
        let calculator = PriorityCalculator::new();

        let policy_results = vec![
            PolicyResult::satisfied(
                "POL-001".to_string(),
                PolicyEffect::Allow,
                PolicyPriority::new(100),
            ),
            create_priority_result(150),
            PolicyResult::satisfied(
                "POL-002".to_string(),
                PolicyEffect::Deny,
                PolicyPriority::new(50),
            ),
            create_priority_result(200),
        ];

        let levels = calculator.extract_priority_policies(&policy_results);
        assert_eq!(levels.len(), 2);
        assert!(levels.contains(&150));
        assert!(levels.contains(&200));
    }

    #[test]
    fn test_default_priority_with_requested() {
        let calculator = PriorityCalculator::new();
        let context = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let intent = Intent::with_priority(
            "intent-003".to_string(),
            ActionType::Creation,
            "document".to_string(),
            context,
            IntentData::empty(),
            Priority::new(100),
        );

        let priority = calculator.default_priority(&intent);
        assert_eq!(priority.value(), 100);
    }

    #[test]
    fn test_default_priority_without_requested() {
        let calculator = PriorityCalculator::new();
        let intent = create_test_intent();

        let priority = calculator.default_priority(&intent);
        assert_eq!(priority.value(), 0); // Priorité minimale
    }

    #[test]
    fn test_calculate_all_satisfied_empty_results() {
        let calculator = PriorityCalculator::new();
        let intent = create_test_intent();

        let result = ComposedResult::AllSatisfied {
            policy_results: Vec::new(),
        };

        let priority = calculator.calculate(&intent, &result).unwrap();
        // Aucune politique de priorité, donc priorité par défaut (minimale)
        assert_eq!(priority.value(), 0);
    }

    #[test]
    fn test_calculate_priority_with_mixed_effects() {
        let calculator = PriorityCalculator::new();
        let intent = create_test_intent();

        let policy_results = vec![
            PolicyResult::satisfied(
                "POL-001".to_string(),
                PolicyEffect::Allow,
                PolicyPriority::new(100),
            ),
            create_priority_result(175),
            PolicyResult::satisfied(
                "POL-002".to_string(),
                PolicyEffect::Require,
                PolicyPriority::new(50),
            ),
            create_priority_result(125),
        ];

        let result = ComposedResult::AllSatisfied {
            policy_results: policy_results.clone(),
        };
        let priority = calculator.calculate(&intent, &result).unwrap();

        // Seules les politiques avec effet Prioritize sont considérées
        // Maximum entre 175 et 125 = 175
        assert_eq!(priority.value(), 175);
    }
}
