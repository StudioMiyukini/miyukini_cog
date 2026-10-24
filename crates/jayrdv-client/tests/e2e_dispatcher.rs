//! Tests E2E : manifeste réel JayRDV → registry → dispatcher → JayRdvClient.

use std::path::Path;
use std::sync::Arc;

use serde_json::{json, Value};

use jayrdv_client::{FakeJayRdvBackend, JayRdvClient};
use miyualicia_dispatcher::{BypassCoreGate, Decision, Dispatcher};
use miyualicia_memory::{EpisodeOutcome, MemoryStore};
use miyualicia_skills::{discover_in_dir, AliciaPolicy, SkillRegistry};

fn load_jayrdv_registry() -> SkillRegistry {
    let tmp = tempfile::tempdir().unwrap();
    let sub = tmp.path().join("jayrdv");
    std::fs::create_dir_all(&sub).unwrap();
    let real = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("jayrdv")
        .join("service.manifest.json");
    let bytes = std::fs::read(&real).unwrap();
    std::fs::write(sub.join("service.manifest.json"), bytes).unwrap();
    let mut reg = SkillRegistry::new();
    let (_count, errors) = discover_in_dir(tmp.path(), &mut reg).unwrap();
    assert!(errors.is_empty(), "discovery errors: {errors:?}");
    reg
}

#[tokio::test]
async fn manifest_loads_four_capabilities() {
    let reg = load_jayrdv_registry();
    assert!(reg.get("jayrdv.book_appointment").is_some());
    assert!(reg.get("jayrdv.cancel_appointment").is_some());
    assert!(reg.get("jayrdv.list_my_appointments").is_some());
    assert!(reg.get("jayrdv.list_available_slots").is_some());

    let book = &reg.get("jayrdv.book_appointment").unwrap().capability;
    assert!(book.writes);
    assert!(book.params.get("professional_id").unwrap().required);
    assert!(book.params.get("service_id").unwrap().required);
    assert!(book.params.get("slot_start").unwrap().required);
}

#[tokio::test]
async fn book_appointment_returns_pending_under_strict_policy() {
    let reg = load_jayrdv_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJayRdvBackend::new());
    let client = JayRdvClient::new(backend);
    let dispatcher = Dispatcher::new();

    let decision = dispatcher
        .execute(
            "jayrdv.book_appointment",
            &json!({
                "professional_id": "dr_smith",
                "service_id": "consult_30",
                "slot_start": "2026-05-15T10:00:00Z"
            }),
            "voice",
            "prends rdv chez dr smith",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await
        .unwrap();
    assert!(matches!(decision, Decision::PendingConfirmation { .. }));
    // Aucun rendez-vous posé tant que pas confirmé
    assert!(mem.recent_episodes(10).unwrap().is_empty());
}

#[tokio::test]
async fn book_appointment_executes_after_confirmation() {
    let reg = load_jayrdv_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJayRdvBackend::new());
    let client = JayRdvClient::new(backend.clone());
    let dispatcher = Dispatcher::new();

    let r = dispatcher
        .execute_confirmed(
            "jayrdv.book_appointment",
            &json!({
                "professional_id": "dr_smith",
                "service_id": "consult_30",
                "slot_start": "2026-05-15T10:00:00Z"
            }),
            "voice",
            "prends rdv chez dr smith",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await
        .unwrap();
    assert!(r.service_result.success);
    assert_eq!(r.service_result.data["appointment"]["status"], "confirmed");
    assert_eq!(backend.snapshot_appointments().len(), 1);

    let recent = mem.recent_episodes(10).unwrap();
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].outcome, EpisodeOutcome::Ok);
    assert_eq!(recent[0].service_id, "jayrdv");
}

#[tokio::test]
async fn read_only_capabilities_skip_confirmation() {
    let reg = load_jayrdv_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJayRdvBackend::new());
    let client = JayRdvClient::new(backend);
    let dispatcher = Dispatcher::new();

    let list = dispatcher
        .execute(
            "jayrdv.list_my_appointments",
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
    assert!(matches!(list, Decision::Executed(_)));
    let recent = mem.recent_episodes(10).unwrap();
    assert_eq!(recent.len(), 1);
}

#[tokio::test]
async fn capabilities_serialize_to_openai_tools() {
    let reg = load_jayrdv_registry();
    let caps: Vec<_> = reg.iter().map(|s| s.capability.clone()).collect();
    let tools = miyualicia_skills::capabilities_to_tools(caps.iter());
    assert_eq!(tools.len(), 4);

    let book = tools
        .iter()
        .find(|t| t.function.name == "jayrdv.book_appointment")
        .unwrap();
    let props = &book.function.parameters["properties"];
    // slot_start est datetime → format date-time
    assert_eq!(props["slot_start"]["type"], "string");
    assert_eq!(props["slot_start"]["format"], "date-time");

    let slots = tools
        .iter()
        .find(|t| t.function.name == "jayrdv.list_available_slots")
        .unwrap();
    let from = &slots.function.parameters["properties"]["from"];
    assert_eq!(from["format"], "date-time");
}
