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
use miyualicia_api::router::build_router;
use miyualicia_dispatcher::{MockServiceClient, ServiceCallResult};
use miyualicia_memory::Persona;

const TEST_SECRET: &[u8] = b"\
    test-secret-key-32-bytes-minimum-required-here\
    abcdefghijklmnopqrstuvwxyz0123456789\
";

/// Token de test : scope admin, personas `voice` et `work` autorisées.
fn token_with(scopes: Vec<JwtScope>, personas: &[&str]) -> String {
    let claims = JwtClaims::new("test-client", scopes, 3600)
        .with_personas(personas.iter().map(|p| (*p).to_string()).collect());
    generate_token(&claims, TEST_SECRET).expect("token")
}

/// Construit un AppState minimal + un token JWT de test à scope admin.
async fn setup_with_runtime(services_dir: Option<&Path>) -> (axum::Router, String, Arc<CapabilitiesRuntime>) {
    setup_with_runtime_cfg(CapabilitiesRuntimeConfig {
        services_dir: services_dir.map(std::path::Path::to_path_buf),
        ..Default::default()
    })
    .await
}

/// Variante avec configuration complète du runtime (ex. TTL d'expiration).
async fn setup_with_runtime_cfg(
    cfg: CapabilitiesRuntimeConfig,
) -> (axum::Router, String, Arc<CapabilitiesRuntime>) {
    let alicia = Arc::new(AliciaService::new(AliciaConfig {
        mqtt: None,
        ..Default::default()
    }));
    let runtime = Arc::new(
        CapabilitiesRuntime::new_with_bypass_gate(cfg)
            .await
            .expect("runtime"),
    );
    alicia.enable_capabilities_runtime(runtime.clone()).await;

    let api_config = ApiConfig::default();
    let router = build_router(alicia, api_config, TEST_SECRET.to_vec());

    let token = token_with(vec![JwtScope::Admin], &["voice", "work"]);

    (router, token, runtime)
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
    assert_eq!(body["tools"].as_array().unwrap().len(), 0);
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
    // R-016 : la pending est enregistrée et référencée par son id
    let id: uuid::Uuid = body["id"].as_str().expect("id").parse().expect("uuid");
    let pc = runtime.pending_confirmation(id).await.unwrap().expect("stockée");
    assert_eq!(pc.persona, "voice");
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

/// Ancien comportement (stateless : `qualified_id` + `params` dans le body)
/// supprimé par R-016 : la confirmation passe par l'`id` renvoyé par `/dispatch`.
#[tokio::test]
async fn dispatch_confirm_executes_stored_pending_by_id() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_manifest(tmp.path());
    let (router, token, runtime) = setup_with_runtime(Some(tmp.path())).await;
    let mock = Arc::new(MockServiceClient::new());
    mock.expect("demo.shout", ServiceCallResult::ok(json!({"shouted": true})));
    runtime.register_service_client("demo", mock.clone()).await;

    let (_, pending) = json_post(
        router.clone(),
        "/api/v1/alicia/capabilities/dispatch",
        &token,
        json!({"qualified_id": "demo.shout", "params": {"loud": true}, "persona": "voice", "user_phrase": "shout"}),
    )
    .await;
    assert_eq!(pending["kind"], "pending_confirmation");
    assert_eq!(mock.calls().len(), 0);

    let (status, body) = json_post(
        router,
        "/api/v1/alicia/capabilities/dispatch/confirm",
        &token,
        json!({"id": pending["id"]}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["kind"], "executed");
    assert_eq!(body["service_result"]["data"]["shouted"], true);
    // Paramètres exécutés = ceux stockés
    assert_eq!(mock.calls(), vec![("demo.shout".to_string(), json!({"loud": true}))]);
}

/// R-016 (négatif) : l'ancien body stateless est refusé, rien n'est exécuté.
#[tokio::test]
async fn dispatch_confirm_with_arbitrary_qualified_id_is_refused() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_manifest(tmp.path());
    let (router, token, runtime) = setup_with_runtime(Some(tmp.path())).await;
    let mock = Arc::new(MockServiceClient::new());
    runtime.register_service_client("demo", mock.clone()).await;

    let (status, _) = json_post(
        router.clone(),
        "/api/v1/alicia/capabilities/dispatch/confirm",
        &token,
        json!({"qualified_id": "demo.shout", "params": {}, "persona": "voice", "user_phrase": "shout"}),
    )
    .await;
    assert!(status.is_client_error(), "body stateless refusé, obtenu {status}");

    // id sans enregistrement en attente → 404
    let (status, _) = json_post(
        router.clone(),
        "/api/v1/alicia/capabilities/dispatch/confirm",
        &token,
        json!({"id": uuid::Uuid::new_v4()}),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);

    // id + champs supplémentaires (tentative de substitution) → refusé
    let id = queued_id(
        runtime
        .dispatch_and_queue("demo.shout", &json!({"a": 1}), "voice", "x")
        .await
        .unwrap(),
    );
    let (status, _) = json_post(
        router,
        "/api/v1/alicia/capabilities/dispatch/confirm",
        &token,
        json!({"id": id, "qualified_id": "demo.ping", "params": {"evil": true}}),
    )
    .await;
    assert!(status.is_client_error());
    assert_eq!(mock.calls().len(), 0, "aucune exécution");
    let pc = runtime.pending_confirmation(id).await.unwrap().unwrap();
    assert_eq!(pc.status, miyualicia_memory::PendingStatus::Pending);
}

// -------------------------------------------------------------------
// POST /capabilities/dispatch/cancel
// -------------------------------------------------------------------

#[tokio::test]
async fn dispatch_cancel_by_id_records_episode_returns_204() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_manifest(tmp.path());
    let (router, token, runtime) = setup_with_runtime(Some(tmp.path())).await;
    runtime
        .register_service_client("demo", Arc::new(MockServiceClient::new()))
        .await;
    let (_, pending) = json_post(
        router.clone(),
        "/api/v1/alicia/capabilities/dispatch",
        &token,
        json!({"qualified_id": "demo.shout", "params": {}, "persona": "voice", "user_phrase": "annule"}),
    )
    .await;

    let req = Request::builder()
        .uri("/api/v1/alicia/capabilities/dispatch/cancel")
        .method("POST")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            serde_json::to_vec(&json!({"id": pending["id"]})).unwrap(),
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
    mem.upsert_persona(&Persona::new("voice".into(), "Voix".into(), String::new(), true))
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
    mem.upsert_persona(&Persona::new("voice".into(), "Voix".into(), String::new(), true))
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
            String::new(),
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
    mem.upsert_persona(&Persona::new("voice".into(), "Voix".into(), String::new(), true))
        .unwrap();
    mem.upsert_persona(&Persona::new("work".into(), "Travail".into(), String::new(), false))
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
    // Persona autorisée par le token mais absente du store → 404. (Une persona
    // non autorisée est refusée en 403 avant toute recherche, R-002.)
    let (router, _admin, _runtime) = setup_with_runtime(None).await;
    let token = token_with(vec![JwtScope::Admin], &["ghost"]);
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
    assert_eq!(runtime.list_tools().await.len(), 0);

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
    // Épisode d'une persona non autorisée pour ce client : jamais renvoyé (R-002).
    runtime
        .dispatch_capability("demo.ping", &Value::Null, "intrus", "secret")
        .await
        .unwrap();

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
    let miyualicia::capabilities_runtime::QueuedDispatch::Queued { id, .. } = outcome else {
        panic!("expected Queued");
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
    let miyualicia::capabilities_runtime::QueuedDispatch::Queued { id, .. } = outcome else {
        panic!("expected Queued");
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
async fn pending_confirm_unknown_id_returns_404() {
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
    // Erreur typée (avant : 500 générique).
    assert_eq!(status, StatusCode::NOT_FOUND);
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
    let token = token_with(vec![JwtScope::Admin], &["voice"]);

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

// -------------------------------------------------------------------
// R-002 : persona dérivée de l'identité authentifiée
// -------------------------------------------------------------------

#[tokio::test]
async fn dispatch_with_foreign_persona_returns_403_and_executes_nothing() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_manifest(tmp.path());
    let (router, _admin, runtime) = setup_with_runtime(Some(tmp.path())).await;
    let mock = Arc::new(MockServiceClient::new());
    runtime.register_service_client("demo", mock.clone()).await;
    let voice_only = token_with(vec![JwtScope::Write], &["voice"]);

    for uri in [
        "/api/v1/alicia/capabilities/dispatch",
        "/api/v1/alicia/capabilities/dispatch_and_queue",
    ] {
        let (status, body) = json_post(
            router.clone(),
            uri,
            &voice_only,
            json!({"qualified_id": "demo.ping", "params": {}, "persona": "work", "user_phrase": "x"}),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{uri}");
        assert_eq!(body["error"]["type"], "PersonaForbidden");
    }
    assert_eq!(mock.calls().len(), 0);
    assert_eq!(runtime.memory().recent_episodes(10).unwrap().len(), 0);
}

#[tokio::test]
async fn admin_token_without_personas_cannot_dispatch() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_manifest(tmp.path());
    let (router, _admin, runtime) = setup_with_runtime(Some(tmp.path())).await;
    let mock = Arc::new(MockServiceClient::new());
    runtime.register_service_client("demo", mock.clone()).await;
    let no_persona = token_with(vec![JwtScope::Admin], &[]);
    for persona in [json!("voice"), Value::Null] {
        let (status, _) = json_post(
            router.clone(),
            "/api/v1/alicia/capabilities/dispatch",
            &no_persona,
            json!({"qualified_id": "demo.ping", "params": {}, "persona": persona, "user_phrase": "x"}),
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);
    }
    assert_eq!(mock.calls().len(), 0);
}

#[tokio::test]
async fn dispatch_without_persona_uses_authorized_persona() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_manifest(tmp.path());
    let (router, _admin, runtime) = setup_with_runtime(Some(tmp.path())).await;
    runtime
        .register_service_client("demo", Arc::new(MockServiceClient::new()))
        .await;
    let work_only = token_with(vec![JwtScope::Write], &["work"]);
    let (status, body) = json_post(
        router,
        "/api/v1/alicia/capabilities/dispatch",
        &work_only,
        json!({"qualified_id": "demo.ping", "params": {}, "user_phrase": "ping"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["kind"], "executed");
    let recent = runtime.memory().recent_episodes(1).unwrap();
    assert_eq!(recent[0].persona, "work");
}

#[tokio::test]
async fn confirm_or_cancel_pending_of_foreign_persona_returns_403() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_manifest(tmp.path());
    let (router, _admin, runtime) = setup_with_runtime(Some(tmp.path())).await;
    let mock = Arc::new(MockServiceClient::new());
    runtime.register_service_client("demo", mock.clone()).await;
    let id = queued_id(
        runtime
        .dispatch_and_queue("demo.shout", &Value::Null, "work", "x")
        .await
        .unwrap(),
    );
    let voice_only = token_with(vec![JwtScope::Write], &["voice"]);
    for (uri, body) in [
        (format!("/api/v1/alicia/capabilities/pending_confirmations/{id}/confirm"), json!({})),
        (format!("/api/v1/alicia/capabilities/pending_confirmations/{id}/cancel"), json!({})),
        ("/api/v1/alicia/capabilities/dispatch/confirm".to_string(), json!({"id": id})),
        ("/api/v1/alicia/capabilities/dispatch/cancel".to_string(), json!({"id": id})),
    ] {
        let (status, _) = json_post(router.clone(), &uri, &voice_only, body).await;
        assert_eq!(status, StatusCode::FORBIDDEN, "{uri}");
    }
    assert_eq!(mock.calls().len(), 0);
    let pc = runtime.pending_confirmation(id).await.unwrap().unwrap();
    assert_eq!(pc.status, miyualicia_memory::PendingStatus::Pending);
}

#[tokio::test]
async fn pending_list_only_shows_authorized_personas() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_with_writes(tmp.path());
    let (router, _admin, runtime) = setup_with_runtime(Some(tmp.path())).await;
    runtime
        .register_service_client("demo", Arc::new(MockServiceClient::new()))
        .await;
    for persona in ["voice", "work"] {
        runtime
            .dispatch_and_queue("demo.shout", &Value::Null, persona, "x")
            .await
            .unwrap();
    }
    let voice_only = token_with(vec![JwtScope::Read], &["voice"]);
    let (status, body) = json_get(
        router.clone(),
        "/api/v1/alicia/capabilities/pending_confirmations",
        &voice_only,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["count"], 1);
    assert_eq!(body["pending"][0]["persona"], "voice");
    let (status, _) = json_get(
        router,
        "/api/v1/alicia/capabilities/pending_confirmations?persona=work",
        &voice_only,
    )
    .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn persona_paths_require_authorized_persona() {
    let (router, _admin, runtime) = setup_with_runtime(None).await;
    let mem = runtime.memory();
    mem.upsert_persona(&Persona::new("voice".into(), "Voix".into(), String::new(), true))
        .unwrap();
    mem.upsert_persona(&Persona::new("work".into(), "Travail".into(), String::new(), false))
        .unwrap();
    let ls = mem
        .propose_learned_skill("work", "x", "demo.x", json!({}))
        .unwrap();
    let voice_only = token_with(vec![JwtScope::Write, JwtScope::Read], &["voice"]);

    let (st, _) = json_get(
        router.clone(),
        "/api/v1/alicia/capabilities/learnings/pending/work",
        &voice_only,
    )
    .await;
    assert_eq!(st, StatusCode::FORBIDDEN);
    for uri in [
        "/api/v1/alicia/capabilities/persona/work/activate".to_string(),
        "/api/v1/alicia/capabilities/learnings/detect/work".to_string(),
        format!("/api/v1/alicia/capabilities/learnings/{}/accept", ls.id),
    ] {
        let (st, _) = json_post(router.clone(), &uri, &voice_only, json!({})).await;
        assert_eq!(st, StatusCode::FORBIDDEN, "{uri}");
    }
    assert_eq!(mem.active_persona_id().unwrap().as_deref(), Some("voice"));
    assert_eq!(mem.active_learned_skills("work").unwrap().len(), 0);
}

// -------------------------------------------------------------------
// R-017 : deux confirmations HTTP concurrentes → une seule exécution
// -------------------------------------------------------------------

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

#[tokio::test]
async fn concurrent_http_confirms_execute_once() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_manifest(tmp.path());
    let (router, token, runtime) = setup_with_runtime(Some(tmp.path())).await;
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
    let uri = format!("/api/v1/alicia/capabilities/pending_confirmations/{id}/confirm");
    let ((s1, _), (s2, _)) = tokio::join!(
        json_post(router.clone(), &uri, &token, json!({})),
        json_post(router.clone(), &uri, &token, json!({})),
    );
    let mut statuses = [s1, s2];
    statuses.sort();
    assert_eq!(statuses, [StatusCode::OK, StatusCode::CONFLICT]);
    assert_eq!(client.calls.load(std::sync::atomic::Ordering::SeqCst), 1);
}

/// Extrait l'id d'une `PendingConfirmation` mise en file.
fn queued_id(outcome: miyualicia::capabilities_runtime::QueuedDispatch) -> uuid::Uuid {
    let miyualicia::capabilities_runtime::QueuedDispatch::Queued { id, .. } = outcome else {
        panic!("expected Queued");
    };
    id
}

// -------------------------------------------------------------------
// Décision propriétaire : client admin `all_personas`
// -------------------------------------------------------------------

/// Token admin avec le claim explicite `all_personas` (aucune persona listée).
fn token_all_personas(scopes: Vec<JwtScope>) -> String {
    let claims = JwtClaims::new("root", scopes, 3600).with_all_personas(true);
    generate_token(&claims, TEST_SECRET).expect("token")
}

fn seed_personas(runtime: &CapabilitiesRuntime, ids: &[&str]) {
    let mem = runtime.memory();
    for (i, id) in ids.iter().enumerate() {
        mem.upsert_persona(&Persona::new((*id).into(), (*id).into(), String::new(), i == 0))
            .unwrap();
    }
}

#[tokio::test]
async fn admin_all_personas_sees_and_acts_on_every_persona() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_manifest(tmp.path());
    let (router, _admin, runtime) = setup_with_runtime(Some(tmp.path())).await;
    seed_personas(&runtime, &["voice", "work", "family"]);
    let mock = Arc::new(MockServiceClient::new());
    mock.expect("demo.shout", ServiceCallResult::ok(json!({"ok": true})));
    mock.expect("demo.ping", ServiceCallResult::ok(json!({"pong": true})));
    runtime.register_service_client("demo", mock.clone()).await;
    let root = token_all_personas(vec![JwtScope::Admin]);

    // Dispatch pour une persona absente de toute liste explicite.
    let (st, body) = json_post(
        router.clone(),
        "/api/v1/alicia/capabilities/dispatch",
        &root,
        json!({"qualified_id": "demo.ping", "params": {}, "persona": "family", "user_phrase": "p"}),
    )
    .await;
    assert_eq!(st, StatusCode::OK, "{body}");
    assert_eq!(body["kind"], "executed");

    // Pending de plusieurs personas : toutes visibles, filtre libre.
    let id_work = queued_id(
        runtime
            .dispatch_and_queue("demo.shout", &Value::Null, "work", "w")
            .await
            .unwrap(),
    );
    runtime
        .dispatch_and_queue("demo.shout", &Value::Null, "voice", "v")
        .await
        .unwrap();
    let (st, body) = json_get(router.clone(), "/api/v1/alicia/capabilities/pending_confirmations", &root).await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(body["count"], 2);
    let (st, body) = json_get(
        router.clone(),
        "/api/v1/alicia/capabilities/pending_confirmations?persona=work",
        &root,
    )
    .await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(body["count"], 1);

    // Confirmation d'une pending d'une autre persona.
    let (st, body) = json_post(
        router.clone(),
        &format!("/api/v1/alicia/capabilities/pending_confirmations/{id_work}/confirm"),
        &root,
        json!({}),
    )
    .await;
    assert_eq!(st, StatusCode::OK, "{body}");
    assert_eq!(body["kind"], "executed");

    // Apprentissages, activation de persona, persona active.
    let ls = runtime
        .memory()
        .propose_learned_skill("work", "x", "demo.x", json!({}))
        .unwrap();
    let (st, body) = json_get(router.clone(), "/api/v1/alicia/capabilities/learnings/pending/work", &root).await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(body["pending"].as_array().unwrap().len(), 1);
    let (st, _) = json_post(
        router.clone(),
        &format!("/api/v1/alicia/capabilities/learnings/{}/accept", ls.id),
        &root,
        json!({}),
    )
    .await;
    assert_eq!(st, StatusCode::NO_CONTENT);
    let (st, _) = json_post(router.clone(), "/api/v1/alicia/capabilities/persona/family/activate", &root, json!({})).await;
    assert_eq!(st, StatusCode::NO_CONTENT);
    let (st, body) = json_get(router.clone(), "/api/v1/alicia/capabilities/persona/active", &root).await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(body["persona_id"], "family");

    // Épisodes : requête non filtrée (family + work au moins).
    let (st, body) = json_get(router.clone(), "/api/v1/alicia/capabilities/episodes/recent", &root).await;
    assert_eq!(st, StatusCode::OK);
    let personas: std::collections::BTreeSet<String> = body["episodes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|e| e["persona"].as_str().unwrap().to_string())
        .collect();
    assert!(personas.contains("family") && personas.contains("work"), "{personas:?}");
}

#[tokio::test]
async fn admin_all_personas_refuses_unknown_persona_and_executes_nothing() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_manifest(tmp.path());
    let (router, _admin, runtime) = setup_with_runtime(Some(tmp.path())).await;
    seed_personas(&runtime, &["voice"]);
    let mock = Arc::new(MockServiceClient::new());
    runtime.register_service_client("demo", mock.clone()).await;
    let root = token_all_personas(vec![JwtScope::Admin]);
    let (st, _) = json_post(
        router.clone(),
        "/api/v1/alicia/capabilities/dispatch",
        &root,
        json!({"qualified_id": "demo.ping", "params": {}, "persona": "fantome", "user_phrase": "p"}),
    )
    .await;
    assert_eq!(st, StatusCode::NOT_FOUND);
    let (st, _) = json_get(router, "/api/v1/alicia/capabilities/learnings/pending/fantome", &root).await;
    assert_eq!(st, StatusCode::NOT_FOUND);
    assert_eq!(mock.calls().len(), 0);
}

#[tokio::test]
async fn all_personas_claim_without_admin_scope_is_inert() {
    // Non-admin inchangé : même avec le claim (jamais émis par la config,
    // ici forgé avec la clé), sans scope admin l'accès reste 403.
    let tmp = tempfile::tempdir().unwrap();
    write_demo_manifest(tmp.path());
    let (router, _admin, runtime) = setup_with_runtime(Some(tmp.path())).await;
    seed_personas(&runtime, &["voice", "work"]);
    let mock = Arc::new(MockServiceClient::new());
    runtime.register_service_client("demo", mock.clone()).await;
    let id = queued_id(
        runtime
            .dispatch_and_queue("demo.shout", &Value::Null, "work", "x")
            .await
            .unwrap(),
    );
    let ecrivain = token_all_personas(vec![JwtScope::Read, JwtScope::Write]);
    let (st, _) = json_post(
        router.clone(),
        "/api/v1/alicia/capabilities/dispatch",
        &ecrivain,
        json!({"qualified_id": "demo.ping", "params": {}, "persona": "work", "user_phrase": "p"}),
    )
    .await;
    assert_eq!(st, StatusCode::FORBIDDEN);
    let (st, _) = json_post(
        router.clone(),
        &format!("/api/v1/alicia/capabilities/pending_confirmations/{id}/confirm"),
        &ecrivain,
        json!({}),
    )
    .await;
    assert_eq!(st, StatusCode::FORBIDDEN);
    let (st, body) = json_get(router.clone(), "/api/v1/alicia/capabilities/pending_confirmations", &ecrivain).await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(body["count"], 0);
    let (st, body) = json_get(router, "/api/v1/alicia/capabilities/episodes/recent", &ecrivain).await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(body["count"], 0);
    assert_eq!(mock.calls().len(), 0);
}

// -------------------------------------------------------------------
// Expiration : pending périmée → Cancelled, plus confirmable (409)
// -------------------------------------------------------------------

#[tokio::test]
async fn expired_pending_cannot_be_confirmed_over_http() {
    let tmp = tempfile::tempdir().unwrap();
    write_demo_manifest(tmp.path());
    let (router, token, runtime) = setup_with_runtime_cfg(CapabilitiesRuntimeConfig {
        services_dir: Some(tmp.path().to_path_buf()),
        pending_expiry: miyualicia::PendingExpiryConfig {
            pending_ttl_secs: 0,
            ..Default::default()
        },
        ..Default::default()
    })
    .await;
    let mock = Arc::new(MockServiceClient::new());
    mock.expect("demo.shout", ServiceCallResult::ok(json!({"ok": true})));
    runtime.register_service_client("demo", mock.clone()).await;
    let id = queued_id(
        runtime
            .dispatch_and_queue("demo.shout", &Value::Null, "voice", "x")
            .await
            .unwrap(),
    );
    // TTL nul : laisser s'écouler quelques millisecondes (conversion ms SQLite).
    tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    for (uri, body) in [
        (format!("/api/v1/alicia/capabilities/pending_confirmations/{id}/confirm"), json!({})),
        ("/api/v1/alicia/capabilities/dispatch/confirm".to_string(), json!({"id": id})),
    ] {
        let (st, _) = json_post(router.clone(), &uri, &token, body).await;
        assert_eq!(st, StatusCode::CONFLICT, "{uri}");
    }
    assert_eq!(mock.calls().len(), 0);
    let pc = runtime.pending_confirmation(id).await.unwrap().unwrap();
    assert_eq!(pc.status, miyualicia_memory::PendingStatus::Cancelled);
    assert_eq!(pc.reason.as_deref(), Some("expired"));
    let (st, body) = json_get(router, "/api/v1/alicia/capabilities/pending_confirmations", &token).await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(body["count"], 0);
}
