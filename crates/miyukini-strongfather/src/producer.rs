//! Producteur de décision pour StrongFather
//!
//! Ce module implémente le producteur de décision qui génère la décision finale
//! à partir des résultats d'évaluation des politiques.
//! Conforme au contrat Core Decision Contract et Architecture & Flows (section 3.6).
//!
//! **Invariants respectés :**
//! - INV-DEC-1 : Décisions non ambiguës
//! - INV-DEC-2 : Décisions justifiées
//! - INV-DEC-3 : Unicité de décision
//! - INV-DIFF-NOPLAN : Décision différée sans planification
//!
//! **Garanties respectées :**
//! - G-DEC-1 à G-DEC-3 : Garanties décisionnelles
//! - G-JUST-1 à G-JUST-3 : Garanties de justification
//! - G-NOEXEC-1 à G-NOEXEC-3 : Garanties de non-exécution

use crate::decision::{Decision, DecisionMetadata, EvaluationContext};
use crate::error::SFError;
use crate::intent::Intent;
use crate::result_composer::ComposedResult;
use crate::types::{
    Clarification, DecisionType, DeferralReason, Justification, Priority, RefusalReason,
};

/// Producteur de décision pour StrongFather
///
/// Génère la décision finale à partir des résultats d'évaluation des politiques.
/// Conforme au contrat Core Decision Contract et Architecture & Flows (section 3.6).
///
/// **Responsabilités :**
/// - Produire la décision (ACCEPTÉE, REFUSÉE, AMBIGUË, DIFFÉRÉE)
/// - Assembler la justification
/// - Référencer les politiques appliquées
/// - Créer le contexte d'évaluation
///
/// **Invariants :**
/// - INV-DEC-1 : Décisions non ambiguës (toujours un type clair)
/// - INV-DEC-2 : Décisions justifiées (justification obligatoire)
/// - INV-DEC-3 : Unicité de décision (une seule décision par intention)
/// - INV-DIFF-NOPLAN : Décision différée sans planification
///
/// **Garanties :**
/// - G-DEC-1 : Décision toujours produite (pas d'erreur pour un rejet)
/// - G-DEC-2 : Décision cohérente avec les résultats d'évaluation
/// - G-DEC-3 : Décision traçable (référence aux politiques)
/// - G-JUST-1 : Justification explicite (toujours présente)
/// - G-JUST-2 : Référence aux politiques (justification référence les politiques)
/// - G-JUST-3 : Justification complète (justification non ambiguë)
/// - G-NOEXEC-1 : Non-exécutabilité (décision jamais exécutable)
/// - G-NOEXEC-2 : Aucune autorité sur l'exécution
/// - G-NOEXEC-3 : Aucune planification (décision différée sans planification)
#[derive(Debug, Clone)]
pub struct DecisionProducer;

impl DecisionProducer {
    /// Crée un nouveau producteur de décision
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::producer::DecisionProducer;
    ///
    /// let producer = DecisionProducer::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Produit une décision à partir d'une intention et des résultats d'évaluation
    ///
    /// Génère la décision finale selon le résultat composé de l'évaluation des politiques.
    /// Conforme au contrat Core Decision Contract et Architecture & Flows (section 3.6).
    ///
    /// **Processus de production :**
    /// 1. Analyse le résultat composé (AllSatisfied, Violated, Ambiguous, Deferred)
    /// 2. Détermine le type de décision approprié
    /// 3. Construit la justification avec références aux politiques
    /// 4. Crée le contexte d'évaluation à partir du contexte d'appel
    /// 5. Assemble la décision complète
    ///
    /// **Règles de production :**
    /// - `AllSatisfied` → `DecisionType::Accepted { priority }` (priorité requise)
    /// - `Violated` → `DecisionType::Refused { reason, violated_policies }`
    /// - `Ambiguous` → `DecisionType::Ambiguous { missing_information, clarifications_required }`
    /// - `Deferred` → `DecisionType::Deferred { reason, context_required }`
    ///
    /// Conforme à :
    /// - Core Decision Contract (sections 2, 3, 4, 5)
    /// - Architecture & Flows (section 3.6)
    /// - INV-DEC-1, INV-DEC-2, INV-DEC-3, INV-DIFF-NOPLAN
    /// - G-DEC-1, G-DEC-2, G-DEC-3, G-JUST-1, G-JUST-2, G-JUST-3, G-NOEXEC-1, G-NOEXEC-2, G-NOEXEC-3
    ///
    /// # Arguments
    /// * `intent` - Intention évaluée
    /// * `result` - Résultat composé de l'évaluation des politiques
    /// * `priority` - Priorité calculée (requise pour AllSatisfied, None pour les autres)
    ///
    /// # Retour
    /// Décision produite, ou erreur si la production échoue.
    ///
    /// # Erreurs
    /// - `ConsistencyError` : Si une incohérence est détectée (ex: AllSatisfied sans priorité)
    /// - `StructuralError` : Si une erreur structurelle est détectée
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::producer::DecisionProducer;
    /// use miyukini_strongfather::result_composer::ComposedResult;
    /// use miyukini_strongfather::intent::{Intent, CallContext, IntentData};
    /// use miyukini_strongfather::types::{ActionType, Priority};
    /// use miyukini_strongfather::policy_engine::PolicyResult;
    /// use miyukini_strongfather::types::{PolicyEffect, PolicyPriority};
    ///
    /// let producer = DecisionProducer::new();
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
    /// let priority = Some(Priority::new(100));
    /// let decision = producer.produce(&intent, &result, priority).unwrap();
    /// ```
    pub fn produce(
        &self,
        intent: &Intent,
        result: &ComposedResult,
        priority: Option<Priority>,
    ) -> Result<Decision, SFError> {
        // Construire le contexte d'évaluation à partir du contexte d'appel
        let evaluation_context = EvaluationContext::new(
            intent.call_context().caller_identity().to_string(),
            intent.call_context().origin().to_string(),
            intent.call_context().instance().to_string(),
        );

        // Construire la justification
        let justification = self.build_justification(intent, result)?;

        // Extraire les identifiants des politiques appliquées
        let policies_applied = result.policy_ids();

        // Produire la décision selon le type de résultat
        match result {
            ComposedResult::AllSatisfied { .. } => {
                // Décision ACCEPTÉE (Core Decision Contract section 3.1)
                // La priorité est requise pour une décision acceptée
                let final_priority = priority.ok_or_else(|| {
                    SFError::consistency(
                        "La priorité est requise pour une décision acceptée",
                        "DecisionProducer::produce",
                    )
                })?;

                Ok(Decision::new(
                    intent.intent_id().to_string(),
                    DecisionType::Accepted {
                        priority: final_priority,
                    },
                    justification,
                    policies_applied,
                    evaluation_context,
                    DecisionMetadata::new(),
                ))
            }
            ComposedResult::Violated { policies, .. } => {
                // Décision REFUSÉE (Core Decision Contract section 3.2)
                let reason = self.build_refusal_reason(policies, result)?;

                Ok(Decision::new(
                    intent.intent_id().to_string(),
                    DecisionType::Refused {
                        reason,
                        violated_policies: policies.clone(),
                    },
                    justification,
                    policies_applied,
                    evaluation_context,
                    DecisionMetadata::new(),
                ))
            }
            ComposedResult::Ambiguous { missing, .. } => {
                // Décision AMBIGUË (Core Decision Contract section 3.3)
                let clarifications = self.build_clarifications(missing)?;

                Ok(Decision::new(
                    intent.intent_id().to_string(),
                    DecisionType::Ambiguous {
                        missing_information: missing.clone(),
                        clarifications_required: clarifications,
                    },
                    justification,
                    policies_applied,
                    evaluation_context,
                    DecisionMetadata::new(),
                ))
            }
            ComposedResult::Deferred {
                reason,
                context_required,
                ..
            } => {
                // Décision DIFFÉRÉE (Core Decision Contract section 3.4)
                // Conforme à INV-DIFF-NOPLAN (pas de planification)
                Ok(Decision::new(
                    intent.intent_id().to_string(),
                    DecisionType::Deferred {
                        reason: reason.clone(),
                        context_required: context_required.clone(),
                    },
                    justification,
                    policies_applied,
                    evaluation_context,
                    DecisionMetadata::new(),
                ))
            }
        }
    }

    /// Construit la justification d'une décision
    ///
    /// Crée une justification explicite qui référence les politiques appliquées
    /// et explique la décision produite.
    ///
    /// Conforme à G-JUST-1, G-JUST-2, G-JUST-3.
    ///
    /// # Arguments
    /// * `intent` - Intention évaluée
    /// * `result` - Résultat composé de l'évaluation
    ///
    /// # Retour
    /// Justification construite, ou erreur si la construction échoue.
    fn build_justification(
        &self,
        intent: &Intent,
        result: &ComposedResult,
    ) -> Result<Justification, SFError> {
        let policy_ids = result.policy_ids();

        // Construire l'explication selon le type de résultat
        let explanation = match result {
            ComposedResult::AllSatisfied { .. } => {
                if policy_ids.is_empty() {
                    format!(
                        "Intention '{}' acceptée : toutes les politiques sont satisfaites (aucune politique applicable)",
                        intent.intent_id()
                    )
                } else {
                    format!(
                        "Intention '{}' acceptée : toutes les politiques sont satisfaites ({} politique(s) appliquée(s))",
                        intent.intent_id(),
                        policy_ids.len()
                    )
                }
            }
            ComposedResult::Violated { policies, .. } => {
                format!(
                    "Intention '{}' refusée : {} politique(s) violée(s) : {}",
                    intent.intent_id(),
                    policies.len(),
                    policies.join(", ")
                )
            }
            ComposedResult::Ambiguous { missing, .. } => {
                format!(
                    "Intention '{}' ambiguë : informations manquantes ({})",
                    intent.intent_id(),
                    missing.join(", ")
                )
            }
            ComposedResult::Deferred {
                reason,
                context_required,
                ..
            } => {
                let reason_desc = match reason {
                    DeferralReason::ContextRequired { description } => description.clone(),
                    DeferralReason::InformationMissing { description } => description.clone(),
                    DeferralReason::ConditionNotMet { description } => description.clone(),
                    DeferralReason::Custom { description } => description.clone(),
                };
                format!(
                    "Intention '{}' différée : {} (contexte requis : {})",
                    intent.intent_id(),
                    reason_desc,
                    context_required.join(", ")
                )
            }
        };

        // Construire la justification avec références aux politiques
        Ok(Justification::with_policies(explanation, policy_ids))
    }

    /// Construit la raison de refus
    ///
    /// Crée une raison de refus explicite basée sur les politiques violées.
    ///
    /// # Arguments
    /// * `violated_policies` - Identifiants des politiques violées
    /// * `_result` - Résultat composé de l'évaluation (réservé pour usage futur)
    ///
    /// # Retour
    /// Raison de refus construite, ou erreur si la construction échoue.
    fn build_refusal_reason(
        &self,
        violated_policies: &[String],
        _result: &ComposedResult,
    ) -> Result<RefusalReason, SFError> {
        if violated_policies.is_empty() {
            return Err(SFError::consistency(
                "Impossible de construire une raison de refus sans politique violée",
                "DecisionProducer::build_refusal_reason",
            ));
        }

        // Construire la description de la violation
        let description = if violated_policies.len() == 1 {
            format!("Politique violée : {}", violated_policies[0])
        } else {
            format!(
                "{} politiques violées : {}",
                violated_policies.len(),
                violated_policies.join(", ")
            )
        };

        Ok(RefusalReason::PolicyViolation { description })
    }

    /// Construit les clarifications requises
    ///
    /// Crée les clarifications nécessaires pour résoudre une ambiguïté.
    ///
    /// # Arguments
    /// * `missing` - Informations manquantes identifiées
    ///
    /// # Retour
    /// Liste des clarifications construites, ou erreur si la construction échoue.
    fn build_clarifications(
        &self,
        missing: &[String],
    ) -> Result<Vec<Clarification>, SFError> {
        if missing.is_empty() {
            return Err(SFError::consistency(
                "Impossible de construire des clarifications sans informations manquantes",
                "DecisionProducer::build_clarifications",
            ));
        }

        // Construire une clarification pour chaque information manquante
        let clarifications: Vec<Clarification> = missing
            .iter()
            .map(|info| {
                Clarification::new(
                    format!("Quelle est la valeur de '{}' ?", info),
                    info.clone(),
                )
            })
            .collect();

        Ok(clarifications)
    }
}

impl Default for DecisionProducer {
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

    #[test]
    fn test_decision_producer_new() {
        let producer = DecisionProducer::new();
        assert!(matches!(producer, DecisionProducer));
    }

    #[test]
    fn test_decision_producer_default() {
        let producer = DecisionProducer::default();
        assert!(matches!(producer, DecisionProducer));
    }

    #[test]
    fn test_produce_accepted() {
        let producer = DecisionProducer::new();
        let intent = create_test_intent();

        let policy_results = vec![PolicyResult::satisfied(
            "POL-001".to_string(),
            PolicyEffect::Allow,
            PolicyPriority::new(100),
        )];
        let result = ComposedResult::AllSatisfied { policy_results };
        let priority = Some(Priority::new(150));

        let decision = producer.produce(&intent, &result, priority).unwrap();

        assert_eq!(decision.intent_id(), "intent-001");
        assert!(decision.is_accepted());
        assert!(!decision.is_refused());
        assert!(!decision.is_ambiguous());
        assert!(!decision.is_deferred());

        match decision.decision_type() {
            DecisionType::Accepted { priority: p } => {
                assert_eq!(p.value(), 150);
            }
            _ => panic!("Expected Accepted decision"),
        }

        // Vérifier la justification
        assert!(!decision.justification().explanation.is_empty());
        assert!(!decision.justification().policy_references.is_empty());
    }

    #[test]
    fn test_produce_accepted_without_priority_should_fail() {
        let producer = DecisionProducer::new();
        let intent = create_test_intent();

        let policy_results = vec![PolicyResult::satisfied(
            "POL-001".to_string(),
            PolicyEffect::Allow,
            PolicyPriority::new(100),
        )];
        let result = ComposedResult::AllSatisfied { policy_results };

        // Pas de priorité fournie
        let error = producer.produce(&intent, &result, None).unwrap_err();
        assert!(matches!(error, SFError::ConsistencyError { .. }));
        assert!(error
            .description()
            .contains("priorité est requise pour une décision acceptée"));
    }

    #[test]
    fn test_produce_refused() {
        let producer = DecisionProducer::new();
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

        let decision = producer.produce(&intent, &result, None).unwrap();

        assert_eq!(decision.intent_id(), "intent-001");
        assert!(!decision.is_accepted());
        assert!(decision.is_refused());
        assert!(!decision.is_ambiguous());
        assert!(!decision.is_deferred());

        match decision.decision_type() {
            DecisionType::Refused {
                reason,
                violated_policies,
            } => {
                assert!(matches!(reason, RefusalReason::PolicyViolation { .. }));
                assert_eq!(violated_policies.len(), 1);
                assert_eq!(violated_policies[0], "POL-001");
            }
            _ => panic!("Expected Refused decision"),
        }

        // Vérifier la justification
        assert!(!decision.justification().explanation.is_empty());
        assert!(decision
            .justification()
            .explanation
            .contains("refusée"));
    }

    #[test]
    fn test_produce_refused_multiple_policies() {
        let producer = DecisionProducer::new();
        let intent = create_test_intent();

        let policy_results = vec![
            PolicyResult::not_satisfied(
                "POL-001".to_string(),
                PolicyEffect::Deny,
                PolicyPriority::new(100),
            ),
            PolicyResult::not_satisfied(
                "POL-002".to_string(),
                PolicyEffect::Require,
                PolicyPriority::new(50),
            ),
        ];
        let result = ComposedResult::Violated {
            policy_results,
            policies: vec!["POL-001".to_string(), "POL-002".to_string()],
        };

        let decision = producer.produce(&intent, &result, None).unwrap();

        assert!(decision.is_refused());
        match decision.decision_type() {
            DecisionType::Refused { violated_policies, .. } => {
                assert_eq!(violated_policies.len(), 2);
                assert!(violated_policies.contains(&"POL-001".to_string()));
                assert!(violated_policies.contains(&"POL-002".to_string()));
            }
            _ => panic!("Expected Refused decision"),
        }
    }

    #[test]
    fn test_produce_ambiguous() {
        let producer = DecisionProducer::new();
        let intent = create_test_intent();

        let policy_results = vec![PolicyResult::indeterminate(
            "POL-001".to_string(),
            PolicyEffect::Require,
            PolicyPriority::new(100),
        )];
        let result = ComposedResult::Ambiguous {
            policy_results,
            missing: vec!["info1".to_string(), "info2".to_string()],
        };

        let decision = producer.produce(&intent, &result, None).unwrap();

        assert_eq!(decision.intent_id(), "intent-001");
        assert!(!decision.is_accepted());
        assert!(!decision.is_refused());
        assert!(decision.is_ambiguous());
        assert!(!decision.is_deferred());

        match decision.decision_type() {
            DecisionType::Ambiguous {
                missing_information,
                clarifications_required,
            } => {
                assert_eq!(missing_information.len(), 2);
                assert_eq!(clarifications_required.len(), 2);
                assert!(missing_information.contains(&"info1".to_string()));
                assert!(missing_information.contains(&"info2".to_string()));
            }
            _ => panic!("Expected Ambiguous decision"),
        }

        // Vérifier la justification
        assert!(!decision.justification().explanation.is_empty());
        assert!(decision
            .justification()
            .explanation
            .contains("ambiguë"));
    }

    #[test]
    fn test_produce_deferred() {
        let producer = DecisionProducer::new();
        let intent = create_test_intent();

        let policy_results = vec![];
        let reason = DeferralReason::ContextRequired {
            description: "Contexte futur requis".to_string(),
        };
        let result = ComposedResult::Deferred {
            policy_results,
            reason: reason.clone(),
            context_required: vec!["context1".to_string(), "context2".to_string()],
        };

        let decision = producer.produce(&intent, &result, None).unwrap();

        assert_eq!(decision.intent_id(), "intent-001");
        assert!(!decision.is_accepted());
        assert!(!decision.is_refused());
        assert!(!decision.is_ambiguous());
        assert!(decision.is_deferred());

        match decision.decision_type() {
            DecisionType::Deferred {
                reason: r,
                context_required,
            } => {
                assert_eq!(r, &reason);
                assert_eq!(context_required.len(), 2);
                assert!(context_required.contains(&"context1".to_string()));
                assert!(context_required.contains(&"context2".to_string()));
            }
            _ => panic!("Expected Deferred decision"),
        }

        // Vérifier la justification
        assert!(!decision.justification().explanation.is_empty());
        assert!(decision
            .justification()
            .explanation
            .contains("différée"));
    }

    #[test]
    fn test_produce_all_satisfied_empty_policies() {
        let producer = DecisionProducer::new();
        let intent = create_test_intent();

        let result = ComposedResult::AllSatisfied {
            policy_results: Vec::new(),
        };
        let priority = Some(Priority::new(100));

        let decision = producer.produce(&intent, &result, priority).unwrap();

        assert!(decision.is_accepted());
        assert_eq!(decision.policies_applied().len(), 0);
    }

    #[test]
    fn test_evaluation_context_from_call_context() {
        let producer = DecisionProducer::new();
        let intent = create_test_intent();

        let result = ComposedResult::AllSatisfied {
            policy_results: Vec::new(),
        };
        let priority = Some(Priority::new(100));

        let decision = producer.produce(&intent, &result, priority).unwrap();

        let eval_context = decision.evaluation_context();
        assert_eq!(eval_context.caller_identity(), "user123");
        assert_eq!(eval_context.origin(), "product");
        assert_eq!(eval_context.instance(), "instance1");
    }

    #[test]
    fn test_justification_always_present() {
        let producer = DecisionProducer::new();
        let intent = create_test_intent();

        // Tester pour tous les types de décisions
        let test_cases = vec![
            (
                ComposedResult::AllSatisfied {
                    policy_results: Vec::new(),
                },
                Some(Priority::new(100)),
            ),
            (
                ComposedResult::Violated {
                    policy_results: Vec::new(),
                    policies: vec!["POL-001".to_string()],
                },
                None,
            ),
            (
                ComposedResult::Ambiguous {
                    policy_results: Vec::new(),
                    missing: vec!["info1".to_string()],
                },
                None,
            ),
            (
                ComposedResult::Deferred {
                    policy_results: Vec::new(),
                    reason: DeferralReason::ContextRequired {
                        description: "Test".to_string(),
                    },
                    context_required: vec!["context1".to_string()],
                },
                None,
            ),
        ];

        for (result, priority) in test_cases {
            let decision = producer.produce(&intent, &result, priority).unwrap();
            assert!(!decision.justification().explanation.is_empty());
            // G-JUST-2 : Référence aux politiques
            // Les policy_references peuvent être vides si aucune politique n'est applicable
            // mais la justification doit toujours être présente (G-JUST-1)
        }
    }

    #[test]
    fn test_policies_applied_always_present() {
        let producer = DecisionProducer::new();
        let intent = create_test_intent();

        let policy_results = vec![
            PolicyResult::satisfied(
                "POL-001".to_string(),
                PolicyEffect::Allow,
                PolicyPriority::new(100),
            ),
            PolicyResult::satisfied(
                "POL-002".to_string(),
                PolicyEffect::Require,
                PolicyPriority::new(50),
            ),
        ];
        let result = ComposedResult::AllSatisfied {
            policy_results: policy_results.clone(),
        };
        let priority = Some(Priority::new(100));

        let decision = producer.produce(&intent, &result, priority).unwrap();

        assert_eq!(decision.policies_applied().len(), 2);
        assert!(decision
            .policies_applied()
            .contains(&"POL-001".to_string()));
        assert!(decision
            .policies_applied()
            .contains(&"POL-002".to_string()));
    }

    #[test]
    fn test_metadata_always_present() {
        let producer = DecisionProducer::new();
        let intent = create_test_intent();

        let result = ComposedResult::AllSatisfied {
            policy_results: Vec::new(),
        };
        let priority = Some(Priority::new(100));

        let decision = producer.produce(&intent, &result, priority).unwrap();

        // Les métadonnées doivent toujours être présentes (même si vides)
        let _metadata = decision.metadata();
    }

    #[test]
    fn test_decision_unicity() {
        let producer = DecisionProducer::new();
        let intent = create_test_intent();

        let result = ComposedResult::AllSatisfied {
            policy_results: Vec::new(),
        };
        let priority = Some(Priority::new(100));

        // Produire plusieurs fois la même décision
        let decision1 = producer.produce(&intent, &result, priority).unwrap();
        let decision2 = producer.produce(&intent, &result, priority).unwrap();

        // INV-DEC-3 : Unicité de décision
        // Les décisions doivent être identiques pour la même intention et le même résultat
        assert_eq!(decision1.intent_id(), decision2.intent_id());
        assert_eq!(decision1.decision_type(), decision2.decision_type());
    }
}
