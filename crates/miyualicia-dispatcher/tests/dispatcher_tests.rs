//! Tests d'intégration end-to-end du dispatcher Alicia.

use async_trait::async_trait;
use serde_json::{json, Value};
use std::collections::BTreeMap;

use miyualicia_dispatcher::{
    BypassCoreGate, CoreGate, CoreGateOutcome, Decision, DispatchError, Dispatcher,
    MockServiceClient, ResolvedLearnedCall, ServiceCallResult,
};
use miyualicia_memory::{EpisodeOutcome, MemoryStore};
use miyualicia_skills::{
    AliciaPolicy, ConfirmationPolicy, ParamSchema, ParamType, ServiceCapability, Skill,
    SkillRegistry,
};

fn schedule_event_capability() -> ServiceCapability {
    let mut params = BTreeMap::new();
    params.insert("title".into(), ParamSchema::required_of(ParamType::String));
    ServiceCapability {
        service_id: "jaykoa".into(),
        intent: "schedule_event".into(),
        phrases_examples: vec!["ajoute une réunion".into()],
        params,
        writes: true,
        confirmation_default: ConfirmationPolicy::AlwaysForWrites,
        description: Some("Programme un événement.".into()),
        category: Some("calendar".into()),
    }
}

fn list_today_capability() -> ServiceCapability {
    ServiceCapability {
        service_id: "jaykoa".into(),
        intent: "list_today".into(),
        phrases_examples: vec!["mon agenda du jour".into()],
        params: BTreeMap::new(),
        writes: false,
        confirmation_default: ConfirmationPolicy::Never,
        description: Some("Liste les événements du jour.".into()),
        category: Some("calendar".into()),
    }
}

/// Capability avec ConfirmationPolicy::UserPreference — utilisée en P4 pour
/// tester la résolution dynamique via MemoryStore.
fn user_pref_capability() -> ServiceCapability {
    ServiceCapability {
        service_id: "jaykoa".into(),
        intent: "delete_event".into(),
        phrases_examples: vec!["supprime l'événement".into()],
        params: BTreeMap::new(),
        writes: true,
        confirmation_default: ConfirmationPolicy::UserPreference,
        description: Some("Supprime un événement.".into()),
        category: Some("calendar".into()),
    }
}

fn registry_with(caps: Vec<ServiceCapability>) -> SkillRegistry {
    let mut reg = SkillRegistry::new();
    for c in caps {
        reg.register(Skill {
            capability: c,
            source: "test".into(),
            registered_at: "2026-05-08T00:00:00Z".into(),
        })
        .unwrap();
    }
    reg
}

#[tokio::test]
async fn read_capability_executes_immediately_under_strict_policy() {
    let reg = registry_with(vec![list_today_capability()]);
    let mem = MemoryStore::open_in_memory().unwrap();
    let gate = BypassCoreGate;
    let svc = MockServiceClient::new();
    svc.expect("jaykoa.list_today", ServiceCallResult::ok(json!({"events": []})));

    let d = Dispatcher::new();
    let decision = d
        .execute(
            "jaykoa.list_today",
            &Value::Null,
            "voice",
            "mon agenda du jour",
            &reg,
            &AliciaPolicy::strict(),
            &gate,
            &svc,
            &mem,
        )
        .await
        .unwrap();

    match decision {
        Decision::Executed(r) => {
            assert_eq!(r.qualified_id, "jaykoa.list_today");
            assert!(r.service_result.success);
        }
        Decision::PendingConfirmation { .. } => panic!("expected Executed"),
    }

    // Audit posé
    let recent = mem.recent_episodes(10).unwrap();
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].outcome, EpisodeOutcome::Ok);
}

#[tokio::test]
async fn write_capability_returns_pending_under_strict_policy() {
    let reg = registry_with(vec![schedule_event_capability()]);
    let mem = MemoryStore::open_in_memory().unwrap();
    let gate = BypassCoreGate;
    let svc = MockServiceClient::new();

    let d = Dispatcher::new();
    let decision = d
        .execute(
            "jaykoa.schedule_event",
            &json!({"title": "x"}),
            "voice",
            "ajoute une réunion",
            &reg,
            &AliciaPolicy::strict(),
            &gate,
            &svc,
            &mem,
        )
        .await
        .unwrap();

    assert!(matches!(decision, Decision::PendingConfirmation { .. }));
    // Aucun appel service ni épisode
    assert_eq!(svc.calls(), [] as [(std::string::String, serde_json::Value); 0]);
    assert_eq!(mem.recent_episodes(10).unwrap(), [] as [miyualicia_memory::Episode; 0]);
}

#[tokio::test]
async fn execute_confirmed_runs_after_consent() {
    let reg = registry_with(vec![schedule_event_capability()]);
    let mem = MemoryStore::open_in_memory().unwrap();
    let gate = BypassCoreGate;
    let svc = MockServiceClient::new();
    svc.expect(
        "jaykoa.schedule_event",
        ServiceCallResult::ok(json!({"id": "evt_42"})),
    );

    let d = Dispatcher::new();
    let r = d
        .execute_confirmed(
            "jaykoa.schedule_event",
            &json!({"title": "x"}),
            "voice",
            "ajoute une réunion",
            &reg,
            &AliciaPolicy::strict(),
            &gate,
            &svc,
            &mem,
        )
        .await
        .unwrap();
    assert_eq!(r.service_result.data["id"], "evt_42");
    let recent = mem.recent_episodes(10).unwrap();
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].outcome, EpisodeOutcome::Ok);
}

#[tokio::test]
async fn execute_cancelled_records_cancelled_episode() {
    let _reg = registry_with(vec![schedule_event_capability()]);
    let mem = MemoryStore::open_in_memory().unwrap();

    let d = Dispatcher::new();
    d.execute_cancelled(
        "jaykoa.schedule_event",
        &json!({}),
        "voice",
        "ajoute une réunion",
        &mem,
    )
    .unwrap();

    let recent = mem.recent_episodes(10).unwrap();
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].outcome, EpisodeOutcome::Cancelled);
}

#[tokio::test]
async fn permissive_policy_skips_confirmation() {
    let reg = registry_with(vec![schedule_event_capability()]);
    let mem = MemoryStore::open_in_memory().unwrap();
    let gate = BypassCoreGate;
    let svc = MockServiceClient::new();
    svc.expect(
        "jaykoa.schedule_event",
        ServiceCallResult::ok(json!({"id": "evt_1"})),
    );

    let d = Dispatcher::new();
    let decision = d
        .execute(
            "jaykoa.schedule_event",
            &json!({"title": "x"}),
            "voice",
            "ajoute",
            &reg,
            &AliciaPolicy::permissive(),
            &gate,
            &svc,
            &mem,
        )
        .await
        .unwrap();

    match decision {
        Decision::Executed(_) => {}
        Decision::PendingConfirmation { .. } => panic!("expected Executed under permissive"),
    }
}

#[tokio::test]
async fn unknown_capability_returns_error() {
    let reg = SkillRegistry::new();
    let mem = MemoryStore::open_in_memory().unwrap();
    let gate = BypassCoreGate;
    let svc = MockServiceClient::new();

    let d = Dispatcher::new();
    let err = d
        .execute(
            "ghost.unknown",
            &Value::Null,
            "voice",
            "?",
            &reg,
            &AliciaPolicy::strict(),
            &gate,
            &svc,
            &mem,
        )
        .await
        .unwrap_err();
    assert!(matches!(err, DispatchError::UnknownCapability(_)));
}

// ----- CoreGate denial -----

struct DenyAllGate;

#[async_trait]
impl CoreGate for DenyAllGate {
    async fn validate(
        &self,
        _capability: &ServiceCapability,
        _params: &Value,
        _persona: &str,
    ) -> CoreGateOutcome {
        CoreGateOutcome::Denied("politique cores : mode lecture seule".into())
    }
}

#[tokio::test]
async fn cores_denial_records_failed_episode_and_errors() {
    let reg = registry_with(vec![list_today_capability()]);
    let mem = MemoryStore::open_in_memory().unwrap();
    let svc = MockServiceClient::new();

    let d = Dispatcher::new();
    let err = d
        .execute(
            "jaykoa.list_today",
            &Value::Null,
            "voice",
            "?",
            &reg,
            &AliciaPolicy::strict(),
            &DenyAllGate,
            &svc,
            &mem,
        )
        .await
        .unwrap_err();
    assert!(matches!(err, DispatchError::CoresDenied(_)));
    let recent = mem.recent_episodes(10).unwrap();
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].outcome, EpisodeOutcome::Failed);
    assert!(recent[0].error.as_deref().unwrap().contains("cores denied"));
    // Le service n'a pas été appelé
    assert_eq!(svc.calls(), [] as [(std::string::String, serde_json::Value); 0]);
}

#[tokio::test]
async fn bypass_cores_validation_skips_gate() {
    let reg = registry_with(vec![list_today_capability()]);
    let mem = MemoryStore::open_in_memory().unwrap();
    let svc = MockServiceClient::new();
    svc.expect("jaykoa.list_today", ServiceCallResult::ok(json!([])));

    let mut policy = AliciaPolicy::strict();
    policy.bypass_cores_validation = true;

    let d = Dispatcher::new();
    let decision = d
        .execute(
            "jaykoa.list_today",
            &Value::Null,
            "voice",
            "?",
            &reg,
            &policy,
            // DenyAllGate ne devrait jamais être appelé puisque la gate est bypassée
            &DenyAllGate,
            &svc,
            &mem,
        )
        .await
        .unwrap();
    match decision {
        Decision::Executed(_) => {}
        Decision::PendingConfirmation { .. } => panic!("expected Executed under bypass_cores"),
    }
    // Service appelé malgré DenyAllGate (bypass actif)
    assert_eq!(svc.calls().len(), 1);
}

#[tokio::test]
async fn service_failure_records_failed_episode() {
    let reg = registry_with(vec![list_today_capability()]);
    let mem = MemoryStore::open_in_memory().unwrap();
    let gate = BypassCoreGate;
    let svc = MockServiceClient::new();
    svc.expect("jaykoa.list_today", ServiceCallResult::fail("backend HS"));

    let d = Dispatcher::new();
    let err = d
        .execute(
            "jaykoa.list_today",
            &Value::Null,
            "voice",
            "?",
            &reg,
            &AliciaPolicy::strict(),
            &gate,
            &svc,
            &mem,
        )
        .await
        .unwrap_err();
    assert!(matches!(err, DispatchError::ServiceFailed { .. }));
    let recent = mem.recent_episodes(10).unwrap();
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].outcome, EpisodeOutcome::Failed);
    assert_eq!(recent[0].error.as_deref().unwrap(), "backend HS");
}

// ----------------------------------------------------------------------
// P4 — ConfirmationPolicy::UserPreference avec consultation MemoryStore
// ----------------------------------------------------------------------

#[tokio::test]
async fn user_pref_capability_defaults_to_pending_when_no_pref_set() {
    let reg = registry_with(vec![user_pref_capability()]);
    let mem = MemoryStore::open_in_memory().unwrap();
    let svc = MockServiceClient::new();

    let d = Dispatcher::new();
    let decision = d
        .execute(
            "jaykoa.delete_event",
            &Value::Null,
            "voice",
            "supprime",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &svc,
            &mem,
        )
        .await
        .unwrap();
    assert!(matches!(decision, Decision::PendingConfirmation { .. }));
    // Aucun appel service ni épisode (pas encore confirmé)
    assert_eq!(svc.calls(), [] as [(std::string::String, serde_json::Value); 0]);
}

#[tokio::test]
async fn user_pref_with_skip_preference_executes_immediately() {
    let reg = registry_with(vec![user_pref_capability()]);
    let mem = MemoryStore::open_in_memory().unwrap();
    let svc = MockServiceClient::new();
    svc.expect(
        "jaykoa.delete_event",
        ServiceCallResult::ok(json!({"deleted": "evt_1"})),
    );

    // L'utilisateur a posé la préférence : ne pas demander pour cette capability
    mem.set_confirmation_preference("voice", "jaykoa.delete_event", false)
        .unwrap();

    let d = Dispatcher::new();
    let decision = d
        .execute(
            "jaykoa.delete_event",
            &Value::Null,
            "voice",
            "supprime",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &svc,
            &mem,
        )
        .await
        .unwrap();
    match decision {
        Decision::Executed(r) => assert_eq!(r.service_result.data["deleted"], "evt_1"),
        Decision::PendingConfirmation { .. } => panic!("expected Executed (préférence = ne pas demander)"),
    }
    assert_eq!(svc.calls().len(), 1);
}

#[tokio::test]
async fn user_pref_with_ask_preference_returns_pending() {
    let reg = registry_with(vec![user_pref_capability()]);
    let mem = MemoryStore::open_in_memory().unwrap();
    let svc = MockServiceClient::new();
    mem.set_confirmation_preference("voice", "jaykoa.delete_event", true)
        .unwrap();

    let d = Dispatcher::new();
    let decision = d
        .execute(
            "jaykoa.delete_event",
            &Value::Null,
            "voice",
            "supprime",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &svc,
            &mem,
        )
        .await
        .unwrap();
    assert!(matches!(decision, Decision::PendingConfirmation { .. }));
}

#[tokio::test]
async fn user_pref_persona_scope_takes_priority_over_global() {
    let reg = registry_with(vec![user_pref_capability()]);
    let mem = MemoryStore::open_in_memory().unwrap();
    let svc = MockServiceClient::new();
    svc.expect("jaykoa.delete_event", ServiceCallResult::ok(json!({})));

    // Global = demander, persona = ne pas demander → la persona gagne
    mem.set_confirmation_preference("global", "jaykoa.delete_event", true)
        .unwrap();
    mem.set_confirmation_preference("voice", "jaykoa.delete_event", false)
        .unwrap();

    let d = Dispatcher::new();
    let decision = d
        .execute(
            "jaykoa.delete_event",
            &Value::Null,
            "voice",
            "supprime",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &svc,
            &mem,
        )
        .await
        .unwrap();
    assert!(matches!(decision, Decision::Executed(_)));

    // Pour un autre persona sans override → fallback global = demander
    let decision2 = d
        .execute(
            "jaykoa.delete_event",
            &Value::Null,
            "work",
            "supprime",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &svc,
            &mem,
        )
        .await
        .unwrap();
    assert!(matches!(decision2, Decision::PendingConfirmation { .. }));
}

#[tokio::test]
async fn bypass_user_confirmations_overrides_user_preference() {
    let reg = registry_with(vec![user_pref_capability()]);
    let mem = MemoryStore::open_in_memory().unwrap();
    let svc = MockServiceClient::new();
    svc.expect("jaykoa.delete_event", ServiceCallResult::ok(json!({})));
    // L'utilisateur a explicitement demandé à confirmer
    mem.set_confirmation_preference("voice", "jaykoa.delete_event", true)
        .unwrap();

    // Mais la policy bypass UX → le bypass gagne
    let mut policy = AliciaPolicy::strict();
    policy.bypass_user_confirmations = true;

    let d = Dispatcher::new();
    let decision = d
        .execute(
            "jaykoa.delete_event",
            &Value::Null,
            "voice",
            "supprime",
            &reg,
            &policy,
            &BypassCoreGate,
            &svc,
            &mem,
        )
        .await
        .unwrap();
    assert!(matches!(decision, Decision::Executed(_)));
}

#[tokio::test]
async fn should_confirm_helper_matrix() {
    let mem = MemoryStore::open_in_memory().unwrap();
    let d = Dispatcher::new();
    let policy = AliciaPolicy::strict();

    // Never → false
    let mut cap = list_today_capability();
    cap.confirmation_default = ConfirmationPolicy::Never;
    assert!(!d
        .should_confirm(&cap, "voice", &policy, &mem)
        .unwrap());

    // Always → true
    cap.confirmation_default = ConfirmationPolicy::Always;
    assert!(d.should_confirm(&cap, "voice", &policy, &mem).unwrap());

    // AlwaysForWrites + writes=false → false
    cap.confirmation_default = ConfirmationPolicy::AlwaysForWrites;
    cap.writes = false;
    assert!(!d
        .should_confirm(&cap, "voice", &policy, &mem)
        .unwrap());

    // AlwaysForWrites + writes=true → true
    cap.writes = true;
    assert!(d.should_confirm(&cap, "voice", &policy, &mem).unwrap());

    // UserPreference, rien posé → true (conservateur)
    cap.confirmation_default = ConfirmationPolicy::UserPreference;
    cap.service_id = "jaykoa".into();
    cap.intent = "delete_event".into();
    assert!(d.should_confirm(&cap, "voice", &policy, &mem).unwrap());

    // UserPreference, pref = false → false
    mem.set_confirmation_preference("voice", "jaykoa.delete_event", false)
        .unwrap();
    assert!(!d
        .should_confirm(&cap, "voice", &policy, &mem)
        .unwrap());
}

// ----------------------------------------------------------------------
// P5.4 — resolve_learned_phrase (raccourci avant LLM)
// ----------------------------------------------------------------------

#[tokio::test]
async fn resolve_learned_phrase_returns_none_when_nothing_matches() {
    let mem = MemoryStore::open_in_memory().unwrap();
    let d = Dispatcher::new();
    let r = d
        .resolve_learned_phrase("anything", "voice", &mem)
        .unwrap();
    assert!(r.is_none());
}

#[tokio::test]
async fn resolve_learned_phrase_returns_match_with_template_params() {
    let mem = MemoryStore::open_in_memory().unwrap();
    let ls = mem
        .propose_learned_skill(
            "voice",
            "12 € café",
            "jaykonta.add_expense",
            json!({"amount": 12, "category": "café"}),
        )
        .unwrap();
    mem.accept_learned_skill(ls.id).unwrap();

    let d = Dispatcher::new();
    let resolved = d
        .resolve_learned_phrase("12 € café", "voice", &mem)
        .unwrap()
        .expect("match");
    assert_eq!(
        resolved,
        ResolvedLearnedCall {
            learned_skill_id: ls.id,
            qualified_id: "jaykonta.add_expense".into(),
            params: json!({"amount": 12, "category": "café"}),
            trigger_phrase: "12 € café".into(),
        }
    );
}

#[tokio::test]
async fn resolve_learned_phrase_then_execute_dispatches_normally() {
    let mut reg = SkillRegistry::new();
    reg.register(Skill {
        capability: ServiceCapability {
            service_id: "jaykonta".into(),
            intent: "add_expense".into(),
            phrases_examples: vec![],
            params: BTreeMap::new(),
            writes: true,
            confirmation_default: ConfirmationPolicy::Never, // pas de confirm pour test direct
            description: Some("Ajoute une dépense.".into()),
            category: None,
        },
        source: "test".into(),
        registered_at: "2026-05-08T00:00:00Z".into(),
    })
    .unwrap();

    let mem = MemoryStore::open_in_memory().unwrap();
    let svc = MockServiceClient::new();
    svc.expect(
        "jaykonta.add_expense",
        ServiceCallResult::ok(json!({"id": "e1"})),
    );

    // Apprentissage : 12€ café (proposé + accepté)
    let ls = mem
        .propose_learned_skill(
            "voice",
            "12 € café",
            "jaykonta.add_expense",
            json!({"amount": 12, "category": "café"}),
        )
        .unwrap();
    mem.accept_learned_skill(ls.id).unwrap();

    // Runtime : utilisateur dit la phrase → resolve + dispatch
    let d = Dispatcher::new();
    let resolved = d
        .resolve_learned_phrase("12 € café", "voice", &mem)
        .unwrap()
        .expect("match");
    let decision = d
        .execute(
            &resolved.qualified_id,
            &resolved.params,
            "voice",
            "12 € café",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &svc,
            &mem,
        )
        .await
        .unwrap();
    assert!(matches!(decision, Decision::Executed(_)));

    // Le service a bien été appelé avec les params templates
    let calls = svc.calls();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].0, "jaykonta.add_expense");
    assert_eq!(calls[0].1["amount"], 12);
    assert_eq!(calls[0].1["category"], "café");

    // Audit posé
    let recent = mem.recent_episodes(10).unwrap();
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].outcome, EpisodeOutcome::Ok);
    assert_eq!(recent[0].user_phrase, "12 € café");
}
