//! # StrongFather — Modèle d'Intention
//!
//! Ce module définit le modèle conceptuel des intentions soumises à StrongFather pour évaluation.
//!
//! ## Références
//!
//! - [Intent Model Contract](../../../docs/core/StrongFather/contracts/intent/StrongFather%20-%20Intent%20Model%20Contract.md)
//! - [Documentation Fondatrice](../../../docs/core/StrongFather/foundation/StrongFather%20-%20Documentation%20Fondatrice.md)

use std::collections::HashMap;

/// @id: strongfather_intent_type
/// @role: domain_model
/// @layer: core
/// @human: Type d'action d'une intention. Définit la catégorie conceptuelle de l'action à évaluer.
/// @do: define_action_type
/// Type d'action d'une intention
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionType {
    /// @id: strongfather_intent_action_creation
    /// @role: domain_model
    /// @layer: core
    /// @human: Intention de créer une nouvelle entité ou un nouveau fait.
    /// @do: represent_creation_action
    Creation,
    /// @id: strongfather_intent_action_modification
    /// @role: domain_model
    /// @layer: core
    /// @human: Intention de modifier une entité ou un fait existant.
    /// @do: represent_modification_action
    Modification,
    /// @id: strongfather_intent_action_deletion
    /// @role: domain_model
    /// @layer: core
    /// @human: Intention de supprimer une entité ou un fait existant.
    /// @do: represent_deletion_action
    Deletion,
    /// @id: strongfather_intent_action_read
    /// @role: domain_model
    /// @layer: core
    /// @human: Intention de lire une entité ou un fait (évaluation d'accès).
    /// @do: represent_read_action
    Read,
    /// @id: strongfather_intent_action_evaluation
    /// @role: domain_model
    /// @layer: core
    /// @human: Intention d'évaluer une condition ou un état sans action.
    /// @do: represent_evaluation_action
    Evaluation,
}

/// @id: strongfather_intent_call_context
/// @role: domain_model
/// @layer: core
/// @human: Contexte d'appel d'une intention. Décrit qui soumet l'intention et dans quel cadre.
/// @do: represent_call_context
/// Contexte d'appel d'une intention
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallContext {
    /// @id: strongfather_intent_context_caller_identity
    /// @role: domain_model
    /// @layer: core
    /// @human: Identifiant de l'appelant qui soumet l'intention.
    /// @do: identify_caller
    pub caller_identity: String,
    /// @id: strongfather_intent_context_origin
    /// @role: domain_model
    /// @layer: core
    /// @human: Origine de l'appel (produit, adaptateur).
    /// @do: identify_origin
    pub origin: String,
    /// @id: strongfather_intent_context_instance
    /// @role: domain_model
    /// @layer: core
    /// @human: Instance concernée par l'intention.
    /// @do: identify_instance
    pub instance: String,
}

/// @id: strongfather_intent_constraint
/// @role: domain_model
/// @layer: core
/// @human: Contrainte explicite fournie par l'appelant. Condition supplémentaire à respecter.
/// @do: represent_constraint
/// Contrainte explicite d'une intention
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constraint {
    /// @id: strongfather_intent_constraint_id
    /// @role: domain_model
    /// @layer: core
    /// @human: Identifiant unique de la contrainte.
    /// @do: identify_constraint
    pub id: String,
    /// @id: strongfather_intent_constraint_description
    /// @role: domain_model
    /// @layer: core
    /// @human: Description déclarative de la contrainte.
    /// @do: describe_constraint
    pub description: String,
}

/// @id: strongfather_intent_data
/// @role: domain_model
/// @layer: core
/// @human: Données descriptives associées à l'intention. Informations non-exécutables.
/// @do: represent_intent_data
/// Données d'une intention
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntentData {
    /// @id: strongfather_intent_data_content
    /// @role: domain_model
    /// @layer: core
    /// @human: Contenu descriptif des données (représentation conceptuelle, pas technique).
    /// @do: store_data_content
    pub content: HashMap<String, String>,
}

/// @id: strongfather_intent_state
/// @role: domain_model
/// @layer: core
/// @human: État du cycle de vie d'une intention dans StrongFather.
/// @do: represent_intent_lifecycle_state
/// État du cycle de vie d'une intention
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntentState {
    /// @id: strongfather_intent_state_submitted
    /// @role: domain_model
    /// @layer: core
    /// @human: L'intention a été soumise à StrongFather pour évaluation.
    /// @do: mark_intent_submitted
    Submitted,
    /// @id: strongfather_intent_state_in_evaluation
    /// @role: domain_model
    /// @layer: core
    /// @human: L'intention est en cours d'évaluation selon les politiques.
    /// @do: mark_intent_in_evaluation
    InEvaluation,
    /// @id: strongfather_intent_state_decided
    /// @role: domain_model
    /// @layer: core
    /// @human: Une décision a été produite pour l'intention.
    /// @do: mark_intent_decided
    Decided,
}

/// @id: strongfather_intent
/// @role: domain_model
/// @layer: core
/// @human: Intention soumise à StrongFather pour évaluation. Représente une demande conceptuelle d'évaluation.
/// @do: represent_intent
/// Intention StrongFather
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Intent {
    /// @id: strongfather_intent_id
    /// @role: domain_model
    /// @layer: core
    /// @human: Identifiant unique de l'intention (INV-ID-GLOBAL : unicité globale).
    /// @do: identify_intent
    pub intent_id: String,
    /// @id: strongfather_intent_action_type
    /// @role: domain_model
    /// @layer: core
    /// @human: Type d'action de l'intention (R-TYPE-1 : exactement un type).
    /// @do: specify_action_type
    pub action_type: ActionType,
    /// @id: strongfather_intent_subject
    /// @role: domain_model
    /// @layer: core
    /// @human: Sujet de l'intention - entité, fait ou concept sur lequel porte l'intention (R-SUBJ-1).
    /// @do: specify_subject
    pub subject: String,
    /// @id: strongfather_intent_call_context
    /// @role: domain_model
    /// @layer: core
    /// @human: Contexte d'appel obligatoire (R-CTX-1 : contexte complet).
    /// @do: provide_call_context
    pub call_context: CallContext,
    /// @id: strongfather_intent_data
    /// @role: domain_model
    /// @layer: core
    /// @human: Données associées à l'intention (R-DATA-1 : données obligatoires, peuvent être vides).
    /// @do: provide_intent_data
    pub data: IntentData,
    /// @id: strongfather_intent_requested_priority
    /// @role: domain_model
    /// @layer: core
    /// @human: Priorité demandée par l'appelant (R-PRIO-1 : optionnelle, indicative).
    /// @do: request_priority
    pub requested_priority: Option<u32>,
    /// @id: strongfather_intent_constraints
    /// @role: domain_model
    /// @layer: core
    /// @human: Contraintes explicites fournies par l'appelant (R-CONSTR-1 : optionnelles).
    /// @do: provide_constraints
    pub constraints: Vec<Constraint>,
    /// @id: strongfather_intent_metadata
    /// @role: domain_model
    /// @layer: core
    /// @human: Métadonnées de traçabilité (R-META-1 : optionnelles, informatives).
    /// @do: provide_metadata
    pub metadata: HashMap<String, String>,
}

impl Intent {
    /// @id: strongfather_intent_new
    /// @role: constructor
    /// @layer: core
    /// @human: Crée une nouvelle intention avec les composants obligatoires.
    /// @do: create_intent
    /// @depends: strongfather_intent
    pub fn new(
        intent_id: String,
        action_type: ActionType,
        subject: String,
        call_context: CallContext,
        data: IntentData,
    ) -> Self {
        Self {
            intent_id,
            action_type,
            subject,
            call_context,
            data,
            requested_priority: None,
            constraints: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// @id: strongfather_intent_with_priority
    /// @role: builder
    /// @layer: core
    /// @human: Ajoute une priorité demandée à l'intention.
    /// @do: add_requested_priority
    /// @depends: strongfather_intent_new
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.requested_priority = Some(priority);
        self
    }

    /// @id: strongfather_intent_with_constraints
    /// @role: builder
    /// @layer: core
    /// @human: Ajoute des contraintes explicites à l'intention.
    /// @do: add_constraints
    /// @depends: strongfather_intent_new
    pub fn with_constraints(mut self, constraints: Vec<Constraint>) -> Self {
        self.constraints = constraints;
        self
    }

    /// @id: strongfather_intent_with_metadata
    /// @role: builder
    /// @layer: core
    /// @human: Ajoute des métadonnées de traçabilité à l'intention.
    /// @do: add_metadata
    /// @depends: strongfather_intent_new
    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = metadata;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: strongfather_intent_test_create_basic
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'une intention basique avec les composants obligatoires.
    /// @do: test_basic_intent_creation
    #[test]
    fn test_create_basic_intent() {
        let context = CallContext {
            caller_identity: "user123".to_string(),
            origin: "product_a".to_string(),
            instance: "instance_1".to_string(),
        };

        let data = IntentData {
            content: HashMap::new(),
        };

        let intent = Intent::new(
            "intent_001".to_string(),
            ActionType::Creation,
            "entity_123".to_string(),
            context,
            data,
        );

        assert_eq!(intent.intent_id, "intent_001");
        assert_eq!(intent.action_type, ActionType::Creation);
        assert_eq!(intent.subject, "entity_123");
        assert!(intent.requested_priority.is_none());
        assert!(intent.constraints.is_empty());
        assert!(intent.metadata.is_empty());
    }

    /// @id: strongfather_intent_test_create_with_priority
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'une intention avec priorité demandée.
    /// @do: test_intent_with_priority
    #[test]
    fn test_create_intent_with_priority() {
        let context = CallContext {
            caller_identity: "user123".to_string(),
            origin: "product_a".to_string(),
            instance: "instance_1".to_string(),
        };

        let data = IntentData {
            content: HashMap::new(),
        };

        let intent = Intent::new(
            "intent_002".to_string(),
            ActionType::Modification,
            "entity_456".to_string(),
            context,
            data,
        )
        .with_priority(10);

        assert_eq!(intent.requested_priority, Some(10));
    }

    /// @id: strongfather_intent_test_create_with_constraints
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'une intention avec contraintes explicites.
    /// @do: test_intent_with_constraints
    #[test]
    fn test_create_intent_with_constraints() {
        let context = CallContext {
            caller_identity: "user123".to_string(),
            origin: "product_a".to_string(),
            instance: "instance_1".to_string(),
        };

        let data = IntentData {
            content: HashMap::new(),
        };

        let constraints = vec![Constraint {
            id: "constraint_1".to_string(),
            description: "Must satisfy condition X".to_string(),
        }];

        let intent = Intent::new(
            "intent_003".to_string(),
            ActionType::Deletion,
            "entity_789".to_string(),
            context,
            data,
        )
        .with_constraints(constraints.clone());

        assert_eq!(intent.constraints.len(), 1);
        assert_eq!(intent.constraints[0].id, "constraint_1");
    }

    /// @id: strongfather_intent_test_all_action_types
    /// @role: test
    /// @layer: core
    /// @human: Test que tous les types d'action sont bien définis.
    /// @do: test_all_action_types
    #[test]
    fn test_all_action_types() {
        let context = CallContext {
            caller_identity: "user123".to_string(),
            origin: "product_a".to_string(),
            instance: "instance_1".to_string(),
        };

        let data = IntentData {
            content: HashMap::new(),
        };

        let types = vec![
            ActionType::Creation,
            ActionType::Modification,
            ActionType::Deletion,
            ActionType::Read,
            ActionType::Evaluation,
        ];

        for (idx, action_type) in types.iter().enumerate() {
            let intent = Intent::new(
                format!("intent_{}", idx),
                *action_type,
                "subject".to_string(),
                context.clone(),
                data.clone(),
            );

            assert_eq!(intent.action_type, *action_type);
        }
    }
}
