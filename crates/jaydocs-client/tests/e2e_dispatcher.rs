//! Tests E2E : manifeste réel JayDocs → registry → dispatcher → JayDocsClient.

use std::path::Path;
use std::sync::Arc;

use serde_json::{json, Value};

use jaydocs_client::{FakeJayDocsBackend, JayDocsClient};
use miyualicia_dispatcher::{BypassCoreGate, Decision, Dispatcher};
use miyualicia_memory::{EpisodeOutcome, MemoryStore};
use miyualicia_skills::{discover_in_dir, AliciaPolicy, SkillRegistry};

fn load_jaydocs_registry() -> SkillRegistry {
    let tmp = tempfile::tempdir().unwrap();
    let sub = tmp.path().join("jaydocs");
    std::fs::create_dir_all(&sub).unwrap();
    let real = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("apps")
        .join("jaybureau-suite")
        .join("jay-docs")
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
    let reg = load_jaydocs_registry();
    assert!(reg.get("jaydocs.create_document").is_some());
    assert!(reg.get("jaydocs.list_my_documents").is_some());
    assert!(reg.get("jaydocs.append_to_document").is_some());

    let create = &reg.get("jaydocs.create_document").unwrap().capability;
    assert!(create.writes);
    assert!(create.params.get("title").unwrap().required);

    let list = &reg.get("jaydocs.list_my_documents").unwrap().capability;
    assert!(!list.writes);
}

#[tokio::test]
async fn create_document_returns_pending_under_strict_policy() {
    let reg = load_jaydocs_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJayDocsBackend::new());
    let client = JayDocsClient::new(backend);
    let dispatcher = Dispatcher::new();

    let decision = dispatcher
        .execute(
            "jaydocs.create_document",
            &json!({"title": "Notes réunion"}),
            "voice",
            "crée un document",
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
async fn create_document_executes_after_confirmation() {
    let reg = load_jaydocs_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJayDocsBackend::new());
    let client = JayDocsClient::new(backend.clone());
    let dispatcher = Dispatcher::new();

    let r = dispatcher
        .execute_confirmed(
            "jaydocs.create_document",
            &json!({"title": "Compte-rendu", "content": "intro"}),
            "voice",
            "crée",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await
        .unwrap();
    assert!(r.service_result.success);
    assert_eq!(r.service_result.data["document"]["title"], "Compte-rendu");
    assert_eq!(backend.snapshot().len(), 1);

    let recent = mem.recent_episodes(10).unwrap();
    assert_eq!(recent[0].outcome, EpisodeOutcome::Ok);
    assert_eq!(recent[0].service_id, "jaydocs");
}

#[tokio::test]
async fn list_documents_skips_confirmation() {
    let reg = load_jaydocs_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJayDocsBackend::new());
    let client = JayDocsClient::new(backend.clone());
    let dispatcher = Dispatcher::new();

    // Crée d'abord un doc via le backend directement.
    use jaydocs_client::JayDocsBackend;
    backend
        .create_document("Projet Alpha", Some("body"))
        .await
        .unwrap();

    let list = dispatcher
        .execute(
            "jaydocs.list_my_documents",
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
    let reg = load_jaydocs_registry();
    let caps: Vec<_> = reg.iter().map(|s| s.capability.clone()).collect();
    let tools = miyualicia_skills::capabilities_to_tools(caps.iter());
    assert_eq!(tools.len(), 3);

    let append = tools
        .iter()
        .find(|t| t.function.name == "jaydocs.append_to_document")
        .unwrap();
    let required = append.function.parameters["required"].as_array().unwrap();
    let req_strs: Vec<&str> = required.iter().filter_map(|v| v.as_str()).collect();
    assert!(req_strs.contains(&"document_id"));
    assert!(req_strs.contains(&"content"));
}
