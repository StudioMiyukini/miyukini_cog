//! Gate **cores** — validation de gouvernance avant qu'Alicia n'exécute une
//! capability hors-limites.
//!
//! Cf. mémoire projet `Alicia — modifications hors-limites validées par les
//! cores` : toute écriture qu'Alicia fait hors de son propre périmètre doit
//! traverser cet arbitrage.
//!
//! Ce module définit le contrat ; les implémentations réelles (branchement sur
//! `BorderGuard`, `StrongFather`, etc.) viennent en P3+.

// @id service.alicia.dispatcher.core_gate
// @role cores_governance_gate
// @layer 7
// @do declare_core_gate
// @human Trait CoreGate + impl bypass pour traverser la gouvernance cores avant exécution skill.

use async_trait::async_trait;
use serde_json::Value;

use miyualicia_skills::ServiceCapability;

/// Décision rendue par les cores sur une demande d'exécution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreGateOutcome {
    /// Les cores autorisent l'exécution.
    Allowed,
    /// Les cores refusent — la `String` porte le motif loggable / affichable.
    Denied(String),
}

/// Trait async décrivant la traversée des cores avant exécution.
///
/// Une vraie implémentation (P3+) traversera :
/// - `BorderGuard` pour le contrôle d'accès (qui parle, à quel service)
/// - `StrongFather` pour la décision arbitrale (politique en vigueur)
/// - le core métier compétent selon la nature de l'action (`writes`, données
///   touchées, etc.)
///
/// L'API reste minimale en P2 : capability + params + persona, retour
/// allow/deny. Les détails plus fins (raison structurée, suggestion
/// alternative...) viendront avec les vraies impls.
#[async_trait]
pub trait CoreGate: Send + Sync {
    /// Soumet la demande aux cores. Doit être idempotente côté lecture
    /// (ne pas modifier l'état).
    async fn validate(
        &self,
        capability: &ServiceCapability,
        params: &Value,
        persona: &str,
    ) -> CoreGateOutcome;
}

/// Implémentation **bypass** — autorise toujours.
///
/// **À n'utiliser que** :
/// - quand `AliciaPolicy.bypass_cores_validation == true` (dev / sandbox)
/// - dans les tests unitaires qui ne valident pas la gouvernance
///
/// Le dispatcher ne sélectionne pas automatiquement ce gate — c'est au caller
/// de fournir l'instance correcte selon `AliciaPolicy`.
#[derive(Debug, Clone, Copy, Default)]
pub struct BypassCoreGate;

#[async_trait]
impl CoreGate for BypassCoreGate {
    async fn validate(
        &self,
        _capability: &ServiceCapability,
        _params: &Value,
        _persona: &str,
    ) -> CoreGateOutcome {
        tracing::warn!(
            "BypassCoreGate utilisé — gouvernance cores court-circuitée"
        );
        CoreGateOutcome::Allowed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyualicia_skills::{ConfirmationPolicy, ServiceCapability};
    use std::collections::BTreeMap;

    fn cap() -> ServiceCapability {
        ServiceCapability {
            service_id: "jaykoa".into(),
            intent: "schedule_event".into(),
            phrases_examples: vec![],
            params: BTreeMap::new(),
            writes: true,
            confirmation_default: ConfirmationPolicy::AlwaysForWrites,
            description: None,
            category: None,
        }
    }

    #[tokio::test]
    async fn bypass_always_allows() {
        let gate = BypassCoreGate;
        let outcome = gate.validate(&cap(), &serde_json::Value::Null, "voice").await;
        assert_eq!(outcome, CoreGateOutcome::Allowed);
    }
}
