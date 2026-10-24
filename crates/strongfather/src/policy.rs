//! # StrongFather — Modèle de Politique
//!
//! Ce module définit le modèle conceptuel des politiques appliquées par StrongFather.
//!
//! ## Références
//!
//! - [Policy Engine Contract](../../../docs/core/StrongFather/contracts/policy/StrongFather%20-%20Policy%20Engine%20Contract.md)
//! - [Policy Language Specification](../../../docs/core/StrongFather/contracts/policy/StrongFather%20—%20Policy%20Language%20Specification.md)

use std::collections::HashMap;

use crate::intent::{ActionType, Intent};

/// Clé de métadonnée d'intention portant la zone (H0, H1…) utilisée par [`PolicyTarget::zones`].
pub const ZONE_METADATA_KEY: &str = "zone";

/// @id strongfather_policy_type
/// @role domain_model
/// @layer core
/// @do define_policy_type
//@human / @human Type de politique StrongFather. Définit la catégorie conceptuelle de la politique.
/// Type de politique
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PolicyType {
    /// @id strongfather_policy_type_permission
    /// @role domain_model
    /// @layer core
    /// @do represent_permission_policy
    //@human / @human Politique de permission - détermine si un acteur est autorisé à effectuer une action.
    Permission,
    /// @id strongfather_policy_type_constraint
    /// @role domain_model
    /// @layer core
    /// @do represent_constraint_policy
    //@human / @human Politique de contrainte - définit des conditions qui doivent être satisfaites.
    Constraint,
    /// @id strongfather_policy_type_priority
    /// @role domain_model
    /// @layer core
    /// @do represent_priority_policy
    //@human / @human Politique de priorité - détermine l'ordre d'importance relative.
    Priority,
    /// @id strongfather_policy_type_validation
    /// @role domain_model
    /// @layer core
    /// @do represent_validation_policy
    //@human / @human Politique de validation - définit des vérifications conceptuelles requises.
    Validation,
    /// @id strongfather_policy_type_composite
    /// @role domain_model
    /// @layer core
    /// @do represent_composite_policy
    //@human / @human Politique composite - combine plusieurs politiques élémentaires.
    Composite,
}

/// @id strongfather_policy_effect
/// @role domain_model
/// @layer core
/// @do define_policy_effect
//@human / @human Effet d'une politique. Détermine l'impact de la politique sur l'évaluation.
/// Effet d'une politique
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyEffect {
    /// @id strongfather_policy_effect_authorize
    /// @role domain_model
    /// @layer core
    /// @do represent_authorize_effect
    //@human / @human Autorise l'intention selon la politique.
    Authorize,
    /// @id strongfather_policy_effect_deny
    /// @role domain_model
    /// @layer core
    /// @do represent_deny_effect
    //@human / @human Interdit l'intention selon la politique.
    Deny,
    /// @id strongfather_policy_effect_constrain
    /// @role domain_model
    /// @layer core
    /// @do represent_constrain_effect
    //@human / @human Contraint l'intention selon la politique.
    Constrain,
    /// @id strongfather_policy_effect_prioritize
    /// @role domain_model
    /// @layer core
    /// @do represent_prioritize_effect
    //@human / @human Priorise l'intention selon la politique.
    Prioritize,
    /// @id strongfather_policy_effect_validate
    /// @role domain_model
    /// @layer core
    /// @do represent_validate_effect
    //@human / @human Valide l'intention selon la politique.
    Validate,
}

/// @id strongfather_policy_priority_level
/// @role domain_model
/// @layer core
/// @do define_policy_priority_level
//@human / @human Niveau de priorité d'une politique. Détermine l'ordre d'évaluation.
/// Niveau de priorité d'une politique
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PolicyPriorityLevel {
    /// @id strongfather_policy_priority_critical
    /// @role domain_model
    /// @layer core
    /// @do represent_critical_priority
    //@human / @human Priorité critique - évaluée en premier, arrêt immédiat si violée.
    Critical = 4,
    /// @id strongfather_policy_priority_important
    /// @role domain_model
    /// @layer core
    /// @do represent_important_priority
    //@human / @human Priorité importante - évaluée après les critiques.
    Important = 3,
    /// @id strongfather_policy_priority_normal
    /// @role domain_model
    /// @layer core
    /// @do represent_normal_priority
    //@human / @human Priorité normale - priorité standard par défaut.
    Normal = 2,
    /// @id strongfather_policy_priority_optional
    /// @role domain_model
    /// @layer core
    /// @do represent_optional_priority
    //@human / @human Priorité optionnelle - évaluée en dernier.
    Optional = 1,
}

impl Default for PolicyPriorityLevel {
    /// @id strongfather_policy_priority_default
    /// @role constructor
    /// @layer core
    /// @do provide_default_priority
    //@human / @human Priorité normale par défaut (RÈGLE-PRIO-4).
    fn default() -> Self {
        Self::Normal
    }
}

/// @id strongfather_policy_id
/// @role domain_model
/// @layer core
/// @do identify_policy
//@human / @human Identifiant unique d'une politique (RÈGLE-STRUCT-1 : unicité).
/// Identifiant de politique
pub type PolicyId = String;

/// @id strongfather_policy_condition
/// @role domain_model
/// @layer core
/// @do represent_policy_condition
//@human / @human Condition d'application d'une politique. Détermine quand la politique s'applique.
/// Condition d'application d'une politique
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyCondition {
    /// @id strongfather_policy_condition_expression
    /// @role domain_model
    /// @layer core
    /// @do express_condition
    //@human / @human Expression déclarative de la condition (représentation conceptuelle).
    pub expression: String,
}

/// @id strongfather_policy_rule
/// @role domain_model
/// @layer core
/// @do represent_policy_rule
//@human / @human Règle déclarative d'une politique. Exprime ce qui est autorisé, interdit ou requis.
/// Règle déclarative d'une politique
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyRule {
    /// @id strongfather_policy_rule_expression
    /// @role domain_model
    /// @layer core
    /// @do express_rule
    //@human / @human Expression déclarative de la règle (représentation conceptuelle).
    pub expression: String,
}

/// @id strongfather_policy_target
/// @role domain_model
/// @layer core
/// @do represent_policy_target
//@human / @human Cible structurée d'une politique : sélection par égalité exacte de champs de l'intention.
//@human Jamais de sous-chaîne : une règle pour `jaykonta` ne vise pas `jaykonta-evil`.
/// Cible structurée d'une politique.
///
/// Chaque champ vaut `None` (« tout ») ou `Some(liste)` (égalité exacte avec l'un des éléments ;
/// une liste vide ne correspond à rien). La politique n'est examinée que si **tous** les champs
/// correspondent.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PolicyTarget {
    /// Types d'action visés.
    pub actions: Option<Vec<ActionType>>,
    /// Sujets visés (`Intent::subject`, égalité exacte).
    pub subjects: Option<Vec<String>>,
    /// Appelants visés (`CallContext::caller_identity`, égalité exacte).
    pub callers: Option<Vec<String>>,
    /// Origines visées (`CallContext::origin`, égalité exacte).
    pub origins: Option<Vec<String>>,
    /// Zones visées (métadonnée `zone` de l'intention ; absente ⇒ ne correspond pas).
    pub zones: Option<Vec<String>>,
}

impl PolicyTarget {
    /// @id strongfather_policy_target_any
    /// @role constructor
    /// @layer core
    /// @do create_any_target
    //@human / @human Cible universelle (toutes les intentions) ; la condition et la règle restreignent ensuite.
    #[must_use]
    pub fn any() -> Self {
        Self::default()
    }

    /// @id strongfather_policy_target_matches
    /// @role query
    /// @layer core
    /// @do match_target_exactly
    //@human / @human Vrai si l'intention correspond exactement à tous les champs renseignés de la cible.
    #[must_use]
    pub fn matches(&self, intent: &Intent) -> bool {
        fn dans(liste: Option<&Vec<String>>, valeur: Option<&str>) -> bool {
            match liste {
                None => true,
                Some(l) => valeur.is_some_and(|v| l.iter().any(|x| x == v)),
            }
        }
        let action_ok = self
            .actions
            .as_ref()
            .is_none_or(|l| l.contains(&intent.action_type));
        action_ok
            && dans(self.subjects.as_ref(), Some(intent.subject.as_str()))
            && dans(
                self.callers.as_ref(),
                Some(intent.call_context.caller_identity.as_str()),
            )
            && dans(
                self.origins.as_ref(),
                Some(intent.call_context.origin.as_str()),
            )
            && dans(
                self.zones.as_ref(),
                intent.metadata.get(ZONE_METADATA_KEY).map(String::as_str),
            )
    }
}

/// @id strongfather_policy
/// @role domain_model
/// @layer core
/// @do represent_policy
//@human / @human Politique StrongFather. Règle déclarative et explicite pour évaluer des intentions.
/// Politique StrongFather
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    /// @id strongfather_policy_id_id_2
    /// @role domain_model
    /// @layer core
    /// @do identify_policy
    //@human / @human Identifiant unique de la politique (RÈGLE-STRUCT-1).
    pub id: PolicyId,
    /// @id strongfather_policy_type_policy_type_2
    /// @role domain_model
    /// @layer core
    /// @do specify_policy_type
    //@human / @human Type de politique (RÈGLE-STRUCT-2 : type obligatoire).
    pub policy_type: PolicyType,
    /// @id strongfather_policy_condition_condition_2
    /// @role domain_model
    /// @layer core
    /// @do specify_application_condition
    //@human / @human Condition d'application (RÈGLE-STRUCT-3 : condition obligatoire).
    pub condition: PolicyCondition,
    /// @id strongfather_policy_rule_rule_2
    /// @role domain_model
    /// @layer core
    /// @do specify_declarative_rule
    //@human / @human Règle déclarative (RÈGLE-STRUCT-4 : règle obligatoire).
    pub rule: PolicyRule,
    /// @id strongfather_policy_effect_effect_2
    /// @role domain_model
    /// @layer core
    /// @do specify_policy_effect
    //@human / @human Effet de la politique (RÈGLE-STRUCT-5 : effet obligatoire).
    pub effect: PolicyEffect,
    /// @id strongfather_policy_priority
    /// @role domain_model
    /// @layer core
    /// @do specify_priority_level
    //@human / @human Priorité relative de la politique (optionnelle, défaut: Normal).
    pub priority: PolicyPriorityLevel,
    /// @id strongfather_policy_metadata
    /// @role domain_model
    /// @layer core
    /// @do provide_metadata
    //@human / @human Métadonnées descriptives (optionnelles).
    pub metadata: HashMap<String, String>,
    /// @id strongfather_policy_composite_policies
    /// @role domain_model
    /// @layer core
    /// @do store_composite_policies
    //@human / @human Politiques élémentaires pour une politique composite (optionnel).
    pub composite_policies: Option<Vec<PolicyId>>,
    /// @id strongfather_policy_target_field
    /// @role domain_model
    /// @layer core
    /// @do specify_policy_target
    //@human / @human Cible structurée (sélection exacte). Défaut : toutes les intentions.
    pub target: PolicyTarget,
}

impl Policy {
    /// @id strongfather_policy_new
    /// @role constructor
    /// @layer core
    /// @do create_policy
    //@human / @human Crée une nouvelle politique avec les composants obligatoires.
    /// @depends strongfather_policy
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
            target: PolicyTarget::default(),
        }
    }

    /// @id strongfather_policy_with_target
    /// @role builder
    /// @layer core
    /// @do set_policy_target
    //@human / @human Définit la cible structurée de la politique.
    /// @depends strongfather_policy_new
    #[must_use]
    pub fn with_target(mut self, target: PolicyTarget) -> Self {
        self.target = target;
        self
    }

    /// @id strongfather_policy_check_coherence
    /// @role query
    /// @layer core
    /// @do check_type_effect_coherence
    //@human / @human Vérifie la cohérence type/effet. Une politique incohérente est refusée au chargement
    //@human et, si elle est visée par une intention, provoque un refus (fail-closed).
    /// Cohérence type ↔ effet :
    /// - `Permission` → `Authorize` | `Deny`
    /// - `Constraint` → `Constrain` | `Deny`
    /// - `Validation` → `Validate` | `Deny`
    /// - `Priority` → `Prioritize`
    /// - `Composite` → `Authorize` | `Deny` | `Constrain` | `Validate`, avec au moins une
    ///   politique élémentaire
    pub fn check_coherence(&self) -> Result<(), String> {
        use PolicyEffect as E;
        use PolicyType as T;
        let ok = matches!(
            (self.policy_type, self.effect),
            (T::Permission, E::Authorize | E::Deny)
                | (T::Constraint, E::Constrain | E::Deny)
                | (T::Validation, E::Validate | E::Deny)
                | (T::Priority, E::Prioritize)
                | (
                    T::Composite,
                    E::Authorize | E::Deny | E::Constrain | E::Validate
                )
        );
        if !ok {
            return Err(format!(
                "politique '{}' : effet {:?} incohérent avec le type {:?}",
                self.id, self.effect, self.policy_type
            ));
        }
        if self.policy_type == T::Composite
            && self.composite_policies.as_ref().is_none_or(Vec::is_empty)
        {
            return Err(format!(
                "politique composite '{}' sans politique élémentaire",
                self.id
            ));
        }
        if self.policy_type != T::Composite && self.composite_policies.is_some() {
            return Err(format!(
                "politique '{}' : composite_policies réservé au type Composite",
                self.id
            ));
        }
        Ok(())
    }

    /// @id strongfather_policy_with_priority
    /// @role builder
    /// @layer core
    /// @do set_policy_priority
    //@human / @human Définit la priorité de la politique.
    /// @depends strongfather_policy_new
    #[must_use]
    pub fn with_priority(mut self, priority: PolicyPriorityLevel) -> Self {
        self.priority = priority;
        self
    }

    /// @id strongfather_policy_with_metadata
    /// @role builder
    /// @layer core
    /// @do add_policy_metadata
    //@human / @human Ajoute des métadonnées à la politique.
    /// @depends strongfather_policy_new
    #[must_use]
    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = metadata;
        self
    }

    /// @id strongfather_policy_with_composite
    /// @role builder
    /// @layer core
    /// @do set_composite_policies
    //@human / @human Définit les politiques élémentaires pour une politique composite.
    /// @depends strongfather_policy_new
    #[must_use]
    pub fn with_composite_policies(mut self, policies: Vec<PolicyId>) -> Self {
        self.composite_policies = Some(policies);
        self
    }

    /// @id strongfather_policy_is_critical
    /// @role query
    /// @layer core
    /// @do check_if_critical
    //@human / @human Vérifie si la politique est critique (RÈGLE-PRIO-2 : arrêt sur violation).
    /// @depends strongfather_policy
    #[must_use]
    pub fn is_critical(&self) -> bool {
        self.priority == PolicyPriorityLevel::Critical
    }
}

/// @id strongfather_policy_set
/// @role domain_model
/// @layer core
/// @do represent_policy_set
//@human / @human Ensemble de politiques chargées depuis une source unique.
/// Ensemble de politiques
#[derive(Debug, Clone)]
pub struct PolicySet {
    /// @id strongfather_policy_set_policies
    /// @role domain_model
    /// @layer core
    /// @do store_policies
    //@human / @human Collection de politiques indexée par identifiant.
    policies: HashMap<PolicyId, Policy>,
}

impl PolicySet {
    /// @id strongfather_policy_set_new
    /// @role constructor
    /// @layer core
    /// @do create_policy_set
    //@human / @human Crée un nouvel ensemble de politiques vide.
    /// @depends strongfather_policy_set
    #[must_use]
    pub fn new() -> Self {
        Self {
            policies: HashMap::new(),
        }
    }

    /// @id strongfather_policy_set_add
    /// @role mutator
    /// @layer core
    /// @do add_policy_to_set
    //@human / @human Ajoute une politique à l'ensemble (VALID-STRUCT-1 : identifiant unique).
    /// @depends strongfather_policy_set_new
    pub fn add(&mut self, policy: Policy) -> Result<(), String> {
        if self.policies.contains_key(&policy.id) {
            return Err(format!("Policy with id '{}' already exists", policy.id));
        }
        self.policies.insert(policy.id.clone(), policy);
        Ok(())
    }

    /// @id strongfather_policy_set_get
    /// @role query
    /// @layer core
    /// @do get_policy_by_id
    //@human / @human Récupère une politique par son identifiant.
    /// @depends strongfather_policy_set_new
    #[must_use]
    pub fn get(&self, id: &PolicyId) -> Option<&Policy> {
        self.policies.get(id)
    }

    /// @id strongfather_policy_set_iter
    /// @role query
    /// @layer core
    /// @do iterate_policies
    //@human / @human Itère sur toutes les politiques de l'ensemble.
    /// @depends strongfather_policy_set_new
    pub fn iter(&self) -> impl Iterator<Item = &Policy> {
        self.policies.values()
    }

    /// @id strongfather_policy_set_len
    /// @role query
    /// @layer core
    /// @do count_policies
    //@human / @human Retourne le nombre de politiques dans l'ensemble.
    /// @depends strongfather_policy_set_new
    #[must_use]
    pub fn len(&self) -> usize {
        self.policies.len()
    }

    /// @id strongfather_policy_set_is_empty
    /// @role query
    /// @layer core
    /// @do check_if_empty
    //@human / @human Vérifie si l'ensemble est vide.
    /// @depends strongfather_policy_set_new
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.policies.is_empty()
    }
}

impl Default for PolicySet {
    /// @id strongfather_policy_set_default
    /// @role constructor
    /// @layer core
    /// @do create_default_policy_set
    //@human / @human Crée un ensemble de politiques vide par défaut.
    fn default() -> Self {
        Self::new()
    }
}

/// @id strongfather_policy_result
/// @role domain_model
/// @layer core
/// @do represent_policy_evaluation_result
//@human / @human Résultat d'évaluation d'une politique. Indique l'effet de la politique sur l'intention.
/// Résultat d'évaluation d'une politique
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyResult {
    /// @id strongfather_policy_result_satisfied
    /// @role domain_model
    /// @layer core
    /// @do represent_satisfied_policy
    //@human / @human Politique satisfaite - l'intention respecte la politique.
    Satisfied {
        /// @id strongfather_policy_result_satisfied_policy_id
        /// @role domain_model
        /// @layer core
        /// @do identify_satisfied_policy
        //@human / @human Identifiant de la politique satisfaite.
        policy_id: PolicyId,
    },
    /// @id strongfather_policy_result_violated
    /// @role domain_model
    /// @layer core
    /// @do represent_violated_policy
    //@human / @human Politique violée - l'intention viole la politique.
    Violated {
        /// @id strongfather_policy_result_violated_policy_id
        /// @role domain_model
        /// @layer core
        /// @do identify_violated_policy
        //@human / @human Identifiant de la politique violée.
        policy_id: PolicyId,
        /// @id strongfather_policy_result_violated_reason
        /// @role domain_model
        /// @layer core
        /// @do explain_violation
        //@human / @human Raison de la violation.
        reason: String,
    },
    /// @id strongfather_policy_result_priority
    /// @role domain_model
    /// @layer core
    /// @do represent_priority_policy_result
    //@human / @human Politique de priorité - établit une priorité pour l'intention.
    Priority {
        /// @id strongfather_policy_result_priority_policy_id
        /// @role domain_model
        /// @layer core
        /// @do identify_priority_policy
        //@human / @human Identifiant de la politique de priorité.
        policy_id: PolicyId,
        /// @id strongfather_policy_result_priority_value
        /// @role domain_model
        /// @layer core
        /// @do store_priority_value
        //@human / @human Valeur de priorité établie.
        value: u32,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id strongfather_policy_test_create_basic
    /// @role test
    /// @layer core
    /// @do test_basic_policy_creation
    //@human / @human Test de création d'une politique basique avec les composants obligatoires.
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

    /// @id strongfather_policy_test_create_with_priority
    /// @role test
    /// @layer core
    /// @do test_policy_with_priority
    //@human / @human Test de création d'une politique avec priorité critique.
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

    /// @id strongfather_policy_test_policy_set
    /// @role test
    /// @layer core
    /// @do test_policy_set_operations
    //@human / @human Test de gestion d'un ensemble de politiques.
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

    /// @id strongfather_policy_test_all_types
    /// @role test
    /// @layer core
    /// @do test_all_policy_types
    //@human / @human Test que tous les types de politiques sont bien définis.
    #[test]
    fn test_all_policy_types() {
        let types = [
            PolicyType::Permission,
            PolicyType::Constraint,
            PolicyType::Priority,
            PolicyType::Validation,
            PolicyType::Composite,
        ];

        for (idx, policy_type) in types.iter().enumerate() {
            let policy = Policy::new(
                format!("POL-{idx}"),
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
