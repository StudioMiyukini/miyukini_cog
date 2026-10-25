//! COG Web Portal — Portail HTTP multi-services Miyukini.
//!
//! @id     cog_web_portal_main
//! @do start_portal_http_server
//! @role   infra
//! @layer app
//! @human  Lance le serveur axum COG Web Portal pour les services Jay via PortalContract.
//!
//! ## Architecture
//!
//! - AppState: Vec<Arc<dyn PortalContract + Send + Sync>> + central_remote_addr
//! - Routes: GET /, GET /:service, GET /:service/:slug, GET|POST /:service/contact, GET /central-remote, GET /central-remote/app
//! - Sécurité: CSP nonce per-request, HSTS, rate limiting 60 req/min/IP, CSRF sur POST

// C-10 : aucun unwrap/expect/panic hors tests dans le code exposé au réseau.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::todo,
        clippy::unimplemented,
        clippy::unreachable
    )
)]

use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};
use cog_portal_contract::PortalContract;
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
    /// Services Portal enregistrés.
    pub services: Vec<Arc<dyn PortalContract + Send + Sync>>,
    /// Adresse du serveur CentralRemote (ex: "127.0.0.1:8091"), si configuré.
    pub central_remote_addr: Option<String>,
}

/// Construit le routeur HTTP du portail (routes, en-têtes de sécurité, limitation de débit).
///
/// Les chemins utilisent la syntaxe axum 0.8 (`{param}`) : la forme `:param` panique
/// à la construction du routeur.
pub fn routeur(state: AppState) -> Router {
    let rate_limiter = RateLimiterLayer::new(RateLimiterConfig {
        max_requests: 60,
        ..Default::default()
    });

    Router::new()
        .route("/", get(routes::home::home))
        .route(
            "/central-remote",
            get(routes::central_remote::central_remote_login),
        )
        .route(
            "/central-remote/app",
            get(routes::central_remote::central_remote_app),
        )
        .route("/{service}", get(routes::service::service_home))
        .route(
            "/{service}/contact",
            get(routes::contact::contact_get).post(routes::contact::contact_post),
        )
        .route("/{service}/{slug}", get(routes::service::service_page))
        .with_state(state)
        .layer(SecurityHeadersLayer)
        .layer(rate_limiter)
}

#[tokio::main]
async fn main() {
    // Initialise le subscriber de logs.
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let app = routeur(build_app_state());

    let addr = adresse_ecoute(std::env::var("PORTAL_LISTEN").ok());

    let listener = match TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => {
            tracing::error!("Impossible d'écouter sur {addr} : {e}");
            std::process::exit(1);
        }
    };

    info!("COG Web Portal listening on {addr}");
    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("Serveur arrêté sur erreur : {e}");
        std::process::exit(1);
    }
}

/// Adresse d'écoute par défaut : boucle locale uniquement (EX-48, C-21).
pub const ADRESSE_ECOUTE_DEFAUT: &str = "127.0.0.1:8090";

/// Adresse d'écoute : `PORTAL_LISTEN` si renseignée (exposition explicite),
/// sinon la boucle locale. Une exposition non locale est journalisée.
pub fn adresse_ecoute(env: Option<String>) -> String {
    match env.map(|v| v.trim().to_string()).filter(|v| !v.is_empty()) {
        Some(v) => {
            if !(v.starts_with("127.") || v.starts_with("[::1]") || v.starts_with("localhost:")) {
                tracing::warn!("PORTAL_LISTEN={v} : le portail est exposé hors de la boucle locale");
            }
            v
        }
        None => ADRESSE_ECOUTE_DEFAUT.to_string(),
    }
}

/// Construit l'AppState. Les services Portal sont enregistrés ici lorsqu'ils
/// sont disponibles ; pour l'instant aucun service n'est exposé tant que les
/// adaptateurs PortalContract ne sont pas raccordés.
pub fn build_app_state() -> AppState {
    let services: Vec<Arc<dyn PortalContract + Send + Sync>> = Vec::new();

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
    fn ecoute_par_defaut_en_boucle_locale() {
        assert_eq!(adresse_ecoute(None), "127.0.0.1:8090");
        assert_eq!(adresse_ecoute(Some("  ".into())), "127.0.0.1:8090");
        assert_eq!(adresse_ecoute(Some("0.0.0.0:9000".into())), "0.0.0.0:9000");
    }

    #[tokio::test]
    async fn routeur_se_construit_et_repond() {
        // Régression : la forme `/:service` (axum 0.7) faisait paniquer la construction
        // du routeur au démarrage (boucle de redémarrage systemd observée en VM).
        use tower::ServiceExt;
        let app = routeur(build_app_state());
        let reponse = app
            .oneshot(
                axum::http::Request::builder()
                    .uri("/")
                    .body(axum::body::Body::empty())
                    .expect("requête de test"),
            )
            .await
            .expect("réponse");
        assert_eq!(reponse.status(), axum::http::StatusCode::OK);
    }

    #[test]
    fn app_state_builds_empty() {
        let state = build_app_state();
        assert!(
            state.services.is_empty(),
            "Aucun service Portal n'est enregistré tant que les adaptateurs ne sont pas raccordés"
        );
    }
}
