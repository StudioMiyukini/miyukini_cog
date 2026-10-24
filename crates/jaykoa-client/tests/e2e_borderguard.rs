//! Tests d'intégration P7 — `BorderGuardCoreGate` câblé dans le dispatcher.
//!
//! Valide que la règle « toute écriture qu'Alicia fait hors-limites passe par
//! les cores » est tenue par construction quand on substitue le
//! `BypassCoreGate` (dev) par le `BorderGuardCoreGate` (prod).

use std::path::Path;
use std::sync::Arc;

use serde_json::{json, Value};

use borderguard::TrustLevel;
use jaykoa_client::{EventSummary, FakeJayKoaBackend, JayKoaClient};
use miyualicia_cores_gate::{test_helpers, BorderGuardCoreGate};
use miyualicia_dispatcher::{Decision, DispatchError, Dispatcher};
use miyualicia_memory::{EpisodeOutcome, MemoryStore};
use miyualicia_skills::{discover_in_dir, AliciaPolicy, SkillRegistry};

/// Charge le manifeste réel de JayKoa dans un registre.
fn load_jaykoa_registry() -> SkillRegistry {
    let tmp = tempfile::tempdir().unwrap();
    let sub = tmp.path().join("jaykoa");
    std::fs::create_dir_all(&sub).unwrap();
    let real = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("jaykoa")
        .join("service.manifest.json");
    let bytes = std::fs::read(&real).unwrap();
    std::fs::write(sub.join("service.manifest.json"), bytes).unwrap();
    let mut reg = SkillRegistry::new();
    let (_count, errors) = discover_in_dir(tmp.path(), &mut reg).unwrap();
    assert!(errors.is_empty());
    reg
}

fn ev(id: &str, title: &str, start: &str) -> EventSummary {
    EventSummary {
        id: id.into(),
        title: title.into(),
        start: start.into(),
        end: None,
        location: None,
        all_day: false,
        source_service: None,
    }
}

#[tokio::test]
async fn trusted_persona_allowed_through_borderguard() {
    let reg = load_jaykoa_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJayKoaBackend::new());
    backend.set_next(Some(ev("e1", "Sync", "2026-05-08T09:00:00Z")));
    let client = JayKoaClient::new(backend);

    // BorderGuard : jaykoa accessible aux Verified+, voice est Trusted
    let gate = test_helpers::simple_gate(
        [("jaykoa", TrustLevel::Verified, true)],
        [("voice", TrustLevel::Trusted)],
    );

    let dispatcher = Dispatcher::new();
    let decision = dispatcher
        .execute(
            "jaykoa.next_event",
            &Value::Null,
            "voice",
            "?",
            &reg,
            &AliciaPolicy::strict(),
            &gate,
            &client,
            &mem,
        )
        .await
        .unwrap();
    assert!(matches!(decision, Decision::Executed(_)));

    let recent = mem.recent_episodes(10).unwrap();
    assert_eq!(recent[0].outcome, EpisodeOutcome::Ok);
}

#[tokio::test]
async fn unknown_persona_denied_by_borderguard() {
    let reg = load_jaykoa_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let client = JayKoaClient::new(Arc::new(FakeJayKoaBackend::new()));

    // BorderGuard exige Verified+, mais "ghost" n'est pas classé → Unknown
    let gate = test_helpers::simple_gate(
        [("jaykoa", TrustLevel::Verified, true)],
        std::iter::empty(),
    );

    let dispatcher = Dispatcher::new();
    let err = dispatcher
        .execute(
            "jaykoa.next_event",
            &Value::Null,
            "ghost",
            "?",
            &reg,
            &AliciaPolicy::strict(),
            &gate,
            &client,
            &mem,
        )
        .await
        .unwrap_err();
    assert!(matches!(err, DispatchError::CoresDenied(_)));

    // Audit posé en Failed avec le motif cores
    let recent = mem.recent_episodes(10).unwrap();
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].outcome, EpisodeOutcome::Failed);
    let error_msg = recent[0].error.as_deref().unwrap();
    assert!(error_msg.contains("cores denied"));
    assert!(error_msg.contains("Unknown") || error_msg.contains("service.jaykoa"));
}

#[tokio::test]
async fn hostile_persona_blocked_even_for_read_only() {
    let reg = load_jaykoa_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let client = JayKoaClient::new(Arc::new(FakeJayKoaBackend::new()));

    let gate = test_helpers::simple_gate(
        [("jaykoa", TrustLevel::Verified, true)],
        [("attacker", TrustLevel::Hostile)],
    );

    let dispatcher = Dispatcher::new();
    let err = dispatcher
        .execute(
            "jaykoa.list_events",
            &json!({"from": "2026-05-08", "to": "2026-05-08"}),
            "attacker",
            "?",
            &reg,
            &AliciaPolicy::strict(),
            &gate,
            &client,
            &mem,
        )
        .await
        .unwrap_err();
    assert!(matches!(err, DispatchError::CoresDenied(_)));
}

#[tokio::test]
async fn unregistered_service_boundary_denied_conservatively() {
    let reg = load_jaykoa_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let client = JayKoaClient::new(Arc::new(FakeJayKoaBackend::new()));

    // Aucune frontière enregistrée → toute exécution refusée
    let gate = test_helpers::simple_gate(
        std::iter::empty(),
        [("voice", TrustLevel::Trusted)],
    );

    let dispatcher = Dispatcher::new();
    let err = dispatcher
        .execute(
            "jaykoa.next_event",
            &Value::Null,
            "voice",
            "?",
            &reg,
            &AliciaPolicy::strict(),
            &gate,
            &client,
            &mem,
        )
        .await
        .unwrap_err();
    match err {
        DispatchError::CoresDenied(msg) => {
            assert!(msg.contains("aucune règle"));
            assert!(msg.contains("service.jaykoa"));
        }
        _ => panic!("expected CoresDenied"),
    }
}

#[tokio::test]
async fn bypass_cores_validation_overrides_borderguard_denial() {
    let reg = load_jaykoa_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJayKoaBackend::new());
    backend.set_next(Some(ev("e1", "x", "2026-05-08T09:00:00Z")));
    let client = JayKoaClient::new(backend);

    // BorderGuard refuserait normalement (aucune règle)
    let gate = test_helpers::simple_gate(std::iter::empty(), std::iter::empty());

    // Mais la policy bypass cores → on passe quand même
    let mut policy = AliciaPolicy::strict();
    policy.bypass_cores_validation = true;

    let dispatcher = Dispatcher::new();
    let decision = dispatcher
        .execute(
            "jaykoa.next_event",
            &Value::Null,
            "voice",
            "?",
            &reg,
            &policy,
            &gate,
            &client,
            &mem,
        )
        .await
        .unwrap();
    assert!(matches!(decision, Decision::Executed(_)));
}

#[tokio::test]
async fn dynamic_borderguard_uses_arc_classifier() {
    // Smoke test : on injecte un classifier custom (boxed) pour s'assurer
    // que le trait object marche bout-à-bout.
    use borderguard::TrustLevelClassifier;

    struct AlwaysTrusted;
    impl TrustLevelClassifier for AlwaysTrusted {
        fn classify(&self, _entity_id: &str) -> TrustLevel {
            TrustLevel::Trusted
        }
    }

    use borderguard::{
        Boundary, BoundaryMetadata, BoundaryRegistry, BoundaryType, CrossingRule, CrossingRules,
    };
    let mut registry = BoundaryRegistry::new();
    let bid = "service.jaykoa".to_string();
    registry.register_boundary(
        Boundary::new(bid.clone(), BoundaryType::Internal, "JayKoa".into()),
        BoundaryMetadata::new("2026-05-08T00:00:00Z".into(), "test".into(), "x".into(), 1),
    );
    let mut rules = CrossingRules::default();
    rules.add(CrossingRule::new(bid.clone(), TrustLevel::Verified, true));
    registry.register_crossing_rules(bid, rules);

    let gate = BorderGuardCoreGate::new(Arc::new(registry), Arc::new(AlwaysTrusted));

    let reg = load_jaykoa_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJayKoaBackend::new());
    backend.set_next(Some(ev("e1", "x", "2026-05-08T09:00:00Z")));
    let client = JayKoaClient::new(backend);

    let dispatcher = Dispatcher::new();
    let decision = dispatcher
        .execute(
            "jaykoa.next_event",
            &Value::Null,
            "anyone-at-all",
            "?",
            &reg,
            &AliciaPolicy::strict(),
            &gate,
            &client,
            &mem,
        )
        .await
        .unwrap();
    assert!(matches!(decision, Decision::Executed(_)));
}
