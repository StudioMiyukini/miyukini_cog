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

/// @id: strongfather_decision_type
/// @role: domain_model
/// @layer: core
/// @human: Type de décision StrongFather. Définit le résultat de l'évaluation d'une intention.
/// @do: define_decision_type
/// Type de décision
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionType {
    /// @id: strongfather_decision_type_accepted
    /// @role: domain_model
    /// @layer: core
    /// @human: Décision acceptée - l'intention est valide selon les politiques et peut être considérée pour exécution.
    /// @do: represent_accepted_decision
    Accepted {
        /// @id: strongfather_decision_accepted_priority
        /// @role: domain_model
        /// @layer: core
        /// @human: Priorité relative établie pour l'intention.
        /// @do: store_established_priority
        priority: u32,
    },
    /// @id: strongfather_decision_type_refused
    /// @role: domain_model
    /// @layer: core
    /// @human: Décision refusée - l'intention est invalide selon les politiques et ne doit pas être exécutée.
    /// @do: represent_refused_decision
    Refused {
        /// @id: strongfather_decision_refused_reason
        /// @role: domain_model
        /// @layer: core
        /// @human: Raison explicite du refus.
        /// @do: store_refusal_reason
        reason: String,
        /// @id: strongfather_decision_refused_violated_policies
        /// @role: domain_model
        /// @layer: core
        /// @human: Liste des politiques violées qui ont conduit au refus.
        /// @do: store_violated_policies
        violated_policies: Vec<PolicyId>,
    },
    /// @id: strongfather_decision_type_ambiguous
    /// @role: domain_model
    /// @layer: core
    /// @human: Décision ambiguë - l'intention est insuffisamment définie et nécessite des clarifications.
    /// @do: represent_ambiguous_decision
    Ambiguous {
        /// @id: strongfather_decision_ambiguous_missing_information
        /// @role: domain_model
        /// @layer: core
        /// @human: Liste des éléments manquants ou insuffisamment définis.
        /// @do: store_missing_information
        missing_information: Vec<String>,
        /// @id: strongfather_decision_ambiguous_clarifications
        /// @role: domain_model
        /// @layer: core
        /// @human: Clarifications nécessaires pour permettre l'évaluation.
        /// @do: store_required_clarifications
        clarifications_required: Vec<String>,
    },
    /// @id: strongfather_decision_type_deferred
    /// @role: domain_model
    /// @layer: core
    /// @human: Décision différée - l'intention ne peut pas être évaluée immédiatement car elle dépend d'un contexte futur.
    /// @do: represent_deferred_decision
    Deferred {
        /// @id: strongfather_decision_deferred_reason
        /// @role: domain_model
        /// @layer: core
        /// @human: Raison de la différation (contexte futur requis).
        /// @do: store_deferral_reason
        reason: String,
        /// @id: strongfather_decision_deferred_context_required
        /// @role: domain_model
        /// @layer: core
        /// @human: Contexte futur requis pour l'évaluation (INV-DIFF-NOPLAN : pas de planification).
        /// @do: store_required_context
        context_required: Vec<String>,
    },
}

/// @id: strongfather_decision_justification
/// @role: domain_model
/// @layer: core
/// @human: Justification d'une décision. Explication conceptuelle de la décision produite (G-JUST-1 : justification obligatoire).
/// @do: represent_decision_justification
/// Justification d'une décision
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Justification {
    /// @id: strongfather_decision_justification_explanation
    /// @role: domain_model
    /// @layer: core
    /// @human: Explication conceptuelle de la décision.
    /// @do: explain_decision
    pub explanation: String,
    /// @id: strongfather_decision_justification_policy_references
    /// @role: domain_model
    /// @layer: core
    /// @human: Références aux politiques appliquées (G-JUST-2 : justification référence les politiques).
    /// @do: reference_policies
    pub policy_references: Vec<PolicyId>,
    /// @id: strongfather_decision_justification_reasoning_steps
    /// @role: domain_model
    /// @layer: core
    /// @human: Étapes de raisonnement (optionnelles pour traçabilité).
    /// @do: store_reasoning_steps
    pub reasoning_steps: Vec<String>,
}

impl Justification {
    /// @id: strongfather_decision_justification_new
    /// @role: constructor
    /// @layer: core
    /// @human: Crée une nouvelle justification avec explication et références aux politiques.
    /// @do: create_justification
    /// @depends: strongfather_decision_justification
    pub fn new(explanation: String, policy_references: Vec<PolicyId>) -> Self {
        Self {
            explanation,
            policy_references,
            reasoning_steps: Vec::new(),
        }
    }

    /// @id: strongfather_decision_justification_with_reasoning
    /// @role: builder
    /// @layer: core
    /// @human: Ajoute des étapes de raisonnement à la justification.
    /// @do: add_reasoning_steps
    /// @depends: strongfather_decision_justification_new
    pub fn with_reasoning_steps(mut self, steps: Vec<String>) -> Self {
        self.reasoning_steps = steps;
        self
    }
}

/// @id: strongfather_decision_metadata
/// @role: domain_model
/// @layer: core
/// @human: Métadonnées de traçabilité d'une décision. Informations pour le suivi et l'audit.
/// @do: represent_decision_metadata
/// Métadonnées d'une décision
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionMetadata {
    /// @id: strongfather_decision_metadata_fields
    /// @role: domain_model
    /// @layer: core
    /// @human: Champs de métadonnées (informatives, non-évaluées).
    /// @do: store_metadata_fields
    pub fields: HashMap<String, String>,
}

impl DecisionMetadata {
    /// @id: strongfather_decision_metadata_new
    /// @role: constructor
    /// @layer: core
    /// @human: Crée de nouvelles métadonnées vides.
    /// @do: create_metadata
    /// @depends: strongfather_decision_metadata
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    /// @id: strongfather_decision_metadata_with_field
    /// @role: builder
    /// @layer: core
    /// @human: Ajoute un champ de métadonnée.
    /// @do: add_metadata_field
    /// @depends: strongfather_decision_metadata_new
    pub fn with_field(mut self, key: String, value: String) -> Self {
        self.fields.insert(key, value);
        self
    }
}

impl Default for DecisionMetadata {
    /// @id: strongfather_decision_metadata_default
    /// @role: constructor
    /// @layer: core
    /// @human: Crée des métadonnées vides par défaut.
    /// @do: create_default_metadata
    fn default() -> Self {
        Self::new()
    }
}

/// @id: strongfather_decision_evaluation_context
/// @role: domain_model
/// @layer: core
/// @human: Contexte d'évaluation utilisé pour produire la décision.
/// @do: represent_evaluation_context
/// Contexte d'évaluation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluationContext {
    /// @id: strongfather_decision_eval_context_fields
    /// @role: domain_model
    /// @layer: core
    /// @human: Champs du contexte d'évaluation (conceptuels, pas techniques).
    /// @do: store_context_fields
    pub fields: HashMap<String, String>,
}

impl EvaluationContext {
    /// @id: strongfather_decision_eval_context_new
    /// @role: constructor
    /// @layer: core
    /// @human: Crée un nouveau contexte d'évaluation vide.
    /// @do: create_evaluation_context
    /// @depends: strongfather_decision_evaluation_context
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
        }
    }

    /// @id: strongfather_decision_eval_context_with_field
    /// @role: builder
    /// @layer: core
    /// @human: Ajoute un champ au contexte d'évaluation.
    /// @do: add_context_field
    /// @depends: strongfather_decision_eval_context_new
    pub fn with_field(mut self, key: String, value: String) -> Self {
        self.fields.insert(key, value);
        self
    }
}

impl Default for EvaluationContext {
    /// @id: strongfather_decision_eval_context_default
    /// @role: constructor
    /// @layer: core
    /// @human: Crée un contexte d'évaluation vide par défaut.
    /// @do: create_default_evaluation_context
    fn default() -> Self {
        Self::new()
    }
}

/// @id: strongfather_decision
/// @role: domain_model
/// @layer: core
/// @human: Décision StrongFather. Résultat conceptuel produit après évaluation d'une intention.
/// @do: represent_decision
/// Décision StrongFather
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision {
    /// @id: strongfather_decision_intent_id
    /// @role: domain_model
    /// @layer: core
    /// @human: Identifiant de l'intention évaluée (R-ID-3 : présent dans toute décision).
    /// @do: reference_intent
    pub intent_id: String,
    /// @id: strongfather_decision_type
    /// @role: domain_model
    /// @layer: core
    /// @human: Type de décision (ACCEPTÉE, REFUSÉE, AMBIGUË, DIFFÉRÉE).
    /// @do: specify_decision_type
    pub decision_type: DecisionType,
    /// @id: strongfather_decision_justification
    /// @role: domain_model
    /// @layer: core
    /// @human: Justification de la décision (G-JUST-1 : justification obligatoire).
    /// @do: provide_justification
    pub justification: Justification,
    /// @id: strongfather_decision_policies_applied
    /// @role: domain_model
    /// @layer: core
    /// @human: Liste des politiques appliquées (INV-TRACE-3 : traçabilité).
    /// @do: list_applied_policies
    pub policies_applied: Vec<PolicyId>,
    /// @id: strongfather_decision_evaluation_context
    /// @role: domain_model
    /// @layer: core
    /// @human: Contexte d'évaluation utilisé.
    /// @do: store_evaluation_context
    pub evaluation_context: EvaluationContext,
    /// @id: strongfather_decision_metadata
    /// @role: domain_model
    /// @layer: core
    /// @human: Métadonnées de traçabilité.
    /// @do: store_decision_metadata
    pub metadata: DecisionMetadata,
}

impl Decision {
    /// @id: strongfather_decision_new
    /// @role: constructor
    /// @layer: core
    /// @human: Crée une nouvelle décision avec les composants obligatoires.
    /// @do: create_decision
    /// @depends: strongfather_decision
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

    /// @id: strongfather_decision_with_evaluation_context
    /// @role: builder
    /// @layer: core
    /// @human: Définit le contexte d'évaluation de la décision.
    /// @do: set_evaluation_context
    /// @depends: strongfather_decision_new
    pub fn with_evaluation_context(mut self, context: EvaluationContext) -> Self {
        self.evaluation_context = context;
        self
    }

    /// @id: strongfather_decision_with_metadata
    /// @role: builder
    /// @layer: core
    /// @human: Définit les métadonnées de la décision.
    /// @do: set_decision_metadata
    /// @depends: strongfather_decision_new
    pub fn with_metadata(mut self, metadata: DecisionMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    /// @id: strongfather_decision_is_accepted
    /// @role: query
    /// @layer: core
    /// @human: Vérifie si la décision est acceptée.
    /// @do: check_if_accepted
    /// @depends: strongfather_decision
    pub fn is_accepted(&self) -> bool {
        matches!(self.decision_type, DecisionType::Accepted { .. })
    }

    /// @id: strongfather_decision_is_refused
    /// @role: query
    /// @layer: core
    /// @human: Vérifie si la décision est refusée.
    /// @do: check_if_refused
    /// @depends: strongfather_decision
    pub fn is_refused(&self) -> bool {
        matches!(self.decision_type, DecisionType::Refused { .. })
    }

    /// @id: strongfather_decision_is_ambiguous
    /// @role: query
    /// @layer: core
    /// @human: Vérifie si la décision est ambiguë.
    /// @do: check_if_ambiguous
    /// @depends: strongfather_decision
    pub fn is_ambiguous(&self) -> bool {
        matches!(self.decision_type, DecisionType::Ambiguous { .. })
    }

    /// @id: strongfather_decision_is_deferred
    /// @role: query
    /// @layer: core
    /// @human: Vérifie si la décision est différée.
    /// @do: check_if_deferred
    /// @depends: strongfather_decision
    pub fn is_deferred(&self) -> bool {
        matches!(self.decision_type, DecisionType::Deferred { .. })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: strongfather_decision_test_create_accepted
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'une décision acceptée.
    /// @do: test_accepted_decision_creation
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

    /// @id: strongfather_decision_test_create_refused
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'une décision refusée.
    /// @do: test_refused_decision_creation
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

    /// @id: strongfather_decision_test_create_ambiguous
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'une décision ambiguë.
    /// @do: test_ambiguous_decision_creation
    #[test]
    fn test_create_ambiguous_decision() {
        let justification = Justification::new(
            "Intention insuffisamment définie".to_string(),
            vec![],
        );

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

    /// @id: strongfather_decision_test_create_deferred
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'une décision différée.
    /// @do: test_deferred_decision_creation
    #[test]
    fn test_create_deferred_decision() {
        let justification = Justification::new(
            "Contexte futur requis pour évaluation".to_string(),
            vec![],
        );

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

    /// @id: strongfather_decision_test_justification
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'une justification avec étapes de raisonnement.
    /// @do: test_justification_with_reasoning
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
