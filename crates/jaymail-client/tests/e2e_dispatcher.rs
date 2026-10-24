//! Tests E2E : manifeste réel JayMail → registry → dispatcher → JayMailClient.

use std::path::Path;
use std::sync::Arc;

use serde_json::{json, Value};

use jaymail_client::{FakeJayMailBackend, JayMailClient, MailSummary};
use miyualicia_dispatcher::{BypassCoreGate, Decision, Dispatcher};
use miyualicia_memory::{EpisodeOutcome, MemoryStore};
use miyualicia_skills::{discover_in_dir, AliciaPolicy, SkillRegistry};

fn load_jaymail_registry() -> SkillRegistry {
    let tmp = tempfile::tempdir().unwrap();
    let sub = tmp.path().join("jaymail");
    std::fs::create_dir_all(&sub).unwrap();
    let real = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("jaybureau-suite")
        .join("jaymail")
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
    let reg = load_jaymail_registry();
    assert!(reg.get("jaymail.send_mail").is_some());
    assert!(reg.get("jaymail.list_recent_inbox").is_some());
    assert!(reg.get("jaymail.search_mail").is_some());

    let send = &reg.get("jaymail.send_mail").unwrap().capability;
    assert!(send.writes);
    assert!(send.params.get("to").unwrap().required);
    assert!(send.params.get("subject").unwrap().required);
    assert!(send.params.get("body").unwrap().required);
}

#[tokio::test]
async fn send_mail_returns_pending_under_strict_policy() {
    let reg = load_jaymail_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJayMailBackend::new());
    let client = JayMailClient::new(backend);
    let dispatcher = Dispatcher::new();

    let decision = dispatcher
        .execute(
            "jaymail.send_mail",
            &json!({"to": "jean@x.com", "subject": "Bonjour", "body": "salut"}),
            "voice",
            "envoie un mail à Jean",
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
async fn send_mail_executes_after_confirmation() {
    let reg = load_jaymail_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJayMailBackend::new());
    let client = JayMailClient::new(backend.clone());
    let dispatcher = Dispatcher::new();

    let r = dispatcher
        .execute_confirmed(
            "jaymail.send_mail",
            &json!({"to": "jean@x.com", "subject": "Test", "body": "hello"}),
            "voice",
            "envoie",
            &reg,
            &AliciaPolicy::strict(),
            &BypassCoreGate,
            &client,
            &mem,
        )
        .await
        .unwrap();
    assert!(r.service_result.success);
    assert_eq!(r.service_result.data["sent"]["to"], "jean@x.com");
    assert_eq!(backend.snapshot_sent().len(), 1);

    let recent = mem.recent_episodes(10).unwrap();
    assert_eq!(recent[0].outcome, EpisodeOutcome::Ok);
    assert_eq!(recent[0].service_id, "jaymail");
}

#[tokio::test]
async fn read_only_capabilities_skip_confirmation() {
    let reg = load_jaymail_registry();
    let mem = MemoryStore::open_in_memory().unwrap();
    let backend = Arc::new(FakeJayMailBackend::new());
    backend.push_inbox(MailSummary {
        id: "1".into(),
        from: "alice@x.com".into(),
        subject: "Salut".into(),
        preview: "Comment vas-tu".into(),
        received_at: "2026-05-08T10:00:00Z".into(),
        unread: true,
    });
    let client = JayMailClient::new(backend);
    let dispatcher = Dispatcher::new();

    let list = dispatcher
        .execute(
            "jaymail.list_recent_inbox",
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
    let reg = load_jaymail_registry();
    let caps: Vec<_> = reg.iter().map(|s| s.capability.clone()).collect();
    let tools = miyualicia_skills::capabilities_to_tools(caps.iter());
    assert_eq!(tools.len(), 3);

    let send = tools
        .iter()
        .find(|t| t.function.name == "jaymail.send_mail")
        .unwrap();
    let required = send.function.parameters["required"].as_array().unwrap();
    let req_strs: Vec<&str> = required.iter().filter_map(|v| v.as_str()).collect();
    assert!(req_strs.contains(&"to"));
    assert!(req_strs.contains(&"subject"));
    assert!(req_strs.contains(&"body"));

    let inbox = tools
        .iter()
        .find(|t| t.function.name == "jaymail.list_recent_inbox")
        .unwrap();
    let unread_only = &inbox.function.parameters["properties"]["unread_only"];
    assert_eq!(unread_only["type"], "boolean");
}
