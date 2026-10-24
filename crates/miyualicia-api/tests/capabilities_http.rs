//! Tests d'intégration HTTP : router REST + handlers `/capabilities/*`.
//!
//! Stack montée par chaque test :
//! - `AliciaService::new(empty_config)` (pas de MQTT, pas de devices)
//! - `CapabilitiesRuntime::new_with_bypass_gate(...)` activé
//! - clients de service mock enregistrés
//! - JWT secret de test partagé
//! - tower::ServiceExt::oneshot pour les requêtes
//! - http_body_util::BodyExt pour parser les bodies

use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

use miyualicia::{AliciaConfig, AliciaService, CapabilitiesRuntime, CapabilitiesRuntimeConfig};
use miyualicia_api::auth::{generate_token, JwtClaims, JwtScope};
use miyualicia_api::config::ApiConfig;
use miyualicia_api::router::{build_router, AppState};
use miyualicia_dispatcher::{MockServiceClient, ServiceCallResult};
use miyualicia_memory::Persona;

const TEST_SECRET: &[u8] = b"\
    test-secret-key-32-bytes-minimum-required-here\
    abcdefghijklmnopqrstuvwxyz0123456789\
";

/// Construit un AppState minimal + un token JWT de test à scope admin.
async fn setup_with_runtime(services_dir: Option<&Path>) -> (axum::Router, String, Arc<CapabilitiesRuntime>) {
    let alicia = Arc::new(AliciaService::new(AliciaConfig {
        mqtt: None,
        ..Default::default()
    }));

    let cfg = CapabilitiesRuntimeConfig {
        services_dir: services_dir.map(|p| p.to_path_buf()),
        ..Default::default()
    };
    let runtime = Arc::new(
        CapabilitiesRuntime::new_with_bypass_gate(cfg)
            .await
            .expect("runtime"),
    );
    alicia.enable_capabilities_runtime(runtime.clone()).await;

    let api_config = ApiConfig::default();
    let router = build_router(alicia, api_config, TEST_SECRET.to_vec());

    let claims = JwtClaims::new("test-client", vec![JwtScope::Admin], 3600);
    let token = generate_token(&claims, TEST_SECRET).expect("token");

    (router, token, runtime)
}

fn auth_header(token: &str) -> (header::HeaderName, String) {
    (header::AUTHORIZATION, format!("Bearer {token}"))
}

async fn parse_body(body: Body) -> Value {
    let bytes = body.collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap_or(Value::Null)
}

async fn json_get(router: axum::Router, uri: &str, token: &str) -> (StatusCode, Value) {
    let req = Request::builder()
        .uri(uri)
        .method("GET")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    let status = resp.status();
    let body = parse_body(resp.into_body()).await;
    (status, body)
}

async fn json_post(
    router: axum::Router,
    uri: &str,
    token: &str,
    body: Value,
) -> (StatusCode, Value) {
    let req = Request::builder()
        .uri(uri)
        .method("POST")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&body).unwrap()))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    let status = resp.status();
    let body = parse_body(resp.into_body()).await;
    (status, body)
}

fn write_demo_manifest(dir: &Path) {
    let sub = dir.join("demo");
    std::fs::create_dir_all(&sub).unwrap();
    std::fs::write(
        sub.join("service.manifest.json"),
        r#"{
            "id": "demo",
            "alicia_capabilities": [
                {"intent": "ping", "phrases_examples": ["ping"], "params": {}, "writes": false},
                {"intent": "shout", "phrases_examples": ["shout"], "params": {}, "writes": true, "confirmation_default": "always_for_writes"}
            ]
        }"#,
    )
    .unwrap();
}

// -------------------------------------------------------------------
// Auth — sans token, 401
// -------------------------------------------------------------------

#[tokio::test]
async fn capabilities_endpoint_without_token_returns_401() {
    let (router, _token, _runtime) = setup_with_runtime(None).await;
    let req = Request::builder()
        .uri("/api/v1/alicia/capabilities/tools")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
}

// -------------------------------------------------------------------
// GET /capabilities/tools
// -------------------------------------------------------------------

#[tokio::test]
async fn tools_returns_loaded_capabilities() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_manifest(tmp.path());
    let (router, token, _runtime) = setup_with_runtime(Some(tmp.path())).await;
    let (status, body) = json_get(router, "/api/v1/alicia/capabilities/tools", &token).await;
    assert_eq!(status, StatusCode::OK);
    let tools = body["tools"].as_array().unwrap();
    assert_eq!(tools.len(), 2);
    let names: Vec<&str> = tools.iter().filter_map(|t| t["function"]["name"].as_str()).collect();
    assert!(names.contains(&"demo.ping"));
    assert!(names.contains(&"demo.shout"));
}

#[tokio::test]
async fn tools_empty_when_no_services_dir() {
    let (router, token, _runtime) = setup_with_runtime(None).await;
    let (status, body) = json_get(router, "/api/v1/alicia/capabilities/tools", &token).await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["tools"].as_array().unwrap().is_empty());
}

// -------------------------------------------------------------------
// GET /capabilities/services
// -------------------------------------------------------------------

#[tokio::test]
async fn services_lists_registered_clients() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_manifest(tmp.path());
    let (router, token, runtime) = setup_with_runtime(Some(tmp.path())).await;
    runtime
        .register_service_client("demo", Arc::new(MockServiceClient::new()))
        .await;
    let (status, body) = json_get(router, "/api/v1/alicia/capabilities/services", &token).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["services"], json!(["demo"]));
}

// -------------------------------------------------------------------
// POST /capabilities/dispatch
// -------------------------------------------------------------------

#[tokio::test]
async fn dispatch_read_only_capability_returns_executed() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_manifest(tmp.path());
    let (router, token, runtime) = setup_with_runtime(Some(tmp.path())).await;
    let mock = MockServiceClient::new();
    mock.expect("demo.ping", ServiceCallResult::ok(json!({"pong": true})));
    runtime
        .register_service_client("demo", Arc::new(mock))
        .await;

    let (status, body) = json_post(
        router,
        "/api/v1/alicia/capabilities/dispatch",
        &token,
        json!({
            "qualified_id": "demo.ping",
            "params": {},
            "persona": "voice",
            "user_phrase": "ping"
        }),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["kind"], "executed");
    assert_eq!(body["qualified_id"], "demo.ping");
    assert_eq!(body["service_result"]["data"]["pong"], true);
}

#[tokio::test]
async fn dispatch_write_capability_returns_pending() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_manifest(tmp.path());
    let (router, token, runtime) = setup_with_runtime(Some(tmp.path())).await;
    let mock = MockServiceClient::new();
    runtime
        .register_service_client("demo", Arc::new(mock))
        .await;

    let (status, body) = json_post(
        router,
        "/api/v1/alicia/capabilities/dispatch",
        &token,
        json!({
            "qualified_id": "demo.shout",
            "params": {},
            "persona": "voice",
            "user_phrase": "shout"
        }),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["kind"], "pending_confirmation");
}

#[tokio::test]
async fn dispatch_unknown_capability_returns_500_with_clear_message() {
    let (router, token, _runtime) = setup_with_runtime(None).await;
    let (status, _body) = json_post(
        router,
        "/api/v1/alicia/capabilities/dispatch",
        &token,
        json!({
            "qualified_id": "ghost.x",
            "params": {},
            "persona": "voice",
            "user_phrase": "?"
        }),
    )
    .await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
}

// -------------------------------------------------------------------
// POST /capabilities/dispatch/confirm
// -------------------------------------------------------------------

#[tokio::test]
async fn dispatch_confirm_executes_write_capability() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_manifest(tmp.path());
    let (router, token, runtime) = setup_with_runtime(Some(tmp.path())).await;
    let mock = MockServiceClient::new();
    mock.expect("demo.shout", ServiceCallResult::ok(json!({"shouted": true})));
    runtime
        .register_service_client("demo", Arc::new(mock))
        .await;

    let (status, body) = json_post(
        router,
        "/api/v1/alicia/capabilities/dispatch/confirm",
        &token,
        json!({
            "qualified_id": "demo.shout",
            "params": {},
            "persona": "voice",
            "user_phrase": "shout"
        }),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["kind"], "executed");
    assert_eq!(body["service_result"]["data"]["shouted"], true);
}

// -------------------------------------------------------------------
// POST /capabilities/dispatch/cancel
// -------------------------------------------------------------------

#[tokio::test]
async fn dispatch_cancel_records_episode_returns_204() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_manifest(tmp.path());
    let (router, token, runtime) = setup_with_runtime(Some(tmp.path())).await;

    let req = Request::builder()
        .uri("/api/v1/alicia/capabilities/dispatch/cancel")
        .method("POST")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            serde_json::to_vec(&json!({
                "qualified_id": "demo.shout",
                "params": {},
                "persona": "voice",
                "user_phrase": "annule"
            }))
            .unwrap(),
        ))
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let recent = runtime.memory().recent_episodes(10).unwrap();
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].outcome, miyualicia_memory::EpisodeOutcome::Cancelled);
}

// -------------------------------------------------------------------
// Apprentissage — pending / accept / reject / detect
// -------------------------------------------------------------------

#[tokio::test]
async fn pending_learnings_returns_proposed_skills() {
    let (router, token, runtime) = setup_with_runtime(None).await;
    let mem = runtime.memory();
    mem.upsert_persona(&Persona::new("voice".into(), "Voix".into(), "".into(), true))
        .unwrap();
    mem.propose_learned_skill("voice", "x", "demo.x", json!({}))
        .unwrap();
    let (status, body) = json_get(
        router,
        "/api/v1/alicia/capabilities/learnings/pending/voice",
        &token,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let pending = body["pending"].as_array().unwrap();
    assert_eq!(pending.len(), 1);
}

#[tokio::test]
async fn accept_learning_makes_it_active() {
    let (router, token, runtime) = setup_with_runtime(None).await;
    let mem = runtime.memory();
    mem.upsert_persona(&Persona::new("voice".into(), "Voix".into(), "".into(), true))
        .unwrap();
    let ls = mem
        .propose_learned_skill("voice", "x", "demo.x", json!({}))
        .unwrap();
    let id = ls.id.to_string();

    let req = Request::builder()
        .uri(format!(
            "/api/v1/alicia/capabilities/learnings/{id}/accept"
        ))
        .method("POST")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    let active = mem.active_learned_skills("voice").unwrap();
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].id, ls.id);
}

#[tokio::test]
async fn detect_learnings_returns_patterns() {
    let (router, token, runtime) = setup_with_runtime(None).await;
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
    let (status, body) = json_post(
        router,
        "/api/v1/alicia/capabilities/learnings/detect/voice",
        &token,
        json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let patterns = body["patterns"].as_array().unwrap();
    assert_eq!(patterns.len(), 1);
    assert_eq!(patterns[0]["qualified_id"], "demo.ping");
}

// -------------------------------------------------------------------
// Persona active
// -------------------------------------------------------------------

#[tokio::test]
async fn persona_active_returns_default() {
    let (router, token, runtime) = setup_with_runtime(None).await;
    runtime
        .memory()
        .upsert_persona(&Persona::new(
            "voice".into(),
            "Voix".into(),
            "".into(),
            true,
        ))
        .unwrap();
    let (status, body) = json_get(
        router,
        "/api/v1/alicia/capabilities/persona/active",
        &token,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["persona_id"], "voice");
}

#[tokio::test]
async fn persona_activate_switches_active() {
    let (router, token, runtime) = setup_with_runtime(None).await;
    let mem = runtime.memory();
    mem.upsert_persona(&Persona::new("voice".into(), "Voix".into(), "".into(), true))
        .unwrap();
    mem.upsert_persona(&Persona::new("work".into(), "Travail".into(), "".into(), false))
        .unwrap();

    let req = Request::builder()
        .uri("/api/v1/alicia/capabilities/persona/work/activate")
        .method("POST")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);
    assert_eq!(mem.active_persona_id().unwrap().as_deref(), Some("work"));
}

#[tokio::test]
async fn persona_activate_unknown_returns_404() {
    let (router, token, _runtime) = setup_with_runtime(None).await;
    let req = Request::builder()
        .uri("/api/v1/alicia/capabilities/persona/ghost/activate")
        .method("POST")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// -------------------------------------------------------------------
// Manifest reload
// -------------------------------------------------------------------

#[tokio::test]
async fn manifest_reload_picks_up_new_manifest() {
    let tmp = tempfile::tempdir().unwrap();
    let (router, token, runtime) = setup_with_runtime(Some(tmp.path())).await;
    // Pas de manifeste au démarrage → 0 capabilities
    assert!(runtime.list_tools().await.is_empty());

    write_demo_manifest(tmp.path());
    let (status, body) = json_post(
        router,
        "/api/v1/alicia/capabilities/manifest/reload",
        &token,
        json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["outcome"], "reloaded");
    assert_eq!(body["added"], json!(["demo"]));
    assert_eq!(runtime.list_tools().await.len(), 2);
}

// -------------------------------------------------------------------
// Episodes
// -------------------------------------------------------------------

#[tokio::test]
async fn episodes_recent_lists_dispatched_calls() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_manifest(tmp.path());
    let (router, token, runtime) = setup_with_runtime(Some(tmp.path())).await;
    let mock = MockServiceClient::new();
    mock.expect("demo.ping", ServiceCallResult::ok(json!({})));
    runtime
        .register_service_client("demo", Arc::new(mock))
        .await;

    // Dispatcher 2 calls
    for phrase in ["ping1", "ping2"] {
        runtime
            .dispatch_capability("demo.ping", &Value::Null, "voice", phrase)
            .await
            .unwrap();
    }

    let (status, body) = json_get(
        router,
        "/api/v1/alicia/capabilities/episodes/recent?limit=10",
        &token,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["count"], 2);
    let episodes = body["episodes"].as_array().unwrap();
    assert_eq!(episodes[0]["user_phrase"], "ping2");
    assert_eq!(episodes[1]["user_phrase"], "ping1");
}

// -------------------------------------------------------------------
// Queue async — dispatch_and_queue + pending_confirmations endpoints
// -------------------------------------------------------------------

fn write_demo_with_writes(dir: &Path) {
    let sub = dir.join("demo");
    std::fs::create_dir_all(&sub).unwrap();
    std::fs::write(
        sub.join("service.manifest.json"),
        r#"{
            "id": "demo",
            "alicia_capabilities": [
                {"intent": "ping", "phrases_examples": ["x"], "params": {}, "writes": false},
                {"intent": "shout", "description": "Crier", "phrases_examples": ["x"],
                 "params": {}, "writes": true, "confirmation_default": "always_for_writes"}
            ]
        }"#,
    )
    .unwrap();
}

#[tokio::test]
async fn dispatch_and_queue_executed_for_read_only() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_with_writes(tmp.path());
    let (router, token, runtime) = setup_with_runtime(Some(tmp.path())).await;
    let mock = MockServiceClient::new();
    mock.expect("demo.ping", ServiceCallResult::ok(json!({"pong": true})));
    runtime
        .register_service_client("demo", Arc::new(mock))
        .await;

    let (status, body) = json_post(
        router,
        "/api/v1/alicia/capabilities/dispatch_and_queue",
        &token,
        json!({
            "qualified_id": "demo.ping",
            "params": {},
            "persona": "voice",
            "user_phrase": "ping"
        }),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["kind"], "executed");
}

#[tokio::test]
async fn dispatch_and_queue_returns_id_for_pending() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_with_writes(tmp.path());
    let (router, token, runtime) = setup_with_runtime(Some(tmp.path())).await;
    let mock = MockServiceClient::new();
    runtime
        .register_service_client("demo", Arc::new(mock))
        .await;

    let (status, body) = json_post(
        router,
        "/api/v1/alicia/capabilities/dispatch_and_queue",
        &token,
        json!({
            "qualified_id": "demo.shout",
            "params": {"loud": true},
            "persona": "voice",
            "user_phrase": "shout"
        }),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["kind"], "queued");
    let id = body["id"].as_str().expect("id");
    let _: uuid::Uuid = id.parse().expect("UUID parsable");
    assert_eq!(body["qualified_id"], "demo.shout");
}

#[tokio::test]
async fn pending_list_endpoint_returns_queued() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_with_writes(tmp.path());
    let (router, token, runtime) = setup_with_runtime(Some(tmp.path())).await;
    runtime
        .register_service_client("demo", Arc::new(MockServiceClient::new()))
        .await;

    // Pose deux pending pour 2 personas distinctes
    runtime
        .dispatch_and_queue("demo.shout", &Value::Null, "voice", "x")
        .await
        .unwrap();
    runtime
        .dispatch_and_queue("demo.shout", &Value::Null, "work", "y")
        .await
        .unwrap();

    let (status, body) = json_get(
        router.clone(),
        "/api/v1/alicia/capabilities/pending_confirmations",
        &token,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["count"], 2);

    // Filtre persona
    let (s2, b2) = json_get(
        router,
        "/api/v1/alicia/capabilities/pending_confirmations?persona=voice",
        &token,
    )
    .await;
    assert_eq!(s2, StatusCode::OK);
    assert_eq!(b2["count"], 1);
}

#[tokio::test]
async fn pending_confirm_endpoint_executes_and_resolves() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_with_writes(tmp.path());
    let (router, token, runtime) = setup_with_runtime(Some(tmp.path())).await;
    let mock = MockServiceClient::new();
    mock.expect("demo.shout", ServiceCallResult::ok(json!({"shouted": true})));
    runtime
        .register_service_client("demo", Arc::new(mock))
        .await;

    // Enqueue
    let outcome = runtime
        .dispatch_and_queue("demo.shout", &Value::Null, "voice", "x")
        .await
        .unwrap();
    let id = match outcome {
        miyualicia::capabilities_runtime::QueuedDispatch::Queued { id, .. } => id,
        _ => panic!("expected Queued"),
    };

    let (status, body) = json_post(
        router,
        &format!(
            "/api/v1/alicia/capabilities/pending_confirmations/{id}/confirm"
        ),
        &token,
        json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["kind"], "executed");
    assert_eq!(body["service_result"]["data"]["shouted"], true);
}

#[tokio::test]
async fn pending_cancel_endpoint_returns_204() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_with_writes(tmp.path());
    let (router, token, runtime) = setup_with_runtime(Some(tmp.path())).await;
    runtime
        .register_service_client("demo", Arc::new(MockServiceClient::new()))
        .await;
    let outcome = runtime
        .dispatch_and_queue("demo.shout", &Value::Null, "voice", "x")
        .await
        .unwrap();
    let id = match outcome {
        miyualicia::capabilities_runtime::QueuedDispatch::Queued { id, .. } => id,
        _ => panic!("expected Queued"),
    };

    let req = Request::builder()
        .uri(format!(
            "/api/v1/alicia/capabilities/pending_confirmations/{id}/cancel"
        ))
        .method("POST")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();
    let resp = router.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // Episode Cancelled posé
    let recent = runtime.memory().recent_episodes(10).unwrap();
    assert_eq!(recent[0].outcome, miyualicia_memory::EpisodeOutcome::Cancelled);
}

#[tokio::test]
async fn pending_confirm_unknown_id_returns_500() {
    let (router, token, _runtime) = setup_with_runtime(None).await;
    let id = uuid::Uuid::new_v4();
    let (status, _body) = json_post(
        router,
        &format!(
            "/api/v1/alicia/capabilities/pending_confirmations/{id}/confirm"
        ),
        &token,
        json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
}

// -------------------------------------------------------------------
// Smoke test : sans runtime activé → 500
// -------------------------------------------------------------------

#[tokio::test]
async fn dispatch_returns_500_when_runtime_disabled() {
    // AliciaService sans enable_capabilities_runtime
    let alicia = Arc::new(AliciaService::new(AliciaConfig {
        mqtt: None,
        ..Default::default()
    }));
    let api_config = ApiConfig::default();
    let router = build_router(alicia, api_config, TEST_SECRET.to_vec());
    let claims = JwtClaims::new("test-client", vec![JwtScope::Admin], 3600);
    let token = generate_token(&claims, TEST_SECRET).expect("token");

    let (status, _body) = json_post(
        router,
        "/api/v1/alicia/capabilities/dispatch",
        &token,
        json!({
            "qualified_id": "demo.ping",
            "params": {},
            "persona": "voice",
            "user_phrase": "?"
        }),
    )
    .await;
    assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
    let _ = Instant::now;
}
