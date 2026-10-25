//! Tests E2E : manifeste réel JayKonta → registry → dispatcher → JayKontaClient.

use std::path::Path;
use std::sync::Arc;

use serde_json::{json, Value};

use jaykonta_client::{FakeJayKontaBackend, JayKontaClient};
use miyualicia_dispatcher::{BypassCoreGate, Decision, Dispatcher};
use miyualicia_memory::{EpisodeOutcome, MemoryStore};
use miyualicia_skills::{discover_in_dir, AliciaPolicy, SkillRegistry};

fn load_jaykonta_registry() -> SkillRegistry {
    let tmp = tempfile::tempdir().unwrap();
    let sub = tmp.path().join("jaykonta");
    std::fs::create_dir_all(&sub).unwrap();
    let real = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("jaykonta")
        .join("service.manifest.json");
    let bytes = std::fs::read(&real).unwrap();
    std::fs::write(sub.join("service.manifest.json"), bytes).unwrap();
    let mut reg = SkillRegistry::new();
    let (_count, errors) = discover_in_dir(tmp.path(), &mut reg).unwrap();
    assert!(errors.is_empty(), "discovery errors: {errors:?}");
    reg
}

#[tokio::test]
async fn manifest_loads_three_capabilities() {
    let reg = load_jaykonta_registry();
    assert!(reg.get("jaykonta.add_movement").is_some());
    assert!(reg.get("jaykonta.list_recent_movements").is_some());
    assert!(reg.get("jaykonta.get_balance").is_some());

    let add = &reg.get("jaykonta.add_movement").unwrap().capability;
    assert!(add.writes);
    assert_eq!(add.params.len(), 5);
    assert!(add.params.get("amount").unwrap().required);
    assert!(add.params.get("kind").unwrap().required);
}

#[tokio::test]
async fn add_movement_returns_pending_under_strict_policy() {
    let reg = load_jaykonta_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJayKontaBackend::new());
    let client = JayKontaClient::new(backend);
    let dispatcher = Dispatcher::new();

    let decision = dispatcher
        .execute(
            "jaykonta.add_movement",
            &json!({"amount": 12.5, "kind": "expense", "label": "café"}),
            "voice",
            "note 12 € de café",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await
        .unwrap();
    assert!(matches!(decision, Decision::PendingConfirmation { .. }));
    // Aucun épisode ni mouvement
    assert_eq!(mem.recent_episodes(10).unwrap(), [] as [miyualicia_memory::Episode; 0]);
}

#[tokio::test]
async fn add_movement_executes_after_confirmation() {
    let reg = load_jaykonta_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJayKontaBackend::new());
    let client = JayKontaClient::new(backend.clone());
    let dispatcher = Dispatcher::new();

    let r = dispatcher
        .execute_confirmed(
            "jaykonta.add_movement",
            &json!({"amount": 12.5, "kind": "expense", "label": "café"}),
            "voice",
            "note 12 € de café",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await
        .unwrap();
    assert!(r.service_result.success);
    assert_eq!(r.service_result.data["movement"]["signed_amount"], -12.5);

    let recent = mem.recent_episodes(10).unwrap();
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].outcome, EpisodeOutcome::Ok);
    assert_eq!(recent[0].service_id, "jaykonta");
    assert_eq!(recent[0].intent, "add_movement");

    assert_eq!(backend.snapshot_movements().len(), 1);
}

#[tokio::test]
async fn read_only_capabilities_skip_confirmation() {
    let reg = load_jaykonta_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJayKontaBackend::new());
    let client = JayKontaClient::new(backend);
    let dispatcher = Dispatcher::new();

    // get_balance et list_recent : confirmation_default=never → exécution directe
    let bal = dispatcher
        .execute(
            "jaykonta.get_balance",
            &Value::Null,
            "voice",
            "?",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await
        .unwrap();
    assert!(matches!(bal, Decision::Executed(_)));

    let list = dispatcher
        .execute(
            "jaykonta.list_recent_movements",
            &json!({"limit": 5}),
            "voice",
            "?",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await
        .unwrap();
    assert!(matches!(list, Decision::Executed(_)));

    let recent = mem.recent_episodes(10).unwrap();
    assert_eq!(recent.len(), 2);
}

#[tokio::test]
async fn user_preference_can_skip_confirmation_for_writes() {
    let reg = load_jaykonta_registry();
    let mem = MemoryStore::open_in_memory().unwrap();

    // L'utilisateur dit explicitement « pour add_movement, ne me redemande pas »
    // → mais policy AlwaysForWrites prime, sauf si la capability était UserPreference.
    // Pour ce test, on utilise bypass_user_confirmations qui agit toujours.
    let backend = Arc::new(FakeJayKontaBackend::new());
    let client = JayKontaClient::new(backend);
    let dispatcher = Dispatcher::new();
    let mut policy = AliciaPolicy::strict();
    policy.bypass_user_confirmations = true;

    let decision = dispatcher
        .execute(
            "jaykonta.add_movement",
            &json!({"amount": 1, "kind": "expense"}),
            "voice",
            "?",
            &reg,
            &policy,
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await
        .unwrap();
    assert!(matches!(decision, Decision::Executed(_)));
}

#[tokio::test]
async fn capabilities_serialize_to_openai_tools() {
    let reg = load_jaykonta_registry();
    let caps: Vec<_> = reg.iter().map(|s| s.capability.clone()).collect();
    let tools = miyualicia_skills::capabilities_to_tools(caps.iter());
    assert_eq!(tools.len(), 3);

    let add_tool = tools
        .iter()
        .find(|t| t.function.name == "jaykonta.add_movement")
        .unwrap();
    let props = &add_tool.function.parameters["properties"];
    // amount : type number
    assert_eq!(props["amount"]["type"], "number");
    // kind : enum avec deux valeurs
    assert_eq!(props["kind"]["type"], "string");
    let enum_vals = props["kind"]["enum"].as_array().unwrap();
    assert_eq!(enum_vals.len(), 2);

    let required = add_tool.function.parameters["required"]
        .as_array()
        .unwrap();
    let req_strs: Vec<&str> = required.iter().filter_map(|v| v.as_str()).collect();
    assert!(req_strs.contains(&"amount"));
    assert!(req_strs.contains(&"kind"));
}
