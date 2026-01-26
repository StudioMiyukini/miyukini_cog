//! Structure Decision pour StrongFather
//!
//! Ce module définit la structure des décisions produites par StrongFather après évaluation
//! d'une intention selon des politiques et des contraintes.
//! Conforme au contrat Core Decision Contract (FONDATION).
//!
//! **Invariants respectés :**
//! - INV-DEC-1 : Décisions non ambiguës
//! - INV-DEC-2 : Décisions justifiées
//! - INV-DEC-3 : Unicité de décision
//! - INV-DIFF-NOPLAN : Décision différée sans planification
//!
//! **Garanties respectées :**
//! - G-JUST-1 : Justification explicite
//! - G-JUST-2 : Référence aux politiques
//! - G-JUST-3 : Justification complète
//! - G-NOEXEC-1 : Non-exécutabilité
//! - G-NOEXEC-2 : Aucune autorité sur l'exécution

use crate::types::{DecisionType, Justification, PolicyId};
use std::collections::HashMap;

/// Décision produite par StrongFather après évaluation d'une intention
///
/// Une décision est le résultat conceptuel produit par StrongFather après évaluation
/// d'une intention selon des politiques et des contraintes. Une décision est un jugement
/// stratégique et politique qui indique la validité, la priorité, ou le besoin de
/// clarification d'une intention, sans jamais posséder d'autorité sur l'exécution ou la persistance.
///
/// Conforme au contrat Core Decision Contract section 2 et 5.
///
/// **Composants obligatoires :**
/// - `intent_id` : Identifiant unique de l'intention évaluée (INV-DEC-3)
/// - `decision_type` : Type de décision (ACCEPTÉE, REFUSÉE, AMBIGUË, DIFFÉRÉE) (INV-DEC-1)
/// - `justification` : Justification explicite de la décision (G-JUST-1, G-JUST-2, G-JUST-3)
/// - `policies_applied` : Liste des politiques appliquées lors de l'évaluation
/// - `evaluation_context` : Contexte utilisé pour l'évaluation
/// - `metadata` : Métadonnées de traçabilité
///
/// **Invariants :**
/// - INV-DEC-1 : Décisions non ambiguës (toujours un type clair)
/// - INV-DEC-2 : Décisions justifiées (justification obligatoire)
/// - INV-DEC-3 : Unicité de décision (une seule décision par intention)
/// - INV-DIFF-NOPLAN : Décision différée sans planification
///
/// **Garanties :**
/// - G-JUST-1 : Justification explicite (toujours présente)
/// - G-JUST-2 : Référence aux politiques (justification référence les politiques)
/// - G-JUST-3 : Justification complète (justification non ambiguë)
/// - G-NOEXEC-1 : Non-exécutabilité (décision jamais exécutable)
/// - G-NOEXEC-2 : Aucune autorité sur l'exécution
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision {
    /// Identifiant unique de l'intention évaluée (obligatoire)
    ///
    /// Conforme à INV-DEC-3 (unicité de décision).
    /// Permet de lier la décision à l'intention évaluée.
    pub intent_id: String,

    /// Type de décision (obligatoire)
    ///
    /// Représente le résultat de l'évaluation : ACCEPTÉE, REFUSÉE, AMBIGUË, ou DIFFÉRÉE.
    /// Conforme à INV-DEC-1 (décisions non ambiguës).
    pub decision_type: DecisionType,

    /// Justification de la décision (obligatoire)
    ///
    /// Explication conceptuelle de la décision produite, incluant les politiques appliquées
    /// et les raisons de la décision.
    /// Conforme à G-JUST-1, G-JUST-2, G-JUST-3.
    pub justification: Justification,

    /// Liste des politiques appliquées lors de l'évaluation (obligatoire)
    ///
    /// Identifiants des politiques qui ont été évaluées pour produire cette décision.
    /// Permet la traçabilité et la justification de la décision.
    pub policies_applied: Vec<PolicyId>,

    /// Contexte d'évaluation utilisé (obligatoire)
    ///
    /// Représente le contexte dans lequel l'évaluation a été effectuée.
    /// Inclut les informations environnementales nécessaires à l'évaluation.
    pub evaluation_context: EvaluationContext,

    /// Métadonnées de traçabilité (obligatoire)
    ///
    /// Informations supplémentaires pour faciliter le suivi et l'audit de la décision.
    /// Informatives, non-évaluées par les politiques.
    pub metadata: DecisionMetadata,
}

impl Decision {
    /// Crée une nouvelle décision avec tous les composants obligatoires
    ///
    /// # Arguments
    /// * `intent_id` - Identifiant unique de l'intention évaluée
    /// * `decision_type` - Type de décision
    /// * `justification` - Justification de la décision
    /// * `policies_applied` - Liste des politiques appliquées
    /// * `evaluation_context` - Contexte d'évaluation utilisé
    /// * `metadata` - Métadonnées de traçabilité
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::decision::{Decision, DecisionMetadata, EvaluationContext};
    /// use miyukini_strongfather::types::{DecisionType, Justification, Priority};
    ///
    /// let justification = Justification::new("Intention valide selon les politiques".to_string());
    /// let context = EvaluationContext::new(
    ///     "user123".to_string(),
    ///     "product".to_string(),
    ///     "instance1".to_string(),
    /// );
    /// let metadata = DecisionMetadata::new();
    ///
    /// let decision = Decision::new(
    ///     "intent-001".to_string(),
    ///     DecisionType::Accepted { priority: Priority::new(100) },
    ///     justification,
    ///     vec!["policy1".to_string()],
    ///     context,
    ///     metadata,
    /// );
    /// ```
    pub fn new(
        intent_id: String,
        decision_type: DecisionType,
        justification: Justification,
        policies_applied: Vec<PolicyId>,
        evaluation_context: EvaluationContext,
        metadata: DecisionMetadata,
    ) -> Self {
        Self {
            intent_id,
            decision_type,
            justification,
            policies_applied,
            evaluation_context,
            metadata,
        }
    }

    /// Retourne l'identifiant de l'intention évaluée
    pub fn intent_id(&self) -> &str {
        &self.intent_id
    }

    /// Retourne le type de décision
    pub fn decision_type(&self) -> &DecisionType {
        &self.decision_type
    }

    /// Retourne la justification de la décision
    pub fn justification(&self) -> &Justification {
        &self.justification
    }

    /// Retourne la liste des politiques appliquées
    pub fn policies_applied(&self) -> &[PolicyId] {
        &self.policies_applied
    }

    /// Retourne le contexte d'évaluation
    pub fn evaluation_context(&self) -> &EvaluationContext {
        &self.evaluation_context
    }

    /// Retourne les métadonnées de traçabilité
    pub fn metadata(&self) -> &DecisionMetadata {
        &self.metadata
    }

    /// Vérifie si la décision est acceptée
    pub fn is_accepted(&self) -> bool {
        matches!(self.decision_type, DecisionType::Accepted { .. })
    }

    /// Vérifie si la décision est refusée
    pub fn is_refused(&self) -> bool {
        matches!(self.decision_type, DecisionType::Refused { .. })
    }

    /// Vérifie si la décision est ambiguë
    pub fn is_ambiguous(&self) -> bool {
        matches!(self.decision_type, DecisionType::Ambiguous { .. })
    }

    /// Vérifie si la décision est différée
    pub fn is_deferred(&self) -> bool {
        matches!(self.decision_type, DecisionType::Deferred { .. })
    }
}

/// Contexte d'évaluation utilisé pour produire une décision
///
/// Représente l'ensemble des informations environnementales nécessaires à l'évaluation
/// d'une intention selon des politiques. Le contexte est conceptuel et non-technique.
///
/// Conforme au contrat Core Decision Contract section 4.4.
///
/// **Composants :**
/// - `caller_identity` : Identifiant de l'appelant (qui a soumis l'intention)
/// - `origin` : Origine de l'appel (produit, adaptateur, etc.)
/// - `instance` : Instance concernée par l'évaluation
/// - `additional_context` : Contexte additionnel (optionnel)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluationContext {
    /// Identifiant de l'appelant (obligatoire)
    ///
    /// Qui a soumis l'intention. Conceptuel, non-technique.
    pub caller_identity: String,

    /// Origine de l'appel (obligatoire)
    ///
    /// D'où provient l'intention (produit, adaptateur, etc.).
    /// Conceptuel, non-technique.
    pub origin: String,

    /// Instance concernée par l'évaluation (obligatoire)
    ///
    /// L'instance dans laquelle l'évaluation a été effectuée.
    /// Conceptuel, non-technique.
    pub instance: String,

    /// Contexte additionnel (optionnel)
    ///
    /// Informations supplémentaires pour l'évaluation.
    /// Format conceptuel, non-technique.
    pub additional_context: HashMap<String, String>,
}

impl EvaluationContext {
    /// Crée un nouveau contexte d'évaluation
    ///
    /// # Arguments
    /// * `caller_identity` - Identifiant de l'appelant
    /// * `origin` - Origine de l'appel
    /// * `instance` - Instance concernée
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::decision::EvaluationContext;
    ///
    /// let context = EvaluationContext::new(
    ///     "user123".to_string(),
    ///     "product".to_string(),
    ///     "instance1".to_string(),
    /// );
    /// ```
    pub fn new(caller_identity: String, origin: String, instance: String) -> Self {
        Self {
            caller_identity,
            origin,
            instance,
            additional_context: HashMap::new(),
        }
    }

    /// Crée un nouveau contexte d'évaluation avec contexte additionnel
    ///
    /// # Arguments
    /// * `caller_identity` - Identifiant de l'appelant
    /// * `origin` - Origine de l'appel
    /// * `instance` - Instance concernée
    /// * `additional_context` - Contexte additionnel
    pub fn with_additional_context(
        caller_identity: String,
        origin: String,
        instance: String,
        additional_context: HashMap<String, String>,
    ) -> Self {
        Self {
            caller_identity,
            origin,
            instance,
            additional_context,
        }
    }

    /// Retourne l'identifiant de l'appelant
    pub fn caller_identity(&self) -> &str {
        &self.caller_identity
    }

    /// Retourne l'origine de l'appel
    pub fn origin(&self) -> &str {
        &self.origin
    }

    /// Retourne l'instance concernée
    pub fn instance(&self) -> &str {
        &self.instance
    }

    /// Retourne le contexte additionnel
    pub fn additional_context(&self) -> &HashMap<String, String> {
        &self.additional_context
    }
}

/// Métadonnées de traçabilité d'une décision
///
/// Représente les informations supplémentaires pour faciliter le suivi et l'audit
/// d'une décision. Les métadonnées sont informatives et non-évaluées par les politiques.
///
/// Conforme au contrat Core Decision Contract section 4.5.
///
/// **Caractéristiques :**
/// - Descriptives : Les métadonnées décrivent la décision
/// - Non-essentielles : Les métadonnées ne sont pas essentielles à la décision elle-même
/// - Informatives : Les métadonnées informent sur la décision
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionMetadata {
    /// Métadonnées sous forme de paires clé-valeur
    ///
    /// Format conceptuel et non-technique. Les valeurs sont des chaînes de caractères
    /// représentant des informations descriptives.
    metadata: HashMap<String, String>,
}

impl DecisionMetadata {
    /// Crée des métadonnées vides
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::decision::DecisionMetadata;
    ///
    /// let metadata = DecisionMetadata::new();
    /// ```
    pub fn new() -> Self {
        Self {
            metadata: HashMap::new(),
        }
    }

    /// Crée des métadonnées avec une seule paire clé-valeur
    ///
    /// # Arguments
    /// * `key` - Clé de la métadonnée
    /// * `value` - Valeur de la métadonnée
    pub fn with_entry(key: String, value: String) -> Self {
        let mut metadata = HashMap::new();
        metadata.insert(key, value);
        Self { metadata }
    }

    /// Crée des métadonnées à partir d'une HashMap
    ///
    /// # Arguments
    /// * `metadata` - HashMap contenant les métadonnées
    pub fn from_map(metadata: HashMap<String, String>) -> Self {
        Self { metadata }
    }

    /// Retourne la valeur associée à une clé (si présente)
    ///
    /// # Arguments
    /// * `key` - Clé à rechercher
    ///
    /// # Retour
    /// Option contenant la valeur si la clé existe, None sinon
    pub fn get(&self, key: &str) -> Option<&String> {
        self.metadata.get(key)
    }

    /// Retourne toutes les métadonnées sous forme de référence à la HashMap
    pub fn as_map(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    /// Vérifie si les métadonnées sont vides
    pub fn is_empty(&self) -> bool {
        self.metadata.is_empty()
    }

    /// Retourne le nombre de paires clé-valeur
    pub fn len(&self) -> usize {
        self.metadata.len()
    }
}

impl Default for DecisionMetadata {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Clarification, DecisionType, DeferralReason, Justification, Priority, RefusalReason};

    #[test]
    fn test_decision_creation() {
        let justification = Justification::new("Intention valide".to_string());
        let context = EvaluationContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let metadata = DecisionMetadata::new();

        let decision = Decision::new(
            "intent-001".to_string(),
            DecisionType::Accepted {
                priority: Priority::new(100),
            },
            justification,
            vec!["policy1".to_string()],
            context.clone(),
            metadata,
        );

        assert_eq!(decision.intent_id(), "intent-001");
        assert!(decision.is_accepted());
        assert!(!decision.is_refused());
        assert!(!decision.is_ambiguous());
        assert!(!decision.is_deferred());
        assert_eq!(decision.policies_applied().len(), 1);
        assert_eq!(decision.evaluation_context(), &context);
    }

    #[test]
    fn test_decision_refused() {
        let justification = Justification::new("Intention invalide".to_string());
        let context = EvaluationContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let metadata = DecisionMetadata::new();

        let reason = RefusalReason::PolicyViolation {
            description: "Politique violée".to_string(),
        };
        let decision = Decision::new(
            "intent-002".to_string(),
            DecisionType::Refused {
                reason,
                violated_policies: vec!["policy1".to_string()],
            },
            justification,
            vec!["policy1".to_string()],
            context,
            metadata,
        );

        assert_eq!(decision.intent_id(), "intent-002");
        assert!(!decision.is_accepted());
        assert!(decision.is_refused());
        assert!(!decision.is_ambiguous());
        assert!(!decision.is_deferred());
    }

    #[test]
    fn test_decision_ambiguous() {
        let justification = Justification::new("Informations manquantes".to_string());
        let context = EvaluationContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let metadata = DecisionMetadata::new();

        let decision = Decision::new(
            "intent-003".to_string(),
            DecisionType::Ambiguous {
                missing_information: vec!["info1".to_string()],
                clarifications_required: vec![Clarification::new(
                    "Question?".to_string(),
                    "Missing info".to_string(),
                )],
            },
            justification,
            vec![],
            context,
            metadata,
        );

        assert_eq!(decision.intent_id(), "intent-003");
        assert!(!decision.is_accepted());
        assert!(!decision.is_refused());
        assert!(decision.is_ambiguous());
        assert!(!decision.is_deferred());
    }

    #[test]
    fn test_decision_deferred() {
        let justification = Justification::new("Contexte futur requis".to_string());
        let context = EvaluationContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let metadata = DecisionMetadata::new();

        let reason = DeferralReason::ContextRequired {
            description: "Contexte requis".to_string(),
        };
        let decision = Decision::new(
            "intent-004".to_string(),
            DecisionType::Deferred {
                reason,
                context_required: vec!["context1".to_string()],
            },
            justification,
            vec![],
            context,
            metadata,
        );

        assert_eq!(decision.intent_id(), "intent-004");
        assert!(!decision.is_accepted());
        assert!(!decision.is_refused());
        assert!(!decision.is_ambiguous());
        assert!(decision.is_deferred());
    }

    #[test]
    fn test_evaluation_context() {
        let context = EvaluationContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );

        assert_eq!(context.caller_identity(), "user123");
        assert_eq!(context.origin(), "product");
        assert_eq!(context.instance(), "instance1");
        assert!(context.additional_context().is_empty());
    }

    #[test]
    fn test_evaluation_context_with_additional() {
        let mut additional = HashMap::new();
        additional.insert("key1".to_string(), "value1".to_string());
        let context = EvaluationContext::with_additional_context(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
            additional,
        );

        assert_eq!(context.additional_context().len(), 1);
        assert_eq!(
            context.additional_context().get("key1"),
            Some(&"value1".to_string())
        );
    }

    #[test]
    fn test_decision_metadata() {
        let metadata = DecisionMetadata::new();
        assert!(metadata.is_empty());
        assert_eq!(metadata.len(), 0);

        let metadata = DecisionMetadata::with_entry("key1".to_string(), "value1".to_string());
        assert!(!metadata.is_empty());
        assert_eq!(metadata.len(), 1);
        assert_eq!(metadata.get("key1"), Some(&"value1".to_string()));
        assert_eq!(metadata.get("nonexistent"), None);
    }

    #[test]
    fn test_decision_metadata_from_map() {
        let mut map = HashMap::new();
        map.insert("key1".to_string(), "value1".to_string());
        map.insert("key2".to_string(), "value2".to_string());

        let metadata = DecisionMetadata::from_map(map);
        assert_eq!(metadata.len(), 2);
        assert_eq!(metadata.get("key1"), Some(&"value1".to_string()));
        assert_eq!(metadata.get("key2"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_decision_metadata_default() {
        let metadata = DecisionMetadata::default();
        assert!(metadata.is_empty());
    }

    #[test]
    fn test_decision_equality() {
        let justification1 = Justification::new("Test".to_string());
        let justification2 = Justification::new("Test".to_string());
        let context1 = EvaluationContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let context2 = EvaluationContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let metadata1 = DecisionMetadata::new();
        let metadata2 = DecisionMetadata::new();

        let decision1 = Decision::new(
            "intent-001".to_string(),
            DecisionType::Accepted {
                priority: Priority::new(100),
            },
            justification1,
            vec!["policy1".to_string()],
            context1,
            metadata1,
        );

        let decision2 = Decision::new(
            "intent-001".to_string(),
            DecisionType::Accepted {
                priority: Priority::new(100),
            },
            justification2,
            vec!["policy1".to_string()],
            context2,
            metadata2,
        );

        assert_eq!(decision1, decision2);
    }

    #[test]
    fn test_decision_justification_access() {
        let justification = Justification::with_policies(
            "Intention valide".to_string(),
            vec!["policy1".to_string(), "policy2".to_string()],
        );
        let context = EvaluationContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let metadata = DecisionMetadata::new();

        let decision = Decision::new(
            "intent-001".to_string(),
            DecisionType::Accepted {
                priority: Priority::new(100),
            },
            justification.clone(),
            vec!["policy1".to_string(), "policy2".to_string()],
            context,
            metadata,
        );

        assert_eq!(decision.justification(), &justification);
        assert_eq!(
            decision.justification().policy_references.len(),
            2
        );
    }
}
