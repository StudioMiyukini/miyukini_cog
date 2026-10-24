//! # StrongFather — Modèle d'Intention
//!
//! Ce module définit le modèle conceptuel des intentions soumises à StrongFather pour évaluation.
//!
//! ## Références
//!
//! - [Intent Model Contract](../../../docs/core/StrongFather/contracts/intent/StrongFather%20-%20Intent%20Model%20Contract.md)
//! - [Documentation Fondatrice](../../../docs/core/StrongFather/foundation/StrongFather%20-%20Documentation%20Fondatrice.md)

use std::collections::HashMap;

use sha2::{Digest, Sha256};

/// @id strongfather_intent_type
/// @role domain_model
/// @layer core
/// @do define_action_type
//@human / @human Type d'action d'une intention. Définit la catégorie conceptuelle de l'action à évaluer.
/// Type d'action d'une intention
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionType {
    /// @id strongfather_intent_action_creation
    /// @role domain_model
    /// @layer core
    /// @do represent_creation_action
    //@human / @human Intention de créer une nouvelle entité ou un nouveau fait.
    Creation,
    /// @id strongfather_intent_action_modification
    /// @role domain_model
    /// @layer core
    /// @do represent_modification_action
    //@human / @human Intention de modifier une entité ou un fait existant.
    Modification,
    /// @id strongfather_intent_action_deletion
    /// @role domain_model
    /// @layer core
    /// @do represent_deletion_action
    //@human / @human Intention de supprimer une entité ou un fait existant.
    Deletion,
    /// @id strongfather_intent_action_read
    /// @role domain_model
    /// @layer core
    /// @do represent_read_action
    //@human / @human Intention de lire une entité ou un fait (évaluation d'accès).
    Read,
    /// @id strongfather_intent_action_evaluation
    /// @role domain_model
    /// @layer core
    /// @do represent_evaluation_action
    //@human / @human Intention d'évaluer une condition ou un état sans action.
    Evaluation,
}

impl ActionType {
    /// @id strongfather_intent_action_as_str
    /// @role domain_model
    /// @layer core
    /// @do expose_canonical_action_name
    //@human / @human Nom canonique de l'action (utilisé par les règles et l'empreinte). Stable : ne pas renommer.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            ActionType::Creation => "creation",
            ActionType::Modification => "modification",
            ActionType::Deletion => "deletion",
            ActionType::Read => "read",
            ActionType::Evaluation => "evaluation",
        }
    }
}

impl std::str::FromStr for ActionType {
    type Err = String;

    /// Analyse stricte (correspondance exacte, sensible à la casse) du nom canonique.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "creation" => Ok(ActionType::Creation),
            "modification" => Ok(ActionType::Modification),
            "deletion" => Ok(ActionType::Deletion),
            "read" => Ok(ActionType::Read),
            "evaluation" => Ok(ActionType::Evaluation),
            autre => Err(format!("type d'action inconnu : '{autre}'")),
        }
    }
}

/// @id strongfather_intent_call_context
/// @role domain_model
/// @layer core
/// @do represent_call_context
//@human / @human Contexte d'appel d'une intention. Décrit qui soumet l'intention et dans quel cadre.
/// Contexte d'appel d'une intention
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallContext {
    /// @id strongfather_intent_context_caller_identity
    /// @role domain_model
    /// @layer core
    /// @do identify_caller
    //@human / @human Identifiant de l'appelant qui soumet l'intention.
    pub caller_identity: String,
    /// @id strongfather_intent_context_origin
    /// @role domain_model
    /// @layer core
    /// @do identify_origin
    //@human / @human Origine de l'appel (produit, adaptateur).
    pub origin: String,
    /// @id strongfather_intent_context_instance
    /// @role domain_model
    /// @layer core
    /// @do identify_instance
    //@human / @human Instance concernée par l'intention.
    pub instance: String,
}

/// @id strongfather_intent_constraint
/// @role domain_model
/// @layer core
/// @do represent_constraint
//@human / @human Contrainte explicite fournie par l'appelant. Condition supplémentaire à respecter.
/// Contrainte explicite d'une intention
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constraint {
    /// @id strongfather_intent_constraint_id
    /// @role domain_model
    /// @layer core
    /// @do identify_constraint
    //@human / @human Identifiant unique de la contrainte.
    pub id: String,
    /// @id strongfather_intent_constraint_description
    /// @role domain_model
    /// @layer core
    /// @do describe_constraint
    //@human / @human Description déclarative de la contrainte.
    pub description: String,
}

/// @id strongfather_intent_data
/// @role domain_model
/// @layer core
/// @do represent_intent_data
//@human / @human Données descriptives associées à l'intention. Informations non-exécutables.
/// Données d'une intention
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntentData {
    /// @id strongfather_intent_data_content
    /// @role domain_model
    /// @layer core
    /// @do store_data_content
    //@human / @human Contenu descriptif des données (représentation conceptuelle, pas technique).
    pub content: HashMap<String, String>,
}

/// @id strongfather_intent_state
/// @role domain_model
/// @layer core
/// @do represent_intent_lifecycle_state
//@human / @human État du cycle de vie d'une intention dans StrongFather.
/// État du cycle de vie d'une intention
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntentState {
    /// @id strongfather_intent_state_submitted
    /// @role domain_model
    /// @layer core
    /// @do mark_intent_submitted
    //@human / @human L'intention a été soumise à StrongFather pour évaluation.
    Submitted,
    /// @id strongfather_intent_state_in_evaluation
    /// @role domain_model
    /// @layer core
    /// @do mark_intent_in_evaluation
    //@human / @human L'intention est en cours d'évaluation selon les politiques.
    InEvaluation,
    /// @id strongfather_intent_state_decided
    /// @role domain_model
    /// @layer core
    /// @do mark_intent_decided
    //@human / @human Une décision a été produite pour l'intention.
    Decided,
}

/// @id strongfather_intent
/// @role domain_model
/// @layer core
/// @do represent_intent
//@human / @human Intention soumise à StrongFather pour évaluation. Représente une demande conceptuelle d'évaluation.
/// Intention StrongFather
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Intent {
    /// @id strongfather_intent_id
    /// @role domain_model
    /// @layer core
    /// @do identify_intent
    //@human / @human Identifiant unique de l'intention (INV-ID-GLOBAL : unicité globale).
    pub intent_id: String,
    /// @id strongfather_intent_action_type
    /// @role domain_model
    /// @layer core
    /// @do specify_action_type
    //@human / @human Type d'action de l'intention (R-TYPE-1 : exactement un type).
    pub action_type: ActionType,
    /// @id strongfather_intent_subject
    /// @role domain_model
    /// @layer core
    /// @do specify_subject
    //@human / @human Sujet de l'intention - entité, fait ou concept sur lequel porte l'intention (R-SUBJ-1).
    pub subject: String,
    /// @id strongfather_intent_call_context_call_context_2
    /// @role domain_model
    /// @layer core
    /// @do provide_call_context
    //@human / @human Contexte d'appel obligatoire (R-CTX-1 : contexte complet).
    pub call_context: CallContext,
    /// @id strongfather_intent_data_data_2
    /// @role domain_model
    /// @layer core
    /// @do provide_intent_data
    //@human / @human Données associées à l'intention (R-DATA-1 : données obligatoires, peuvent être vides).
    pub data: IntentData,
    /// @id strongfather_intent_requested_priority
    /// @role domain_model
    /// @layer core
    /// @do request_priority
    //@human / @human Priorité demandée par l'appelant (R-PRIO-1 : optionnelle, indicative).
    pub requested_priority: Option<u32>,
    /// @id strongfather_intent_constraints
    /// @role domain_model
    /// @layer core
    /// @do provide_constraints
    //@human / @human Contraintes explicites fournies par l'appelant (R-CONSTR-1 : optionnelles).
    pub constraints: Vec<Constraint>,
    /// @id strongfather_intent_metadata
    /// @role domain_model
    /// @layer core
    /// @do provide_metadata
    //@human / @human Métadonnées de traçabilité (R-META-1 : optionnelles, informatives).
    pub metadata: HashMap<String, String>,
}

impl Intent {
    /// @id strongfather_intent_new
    /// @role constructor
    /// @layer core
    /// @do create_intent
    //@human / @human Crée une nouvelle intention avec les composants obligatoires.
    /// @depends strongfather_intent
    #[must_use]
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

    /// @id strongfather_intent_with_priority
    /// @role builder
    /// @layer core
    /// @do add_requested_priority
    //@human / @human Ajoute une priorité demandée à l'intention.
    /// @depends strongfather_intent_new
    #[must_use]
    pub fn with_priority(mut self, priority: u32) -> Self {
        self.requested_priority = Some(priority);
        self
    }

    /// @id strongfather_intent_with_constraints
    /// @role builder
    /// @layer core
    /// @do add_constraints
    //@human / @human Ajoute des contraintes explicites à l'intention.
    /// @depends strongfather_intent_new
    #[must_use]
    pub fn with_constraints(mut self, constraints: Vec<Constraint>) -> Self {
        self.constraints = constraints;
        self
    }

    /// @id strongfather_intent_with_metadata
    /// @role builder
    /// @layer core
    /// @do add_metadata
    //@human / @human Ajoute des métadonnées de traçabilité à l'intention.
    /// @depends strongfather_intent_new
    #[must_use]
    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = metadata;
        self
    }

    /// @id strongfather_intent_digest
    /// @role domain_model
    /// @layer core
    /// @do compute_intent_digest
    //@human / @human Empreinte stable (SHA-256 hex) d'un encodage canonique de l'intention.
    //@human Les tables sont triées par clé, chaque champ est préfixé par sa longueur : deux
    //@human intentions égales donnent la même empreinte, toute modification la change.
    /// Empreinte SHA-256 (hexadécimal minuscule) de l'encodage canonique de l'intention.
    ///
    /// Format `SF-INTENT-v1` : chaque champ est encodé `longueur (u64 BE) ‖ octets`,
    /// les tables (`data`, `metadata`) sont triées par clé, les contraintes gardent leur ordre.
    #[must_use]
    pub fn digest(&self) -> String {
        fn champ(h: &mut Sha256, octets: &[u8]) {
            h.update((octets.len() as u64).to_be_bytes());
            h.update(octets);
        }
        fn table(h: &mut Sha256, t: &HashMap<String, String>) {
            let mut cles: Vec<&String> = t.keys().collect();
            cles.sort();
            h.update((cles.len() as u64).to_be_bytes());
            for k in cles {
                champ(h, k.as_bytes());
                champ(h, t[k].as_bytes());
            }
        }

        let mut h = Sha256::new();
        champ(&mut h, b"SF-INTENT-v1");
        champ(&mut h, self.intent_id.as_bytes());
        champ(&mut h, self.action_type.as_str().as_bytes());
        champ(&mut h, self.subject.as_bytes());
        champ(&mut h, self.call_context.caller_identity.as_bytes());
        champ(&mut h, self.call_context.origin.as_bytes());
        champ(&mut h, self.call_context.instance.as_bytes());
        table(&mut h, &self.data.content);
        match self.requested_priority {
            None => h.update([0u8]),
            Some(p) => {
                h.update([1u8]);
                h.update(p.to_be_bytes());
            }
        }
        h.update((self.constraints.len() as u64).to_be_bytes());
        for c in &self.constraints {
            champ(&mut h, c.id.as_bytes());
            champ(&mut h, c.description.as_bytes());
        }
        table(&mut h, &self.metadata);

        let octets = h.finalize();
        let mut hex = String::with_capacity(octets.len() * 2);
        for b in octets {
            use std::fmt::Write as _;
            let _ = write!(hex, "{b:02x}");
        }
        hex
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id strongfather_intent_test_create_basic
    /// @role test
    /// @layer core
    /// @do test_basic_intent_creation
    //@human / @human Test de création d'une intention basique avec les composants obligatoires.
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
        assert_eq!(intent.constraints, Vec::<Constraint>::new());
        assert!(intent.metadata.is_empty());
    }

    /// @id strongfather_intent_test_create_with_priority
    /// @role test
    /// @layer core
    /// @do test_intent_with_priority
    //@human / @human Test de création d'une intention avec priorité demandée.
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

    /// @id strongfather_intent_test_create_with_constraints
    /// @role test
    /// @layer core
    /// @do test_intent_with_constraints
    //@human / @human Test de création d'une intention avec contraintes explicites.
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

    /// @id strongfather_intent_test_all_action_types
    /// @role test
    /// @layer core
    /// @do test_all_action_types
    //@human / @human Test que tous les types d'action sont bien définis.
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

        let types = [
            ActionType::Creation,
            ActionType::Modification,
            ActionType::Deletion,
            ActionType::Read,
            ActionType::Evaluation,
        ];

        for (idx, action_type) in types.iter().enumerate() {
            let intent = Intent::new(
                format!("intent_{idx}"),
                *action_type,
                "subject".to_string(),
                context.clone(),
                data.clone(),
            );

            assert_eq!(intent.action_type, *action_type);
        }
    }

    fn intention_digest() -> Intent {
        let mut data = HashMap::new();
        data.insert("b".to_string(), "2".to_string());
        data.insert("a".to_string(), "1".to_string());
        Intent::new(
            "i-1".to_string(),
            ActionType::Read,
            "jaykonta".to_string(),
            CallContext {
                caller_identity: "op-1".to_string(),
                origin: "maia".to_string(),
                instance: "cog-1".to_string(),
            },
            IntentData { content: data },
        )
    }

    /// @id strongfather_intent_test_digest_stable
    /// @role test
    /// @layer core
    /// @do test_digest_stability
    //@human / @human L'empreinte est stable (indépendante de l'ordre d'insertion) et change si l'intention change.
    #[test]
    fn test_digest_stable_et_sensible() {
        let base = intention_digest();
        let mut data = HashMap::new();
        data.insert("a".to_string(), "1".to_string());
        data.insert("b".to_string(), "2".to_string());
        let reordonnee = Intent {
            data: IntentData { content: data },
            ..intention_digest()
        };
        assert_eq!(base.digest(), reordonnee.digest());
        assert_eq!(base.digest().len(), 64);

        let mut autre_sujet = intention_digest();
        autre_sujet.subject = "jaykonta-evil".to_string();
        assert_ne!(base.digest(), autre_sujet.digest());
        let mut autre_data = intention_digest();
        autre_data
            .data
            .content
            .insert("a".to_string(), "10".to_string());
        assert_ne!(base.digest(), autre_data.digest());
        // Pas de confusion par concaténation de champs (préfixes de longueur).
        let mut concat = intention_digest();
        concat.subject = "jaykontaop-1".to_string();
        concat.call_context.caller_identity = String::new();
        assert_ne!(base.digest(), concat.digest());
        assert_ne!(base.digest(), intention_digest().with_priority(0).digest());
    }

    /// @id strongfather_intent_test_action_from_str
    /// @role test
    /// @layer core
    /// @do test_action_parsing
    //@human / @human Le nom canonique d'action fait l'aller-retour, sans correspondance approximative.
    #[test]
    fn test_action_from_str_exact() {
        for a in [
            ActionType::Creation,
            ActionType::Modification,
            ActionType::Deletion,
            ActionType::Read,
            ActionType::Evaluation,
        ] {
            assert_eq!(a.as_str().parse::<ActionType>(), Ok(a));
        }
        assert!("Read".parse::<ActionType>().is_err());
        assert!("reading".parse::<ActionType>().is_err());
    }
}
