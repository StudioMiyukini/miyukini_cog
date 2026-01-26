//! Structure Intent pour StrongFather
//!
//! Ce module définit la structure des intentions soumises à StrongFather pour évaluation.
//! Conforme au contrat Intent Model Contract (FONDATION).
//!
//! **Invariants respectés :**
//! - INV-INT-1 : Identifiant obligatoire et unique
//! - INV-INT-2 : Type d'action obligatoire
//! - INV-INT-3 : Contexte d'appel obligatoire et complet
//! - INV-INT-4 : Non-exécution (les intentions sont uniquement évaluées)
//! - INV-INT-5 : Non-modification d'état (les intentions sont déclaratives)
//! - INV-ID-GLOBAL : Unicité globale de l'identifiant

use crate::types::{ActionType, Priority};
use std::collections::HashMap;

/// Intention soumise à StrongFather pour évaluation
///
/// Une intention est une demande conceptuelle d'évaluation qui représente ce qu'un
/// appelant souhaite faire évaluer par le moteur de décision, sans jamais constituer
/// une commande d'exécution ou une instruction technique.
///
/// Conforme au contrat Intent Model Contract section 2 et 3.
///
/// **Composants obligatoires :**
/// - `intent_id` : Identifiant unique (INV-INT-1, R-ID-1)
/// - `action_type` : Type d'action (INV-INT-2, R-TYPE-1)
/// - `subject` : Sujet de l'intention (R-SUBJ-1)
/// - `call_context` : Contexte d'appel complet (INV-INT-3, R-CTX-1, R-CTX-3)
/// - `data` : Données de l'intention (R-DATA-1, peut être vide pour certains types)
///
/// **Composants optionnels :**
/// - `requested_priority` : Priorité demandée par l'appelant (R-PRIO-1)
/// - `constraints` : Contraintes explicites (R-CONSTR-1)
/// - `metadata` : Métadonnées de traçabilité (R-META-1)
/// - `references` : Références croisées vers d'autres intentions (R-REF-1)
///
/// **Invariants :**
/// - INV-INT-1 : Identifiant obligatoire et immutable
/// - INV-INT-2 : Type d'action obligatoire (un parmi les types autorisés)
/// - INV-INT-3 : Contexte d'appel obligatoire et complet
/// - INV-INT-4 : Non-exécution (les intentions ne sont jamais exécutées)
/// - INV-INT-5 : Non-modification d'état (les intentions sont déclaratives)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Intent {
    /// Identifiant unique de l'intention (obligatoire, immutable)
    ///
    /// Conforme à INV-INT-1, R-ID-1, R-ID-2.
    /// L'identifiant est conceptuel (non-technique) et permet la traçabilité.
    pub intent_id: String,

    /// Type d'action de l'intention (obligatoire)
    ///
    /// Conforme à INV-INT-2, R-TYPE-1, R-TYPE-2.
    /// Doit être l'un des types autorisés : Creation, Modification, Deletion, Read, Evaluation.
    pub action_type: ActionType,

    /// Sujet de l'intention (obligatoire)
    ///
    /// L'entité, le fait, ou le concept sur lequel porte l'intention.
    /// Conforme à R-SUBJ-1, R-SUBJ-2, R-SUBJ-3.
    /// Doit être identifiable, conceptuel, et cohérent avec le type d'action.
    pub subject: String,

    /// Contexte d'appel (obligatoire, complet)
    ///
    /// Ensemble des informations décrivant qui soumet l'intention et dans quel cadre.
    /// Conforme à INV-INT-3, R-CTX-1, R-CTX-2, R-CTX-3.
    /// Le contexte n'est jamais présupposé valide (zero-trust, INV-INT-6).
    pub call_context: CallContext,

    /// Données de l'intention (obligatoire, peut être vide)
    ///
    /// Informations descriptives associées à l'action souhaitée.
    /// Conforme à R-DATA-1, R-DATA-2, R-DATA-3.
    /// Les données sont descriptives, non-exécutables, et pertinentes par rapport au type d'action.
    pub data: IntentData,

    /// Priorité demandée par l'appelant (optionnel)
    ///
    /// Indication fournie par l'appelant sur l'importance relative qu'il attribue à l'intention.
    /// Conforme à R-PRIO-1, R-PRIO-2, R-PRIO-3.
    /// Indicative, non contraignante. StrongFather peut l'ignorer.
    pub requested_priority: Option<Priority>,

    /// Contraintes explicites (optionnel)
    ///
    /// Conditions supplémentaires fournies par l'appelant qui doivent être respectées.
    /// Conforme à R-CONSTR-1, R-CONSTR-2, R-CONSTR-3.
    /// Déclaratives, additionnelles aux politiques, évaluables par StrongFather.
    pub constraints: Vec<Constraint>,

    /// Métadonnées de traçabilité (optionnel)
    ///
    /// Informations supplémentaires pour faciliter le suivi et l'audit de l'intention.
    /// Conforme à R-META-1, R-META-2, R-META-3.
    /// Informatives, non-évaluées par les politiques, conservées dans les décisions.
    pub metadata: HashMap<String, String>,

    /// Références croisées vers d'autres intentions ou décisions (optionnel)
    ///
    /// Liens vers d'autres intentions ou décisions ayant une relation conceptuelle.
    /// Conforme à R-REF-1, R-REF-2, R-REF-3.
    /// Relationnelles, informatives, optionnelles, ne créent pas de dépendances cycliques.
    pub references: Vec<String>,
}

impl Intent {
    /// Crée une nouvelle intention avec les composants obligatoires
    ///
    /// # Arguments
    /// * `intent_id` - Identifiant unique de l'intention
    /// * `action_type` - Type d'action
    /// * `subject` - Sujet de l'intention
    /// * `call_context` - Contexte d'appel complet
    /// * `data` - Données de l'intention
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::intent::{Intent, CallContext, IntentData};
    /// use miyukini_strongfather::types::ActionType;
    ///
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
    /// ```
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
            references: Vec::new(),
        }
    }

    /// Crée une nouvelle intention avec priorité demandée
    ///
    /// # Arguments
    /// * `intent_id` - Identifiant unique de l'intention
    /// * `action_type` - Type d'action
    /// * `subject` - Sujet de l'intention
    /// * `call_context` - Contexte d'appel complet
    /// * `data` - Données de l'intention
    /// * `requested_priority` - Priorité demandée par l'appelant
    pub fn with_priority(
        intent_id: String,
        action_type: ActionType,
        subject: String,
        call_context: CallContext,
        data: IntentData,
        requested_priority: Priority,
    ) -> Self {
        Self {
            intent_id,
            action_type,
            subject,
            call_context,
            data,
            requested_priority: Some(requested_priority),
            constraints: Vec::new(),
            metadata: HashMap::new(),
            references: Vec::new(),
        }
    }

    /// Retourne l'identifiant de l'intention
    pub fn intent_id(&self) -> &str {
        &self.intent_id
    }

    /// Retourne le type d'action
    pub fn action_type(&self) -> ActionType {
        self.action_type
    }

    /// Retourne le sujet de l'intention
    pub fn subject(&self) -> &str {
        &self.subject
    }

    /// Retourne le contexte d'appel
    pub fn call_context(&self) -> &CallContext {
        &self.call_context
    }

    /// Retourne les données de l'intention
    pub fn data(&self) -> &IntentData {
        &self.data
    }

    /// Retourne la priorité demandée (si présente)
    pub fn requested_priority(&self) -> Option<Priority> {
        self.requested_priority
    }

    /// Retourne les contraintes explicites
    pub fn constraints(&self) -> &[Constraint] {
        &self.constraints
    }

    /// Retourne les métadonnées de traçabilité
    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    /// Retourne les références croisées
    pub fn references(&self) -> &[String] {
        &self.references
    }
}

/// Contexte d'appel d'une intention
///
/// Représente l'ensemble des informations décrivant qui soumet l'intention et dans quel cadre.
/// Conforme au contrat Intent Model Contract section 3.4.
///
/// **Composants obligatoires :**
/// - `caller_identity` : Identifiant de l'appelant (qui soumet l'intention)
/// - `origin` : Origine de l'appel (d'où provient l'intention : produit, adaptateur)
/// - `instance` : Instance concernée par l'intention
///
/// **Invariants :**
/// - INV-INT-3 : Contexte obligatoire et complet
/// - INV-INT-6 : Le contexte n'est jamais présupposé valide (zero-trust)
/// - R-CTX-1, R-CTX-2, R-CTX-3 : Règles de contexte
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallContext {
    /// Identifiant de l'appelant (obligatoire)
    ///
    /// Qui soumet l'intention. Conceptuel, non-technique.
    /// Conforme à R-CTX-1, R-CTX-3.
    pub caller_identity: String,

    /// Origine de l'appel (obligatoire)
    ///
    /// D'où provient l'intention (produit, adaptateur, etc.).
    /// Conceptuel, non-technique.
    /// Conforme à R-CTX-1, R-CTX-3.
    pub origin: String,

    /// Instance concernée par l'intention (obligatoire)
    ///
    /// L'instance dans laquelle l'intention est soumise.
    /// Conceptuel, non-technique.
    /// Conforme à R-CTX-1, R-CTX-3.
    pub instance: String,
}

impl CallContext {
    /// Crée un nouveau contexte d'appel
    ///
    /// # Arguments
    /// * `caller_identity` - Identifiant de l'appelant
    /// * `origin` - Origine de l'appel
    /// * `instance` - Instance concernée
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::intent::CallContext;
    ///
    /// let context = CallContext::new(
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
}

/// État du cycle de vie d'une intention dans StrongFather
///
/// Représente l'état conceptuel d'une intention dans son cycle de vie.
/// Conforme au contrat Intent Model Contract section 5.
///
/// **États :**
/// - `Submitted` : L'intention a été soumise à StrongFather pour évaluation (SOUMISE)
/// - `InEvaluation` : L'intention est en cours d'évaluation selon les politiques (EN_ÉVALUATION)
/// - `Decided` : Une décision a été produite pour l'intention (DÉCIDÉE)
///
/// **Caractéristiques :**
/// - Unidirectionnel : Le cycle est unidirectionnel (pas de retour arrière)
/// - Non-technique : Les états sont conceptuels, pas techniques
/// - Terminant : Toute intention termine dans l'état DÉCIDÉE (INV-CYCLE-1)
///
/// **Invariants :**
/// - INV-CYCLE-1 : Terminaison garantie
/// - INV-CYCLE-2 : Unicité de décision
/// - INV-CYCLE-3 : Irréversibilité
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntentState {
    /// État SOUMISE : L'intention a été soumise à StrongFather pour évaluation
    ///
    /// Transition : SOUMISE → EN_ÉVALUATION
    /// Conditions : Intention structurellement valide, tous les composants obligatoires présents
    Submitted,

    /// État EN_ÉVALUATION : L'intention est en cours d'évaluation selon les politiques
    ///
    /// Transition : EN_ÉVALUATION → DÉCIDÉE
    /// Conditions : Évaluation terminée, décision produite, décision associée à l'identifiant
    InEvaluation,

    /// État DÉCIDÉE : Une décision a été produite pour l'intention
    ///
    /// État final. L'intention ne peut plus changer d'état (INV-CYCLE-3).
    Decided,
}

impl IntentState {
    /// Retourne une représentation textuelle de l'état
    pub fn as_str(&self) -> &'static str {
        match self {
            IntentState::Submitted => "SOUMISE",
            IntentState::InEvaluation => "EN_ÉVALUATION",
            IntentState::Decided => "DÉCIDÉE",
        }
    }
}

/// Données de l'intention
///
/// Représente les informations descriptives associées à l'action souhaitée.
/// Conforme au contrat Intent Model Contract section 3.5.
///
/// **Caractéristiques :**
/// - Descriptives : Les données décrivent ce qui est souhaité
/// - Non-exécutables : Les données ne sont pas des instructions d'exécution
/// - Pertinentes : Les données doivent être pertinentes par rapport au type d'action
///
/// **Règles :**
/// - R-DATA-1 : Toute intention DOIT posséder des données associées (peuvent être vides)
/// - R-DATA-2 : Les données NE DOIVENT JAMAIS contenir de commandes d'exécution
/// - R-DATA-3 : Les données DOIVENT être cohérentes avec le type d'action
///
/// **Implémentation :**
/// Les données sont représentées comme une structure flexible permettant de stocker
/// des informations descriptives sous forme de paires clé-valeur. Le format est conceptuel
/// et non-technique, permettant au produit de définir sa propre structure de données.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntentData {
    /// Données descriptives sous forme de paires clé-valeur
    ///
    /// Format conceptuel et non-technique. Les valeurs sont des chaînes de caractères
    /// représentant des informations descriptives, jamais des commandes d'exécution.
    data: HashMap<String, String>,
}

impl IntentData {
    /// Crée des données d'intention vides
    ///
    /// Conforme à R-DATA-1 (les données peuvent être vides pour certains types d'action).
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::intent::IntentData;
    ///
    /// let data = IntentData::empty();
    /// ```
    pub fn empty() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    /// Crée des données d'intention avec une seule paire clé-valeur
    ///
    /// # Arguments
    /// * `key` - Clé de la donnée
    /// * `value` - Valeur de la donnée
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::intent::IntentData;
    ///
    /// let data = IntentData::with_entry("title".to_string(), "Mon document".to_string());
    /// ```
    pub fn with_entry(key: String, value: String) -> Self {
        let mut data = HashMap::new();
        data.insert(key, value);
        Self { data }
    }

    /// Crée des données d'intention à partir d'une HashMap
    ///
    /// # Arguments
    /// * `data` - HashMap contenant les données descriptives
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::intent::IntentData;
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("title".to_string(), "Mon document".to_string());
    /// map.insert("content".to_string(), "Contenu du document".to_string());
    /// let data = IntentData::from_map(map);
    /// ```
    pub fn from_map(data: HashMap<String, String>) -> Self {
        Self { data }
    }

    /// Retourne la valeur associée à une clé (si présente)
    ///
    /// # Arguments
    /// * `key` - Clé à rechercher
    ///
    /// # Retour
    /// Option contenant la valeur si la clé existe, None sinon
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    /// Retourne toutes les données sous forme de référence à la HashMap
    pub fn as_map(&self) -> &HashMap<String, String> {
        &self.data
    }

    /// Vérifie si les données sont vides
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Retourne le nombre de paires clé-valeur
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

/// Contrainte explicite
///
/// Représente une condition supplémentaire fournie par l'appelant qui doit être respectée.
/// Conforme au contrat Intent Model Contract section 4.2.
///
/// **Caractéristiques :**
/// - Déclaratives : Les contraintes sont déclaratives, pas techniques
/// - Additionnelles : Les contraintes s'ajoutent aux politiques, sans les remplacer
/// - Évaluables : Les contraintes doivent être évaluables par StrongFather
///
/// **Règles :**
/// - R-CONSTR-1 : Les contraintes explicites sont optionnelles
/// - R-CONSTR-2 : Les contraintes NE PEUVENT JAMAIS contredire les politiques
/// - R-CONSTR-3 : Les contraintes DOIVENT être évaluables par StrongFather
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constraint {
    /// Identifiant de la contrainte
    pub constraint_id: String,

    /// Description de la contrainte
    pub description: String,

    /// Condition à évaluer (format conceptuel, non-technique)
    pub condition: String,
}

impl Constraint {
    /// Crée une nouvelle contrainte
    ///
    /// # Arguments
    /// * `constraint_id` - Identifiant de la contrainte
    /// * `description` - Description de la contrainte
    /// * `condition` - Condition à évaluer
    pub fn new(constraint_id: String, description: String, condition: String) -> Self {
        Self {
            constraint_id,
            description,
            condition,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ActionType;

    #[test]
    fn test_intent_creation() {
        let context = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let intent = Intent::new(
            "intent-001".to_string(),
            ActionType::Creation,
            "document".to_string(),
            context.clone(),
            IntentData::empty(),
        );

        assert_eq!(intent.intent_id(), "intent-001");
        assert_eq!(intent.action_type(), ActionType::Creation);
        assert_eq!(intent.subject(), "document");
        assert_eq!(intent.call_context(), &context);
        assert!(intent.requested_priority().is_none());
        assert!(intent.constraints().is_empty());
        assert!(intent.metadata().is_empty());
        assert!(intent.references().is_empty());
    }

    #[test]
    fn test_intent_with_priority() {
        let context = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let priority = Priority::new(100);
        let intent = Intent::with_priority(
            "intent-002".to_string(),
            ActionType::Modification,
            "document".to_string(),
            context,
            IntentData::empty(),
            priority,
        );

        assert_eq!(intent.intent_id(), "intent-002");
        assert_eq!(intent.action_type(), ActionType::Modification);
        assert!(intent.requested_priority().is_some());
        assert_eq!(intent.requested_priority().unwrap().value(), 100);
    }

    #[test]
    fn test_call_context() {
        let context = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );

        assert_eq!(context.caller_identity(), "user123");
        assert_eq!(context.origin(), "product");
        assert_eq!(context.instance(), "instance1");
    }

    #[test]
    fn test_intent_state() {
        assert_eq!(IntentState::Submitted.as_str(), "SOUMISE");
        assert_eq!(IntentState::InEvaluation.as_str(), "EN_ÉVALUATION");
        assert_eq!(IntentState::Decided.as_str(), "DÉCIDÉE");
    }

    #[test]
    fn test_intent_data_empty() {
        let data = IntentData::empty();
        assert!(data.is_empty());
        assert_eq!(data.len(), 0);
    }

    #[test]
    fn test_intent_data_with_entry() {
        let data = IntentData::with_entry("title".to_string(), "Mon document".to_string());
        assert!(!data.is_empty());
        assert_eq!(data.len(), 1);
        assert_eq!(data.get("title"), Some(&"Mon document".to_string()));
        assert_eq!(data.get("nonexistent"), None);
    }

    #[test]
    fn test_intent_data_from_map() {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        map.insert("title".to_string(), "Mon document".to_string());
        map.insert("content".to_string(), "Contenu".to_string());

        let data = IntentData::from_map(map);
        assert_eq!(data.len(), 2);
        assert_eq!(data.get("title"), Some(&"Mon document".to_string()));
        assert_eq!(data.get("content"), Some(&"Contenu".to_string()));
    }

    #[test]
    fn test_constraint() {
        let constraint = Constraint::new(
            "constraint-001".to_string(),
            "Description".to_string(),
            "condition".to_string(),
        );

        assert_eq!(constraint.constraint_id, "constraint-001");
        assert_eq!(constraint.description, "Description");
        assert_eq!(constraint.condition, "condition");
    }

    #[test]
    fn test_intent_equality() {
        let context1 = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );

        let intent1 = Intent::new(
            "intent-001".to_string(),
            ActionType::Creation,
            "document".to_string(),
            context1.clone(),
            IntentData::empty(),
        );

        let intent2 = Intent::new(
            "intent-001".to_string(),
            ActionType::Creation,
            "document".to_string(),
            context1,
            IntentData::empty(),
        );

        assert_eq!(intent1, intent2);
    }

    #[test]
    fn test_intent_state_equality() {
        assert_eq!(IntentState::Submitted, IntentState::Submitted);
        assert_ne!(IntentState::Submitted, IntentState::InEvaluation);
        assert_ne!(IntentState::Submitted, IntentState::Decided);
    }

    #[test]
    fn test_call_context_equality() {
        let context1 = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let context2 = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        let context3 = CallContext::new(
            "user456".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );

        assert_eq!(context1, context2);
        assert_ne!(context1, context3);
    }
}
