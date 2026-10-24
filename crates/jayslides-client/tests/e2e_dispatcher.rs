//! Tests E2E : manifeste réel JaySlides → registry → dispatcher → JaySlidesClient.

use std::path::Path;
use std::sync::Arc;

use serde_json::{json, Value};

use jayslides_client::{FakeJaySlidesBackend, JaySlidesBackend, JaySlidesClient};
use miyualicia_dispatcher::{BypassCoreGate, Decision, Dispatcher};
use miyualicia_memory::{EpisodeOutcome, MemoryStore};
use miyualicia_skills::{discover_in_dir, AliciaPolicy, SkillRegistry};

fn load_jayslides_registry() -> SkillRegistry {
    let tmp = tempfile::tempdir().unwrap();
    let sub = tmp.path().join("jayslides");
    std::fs::create_dir_all(&sub).unwrap();
    let real = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("apps")
        .join("jaybureau-suite")
        .join("jay-slides")
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
    let reg = load_jayslides_registry();
    assert!(reg.get("jayslides.create_presentation").is_some());
    assert!(reg.get("jayslides.list_my_presentations").is_some());
    assert!(reg.get("jayslides.add_slide").is_some());

    let add = &reg.get("jayslides.add_slide").unwrap().capability;
    assert!(add.writes);
    assert!(add.params.get("presentation_id").unwrap().required);
    assert!(add.params.get("title").unwrap().required);
}

#[tokio::test]
async fn add_slide_returns_pending_under_strict_policy() {
    let reg = load_jayslides_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJaySlidesBackend::new());
    let client = JaySlidesClient::new(backend);
    let dispatcher = Dispatcher::new();

    let decision = dispatcher
        .execute(
            "jayslides.add_slide",
            &json!({"presentation_id": "pres_1", "title": "Intro"}),
            "voice",
            "ajoute slide intro",
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
async fn add_slide_executes_after_confirmation() {
    let reg = load_jayslides_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJaySlidesBackend::new());
    backend.create_presentation("X").await.unwrap();
    let client = JaySlidesClient::new(backend.clone());
    let dispatcher = Dispatcher::new();

    let r = dispatcher
        .execute_confirmed(
            "jayslides.add_slide",
            &json!({"presentation_id": "pres_1", "title": "Conclusion", "content": "Merci"}),
            "voice",
            "ajoute",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await
        .unwrap();
    assert!(r.service_result.success);
    assert_eq!(backend.slides_of("pres_1").len(), 1);
    assert_eq!(backend.slides_of("pres_1")[0].title, "Conclusion");

    let recent = mem.recent_episodes(10).unwrap();
    assert_eq!(recent[0].outcome, EpisodeOutcome::Ok);
    assert_eq!(recent[0].service_id, "jayslides");
}

#[tokio::test]
async fn list_skips_confirmation() {
    let reg = load_jayslides_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJaySlidesBackend::new());
    backend.create_presentation("Pitch").await.unwrap();
    let client = JaySlidesClient::new(backend);
    let dispatcher = Dispatcher::new();

    let list = dispatcher
        .execute(
            "jayslides.list_my_presentations",
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
    let reg = load_jayslides_registry();
    let caps: Vec<_> = reg.iter().map(|s| s.capability.clone()).collect();
    let tools = miyualicia_skills::capabilities_to_tools(caps.iter());
    assert_eq!(tools.len(), 3);

    let add = tools
        .iter()
        .find(|t| t.function.name == "jayslides.add_slide")
        .unwrap();
    let required = add.function.parameters["required"].as_array().unwrap();
    let req_strs: Vec<&str> = required.iter().filter_map(|v| v.as_str()).collect();
    assert!(req_strs.contains(&"presentation_id"));
    assert!(req_strs.contains(&"title"));
}
