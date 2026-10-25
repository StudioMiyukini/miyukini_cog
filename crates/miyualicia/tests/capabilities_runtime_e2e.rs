//! Tests d'intégration : `AliciaService` + `CapabilitiesRuntime`.
//!
//! Valide la composition : un `AliciaService` peut activer le sous-système
//! des capacités, dispatcher une capacité réelle (JayKoa via fake backend),
//! et conserver le comportement device historique intact.

use std::sync::Arc;

use serde_json::{json, Value};

use borderguard::TrustLevel;
use miyualicia::{
    AliciaConfig, AliciaService, CapabilitiesRuntime, CapabilitiesRuntimeConfig, PendingExpiryConfig,
};
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
    mem.upsert_persona(&Persona::new("voice".into(), "Voix".into(), String::new(), true))
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
        miyualicia::capabilities_runtime::QueuedDispatch::Queued { .. } => panic!("expected Executed (read-only sans confirmation)"),
    }
    // Pas de pending posée
    assert_eq!(runtime.list_pending_confirmations().await.unwrap(), [] as [miyualicia_memory::PendingConfirmation; 0]);
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
        miyualicia::capabilities_runtime::QueuedDispatch::Executed(_) => panic!("expected Queued"),
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
    let miyualicia::capabilities_runtime::QueuedDispatch::Queued { id, .. } = outcome else {
        panic!("expected Queued")
    };

    let result = runtime.confirm_queued(id).await.unwrap();
    assert_eq!(result.service_result.data["shouted"], true);

    // La pending est désormais Confirmed et n'apparaît plus dans la liste pending
    assert_eq!(runtime.list_pending_confirmations().await.unwrap(), [] as [miyualicia_memory::PendingConfirmation; 0]);
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
    let miyualicia::capabilities_runtime::QueuedDispatch::Queued { id, .. } = outcome else {
        panic!("expected Queued")
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
    let miyualicia::capabilities_runtime::QueuedDispatch::Queued { id, .. } = outcome else {
        panic!("expected Queued")
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

// -------------------------------------------------------------------
// R-017 : confirmation concurrente — une seule exécution
// -------------------------------------------------------------------

/// Client de service lent qui compte ses appels : l'attente force
/// l'entrelacement des deux confirmations concurrentes.
struct SlowCountingClient {
    calls: std::sync::atomic::AtomicUsize,
}

impl miyualicia_dispatcher::ServiceClient for SlowCountingClient {
    // Forme désucrée d'`async_trait` (évite une dépendance de dev).
    fn call<'life0, 'life1, 'life2, 'async_trait>(
        &'life0 self,
        _qualified_id: &'life1 str,
        _params: &'life2 Value,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = ServiceCallResult> + Send + 'async_trait>,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            self.calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            ServiceCallResult::ok(json!({"done": true}))
        })
    }
}

async fn runtime_with_shout(tmp: &std::path::Path) -> Arc<CapabilitiesRuntime> {
    runtime_with_shout_cfg(tmp, None, PendingExpiryConfig::default()).await
}

/// Variante paramétrable : base SQLite fichier (redémarrage simulé) et TTL
/// d'expiration des confirmations.
async fn runtime_with_shout_cfg(
    tmp: &std::path::Path,
    memory_db_path: Option<std::path::PathBuf>,
    pending_expiry: PendingExpiryConfig,
) -> Arc<CapabilitiesRuntime> {
    let demo = tmp.join("demo");
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
    Arc::new(
        CapabilitiesRuntime::new_with_bypass_gate(CapabilitiesRuntimeConfig {
            services_dir: Some(tmp.to_path_buf()),
            memory_db_path,
            pending_expiry,
            ..Default::default()
        })
        .await
        .unwrap(),
    )
}

#[tokio::test]
async fn concurrent_double_confirm_executes_once() {
    let tmp = tempfile::tempdir().unwrap();
    let runtime = runtime_with_shout(tmp.path()).await;
    let client = Arc::new(SlowCountingClient {
        calls: std::sync::atomic::AtomicUsize::new(0),
    });
    runtime.register_service_client("demo", client.clone()).await;

    let id = queued_id(
        runtime
        .dispatch_and_queue("demo.shout", &Value::Null, "voice", "x")
        .await
        .unwrap(),
    );

    let (a, b) = tokio::join!(runtime.confirm_queued(id), runtime.confirm_queued(id));
    let oks = [a.is_ok(), b.is_ok()].iter().filter(|x| **x).count();
    assert_eq!(oks, 1, "une seule confirmation doit réussir");
    let refusal = if a.is_err() { a.unwrap_err() } else { b.unwrap_err() };
    assert!(
        matches!(
            refusal,
            miyualicia::AliciaError::PendingConfirmationAlreadyResolved { .. }
        ),
        "refus typé attendu, obtenu {refusal:?}"
    );
    assert_eq!(
        client.calls.load(std::sync::atomic::Ordering::SeqCst),
        1,
        "le service ne doit être appelé qu'une fois"
    );
    let resolved = runtime.pending_confirmation(id).await.unwrap().unwrap();
    assert_eq!(resolved.status, miyualicia_memory::PendingStatus::Confirmed);
}

#[tokio::test]
async fn confirm_failure_marks_failed_and_is_not_retryable() {
    let tmp = tempfile::tempdir().unwrap();
    let runtime = runtime_with_shout(tmp.path()).await;
    // Aucun ServiceClient enregistré pour `demo` : l'exécution échoue.
    let id = queued_id(
        runtime
        .dispatch_and_queue("demo.shout", &Value::Null, "voice", "x")
        .await
        .unwrap(),
    );
    assert!(runtime.confirm_queued(id).await.is_err());
    let pc = runtime.pending_confirmation(id).await.unwrap().unwrap();
    assert_eq!(pc.status, miyualicia_memory::PendingStatus::Failed);
    // Pas de nouvelle tentative possible
    let err = runtime.confirm_queued(id).await.unwrap_err();
    assert!(matches!(
        err,
        miyualicia::AliciaError::PendingConfirmationAlreadyResolved { .. }
    ));
}

#[tokio::test]
async fn cancel_after_confirm_is_refused() {
    let tmp = tempfile::tempdir().unwrap();
    let runtime = runtime_with_shout(tmp.path()).await;
    runtime
        .register_service_client("demo", Arc::new(MockServiceClient::new()))
        .await;
    let id = queued_id(
        runtime
        .dispatch_and_queue("demo.shout", &Value::Null, "voice", "x")
        .await
        .unwrap(),
    );
    runtime.confirm_queued(id).await.unwrap();
    assert!(runtime.cancel_queued(id).await.is_err());
}

#[tokio::test]
async fn handle_tool_calls_queues_pending_confirmation() {
    let tmp = tempfile::tempdir().unwrap();
    let runtime = runtime_with_shout(tmp.path()).await;
    let mock = Arc::new(MockServiceClient::new());
    runtime.register_service_client("demo", mock.clone()).await;
    let response = json!({
        "choices": [{
            "message": {
                "role": "assistant",
                "tool_calls": [{
                    "id": "call_1",
                    "type": "function",
                    "function": {"name": "demo.shout", "arguments": "{\"loud\":true}"}
                }]
            }
        }]
    });
    let calls = CapabilitiesRuntime::extract_tool_calls(&response).expect("calls");
    let handled = runtime.handle_tool_calls(&calls, "voice", "crie").await.unwrap();
    assert!(handled[0].tool_message.is_none());
    let pid = handled[0].pending_id.expect("pending mise en file");
    assert_eq!(mock.calls().len(), 0, "rien d'exécuté avant confirmation");
    let pc = runtime.pending_confirmation(pid).await.unwrap().unwrap();
    assert_eq!(pc.qualified_id, "demo.shout");
    assert_eq!(pc.params, json!({"loud": true}));
    runtime.confirm_queued(pid).await.unwrap();
    assert_eq!(mock.calls().len(), 1);
}

/// Extrait l'id d'une `PendingConfirmation` mise en file.
fn queued_id(outcome: miyualicia::capabilities_runtime::QueuedDispatch) -> uuid::Uuid {
    let miyualicia::capabilities_runtime::QueuedDispatch::Queued { id, .. } = outcome else {
        panic!("expected Queued");
    };
    id
}

// -------------------------------------------------------------------
// Expiration des confirmations : Executing bloquée → Failed (jamais
// ré-exécutée), Pending périmée → Cancelled (plus confirmable)
// -------------------------------------------------------------------

fn counting_client() -> Arc<SlowCountingClient> {
    Arc::new(SlowCountingClient {
        calls: std::sync::atomic::AtomicUsize::new(0),
    })
}

/// Simule un arrêt brutal pendant l'exécution : la confirmation est
/// réclamée (`Executing`) puis le runtime disparaît sans finaliser.
async fn crash_mid_execution(tmp: &std::path::Path, db: &std::path::Path) -> uuid::Uuid {
    let rt = runtime_with_shout_cfg(tmp, Some(db.to_path_buf()), PendingExpiryConfig::default()).await;
    let id = queued_id(
        rt.dispatch_and_queue("demo.shout", &Value::Null, "voice", "x")
            .await
            .unwrap(),
    );
    rt.memory().claim_pending_confirmation(id).unwrap();
    id
}

#[tokio::test]
async fn stuck_executing_older_than_ttl_is_failed_at_startup_and_never_executed() {
    let tmp = tempfile::tempdir().unwrap();
    let db = tmp.path().join("memoire.db");
    let id = crash_mid_execution(tmp.path(), &db).await;

    // Redémarrage avec TTL Executing nul : la réclamation est « trop vieille ».
    let rt = runtime_with_shout_cfg(
        tmp.path(),
        Some(db.clone()),
        PendingExpiryConfig {
            executing_ttl_secs: 0,
            ..Default::default()
        },
    )
    .await;
    let client = counting_client();
    rt.register_service_client("demo", client.clone()).await;

    let pc = rt.memory().pending_confirmation(id).unwrap().unwrap();
    assert_eq!(pc.status, miyualicia_memory::PendingStatus::Failed);
    assert_eq!(pc.reason.as_deref(), Some("expired"));
    let err = rt.confirm_queued(id).await.unwrap_err();
    assert!(matches!(
        err,
        miyualicia::AliciaError::PendingConfirmationAlreadyResolved { .. }
    ));
    assert_eq!(client.calls.load(std::sync::atomic::Ordering::SeqCst), 0);
}

#[tokio::test]
async fn fresh_executing_is_untouched_at_startup_and_not_reexecuted() {
    let tmp = tempfile::tempdir().unwrap();
    let db = tmp.path().join("memoire.db");
    let id = crash_mid_execution(tmp.path(), &db).await;

    // Redémarrage avec le TTL par défaut (15 min) : rien n'expire encore.
    let rt = runtime_with_shout_cfg(tmp.path(), Some(db.clone()), PendingExpiryConfig::default()).await;
    let client = counting_client();
    rt.register_service_client("demo", client.clone()).await;
    let report = rt.sweep_expired_confirmations().unwrap();
    assert!(report.is_empty());
    let pc = rt.pending_confirmation(id).await.unwrap().unwrap();
    assert_eq!(pc.status, miyualicia_memory::PendingStatus::Executing);
    assert!(pc.reason.is_none());
    // Toujours non réclamable : aucune ré-exécution automatique ni manuelle.
    assert!(rt.confirm_queued(id).await.is_err());
    assert_eq!(client.calls.load(std::sync::atomic::Ordering::SeqCst), 0);
}

#[tokio::test]
async fn pending_older_than_ttl_is_cancelled_and_cannot_be_confirmed() {
    let tmp = tempfile::tempdir().unwrap();
    let rt = runtime_with_shout_cfg(
        tmp.path(),
        None,
        PendingExpiryConfig {
            pending_ttl_secs: 0,
            ..Default::default()
        },
    )
    .await;
    let client = counting_client();
    rt.register_service_client("demo", client.clone()).await;
    let id = queued_id(
        rt.dispatch_and_queue("demo.shout", &Value::Null, "voice", "x")
            .await
            .unwrap(),
    );
    // TTL nul : laisser s'écouler quelques millisecondes (la date de création
    // est convertie en ms par SQLite via julianday, arrondi flottant).
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    let err = rt.confirm_queued(id).await.unwrap_err();
    assert!(matches!(
        err,
        miyualicia::AliciaError::PendingConfirmationAlreadyResolved { .. }
    ));
    assert_eq!(client.calls.load(std::sync::atomic::Ordering::SeqCst), 0);
    let pc = rt.pending_confirmation(id).await.unwrap().unwrap();
    assert_eq!(pc.status, miyualicia_memory::PendingStatus::Cancelled);
    assert_eq!(pc.reason.as_deref(), Some("expired"));
    assert_eq!(rt.list_pending_confirmations().await.unwrap().len(), 0);
}

#[tokio::test]
async fn periodic_sweeper_expires_stuck_executing() {
    let tmp = tempfile::tempdir().unwrap();
    let rt = runtime_with_shout_cfg(
        tmp.path(),
        None,
        PendingExpiryConfig {
            executing_ttl_secs: 0,
            ..Default::default()
        },
    )
    .await;
    let id = queued_id(
        rt.dispatch_and_queue("demo.shout", &Value::Null, "voice", "x")
            .await
            .unwrap(),
    );
    rt.memory().claim_pending_confirmation(id).unwrap();
    let handle = rt.spawn_expiry_sweeper(std::time::Duration::from_secs(60));
    // Le premier tick est immédiat.
    let mut statut = miyualicia_memory::PendingStatus::Executing;
    for _ in 0..50 {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        statut = rt.memory().pending_confirmation(id).unwrap().unwrap().status;
        if statut != miyualicia_memory::PendingStatus::Executing {
            break;
        }
    }
    handle.abort();
    assert_eq!(statut, miyualicia_memory::PendingStatus::Failed);
}

#[test]
fn pending_expiry_config_defaults_and_toml() {
    let d = PendingExpiryConfig::default();
    assert_eq!(d.executing_ttl_secs, 900);
    assert_eq!(d.pending_ttl_secs, 86_400);
    let partiel: PendingExpiryConfig = serde_json::from_value(json!({"pending_ttl_secs": 3600})).unwrap();
    assert_eq!(partiel.executing_ttl_secs, 900);
    assert_eq!(partiel.pending_ttl_secs, 3600);
}
