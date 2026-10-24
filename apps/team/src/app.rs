//! @id     team_app
//! @do     build_router
//! @role   orchestration
//! @layer  app
//! @human  team — module « app » (assemblage du routeur HTTP sécurisé).

//! Construction du routeur axum du hub TEAM.
//!
//! - `GET /team/health` : seule route publique, réponse minimale ;
//! - toutes les autres routes passent par `auth::auth_middleware` (requête
//!   signée par un nœud ou un client épinglé, sans secret partagé) ;
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
    // Clés validées au démarrage (`validate_startup`) ; en cas d'erreur ici,
    // aucune clé n'est retenue : tout est refusé (fail-closed).
    let (noeuds, clients) = state.config.cles_autorisees().unwrap_or_default();
    let auth = AuthConfig::new(noeuds, clients);
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
    use miyauth::http::service::{encoder_cle_publique, entetes_signees};
    use miyusceau::CleSignature;
    use tower::ServiceExt;

    use super::*;
    use crate::auth::{USAGE_API, USAGE_NOEUD};
    use crate::config::TeamConfig;
    use crate::nodes::registry::NodeRegistry;
    use crate::orchestrator::scheduler::TaskScheduler;

    fn cle_noeud() -> CleSignature {
        CleSignature::depuis_graine(&[1_u8; 32])
    }
    fn cle_client() -> CleSignature {
        CleSignature::depuis_graine(&[2_u8; 32])
    }

    fn state_with(cfg: TeamConfig) -> AppState {
        AppState {
            registry: NodeRegistry::new(),
            scheduler: TaskScheduler::new(),
            sequences: Arc::new(Vec::new()),
            config: Arc::new(cfg),
            client: reqwest::Client::new(),
            cle: Arc::new(CleSignature::generer()),
        }
    }

    fn secured() -> AppState {
        state_with(TeamConfig {
            noeuds_autorises: vec![encoder_cle_publique(&cle_noeud().cle_publique())],
            clients_autorises: vec![encoder_cle_publique(&cle_client().cle_publique())],
            cors_allowed_origins: vec!["http://localhost:3000".into()],
            ..TeamConfig::default()
        })
    }

    /// Requête éventuellement signée par `cle` pour `usage`.
    fn req(method: Method, uri: &str, body: &str, cle: Option<(&CleSignature, &str)>) -> Request<Body> {
        let mut b = Request::builder()
            .method(method.clone())
            .uri(uri)
            .header(header::CONTENT_TYPE, "application/json");
        if let Some((cle, usage)) = cle {
            for (n, v) in entetes_signees(cle, usage, method.as_str(), uri, body.as_bytes()) {
                b = b.header(n, v);
            }
        }
        b.body(Body::from(body.to_string())).unwrap()
    }

    fn get_client(uri: &str) -> Request<Body> {
        req(Method::GET, uri, "", Some((&cle_client(), USAGE_API)))
    }

    fn register_body(id: &str, maia_url: &str) -> String {
        json!({
            "id": id,
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
        })
        .to_string()
    }

    fn register_req(maia_url: &str, cle: Option<&CleSignature>) -> Request<Body> {
        req(Method::POST, "/team/register", &register_body("node-1", maia_url), cle.map(|c| (c, USAGE_NOEUD)))
    }

    #[tokio::test]
    async fn non_signe_401() {
        let app = build_router(secured());
        for uri in ["/api/status", "/api/nodes", "/team/nodes", "/api/sequences"] {
            let resp = app.clone().oneshot(req(Method::GET, uri, "", None)).await.unwrap();
            assert_eq!(resp.status(), StatusCode::UNAUTHORIZED, "{uri}");
        }
        let resp = app
            .clone()
            .oneshot(register_req("http://127.0.0.1:11434", None))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
        // Ancien secret Bearer : sans effet.
        let r = Request::builder()
            .uri("/api/status")
            .header(header::AUTHORIZATION, "Bearer test-secret-0123456789")
            .body(Body::empty())
            .unwrap();
        assert_eq!(app.oneshot(r).await.unwrap().status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn cle_inconnue_ou_mauvaise_famille_401() {
        let app = build_router(secured());
        let intrus = CleSignature::generer();
        let r = req(Method::GET, "/api/status", "", Some((&intrus, USAGE_API)));
        assert_eq!(app.clone().oneshot(r).await.unwrap().status(), StatusCode::UNAUTHORIZED);
        // Clé de nœud sur une route cliente, et inversement.
        let r = req(Method::GET, "/api/status", "", Some((&cle_noeud(), USAGE_API)));
        assert_eq!(app.clone().oneshot(r).await.unwrap().status(), StatusCode::UNAUTHORIZED);
        let r = register_req("http://127.0.0.1:11434", Some(&cle_client()));
        assert_eq!(app.oneshot(r).await.unwrap().status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn rejeu_perime_et_corps_altere_401() {
        let app = build_router(secured());
        let corps = register_body("node-1", "http://127.0.0.1:11434");
        let signe = register_req("http://127.0.0.1:11434", Some(&cle_noeud()));
        let entetes = signe.headers().clone();
        assert_eq!(app.clone().oneshot(signe).await.unwrap().status(), StatusCode::OK);
        // Rejeu.
        let mut rejeu = req(Method::POST, "/team/register", &corps, None);
        rejeu.headers_mut().extend(entetes);
        assert_eq!(app.clone().oneshot(rejeu).await.unwrap().status(), StatusCode::UNAUTHORIZED);
        // Corps altéré.
        let signe = register_req("http://127.0.0.1:11434", Some(&cle_noeud()));
        let mut altere = req(Method::POST, "/team/register", &corps.replace("node-1", "node-2"), None);
        altere.headers_mut().extend(signe.headers().clone());
        assert_eq!(app.clone().oneshot(altere).await.unwrap().status(), StatusCode::UNAUTHORIZED);
        // Horodatage périmé.
        let vieux = miyauth::http::signer_requete(
            &cle_client(),
            USAGE_API,
            "GET",
            "/api/status",
            b"",
            miyauth::http::service::maintenant_unix() - 600,
        );
        let mut r = req(Method::GET, "/api/status", "", None);
        for (n, v) in vieux.en_entetes() {
            r.headers_mut().insert(n, v.parse().unwrap());
        }
        assert_eq!(app.oneshot(r).await.unwrap().status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn signe_200() {
        let app = build_router(secured());
        let resp = app.clone().oneshot(get_client("/api/status")).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let resp = app
            .oneshot(register_req("http://127.0.0.1:11434", Some(&cle_noeud())))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn noeud_lie_a_sa_cle() {
        let autre = CleSignature::generer();
        let state = state_with(TeamConfig {
            noeuds_autorises: vec![
                encoder_cle_publique(&cle_noeud().cle_publique()),
                encoder_cle_publique(&autre.cle_publique()),
            ],
            ..TeamConfig::default()
        });
        let app = build_router(state);
        let r = register_req("http://127.0.0.1:11434", Some(&cle_noeud()));
        assert_eq!(app.clone().oneshot(r).await.unwrap().status(), StatusCode::OK);
        // Même identifiant, autre clé épinglée : 409.
        let r = register_req("http://127.0.0.1:11434", Some(&autre));
        assert_eq!(app.clone().oneshot(r).await.unwrap().status(), StatusCode::CONFLICT);
        // Heartbeat / retrait par l'autre clé : refusés.
        let hb = json!({"id": "node-1"}).to_string();
        let r = req(Method::POST, "/team/heartbeat", &hb, Some((&autre, USAGE_NOEUD)));
        assert_eq!(app.clone().oneshot(r).await.unwrap().status(), StatusCode::NOT_FOUND);
        let r = req(Method::DELETE, "/team/nodes/node-1", "", Some((&autre, USAGE_NOEUD)));
        assert_eq!(app.clone().oneshot(r).await.unwrap().status(), StatusCode::NOT_FOUND);
        // Par sa clé : acceptés.
        let r = req(Method::POST, "/team/heartbeat", &hb, Some((&cle_noeud(), USAGE_NOEUD)));
        assert_eq!(app.clone().oneshot(r).await.unwrap().status(), StatusCode::OK);
        let r = req(Method::DELETE, "/team/nodes/node-1", "", Some((&cle_noeud(), USAGE_NOEUD)));
        assert_eq!(app.oneshot(r).await.unwrap().status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn sante_publique_sans_signature() {
        let app = build_router(secured());
        let resp = app.oneshot(req(Method::GET, "/team/health", "", None)).await.unwrap();
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
                .oneshot(register_req(url, Some(&cle_noeud())))
                .await
                .unwrap();
            assert_eq!(resp.status(), StatusCode::BAD_REQUEST, "{url}");
        }
        assert_eq!(registry.total_count(), 0);
    }

    #[tokio::test]
    async fn url_de_noeud_liste_blanche_acceptee() {
        let state = state_with(TeamConfig {
            noeuds_autorises: vec![encoder_cle_publique(&cle_noeud().cle_publique())],
            node_allowed_hosts: vec!["192.168.1.42".into()],
            ..TeamConfig::default()
        });
        let app = build_router(state);
        let resp = app
            .oneshot(register_req("http://192.168.1.42:11434", Some(&cle_noeud())))
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
        let r = req(Method::POST, "/team/heartbeat", &big, Some((&cle_noeud(), USAGE_NOEUD)));
        let resp = app.oneshot(r).await.unwrap();
        assert_eq!(resp.status(), StatusCode::PAYLOAD_TOO_LARGE);
    }

    #[tokio::test]
    async fn aucune_cle_epinglee_tout_refuse_meme_en_loopback() {
        // Ancien comportement « loopback ouvert » supprimé : 503, même avec un
        // Host local et une requête signée.
        let app = build_router(state_with(TeamConfig::default()));
        let mut r = get_client("/api/status");
        r.headers_mut().insert(header::HOST, "127.0.0.1:7800".parse().unwrap());
        assert_eq!(app.clone().oneshot(r).await.unwrap().status(), StatusCode::SERVICE_UNAVAILABLE);
        let mut r = req(Method::GET, "/api/status", "", None);
        r.headers_mut().insert(header::HOST, "127.0.0.1:7800".parse().unwrap());
        assert_eq!(app.clone().oneshot(r).await.unwrap().status(), StatusCode::SERVICE_UNAVAILABLE);
        let r = register_req("http://127.0.0.1:11434", Some(&cle_noeud()));
        assert_eq!(app.oneshot(r).await.unwrap().status(), StatusCode::SERVICE_UNAVAILABLE);
    }
}
