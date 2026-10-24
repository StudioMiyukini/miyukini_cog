//! # StrongFather — Moteur de Politiques
//!
//! Ce module définit le moteur d'application des politiques et la fonction de décision
//! [`PolicyEngine::decide`] (pure, déterministe, refus par défaut).
//!
//! ## Ordre de décision (`decide`)
//!
//! 1. Validation structurelle de l'intention (`IntentValidator`) — échec ⇒ **Deny**
//! 2. Sélection des politiques par **cible structurée** (égalité exacte : action, sujet,
//!    appelant, origine, zone) ; ordre stable : priorité décroissante puis identifiant
//! 3. Pour chaque politique sélectionnée : cohérence type/effet, `condition`, puis `rule`
//!    (et politiques élémentaires pour une composite). **Toute erreur** (expression mal
//!    formée, attribut inconnu, type, composite introuvable ou cyclique) ⇒ **Deny** immédiat
//! 4. Une politique `Deny` applicable dont la règle est vraie ⇒ **Deny** (le refus explicite
//!    l'emporte toujours sur une autorisation, quelle que soit la priorité)
//! 5. Une politique `Constrain` / `Validate` applicable dont la règle est fausse ⇒ **Deny**
//! 6. Une politique `Authorize` applicable dont la règle est vraie ⇒ **Allow**
//! 7. Sinon ⇒ **Deny** « défaut: refus »
//!
//! ## Références
//!
//! - [Policy Engine Contract](../../../docs/core/StrongFather/contracts/policy/StrongFather%20-%20Policy%20Engine%20Contract.md)
//! - [Documentation Fondatrice](../../../docs/core/StrongFather/foundation/StrongFather%20-%20Documentation%20Fondatrice.md)

use std::collections::HashMap;
use std::sync::Arc;

use crate::decision::{
    Decision, DecisionType, EvaluationContext, Justification, DEFAULT_DENY_RULE,
};
use crate::expr::Expr;
use crate::intent::Intent;
use crate::policy::{
    Policy, PolicyEffect, PolicyId, PolicyPriorityLevel, PolicyResult, PolicySet, PolicyType,
};
use crate::priority::PriorityCalculator;
use crate::validator::IntentValidator;

/// Profondeur maximale d'imbrication des politiques composites.
pub const MAX_COMPOSITE_DEPTH: usize = 8;

/// Identifiant de règle cité quand l'intention est structurellement invalide.
pub const STRUCTURAL_VALIDATION_RULE: &str = "validation-structurelle";

/// @id strongfather_policy_engine_error
/// @role error
/// @layer core
/// @do represent_policy_engine_error
//@human / @human Erreur du moteur de politiques. Indique un dysfonctionnement interne.
/// Erreur du moteur de politiques
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyEngineError {
    /// @id strongfather_policy_engine_error_consistency
    /// @role error
    /// @layer core
    /// @do represent_consistency_error
    //@human / @human Erreur de cohérence - violation d'un invariant.
    ConsistencyError {
        /// @id strongfather_policy_engine_error_invariant
        /// @role error
        /// @layer core
        /// @do identify_violated_invariant
        //@human / @human Identifiant de l'invariant violé.
        violated_invariant: String,
        /// @id strongfather_policy_engine_error_reason
        /// @role error
        /// @layer core
        /// @do explain_error
        //@human / @human Raison de l'erreur.
        reason: String,
    },
}

impl std::fmt::Display for PolicyEngineError {
    /// @id strongfather_policy_engine_error_display
    /// @role formatter
    /// @layer core
    /// @do format_policy_engine_error
    //@human / @human Formate l'erreur pour affichage.
    /// @depends strongfather_policy_engine_error
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PolicyEngineError::ConsistencyError {
                violated_invariant,
                reason,
            } => {
                write!(f, "Consistency error: {violated_invariant} - {reason}")
            }
        }
    }
}

impl std::error::Error for PolicyEngineError {}

/// Expressions compilées d'une politique (condition, règle), ou l'erreur d'analyse.
type CompiledExprs = Result<(Expr, Expr), String>;

/// État immuable partagé du moteur (clonage bon marché via `Arc`).
#[derive(Debug)]
struct Compiled {
    policies: PolicySet,
    exprs: HashMap<PolicyId, CompiledExprs>,
    /// Ordre d'examen stable : priorité décroissante, puis identifiant croissant.
    order: Vec<PolicyId>,
}

/// @id strongfather_policy_engine
/// @role service
/// @layer core
/// @do apply_policies_to_intent
//@human / @human Moteur de politiques. Applique les politiques pour évaluer des intentions (G-POL-1 : évaluation déterministe).
//@human Les expressions sont analysées une seule fois à la construction ; le moteur se clone à coût constant.
/// Moteur de politiques
#[derive(Debug, Clone)]
pub struct PolicyEngine {
    inner: Arc<Compiled>,
}

/// Résultat intermédiaire de l'examen d'une politique.
enum Outcome {
    /// Erreur ⇒ refus par sécurité.
    Error(String),
    /// Condition fausse : politique non applicable.
    NotApplicable,
    /// Condition vraie ; `holds` = valeur de la règle (et des élémentaires).
    Applicable { holds: bool },
}

impl PolicyEngine {
    /// @id strongfather_policy_engine_new
    /// @role constructor
    /// @layer core
    /// @do create_policy_engine
    //@human / @human Crée un moteur. Les politiques invalides sont conservées mais toute intention
    //@human qu'elles visent sera refusée (fail-closed). Préférer `try_new` pour rejeter dès la construction.
    /// @depends strongfather_policy_engine
    #[must_use]
    pub fn new(policies: PolicySet) -> Self {
        let mut exprs = HashMap::new();
        for p in policies.iter() {
            exprs.insert(p.id.clone(), compile(p));
        }
        let mut order: Vec<(PolicyPriorityLevel, PolicyId)> = policies
            .iter()
            .map(|p| (p.priority, p.id.clone()))
            .collect();
        order.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
        Self {
            inner: Arc::new(Compiled {
                policies,
                exprs,
                order: order.into_iter().map(|(_, id)| id).collect(),
            }),
        }
    }

    /// @id strongfather_policy_engine_try_new
    /// @role constructor
    /// @layer core
    /// @do create_validated_policy_engine
    //@human / @human Crée un moteur après validation complète : cohérence type/effet, expressions
    //@human analysables, composites existantes et acycliques. Toute anomalie ⇒ erreur.
    pub fn try_new(policies: PolicySet) -> Result<Self, PolicyEngineError> {
        let engine = Self::new(policies);
        engine.validate()?;
        Ok(engine)
    }

    /// @id strongfather_policy_engine_policies
    /// @role query
    /// @layer core
    /// @do expose_policy_set
    //@human / @human Ensemble de politiques chargé dans le moteur (lecture seule).
    #[must_use]
    pub fn policies(&self) -> &PolicySet {
        &self.inner.policies
    }

    /// @id strongfather_policy_engine_validate
    /// @role query
    /// @layer core
    /// @do validate_policy_set
    //@human / @human Vérifie l'ensemble : cohérence, expressions, références composites, absence de cycle.
    pub fn validate(&self) -> Result<(), PolicyEngineError> {
        let err = |inv: &str, reason: String| PolicyEngineError::ConsistencyError {
            violated_invariant: inv.to_string(),
            reason,
        };
        for id in &self.inner.order {
            let p = self
                .inner
                .policies
                .get(id)
                .ok_or_else(|| err("INV-POL-SOURCE", format!("politique '{id}' absente")))?;
            p.check_coherence().map_err(|e| err("RÈGLE-STRUCT-5", e))?;
            if let Some(Err(e)) = self.inner.exprs.get(id) {
                return Err(err("RÈGLE-STRUCT-4", e.clone()));
            }
            self.check_composite(p, &mut Vec::new())
                .map_err(|e| err("RÈGLE-COMPOSITE", e))?;
        }
        Ok(())
    }

    fn check_composite(&self, p: &Policy, stack: &mut Vec<PolicyId>) -> Result<(), String> {
        if stack.contains(&p.id) {
            return Err(format!("cycle de composites via '{}'", p.id));
        }
        if stack.len() >= MAX_COMPOSITE_DEPTH {
            return Err(format!("composites trop profondes via '{}'", p.id));
        }
        if let Some(children) = &p.composite_policies {
            stack.push(p.id.clone());
            for c in children {
                let child =
                    self.inner.policies.get(c).ok_or_else(|| {
                        format!("composite '{}' : élémentaire '{c}' inconnue", p.id)
                    })?;
                self.check_composite(child, stack)?;
            }
            stack.pop();
        }
        Ok(())
    }

    /// Évalue la règle d'une politique (et, pour une composite, le prédicat complet de chaque
    /// élémentaire : cible ∧ condition ∧ règle).
    fn holds(&self, p: &Policy, intent: &Intent, depth: usize) -> Result<bool, String> {
        if depth > MAX_COMPOSITE_DEPTH {
            return Err(format!("composites trop profondes via '{}'", p.id));
        }
        p.check_coherence()?;
        let (_, rule) = self.exprs_of(p)?;
        if !rule
            .eval(intent)
            .map_err(|e| format!("règle de '{}' : {e}", p.id))?
        {
            return Ok(false);
        }
        if p.policy_type == PolicyType::Composite {
            for c in p.composite_policies.as_deref().unwrap_or_default() {
                let child =
                    self.inner.policies.get(c).ok_or_else(|| {
                        format!("composite '{}' : élémentaire '{c}' inconnue", p.id)
                    })?;
                if !child.target.matches(intent) {
                    return Ok(false);
                }
                let (cond, _) = self.exprs_of(child)?;
                if !cond
                    .eval(intent)
                    .map_err(|e| format!("condition de '{}' : {e}", child.id))?
                {
                    return Ok(false);
                }
                if !self.holds(child, intent, depth + 1)? {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    fn exprs_of(&self, p: &Policy) -> Result<&(Expr, Expr), String> {
        match self.inner.exprs.get(&p.id) {
            Some(Ok(e)) => Ok(e),
            Some(Err(e)) => Err(e.clone()),
            None => Err(format!("politique '{}' non compilée", p.id)),
        }
    }

    fn examine(&self, p: &Policy, intent: &Intent) -> Outcome {
        if let Err(e) = p.check_coherence() {
            return Outcome::Error(e);
        }
        let cond = match self.exprs_of(p) {
            Ok((cond, _)) => cond,
            Err(e) => return Outcome::Error(e),
        };
        match cond.eval(intent) {
            Err(e) => Outcome::Error(format!("condition de '{}' : {e}", p.id)),
            Ok(false) => Outcome::NotApplicable,
            Ok(true) => match self.holds(p, intent, 0) {
                Err(e) => Outcome::Error(e),
                Ok(holds) => Outcome::Applicable { holds },
            },
        }
    }

    /// @id strongfather_policy_engine_decide
    /// @role evaluator
    /// @layer core
    /// @do decide_intent
    //@human / @human Décide ALLOW / DENY pour une intention. Pure, déterministe, refus par défaut.
    //@human Ordre : validation → sélection exacte → erreurs (refus) → Deny explicite → contrainte
    //@human violée → Authorize → défaut: refus. La décision porte l'empreinte de l'intention.
    #[must_use]
    pub fn decide(&self, intent: &Intent) -> Decision {
        let digest = intent.digest();
        let mut steps: Vec<String> = Vec::new();

        if let Err(e) = IntentValidator::new().validate(intent) {
            return refuse(
                intent,
                digest,
                STRUCTURAL_VALIDATION_RULE,
                format!("intention structurellement invalide : {e}"),
                Vec::new(),
                vec![format!("validation structurelle : {e}")],
            );
        }

        let mut examined: Vec<PolicyId> = Vec::new();
        let mut deny: Option<&Policy> = None;
        let mut violated: Option<&Policy> = None;
        let mut allow: Option<&Policy> = None;
        let mut prio_results = Vec::new();

        for id in &self.inner.order {
            let Some(p) = self.inner.policies.get(id) else {
                continue;
            };
            if !p.target.matches(intent) {
                continue;
            }
            examined.push(p.id.clone());
            match self.examine(p, intent) {
                Outcome::Error(e) => {
                    steps.push(format!("{} : erreur ({e})", p.id));
                    return refuse(
                        intent,
                        digest,
                        &p.id,
                        format!(
                            "évaluation impossible de '{}' : {e} — refus par sécurité",
                            p.id
                        ),
                        examined,
                        steps,
                    );
                }
                Outcome::NotApplicable => steps.push(format!("{} : condition fausse", p.id)),
                Outcome::Applicable { holds } => {
                    steps.push(format!("{} : applicable, règle = {holds}", p.id));
                    match p.effect {
                        PolicyEffect::Deny if holds && deny.is_none() => deny = Some(p),
                        PolicyEffect::Constrain | PolicyEffect::Validate
                            if !holds && violated.is_none() =>
                        {
                            violated = Some(p);
                        }
                        PolicyEffect::Authorize if holds && allow.is_none() => allow = Some(p),
                        PolicyEffect::Prioritize if holds => {
                            prio_results.push(PolicyResult::Priority {
                                policy_id: p.id.clone(),
                                value: priority_value(p.priority),
                            });
                        }
                        _ => {}
                    }
                }
            }
        }

        if let Some(p) = deny {
            return refuse(
                intent,
                digest,
                &p.id,
                format!("refus explicite par la politique '{}'", p.id),
                examined,
                steps,
            );
        }
        if let Some(p) = violated {
            return refuse(
                intent,
                digest,
                &p.id,
                format!("contrainte non satisfaite : politique '{}'", p.id),
                examined,
                steps,
            );
        }
        if let Some(p) = allow {
            let priority = PriorityCalculator::new().calculate(intent, &prio_results);
            let mut d = Decision::new(
                intent.intent_id.clone(),
                DecisionType::Accepted {
                    priority: priority.value,
                },
                Justification::new(
                    format!("autorisé par la politique '{}'", p.id),
                    vec![p.id.clone()],
                )
                .with_reasoning_steps(steps),
                examined,
            )
            .with_evaluation_context(EvaluationContext::new().with_field(
                "policy_count".to_string(),
                self.inner.order.len().to_string(),
            ));
            d.deciding_rule.clone_from(&p.id);
            d.intent_digest = digest;
            return d;
        }
        steps.push("aucune politique d'autorisation applicable".to_string());
        refuse(
            intent,
            digest,
            DEFAULT_DENY_RULE,
            "aucune politique n'autorise cette intention (défaut: refus)".to_string(),
            examined,
            steps,
        )
    }

    /// @id strongfather_policy_engine_apply
    /// @role evaluator
    /// @layer core
    /// @do apply_policies
    //@human / @human Applique les politiques visées par l'intention et retourne leurs résultats détaillés
    //@human (diagnostic). Ne rend pas de verdict : utiliser `decide` pour autoriser une action.
    /// @depends strongfather_policy_engine_new
    pub fn apply(&self, intent: &Intent) -> Result<Vec<PolicyResult>, PolicyEngineError> {
        let mut results = Vec::new();
        for id in &self.inner.order {
            let Some(p) = self.inner.policies.get(id) else {
                continue;
            };
            if !p.target.matches(intent) {
                continue;
            }
            let violated = |reason: String| PolicyResult::Violated {
                policy_id: p.id.clone(),
                reason,
            };
            let satisfied = PolicyResult::Satisfied {
                policy_id: p.id.clone(),
            };
            let result = match (self.examine(p, intent), p.effect) {
                (Outcome::Error(e), _) => violated(format!("évaluation impossible : {e}")),
                (Outcome::Applicable { holds: true }, PolicyEffect::Prioritize) => {
                    PolicyResult::Priority {
                        policy_id: p.id.clone(),
                        value: priority_value(p.priority),
                    }
                }
                (Outcome::Applicable { holds: true }, PolicyEffect::Deny) => {
                    violated("refus explicite".to_string())
                }
                (
                    Outcome::Applicable { holds: false },
                    PolicyEffect::Constrain | PolicyEffect::Validate,
                ) => violated("contrainte non satisfaite".to_string()),
                (Outcome::Applicable { holds: false }, PolicyEffect::Authorize) => {
                    violated("permission non accordée".to_string())
                }
                _ => satisfied,
            };
            // RÈGLE-PRIO-2 : arrêt sur violation critique
            let stop = p.is_critical() && matches!(result, PolicyResult::Violated { .. });
            results.push(result);
            if stop {
                break;
            }
        }
        Ok(results)
    }
}

fn compile(p: &Policy) -> CompiledExprs {
    let cond = Expr::parse(&p.condition.expression)
        .map_err(|e| format!("condition de '{}' : {e}", p.id))?;
    let rule = Expr::parse(&p.rule.expression).map_err(|e| format!("règle de '{}' : {e}", p.id))?;
    Ok((cond, rule))
}

fn priority_value(level: PolicyPriorityLevel) -> u32 {
    match level {
        PolicyPriorityLevel::Critical => 100,
        PolicyPriorityLevel::Important => 50,
        PolicyPriorityLevel::Normal => 25,
        PolicyPriorityLevel::Optional => 10,
    }
}

fn refuse(
    intent: &Intent,
    digest: String,
    rule: &str,
    reason: String,
    examined: Vec<PolicyId>,
    steps: Vec<String>,
) -> Decision {
    let refs = if rule == DEFAULT_DENY_RULE {
        Vec::new()
    } else {
        vec![rule.to_string()]
    };
    let mut d = Decision::new(
        intent.intent_id.clone(),
        DecisionType::Refused {
            reason: reason.clone(),
            violated_policies: refs.clone(),
        },
        Justification::new(reason, refs).with_reasoning_steps(steps),
        examined,
    );
    d.deciding_rule = rule.to_string();
    d.intent_digest = digest;
    d
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::decision::Verdict;
    use crate::intent::{ActionType, CallContext, Intent, IntentData};
    use crate::policy::{
        Policy, PolicyCondition, PolicyEffect, PolicyRule, PolicySet, PolicyTarget,
    };
    use std::collections::HashMap;

    fn intent(action: ActionType, subject: &str, caller: &str) -> Intent {
        let mut data = HashMap::new();
        data.insert("montant".to_string(), "42".to_string());
        Intent::new(
            "intent_001".to_string(),
            action,
            subject.to_string(),
            CallContext {
                caller_identity: caller.to_string(),
                origin: "maia".to_string(),
                instance: "cog-1".to_string(),
            },
            IntentData { content: data },
        )
    }

    fn pol(id: &str, t: PolicyType, e: PolicyEffect, cond: &str, rule: &str) -> Policy {
        Policy::new(
            id.to_string(),
            t,
            PolicyCondition {
                expression: cond.to_string(),
            },
            PolicyRule {
                expression: rule.to_string(),
            },
            e,
        )
    }

    fn subjects(s: &[&str]) -> PolicyTarget {
        PolicyTarget {
            subjects: Some(s.iter().map(|x| (*x).to_string()).collect()),
            ..PolicyTarget::default()
        }
    }

    fn engine(policies: Vec<Policy>) -> PolicyEngine {
        let mut set = PolicySet::new();
        for p in policies {
            set.add(p).unwrap();
        }
        PolicyEngine::new(set)
    }

    fn allow_read_jaykonta() -> Policy {
        pol(
            "allow-read-jaykonta",
            PolicyType::Permission,
            PolicyEffect::Authorize,
            "true",
            "caller == 'op-1'",
        )
        .with_target(PolicyTarget {
            actions: Some(vec![ActionType::Read]),
            ..subjects(&["jaykonta"])
        })
    }

    /// @id strongfather_policy_engine_test_default_deny
    /// @role test
    /// @layer core
    /// @do test_default_deny
    //@human / @human Sans politique applicable, la décision est un refus « défaut: refus ».
    #[test]
    fn test_refus_par_defaut() {
        let d = engine(vec![]).decide(&intent(ActionType::Read, "jaykonta", "op-1"));
        assert_eq!(d.verdict(), Verdict::Deny);
        assert!(d.is_refused());
        assert_eq!(d.deciding_rule, DEFAULT_DENY_RULE);

        // Politique existante mais visant une autre action ⇒ refus par défaut.
        let d = engine(vec![allow_read_jaykonta()]).decide(&intent(
            ActionType::Deletion,
            "jaykonta",
            "op-1",
        ));
        assert_eq!(d.verdict(), Verdict::Deny);
        assert_eq!(d.deciding_rule, DEFAULT_DENY_RULE);
    }

    /// @id strongfather_policy_engine_test_allow
    /// @role test
    /// @layer core
    /// @do test_allow_when_rule_true
    //@human / @human Une politique Authorize ciblée dont la règle est vraie autorise ; fausse ⇒ refus.
    #[test]
    fn test_autorisation_regle_vraie_ou_fausse() {
        let e = engine(vec![allow_read_jaykonta()]);
        let ok = intent(ActionType::Read, "jaykonta", "op-1");
        let d = e.decide(&ok);
        assert_eq!(d.verdict(), Verdict::Allow);
        assert_eq!(d.deciding_rule, "allow-read-jaykonta");
        assert!(d.matches_intent(&ok));

        let d = e.decide(&intent(ActionType::Read, "jaykonta", "op-2"));
        assert_eq!(d.verdict(), Verdict::Deny);
        assert_eq!(d.deciding_rule, DEFAULT_DENY_RULE);
    }

    /// @id strongfather_policy_engine_test_condition
    /// @role test
    /// @layer core
    /// @do test_condition_gates_applicability
    //@human / @human La condition détermine l'applicabilité : condition fausse ⇒ la politique ne compte pas.
    #[test]
    fn test_condition_vraie_fausse() {
        let p = pol(
            "allow-small",
            PolicyType::Permission,
            PolicyEffect::Authorize,
            "data.montant <= 100",
            "true",
        );
        let e = engine(vec![p]);
        assert_eq!(
            e.decide(&intent(ActionType::Creation, "x", "op-1"))
                .verdict(),
            Verdict::Allow
        );
        let mut big = intent(ActionType::Creation, "x", "op-1");
        big.data
            .content
            .insert("montant".to_string(), "1000".to_string());
        assert_eq!(e.decide(&big).verdict(), Verdict::Deny);
    }

    /// @id strongfather_policy_engine_test_deny_over_allow
    /// @role test
    /// @layer core
    /// @do test_deny_beats_allow
    //@human / @human Un refus explicite l'emporte sur une autorisation, même de priorité supérieure.
    #[test]
    fn test_refus_l_emporte_sur_autorisation() {
        let allow = allow_read_jaykonta().with_priority(PolicyPriorityLevel::Critical);
        let deny = pol(
            "deny-op-1",
            PolicyType::Permission,
            PolicyEffect::Deny,
            "true",
            "caller == 'op-1'",
        )
        .with_priority(PolicyPriorityLevel::Optional);
        let d = engine(vec![allow, deny]).decide(&intent(ActionType::Read, "jaykonta", "op-1"));
        assert_eq!(d.verdict(), Verdict::Deny);
        assert_eq!(d.deciding_rule, "deny-op-1");
    }

    /// @id strongfather_policy_engine_test_malformed
    /// @role test
    /// @layer core
    /// @do test_malformed_fails_closed
    //@human / @human Expression mal formée ou attribut inconnu ⇒ refus (fail-closed), même si une autre règle autorise.
    #[test]
    fn test_expression_invalide_refus() {
        let malformed = pol(
            "broken",
            PolicyType::Permission,
            PolicyEffect::Authorize,
            "true",
            "caller == ",
        );
        let i = intent(ActionType::Read, "jaykonta", "op-1");
        let d = engine(vec![allow_read_jaykonta(), malformed.clone()]).decide(&i);
        assert_eq!(d.verdict(), Verdict::Deny);
        assert_eq!(d.deciding_rule, "broken");
        assert!(d.reason().contains("refus par sécurité"));

        let unknown = pol(
            "unknown-attr",
            PolicyType::Permission,
            PolicyEffect::Authorize,
            "true",
            "user.role == 'admin'",
        );
        let d = engine(vec![unknown]).decide(&i);
        assert_eq!(d.verdict(), Verdict::Deny);
        assert_eq!(d.deciding_rule, "unknown-attr");

        // Un Deny dont la condition est invalide refuse aussi (pas d'autorisation par erreur).
        let bad_deny = pol(
            "bad-deny",
            PolicyType::Permission,
            PolicyEffect::Deny,
            "data.inexistant == 'x'",
            "true",
        );
        let d = engine(vec![allow_read_jaykonta(), bad_deny]).decide(&i);
        assert_eq!(d.verdict(), Verdict::Deny);
        assert_eq!(d.deciding_rule, "bad-deny");

        // try_new rejette l'ensemble dès la construction.
        let mut set = PolicySet::new();
        set.add(malformed).unwrap();
        assert!(PolicyEngine::try_new(set).is_err());
    }

    /// @id strongfather_policy_engine_test_no_substring
    /// @role test
    /// @layer core
    /// @do test_exact_target_selection
    //@human / @human Une règle pour `jaykonta` ne vise ni `jaykonta-evil` ni `jaykont` (pas de sous-chaîne).
    #[test]
    fn test_pas_de_confusion_sous_chaine() {
        let e = engine(vec![allow_read_jaykonta()]);
        for s in ["jaykonta-evil", "jaykont", "xjaykonta", "JayKonta"] {
            let d = e.decide(&intent(ActionType::Read, s, "op-1"));
            assert_eq!(d.verdict(), Verdict::Deny, "sujet {s}");
            assert_eq!(d.deciding_rule, DEFAULT_DENY_RULE);
        }
        // Ancienne sélection par `contains` : une condition mentionnant « read » ne doit plus suffire.
        let p = pol(
            "legacy",
            PolicyType::Permission,
            PolicyEffect::Authorize,
            "action == 'read' && subject == 'jaykonta'",
            "true",
        );
        let d = engine(vec![p]).decide(&intent(ActionType::Read, "jaykonta-evil", "op-1"));
        assert_eq!(d.verdict(), Verdict::Deny);
    }

    /// @id strongfather_policy_engine_test_constraint
    /// @role test
    /// @layer core
    /// @do test_constraint_evaluated
    //@human / @human Constraint et Validation sont réellement évaluées : règle fausse ⇒ refus.
    #[test]
    fn test_contrainte_et_validation_evaluees() {
        let allow = pol(
            "allow-all-create",
            PolicyType::Permission,
            PolicyEffect::Authorize,
            "action == 'creation'",
            "true",
        );
        let constraint = pol(
            "max-montant",
            PolicyType::Constraint,
            PolicyEffect::Constrain,
            "true",
            "data.montant <= 50",
        );
        let validation = pol(
            "origine-connue",
            PolicyType::Validation,
            PolicyEffect::Validate,
            "true",
            "origin in ['maia', 'alicia']",
        );
        let e = engine(vec![allow, constraint, validation]);
        let ok = intent(ActionType::Creation, "x", "op-1");
        assert_eq!(e.decide(&ok).verdict(), Verdict::Allow);

        let mut trop = ok.clone();
        trop.data
            .content
            .insert("montant".to_string(), "51".to_string());
        let d = e.decide(&trop);
        assert_eq!(d.verdict(), Verdict::Deny);
        assert_eq!(d.deciding_rule, "max-montant");

        let mut etranger = ok.clone();
        etranger.call_context.origin = "inconnu".to_string();
        let d = e.decide(&etranger);
        assert_eq!(d.verdict(), Verdict::Deny);
        assert_eq!(d.deciding_rule, "origine-connue");
    }

    /// @id strongfather_policy_engine_test_composite
    /// @role test
    /// @layer core
    /// @do test_composite_evaluated
    //@human / @human Une composite n'autorise que si toutes ses élémentaires sont vraies ; référence absente ou cycle ⇒ refus.
    #[test]
    fn test_composite_evaluee() {
        let a = pol(
            "is-op-1",
            PolicyType::Permission,
            PolicyEffect::Deny,
            "false",
            "caller == 'op-1'",
        );
        let b = pol(
            "is-small",
            PolicyType::Permission,
            PolicyEffect::Deny,
            "false",
            "data.montant < 100",
        );
        let comp = pol(
            "comp",
            PolicyType::Composite,
            PolicyEffect::Authorize,
            "true",
            "true",
        )
        .with_composite_policies(vec!["is-op-1".to_string(), "is-small".to_string()]);
        // Les élémentaires ont une condition « false » au niveau racine : elles ne comptent
        // que comme briques de la composite… mais leur condition s'applique aussi dans la composite.
        let e = engine(vec![a, b, comp]);
        assert_eq!(
            e.decide(&intent(ActionType::Read, "x", "op-1")).verdict(),
            Verdict::Deny,
            "condition fausse d'une élémentaire ⇒ composite fausse"
        );

        let a = pol(
            "is-op-1",
            PolicyType::Permission,
            PolicyEffect::Deny,
            "action == 'evaluation'",
            "caller == 'op-1'",
        );
        let b = pol(
            "is-small",
            PolicyType::Permission,
            PolicyEffect::Deny,
            "action == 'evaluation'",
            "data.montant < 100",
        );
        let comp = pol(
            "comp",
            PolicyType::Composite,
            PolicyEffect::Authorize,
            "true",
            "true",
        )
        .with_composite_policies(vec!["is-op-1".to_string(), "is-small".to_string()]);
        let e = engine(vec![a, b, comp]);
        // Pour `read`, les élémentaires sont non applicables (condition fausse) ⇒ composite fausse.
        assert_eq!(
            e.decide(&intent(ActionType::Read, "x", "op-1")).verdict(),
            Verdict::Deny
        );

        let a = pol(
            "is-op-1",
            PolicyType::Validation,
            PolicyEffect::Validate,
            "true",
            "caller == 'op-1'",
        );
        let b = pol(
            "is-small",
            PolicyType::Validation,
            PolicyEffect::Validate,
            "true",
            "data.montant < 100",
        );
        let comp = pol(
            "comp",
            PolicyType::Composite,
            PolicyEffect::Authorize,
            "true",
            "true",
        )
        .with_composite_policies(vec!["is-op-1".to_string(), "is-small".to_string()]);
        let e = engine(vec![a, b, comp]);
        let d = e.decide(&intent(ActionType::Read, "x", "op-1"));
        assert_eq!(d.verdict(), Verdict::Allow);
        assert_eq!(d.deciding_rule, "comp");
        assert_eq!(
            e.decide(&intent(ActionType::Read, "x", "op-2")).verdict(),
            Verdict::Deny
        );

        let missing = pol(
            "comp",
            PolicyType::Composite,
            PolicyEffect::Authorize,
            "true",
            "true",
        )
        .with_composite_policies(vec!["absente".to_string()]);
        let d = engine(vec![missing]).decide(&intent(ActionType::Read, "x", "op-1"));
        assert_eq!(d.verdict(), Verdict::Deny);
        assert_eq!(d.deciding_rule, "comp");

        let c1 = pol(
            "c1",
            PolicyType::Composite,
            PolicyEffect::Authorize,
            "true",
            "true",
        )
        .with_composite_policies(vec!["c2".to_string()]);
        let c2 = pol(
            "c2",
            PolicyType::Composite,
            PolicyEffect::Validate,
            "true",
            "true",
        )
        .with_composite_policies(vec!["c1".to_string()]);
        let mut set = PolicySet::new();
        set.add(c1.clone()).unwrap();
        set.add(c2.clone()).unwrap();
        assert!(PolicyEngine::try_new(set).is_err());
        let d = engine(vec![c1, c2]).decide(&intent(ActionType::Read, "x", "op-1"));
        assert_eq!(d.verdict(), Verdict::Deny);
    }

    /// @id strongfather_policy_engine_test_incoherent
    /// @role test
    /// @layer core
    /// @do test_incoherent_policy_denies
    //@human / @human Une politique dont l'effet contredit le type provoque un refus.
    #[test]
    fn test_politique_incoherente_refus() {
        let p = pol(
            "bad",
            PolicyType::Constraint,
            PolicyEffect::Authorize,
            "true",
            "true",
        );
        let d = engine(vec![p]).decide(&intent(ActionType::Read, "x", "op-1"));
        assert_eq!(d.verdict(), Verdict::Deny);
        assert_eq!(d.deciding_rule, "bad");
    }

    /// @id strongfather_policy_engine_test_invalid_intent
    /// @role test
    /// @layer core
    /// @do test_invalid_intent_denied
    //@human / @human Une intention structurellement invalide est refusée avant toute politique.
    #[test]
    fn test_intention_invalide_refusee() {
        let p = pol(
            "all",
            PolicyType::Permission,
            PolicyEffect::Authorize,
            "true",
            "true",
        );
        let d = engine(vec![p]).decide(&intent(ActionType::Read, "x", ""));
        assert_eq!(d.verdict(), Verdict::Deny);
        assert_eq!(d.deciding_rule, STRUCTURAL_VALIDATION_RULE);
    }

    /// @id strongfather_policy_engine_test_digest
    /// @role test
    /// @layer core
    /// @do test_decision_digest_binding
    //@human / @human La décision porte l'empreinte de l'intention ; une autre intention ne correspond pas.
    #[test]
    fn test_empreinte_decision() {
        let e = engine(vec![allow_read_jaykonta()]);
        let i = intent(ActionType::Read, "jaykonta", "op-1");
        let d1 = e.decide(&i);
        let d2 = e.clone().decide(&i);
        assert_eq!(d1, d2, "déterminisme");
        assert_eq!(d1.intent_digest, i.digest());
        let mut autre = i.clone();
        autre
            .data
            .content
            .insert("montant".to_string(), "43".to_string());
        assert!(!d1.matches_intent(&autre));
        assert_ne!(e.decide(&autre).intent_digest, d1.intent_digest);
    }

    /// @id strongfather_policy_engine_test_priority
    /// @role test
    /// @layer core
    /// @do test_priority_policies
    //@human / @human Les politiques de priorité fixent la priorité de la décision acceptée.
    #[test]
    fn test_priorite_decision() {
        let prio = pol(
            "prio",
            PolicyType::Priority,
            PolicyEffect::Prioritize,
            "true",
            "true",
        )
        .with_priority(PolicyPriorityLevel::Important);
        let d = engine(vec![allow_read_jaykonta(), prio]).decide(&intent(
            ActionType::Read,
            "jaykonta",
            "op-1",
        ));
        assert_eq!(d.decision_type, DecisionType::Accepted { priority: 50 });
    }

    /// @id strongfather_policy_engine_test_apply
    /// @role test
    /// @layer core
    /// @do test_apply_permission_policy
    //@human / @human `apply` retourne le détail par politique visée ; règle inconnue ⇒ Violated.
    #[test]
    fn test_apply_detail() {
        let p = pol(
            "POL-001",
            PolicyType::Permission,
            PolicyEffect::Authorize,
            "action == 'creation'",
            "user.role == 'admin'",
        );
        let e = engine(vec![p, allow_read_jaykonta()]);
        let results = e
            .apply(&intent(ActionType::Creation, "entity_123", "user123"))
            .unwrap();
        assert_eq!(results.len(), 1);
        assert!(matches!(results[0], PolicyResult::Violated { .. }));
    }
}
