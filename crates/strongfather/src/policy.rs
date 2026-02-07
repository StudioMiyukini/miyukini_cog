//! # StrongFather — Modèle de Politique
//!
//! Ce module définit le modèle conceptuel des politiques appliquées par StrongFather.
//!
//! ## Références
//!
//! - [Policy Engine Contract](../../../docs/core/StrongFather/contracts/policy/StrongFather%20-%20Policy%20Engine%20Contract.md)
//! - [Policy Language Specification](../../../docs/core/StrongFather/contracts/policy/StrongFather%20—%20Policy%20Language%20Specification.md)

use std::collections::HashMap;

/// @id: strongfather_policy_type
/// @role: domain_model
/// @layer: core
/// @human: Type de politique StrongFather. Définit la catégorie conceptuelle de la politique.
/// @do: define_policy_type
/// Type de politique
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolicyType {
    /// @id: strongfather_policy_type_permission
    /// @role: domain_model
    /// @layer: core
    /// @human: Politique de permission - détermine si un acteur est autorisé à effectuer une action.
    /// @do: represent_permission_policy
    Permission,
    /// @id: strongfather_policy_type_constraint
    /// @role: domain_model
    /// @layer: core
    /// @human: Politique de contrainte - définit des conditions qui doivent être satisfaites.
    /// @do: represent_constraint_policy
    Constraint,
    /// @id: strongfather_policy_type_priority
    /// @role: domain_model
    /// @layer: core
    /// @human: Politique de priorité - détermine l'ordre d'importance relative.
    /// @do: represent_priority_policy
    Priority,
    /// @id: strongfather_policy_type_validation
    /// @role: domain_model
    /// @layer: core
    /// @human: Politique de validation - définit des vérifications conceptuelles requises.
    /// @do: represent_validation_policy
    Validation,
    /// @id: strongfather_policy_type_composite
    /// @role: domain_model
    /// @layer: core
    /// @human: Politique composite - combine plusieurs politiques élémentaires.
    /// @do: represent_composite_policy
    Composite,
}

/// @id: strongfather_policy_effect
/// @role: domain_model
/// @layer: core
/// @human: Effet d'une politique. Détermine l'impact de la politique sur l'évaluation.
/// @do: define_policy_effect
/// Effet d'une politique
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyEffect {
    /// @id: strongfather_policy_effect_authorize
    /// @role: domain_model
    /// @layer: core
    /// @human: Autorise l'intention selon la politique.
    /// @do: represent_authorize_effect
    Authorize,
    /// @id: strongfather_policy_effect_deny
    /// @role: domain_model
    /// @layer: core
    /// @human: Interdit l'intention selon la politique.
    /// @do: represent_deny_effect
    Deny,
    /// @id: strongfather_policy_effect_constrain
    /// @role: domain_model
    /// @layer: core
    /// @human: Contraint l'intention selon la politique.
    /// @do: represent_constrain_effect
    Constrain,
    /// @id: strongfather_policy_effect_prioritize
    /// @role: domain_model
    /// @layer: core
    /// @human: Priorise l'intention selon la politique.
    /// @do: represent_prioritize_effect
    Prioritize,
    /// @id: strongfather_policy_effect_validate
    /// @role: domain_model
    /// @layer: core
    /// @human: Valide l'intention selon la politique.
    /// @do: represent_validate_effect
    Validate,
}

/// @id: strongfather_policy_priority_level
/// @role: domain_model
/// @layer: core
/// @human: Niveau de priorité d'une politique. Détermine l'ordre d'évaluation.
/// @do: define_policy_priority_level
/// Niveau de priorité d'une politique
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PolicyPriorityLevel {
    /// @id: strongfather_policy_priority_critical
    /// @role: domain_model
    /// @layer: core
    /// @human: Priorité critique - évaluée en premier, arrêt immédiat si violée.
    /// @do: represent_critical_priority
    Critical = 4,
    /// @id: strongfather_policy_priority_important
    /// @role: domain_model
    /// @layer: core
    /// @human: Priorité importante - évaluée après les critiques.
    /// @do: represent_important_priority
    Important = 3,
    /// @id: strongfather_policy_priority_normal
    /// @role: domain_model
    /// @layer: core
    /// @human: Priorité normale - priorité standard par défaut.
    /// @do: represent_normal_priority
    Normal = 2,
    /// @id: strongfather_policy_priority_optional
    /// @role: domain_model
    /// @layer: core
    /// @human: Priorité optionnelle - évaluée en dernier.
    /// @do: represent_optional_priority
    Optional = 1,
}

impl Default for PolicyPriorityLevel {
    /// @id: strongfather_policy_priority_default
    /// @role: constructor
    /// @layer: core
    /// @human: Priorité normale par défaut (RÈGLE-PRIO-4).
    /// @do: provide_default_priority
    fn default() -> Self {
        Self::Normal
    }
}

/// @id: strongfather_policy_id
/// @role: domain_model
/// @layer: core
/// @human: Identifiant unique d'une politique (RÈGLE-STRUCT-1 : unicité).
/// @do: identify_policy
/// Identifiant de politique
pub type PolicyId = String;

/// @id: strongfather_policy_condition
/// @role: domain_model
/// @layer: core
/// @human: Condition d'application d'une politique. Détermine quand la politique s'applique.
/// @do: represent_policy_condition
/// Condition d'application d'une politique
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyCondition {
    /// @id: strongfather_policy_condition_expression
    /// @role: domain_model
    /// @layer: core
    /// @human: Expression déclarative de la condition (représentation conceptuelle).
    /// @do: express_condition
    pub expression: String,
}

/// @id: strongfather_policy_rule
/// @role: domain_model
/// @layer: core
/// @human: Règle déclarative d'une politique. Exprime ce qui est autorisé, interdit ou requis.
/// @do: represent_policy_rule
/// Règle déclarative d'une politique
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyRule {
    /// @id: strongfather_policy_rule_expression
    /// @role: domain_model
    /// @layer: core
    /// @human: Expression déclarative de la règle (représentation conceptuelle).
    /// @do: express_rule
    pub expression: String,
}

/// @id: strongfather_policy
/// @role: domain_model
/// @layer: core
/// @human: Politique StrongFather. Règle déclarative et explicite pour évaluer des intentions.
/// @do: represent_policy
/// Politique StrongFather
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    /// @id: strongfather_policy_id
    /// @role: domain_model
    /// @layer: core
    /// @human: Identifiant unique de la politique (RÈGLE-STRUCT-1).
    /// @do: identify_policy
    pub id: PolicyId,
    /// @id: strongfather_policy_type
    /// @role: domain_model
    /// @layer: core
    /// @human: Type de politique (RÈGLE-STRUCT-2 : type obligatoire).
    /// @do: specify_policy_type
    pub policy_type: PolicyType,
    /// @id: strongfather_policy_condition
    /// @role: domain_model
    /// @layer: core
    /// @human: Condition d'application (RÈGLE-STRUCT-3 : condition obligatoire).
    /// @do: specify_application_condition
    pub condition: PolicyCondition,
    /// @id: strongfather_policy_rule
    /// @role: domain_model
    /// @layer: core
    /// @human: Règle déclarative (RÈGLE-STRUCT-4 : règle obligatoire).
    /// @do: specify_declarative_rule
    pub rule: PolicyRule,
    /// @id: strongfather_policy_effect
    /// @role: domain_model
    /// @layer: core
    /// @human: Effet de la politique (RÈGLE-STRUCT-5 : effet obligatoire).
    /// @do: specify_policy_effect
    pub effect: PolicyEffect,
    /// @id: strongfather_policy_priority
    /// @role: domain_model
    /// @layer: core
    /// @human: Priorité relative de la politique (optionnelle, défaut: Normal).
    /// @do: specify_priority_level
    pub priority: PolicyPriorityLevel,
    /// @id: strongfather_policy_metadata
    /// @role: domain_model
    /// @layer: core
    /// @human: Métadonnées descriptives (optionnelles).
    /// @do: provide_metadata
    pub metadata: HashMap<String, String>,
    /// @id: strongfather_policy_composite_policies
    /// @role: domain_model
    /// @layer: core
    /// @human: Politiques élémentaires pour une politique composite (optionnel).
    /// @do: store_composite_policies
    pub composite_policies: Option<Vec<PolicyId>>,
}

impl Policy {
    /// @id: strongfather_policy_new
    /// @role: constructor
    /// @layer: core
    /// @human: Crée une nouvelle politique avec les composants obligatoires.
    /// @do: create_policy
    /// @depends: strongfather_policy
    #[must_use] 
    pub fn new(
        id: PolicyId,
        policy_type: PolicyType,
        condition: PolicyCondition,
        rule: PolicyRule,
        effect: PolicyEffect,
    ) -> Self {
        Self {
            id,
            policy_type,
            condition,
            rule,
            effect,
            priority: PolicyPriorityLevel::default(),
            metadata: HashMap::new(),
            composite_policies: None,
        }
    }

    /// @id: strongfather_policy_with_priority
    /// @role: builder
    /// @layer: core
    /// @human: Définit la priorité de la politique.
    /// @do: set_policy_priority
    /// @depends: strongfather_policy_new
    #[must_use] 
    pub fn with_priority(mut self, priority: PolicyPriorityLevel) -> Self {
        self.priority = priority;
        self
    }

    /// @id: strongfather_policy_with_metadata
    /// @role: builder
    /// @layer: core
    /// @human: Ajoute des métadonnées à la politique.
    /// @do: add_policy_metadata
    /// @depends: strongfather_policy_new
    #[must_use] 
    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = metadata;
        self
    }

    /// @id: strongfather_policy_with_composite
    /// @role: builder
    /// @layer: core
    /// @human: Définit les politiques élémentaires pour une politique composite.
    /// @do: set_composite_policies
    /// @depends: strongfather_policy_new
    #[must_use] 
    pub fn with_composite_policies(mut self, policies: Vec<PolicyId>) -> Self {
        self.composite_policies = Some(policies);
        self
    }

    /// @id: strongfather_policy_is_critical
    /// @role: query
    /// @layer: core
    /// @human: Vérifie si la politique est critique (RÈGLE-PRIO-2 : arrêt sur violation).
    /// @do: check_if_critical
    /// @depends: strongfather_policy
    #[must_use] 
    pub fn is_critical(&self) -> bool {
        self.priority == PolicyPriorityLevel::Critical
    }
}

/// @id: strongfather_policy_set
/// @role: domain_model
/// @layer: core
/// @human: Ensemble de politiques chargées depuis une source unique.
/// @do: represent_policy_set
/// Ensemble de politiques
#[derive(Debug, Clone)]
pub struct PolicySet {
    /// @id: strongfather_policy_set_policies
    /// @role: domain_model
    /// @layer: core
    /// @human: Collection de politiques indexée par identifiant.
    /// @do: store_policies
    policies: HashMap<PolicyId, Policy>,
}

impl PolicySet {
    /// @id: strongfather_policy_set_new
    /// @role: constructor
    /// @layer: core
    /// @human: Crée un nouvel ensemble de politiques vide.
    /// @do: create_policy_set
    /// @depends: strongfather_policy_set
    #[must_use] 
    pub fn new() -> Self {
        Self {
            policies: HashMap::new(),
        }
    }

    /// @id: strongfather_policy_set_add
    /// @role: mutator
    /// @layer: core
    /// @human: Ajoute une politique à l'ensemble (VALID-STRUCT-1 : identifiant unique).
    /// @do: add_policy_to_set
    /// @depends: strongfather_policy_set_new
    pub fn add(&mut self, policy: Policy) -> Result<(), String> {
        if self.policies.contains_key(&policy.id) {
            return Err(format!("Policy with id '{}' already exists", policy.id));
        }
        self.policies.insert(policy.id.clone(), policy);
        Ok(())
    }

    /// @id: strongfather_policy_set_get
    /// @role: query
    /// @layer: core
    /// @human: Récupère une politique par son identifiant.
    /// @do: get_policy_by_id
    /// @depends: strongfather_policy_set_new
    #[must_use] 
    pub fn get(&self, id: &PolicyId) -> Option<&Policy> {
        self.policies.get(id)
    }

    /// @id: strongfather_policy_set_iter
    /// @role: query
    /// @layer: core
    /// @human: Itère sur toutes les politiques de l'ensemble.
    /// @do: iterate_policies
    /// @depends: strongfather_policy_set_new
    pub fn iter(&self) -> impl Iterator<Item = &Policy> {
        self.policies.values()
    }

    /// @id: strongfather_policy_set_len
    /// @role: query
    /// @layer: core
    /// @human: Retourne le nombre de politiques dans l'ensemble.
    /// @do: count_policies
    /// @depends: strongfather_policy_set_new
    #[must_use] 
    pub fn len(&self) -> usize {
        self.policies.len()
    }

    /// @id: strongfather_policy_set_is_empty
    /// @role: query
    /// @layer: core
    /// @human: Vérifie si l'ensemble est vide.
    /// @do: check_if_empty
    /// @depends: strongfather_policy_set_new
    #[must_use] 
    pub fn is_empty(&self) -> bool {
        self.policies.is_empty()
    }
}

impl Default for PolicySet {
    /// @id: strongfather_policy_set_default
    /// @role: constructor
    /// @layer: core
    /// @human: Crée un ensemble de politiques vide par défaut.
    /// @do: create_default_policy_set
    fn default() -> Self {
        Self::new()
    }
}

/// @id: strongfather_policy_result
/// @role: domain_model
/// @layer: core
/// @human: Résultat d'évaluation d'une politique. Indique l'effet de la politique sur l'intention.
/// @do: represent_policy_evaluation_result
/// Résultat d'évaluation d'une politique
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyResult {
    /// @id: strongfather_policy_result_satisfied
    /// @role: domain_model
    /// @layer: core
    /// @human: Politique satisfaite - l'intention respecte la politique.
    /// @do: represent_satisfied_policy
    Satisfied {
        /// @id: strongfather_policy_result_satisfied_policy_id
        /// @role: domain_model
        /// @layer: core
        /// @human: Identifiant de la politique satisfaite.
        /// @do: identify_satisfied_policy
        policy_id: PolicyId,
    },
    /// @id: strongfather_policy_result_violated
    /// @role: domain_model
    /// @layer: core
    /// @human: Politique violée - l'intention viole la politique.
    /// @do: represent_violated_policy
    Violated {
        /// @id: strongfather_policy_result_violated_policy_id
        /// @role: domain_model
        /// @layer: core
        /// @human: Identifiant de la politique violée.
        /// @do: identify_violated_policy
        policy_id: PolicyId,
        /// @id: strongfather_policy_result_violated_reason
        /// @role: domain_model
        /// @layer: core
        /// @human: Raison de la violation.
        /// @do: explain_violation
        reason: String,
    },
    /// @id: strongfather_policy_result_priority
    /// @role: domain_model
    /// @layer: core
    /// @human: Politique de priorité - établit une priorité pour l'intention.
    /// @do: represent_priority_policy_result
    Priority {
        /// @id: strongfather_policy_result_priority_policy_id
        /// @role: domain_model
        /// @layer: core
        /// @human: Identifiant de la politique de priorité.
        /// @do: identify_priority_policy
        policy_id: PolicyId,
        /// @id: strongfather_policy_result_priority_value
        /// @role: domain_model
        /// @layer: core
        /// @human: Valeur de priorité établie.
        /// @do: store_priority_value
        value: u32,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: strongfather_policy_test_create_basic
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'une politique basique avec les composants obligatoires.
    /// @do: test_basic_policy_creation
    #[test]
    fn test_create_basic_policy() {
        let policy = Policy::new(
            "POL-001".to_string(),
            PolicyType::Permission,
            PolicyCondition {
                expression: "action == 'modify'".to_string(),
            },
            PolicyRule {
                expression: "user.role == 'admin'".to_string(),
            },
            PolicyEffect::Authorize,
        );

        assert_eq!(policy.id, "POL-001");
        assert_eq!(policy.policy_type, PolicyType::Permission);
        assert_eq!(policy.effect, PolicyEffect::Authorize);
        assert_eq!(policy.priority, PolicyPriorityLevel::Normal);
        assert!(policy.metadata.is_empty());
    }

    /// @id: strongfather_policy_test_create_with_priority
    /// @role: test
    /// @layer: core
    /// @human: Test de création d'une politique avec priorité critique.
    /// @do: test_policy_with_priority
    #[test]
    fn test_create_policy_with_priority() {
        let policy = Policy::new(
            "POL-002".to_string(),
            PolicyType::Constraint,
            PolicyCondition {
                expression: "action == 'delete'".to_string(),
            },
            PolicyRule {
                expression: "entity.dependencies.count == 0".to_string(),
            },
            PolicyEffect::Constrain,
        )
        .with_priority(PolicyPriorityLevel::Critical);

        assert_eq!(policy.priority, PolicyPriorityLevel::Critical);
        assert!(policy.is_critical());
    }

    /// @id: strongfather_policy_test_policy_set
    /// @role: test
    /// @layer: core
    /// @human: Test de gestion d'un ensemble de politiques.
    /// @do: test_policy_set_operations
    #[test]
    fn test_policy_set() {
        let mut set = PolicySet::new();
        assert!(set.is_empty());

        let policy1 = Policy::new(
            "POL-001".to_string(),
            PolicyType::Permission,
            PolicyCondition {
                expression: "true".to_string(),
            },
            PolicyRule {
                expression: "true".to_string(),
            },
            PolicyEffect::Authorize,
        );

        assert!(set.add(policy1.clone()).is_ok());
        assert_eq!(set.len(), 1);
        assert!(set.get(&"POL-001".to_string()).is_some());

        // Tentative d'ajout d'une politique avec le même ID
        let policy2 = Policy::new(
            "POL-001".to_string(),
            PolicyType::Constraint,
            PolicyCondition {
                expression: "true".to_string(),
            },
            PolicyRule {
                expression: "true".to_string(),
            },
            PolicyEffect::Deny,
        );

        assert!(set.add(policy2).is_err());
    }

    /// @id: strongfather_policy_test_all_types
    /// @role: test
    /// @layer: core
    /// @human: Test que tous les types de politiques sont bien définis.
    /// @do: test_all_policy_types
    #[test]
    fn test_all_policy_types() {
        let types = vec![
            PolicyType::Permission,
            PolicyType::Constraint,
            PolicyType::Priority,
            PolicyType::Validation,
            PolicyType::Composite,
        ];

        for (idx, policy_type) in types.iter().enumerate() {
            let policy = Policy::new(
                format!("POL-{}", idx),
                *policy_type,
                PolicyCondition {
                    expression: "true".to_string(),
                },
                PolicyRule {
                    expression: "true".to_string(),
                },
                PolicyEffect::Authorize,
            );

            assert_eq!(policy.policy_type, *policy_type);
        }
    }
}
