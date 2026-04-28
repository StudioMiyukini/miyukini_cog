//! COG Web Portal — Portail HTTP multi-services Miyukini.
//!
//! @id: cog_web_portal_main @do: start_portal_http_server
//! @role: infra @layer: app
//! @human: Lance le serveur axum COG Web Portal avec JayFestival + JayXpose via PortalContract.
//!
//! ## Architecture
//!
//! - AppState: Vec<Arc<dyn PortalContract + Send + Sync>> + central_remote_addr
//! - Routes: GET /, GET /:service, GET /:service/:slug, GET|POST /:service/contact, GET /central-remote, GET /central-remote/app
//! - Sécurité: CSP nonce per-request, HSTS, rate limiting 60 req/min/IP, CSRF sur POST
//! - DB path: PORTAL_JF_DB et PORTAL_JX_DB (env), défaut :memory: en développement

use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};
use cog_portal_contract::PortalContract;
use jayfestival::data::JayFestivalDb;
use jayfestival::portal_contract::JayFestivalPortalService;
use jayxpose::data::JayXposeDb;
use jayxpose::portal_contract::JayXposePortalService;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};

mod csrf;
mod rate_limiter;
mod routes;
mod security_headers;
mod templates;

use rate_limiter::{RateLimiterConfig, RateLimiterLayer};
use security_headers::SecurityHeadersLayer;

/// État partagé du Portal — services enregistrés.
#[derive(Clone)]
pub struct AppState {
    /// Services Portal enregistrés (JayFestival, JayXpose, …).
    pub services: Vec<Arc<dyn PortalContract + Send + Sync>>,
    /// Adresse du serveur CentralRemote (ex: "127.0.0.1:8091"), si configuré.
    pub central_remote_addr: Option<String>,
}

#[tokio::main]
async fn main() {
    // Initialise le subscriber de logs.
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let state = build_app_state();

    let rate_limiter = RateLimiterLayer::new(RateLimiterConfig {
        max_requests: 60,
        ..Default::default()
    });

    let app = Router::new()
        .route("/", get(routes::home::home))
        .route(
            "/central-remote",
            get(routes::central_remote::central_remote_login),
        )
        .route(
            "/central-remote/app",
            get(routes::central_remote::central_remote_app),
        )
        .route("/:service", get(routes::service::service_home))
        .route(
            "/:service/contact",
            get(routes::contact::contact_get).post(routes::contact::contact_post),
        )
        .route("/:service/:slug", get(routes::service::service_page))
        .with_state(state)
        .layer(SecurityHeadersLayer)
        .layer(rate_limiter);

    let addr = std::env::var("PORTAL_LISTEN")
        .unwrap_or_else(|_| "0.0.0.0:8090".to_string());

    let listener = TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("Cannot bind {addr}: {e}"));

    info!("COG Web Portal listening on {addr}");
    axum::serve(listener, app)
        .await
        .expect("server error");
}

/// Construit l'AppState en ouvrant les DBs des services configurés.
pub fn build_app_state() -> AppState {
    let mut services: Vec<Arc<dyn PortalContract + Send + Sync>> = Vec::new();

    // JayFestival — chemin DB depuis env ou défaut développement.
    let jf_path = std::env::var("PORTAL_JF_DB")
        .unwrap_or_else(|_| ":memory:".to_string());
    match JayFestivalDb::open(&jf_path) {
        Ok(db) => {
            info!("JayFestival DB opened: {jf_path}");
            services.push(Arc::new(JayFestivalPortalService::new(Arc::new(db))));
        }
        Err(e) => {
            tracing::warn!("JayFestival DB unavailable ({e}), service not registered");
        }
    }

    // JayXpose — chemin DB depuis env ou défaut développement.
    let jx_path = std::env::var("PORTAL_JX_DB")
        .unwrap_or_else(|_| ":memory:".to_string());
    match JayXposeDb::open(&jx_path) {
        Ok(db) => {
            info!("JayXpose DB opened: {jx_path}");
            services.push(Arc::new(JayXposePortalService::new(Arc::new(db))));
        }
        Err(e) => {
            tracing::warn!("JayXpose DB unavailable ({e}), service not registered");
        }
    }

    // CentralRemote — adresse du serveur remote de Central (optionnel).
    let central_remote_addr = std::env::var("PORTAL_CENTRAL_REMOTE_ADDR").ok();
    if let Some(ref addr) = central_remote_addr {
        info!("CentralRemote configured: {addr}");
    }

    AppState {
        services,
        central_remote_addr,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_state_builds_with_memory_dbs() {
        let state = build_app_state();
        // En développement, les deux services doivent être enregistrés.
        assert_eq!(
            state.services.len(),
            2,
            "jayfestival + jayxpose should both be registered"
        );
        let slugs: Vec<&str> = state.services.iter().map(|s| s.service_slug()).collect();
        assert!(slugs.contains(&"jayfestival"));
        assert!(slugs.contains(&"jayxpose"));
    }

    #[test]
    fn smoke_portal_implemented() {
        // Remplace le RED E00 — le portail est implémenté en E04. GREEN.
        let state = build_app_state();
        assert!(!state.services.is_empty(), "Portal must have at least one service");
    }
}
