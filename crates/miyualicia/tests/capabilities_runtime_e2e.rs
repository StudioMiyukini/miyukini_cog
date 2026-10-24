//! Tests d'intégration : `AliciaService` + `CapabilitiesRuntime`.
//!
//! Valide la composition : un `AliciaService` peut activer le sous-système
//! des capacités, dispatcher une capacité réelle (JayKoa via fake backend),
//! et conserver le comportement device historique intact.

use std::sync::Arc;

use serde_json::{json, Value};

use borderguard::TrustLevel;
use miyualicia::{AliciaConfig, AliciaService, CapabilitiesRuntime, CapabilitiesRuntimeConfig};
use miyualicia_cores_gate::test_helpers::simple_gate;
use miyualicia_dispatcher::{Decision, MockServiceClient, ServiceCallResult};
use miyualicia_memory::Persona;

fn empty_config() -> AliciaConfig {
    AliciaConfig {
        mqtt: None,
        ..Default::default()
    }
}

#[tokio::test]
async fn alicia_service_starts_without_capabilities_runtime() {
    let svc = AliciaService::new(empty_config());
    assert!(!svc.capabilities_enabled().await);
    assert!(svc.capabilities().await.is_none());
}

#[tokio::test]
async fn enabling_capabilities_runtime_registers_clients_and_dispatches() {
    let tmp = tempfile::tempdir().unwrap();
    // Manifeste minimal d'un service "demo" exposant `demo.ping`
    let demo_dir = tmp.path().join("demo");
    std::fs::create_dir_all(&demo_dir).unwrap();
    std::fs::write(
        demo_dir.join("service.manifest.json"),
        r#"{
            "id": "demo",
            "alicia_capabilities": [
                {"intent": "ping", "phrases_examples": ["ping"], "params": {}, "writes": false}
            ]
        }"#,
    )
    .unwrap();

    let cfg = CapabilitiesRuntimeConfig {
        services_dir: Some(tmp.path().to_path_buf()),
        ..Default::default()
    };
    // Gate BorderGuard avec une règle pour `demo` et persona `voice` Trusted
    let gate = simple_gate(
        [("demo", TrustLevel::Verified, true)],
        [("voice", TrustLevel::Trusted)],
    );

    let runtime = Arc::new(
        CapabilitiesRuntime::new(cfg, Arc::new(gate))
            .await
            .unwrap(),
    );

    // Mock service client pour `demo`
    let mock = MockServiceClient::new();
    mock.expect("demo.ping", ServiceCallResult::ok(json!({"pong": true})));
    runtime
        .register_service_client("demo", Arc::new(mock))
        .await;

    let svc = AliciaService::new(empty_config());
    svc.enable_capabilities_runtime(runtime.clone()).await;
    assert!(svc.capabilities_enabled().await);

    // Tools exposés
    let tools = runtime.list_tools().await;
    assert_eq!(tools.len(), 1);
    assert_eq!(tools[0].function.name, "demo.ping");

    // Dispatch
    let decision = runtime
        .dispatch_capability("demo.ping", &Value::Null, "voice", "ping")
        .await
        .unwrap();
    assert!(matches!(decision, Decision::Executed(_)));

    // Audit posé
    let recent = runtime.memory().recent_episodes(10).unwrap();
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].service_id, "demo");
}

#[tokio::test]
async fn capabilities_runtime_denied_for_low_trust_persona() {
    let tmp = tempfile::tempdir().unwrap();
    let demo = tmp.path().join("demo");
    std::fs::create_dir_all(&demo).unwrap();
    std::fs::write(
        demo.join("service.manifest.json"),
        r#"{"id":"demo","alicia_capabilities":[{"intent":"ping","phrases_examples":["x"],"params":{},"writes":false}]}"#,
    )
    .unwrap();

    let cfg = CapabilitiesRuntimeConfig {
        services_dir: Some(tmp.path().to_path_buf()),
        ..Default::default()
    };
    // gate exige Trusted, voice = Verified seulement → refus
    let gate = simple_gate(
        [("demo", TrustLevel::Trusted, true)],
        [("voice", TrustLevel::Verified)],
    );
    let runtime = Arc::new(
        CapabilitiesRuntime::new(cfg, Arc::new(gate))
            .await
            .unwrap(),
    );
    let mock = MockServiceClient::new();
    mock.expect("demo.ping", ServiceCallResult::ok(json!({})));
    runtime
        .register_service_client("demo", Arc::new(mock))
        .await;

    let err = runtime
        .dispatch_capability("demo.ping", &Value::Null, "voice", "?")
        .await
        .unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("CoresDenied") || msg.contains("trust"));
}

#[tokio::test]
async fn manifest_watcher_via_runtime_picks_up_added_service() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg = CapabilitiesRuntimeConfig {
        services_dir: Some(tmp.path().to_path_buf()),
        ..Default::default()
    };
    let runtime = CapabilitiesRuntime::new_with_bypass_gate(cfg).await.unwrap();
    // 1er tick sur dossier vide
    let _ = runtime.tick_manifest_watcher().await.unwrap();

    let demo = tmp.path().join("demo");
    std::fs::create_dir_all(&demo).unwrap();
    std::fs::write(
        demo.join("service.manifest.json"),
        r#"{"id":"demo","alicia_capabilities":[{"intent":"x","phrases_examples":["a"],"params":{},"writes":false}]}"#,
    )
    .unwrap();
    let report = runtime
        .tick_manifest_watcher()
        .await
        .unwrap()
        .expect("reload");
    assert_eq!(report.added, vec!["demo".to_string()]);
    assert_eq!(runtime.list_tools().await.len(), 1);
}

#[tokio::test]
async fn learned_skill_resolution_via_runtime() {
    let runtime = Arc::new(
        CapabilitiesRuntime::new_with_bypass_gate(CapabilitiesRuntimeConfig::default())
            .await
            .unwrap(),
    );
    let mem = runtime.memory();
    mem.upsert_persona(&Persona::new("voice".into(), "Voix".into(), "".into(), true))
        .unwrap();
    let ls = mem
        .propose_learned_skill(
            "voice",
            "12 € café",
            "demo.add_expense",
            json!({"amount": 12, "category": "café"}),
        )
        .unwrap();
    mem.accept_learned_skill(ls.id).unwrap();

    let resolved = runtime
        .resolve_learned_phrase("12 € café", "voice")
        .unwrap()
        .expect("match");
    assert_eq!(resolved.qualified_id, "demo.add_expense");
    assert_eq!(resolved.params["category"], "café");
}

// ----------------------------------------------------------------------
// Queue async des PendingConfirmation (Q.2)
// ----------------------------------------------------------------------

#[tokio::test]
async fn dispatch_and_queue_executed_for_read_only_capability() {
    let tmp = tempfile::tempdir().unwrap();
    let demo = tmp.path().join("demo");
    std::fs::create_dir_all(&demo).unwrap();
    std::fs::write(
        demo.join("service.manifest.json"),
        r#"{"id":"demo","alicia_capabilities":[{"intent":"ping","phrases_examples":["x"],"params":{},"writes":false}]}"#,
    )
    .unwrap();
    let runtime = Arc::new(
        CapabilitiesRuntime::new_with_bypass_gate(CapabilitiesRuntimeConfig {
            services_dir: Some(tmp.path().to_path_buf()),
            ..Default::default()
        })
        .await
        .unwrap(),
    );
    let mock = MockServiceClient::new();
    mock.expect("demo.ping", ServiceCallResult::ok(json!({"pong": true})));
    runtime
        .register_service_client("demo", Arc::new(mock))
        .await;

    let outcome = runtime
        .dispatch_and_queue("demo.ping", &Value::Null, "voice", "ping")
        .await
        .unwrap();
    match outcome {
        miyualicia::capabilities_runtime::QueuedDispatch::Executed(r) => {
            assert_eq!(r.service_result.data["pong"], true);
        }
        _ => panic!("expected Executed (read-only sans confirmation)"),
    }
    // Pas de pending posée
    assert!(runtime.list_pending_confirmations().await.unwrap().is_empty());
}

#[tokio::test]
async fn dispatch_and_queue_queued_for_write_capability() {
    let tmp = tempfile::tempdir().unwrap();
    let demo = tmp.path().join("demo");
    std::fs::create_dir_all(&demo).unwrap();
    std::fs::write(
        demo.join("service.manifest.json"),
        r#"{
            "id": "demo",
            "alicia_capabilities": [{
                "intent": "shout",
                "description": "Crier fort",
                "phrases_examples": ["x"],
                "params": {},
                "writes": true,
                "confirmation_default": "always_for_writes"
            }]
        }"#,
    )
    .unwrap();
    let runtime = Arc::new(
        CapabilitiesRuntime::new_with_bypass_gate(CapabilitiesRuntimeConfig {
            services_dir: Some(tmp.path().to_path_buf()),
            ..Default::default()
        })
        .await
        .unwrap(),
    );
    let mock = MockServiceClient::new();
    runtime
        .register_service_client("demo", Arc::new(mock))
        .await;

    let outcome = runtime
        .dispatch_and_queue("demo.shout", &Value::Null, "voice", "shout!")
        .await
        .unwrap();
    let queued_id = match outcome {
        miyualicia::capabilities_runtime::QueuedDispatch::Queued {
            id,
            qualified_id,
            prompt,
        } => {
            assert_eq!(qualified_id, "demo.shout");
            assert!(prompt.contains("Crier") || !prompt.is_empty());
            id
        }
        _ => panic!("expected Queued"),
    };

    let pending = runtime.list_pending_confirmations().await.unwrap();
    assert_eq!(pending.len(), 1);
    assert_eq!(pending[0].id, queued_id);
}

#[tokio::test]
async fn confirm_queued_executes_and_resolves() {
    let tmp = tempfile::tempdir().unwrap();
    let demo = tmp.path().join("demo");
    std::fs::create_dir_all(&demo).unwrap();
    std::fs::write(
        demo.join("service.manifest.json"),
        r#"{
            "id": "demo",
            "alicia_capabilities": [{
                "intent": "shout",
                "phrases_examples": ["x"],
                "params": {},
                "writes": true,
                "confirmation_default": "always_for_writes"
            }]
        }"#,
    )
    .unwrap();
    let runtime = Arc::new(
        CapabilitiesRuntime::new_with_bypass_gate(CapabilitiesRuntimeConfig {
            services_dir: Some(tmp.path().to_path_buf()),
            ..Default::default()
        })
        .await
        .unwrap(),
    );
    let mock = MockServiceClient::new();
    mock.expect("demo.shout", ServiceCallResult::ok(json!({"shouted": true})));
    runtime
        .register_service_client("demo", Arc::new(mock))
        .await;

    let outcome = runtime
        .dispatch_and_queue("demo.shout", &json!({"loud": true}), "voice", "shout!")
        .await
        .unwrap();
    let id = match outcome {
        miyualicia::capabilities_runtime::QueuedDispatch::Queued { id, .. } => id,
        _ => panic!("expected Queued"),
    };

    let result = runtime.confirm_queued(id).await.unwrap();
    assert_eq!(result.service_result.data["shouted"], true);

    // La pending est désormais Confirmed et n'apparaît plus dans la liste pending
    assert!(runtime.list_pending_confirmations().await.unwrap().is_empty());
    let resolved = runtime.pending_confirmation(id).await.unwrap().unwrap();
    assert_eq!(
        resolved.status,
        miyualicia_memory::PendingStatus::Confirmed
    );
    assert!(resolved.resolved_at.is_some());

    // Episode Ok dans l'audit
    let recent = runtime.memory().recent_episodes(10).unwrap();
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].outcome, miyualicia_memory::EpisodeOutcome::Ok);
}

#[tokio::test]
async fn cancel_queued_records_cancelled_episode_and_resolves() {
    let tmp = tempfile::tempdir().unwrap();
    let demo = tmp.path().join("demo");
    std::fs::create_dir_all(&demo).unwrap();
    std::fs::write(
        demo.join("service.manifest.json"),
        r#"{
            "id": "demo",
            "alicia_capabilities": [{
                "intent": "shout",
                "phrases_examples": ["x"],
                "params": {},
                "writes": true,
                "confirmation_default": "always_for_writes"
            }]
        }"#,
    )
    .unwrap();
    let runtime = Arc::new(
        CapabilitiesRuntime::new_with_bypass_gate(CapabilitiesRuntimeConfig {
            services_dir: Some(tmp.path().to_path_buf()),
            ..Default::default()
        })
        .await
        .unwrap(),
    );

    let outcome = runtime
        .dispatch_and_queue("demo.shout", &Value::Null, "voice", "shout!")
        .await
        .unwrap();
    let id = match outcome {
        miyualicia::capabilities_runtime::QueuedDispatch::Queued { id, .. } => id,
        _ => panic!("expected Queued"),
    };

    runtime.cancel_queued(id).await.unwrap();

    // Pending Cancelled
    let resolved = runtime.pending_confirmation(id).await.unwrap().unwrap();
    assert_eq!(
        resolved.status,
        miyualicia_memory::PendingStatus::Cancelled
    );

    // Episode Cancelled dans l'audit
    let recent = runtime.memory().recent_episodes(10).unwrap();
    assert_eq!(recent.len(), 1);
    assert_eq!(
        recent[0].outcome,
        miyualicia_memory::EpisodeOutcome::Cancelled
    );
}

#[tokio::test]
async fn confirm_unknown_queued_returns_error() {
    let runtime = CapabilitiesRuntime::new_with_bypass_gate(CapabilitiesRuntimeConfig::default())
        .await
        .unwrap();
    let err = runtime
        .confirm_queued(uuid::Uuid::new_v4())
        .await
        .unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("introuvable"));
}

#[tokio::test]
async fn confirm_already_resolved_returns_error() {
    let tmp = tempfile::tempdir().unwrap();
    let demo = tmp.path().join("demo");
    std::fs::create_dir_all(&demo).unwrap();
    std::fs::write(
        demo.join("service.manifest.json"),
        r#"{
            "id": "demo",
            "alicia_capabilities": [{
                "intent": "shout",
                "phrases_examples": ["x"],
                "params": {},
                "writes": true,
                "confirmation_default": "always_for_writes"
            }]
        }"#,
    )
    .unwrap();
    let runtime = Arc::new(
        CapabilitiesRuntime::new_with_bypass_gate(CapabilitiesRuntimeConfig {
            services_dir: Some(tmp.path().to_path_buf()),
            ..Default::default()
        })
        .await
        .unwrap(),
    );
    let mock = MockServiceClient::new();
    mock.expect("demo.shout", ServiceCallResult::ok(json!({})));
    runtime
        .register_service_client("demo", Arc::new(mock))
        .await;

    let outcome = runtime
        .dispatch_and_queue("demo.shout", &Value::Null, "voice", "x")
        .await
        .unwrap();
    let id = match outcome {
        miyualicia::capabilities_runtime::QueuedDispatch::Queued { id, .. } => id,
        _ => panic!("expected Queued"),
    };
    runtime.confirm_queued(id).await.unwrap();
    // Re-confirm doit échouer
    let err = runtime.confirm_queued(id).await.unwrap_err();
    let msg = format!("{err}");
    assert!(msg.contains("déjà résolue"));
}

#[tokio::test]
async fn handle_tool_calls_dispatches_and_builds_tool_messages() {
    let tmp = tempfile::tempdir().unwrap();
    let demo = tmp.path().join("demo");
    std::fs::create_dir_all(&demo).unwrap();
    std::fs::write(
        demo.join("service.manifest.json"),
        r#"{"id":"demo","alicia_capabilities":[{"intent":"ping","phrases_examples":["x"],"params":{},"writes":false}]}"#,
    )
    .unwrap();
    let runtime = Arc::new(
        CapabilitiesRuntime::new_with_bypass_gate(CapabilitiesRuntimeConfig {
            services_dir: Some(tmp.path().to_path_buf()),
            ..Default::default()
        })
        .await
        .unwrap(),
    );
    let mock = MockServiceClient::new();
    mock.expect("demo.ping", ServiceCallResult::ok(json!({"pong": true})));
    runtime
        .register_service_client("demo", Arc::new(mock))
        .await;

    // Simule une réponse LLM qui contient un tool_call
    let response = json!({
        "choices": [{
            "message": {
                "role": "assistant",
                "tool_calls": [{
                    "id": "call_42",
                    "type": "function",
                    "function": {"name": "demo.ping", "arguments": "{}"}
                }]
            }
        }]
    });
    let calls = CapabilitiesRuntime::extract_tool_calls(&response).expect("some calls");
    let handled = runtime
        .handle_tool_calls(&calls, "voice", "ping")
        .await
        .unwrap();
    assert_eq!(handled.len(), 1);
    assert_eq!(handled[0].id, "call_42");
    assert_eq!(handled[0].qualified_id, "demo.ping");
    let tool_msg = handled[0].tool_message.as_ref().expect("tool message");
    assert_eq!(tool_msg["role"], "tool");
    assert_eq!(tool_msg["tool_call_id"], "call_42");
    // content est une string JSON ; on la reparse
    let inner: serde_json::Value =
        serde_json::from_str(tool_msg["content"].as_str().unwrap()).unwrap();
    assert_eq!(inner["success"], true);
    assert_eq!(inner["data"]["pong"], true);
}

#[tokio::test]
async fn build_system_context_includes_facts_and_episodes() {
    let runtime = Arc::new(
        CapabilitiesRuntime::new_with_bypass_gate(CapabilitiesRuntimeConfig::default())
            .await
            .unwrap(),
    );
    let mem = runtime.memory();
    mem.upsert_persona(&Persona::new(
        "voice".into(),
        "Voix".into(),
        "Persona vocal principal.".into(),
        true,
    ))
    .unwrap();
    mem.remember(&miyualicia_memory::Fact::new(
        "voice".into(),
        "user".into(),
        "Jean préfère le métro".into(),
        "user".into(),
    ))
    .unwrap();

    let ctx = runtime.build_system_context("voice").unwrap();
    assert!(ctx.contains("Voix"));
    assert!(ctx.contains("Persona vocal principal."));
    assert!(ctx.contains("Jean préfère le métro"));
}

#[tokio::test]
async fn detect_then_propose_pattern_via_runtime() {
    let runtime = CapabilitiesRuntime::new_with_bypass_gate(CapabilitiesRuntimeConfig::default())
        .await
        .unwrap();
    let mem = runtime.memory();
    for _ in 0..3 {
        mem.record_episode(&miyualicia_memory::Episode::new(
            "voice".into(),
            "demo".into(),
            "ping".into(),
            "ping".into(),
            json!({}),
            miyualicia_memory::EpisodeOutcome::Ok,
            None,
        ))
        .unwrap();
    }
    let patterns = runtime.detect_patterns("voice").await.unwrap();
    assert_eq!(patterns.len(), 1);
    let proposed = runtime.propose_pattern(&patterns[0]).await.unwrap();
    assert_eq!(proposed.qualified_id, "demo.ping");
    let pending = mem.pending_learned_skills("voice").unwrap();
    assert_eq!(pending.len(), 1);
}
