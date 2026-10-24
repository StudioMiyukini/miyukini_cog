//! @id     team_app
//! @do     build_router
//! @role   orchestration
//! @layer  app
//! @human  team — module « app » (assemblage du routeur HTTP sécurisé).

//! Construction du routeur axum du hub TEAM.
//!
//! - `GET /team/health` : seule route publique, réponse minimale ;
//! - toutes les autres routes passent par `auth::auth_middleware` ;
//! - corps JSON bornés (`DefaultBodyLimit`, C-13) ;
//! - CORS par liste blanche (`config::build_cors`).

use axum::{
    extract::DefaultBodyLimit,
    middleware,
    routing::{delete, get, post},
    Json, Router,
};
use serde_json::json;
use tower_http::trace::TraceLayer;

use crate::auth::{auth_middleware, AuthConfig};
use crate::{api, config, mws, AppState};

/// Taille maximale du corps des routes d'enregistrement / heartbeat (octets).
pub const MAX_NODE_BODY_BYTES: usize = 16 * 1024;
/// Taille maximale du corps d'une soumission de tâche (octets).
pub const MAX_TASK_BODY_BYTES: usize = 256 * 1024;

/// GET /team/health — sonde de vie publique, sans information interne.
async fn health() -> Json<serde_json::Value> {
    Json(json!({ "status": "ok" }))
}

/// Construit le routeur complet (auth, limites, CORS, trace).
pub fn build_router(state: AppState) -> Router {
    let auth = AuthConfig::new(state.config.secret());
    let cors = config::build_cors(&state.config.cors_allowed_origins);

    let protected = Router::new()
        // ── Enregistrement MAIA ────────────────────────────────────
        .route(
            "/team/register",
            post(api::register_node).layer(DefaultBodyLimit::max(MAX_NODE_BODY_BYTES)),
        )
        .route(
            "/team/heartbeat",
            post(api::node_heartbeat).layer(DefaultBodyLimit::max(MAX_NODE_BODY_BYTES)),
        )
        .route("/team/nodes/{id}", delete(api::unregister_node))
        .route("/team/nodes", get(api::list_nodes_internal))
        // ── MWS Gateway ────────────────────────────────────────────
        .route(
            "/api/task",
            post(mws::routes::submit_task).layer(DefaultBodyLimit::max(MAX_TASK_BODY_BYTES)),
        )
        .route("/api/task/{id}", get(mws::routes::get_task))
        .route("/api/nodes", get(mws::routes::list_nodes))
        .route("/api/sequences", get(mws::routes::list_sequences))
        .route("/api/status", get(mws::routes::team_status))
        .route_layer(middleware::from_fn_with_state(auth, auth_middleware));

    Router::new()
        .route("/team/health", get(health))
        .merge(protected)
        // Borne globale par défaut pour toute route sans limite explicite
        .layer(DefaultBodyLimit::max(MAX_NODE_BODY_BYTES))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{
        body::Body,
        http::{header, Method, Request, StatusCode},
    };
    use tower::ServiceExt;

    use super::*;
    use crate::config::TeamConfig;
    use crate::nodes::registry::NodeRegistry;
    use crate::orchestrator::scheduler::TaskScheduler;

    const SECRET: &str = "test-secret-0123456789";

    fn state_with(cfg: TeamConfig) -> AppState {
        AppState {
            registry: NodeRegistry::new(),
            scheduler: TaskScheduler::new(),
            sequences: Arc::new(Vec::new()),
            config: Arc::new(cfg),
            client: reqwest::Client::new(),
        }
    }

    fn secured() -> AppState {
        state_with(TeamConfig {
            hmac_secret: Some(SECRET.into()),
            cors_allowed_origins: vec!["http://localhost:3000".into()],
            ..TeamConfig::default()
        })
    }

    fn get_req(uri: &str, token: Option<&str>) -> Request<Body> {
        let mut b = Request::builder().method(Method::GET).uri(uri);
        if let Some(t) = token {
            b = b.header(header::AUTHORIZATION, format!("Bearer {t}"));
        }
        b.body(Body::empty()).unwrap()
    }

    fn register_req(maia_url: &str, token: Option<&str>) -> Request<Body> {
        let body = json!({
            "id": "node-1",
            "name": "Nœud test",
            "maia_url": maia_url,
            "version": "0.1.0",
            "mode": "pro",
            "hardware_tier": 2,
            "cpu": "CPU",
            "ram_mb": 1024,
            "gpu": null,
            "model_active": null,
            "persona": null,
        });
        let mut b = Request::builder()
            .method(Method::POST)
            .uri("/team/register")
            .header(header::CONTENT_TYPE, "application/json");
        if let Some(t) = token {
            b = b.header(header::AUTHORIZATION, format!("Bearer {t}"));
        }
        b.body(Body::from(body.to_string())).unwrap()
    }

    #[tokio::test]
    async fn sans_jeton_401() {
        let app = build_router(secured());
        for uri in ["/api/status", "/api/nodes", "/team/nodes", "/api/sequences"] {
            let resp = app.clone().oneshot(get_req(uri, None)).await.unwrap();
            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED, "{uri}");
        }
        let resp = app
            .oneshot(register_req("http://127.0.0.1:11434", None))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn mauvais_jeton_401() {
        let app = build_router(secured());
        let resp = app
            .clone()
            .oneshot(get_req("/api/status", Some("mauvais-secret-0123456")))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        // Schéma autre que Bearer refusé
        let req = Request::builder()
            .uri("/api/status")
            .header(header::AUTHORIZATION, format!("Basic {SECRET}"))
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn bon_jeton_200() {
        let app = build_router(secured());
        let resp = app
            .clone()
            .oneshot(get_req("/api/status", Some(SECRET)))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let resp = app
            .oneshot(register_req("http://127.0.0.1:11434", Some(SECRET)))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn sante_publique_sans_jeton() {
        let app = build_router(secured());
        let resp = app.oneshot(get_req("/team/health", None)).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn url_de_noeud_refusee_400() {
        let state = secured();
        let registry = state.registry.clone();
        let app = build_router(state);
        for url in [
            "http://169.254.169.254/latest/meta-data",
            "http://example.com:11434",
            "http://192.168.1.42:11434", // privé mais hors liste blanche
            "file:///etc/passwd",
            "http://localhost.evil.com/",
            "http://user:pw@127.0.0.1:11434",
        ] {
            let resp = app
                .clone()
                .oneshot(register_req(url, Some(SECRET)))
                .await
                .unwrap();
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST, "{url}");
        }
        assert_eq!(registry.total_count(), 0);
    }

    #[tokio::test]
    async fn url_de_noeud_liste_blanche_acceptee() {
        let state = state_with(TeamConfig {
            hmac_secret: Some(SECRET.into()),
            node_allowed_hosts: vec!["192.168.1.42".into()],
            ..TeamConfig::default()
        });
        let app = build_router(state);
        let resp = app
            .oneshot(register_req("http://192.168.1.42:11434", Some(SECRET)))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn cors_origine_etrangere_refusee() {
        let app = build_router(secured());
        let preflight = |origin: &str| {
            Request::builder()
                .method(Method::OPTIONS)
                .uri("/api/status")
                .header(header::ORIGIN, origin)
                .header(header::ACCESS_CONTROL_REQUEST_METHOD, "GET")
                .body(Body::empty())
                .unwrap()
        };
        let resp = app
            .clone()
            .oneshot(preflight("https://evil.example"))
            .await
            .unwrap();
        assert!(resp
            .headers()
            .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
            .is_none());
        // Origine listée : autorisée
        let resp = app
            .oneshot(preflight("http://localhost:3000"))
            .await
            .unwrap();
        assert_eq!(
            resp.headers().get(header::ACCESS_CONTROL_ALLOW_ORIGIN).unwrap(),
            "http://localhost:3000"
        );
    }

    #[tokio::test]
    async fn cors_vide_par_defaut() {
        let app = build_router(state_with(TeamConfig::default()));
        let req = Request::builder()
            .uri("/team/health")
            .header(header::ORIGIN, "http://localhost:3000")
            .body(Body::empty())
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert!(resp
            .headers()
            .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
            .is_none());
    }

    #[tokio::test]
    async fn corps_trop_gros_refuse() {
        let app = build_router(secured());
        let big = format!("{{\"id\":\"{}\"}}", "a".repeat(MAX_NODE_BODY_BYTES + 1));
        let req = Request::builder()
            .method(Method::POST)
            .uri("/team/heartbeat")
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::AUTHORIZATION, format!("Bearer {SECRET}"))
            .body(Body::from(big))
            .unwrap();
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::PAYLOAD_TOO_LARGE);
    }

    fn get_sans_secret(host: Option<&str>, origin: Option<&str>) -> Request<Body> {
        let mut b = Request::builder().method(Method::GET).uri("/api/status");
        if let Some(h) = host {
            b = b.header(header::HOST, h);
        }
        if let Some(o) = origin {
            b = b.header(header::ORIGIN, o);
        }
        b.body(Body::empty()).unwrap()
    }

    #[tokio::test]
    async fn sans_secret_loopback_ouvert() {
        // Comportement documenté : pas de secret ⇒ bind loopback imposé,
        // routes ouvertes aux seules requêtes locales (Host local).
        let app = build_router(state_with(TeamConfig::default()));
        let resp = app
            .oneshot(get_sans_secret(Some("127.0.0.1:7800"), None))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn sans_secret_dns_rebinding_refuse() {
        // Hôte étranger (rebinding) ou origine web étrangère : 403.
        for (host, origin) in [
            (Some("evil.example:7800"), None),
            (Some("127.0.0.1:7800"), Some("http://evil.example")),
            (Some("localhost:7800"), Some("http://localhost.evil.example")),
            (None, None),
        ] {
            let app = build_router(state_with(TeamConfig::default()));
            let resp = app.oneshot(get_sans_secret(host, origin)).await.unwrap();
            assert_eq!(resp.status(), StatusCode::FORBIDDEN, "{host:?} {origin:?}");
        }
    }
}
