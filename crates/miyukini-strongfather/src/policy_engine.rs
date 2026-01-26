//! Moteur de politiques pour StrongFather
//!
//! Ce module implémente le moteur de politiques qui applique les politiques
//! à une intention pour produire des résultats d'évaluation.
//! Conforme au contrat Policy Engine Contract (FONDATION).
//!
//! **Invariants respectés :**
//! - INV-POL-2 : Politiques immutables pendant évaluation
//! - INV-POL-3 : Déterminisme d'évaluation
//!
//! **Garanties respectées :**
//! - G-POL-1 : Évaluation déterministe
//! - G-POL-2 : Évaluation complète
//! - G-POL-3 : Évaluation ordonnée
//! - G-POL-4 : Évaluation traçable
//! - G-POL-5 : Aucune exécution
//! - G-POL-6 : Aucune modification d'état
//! - G-POL-7 : Aucune persistance
//! - G-POL-8 : Cohérence des politiques
//! - G-POL-9 : Cohérence des décisions
//! - G-POL-10 : Justifiabilité
//! - G-POL-11 : Évaluation en zero-trust
//! - G-POL-12 : Vérification systématique

use crate::error::SFError;
use crate::intent::Intent;
use crate::policy::{Policy, PolicyCondition, PolicySet};
use crate::types::{PolicyEffect, PolicyId, PolicyPriority};
use crate::decision::EvaluationContext;

/// Résultat d'évaluation d'une politique
///
/// Représente le résultat de l'application d'une politique à une intention.
/// Conforme au contrat Policy Engine Contract section 6.
///
/// **Types de résultats :**
/// - `Satisfied` : La politique est satisfaite (la règle est respectée)
/// - `NotSatisfied` : La politique n'est pas satisfaite (la règle est violée)
/// - `Indeterminate` : La politique ne peut pas être évaluée de manière déterminée
///
/// **Caractéristiques :**
/// - Déterministe : Le résultat est toujours le même pour une même politique et intention
/// - Traçable : Le résultat référence la politique évaluée
/// - Justifiable : Le résultat peut être justifié selon la règle de la politique
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyResult {
    /// Identifiant de la politique évaluée
    ///
    /// Permet de tracer quelle politique a produit ce résultat.
    pub policy_id: PolicyId,
    /// Type de résultat d'évaluation
    ///
    /// Indique si la politique est satisfaite, non satisfaite, ou indéterminée.
    pub result: PolicyResultType,
    /// Effet de la politique
    ///
    /// L'effet que la politique produit (Allow, Deny, Require, Validate, Prioritize).
    pub effect: PolicyEffect,
    /// Priorité de la politique
    ///
    /// Utilisée pour la résolution de conflits.
    pub priority: PolicyPriority,
    /// Justification du résultat (optionnel)
    ///
    /// Explication de pourquoi la politique a produit ce résultat.
    pub justification: Option<String>,
}

impl PolicyResult {
    /// Crée un nouveau résultat de politique satisfaite
    pub fn satisfied(
        policy_id: PolicyId,
        effect: PolicyEffect,
        priority: PolicyPriority,
    ) -> Self {
        Self {
            policy_id,
            result: PolicyResultType::Satisfied,
            effect,
            priority,
            justification: None,
        }
    }

    /// Crée un nouveau résultat de politique satisfaite avec justification
    pub fn satisfied_with_justification(
        policy_id: PolicyId,
        effect: PolicyEffect,
        priority: PolicyPriority,
        justification: String,
    ) -> Self {
        Self {
            policy_id,
            result: PolicyResultType::Satisfied,
            effect,
            priority,
            justification: Some(justification),
        }
    }

    /// Crée un nouveau résultat de politique non satisfaite
    pub fn not_satisfied(
        policy_id: PolicyId,
        effect: PolicyEffect,
        priority: PolicyPriority,
    ) -> Self {
        Self {
            policy_id,
            result: PolicyResultType::NotSatisfied,
            effect,
            priority,
            justification: None,
        }
    }

    /// Crée un nouveau résultat de politique non satisfaite avec justification
    pub fn not_satisfied_with_justification(
        policy_id: PolicyId,
        effect: PolicyEffect,
        priority: PolicyPriority,
        justification: String,
    ) -> Self {
        Self {
            policy_id,
            result: PolicyResultType::NotSatisfied,
            effect,
            priority,
            justification: Some(justification),
        }
    }

    /// Crée un nouveau résultat de politique indéterminée
    pub fn indeterminate(
        policy_id: PolicyId,
        effect: PolicyEffect,
        priority: PolicyPriority,
    ) -> Self {
        Self {
            policy_id,
            result: PolicyResultType::Indeterminate,
            effect,
            priority,
            justification: None,
        }
    }

    /// Crée un nouveau résultat de politique indéterminée avec justification
    pub fn indeterminate_with_justification(
        policy_id: PolicyId,
        effect: PolicyEffect,
        priority: PolicyPriority,
        justification: String,
    ) -> Self {
        Self {
            policy_id,
            result: PolicyResultType::Indeterminate,
            effect,
            priority,
            justification: Some(justification),
        }
    }

    /// Vérifie si la politique est satisfaite
    pub fn is_satisfied(&self) -> bool {
        matches!(self.result, PolicyResultType::Satisfied)
    }

    /// Vérifie si la politique n'est pas satisfaite
    pub fn is_not_satisfied(&self) -> bool {
        matches!(self.result, PolicyResultType::NotSatisfied)
    }

    /// Vérifie si la politique est indéterminée
    pub fn is_indeterminate(&self) -> bool {
        matches!(self.result, PolicyResultType::Indeterminate)
    }
}

/// Type de résultat d'évaluation d'une politique
///
/// Représente le résultat conceptuel de l'application d'une politique.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyResultType {
    /// La politique est satisfaite (la règle est respectée)
    Satisfied,
    /// La politique n'est pas satisfaite (la règle est violée)
    NotSatisfied,
    /// La politique ne peut pas être évaluée de manière déterminée
    Indeterminate,
}

/// Moteur de politiques StrongFather
///
/// Applique les politiques à une intention pour produire des résultats d'évaluation.
/// Conforme au contrat Policy Engine Contract.
///
/// **Caractéristiques :**
/// - Déterministe : Pour une intention et des politiques données, le résultat est toujours le même (G-POL-1, INV-POL-3)
/// - Complet : Toutes les politiques applicables sont évaluées (G-POL-2)
/// - Ordonné : Les politiques sont évaluées dans l'ordre de priorité (G-POL-3)
/// - Traçable : Toute évaluation est traçable (G-POL-4)
/// - Zero-trust : Toute politique est évaluée sans présupposer la validité (G-POL-11, G-POL-12)
///
/// **Invariants respectés :**
/// - INV-POL-2 : Politiques immutables pendant évaluation (PolicySet est immuable)
/// - INV-POL-3 : Déterminisme d'évaluation (même intention + mêmes politiques = même résultat)
///
/// **Garanties respectées :**
/// - G-POL-1 à G-POL-12 : Toutes les garanties du Policy Engine Contract
#[derive(Debug, Clone)]
pub struct PolicyEngine {
    /// Ensemble de politiques à appliquer
    ///
    /// Immutable pendant l'évaluation (INV-POL-2).
    policies: PolicySet,
}

impl PolicyEngine {
    /// Crée un nouveau moteur de politiques avec un ensemble de politiques
    ///
    /// # Arguments
    /// * `policies` - Ensemble de politiques à appliquer
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::policy_engine::PolicyEngine;
    /// use miyukini_strongfather::policy::PolicySet;
    ///
    /// let policies = PolicySet::new();
    /// let engine = PolicyEngine::new(policies);
    /// ```
    pub fn new(policies: PolicySet) -> Self {
        Self { policies }
    }

    /// Applique les politiques à une intention
    ///
    /// Évalue toutes les politiques applicables à l'intention selon leur priorité
    /// et produit un résultat pour chaque politique évaluée.
    ///
    /// Conforme au contrat Policy Engine Contract section 5 et 6.
    ///
    /// **Processus d'évaluation :**
    /// 1. Filtre les politiques applicables selon leur condition (G-POL-2)
    /// 2. Trie les politiques par priorité décroissante (G-POL-3, RÈGLE-PRIO-1)
    /// 3. Évalue chaque politique dans l'ordre (G-POL-1, INV-POL-3)
    /// 4. Produit un résultat pour chaque politique (G-POL-4)
    ///
    /// **Résolution de conflits :**
    /// Les conflits sont résolus selon les règles RÈGLE-CONFLIT-1 à RÈGLE-CONFLIT-4.
    /// La résolution est effectuée par le ResultComposer (SF-STEP-10), pas ici.
    ///
    /// # Arguments
    /// * `intent` - Intention à évaluer
    /// * `context` - Contexte d'évaluation
    ///
    /// # Retour
    /// Résultat contenant un vecteur de résultats de politiques, ou une erreur si l'évaluation échoue.
    ///
    /// # Erreurs
    /// - `ResourceError` : Si les politiques ne sont pas disponibles
    /// - `ConsistencyError` : Si une incohérence est détectée dans l'évaluation
    ///
    /// # Exemple
    /// ```
    /// use miyukini_strongfather::policy_engine::PolicyEngine;
    /// use miyukini_strongfather::policy::{Policy, PolicyCondition, PolicyRule, PolicySet};
    /// use miyukini_strongfather::intent::{Intent, CallContext, IntentData};
    /// use miyukini_strongfather::decision::EvaluationContext;
    /// use miyukini_strongfather::types::{ActionType, PolicyEffect, PolicyPriority, PolicyType};
    ///
    /// // Créer une politique pour l'exemple
    /// let policy = Policy::new(
    ///     "POL-001",
    ///     PolicyType::Permission,
    ///     PolicyCondition::always(),
    ///     PolicyRule::new("always"),
    ///     PolicyEffect::Allow,
    ///     PolicyPriority::new(100),
    /// );
    /// let policies = PolicySet::from_vec(vec![policy]);
    /// let engine = PolicyEngine::new(policies);
    ///
    /// let call_context = CallContext::new(
    ///     "user123".to_string(),
    ///     "product".to_string(),
    ///     "instance1".to_string(),
    /// );
    /// let intent = Intent::new(
    ///     "intent-001".to_string(),
    ///     ActionType::Creation,
    ///     "document".to_string(),
    ///     call_context,
    ///     IntentData::empty(),
    /// );
    ///
    /// let eval_context = EvaluationContext::new(
    ///     "user123".to_string(),
    ///     "product".to_string(),
    ///     "instance1".to_string(),
    /// );
    ///
    /// let results = engine.apply(&intent, &eval_context).unwrap();
    /// ```
    pub fn apply(
        &self,
        intent: &Intent,
        context: &EvaluationContext,
    ) -> Result<Vec<PolicyResult>, SFError> {
        // Vérification que les politiques sont disponibles (G-POL-2)
        if self.policies.is_empty() {
            return Err(SFError::resource(
                "Aucune politique disponible pour l'évaluation",
                "PolicyEngine::apply",
            ));
        }

        // Étape 1 : Filtrer les politiques applicables selon leur condition (G-POL-2)
        let applicable_policies = self.filter_applicable_policies(intent, context)?;

        if applicable_policies.is_empty() {
            // Aucune politique applicable : retourner un résultat vide
            // (conforme à G-POL-2 : toutes les politiques applicables sont évaluées)
            return Ok(Vec::new());
        }

        // Étape 2 : Trier les politiques par priorité décroissante (G-POL-3, RÈGLE-PRIO-1)
        let mut sorted_policies = applicable_policies;
        sorted_policies.sort_by(|a, b| {
            // Tri décroissant : priorité élevée en premier
            b.priority().cmp(&a.priority())
        });

        // Étape 3 : Évaluer chaque politique dans l'ordre (G-POL-1, INV-POL-3)
        let mut results = Vec::new();
        for policy in sorted_policies {
            let result = self.evaluate_policy(&policy, intent, context)?;
            results.push(result);
        }

        // Étape 4 : Retourner les résultats (G-POL-4)
        Ok(results)
    }

    /// Filtre les politiques applicables selon leur condition
    ///
    /// Conforme à G-POL-2 (évaluation complète) : toutes les politiques applicables
    /// sont identifiées selon leur condition d'application.
    ///
    /// # Arguments
    /// * `intent` - Intention à évaluer
    /// * `context` - Contexte d'évaluation
    ///
    /// # Retour
    /// Vecteur de références aux politiques applicables
    fn filter_applicable_policies(
        &self,
        intent: &Intent,
        context: &EvaluationContext,
    ) -> Result<Vec<&Policy>, SFError> {
        let mut applicable = Vec::new();

        for policy in self.policies.iter() {
            if self.is_policy_applicable(policy, intent, context)? {
                applicable.push(policy);
            }
        }

        Ok(applicable)
    }

    /// Vérifie si une politique est applicable à une intention
    ///
    /// Évalue la condition d'application de la politique selon l'intention et le contexte.
    /// Conforme à G-POL-11, G-POL-12 (zero-trust, vérification systématique).
    ///
    /// # Arguments
    /// * `policy` - Politique à vérifier
    /// * `intent` - Intention à évaluer
    /// * `context` - Contexte d'évaluation
    ///
    /// # Retour
    /// true si la politique est applicable, false sinon, ou une erreur si l'évaluation échoue
    fn is_policy_applicable(
        &self,
        policy: &Policy,
        intent: &Intent,
        context: &EvaluationContext,
    ) -> Result<bool, SFError> {
        match policy.condition() {
            PolicyCondition::Always => {
                // La politique s'applique toujours
                Ok(true)
            }
            PolicyCondition::When { criteria } => {
                // La politique s'applique si les critères sont satisfaits
                // Note : L'évaluation réelle des critères sera effectuée ici
                // Pour l'instant, on fait une évaluation simplifiée basée sur le type d'action
                // et le sujet de l'intention
                self.evaluate_condition_criteria(criteria, intent, context)
            }
        }
    }

    /// Évalue les critères d'une condition "When"
    ///
    /// Évalue les critères déclaratifs d'une condition selon l'intention et le contexte.
    /// Conforme à G-POL-11, G-POL-12 (zero-trust, vérification systématique).
    ///
    /// # Arguments
    /// * `criteria` - Critères déclaratifs à évaluer
    /// * `intent` - Intention à évaluer
    /// * `context` - Contexte d'évaluation
    ///
    /// # Retour
    /// true si les critères sont satisfaits, false sinon, ou une erreur si l'évaluation échoue
    ///
    /// # Note
    /// Cette implémentation est simplifiée. Une implémentation complète devrait
    /// parser et évaluer les critères déclaratifs de manière plus sophistiquée.
    fn evaluate_condition_criteria(
        &self,
        criteria: &str,
        intent: &Intent,
        _context: &EvaluationContext,
    ) -> Result<bool, SFError> {
        let _ = _context; // Utilisé implicitement dans les futures améliorations
        let _ = intent; // Utilisé implicitement dans les futures améliorations
        // Évaluation simplifiée des critères
        // Pour une implémentation complète, il faudrait parser les critères
        // et les évaluer selon l'intention et le contexte

        // Exemples de critères simples :
        // - "action_type == Creation" : vérifie le type d'action
        // - "subject == 'document'" : vérifie le sujet

        // Pour l'instant, on fait une évaluation basique
        // Si les critères contiennent "action_type == Creation", on vérifie le type d'action
        if criteria.contains("action_type == Creation") {
            return Ok(intent.action_type() == crate::types::ActionType::Creation);
        }

        if criteria.contains("action_type == Modification") {
            return Ok(intent.action_type() == crate::types::ActionType::Modification);
        }

        if criteria.contains("action_type == Deletion") {
            return Ok(intent.action_type() == crate::types::ActionType::Deletion);
        }

        if criteria.contains("action_type == Read") {
            return Ok(intent.action_type() == crate::types::ActionType::Read);
        }

        if criteria.contains("action_type == Evaluation") {
            return Ok(intent.action_type() == crate::types::ActionType::Evaluation);
        }

        // Si les critères contiennent "subject == '...'", on vérifie le sujet
        if let Some(subject_match) = criteria.strip_prefix("subject == '") {
            if let Some(subject_end) = subject_match.strip_suffix("'") {
                return Ok(intent.subject() == subject_end);
            }
        }

        // Par défaut, si les critères ne sont pas reconnus, on considère que la politique
        // n'est pas applicable (zero-trust : on ne présuppose pas la validité)
        // Cela pourrait être amélioré avec un vrai parser de critères
        Ok(false)
    }

    /// Évalue une politique sur une intention
    ///
    /// Applique la règle déclarative de la politique à l'intention et produit un résultat.
    /// Conforme à G-POL-1 (évaluation déterministe), INV-POL-3 (déterminisme).
    ///
    /// # Arguments
    /// * `policy` - Politique à évaluer
    /// * `intent` - Intention à évaluer
    /// * `context` - Contexte d'évaluation
    ///
    /// # Retour
    /// Résultat d'évaluation de la politique, ou une erreur si l'évaluation échoue
    fn evaluate_policy(
        &self,
        policy: &Policy,
        intent: &Intent,
        context: &EvaluationContext,
    ) -> Result<PolicyResult, SFError> {
        // Évaluation de la règle déclarative de la politique
        // Conforme à G-POL-11, G-POL-12 (zero-trust, vérification systématique)
        let rule_satisfied = self.evaluate_policy_rule(policy, intent, context)?;

        // Production du résultat selon que la règle est satisfaite ou non
        let result = if rule_satisfied {
            PolicyResult::satisfied(
                policy.id().clone(),
                policy.effect().clone(),
                policy.priority(),
            )
        } else {
            // Si la règle n'est pas satisfaite, on vérifie l'effet de la politique
            match policy.effect() {
                PolicyEffect::Deny => {
                    // Une politique Deny non satisfaite signifie que l'intention est interdite
                    PolicyResult::not_satisfied(
                        policy.id().clone(),
                        policy.effect().clone(),
                        policy.priority(),
                    )
                }
                PolicyEffect::Allow => {
                    // Une politique Allow non satisfaite signifie que l'autorisation n'est pas accordée
                    PolicyResult::not_satisfied(
                        policy.id().clone(),
                        policy.effect().clone(),
                        policy.priority(),
                    )
                }
                PolicyEffect::Require => {
                    // Une politique Require non satisfaite signifie que la condition requise n'est pas remplie
                    PolicyResult::not_satisfied(
                        policy.id().clone(),
                        policy.effect().clone(),
                        policy.priority(),
                    )
                }
                PolicyEffect::Validate => {
                    // Une politique Validate non satisfaite signifie que la validation a échoué
                    PolicyResult::not_satisfied(
                        policy.id().clone(),
                        policy.effect().clone(),
                        policy.priority(),
                    )
                }
                PolicyEffect::Prioritize { .. } => {
                    // Une politique Prioritize non satisfaite signifie que la priorité n'est pas appliquée
                    PolicyResult::not_satisfied(
                        policy.id().clone(),
                        policy.effect().clone(),
                        policy.priority(),
                    )
                }
            }
        };

        Ok(result)
    }

    /// Évalue la règle déclarative d'une politique
    ///
    /// Évalue l'expression déclarative de la règle selon l'intention et le contexte.
    /// Conforme à G-POL-11, G-POL-12 (zero-trust, vérification systématique).
    ///
    /// # Arguments
    /// * `policy` - Politique dont la règle doit être évaluée
    /// * `intent` - Intention à évaluer
    /// * `context` - Contexte d'évaluation
    ///
    /// # Retour
    /// true si la règle est satisfaite, false sinon, ou une erreur si l'évaluation échoue
    ///
    /// # Note
    /// Cette implémentation est simplifiée. Une implémentation complète devrait
    /// parser et évaluer les règles déclaratives de manière plus sophistiquée.
    fn evaluate_policy_rule(
        &self,
        policy: &Policy,
        _intent: &Intent,
        context: &EvaluationContext,
    ) -> Result<bool, SFError> {
        let rule_expression = policy.rule().expression();

        // Évaluation simplifiée de la règle
        // Pour une implémentation complète, il faudrait parser l'expression
        // et l'évaluer selon l'intention et le contexte

        // Exemples de règles simples :
        // - "actor.role == 'admin'" : vérifie le rôle de l'acteur
        // - "resource.owner == actor.id" : vérifie la propriété de la ressource

        // Pour l'instant, on fait une évaluation basique
        // Si la règle contient "actor.role == 'admin'", on vérifie le contexte
        if rule_expression.contains("actor.role == 'admin'") {
            // Vérification simplifiée : on suppose que si le caller_identity contient "admin",
            // alors c'est un admin (à améliorer avec un vrai système de rôles)
            return Ok(context.caller_identity().contains("admin"));
        }

        // Si la règle contient "always", on considère qu'elle est toujours satisfaite
        if rule_expression.contains("always") {
            return Ok(true);
        }

        // Si la règle contient "never", on considère qu'elle n'est jamais satisfaite
        if rule_expression.contains("never") {
            return Ok(false);
        }

        // Par défaut, si la règle n'est pas reconnue, on considère qu'elle n'est pas satisfaite
        // (zero-trust : on ne présuppose pas la validité)
        // Cela pourrait être amélioré avec un vrai parser de règles
        Ok(false)
    }

    /// Retourne l'ensemble de politiques du moteur
    pub fn policies(&self) -> &PolicySet {
        &self.policies
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::{CallContext, IntentData};
    use crate::policy::{Policy, PolicyCondition, PolicyRule, PolicySet};
    use crate::types::{ActionType, PolicyEffect, PolicyPriority, PolicyType};

    fn create_test_intent() -> Intent {
        let call_context = CallContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        );
        Intent::new(
            "intent-001".to_string(),
            ActionType::Creation,
            "document".to_string(),
            call_context,
            IntentData::empty(),
        )
    }

    fn create_test_context() -> EvaluationContext {
        EvaluationContext::new(
            "user123".to_string(),
            "product".to_string(),
            "instance1".to_string(),
        )
    }

    #[test]
    fn test_policy_engine_new() {
        let policies = PolicySet::new();
        let engine = PolicyEngine::new(policies);
        assert!(engine.policies().is_empty());
    }

    #[test]
    fn test_policy_engine_apply_empty_policies() {
        let policies = PolicySet::new();
        let engine = PolicyEngine::new(policies);
        let intent = create_test_intent();
        let context = create_test_context();

        let result = engine.apply(&intent, &context);
        assert!(result.is_err());
        match result {
            Err(SFError::ResourceError { .. }) => {}
            _ => panic!("Expected ResourceError"),
        }
    }

    #[test]
    fn test_policy_engine_apply_with_always_policy() {
        let policy = Policy::new(
            "POL-001",
            PolicyType::Permission,
            PolicyCondition::always(),
            PolicyRule::new("always"),
            PolicyEffect::Allow,
            PolicyPriority::new(100),
        );
        let policies = PolicySet::from_vec(vec![policy]);
        let engine = PolicyEngine::new(policies);
        let intent = create_test_intent();
        let context = create_test_context();

        let results = engine.apply(&intent, &context).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].policy_id, "POL-001");
        assert!(results[0].is_satisfied());
    }

    #[test]
    fn test_policy_engine_apply_with_when_policy() {
        let policy = Policy::new(
            "POL-002",
            PolicyType::Permission,
            PolicyCondition::when("action_type == Creation"),
            PolicyRule::new("always"),
            PolicyEffect::Allow,
            PolicyPriority::new(100),
        );
        let policies = PolicySet::from_vec(vec![policy]);
        let engine = PolicyEngine::new(policies);
        let intent = create_test_intent();
        let context = create_test_context();

        let results = engine.apply(&intent, &context).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].policy_id, "POL-002");
        assert!(results[0].is_satisfied());
    }

    #[test]
    fn test_policy_engine_apply_with_non_applicable_policy() {
        let policy = Policy::new(
            "POL-003",
            PolicyType::Permission,
            PolicyCondition::when("action_type == Modification"),
            PolicyRule::new("always"),
            PolicyEffect::Allow,
            PolicyPriority::new(100),
        );
        let policies = PolicySet::from_vec(vec![policy]);
        let engine = PolicyEngine::new(policies);
        let intent = create_test_intent(); // ActionType::Creation
        let context = create_test_context();

        let results = engine.apply(&intent, &context).unwrap();
        // La politique n'est pas applicable car l'action est Creation, pas Modification
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_policy_engine_apply_with_multiple_policies() {
        let policy1 = Policy::new(
            "POL-001",
            PolicyType::Permission,
            PolicyCondition::always(),
            PolicyRule::new("always"),
            PolicyEffect::Allow,
            PolicyPriority::new(100),
        );
        let policy2 = Policy::new(
            "POL-002",
            PolicyType::Constraint,
            PolicyCondition::always(),
            PolicyRule::new("always"),
            PolicyEffect::Require,
            PolicyPriority::new(50),
        );
        let policies = PolicySet::from_vec(vec![policy1, policy2]);
        let engine = PolicyEngine::new(policies);
        let intent = create_test_intent();
        let context = create_test_context();

        let results = engine.apply(&intent, &context).unwrap();
        assert_eq!(results.len(), 2);
        // Les politiques doivent être triées par priorité décroissante
        assert_eq!(results[0].policy_id, "POL-001"); // Priorité 100
        assert_eq!(results[1].policy_id, "POL-002"); // Priorité 50
    }

    #[test]
    fn test_policy_engine_apply_with_priority_ordering() {
        let policy1 = Policy::new(
            "POL-001",
            PolicyType::Permission,
            PolicyCondition::always(),
            PolicyRule::new("always"),
            PolicyEffect::Allow,
            PolicyPriority::new(50), // Priorité plus faible
        );
        let policy2 = Policy::new(
            "POL-002",
            PolicyType::Constraint,
            PolicyCondition::always(),
            PolicyRule::new("always"),
            PolicyEffect::Require,
            PolicyPriority::new(100), // Priorité plus élevée
        );
        let policies = PolicySet::from_vec(vec![policy1, policy2]);
        let engine = PolicyEngine::new(policies);
        let intent = create_test_intent();
        let context = create_test_context();

        let results = engine.apply(&intent, &context).unwrap();
        assert_eq!(results.len(), 2);
        // Les politiques doivent être triées par priorité décroissante
        assert_eq!(results[0].policy_id, "POL-002"); // Priorité 100 en premier
        assert_eq!(results[1].policy_id, "POL-001"); // Priorité 50 en second
    }

    #[test]
    fn test_policy_result_satisfied() {
        let result = PolicyResult::satisfied(
            "POL-001".to_string(),
            PolicyEffect::Allow,
            PolicyPriority::new(100),
        );
        assert!(result.is_satisfied());
        assert!(!result.is_not_satisfied());
        assert!(!result.is_indeterminate());
        assert_eq!(result.policy_id, "POL-001");
    }

    #[test]
    fn test_policy_result_not_satisfied() {
        let result = PolicyResult::not_satisfied(
            "POL-002".to_string(),
            PolicyEffect::Deny,
            PolicyPriority::new(50),
        );
        assert!(!result.is_satisfied());
        assert!(result.is_not_satisfied());
        assert!(!result.is_indeterminate());
        assert_eq!(result.policy_id, "POL-002");
    }

    #[test]
    fn test_policy_result_indeterminate() {
        let result = PolicyResult::indeterminate(
            "POL-003".to_string(),
            PolicyEffect::Require,
            PolicyPriority::new(75),
        );
        assert!(!result.is_satisfied());
        assert!(!result.is_not_satisfied());
        assert!(result.is_indeterminate());
        assert_eq!(result.policy_id, "POL-003");
    }

    #[test]
    fn test_policy_result_with_justification() {
        let result = PolicyResult::satisfied_with_justification(
            "POL-001".to_string(),
            PolicyEffect::Allow,
            PolicyPriority::new(100),
            "Règle satisfaite car l'acteur est admin".to_string(),
        );
        assert!(result.is_satisfied());
        assert!(result.justification.is_some());
        assert_eq!(
            result.justification.as_ref().unwrap(),
            "Règle satisfaite car l'acteur est admin"
        );
    }

    #[test]
    fn test_policy_engine_determinism() {
        let policy = Policy::new(
            "POL-001",
            PolicyType::Permission,
            PolicyCondition::always(),
            PolicyRule::new("always"),
            PolicyEffect::Allow,
            PolicyPriority::new(100),
        );
        let policies = PolicySet::from_vec(vec![policy]);
        let engine = PolicyEngine::new(policies);
        let intent = create_test_intent();
        let context = create_test_context();

        // Première évaluation
        let results1 = engine.apply(&intent, &context).unwrap();
        // Deuxième évaluation (même intention, mêmes politiques)
        let results2 = engine.apply(&intent, &context).unwrap();

        // Les résultats doivent être identiques (déterminisme, G-POL-1, INV-POL-3)
        assert_eq!(results1.len(), results2.len());
        assert_eq!(results1[0].policy_id, results2[0].policy_id);
        assert_eq!(results1[0].result, results2[0].result);
    }
}
