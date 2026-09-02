//! # StrongFather — Modèle de Décision
//!
//! Ce module définit le modèle conceptuel des décisions produites par StrongFather.
//!
//! ## Références
//!
//! - [Core Decision Contract](../../../docs/core/StrongFather/contracts/decision/StrongFather%20-%20Core%20Decision%20Contract.md)
//! - [Documentation Fondatrice](../../../docs/core/StrongFather/foundation/StrongFather%20-%20Documentation%20Fondatrice.md)

use std::collections::HashMap;

use crate::policy::PolicyId;

/// @id strongfather_decision_type
/// @role domain_model
/// @layer core
/// @do define_decision_type
//@human / @human Type de décision StrongFather. Définit le résultat de l'évaluation d'une intention.
/// Type de décision
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionType {
    /// @id strongfather_decision_type_accepted
    /// @role domain_model
    /// @layer core
    /// @do represent_accepted_decision
    //@human / @human Décision acceptée - l'intention est valide selon les politiques et peut être considérée pour exécution.
    Accepted {
        /// @id strongfather_decision_accepted_priority
        /// @role domain_model
        /// @layer core
        /// @do store_established_priority
        //@human / @human Priorité relative établie pour l'intention.
        priority: u32,
    },
    /// @id strongfather_decision_type_refused
    /// @role domain_model
    /// @layer core
    /// @do represent_refused_decision
    //@human / @human Décision refusée - l'intention est invalide selon les politiques et ne doit pas être exécutée.
    Refused {
        /// @id strongfather_decision_refused_reason
        /// @role domain_model
        /// @layer core
        /// @do store_refusal_reason
        //@human / @human Raison explicite du refus.
        reason: String,
        /// @id strongfather_decision_refused_violated_policies
        /// @role domain_model
        /// @layer core
        /// @do store_violated_policies
        //@human / @human Liste des politiques violées qui ont conduit au refus.
        violated_policies: Vec<PolicyId>,
    },
    /// @id strongfather_decision_type_ambiguous
    /// @role domain_model
    /// @layer core
    /// @do represent_ambiguous_decision
    //@human / @human Décision ambiguë - l'intention est insuffisamment définie et nécessite des clarifications.
    Ambiguous {
        /// @id strongfather_decision_ambiguous_missing_information
        /// @role domain_model
        /// @layer core
        /// @do store_missing_information
        //@human / @human Liste des éléments manquants ou insuffisamment définis.
        missing_information: Vec<String>,
        /// @id strongfather_decision_ambiguous_clarifications
        /// @role domain_model
        /// @layer core
        /// @do store_required_clarifications
        //@human / @human Clarifications nécessaires pour permettre l'évaluation.
        clarifications_required: Vec<String>,
    },
    /// @id strongfather_decision_type_deferred
    /// @role domain_model
    /// @layer core
    /// @do represent_deferred_decision
    //@human / @human Décision différée - l'intention ne peut pas être évaluée immédiatement car elle dépend d'un contexte futur.
    Deferred {
        /// @id strongfather_decision_deferred_reason
        /// @role domain_model
        /// @layer core
        /// @do store_deferral_reason
        //@human / @human Raison de la différation (contexte futur requis).
        reason: String,
        /// @id strongfather_decision_deferred_context_required
        /// @role domain_model
        /// @layer core
        /// @do store_required_context
        //@human / @human Contexte futur requis pour l'évaluation (INV-DIFF-NOPLAN : pas de planification).
        context_required: Vec<String>,
    },
}

/// @id strongfather_decision_justification
/// @role domain_model
/// @layer core
/// @do represent_decision_justification
//@human / @human Justification d'une décision. Explication conceptuelle de la décision produite (G-JUST-1 : justification obligatoire).
/// Justification d'une décision
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Justification {
    /// @id strongfather_decision_justification_explanation
    /// @role domain_model
    /// @layer core
    /// @do explain_decision
    //@human / @human Explication conceptuelle de la décision.
    pub explanation: String,
    /// @id strongfather_decision_justification_policy_references
    /// @role domain_model
    /// @layer core
    /// @do reference_policies
    //@human / @human Références aux politiques appliquées (G-JUST-2 : justification référence les politiques).
    pub policy_references: Vec<PolicyId>,
    /// @id strongfather_decision_justification_reasoning_steps
    /// @role domain_model
    /// @layer core
    /// @do store_reasoning_steps
    //@human / @human Étapes de raisonnement (optionnelles pour traçabilité).
    pub reasoning_steps: Vec<String>,
}

impl Justification {
    /// @id strongfather_decision_justification_new
    /// @role constructor
    /// @layer core
    /// @do create_justification
    //@human / @human Crée une nouvelle justification avec explication et références aux politiques.
    /// @depends strongfather_decision_justification
    #[must_use]
    pub fn new(explanation: String, policy_references: Vec<PolicyId>) -> Self {
        Self {
            explanation,
            policy_references,
            reasoning_steps: Vec::new(),
        }
    }

    /// @id strongfather_decision_justification_with_reasoning
    /// @role builder
    /// @layer core
    /// @do add_reasoning_steps
    //@human / @human Ajoute des étapes de raisonnement à la justification.
    /// @depends strongfather_decision_justification_new
    #[must_use]
    pub fn with_reasoning_steps(mut self, steps: Vec<String>) -> Self {
        self.reasoning_steps = steps;
        self
    }
}

/// @id strongfather_decision_metadata
/// @role domain_model
/// @layer core
/// @do represent_decision_metadata
//@human / @human Métadonnées de traçabilité d'une décision. Informations pour le suivi et l'audit.
/// Métadonnées d'une décision
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionMetadata {
    /// @id strongfather_decision_metadata_fields
    /// @role domain_model
    /// @layer core
    /// @do store_metadata_fields
    //@human / @human Champs de métadonnées (informatives, non-évaluées).
    pub fields: HashMap<String, String>,
}

impl DecisionMetadata {
    /// @id strongfather_decision_metadata_new
    /// @role constructor
    /// @layer core
    /// @do create_metadata
    //@human / @human Crée de nouvelles métadonnées vides.
    /// @depends strongfather_decision_metadata
    #[must_use]
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    /// @id strongfather_decision_metadata_with_field
    /// @role builder
    /// @layer core
    /// @do add_metadata_field
    //@human / @human Ajoute un champ de métadonnée.
    /// @depends strongfather_decision_metadata_new
    #[must_use]
    pub fn with_field(mut self, key: String, value: String) -> Self {
        self.fields.insert(key, value);
        self
    }
}

impl Default for DecisionMetadata {
    /// @id strongfather_decision_metadata_default
    /// @role constructor
    /// @layer core
    /// @do create_default_metadata
    //@human / @human Crée des métadonnées vides par défaut.
    fn default() -> Self {
        Self::new()
    }
}

/// @id strongfather_decision_evaluation_context
/// @role domain_model
/// @layer core
/// @do represent_evaluation_context
//@human / @human Contexte d'évaluation utilisé pour produire la décision.
/// Contexte d'évaluation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluationContext {
    /// @id strongfather_decision_eval_context_fields
    /// @role domain_model
    /// @layer core
    /// @do store_context_fields
    //@human / @human Champs du contexte d'évaluation (conceptuels, pas techniques).
    pub fields: HashMap<String, String>,
}

impl EvaluationContext {
    /// @id strongfather_decision_eval_context_new
    /// @role constructor
    /// @layer core
    /// @do create_evaluation_context
    //@human / @human Crée un nouveau contexte d'évaluation vide.
    /// @depends strongfather_decision_evaluation_context
    #[must_use]
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    /// @id strongfather_decision_eval_context_with_field
    /// @role builder
    /// @layer core
    /// @do add_context_field
    //@human / @human Ajoute un champ au contexte d'évaluation.
    /// @depends strongfather_decision_eval_context_new
    #[must_use]
    pub fn with_field(mut self, key: String, value: String) -> Self {
        self.fields.insert(key, value);
        self
    }
}

impl Default for EvaluationContext {
    /// @id strongfather_decision_eval_context_default
    /// @role constructor
    /// @layer core
    /// @do create_default_evaluation_context
    //@human / @human Crée un contexte d'évaluation vide par défaut.
    fn default() -> Self {
        Self::new()
    }
}

/// @id strongfather_decision
/// @role domain_model
/// @layer core
/// @do represent_decision
//@human / @human Décision StrongFather. Résultat conceptuel produit après évaluation d'une intention.
/// Décision StrongFather
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision {
    /// @id strongfather_decision_intent_id
    /// @role domain_model
    /// @layer core
    /// @do reference_intent
    //@human / @human Identifiant de l'intention évaluée (R-ID-3 : présent dans toute décision).
    pub intent_id: String,
    /// @id strongfather_decision_type_decision_type_2
    /// @role domain_model
    /// @layer core
    /// @do specify_decision_type
    //@human / @human Type de décision (ACCEPTÉE, REFUSÉE, AMBIGUË, DIFFÉRÉE).
    pub decision_type: DecisionType,
    /// @id strongfather_decision_justification_justification_2
    /// @role domain_model
    /// @layer core
    /// @do provide_justification
    //@human / @human Justification de la décision (G-JUST-1 : justification obligatoire).
    pub justification: Justification,
    /// @id strongfather_decision_policies_applied
    /// @role domain_model
    /// @layer core
    /// @do list_applied_policies
    //@human / @human Liste des politiques appliquées (INV-TRACE-3 : traçabilité).
    pub policies_applied: Vec<PolicyId>,
    /// @id strongfather_decision_evaluation_context_evaluation_context_2
    /// @role domain_model
    /// @layer core
    /// @do store_evaluation_context
    //@human / @human Contexte d'évaluation utilisé.
    pub evaluation_context: EvaluationContext,
    /// @id strongfather_decision_metadata_metadata_2
    /// @role domain_model
    /// @layer core
    /// @do store_decision_metadata
    //@human / @human Métadonnées de traçabilité.
    pub metadata: DecisionMetadata,
}

impl Decision {
    /// @id strongfather_decision_new
    /// @role constructor
    /// @layer core
    /// @do create_decision
    //@human / @human Crée une nouvelle décision avec les composants obligatoires.
    /// @depends strongfather_decision
    #[must_use]
    pub fn new(
        intent_id: String,
        decision_type: DecisionType,
        justification: Justification,
        policies_applied: Vec<PolicyId>,
    ) -> Self {
        Self {
            intent_id,
            decision_type,
            justification,
            policies_applied,
            evaluation_context: EvaluationContext::default(),
            metadata: DecisionMetadata::default(),
        }
    }

    /// @id strongfather_decision_with_evaluation_context
    /// @role builder
    /// @layer core
    /// @do set_evaluation_context
    //@human / @human Définit le contexte d'évaluation de la décision.
    /// @depends strongfather_decision_new
    #[must_use]
    pub fn with_evaluation_context(mut self, context: EvaluationContext) -> Self {
        self.evaluation_context = context;
        self
    }

    /// @id strongfather_decision_with_metadata
    /// @role builder
    /// @layer core
    /// @do set_decision_metadata
    //@human / @human Définit les métadonnées de la décision.
    /// @depends strongfather_decision_new
    #[must_use]
    pub fn with_metadata(mut self, metadata: DecisionMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// @id strongfather_decision_is_accepted
    /// @role query
    /// @layer core
    /// @do check_if_accepted
    //@human / @human Vérifie si la décision est acceptée.
    /// @depends strongfather_decision
    #[must_use]
    pub fn is_accepted(&self) -> bool {
        matches!(self.decision_type, DecisionType::Accepted { .. })
    }

    /// @id strongfather_decision_is_refused
    /// @role query
    /// @layer core
    /// @do check_if_refused
    //@human / @human Vérifie si la décision est refusée.
    /// @depends strongfather_decision
    #[must_use]
    pub fn is_refused(&self) -> bool {
        matches!(self.decision_type, DecisionType::Refused { .. })
    }

    /// @id strongfather_decision_is_ambiguous
    /// @role query
    /// @layer core
    /// @do check_if_ambiguous
    //@human / @human Vérifie si la décision est ambiguë.
    /// @depends strongfather_decision
    #[must_use]
    pub fn is_ambiguous(&self) -> bool {
        matches!(self.decision_type, DecisionType::Ambiguous { .. })
    }

    /// @id strongfather_decision_is_deferred
    /// @role query
    /// @layer core
    /// @do check_if_deferred
    //@human / @human Vérifie si la décision est différée.
    /// @depends strongfather_decision
    #[must_use]
    pub fn is_deferred(&self) -> bool {
        matches!(self.decision_type, DecisionType::Deferred { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id strongfather_decision_test_create_accepted
    /// @role test
    /// @layer core
    /// @do test_accepted_decision_creation
    //@human / @human Test de création d'une décision acceptée.
    #[test]
    fn test_create_accepted_decision() {
        let justification = Justification::new(
            "Intention valide selon toutes les politiques".to_string(),
            vec!["POL-001".to_string(), "POL-002".to_string()],
        );

        let decision = Decision::new(
            "intent_001".to_string(),
            DecisionType::Accepted { priority: 10 },
            justification,
            vec!["POL-001".to_string(), "POL-002".to_string()],
        );

        assert_eq!(decision.intent_id, "intent_001");
        assert!(decision.is_accepted());
        assert!(!decision.is_refused());
        assert!(!decision.is_ambiguous());
        assert!(!decision.is_deferred());
    }

    /// @id strongfather_decision_test_create_refused
    /// @role test
    /// @layer core
    /// @do test_refused_decision_creation
    //@human / @human Test de création d'une décision refusée.
    #[test]
    fn test_create_refused_decision() {
        let justification = Justification::new(
            "Intention invalide : politique POL-003 violée".to_string(),
            vec!["POL-003".to_string()],
        );

        let decision = Decision::new(
            "intent_002".to_string(),
            DecisionType::Refused {
                reason: "Politique de contrainte violée".to_string(),
                violated_policies: vec!["POL-003".to_string()],
            },
            justification,
            vec!["POL-003".to_string()],
        );

        assert_eq!(decision.intent_id, "intent_002");
        assert!(decision.is_refused());
        assert!(!decision.is_accepted());
    }

    /// @id strongfather_decision_test_create_ambiguous
    /// @role test
    /// @layer core
    /// @do test_ambiguous_decision_creation
    //@human / @human Test de création d'une décision ambiguë.
    #[test]
    fn test_create_ambiguous_decision() {
        let justification =
            Justification::new("Intention insuffisamment définie".to_string(), vec![]);

        let decision = Decision::new(
            "intent_003".to_string(),
            DecisionType::Ambiguous {
                missing_information: vec!["subject".to_string(), "data".to_string()],
                clarifications_required: vec!["Préciser le sujet".to_string()],
            },
            justification,
            vec![],
        );

        assert_eq!(decision.intent_id, "intent_003");
        assert!(decision.is_ambiguous());
    }

    /// @id strongfather_decision_test_create_deferred
    /// @role test
    /// @layer core
    /// @do test_deferred_decision_creation
    //@human / @human Test de création d'une décision différée.
    #[test]
    fn test_create_deferred_decision() {
        let justification =
            Justification::new("Contexte futur requis pour évaluation".to_string(), vec![]);

        let decision = Decision::new(
            "intent_004".to_string(),
            DecisionType::Deferred {
                reason: "Contexte utilisateur non disponible".to_string(),
                context_required: vec!["user_context".to_string()],
            },
            justification,
            vec![],
        );

        assert_eq!(decision.intent_id, "intent_004");
        assert!(decision.is_deferred());
    }

    /// @id strongfather_decision_test_justification
    /// @role test
    /// @layer core
    /// @do test_justification_with_reasoning
    //@human / @human Test de création d'une justification avec étapes de raisonnement.
    #[test]
    fn test_justification_with_reasoning() {
        let justification = Justification::new(
            "Évaluation complète".to_string(),
            vec!["POL-001".to_string()],
        )
        .with_reasoning_steps(vec![
            "Étape 1: Validation structurelle".to_string(),
            "Étape 2: Application des politiques".to_string(),
        ]);

        assert_eq!(justification.reasoning_steps.len(), 2);
    }
}
