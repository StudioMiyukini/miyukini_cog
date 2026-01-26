//! Structures de politiques pour StrongFather
//!
//! Ce module définit les structures de base pour les politiques StrongFather :
//! - Policy : Structure principale d'une politique
//! - PolicyCondition : Condition d'application d'une politique
//! - PolicyRule : Règle déclarative d'une politique
//! - PolicySet : Collection immuable de politiques
//!
//! Conformité : Policy Engine Contract
//!
//! Invariants respectés :
//! - INV-POL-1 : Politiques explicites
//! - INV-POL-2 : Immutables pendant évaluation

use std::sync::Arc;

use crate::types::{PolicyEffect, PolicyId, PolicyPriority, PolicyType};

/// Condition d'application d'une politique
///
/// Représente la condition qui détermine quand une politique s'applique.
/// Conforme à Policy Engine Contract section 4.1.
///
/// Les conditions sont déclaratives et ne contiennent pas de logique d'exécution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyCondition {
    /// La politique s'applique toujours
    ///
    /// Utilisée pour les politiques qui doivent être évaluées pour toutes les intentions.
    Always,
    /// La politique s'applique lorsque certaines conditions sont remplies
    ///
    /// Les conditions sont exprimées de manière déclarative via des critères.
    /// Note : L'évaluation réelle de la condition sera effectuée par le PolicyEngine
    /// lors de l'application de la politique.
    When {
        /// Critères déclaratifs de la condition
        ///
        /// Représente les critères qui doivent être satisfaits pour que la politique s'applique.
        /// Format déclaratif (ex: "action_type == Creation", "subject.category == 'document'").
        criteria: String,
    },
}

impl PolicyCondition {
    /// Crée une condition "Always"
    pub fn always() -> Self {
        PolicyCondition::Always
    }

    /// Crée une condition "When" avec les critères donnés
    pub fn when(criteria: impl Into<String>) -> Self {
        PolicyCondition::When {
            criteria: criteria.into(),
        }
    }

    /// Vérifie si la condition est "Always"
    pub fn is_always(&self) -> bool {
        matches!(self, PolicyCondition::Always)
    }

    /// Retourne les critères si la condition est "When", None sinon
    pub fn criteria(&self) -> Option<&str> {
        match self {
            PolicyCondition::Always => None,
            PolicyCondition::When { criteria } => Some(criteria),
        }
    }
}

/// Règle déclarative d'une politique
///
/// Représente l'expression déclarative de la règle que la politique applique.
/// Conforme à Policy Engine Contract section 4.1.
///
/// Les règles sont déclaratives et ne contiennent pas de logique d'exécution
/// ou de logique métier spécifique.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyRule {
    /// Expression déclarative de la règle
    ///
    /// Représente la règle de manière déclarative.
    /// Format déclaratif (ex: "actor.role == 'admin'", "resource.owner == actor.id").
    expression: String,
    /// Description de la règle (optionnel)
    ///
    /// Fournit une description lisible de la règle pour faciliter la compréhension.
    description: Option<String>,
}

impl PolicyRule {
    /// Crée une nouvelle règle avec l'expression donnée
    pub fn new(expression: impl Into<String>) -> Self {
        Self {
            expression: expression.into(),
            description: None,
        }
    }

    /// Crée une nouvelle règle avec expression et description
    pub fn with_description(
        expression: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            expression: expression.into(),
            description: Some(description.into()),
        }
    }

    /// Retourne l'expression de la règle
    pub fn expression(&self) -> &str {
        &self.expression
    }

    /// Retourne la description de la règle si disponible
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

/// Structure principale d'une politique
///
/// Représente une politique StrongFather avec tous ses composants obligatoires.
/// Conforme à Policy Engine Contract section 4.1.
///
/// **Composants obligatoires :**
/// - `policy_id` : Identifiant unique de la politique
/// - `policy_type` : Type de la politique
/// - `condition` : Condition d'application
/// - `rule` : Règle déclarative
/// - `effect` : Effet de la politique
/// - `priority` : Priorité de la politique (pour résolution de conflits)
///
/// **Invariants respectés :**
/// - INV-POL-1 : Politiques explicites (tous les composants sont obligatoires)
/// - INV-POL-2 : Immutables pendant évaluation (structure Clone mais pas de mutation)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    /// Identifiant unique de la politique
    ///
    /// Doit être unique dans le PolicySet (RÈGLE-STRUCT-1).
    pub policy_id: PolicyId,
    /// Type de la politique
    ///
    /// Détermine la catégorie de la politique (permission, contrainte, priorité, validation, composite).
    pub policy_type: PolicyType,
    /// Condition d'application de la politique
    ///
    /// Détermine quand la politique s'applique (toujours ou selon certains critères).
    pub condition: PolicyCondition,
    /// Règle déclarative de la politique
    ///
    /// Expression déclarative de la règle que la politique applique.
    pub rule: PolicyRule,
    /// Effet de la politique
    ///
    /// Détermine l'effet produit lorsque la politique est appliquée (Allow, Deny, Require, Validate, Prioritize).
    pub effect: PolicyEffect,
    /// Priorité de la politique
    ///
    /// Utilisée pour la résolution de conflits entre politiques.
    /// Plus la valeur est élevée, plus la politique est prioritaire.
    pub priority: PolicyPriority,
}

impl Policy {
    /// Crée une nouvelle politique avec tous les composants obligatoires
    ///
    /// # Arguments
    /// * `policy_id` - Identifiant unique de la politique
    /// * `policy_type` - Type de la politique
    /// * `condition` - Condition d'application
    /// * `rule` - Règle déclarative
    /// * `effect` - Effet de la politique
    /// * `priority` - Priorité de la politique
    pub fn new(
        policy_id: impl Into<PolicyId>,
        policy_type: PolicyType,
        condition: PolicyCondition,
        rule: PolicyRule,
        effect: PolicyEffect,
        priority: PolicyPriority,
    ) -> Self {
        Self {
            policy_id: policy_id.into(),
            policy_type,
            condition,
            rule,
            effect,
            priority,
        }
    }

    /// Retourne l'identifiant de la politique
    pub fn id(&self) -> &PolicyId {
        &self.policy_id
    }

    /// Retourne le type de la politique
    pub fn policy_type(&self) -> &PolicyType {
        &self.policy_type
    }

    /// Retourne la condition d'application
    pub fn condition(&self) -> &PolicyCondition {
        &self.condition
    }

    /// Retourne la règle déclarative
    pub fn rule(&self) -> &PolicyRule {
        &self.rule
    }

    /// Retourne l'effet de la politique
    pub fn effect(&self) -> &PolicyEffect {
        &self.effect
    }

    /// Retourne la priorité de la politique
    pub fn priority(&self) -> PolicyPriority {
        self.priority
    }
}

/// Collection immuable de politiques
///
/// Représente un ensemble de politiques chargées depuis une source.
/// Conforme à Policy Engine Contract et Policy Source Contract.
///
/// **Caractéristiques :**
/// - Immutabilité : Les politiques ne peuvent pas être modifiées après création (INV-POL-2)
/// - Unicité : Chaque politique doit avoir un identifiant unique (RÈGLE-STRUCT-1)
/// - Partage : Utilise Arc pour permettre le partage sans copie
///
/// **Invariants respectés :**
/// - INV-POL-2 : Immutables pendant évaluation
/// - INV-POL-SOURCE : Source unique configurée (géré par PolicySource)
#[derive(Debug, Clone)]
pub struct PolicySet {
    /// Politiques contenues dans l'ensemble
    ///
    /// Utilise Arc pour permettre le partage immuable de la collection.
    policies: Arc<Vec<Policy>>,
}

impl PolicySet {
    /// Crée un nouvel ensemble vide de politiques
    pub fn new() -> Self {
        Self {
            policies: Arc::new(Vec::new()),
        }
    }

    /// Crée un nouvel ensemble de politiques à partir d'un vecteur
    ///
    /// # Arguments
    /// * `policies` - Vecteur de politiques à inclure dans l'ensemble
    ///
    /// # Note
    /// L'unicité des identifiants n'est pas vérifiée à la construction.
    /// Cette vérification doit être effectuée par PolicySource lors du chargement.
    pub fn from_vec(policies: Vec<Policy>) -> Self {
        Self {
            policies: Arc::new(policies),
        }
    }

    /// Retourne le nombre de politiques dans l'ensemble
    pub fn len(&self) -> usize {
        self.policies.len()
    }

    /// Vérifie si l'ensemble est vide
    pub fn is_empty(&self) -> bool {
        self.policies.is_empty()
    }

    /// Retourne un itérateur sur les politiques
    pub fn iter(&self) -> impl Iterator<Item = &Policy> {
        self.policies.iter()
    }

    /// Recherche une politique par son identifiant
    ///
    /// # Arguments
    /// * `policy_id` - Identifiant de la politique à rechercher
    ///
    /// # Retour
    /// Some(&Policy) si trouvée, None sinon
    pub fn find_by_id(&self, policy_id: &PolicyId) -> Option<&Policy> {
        self.policies.iter().find(|p| p.policy_id == *policy_id)
    }

    /// Vérifie si une politique avec l'identifiant donné existe
    pub fn contains(&self, policy_id: &PolicyId) -> bool {
        self.find_by_id(policy_id).is_some()
    }

    /// Retourne toutes les politiques d'un type donné
    ///
    /// # Arguments
    /// * `policy_type` - Type de politique à filtrer
    ///
    /// # Retour
    /// Vecteur de références aux politiques du type donné
    pub fn by_type(&self, policy_type: &PolicyType) -> Vec<&Policy> {
        self.policies
            .iter()
            .filter(|p| &p.policy_type == policy_type)
            .collect()
    }
}

impl Default for PolicySet {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for &'a PolicySet {
    type Item = &'a Policy;
    type IntoIter = std::slice::Iter<'a, Policy>;

    fn into_iter(self) -> Self::IntoIter {
        self.policies.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{LogicalOperator, PolicyEffect, PolicyPriority, PolicyType};

    #[test]
    fn test_policy_condition_always() {
        let condition = PolicyCondition::always();
        assert!(condition.is_always());
        assert_eq!(condition.criteria(), None);
    }

    #[test]
    fn test_policy_condition_when() {
        let condition = PolicyCondition::when("action_type == Creation");
        assert!(!condition.is_always());
        assert_eq!(condition.criteria(), Some("action_type == Creation"));
    }

    #[test]
    fn test_policy_rule_new() {
        let rule = PolicyRule::new("actor.role == 'admin'");
        assert_eq!(rule.expression(), "actor.role == 'admin'");
        assert_eq!(rule.description(), None);
    }

    #[test]
    fn test_policy_rule_with_description() {
        let rule = PolicyRule::with_description(
            "actor.role == 'admin'",
            "L'acteur doit avoir le rôle admin",
        );
        assert_eq!(rule.expression(), "actor.role == 'admin'");
        assert_eq!(rule.description(), Some("L'acteur doit avoir le rôle admin"));
    }

    #[test]
    fn test_policy_creation() {
        let policy = Policy::new(
            "POL-001",
            PolicyType::Permission,
            PolicyCondition::always(),
            PolicyRule::new("actor.role == 'admin'"),
            PolicyEffect::Allow,
            PolicyPriority::new(100),
        );

        assert_eq!(policy.id(), "POL-001");
        assert_eq!(policy.policy_type(), &PolicyType::Permission);
        assert!(policy.condition().is_always());
        assert_eq!(policy.rule().expression(), "actor.role == 'admin'");
        assert_eq!(policy.effect(), &PolicyEffect::Allow);
        assert_eq!(policy.priority().value(), 100);
    }

    #[test]
    fn test_policy_composite() {
        let policy = Policy::new(
            "POL-002",
            PolicyType::Composite {
                operator: LogicalOperator::And,
                policies: vec!["POL-001".to_string(), "POL-003".to_string()],
            },
            PolicyCondition::when("action_type == Creation"),
            PolicyRule::new("composite_rule"),
            PolicyEffect::Require,
            PolicyPriority::new(50),
        );

        assert_eq!(policy.id(), "POL-002");
        match policy.policy_type() {
            PolicyType::Composite { operator, policies } => {
                assert_eq!(operator, &LogicalOperator::And);
                assert_eq!(policies.len(), 2);
            }
            _ => panic!("Expected Composite policy type"),
        }
    }

    #[test]
    fn test_policy_set_new() {
        let set = PolicySet::new();
        assert!(set.is_empty());
        assert_eq!(set.len(), 0);
    }

    #[test]
    fn test_policy_set_from_vec() {
        let policies = vec![
            Policy::new(
                "POL-001",
                PolicyType::Permission,
                PolicyCondition::always(),
                PolicyRule::new("rule1"),
                PolicyEffect::Allow,
                PolicyPriority::new(100),
            ),
            Policy::new(
                "POL-002",
                PolicyType::Constraint,
                PolicyCondition::when("action_type == Creation"),
                PolicyRule::new("rule2"),
                PolicyEffect::Require,
                PolicyPriority::new(50),
            ),
        ];

        let set = PolicySet::from_vec(policies);
        assert_eq!(set.len(), 2);
        assert!(!set.is_empty());
    }

    #[test]
    fn test_policy_set_find_by_id() {
        let policies = vec![
            Policy::new(
                "POL-001",
                PolicyType::Permission,
                PolicyCondition::always(),
                PolicyRule::new("rule1"),
                PolicyEffect::Allow,
                PolicyPriority::new(100),
            ),
            Policy::new(
                "POL-002",
                PolicyType::Constraint,
                PolicyCondition::always(),
                PolicyRule::new("rule2"),
                PolicyEffect::Require,
                PolicyPriority::new(50),
            ),
        ];

        let set = PolicySet::from_vec(policies);

        let found = set.find_by_id(&"POL-001".to_string());
        assert!(found.is_some());
        assert_eq!(found.unwrap().id(), "POL-001");

        let not_found = set.find_by_id(&"POL-999".to_string());
        assert!(not_found.is_none());
    }

    #[test]
    fn test_policy_set_contains() {
        let policies = vec![Policy::new(
            "POL-001",
            PolicyType::Permission,
            PolicyCondition::always(),
            PolicyRule::new("rule1"),
            PolicyEffect::Allow,
            PolicyPriority::new(100),
        )];

        let set = PolicySet::from_vec(policies);

        assert!(set.contains(&"POL-001".to_string()));
        assert!(!set.contains(&"POL-999".to_string()));
    }

    #[test]
    fn test_policy_set_by_type() {
        let policies = vec![
            Policy::new(
                "POL-001",
                PolicyType::Permission,
                PolicyCondition::always(),
                PolicyRule::new("rule1"),
                PolicyEffect::Allow,
                PolicyPriority::new(100),
            ),
            Policy::new(
                "POL-002",
                PolicyType::Permission,
                PolicyCondition::always(),
                PolicyRule::new("rule2"),
                PolicyEffect::Deny,
                PolicyPriority::new(50),
            ),
            Policy::new(
                "POL-003",
                PolicyType::Constraint,
                PolicyCondition::always(),
                PolicyRule::new("rule3"),
                PolicyEffect::Require,
                PolicyPriority::new(75),
            ),
        ];

        let set = PolicySet::from_vec(policies);

        let permissions = set.by_type(&PolicyType::Permission);
        assert_eq!(permissions.len(), 2);

        let constraints = set.by_type(&PolicyType::Constraint);
        assert_eq!(constraints.len(), 1);
    }

    #[test]
    fn test_policy_set_iter() {
        let policies = vec![
            Policy::new(
                "POL-001",
                PolicyType::Permission,
                PolicyCondition::always(),
                PolicyRule::new("rule1"),
                PolicyEffect::Allow,
                PolicyPriority::new(100),
            ),
            Policy::new(
                "POL-002",
                PolicyType::Constraint,
                PolicyCondition::always(),
                PolicyRule::new("rule2"),
                PolicyEffect::Require,
                PolicyPriority::new(50),
            ),
        ];

        let set = PolicySet::from_vec(policies);

        let count: usize = set.iter().count();
        assert_eq!(count, 2);

        let ids: Vec<&PolicyId> = set.iter().map(|p| p.id()).collect();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&&"POL-001".to_string()));
        assert!(ids.contains(&&"POL-002".to_string()));
    }

    #[test]
    fn test_policy_set_into_iterator() {
        let policies = vec![Policy::new(
            "POL-001",
            PolicyType::Permission,
            PolicyCondition::always(),
            PolicyRule::new("rule1"),
            PolicyEffect::Allow,
            PolicyPriority::new(100),
        )];

        let set = PolicySet::from_vec(policies);

        let count: usize = (&set).into_iter().count();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_policy_set_immutability() {
        let policies = vec![Policy::new(
            "POL-001",
            PolicyType::Permission,
            PolicyCondition::always(),
            PolicyRule::new("rule1"),
            PolicyEffect::Allow,
            PolicyPriority::new(100),
        )];

        let set1 = PolicySet::from_vec(policies);
        let set2 = set1.clone(); // Clone partage la même Arc

        assert_eq!(set1.len(), set2.len());
        assert_eq!(set1.find_by_id(&"POL-001".to_string()).unwrap().id(), "POL-001");
        assert_eq!(set2.find_by_id(&"POL-001".to_string()).unwrap().id(), "POL-001");
    }

    #[test]
    fn test_policy_effect_prioritize() {
        let policy = Policy::new(
            "POL-001",
            PolicyType::Priority,
            PolicyCondition::always(),
            PolicyRule::new("rule1"),
            PolicyEffect::Prioritize { level: 150 },
            PolicyPriority::new(100),
        );

        match policy.effect() {
            PolicyEffect::Prioritize { level } => assert_eq!(*level, 150),
            _ => panic!("Expected Prioritize effect"),
        }
    }

    #[test]
    fn test_policy_condition_clone() {
        let condition1 = PolicyCondition::when("action_type == Creation");
        let condition2 = condition1.clone();
        assert_eq!(condition1, condition2);
    }

    #[test]
    fn test_policy_rule_clone() {
        let rule1 = PolicyRule::with_description("rule1", "description1");
        let rule2 = rule1.clone();
        assert_eq!(rule1, rule2);
    }

    #[test]
    fn test_policy_clone() {
        let policy1 = Policy::new(
            "POL-001",
            PolicyType::Permission,
            PolicyCondition::always(),
            PolicyRule::new("rule1"),
            PolicyEffect::Allow,
            PolicyPriority::new(100),
        );
        let policy2 = policy1.clone();
        assert_eq!(policy1, policy2);
    }
}
