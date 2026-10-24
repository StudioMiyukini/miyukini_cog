//! Tests E2E : manifeste réel JayMessage → registry → dispatcher → JayMessageClient.

use std::path::Path;
use std::sync::Arc;

use serde_json::{json, Value};

use jaymessage_client::{FakeJayMessageBackend, JayMessageClient, ThreadSummary};
use miyualicia_dispatcher::{BypassCoreGate, Decision, Dispatcher};
use miyualicia_memory::{EpisodeOutcome, MemoryStore};
use miyualicia_skills::{discover_in_dir, AliciaPolicy, SkillRegistry};

fn load_jaymessage_registry() -> SkillRegistry {
    let tmp = tempfile::tempdir().unwrap();
    let sub = tmp.path().join("jaymessage");
    std::fs::create_dir_all(&sub).unwrap();
    let real = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("jaybureau-suite")
        .join("jaymessage")
        .join("service.manifest.json");
    let bytes = std::fs::read(&real).unwrap();
    std::fs::write(sub.join("service.manifest.json"), bytes).unwrap();
    let mut reg = SkillRegistry::new();
    let (_count, errors) = discover_in_dir(tmp.path(), &mut reg).unwrap();
    assert!(errors.is_empty());
    reg
}

#[tokio::test]
async fn manifest_loads_three_capabilities() {
    let reg = load_jaymessage_registry();
    assert!(reg.get("jaymessage.send_message").is_some());
    assert!(reg.get("jaymessage.list_recent_threads").is_some());
    assert!(reg.get("jaymessage.search_threads").is_some());
    let send = &reg.get("jaymessage.send_message").unwrap().capability;
    assert!(send.writes);
}

#[tokio::test]
async fn send_message_returns_pending_under_strict_policy() {
    let reg = load_jaymessage_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJayMessageBackend::new());
    backend.push_thread(ThreadSummary {
        id: "t1".into(),
        name: "Jean".into(),
        last_message_preview: "...".into(),
        last_message_from: "x".into(),
        last_message_at: "2026-05-08T10:00:00Z".into(),
        unread_count: 0,
    });
    let client = JayMessageClient::new(backend);
    let dispatcher = Dispatcher::new();
    let decision = dispatcher
        .execute(
            "jaymessage.send_message",
            &json!({"thread_id": "t1", "body": "hello"}),
            "voice",
            "envoie un message à Jean",
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
async fn send_message_executes_after_confirmation() {
    let reg = load_jaymessage_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJayMessageBackend::new());
    let client = JayMessageClient::new(backend.clone());
    let dispatcher = Dispatcher::new();
    let r = dispatcher
        .execute_confirmed(
            "jaymessage.send_message",
            &json!({"thread_id": "t1", "body": "ok"}),
            "voice",
            "x",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await
        .unwrap();
    assert!(r.service_result.success);
    assert_eq!(backend.snapshot_sent().len(), 1);
    let recent = mem.recent_episodes(10).unwrap();
    assert_eq!(recent[0].outcome, EpisodeOutcome::Ok);
    assert_eq!(recent[0].service_id, "jaymessage");
}

#[tokio::test]
async fn read_only_capabilities_skip_confirmation() {
    let reg = load_jaymessage_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJayMessageBackend::new());
    let client = JayMessageClient::new(backend);
    let dispatcher = Dispatcher::new();
    let list = dispatcher
        .execute(
            "jaymessage.list_recent_threads",
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
}

#[tokio::test]
async fn capabilities_serialize_to_openai_tools() {
    let reg = load_jaymessage_registry();
    let caps: Vec<_> = reg.iter().map(|s| s.capability.clone()).collect();
    let tools = miyualicia_skills::capabilities_to_tools(caps.iter());
    assert_eq!(tools.len(), 3);
    let send = tools
        .iter()
        .find(|t| t.function.name == "jaymessage.send_message")
        .unwrap();
    let required = send.function.parameters["required"].as_array().unwrap();
    let req_strs: Vec<&str> = required.iter().filter_map(|v| v.as_str()).collect();
    assert!(req_strs.contains(&"thread_id"));
    assert!(req_strs.contains(&"body"));
}
