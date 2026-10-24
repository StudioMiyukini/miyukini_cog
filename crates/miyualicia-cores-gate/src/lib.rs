//! # miyualicia-cores-gate
//!
//! Implémentation concrète du trait `CoreGate` du dispatcher Alicia, branchée
//! sur **BorderGuard**.
//!
//! ## Convention de mapping
//!
//! - Chaque service COG expose une **frontière BorderGuard** identifiée
//!   `service.<service_id>` (ex. `service.jaykoa`, `service.jaykonta`).
//! - Le **persona** active sous lequel Alicia agit est l'**entité** soumise
//!   au [`TrustLevelClassifier`].
//! - La validation consulte les `CrossingRules` enregistrés pour la frontière
//!   et autorise si :
//!     - le `trust(persona)` est `>= min_trust_level` d'une règle, **et**
//!     - cette règle a `allowed = true`.
//!
//! Sinon le franchissement est refusé avec un message lisible (qui sera
//! repris dans l'`Episode` audité par le dispatcher).
//!
//! ## Lois d'autonomie respectées
//!
//! - Le crate ne fait que **consulter** BorderGuard (`get_crossing_rules`),
//!   conforme à INV-BG-2 (« BorderGuard définit, n'applique pas »).
//! - Aucune dépendance réseau, aucun appel persistant : registre + classifier
//!   sont injectés par le caller (typiquement le runtime Alicia qui les a
//!   chargés depuis KindMother).

#![doc(html_root_url = "https://docs.miyukini.com/miyualicia-cores-gate")]

// @id service.alicia.cores_gate.lib
// @role cores_gate_borderguard_impl
// @layer 7
// @do borderguard_core_gate
// @human Bridge BorderGuard ↔ trait CoreGate du dispatcher Alicia.

use std::sync::Arc;

use async_trait::async_trait;

use borderguard::{BoundaryRegistry, TrustLevel, TrustLevelClassifier};
use miyualicia_dispatcher::{CoreGate, CoreGateOutcome};
use miyualicia_skills::ServiceCapability;
use serde_json::Value;

/// Préfixe de l'identifiant BorderGuard pour les frontières de services COG.
pub const SERVICE_BOUNDARY_PREFIX: &str = "service.";

/// Implémentation `CoreGate` qui interroge un `BoundaryRegistry` BorderGuard
/// avant chaque exécution.
///
/// Le `registry` et le `classifier` sont passés en `Arc` pour permettre :
/// - le partage entre threads (`async-trait` exige `Send + Sync`)
/// - la mise à jour live du registre par un autre composant runtime sans
///   reconstruction du gate
pub struct BorderGuardCoreGate {
    registry: Arc<BoundaryRegistry>,
    classifier: Arc<dyn TrustLevelClassifier + Send + Sync>,
}

impl BorderGuardCoreGate {
    /// Construit un gate à partir d'un registre et d'un classificateur.
    #[must_use]
    pub fn new(
        registry: Arc<BoundaryRegistry>,
        classifier: Arc<dyn TrustLevelClassifier + Send + Sync>,
    ) -> Self {
        Self {
            registry,
            classifier,
        }
    }

    /// Identifiant de frontière correspondant à un `service_id`.
    #[must_use]
    pub fn boundary_id_for(service_id: &str) -> String {
        format!("{SERVICE_BOUNDARY_PREFIX}{service_id}")
    }
}

#[async_trait]
impl CoreGate for BorderGuardCoreGate {
    async fn validate(
        &self,
        capability: &ServiceCapability,
        _params: &Value,
        persona: &str,
    ) -> CoreGateOutcome {
        let boundary_id = Self::boundary_id_for(&capability.service_id);

        let rules = match self.registry.get_crossing_rules(&boundary_id) {
            Some(r) => r,
            None => {
                tracing::warn!(
                    %boundary_id,
                    persona = %persona,
                    qualified_id = %capability.qualified_id(),
                    "aucune règle BorderGuard pour la frontière — refus conservateur"
                );
                return CoreGateOutcome::Denied(format!(
                    "aucune règle de franchissement pour '{boundary_id}'"
                ));
            }
        };

        let trust = self.classifier.classify(persona);

        // Une règle « passe » si trust(persona) >= min_trust ET allowed=true.
        // S'il existe au moins une telle règle pour ce niveau de confiance,
        // on autorise. Sinon refus avec contexte.
        let passes = rules
            .rules
            .iter()
            .any(|r| trust >= r.min_trust_level && r.allowed);

        if passes {
            CoreGateOutcome::Allowed
        } else {
            tracing::info!(
                %boundary_id,
                persona = %persona,
                ?trust,
                qualified_id = %capability.qualified_id(),
                writes = capability.writes,
                "BorderGuard refuse le franchissement"
            );
            CoreGateOutcome::Denied(format!(
                "trust '{trust:?}' insuffisant pour franchir '{boundary_id}'"
            ))
        }
    }
}

// ----------------------------------------------------------------------
// Builder helper pour les tests
// ----------------------------------------------------------------------

/// Helper de test : construit un `BorderGuardCoreGate` complet en quelques
/// lignes (registre + classifier + une règle).
///
/// Usage typique :
/// ```ignore
/// use miyualicia_cores_gate::test_helpers::*;
/// use borderguard::TrustLevel;
/// let gate = simple_gate(
///     [("jaykoa", TrustLevel::Verified, true)],
///     [("voice", TrustLevel::Trusted)],
/// );
/// ```
pub mod test_helpers {
    use super::*;
    use borderguard::{Boundary, BoundaryMetadata, BoundaryType, CrossingRule, CrossingRules, DefaultTrustLevelClassifier};

    /// Construit un gate avec un set minimal de règles et de personas.
    ///
    /// `services` : pour chaque `(service_id, min_trust, allowed)`, enregistre
    /// la frontière `service.<id>` et une `CrossingRule` correspondante.
    /// `personas` : pour chaque `(persona, trust_level)`, enregistre la
    /// classification dans le `DefaultTrustLevelClassifier`.
    pub fn simple_gate(
        services: impl IntoIterator<Item = (&'static str, TrustLevel, bool)>,
        personas: impl IntoIterator<Item = (&'static str, TrustLevel)>,
    ) -> BorderGuardCoreGate {
        let mut registry = BoundaryRegistry::new();
        for (service_id, min_trust, allowed) in services {
            let boundary_id = BorderGuardCoreGate::boundary_id_for(service_id);
            let boundary = Boundary::new(
                boundary_id.clone(),
                BoundaryType::Internal,
                format!("frontière du service {service_id}"),
            );
            let meta = BoundaryMetadata::new(
                "2026-05-08T00:00:00Z".into(),
                "test".into(),
                "test boundary".into(),
                1,
            );
            registry.register_boundary(boundary, meta);
            let mut rules = CrossingRules::default();
            rules.add(CrossingRule::new(boundary_id.clone(), min_trust, allowed));
            registry.register_crossing_rules(boundary_id, rules);
        }

        let mut classifier = DefaultTrustLevelClassifier::new();
        for (persona, level) in personas {
            classifier.register(persona.into(), level);
        }
        BorderGuardCoreGate::new(Arc::new(registry), Arc::new(classifier))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    use miyualicia_skills::{ConfirmationPolicy, ServiceCapability};

    fn capability(service: &str, intent: &str, writes: bool) -> ServiceCapability {
        ServiceCapability {
            service_id: service.into(),
            intent: intent.into(),
            phrases_examples: vec![],
            params: BTreeMap::new(),
            writes,
            confirmation_default: ConfirmationPolicy::Never,
            description: None,
            category: None,
        }
    }

    #[tokio::test]
    async fn allowed_when_trust_meets_min_level() {
        let gate = test_helpers::simple_gate(
            [("jaykoa", TrustLevel::Verified, true)],
            [("voice", TrustLevel::Trusted)],
        );
        let cap = capability("jaykoa", "list_events", false);
        let outcome = gate.validate(&cap, &Value::Null, "voice").await;
        assert_eq!(outcome, CoreGateOutcome::Allowed);
    }

    #[tokio::test]
    async fn denied_when_trust_below_min_level() {
        let gate = test_helpers::simple_gate(
            [("jaykoa", TrustLevel::Trusted, true)],
            [("voice", TrustLevel::Verified)],
        );
        let cap = capability("jaykoa", "list_events", false);
        let outcome = gate.validate(&cap, &Value::Null, "voice").await;
        match outcome {
            CoreGateOutcome::Denied(msg) => assert!(msg.contains("Verified")),
            CoreGateOutcome::Allowed => panic!("expected Denied"),
        }
    }

    #[tokio::test]
    async fn denied_when_rule_explicitly_disallows() {
        let gate = test_helpers::simple_gate(
            [("jaykoa", TrustLevel::Unknown, false)],
            [("voice", TrustLevel::Trusted)],
        );
        let cap = capability("jaykoa", "list_events", false);
        let outcome = gate.validate(&cap, &Value::Null, "voice").await;
        assert!(matches!(outcome, CoreGateOutcome::Denied(_)));
    }

    #[tokio::test]
    async fn denied_when_no_rules_for_boundary() {
        // Registre vide → pas de règles pour service.jaykoa
        let gate = test_helpers::simple_gate(
            std::iter::empty(),
            [("voice", TrustLevel::Trusted)],
        );
        let cap = capability("jaykoa", "list_events", false);
        let outcome = gate.validate(&cap, &Value::Null, "voice").await;
        match outcome {
            CoreGateOutcome::Denied(msg) => {
                assert!(msg.contains("service.jaykoa"));
                assert!(msg.contains("aucune règle"));
            }
            CoreGateOutcome::Allowed => panic!("absence de règles → refus conservateur"),
        }
    }

    #[tokio::test]
    async fn unknown_persona_falls_back_to_unknown_trust() {
        // Aucune entrée pour "ghost" dans le classifier → Unknown
        let gate = test_helpers::simple_gate(
            [("jaykoa", TrustLevel::Verified, true)],
            std::iter::empty(),
        );
        let cap = capability("jaykoa", "list_events", false);
        let outcome = gate.validate(&cap, &Value::Null, "ghost").await;
        // Unknown < Verified → refus
        assert!(matches!(outcome, CoreGateOutcome::Denied(_)));
    }

    #[tokio::test]
    async fn hostile_persona_always_denied_unless_explicit_allow() {
        let gate = test_helpers::simple_gate(
            [("jaykoa", TrustLevel::Trusted, true)],
            [("attacker", TrustLevel::Hostile)],
        );
        let cap = capability("jaykoa", "delete_event", true);
        let outcome = gate.validate(&cap, &Value::Null, "attacker").await;
        assert!(matches!(outcome, CoreGateOutcome::Denied(_)));
    }

    #[tokio::test]
    async fn most_permissive_rule_wins_when_multiple() {
        // Rule 1: Trusted, allowed=true. Rule 2: Verified, allowed=false
        // → Verified persona doit être autorisé via Rule 1 seulement si
        //   trust >= Trusted, donc refusé. Mais Trusted persona doit passer.
        use borderguard::{
            Boundary, BoundaryMetadata, BoundaryType, CrossingRule, CrossingRules,
            DefaultTrustLevelClassifier,
        };
        let mut registry = BoundaryRegistry::new();
        let bid = BorderGuardCoreGate::boundary_id_for("jaykoa");
        registry.register_boundary(
            Boundary::new(bid.clone(), BoundaryType::Internal, "jaykoa".into()),
            BoundaryMetadata::new("2026-05-08T00:00:00Z".into(), "test".into(), "x".into(), 1),
        );
        let mut rules = CrossingRules::default();
        rules.add(CrossingRule::new(bid.clone(), TrustLevel::Trusted, true));
        rules.add(CrossingRule::new(bid.clone(), TrustLevel::Verified, false));
        registry.register_crossing_rules(bid, rules);

        let mut classifier = DefaultTrustLevelClassifier::new();
        classifier.register("alice".into(), TrustLevel::Trusted);
        classifier.register("bob".into(), TrustLevel::Verified);
        let gate = BorderGuardCoreGate::new(Arc::new(registry), Arc::new(classifier));
        let cap = capability("jaykoa", "list_events", false);

        // alice = Trusted → matche Rule 1 (allowed=true)
        let r1 = gate.validate(&cap, &Value::Null, "alice").await;
        assert_eq!(r1, CoreGateOutcome::Allowed);

        // bob = Verified → matche Rule 2 (allowed=false), pas Rule 1
        let r2 = gate.validate(&cap, &Value::Null, "bob").await;
        assert!(matches!(r2, CoreGateOutcome::Denied(_)));
    }

    #[test]
    fn boundary_id_for_uses_prefix() {
        assert_eq!(
            BorderGuardCoreGate::boundary_id_for("jaykoa"),
            "service.jaykoa"
        );
        assert_eq!(
            BorderGuardCoreGate::boundary_id_for("jaykonta"),
            "service.jaykonta"
        );
    }
}
