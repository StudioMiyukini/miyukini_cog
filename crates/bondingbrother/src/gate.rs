//! @id     bondingbrother_gate
//! @do     imposer_decision_strongfather
//! @role   orchestration
//! @layer  core
//! @human  Porte obligatoire : aucune action gouvernée sans décision ALLOW de StrongFather.
//!
//! # BondingBrother — Porte de gouvernance
//!
//! - [`Gate::autoriser`] soumet une intention et rend soit un [`DecisionAutorisee`]
//!   (jeton infalsifiable : champs privés, constructible uniquement dans ce module),
//!   soit un [`Refus`] portant la décision complète
//! - [`executer_si_autorise`] est la façon canonique d'exécuter une action gouvernée :
//!   la fermeture n'est appelée que si la porte autorise, une seule fois, avec le jeton
//! - [`PorteStrongFather`] est l'implémentation adossée à `strongfather::PolicyEngine::decide`
//!   (refus par défaut, fail-closed)

use std::fmt;
use std::path::Path;

use strongfather::{Decision, Intent, PolicyEngine, PolicyLoadError, PolicySet, Verdict};

/// @id bondingbrother_decision_autorisee
/// @role data
/// @layer core
/// @do carry_allow_token
//@human / @human Jeton d'autorisation. Champs privés, ni `Clone` ni constructeur public : seule
//@human une porte de ce module peut en produire, à partir d'une décision ALLOW de StrongFather.
#[derive(Debug, PartialEq, Eq)]
pub struct DecisionAutorisee {
    decision: Decision,
}

impl DecisionAutorisee {
    /// Scelle une décision ALLOW. Réservé au crate : rend `None` si la décision n'autorise pas.
    fn sceller(decision: Decision, intent: &Intent) -> Option<Self> {
        (decision.verdict() == Verdict::Allow && decision.matches_intent(intent))
            .then_some(Self { decision })
    }

    /// @id bondingbrother_decision_autorisee_decision
    /// @role query
    /// @layer core
    /// @do expose_decision
    //@human / @human Décision StrongFather complète (à journaliser avec l'action).
    #[must_use]
    pub fn decision(&self) -> &Decision {
        &self.decision
    }

    /// @id bondingbrother_decision_autorisee_regle
    /// @role query
    /// @layer core
    /// @do expose_deciding_rule
    //@human / @human Identifiant de la politique qui a autorisé.
    #[must_use]
    pub fn regle(&self) -> &str {
        &self.decision.deciding_rule
    }

    /// @id bondingbrother_decision_autorisee_empreinte
    /// @role query
    /// @layer core
    /// @do expose_intent_digest
    //@human / @human Empreinte SHA-256 de l'intention autorisée.
    #[must_use]
    pub fn empreinte(&self) -> &str {
        &self.decision.intent_digest
    }

    /// @id bondingbrother_decision_autorisee_couvre
    /// @role query
    /// @layer core
    /// @do check_token_covers_intent
    //@human / @human Vrai si le jeton couvre exactement cette intention (même id, même empreinte).
    #[must_use]
    pub fn couvre(&self, intent: &Intent) -> bool {
        self.decision.matches_intent(intent)
    }
}

/// @id bondingbrother_refus
/// @role error
/// @layer core
/// @do represent_refusal
//@human / @human Refus de la porte : porte la décision StrongFather (règle décisive, raison, empreinte).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refus {
    /// Décision de refus produite par StrongFather (en boîte : `Result` léger).
    pub decision: Box<Decision>,
}

impl Refus {
    /// Raison lisible du refus.
    #[must_use]
    pub fn raison(&self) -> &str {
        self.decision.reason()
    }

    /// Identifiant de la règle décisive (ou `"défaut: refus"`).
    #[must_use]
    pub fn regle(&self) -> &str {
        &self.decision.deciding_rule
    }
}

impl fmt::Display for Refus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "refusé par StrongFather [{}] : {}",
            self.regle(),
            self.raison()
        )
    }
}

impl std::error::Error for Refus {}

/// @id bondingbrother_gate_trait
/// @role infrastructure
/// @layer core
/// @do define_gate_contract
//@human / @human Contrat de la porte obligatoire. Synchrone, sans effet de bord sur l'action.
pub trait Gate {
    /// @id bondingbrother_gate_autoriser
    /// @role evaluator
    /// @layer core
    /// @do authorize_intent
    //@human / @human Soumet l'intention à StrongFather ; ALLOW ⇒ jeton, sinon refus motivé.
    fn autoriser(&self, intent: &Intent) -> Result<DecisionAutorisee, Refus>;
}

/// @id bondingbrother_porte_strongfather
/// @role service
/// @layer core
/// @do gate_with_strongfather
//@human / @human Porte adossée au moteur StrongFather. Clonage à coût constant (moteur partagé).
#[derive(Debug, Clone)]
pub struct PorteStrongFather {
    moteur: PolicyEngine,
}

impl PorteStrongFather {
    /// @id bondingbrother_porte_strongfather_new
    /// @role constructor
    /// @layer core
    /// @do create_gate_from_engine
    //@human / @human Crée la porte à partir d'un moteur déjà construit.
    #[must_use]
    pub fn new(moteur: PolicyEngine) -> Self {
        Self { moteur }
    }

    /// @id bondingbrother_porte_strongfather_from_policies
    /// @role constructor
    /// @layer core
    /// @do create_gate_from_policies
    //@human / @human Crée la porte depuis un ensemble validé ; politique invalide ⇒ erreur.
    pub fn depuis_politiques(
        politiques: PolicySet,
    ) -> Result<Self, strongfather::PolicyEngineError> {
        PolicyEngine::try_new(politiques).map(Self::new)
    }

    /// @id bondingbrother_porte_strongfather_from_file
    /// @role constructor
    /// @layer core
    /// @do create_gate_from_file
    //@human / @human Crée la porte depuis un fichier TOML / JSON ; illisible ou invalide ⇒ erreur.
    pub fn depuis_fichier(chemin: impl AsRef<Path>) -> Result<Self, PolicyLoadError> {
        PolicyEngine::load_file(chemin).map(Self::new)
    }

    /// Moteur sous-jacent (lecture seule).
    #[must_use]
    pub fn moteur(&self) -> &PolicyEngine {
        &self.moteur
    }
}

impl Gate for PorteStrongFather {
    fn autoriser(&self, intent: &Intent) -> Result<DecisionAutorisee, Refus> {
        let decision = self.moteur.decide(intent);
        if decision.verdict() == Verdict::Allow {
            // `sceller` revérifie verdict et empreinte : défense en profondeur.
            if let Some(jeton) = DecisionAutorisee::sceller(decision.clone(), intent) {
                return Ok(jeton);
            }
        }
        Err(Refus {
            decision: Box::new(decision),
        })
    }
}

impl<G: Gate + ?Sized> Gate for &G {
    fn autoriser(&self, intent: &Intent) -> Result<DecisionAutorisee, Refus> {
        (**self).autoriser(intent)
    }
}

impl<G: Gate + ?Sized> Gate for std::sync::Arc<G> {
    fn autoriser(&self, intent: &Intent) -> Result<DecisionAutorisee, Refus> {
        (**self).autoriser(intent)
    }
}

/// @id bondingbrother_executer_si_autorise
/// @role orchestration
/// @layer core
/// @do run_governed_action
//@human / @human Façon canonique d'exécuter une action gouvernée : l'action n'est appelée que si la
//@human porte autorise, exactement une fois, avec le jeton (à joindre au journal de l'action).
pub fn executer_si_autorise<G, T, F>(gate: &G, intent: &Intent, action: F) -> Result<T, Refus>
where
    G: Gate + ?Sized,
    F: FnOnce(&DecisionAutorisee) -> T,
{
    let jeton = gate.autoriser(intent)?;
    Ok(action(&jeton))
}

#[cfg(test)]
mod tests {
    use super::*;
    use strongfather::{
        ActionType, CallContext, IntentData, Policy, PolicyCondition, PolicyEffect, PolicyRule,
        PolicyType,
    };

    fn intent(caller: &str) -> Intent {
        Intent::new(
            "i-1".to_string(),
            ActionType::Read,
            "jaykonta".to_string(),
            CallContext {
                caller_identity: caller.to_string(),
                origin: "maia".to_string(),
                instance: "cog".to_string(),
            },
            IntentData {
                content: std::collections::HashMap::new(),
            },
        )
    }

    fn porte() -> PorteStrongFather {
        let mut set = PolicySet::new();
        set.add(Policy::new(
            "op-1-lit".to_string(),
            PolicyType::Permission,
            PolicyCondition {
                expression: "action == 'read'".to_string(),
            },
            PolicyRule {
                expression: "caller == 'op-1'".to_string(),
            },
            PolicyEffect::Authorize,
        ))
        .unwrap();
        PorteStrongFather::depuis_politiques(set).unwrap()
    }

    /// @id bondingbrother_gate_test_allow_deny
    /// @role test
    /// @layer core
    /// @do test_gate_allow_and_deny
    //@human / @human La porte délivre un jeton lié à l'intention, ou un refus motivé.
    #[test]
    fn test_porte_autorise_et_refuse() {
        let p = porte();
        let i = intent("op-1");
        let jeton = p.autoriser(&i).unwrap();
        assert_eq!(jeton.regle(), "op-1-lit");
        assert!(jeton.couvre(&i));
        assert!(!jeton.couvre(&intent("op-2")));

        let refus = p.autoriser(&intent("op-2")).unwrap_err();
        assert_eq!(refus.regle(), strongfather::DEFAULT_DENY_RULE);
        assert!(refus.to_string().contains("défaut: refus"));
    }

    /// @id bondingbrother_gate_test_seal
    /// @role test
    /// @layer core
    /// @do test_seal_rejects_forged_decision
    //@human / @human Une décision refusée ou sans empreinte ne peut pas être scellée en jeton.
    #[test]
    fn test_sceau_rejette_decision_forgee() {
        let i = intent("op-2");
        let refus = porte().moteur().decide(&i);
        assert!(DecisionAutorisee::sceller(refus, &i).is_none());

        let mut forgee = porte().moteur().decide(&intent("op-1"));
        forgee.intent_digest = String::new();
        assert!(DecisionAutorisee::sceller(forgee, &intent("op-1")).is_none());

        let autre = porte().moteur().decide(&intent("op-1"));
        assert!(DecisionAutorisee::sceller(autre, &i).is_none());
    }
}
