//! Tests E2E : manifeste réel JaySheets → registry → dispatcher → JaySheetsClient.

use std::path::Path;
use std::sync::Arc;

use serde_json::{json, Value};

use jaysheets_client::{FakeJaySheetsBackend, JaySheetsBackend, JaySheetsClient};
use miyualicia_dispatcher::{BypassCoreGate, Decision, Dispatcher};
use miyualicia_memory::{EpisodeOutcome, MemoryStore};
use miyualicia_skills::{discover_in_dir, AliciaPolicy, SkillRegistry};

fn load_jaysheets_registry() -> SkillRegistry {
    let tmp = tempfile::tempdir().unwrap();
    let sub = tmp.path().join("jaysheets");
    std::fs::create_dir_all(&sub).unwrap();
    let real = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("apps")
        .join("jaybureau-suite")
        .join("jay-sheets")
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
    let reg = load_jaysheets_registry();
    assert!(reg.get("jaysheets.create_sheet").is_some());
    assert!(reg.get("jaysheets.list_my_sheets").is_some());
    assert!(reg.get("jaysheets.write_cell").is_some());

    let write = &reg.get("jaysheets.write_cell").unwrap().capability;
    assert!(write.writes);
    assert!(write.params.get("sheet_id").unwrap().required);
    assert!(write.params.get("cell").unwrap().required);
    assert!(write.params.get("value").unwrap().required);
}

#[tokio::test]
async fn write_cell_returns_pending_under_strict_policy() {
    let reg = load_jaysheets_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJaySheetsBackend::new());
    let client = JaySheetsClient::new(backend);
    let dispatcher = Dispatcher::new();

    let decision = dispatcher
        .execute(
            "jaysheets.write_cell",
            &json!({"sheet_id": "sheet_1", "cell": "A1", "value": "42"}),
            "voice",
            "écris 42 en A1",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await
        .unwrap();
    assert!(matches!(decision, Decision::PendingConfirmation { .. }));
}

#[tokio::test]
async fn write_cell_executes_after_confirmation() {
    let reg = load_jaysheets_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJaySheetsBackend::new());
    backend.create_sheet("X").await.unwrap();
    let client = JaySheetsClient::new(backend.clone());
    let dispatcher = Dispatcher::new();

    let r = dispatcher
        .execute_confirmed(
            "jaysheets.write_cell",
            &json!({"sheet_id": "sheet_1", "cell": "B5", "value": "total"}),
            "voice",
            "remplis B5",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await
        .unwrap();
    assert!(r.service_result.success);
    assert_eq!(
        backend.cell_value("sheet_1", "B5").as_deref(),
        Some("total")
    );

    let recent = mem.recent_episodes(10).unwrap();
    assert_eq!(recent[0].outcome, EpisodeOutcome::Ok);
    assert_eq!(recent[0].service_id, "jaysheets");
}

#[tokio::test]
async fn list_skips_confirmation() {
    let reg = load_jaysheets_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJaySheetsBackend::new());
    backend.create_sheet("Budget").await.unwrap();
    let client = JaySheetsClient::new(backend);
    let dispatcher = Dispatcher::new();

    let list = dispatcher
        .execute(
            "jaysheets.list_my_sheets",
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
    if let Decision::Executed(r) = list {
        assert_eq!(r.service_result.data["count"], 1);
    } else {
        panic!("expected Executed");
    }
}

#[tokio::test]
async fn capabilities_serialize_to_openai_tools() {
    let reg = load_jaysheets_registry();
    let caps: Vec<_> = reg.iter().map(|s| s.capability.clone()).collect();
    let tools = miyualicia_skills::capabilities_to_tools(caps.iter());
    assert_eq!(tools.len(), 3);

    let write = tools
        .iter()
        .find(|t| t.function.name == "jaysheets.write_cell")
        .unwrap();
    let required = write.function.parameters["required"].as_array().unwrap();
    let req_strs: Vec<&str> = required.iter().filter_map(|v| v.as_str()).collect();
    assert!(req_strs.contains(&"sheet_id"));
    assert!(req_strs.contains(&"cell"));
    assert!(req_strs.contains(&"value"));
}
