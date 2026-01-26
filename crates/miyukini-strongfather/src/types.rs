//! Types de base pour StrongFather
//!
//! Ce module définit tous les types fondamentaux utilisés par StrongFather :
//! - Types d'action (ActionType)
//! - Types de décision (DecisionType)
//! - Types de politique (PolicyType, PolicyEffect)
//! - Types auxiliaires (Priority, Justification, etc.)
//!
//! Conformité : Intent Model Contract, Core Decision Contract, Policy Engine Contract

/// Type d'action d'une intention
///
/// Représente la catégorie conceptuelle de l'action que l'appelant souhaite évaluer.
/// Conforme à Intent Model Contract section 3.2.
///
/// Invariant : INV-INT-2 (type obligatoire)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActionType {
    /// Intention de créer une nouvelle entité ou un nouveau fait
    Creation,
    /// Intention de modifier une entité ou un fait existant
    Modification,
    /// Intention de supprimer une entité ou un fait existant
    Deletion,
    /// Intention de lire une entité ou un fait (évaluation d'accès)
    Read,
    /// Intention d'évaluer une condition ou un état sans action
    Evaluation,
}

impl ActionType {
    /// Retourne une représentation textuelle du type d'action
    pub fn as_str(&self) -> &'static str {
        match self {
            ActionType::Creation => "Creation",
            ActionType::Modification => "Modification",
            ActionType::Deletion => "Deletion",
            ActionType::Read => "Read",
            ActionType::Evaluation => "Evaluation",
        }
    }
}

/// Type de décision produite par StrongFather
///
/// Représente le résultat de l'évaluation d'une intention.
/// Conforme à Core Decision Contract section 3.
///
/// Invariant : INV-DEC-1 (décisions non ambiguës)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionType {
    /// Décision acceptée avec priorité établie
    Accepted {
        /// Priorité calculée pour cette intention
        priority: Priority,
    },
    /// Décision refusée avec raison explicite
    Refused {
        /// Raison du refus
        reason: RefusalReason,
        /// Identifiants des politiques violées
        violated_policies: Vec<PolicyId>,
    },
    /// Décision ambiguë nécessitant des clarifications
    Ambiguous {
        /// Informations manquantes identifiées
        missing_information: Vec<String>,
        /// Clarifications requises
        clarifications_required: Vec<Clarification>,
    },
    /// Décision différée (contexte futur requis)
    ///
    /// Note : Conforme à INV-DIFF-NOPLAN, cette décision n'implique aucune planification.
    /// Seul l'adaptateur décide quand re-soumettre l'intention.
    Deferred {
        /// Raison du différé
        reason: DeferralReason,
        /// Contexte futur requis pour l'évaluation
        context_required: Vec<String>,
    },
}

/// Type de politique
///
/// Représente la catégorie d'une politique StrongFather.
/// Conforme à Policy Engine Contract section 3.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PolicyType {
    /// Politique de permission (autorise/interdit selon l'acteur)
    Permission,
    /// Politique de contrainte (condition obligatoire indépendante de l'acteur)
    Constraint,
    /// Politique de priorité (détermine l'ordre d'importance relative)
    Priority,
    /// Politique de validation (valide la cohérence ou la conformité)
    Validation,
    /// Politique composite (combinaison de plusieurs politiques)
    Composite {
        /// Opérateur logique de combinaison
        operator: LogicalOperator,
        /// Identifiants des politiques composantes
        policies: Vec<PolicyId>,
    },
}

impl PolicyType {
    /// Retourne une représentation textuelle du type de politique
    pub fn as_str(&self) -> &'static str {
        match self {
            PolicyType::Permission => "Permission",
            PolicyType::Constraint => "Constraint",
            PolicyType::Priority => "Priority",
            PolicyType::Validation => "Validation",
            PolicyType::Composite { .. } => "Composite",
        }
    }
}

/// Opérateur logique pour les politiques composites
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LogicalOperator {
    /// ET logique (toutes les politiques doivent être satisfaites)
    And,
    /// OU logique (au moins une politique doit être satisfaite)
    Or,
    /// NON logique (la politique ne doit pas être satisfaite)
    Not,
}

/// Effet d'une politique
///
/// Représente l'effet qu'une politique produit lorsqu'elle est appliquée.
/// Conforme à Policy Engine Contract section 4.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyEffect {
    /// Autorise l'intention
    Allow,
    /// Interdit l'intention
    Deny,
    /// Requiert une condition
    Require,
    /// Valide une condition
    Validate,
    /// Priorise l'intention avec un niveau donné
    Prioritize {
        /// Niveau de priorité (plus élevé = plus prioritaire)
        level: u8,
    },
}

/// Priorité d'une intention
///
/// Représente l'ordre d'importance relative d'une intention.
/// Conforme à Policy Engine Contract section 3.3.
///
/// Note : La priorité est relative et conceptuelle, pas temporelle (INV-AUTH-3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Priority {
    /// Valeur de priorité (plus élevée = plus prioritaire)
    value: u8,
}

impl Priority {
    /// Crée une nouvelle priorité avec la valeur donnée
    ///
    /// # Arguments
    /// * `value` - Valeur de priorité (0-255, où 255 est la priorité maximale)
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    /// Retourne la valeur de priorité
    pub fn value(&self) -> u8 {
        self.value
    }

    /// Priorité minimale (0)
    pub fn min() -> Self {
        Self { value: 0 }
    }

    /// Priorité maximale (255)
    pub fn max() -> Self {
        Self { value: 255 }
    }
}

/// Priorité d'une politique
///
/// Utilisée pour la résolution de conflits entre politiques.
/// Plus la valeur est élevée, plus la politique est prioritaire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PolicyPriority {
    /// Valeur de priorité (plus élevée = plus prioritaire)
    value: u8,
}

impl PolicyPriority {
    /// Crée une nouvelle priorité de politique avec la valeur donnée
    ///
    /// # Arguments
    /// * `value` - Valeur de priorité (0-255, où 255 est la priorité maximale)
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    /// Retourne la valeur de priorité
    pub fn value(&self) -> u8 {
        self.value
    }

    /// Priorité minimale (0)
    pub fn min() -> Self {
        Self { value: 0 }
    }

    /// Priorité maximale (255)
    pub fn max() -> Self {
        Self { value: 255 }
    }
}

/// Identifiant d'une politique
///
/// Type alias pour garantir la cohérence des identifiants de politiques.
pub type PolicyId = String;

/// Raison d'un refus
///
/// Représente la raison explicite pour laquelle une intention a été refusée.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefusalReason {
    /// Refus structurel (intention invalide structurellement)
    Structural {
        /// Description de l'erreur structurelle
        description: String,
    },
    /// Refus par violation de politique
    PolicyViolation {
        /// Description de la violation
        description: String,
    },
    /// Refus par contrainte non satisfaite
    ConstraintNotSatisfied {
        /// Description de la contrainte non satisfaite
        description: String,
    },
    /// Refus par manque d'autorisation
    Unauthorized {
        /// Description du manque d'autorisation
        description: String,
    },
    /// Refus pour une raison personnalisée
    Custom {
        /// Description personnalisée
        description: String,
    },
}

/// Raison d'un différé
///
/// Représente la raison pour laquelle une décision a été différée.
/// Conforme à INV-DIFF-NOPLAN (pas de planification).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeferralReason {
    /// Contexte futur requis
    ContextRequired {
        /// Description du contexte requis
        description: String,
    },
    /// Informations manquantes
    InformationMissing {
        /// Description des informations manquantes
        description: String,
    },
    /// Condition non encore satisfaite
    ConditionNotMet {
        /// Description de la condition
        description: String,
    },
    /// Différé pour une raison personnalisée
    Custom {
        /// Description personnalisée
        description: String,
    },
}

/// Clarification requise
///
/// Représente une clarification nécessaire pour résoudre une ambiguïté.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clarification {
    /// Question ou demande de clarification
    pub question: String,
    /// Information manquante identifiée
    pub missing_information: String,
    /// Contexte de la clarification
    pub context: Option<String>,
}

impl Clarification {
    /// Crée une nouvelle clarification
    pub fn new(question: String, missing_information: String) -> Self {
        Self {
            question,
            missing_information,
            context: None,
        }
    }

    /// Crée une nouvelle clarification avec contexte
    pub fn with_context(question: String, missing_information: String, context: String) -> Self {
        Self {
            question,
            missing_information,
            context: Some(context),
        }
    }
}

/// Justification d'une décision
///
/// Représente la justification explicite d'une décision.
/// Conforme à Core Decision Contract section 4.3 (G-JUST-1, G-JUST-2, G-JUST-3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Justification {
    /// Explication conceptuelle de la décision
    pub explanation: String,
    /// Références aux politiques appliquées
    pub policy_references: Vec<PolicyId>,
    /// Étapes de raisonnement
    pub reasoning_steps: Vec<ReasoningStep>,
}

impl Justification {
    /// Crée une nouvelle justification
    pub fn new(explanation: String) -> Self {
        Self {
            explanation,
            policy_references: Vec::new(),
            reasoning_steps: Vec::new(),
        }
    }

    /// Crée une nouvelle justification avec références aux politiques
    pub fn with_policies(explanation: String, policy_references: Vec<PolicyId>) -> Self {
        Self {
            explanation,
            policy_references,
            reasoning_steps: Vec::new(),
        }
    }
}

/// Étape de raisonnement
///
/// Représente une étape dans le processus de raisonnement qui a mené à une décision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReasoningStep {
    /// Description de l'étape
    pub description: String,
    /// Politiques appliquées à cette étape
    pub policies_applied: Vec<PolicyId>,
    /// Résultat de cette étape
    pub result: String,
}

impl ReasoningStep {
    /// Crée une nouvelle étape de raisonnement
    pub fn new(description: String, result: String) -> Self {
        Self {
            description,
            policies_applied: Vec::new(),
            result,
        }
    }

    /// Crée une nouvelle étape de raisonnement avec politiques
    pub fn with_policies(
        description: String,
        result: String,
        policies_applied: Vec<PolicyId>,
    ) -> Self {
        Self {
            description,
            policies_applied,
            result,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_type_as_str() {
        assert_eq!(ActionType::Creation.as_str(), "Creation");
        assert_eq!(ActionType::Modification.as_str(), "Modification");
        assert_eq!(ActionType::Deletion.as_str(), "Deletion");
        assert_eq!(ActionType::Read.as_str(), "Read");
        assert_eq!(ActionType::Evaluation.as_str(), "Evaluation");
    }

    #[test]
    fn test_action_type_equality() {
        assert_eq!(ActionType::Creation, ActionType::Creation);
        assert_ne!(ActionType::Creation, ActionType::Modification);
    }

    #[test]
    fn test_decision_type_accepted() {
        let priority = Priority::new(100);
        let decision = DecisionType::Accepted { priority };
        match decision {
            DecisionType::Accepted { priority: p } => assert_eq!(p.value(), 100),
            _ => panic!("Expected Accepted decision"),
        }
    }

    #[test]
    fn test_decision_type_refused() {
        let reason = RefusalReason::PolicyViolation {
            description: "Test violation".to_string(),
        };
        let policies = vec!["policy1".to_string(), "policy2".to_string()];
        let decision = DecisionType::Refused {
            reason: reason.clone(),
            violated_policies: policies.clone(),
        };
        match decision {
            DecisionType::Refused {
                reason: r,
                violated_policies: p,
            } => {
                assert_eq!(r, reason);
                assert_eq!(p, policies);
            }
            _ => panic!("Expected Refused decision"),
        }
    }

    #[test]
    fn test_decision_type_ambiguous() {
        let missing = vec!["info1".to_string(), "info2".to_string()];
        let clarifications = vec![Clarification::new(
            "Question?".to_string(),
            "Missing info".to_string(),
        )];
        let decision = DecisionType::Ambiguous {
            missing_information: missing.clone(),
            clarifications_required: clarifications.clone(),
        };
        match decision {
            DecisionType::Ambiguous {
                missing_information: m,
                clarifications_required: c,
            } => {
                assert_eq!(m, missing);
                assert_eq!(c, clarifications);
            }
            _ => panic!("Expected Ambiguous decision"),
        }
    }

    #[test]
    fn test_decision_type_deferred() {
        let reason = DeferralReason::ContextRequired {
            description: "Context needed".to_string(),
        };
        let context = vec!["context1".to_string()];
        let decision = DecisionType::Deferred {
            reason: reason.clone(),
            context_required: context.clone(),
        };
        match decision {
            DecisionType::Deferred {
                reason: r,
                context_required: c,
            } => {
                assert_eq!(r, reason);
                assert_eq!(c, context);
            }
            _ => panic!("Expected Deferred decision"),
        }
    }

    #[test]
    fn test_policy_type_as_str() {
        assert_eq!(PolicyType::Permission.as_str(), "Permission");
        assert_eq!(PolicyType::Constraint.as_str(), "Constraint");
        assert_eq!(PolicyType::Priority.as_str(), "Priority");
        assert_eq!(PolicyType::Validation.as_str(), "Validation");
        assert_eq!(
            PolicyType::Composite {
                operator: LogicalOperator::And,
                policies: vec![]
            }
            .as_str(),
            "Composite"
        );
    }

    #[test]
    fn test_policy_type_composite() {
        let policies = vec!["policy1".to_string(), "policy2".to_string()];
        let composite = PolicyType::Composite {
            operator: LogicalOperator::And,
            policies: policies.clone(),
        };
        match composite {
            PolicyType::Composite { operator, policies: p } => {
                assert_eq!(operator, LogicalOperator::And);
                assert_eq!(p, policies);
            }
            _ => panic!("Expected Composite policy type"),
        }
    }

    #[test]
    fn test_policy_effect() {
        assert_eq!(PolicyEffect::Allow, PolicyEffect::Allow);
        assert_eq!(PolicyEffect::Deny, PolicyEffect::Deny);
        match (PolicyEffect::Prioritize { level: 100 }) {
            PolicyEffect::Prioritize { level } => assert_eq!(level, 100),
            _ => panic!("Expected Prioritize effect"),
        }
    }

    #[test]
    fn test_priority() {
        let p1 = Priority::new(50);
        let p2 = Priority::new(100);
        let p3 = Priority::new(50);

        assert_eq!(p1.value(), 50);
        assert_eq!(p2.value(), 100);
        assert_eq!(p1, p3);
        assert!(p2 > p1);
        assert_eq!(Priority::min().value(), 0);
        assert_eq!(Priority::max().value(), 255);
    }

    #[test]
    fn test_policy_priority() {
        let pp1 = PolicyPriority::new(50);
        let pp2 = PolicyPriority::new(100);

        assert_eq!(pp1.value(), 50);
        assert_eq!(pp2.value(), 100);
        assert!(pp2 > pp1);
        assert_eq!(PolicyPriority::min().value(), 0);
        assert_eq!(PolicyPriority::max().value(), 255);
    }

    #[test]
    fn test_refusal_reason() {
        let reason = RefusalReason::PolicyViolation {
            description: "Test".to_string(),
        };
        match reason {
            RefusalReason::PolicyViolation { description } => {
                assert_eq!(description, "Test");
            }
            _ => panic!("Expected PolicyViolation reason"),
        }
    }

    #[test]
    fn test_deferral_reason() {
        let reason = DeferralReason::ContextRequired {
            description: "Test".to_string(),
        };
        match reason {
            DeferralReason::ContextRequired { description } => {
                assert_eq!(description, "Test");
            }
            _ => panic!("Expected ContextRequired reason"),
        }
    }

    #[test]
    fn test_clarification() {
        let clar = Clarification::new("Question?".to_string(), "Missing info".to_string());
        assert_eq!(clar.question, "Question?");
        assert_eq!(clar.missing_information, "Missing info");
        assert_eq!(clar.context, None);

        let clar_with_ctx = Clarification::with_context(
            "Question?".to_string(),
            "Missing info".to_string(),
            "Context".to_string(),
        );
        assert_eq!(clar_with_ctx.context, Some("Context".to_string()));
    }

    #[test]
    fn test_justification() {
        let just = Justification::new("Explanation".to_string());
        assert_eq!(just.explanation, "Explanation");
        assert!(just.policy_references.is_empty());
        assert!(just.reasoning_steps.is_empty());

        let policies = vec!["policy1".to_string()];
        let just_with_policies =
            Justification::with_policies("Explanation".to_string(), policies.clone());
        assert_eq!(just_with_policies.policy_references, policies);
    }

    #[test]
    fn test_reasoning_step() {
        let step = ReasoningStep::new("Step 1".to_string(), "Result".to_string());
        assert_eq!(step.description, "Step 1");
        assert_eq!(step.result, "Result");
        assert!(step.policies_applied.is_empty());

        let policies = vec!["policy1".to_string()];
        let step_with_policies = ReasoningStep::with_policies(
            "Step 1".to_string(),
            "Result".to_string(),
            policies.clone(),
        );
        assert_eq!(step_with_policies.policies_applied, policies);
    }

    #[test]
    fn test_logical_operator() {
        assert_eq!(LogicalOperator::And, LogicalOperator::And);
        assert_ne!(LogicalOperator::And, LogicalOperator::Or);
    }
}
