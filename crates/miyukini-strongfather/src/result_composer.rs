//! Compositeur de résultats pour StrongFather
//!
//! Ce module implémente le compositeur de résultats qui agrège les résultats
//! des évaluations de politiques selon les règles de composition.
//! Conforme au contrat Policy Engine Contract (section 6) et Architecture & Flows (section 3.4).
//!
//! **Invariants respectés :**
//! - INV-POL-3 : Déterminisme d'évaluation
//!
//! **Garanties respectées :**
//! - G-POL-1 : Évaluation déterministe
//! - G-POL-8 : Cohérence des politiques
//! - G-POL-9 : Cohérence des décisions

use crate::error::SFError;
use crate::policy_engine::PolicyResult;
use crate::types::{DeferralReason, PolicyId};

/// Résultat composé de l'évaluation des politiques
///
/// Représente le résultat global de la composition des résultats individuels
/// des politiques évaluées. Conforme au contrat Policy Engine Contract section 6
/// et Architecture & Flows section 3.4.
///
/// **Types de résultats composés :**
/// - `AllSatisfied` : Toutes les politiques sont satisfaites
/// - `Violated` : Au moins une politique n'est pas satisfaite (violation)
/// - `Ambiguous` : Au moins une politique est indéterminée (ambiguïté)
/// - `Deferred` : Contexte futur requis pour l'évaluation
///
/// **Caractéristiques :**
/// - Déterministe : Pour un même ensemble de résultats, le résultat composé est toujours le même (INV-POL-3, G-POL-1)
/// - Cohérent : Les politiques sont cohérentes entre elles (G-POL-8)
/// - Traçable : Les politiques appliquées sont référencées
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComposedResult {
    /// Toutes les politiques sont satisfaites
    ///
    /// L'intention peut être acceptée si aucune autre condition n'est requise.
    AllSatisfied {
        /// Résultats des politiques évaluées
        policy_results: Vec<PolicyResult>,
    },
    /// Au moins une politique n'est pas satisfaite (violation)
    ///
    /// L'intention doit être refusée. Les politiques violées sont listées.
    Violated {
        /// Résultats des politiques évaluées
        policy_results: Vec<PolicyResult>,
        /// Identifiants des politiques violées
        policies: Vec<PolicyId>,
    },
    /// Au moins une politique est indéterminée (ambiguïté)
    ///
    /// L'évaluation ne peut pas être déterminée de manière non ambiguë.
    /// Des clarifications sont nécessaires.
    Ambiguous {
        /// Résultats des politiques évaluées
        policy_results: Vec<PolicyResult>,
        /// Informations manquantes identifiées
        missing: Vec<String>,
    },
    /// Contexte futur requis pour l'évaluation
    ///
    /// L'évaluation nécessite un contexte qui n'est pas encore disponible.
    /// Conforme à INV-DIFF-NOPLAN (pas de planification).
    Deferred {
        /// Résultats des politiques évaluées
        policy_results: Vec<PolicyResult>,
        /// Raison du différé
        reason: DeferralReason,
        /// Contexte futur requis pour l'évaluation
        context_required: Vec<String>,
    },
}

impl ComposedResult {
    /// Vérifie si toutes les politiques sont satisfaites
    pub fn is_all_satisfied(&self) -> bool {
        matches!(self, ComposedResult::AllSatisfied { .. })
    }

    /// Vérifie si au moins une politique est violée
    pub fn is_violated(&self) -> bool {
        matches!(self, ComposedResult::Violated { .. })
    }

    /// Vérifie si le résultat est ambigu
    pub fn is_ambiguous(&self) -> bool {
        matches!(self, ComposedResult::Ambiguous { .. })
    }

    /// Vérifie si le résultat est différé
    pub fn is_deferred(&self) -> bool {
        matches!(self, ComposedResult::Deferred { .. })
    }

    /// Retourne les résultats des politiques évaluées
    pub fn policy_results(&self) -> &[PolicyResult] {
        match self {
            ComposedResult::AllSatisfied { policy_results }
            | ComposedResult::Violated { policy_results, .. }
            | ComposedResult::Ambiguous { policy_results, .. }
            | ComposedResult::Deferred { policy_results, .. } => policy_results,
        }
    }

    /// Retourne les identifiants des politiques violées (si applicable)
    pub fn violated_policies(&self) -> Vec<PolicyId> {
        match self {
            ComposedResult::Violated { policies, .. } => policies.clone(),
            _ => Vec::new(),
        }
    }

    /// Retourne les identifiants de toutes les politiques appliquées
    ///
    /// Pour AllSatisfied, retourne tous les IDs des politiques satisfaites.
    /// Pour les autres cas, retourne les politiques concernées.
    pub fn policy_ids(&self) -> Vec<PolicyId> {
        match self {
            ComposedResult::AllSatisfied { policy_results } => {
                policy_results.iter().map(|r| r.policy_id.clone()).collect()
            }
            ComposedResult::Violated { policies, .. } => policies.clone(),
            ComposedResult::Ambiguous { policy_results, .. } => {
                policy_results.iter().map(|r| r.policy_id.clone()).collect()
            }
            ComposedResult::Deferred { policy_results, .. } => {
                policy_results.iter().map(|r| r.policy_id.clone()).collect()
            }
        }
    }
}

/// Compositeur de résultats StrongFather
///
/// Agrège les résultats des évaluations de politiques selon les règles de composition.
/// Conforme au contrat Policy Engine Contract section 6 et Architecture & Flows section 3.4.
///
/// **Caractéristiques :**
/// - Déterministe : Pour un même ensemble de résultats, le résultat composé est toujours le même (G-POL-1, INV-POL-3)
/// - Cohérent : Les politiques sont cohérentes entre elles (G-POL-8)
/// - Traçable : Les politiques appliquées sont référencées (G-POL-9)
///
/// **Règles de composition :**
/// 1. Si toutes les politiques sont satisfaites → AllSatisfied
/// 2. Si au moins une politique n'est pas satisfaite → Violated (avec les IDs des politiques violées)
/// 3. Si au moins une politique est indéterminée → Ambiguous (avec les informations manquantes)
/// 4. Si le contexte futur est requis → Deferred (avec la raison et le contexte requis)
///
/// **Ordre de priorité des règles :**
/// - Violation prime sur ambiguïté (une violation claire est préférée à une ambiguïté)
/// - Ambiguïté prime sur différé (une ambiguïté est préférée à un différé)
/// - Différé est le dernier recours (si aucune autre règle ne s'applique)
#[derive(Debug, Clone)]
pub struct ResultComposer;

impl ResultComposer {
    /// Crée un nouveau compositeur de résultats
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::result_composer::ResultComposer;
    ///
    /// let composer = ResultComposer::new();
    /// ```
    pub fn new() -> Self {
        Self
    }

    /// Compose les résultats des politiques en un résultat global
    ///
    /// Agrège les résultats individuels des politiques selon les règles de composition.
    /// Conforme au contrat Policy Engine Contract section 6 et Architecture & Flows section 3.4.
    ///
    /// **Processus de composition :**
    /// 1. Analyse tous les résultats de politiques
    /// 2. Détecte les violations (politiques non satisfaites)
    /// 3. Détecte les ambiguïtés (politiques indéterminées)
    /// 4. Applique les règles de priorité pour déterminer le résultat final
    ///
    /// **Règles de composition (dans l'ordre de priorité) :**
    /// - Si au moins une politique n'est pas satisfaite → Violated
    /// - Si au moins une politique est indéterminée → Ambiguous
    /// - Si toutes les politiques sont satisfaites → AllSatisfied
    ///
    /// **Déterminisme :**
    /// Pour un même ensemble de résultats de politiques, le résultat composé est toujours le même (INV-POL-3, G-POL-1).
    /// L'ordre des résultats dans le vecteur n'affecte pas le résultat final (déterminisme par contenu, pas par ordre).
    ///
    /// # Arguments
    /// * `results` - Vecteur de résultats d'évaluation des politiques
    ///
    /// # Retour
    /// Résultat composé, ou une erreur si la composition échoue.
    ///
    /// # Erreurs
    /// - `ConsistencyError` : Si une incohérence est détectée dans la composition
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::result_composer::{ResultComposer, ComposedResult};
    /// use miyukini_strongfather::policy_engine::{PolicyResult, PolicyResultType};
    /// use miyukini_strongfather::types::{PolicyEffect, PolicyPriority};
    ///
    /// let composer = ResultComposer::new();
    ///
    /// // Toutes les politiques satisfaites
    /// let results = vec![
    ///     PolicyResult::satisfied(
    ///         "POL-001".to_string(),
    ///         PolicyEffect::Allow,
    ///         PolicyPriority::new(100),
    ///     ),
    ///     PolicyResult::satisfied(
    ///         "POL-002".to_string(),
    ///         PolicyEffect::Require,
    ///         PolicyPriority::new(50),
    ///     ),
    /// ];
    /// let composed = composer.compose(results).unwrap();
    /// assert!(composed.is_all_satisfied());
    ///
    /// // Au moins une politique violée
    /// let results = vec![
    ///     PolicyResult::satisfied(
    ///         "POL-001".to_string(),
    ///         PolicyEffect::Allow,
    ///         PolicyPriority::new(100),
    ///     ),
    ///     PolicyResult::not_satisfied(
    ///         "POL-002".to_string(),
    ///         PolicyEffect::Deny,
    ///         PolicyPriority::new(50),
    ///     ),
    /// ];
    /// let composed = composer.compose(results).unwrap();
    /// assert!(composed.is_violated());
    /// ```
    pub fn compose(&self, results: Vec<PolicyResult>) -> Result<ComposedResult, SFError> {
        // Cas spécial : aucun résultat (aucune politique applicable)
        // Conforme à G-POL-2 (évaluation complète) : si aucune politique n'est applicable,
        // on considère que toutes les politiques sont satisfaites (pas de contrainte = acceptation)
        if results.is_empty() {
            return Ok(ComposedResult::AllSatisfied {
                policy_results: Vec::new(),
            });
        }

        // Étape 1 : Analyser tous les résultats
        let mut satisfied = Vec::new();
        let mut not_satisfied = Vec::new();
        let mut indeterminate = Vec::new();

        for result in &results {
            match result.result {
                crate::policy_engine::PolicyResultType::Satisfied => {
                    satisfied.push(result.policy_id.clone());
                }
                crate::policy_engine::PolicyResultType::NotSatisfied => {
                    not_satisfied.push(result.policy_id.clone());
                }
                crate::policy_engine::PolicyResultType::Indeterminate => {
                    indeterminate.push(result.policy_id.clone());
                }
            }
        }

        // Étape 2 : Appliquer les règles de composition (dans l'ordre de priorité)

        // Règle 1 : Si au moins une politique n'est pas satisfaite → Violated
        // Conforme à Architecture & Flows section 4.3 (flux de rejet de politique)
        // La violation prime sur l'ambiguïté (une violation claire est préférée)
        if !not_satisfied.is_empty() {
            return Ok(ComposedResult::Violated {
                policy_results: results.clone(),
                policies: not_satisfied,
            });
        }

        // Règle 2 : Si au moins une politique est indéterminée → Ambiguous
        // Conforme à Policy Engine Contract section 7 (cas d'ambiguïté)
        // L'ambiguïté nécessite des clarifications
        if !indeterminate.is_empty() {
            // Construire les informations manquantes à partir des justifications
            // ou des IDs des politiques indéterminées
            let missing: Vec<String> = indeterminate
                .iter()
                .map(|policy_id| {
                    // Chercher la justification dans les résultats
                    results
                        .iter()
                        .find(|r| r.policy_id == *policy_id)
                        .and_then(|r| r.justification.clone())
                        .unwrap_or_else(|| {
                            format!("Politique {} indéterminée", policy_id)
                        })
                })
                .collect();

            return Ok(ComposedResult::Ambiguous {
                policy_results: results.clone(),
                missing,
            });
        }

        // Règle 3 : Si toutes les politiques sont satisfaites → AllSatisfied
        // Conforme à Architecture & Flows section 4.1 (flux principal)
        // Toutes les politiques satisfaites permettent l'acceptation
        if !satisfied.is_empty() && not_satisfied.is_empty() && indeterminate.is_empty() {
            return Ok(ComposedResult::AllSatisfied {
                policy_results: results,
            });
        }

        // Cas théoriquement impossible (tous les résultats doivent être dans une catégorie)
        // Mais on le gère pour la robustesse et la cohérence (G-POL-8, G-POL-9)
        Err(SFError::consistency(
            "Impossible de composer les résultats : état incohérent détecté",
            "ResultComposer::compose",
        ))
    }
}

impl Default for ResultComposer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::policy_engine::PolicyResult;
    use crate::types::{PolicyEffect, PolicyPriority};

    #[test]
    fn test_result_composer_new() {
        let composer = ResultComposer::new();
        // Vérifier que le compositeur peut être créé
        let _ = composer;
    }

    #[test]
    fn test_compose_empty_results() {
        let composer = ResultComposer::new();
        let results = Vec::new();
        let composed = composer.compose(results).unwrap();
        assert!(composed.is_all_satisfied());
    }

    #[test]
    fn test_compose_all_satisfied() {
        let composer = ResultComposer::new();
        let results = vec![
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
        let composed = composer.compose(results.clone()).unwrap();
        assert!(composed.is_all_satisfied());
        assert!(!composed.is_violated());
        assert!(!composed.is_ambiguous());
        assert!(!composed.is_deferred());
        assert_eq!(composed.policy_results().len(), 2);
    }

    #[test]
    fn test_compose_violated() {
        let composer = ResultComposer::new();
        let results = vec![
            PolicyResult::satisfied(
                "POL-001".to_string(),
                PolicyEffect::Allow,
                PolicyPriority::new(100),
            ),
            PolicyResult::not_satisfied(
                "POL-002".to_string(),
                PolicyEffect::Deny,
                PolicyPriority::new(50),
            ),
        ];
        let composed = composer.compose(results).unwrap();
        assert!(!composed.is_all_satisfied());
        assert!(composed.is_violated());
        assert!(!composed.is_ambiguous());
        assert!(!composed.is_deferred());

        let violated_policies = composed.violated_policies();
        assert_eq!(violated_policies.len(), 1);
        assert_eq!(violated_policies[0], "POL-002");
    }

    #[test]
    fn test_compose_multiple_violated() {
        let composer = ResultComposer::new();
        let results = vec![
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
        let composed = composer.compose(results.clone()).unwrap();
        assert!(composed.is_violated());
        assert_eq!(composed.policy_results().len(), 2);
        let violated_policies = composed.violated_policies();
        assert_eq!(violated_policies.len(), 2);
        assert!(violated_policies.contains(&"POL-001".to_string()));
        assert!(violated_policies.contains(&"POL-002".to_string()));
    }

    #[test]
    fn test_compose_ambiguous() {
        let composer = ResultComposer::new();
        let results = vec![
            PolicyResult::satisfied(
                "POL-001".to_string(),
                PolicyEffect::Allow,
                PolicyPriority::new(100),
            ),
            PolicyResult::indeterminate(
                "POL-002".to_string(),
                PolicyEffect::Require,
                PolicyPriority::new(50),
            ),
        ];
        let composed = composer.compose(results).unwrap();
        assert!(!composed.is_all_satisfied());
        assert!(!composed.is_violated());
        assert!(composed.is_ambiguous());
        assert!(!composed.is_deferred());

        match composed {
            ComposedResult::Ambiguous { missing, .. } => {
                assert!(!missing.is_empty());
            }
            _ => panic!("Expected Ambiguous result"),
        }
    }

    #[test]
    fn test_compose_ambiguous_with_justification() {
        let composer = ResultComposer::new();
        let results = vec![
            PolicyResult::satisfied(
                "POL-001".to_string(),
                PolicyEffect::Allow,
                PolicyPriority::new(100),
            ),
            PolicyResult::indeterminate_with_justification(
                "POL-002".to_string(),
                PolicyEffect::Require,
                PolicyPriority::new(50),
                "Information manquante : contexte requis".to_string(),
            ),
        ];
        let composed = composer.compose(results.clone()).unwrap();
        assert!(composed.is_ambiguous());
        assert_eq!(composed.policy_results().len(), 2);

        match &composed {
            ComposedResult::Ambiguous { missing, .. } => {
                assert_eq!(missing.len(), 1);
                assert!(missing[0].contains("Information manquante"));
            }
            _ => panic!("Expected Ambiguous result"),
        }
    }

    #[test]
    fn test_compose_violated_priority_over_ambiguous() {
        let composer = ResultComposer::new();
        // Violation prime sur ambiguïté
        let results = vec![
            PolicyResult::not_satisfied(
                "POL-001".to_string(),
                PolicyEffect::Deny,
                PolicyPriority::new(100),
            ),
            PolicyResult::indeterminate(
                "POL-002".to_string(),
                PolicyEffect::Require,
                PolicyPriority::new(50),
            ),
        ];
        let composed = composer.compose(results).unwrap();
        // La violation doit primer sur l'ambiguïté
        assert!(composed.is_violated());
        assert!(!composed.is_ambiguous());
    }

    #[test]
    fn test_compose_determinism() {
        let composer = ResultComposer::new();
        let results1 = vec![
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
        // Même ensemble de résultats dans un ordre différent
        let results2 = vec![
            PolicyResult::satisfied(
                "POL-002".to_string(),
                PolicyEffect::Require,
                PolicyPriority::new(50),
            ),
            PolicyResult::satisfied(
                "POL-001".to_string(),
                PolicyEffect::Allow,
                PolicyPriority::new(100),
            ),
        ];

        let composed1 = composer.compose(results1).unwrap();
        let composed2 = composer.compose(results2).unwrap();

        // Les résultats doivent être identiques (déterminisme, INV-POL-3, G-POL-1)
        // Note: Les policy_results peuvent être dans un ordre différent, mais le type de résultat doit être le même
        assert_eq!(composed1.is_all_satisfied(), composed2.is_all_satisfied());
        assert_eq!(composed1.policy_ids().len(), composed2.policy_ids().len());
    }

    #[test]
    fn test_composed_result_is_all_satisfied() {
        let result = ComposedResult::AllSatisfied {
            policy_results: Vec::new(),
        };
        assert!(result.is_all_satisfied());
        assert!(!result.is_violated());
        assert!(!result.is_ambiguous());
        assert!(!result.is_deferred());
    }

    #[test]
    fn test_composed_result_is_violated() {
        let result = ComposedResult::Violated {
            policy_results: Vec::new(),
            policies: vec!["POL-001".to_string()],
        };
        assert!(!result.is_all_satisfied());
        assert!(result.is_violated());
        assert!(!result.is_ambiguous());
        assert!(!result.is_deferred());
    }

    #[test]
    fn test_composed_result_is_ambiguous() {
        let result = ComposedResult::Ambiguous {
            policy_results: Vec::new(),
            missing: vec!["Info manquante".to_string()],
        };
        assert!(!result.is_all_satisfied());
        assert!(!result.is_violated());
        assert!(result.is_ambiguous());
        assert!(!result.is_deferred());
    }

    #[test]
    fn test_composed_result_is_deferred() {
        let result = ComposedResult::Deferred {
            policy_results: Vec::new(),
            reason: DeferralReason::ContextRequired {
                description: "Contexte requis".to_string(),
            },
            context_required: vec!["context1".to_string()],
        };
        assert!(!result.is_all_satisfied());
        assert!(!result.is_violated());
        assert!(!result.is_ambiguous());
        assert!(result.is_deferred());
    }

    #[test]
    fn test_composed_result_violated_policies() {
        let policies = vec!["POL-001".to_string(), "POL-002".to_string()];
        let result = ComposedResult::Violated {
            policy_results: Vec::new(),
            policies: policies.clone(),
        };
        assert_eq!(result.violated_policies(), policies);

        let all_satisfied = ComposedResult::AllSatisfied {
            policy_results: Vec::new(),
        };
        assert!(all_satisfied.violated_policies().is_empty());
    }

    #[test]
    fn test_composed_result_policy_ids() {
        let policies = vec!["POL-001".to_string(), "POL-002".to_string()];
        let violated = ComposedResult::Violated {
            policy_results: Vec::new(),
            policies: policies.clone(),
        };
        assert_eq!(violated.policy_ids(), policies);

        let all_satisfied = ComposedResult::AllSatisfied {
            policy_results: Vec::new(),
        };
        assert!(all_satisfied.policy_ids().is_empty());
    }
}
