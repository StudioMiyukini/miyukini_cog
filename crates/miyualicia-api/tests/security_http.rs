//! Tests de securite HTTP (audit C9) : /auth/token fail-closed, scopes, rate limit, CORS.
//! `/auth/token` (secret partage) n'existe qu'avec la feature `jeton-secret-client`
//! (lot 2, #23) : `cargo test -p miyualicia-api --features jeton-secret-client`.
//!
//! @id     alicia_api_security_http_tests
//! @role   test
//! @layer  7

use std::sync::Arc;

#[cfg(feature = "jeton-secret-client")]
use argon2::password_hash::{PasswordHasher, SaltString};
#[cfg(feature = "jeton-secret-client")]
use argon2::Argon2;
use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::ServiceExt;

use miyualicia::{AliciaConfig, AliciaService};
use miyualicia_api::auth::{generate_token, JwtClaims, JwtScope};
#[cfg(feature = "jeton-secret-client")]
use miyualicia_api::config::ApiClientConfig;
use miyualicia_api::config::ApiConfig;
use miyualicia_api::router::build_router;

const SECRET: &[u8] = b"test-secret-key-32-bytes-minimum-required-here-0123456789";

#[cfg(feature = "jeton-secret-client")]
fn hash(secret: &str) -> String {
    let salt = SaltString::from_b64("c2VsdGVzdHNlbHRlc3Q").unwrap();
    Argon2::default()
        .hash_password(secret.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

fn router(cfg: ApiConfig) -> axum::Router {
    let alicia = Arc::new(AliciaService::new(AliciaConfig {
        mqtt: None,
        ..Default::default()
    }));
    build_router(alicia, cfg, SECRET.to_vec())
}

async fn call(r: axum::Router, req: Request<Body>) -> (StatusCode, Value, axum::http::HeaderMap) {
    let resp = r.oneshot(req).await.unwrap();
    let st = resp.status();
    let h = resp.headers().clone();
    let b = resp.into_body().collect().await.unwrap().to_bytes();
    (st, serde_json::from_slice(&b).unwrap_or(Value::Null), h)
}

#[cfg(feature = "jeton-secret-client")]
fn token_req(client: &str, secret: &str) -> Request<Body> {
    Request::builder()
        .uri("/api/v1/alicia/auth/token")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(
            serde_json::to_vec(&json!({"client_id": client, "secret": secret})).unwrap(),
        ))
        .unwrap()
}

fn post_with(token: &str, uri: &str, body: Value) -> Request<Body> {
    Request::builder()
        .uri(uri)
        .method("POST")
        .header(header::AUTHORIZATION, format!("Bearer {token}"))
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(serde_json::to_vec(&body).unwrap()))
        .unwrap()
}

#[cfg(feature = "jeton-secret-client")]
fn cfg_with_clients() -> ApiConfig {
    ApiConfig {
        clients: vec![
            ApiClientConfig {
                client_id: "ops".into(),
                secret_hash: hash("s3cret"),
                scopes: vec![JwtScope::Admin],
                personas: vec!["voice".into()],
                all_personas: false,
            },
            ApiClientConfig {
                client_id: "admin".into(),
                secret_hash: hash("lecteur"),
                scopes: vec![JwtScope::Read],
                personas: vec![],
                all_personas: false,
            },
            ApiClientConfig {
                client_id: "root".into(),
                secret_hash: hash("racine"),
                scopes: vec![JwtScope::Admin],
                personas: vec![],
                all_personas: true,
            },
        ],
        ..Default::default()
    }
}

#[cfg(feature = "jeton-secret-client")]
#[tokio::test]
async fn token_refused_when_no_client_configured() {
    let (st, _, _) = call(router(ApiConfig::default()), token_req("admin", "nimporte")).await;
    assert_eq!(st, StatusCode::UNAUTHORIZED);
}

#[cfg(feature = "jeton-secret-client")]
#[tokio::test]
async fn token_refused_with_wrong_secret() {
    let (st, _, _) = call(router(cfg_with_clients()), token_req("ops", "faux")).await;
    assert_eq!(st, StatusCode::UNAUTHORIZED);
}

#[cfg(feature = "jeton-secret-client")]
#[tokio::test]
async fn token_scopes_come_from_config_not_client_id() {
    let (st, body, _) = call(router(cfg_with_clients()), token_req("admin", "lecteur")).await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(body["scopes"], json!(["read"]));
    let (st, body, _) = call(router(cfg_with_clients()), token_req("ops", "s3cret")).await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(body["scopes"], json!(["admin"]));
}

#[cfg(feature = "jeton-secret-client")]
#[tokio::test]
async fn token_personas_come_from_config() {
    // R-002 : les personas du token viennent de la config du client.
    let (st, body, _) = call(router(cfg_with_clients()), token_req("ops", "s3cret")).await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(body["personas"], json!(["voice"]));
    let tok = body["access_token"].as_str().unwrap();
    let claims = miyualicia_api::auth::verify_token(tok, SECRET).unwrap();
    assert_eq!(claims.personas, vec!["voice".to_string()]);

    let (_, body, _) = call(router(cfg_with_clients()), token_req("admin", "lecteur")).await;
    assert_eq!(body["personas"], json!([]));
    assert_eq!(body["all_personas"], json!(false));
    let tok = body["access_token"].as_str().unwrap();
    assert!(!miyualicia_api::auth::verify_token(tok, SECRET).unwrap().all_personas);
}

#[cfg(feature = "jeton-secret-client")]
#[tokio::test]
async fn token_all_personas_claim_comes_from_admin_config() {
    // Décision propriétaire : drapeau explicite, porté en claim booléen signé.
    let (st, body, _) = call(router(cfg_with_clients()), token_req("root", "racine")).await;
    assert_eq!(st, StatusCode::OK);
    assert_eq!(body["all_personas"], json!(true));
    assert_eq!(body["personas"], json!([]));
    let tok = body["access_token"].as_str().unwrap();
    let claims = miyualicia_api::auth::verify_token(tok, SECRET).unwrap();
    assert!(claims.all_personas);
    assert!(claims.has_all_personas());
}

#[cfg(feature = "jeton-secret-client")]
#[test]
fn all_personas_without_admin_scope_rejected_at_config_validation() {
    let cfg = ApiConfig {
        clients: vec![ApiClientConfig {
            client_id: "ecrivain".into(),
            secret_hash: hash("x"),
            scopes: vec![JwtScope::Read, JwtScope::Write],
            personas: vec![],
            all_personas: true,
        }],
        ..Default::default()
    };
    assert!(cfg.validate().is_err());
}

#[tokio::test]
async fn read_only_token_forbidden_on_write_and_admin_routes() {
    let claims = JwtClaims::new("lecteur", vec![JwtScope::Read], 3600);
    let tok = generate_token(&claims, SECRET).unwrap();
    let body = json!({"qualified_id":"a.b","params":{},"persona":"p","user_phrase":"x"});
    for uri in [
        "/api/v1/alicia/capabilities/dispatch",
        "/api/v1/alicia/capabilities/dispatch_and_queue",
        "/api/v1/alicia/capabilities/manifest/reload",
        "/api/v1/alicia/capabilities/persona/x/activate",
        "/api/v1/alicia/devices/00000000-0000-0000-0000-000000000000/command",
    ] {
        let b = if uri.ends_with("/command") { json!({"action":"on"}) } else { body.clone() };
        let (st, _, _) = call(router(ApiConfig::default()), post_with(&tok, uri, b)).await;
        assert_eq!(st, StatusCode::FORBIDDEN, "{uri}");
    }
}

#[tokio::test]
async fn rate_limit_returns_429() {
    let cfg = ApiConfig {
        rate_limit_per_min: 3,
        ..Default::default()
    };
    let r = router(cfg);
    let mut last = StatusCode::OK;
    for _ in 0..5 {
        let req = Request::builder()
            .uri("/api/v1/alicia/health")
            .body(Body::empty())
            .unwrap();
        last = call(r.clone(), req).await.0;
    }
    assert_eq!(last, StatusCode::TOO_MANY_REQUESTS);
}

#[tokio::test]
async fn cors_only_explicit_origin() {
    let cfg = ApiConfig {
        cors_enabled: true,
        cors_origins: vec!["https://ok.example".into()],
        ..Default::default()
    };
    let mk = |origin: &str| {
        Request::builder()
            .uri("/api/v1/alicia/health")
            .header(header::ORIGIN, origin)
            .body(Body::empty())
            .unwrap()
    };
    let (_, _, h) = call(router(cfg.clone()), mk("https://ok.example")).await;
    assert_eq!(h.get("access-control-allow-origin").unwrap(), "https://ok.example");
    let (_, _, h) = call(router(cfg), mk("https://evil.example")).await;
    assert!(h.get("access-control-allow-origin").is_none());
}

#[cfg(not(feature = "jeton-secret-client"))]
#[tokio::test]
async fn auth_token_route_absent_by_default() {
    // Lot 2, #23 : plus d'echange client_id/secret partage -> JWT par defaut.
    let req = Request::builder()
        .uri("/api/v1/alicia/auth/token")
        .method("POST")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(r#"{"client_id":"ops","secret":"s3cret"}"#))
        .unwrap();
    let (st, body, _) = call(router(ApiConfig::default()), req).await;
    assert_eq!(st, StatusCode::NOT_FOUND);
    assert!(body.get("access_token").is_none());
}
