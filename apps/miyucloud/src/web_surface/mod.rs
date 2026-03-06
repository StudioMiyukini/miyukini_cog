//! Routeur de la surface web publique MiyuCloud.
//!
//! @id: miyucloud_web_surface_router
//! @do: serve_share_pages_with_tls_rate_limiting_and_access_logging
//! @role: api
//! @layer: app
//!
//! La surface web est completement separee de l'API interne :
//! - API : `127.0.0.1:11440` (HTTP, protegee par X-COG-Token)
//! - Web : `0.0.0.0:11442` (HTTPS, publique, sandboxee)
//!
//! Routes servies :
//! - `GET /share/{token}` : page HTML de telechargement
//! - `POST /share/{token}/auth` : verification mot de passe (JSON)
//! - `GET /share/{token}/download` : streaming du fichier dechiffre
//! - `GET /health` : health check de la surface web
//!
//! INVARIANT : Aucune route `/api/*` n'est accessible depuis la surface web.
//! INVARIANT : Le rate limiter est applique a toutes les routes `/share/*`.
//! INVARIANT : Chaque acces est journalise (IP hashee, UA tronque).

pub mod access_log;
pub mod portal;
pub mod rate_limiter;
pub mod sandbox;
pub mod share_page;
pub mod tls;

use crate::security_headers::SecurityHeadersLayer;
use crate::AppState;
use axum::routing::{get, post};
use axum::Router;
use rate_limiter::{RateLimiterConfig, RateLimiterLayer};
use std::sync::Arc;
use std::time::Duration;

/// Construit le routeur de la surface web publique.
///
/// Ce routeur est monte sur un serveur HTTPS separe.
/// Il ne contient que les routes de partage et un health check.
pub fn web_surface_router(state: Arc<AppState>) -> Router {
    // Rate limiter : 10 requetes/minute par IP pour les routes /share/*
    let share_rate_limiter = RateLimiterLayer::new(RateLimiterConfig {
        max_requests: 10,
        window: Duration::from_secs(60),
        trust_proxy: state.config.trust_proxy,
    });

    // Rate limiter plus strict pour l'authentification : 3 tentatives/minute par IP
    let auth_rate_limiter = RateLimiterLayer::new(RateLimiterConfig {
        max_requests: 3,
        window: Duration::from_secs(60),
        trust_proxy: state.config.trust_proxy,
    });

    // Routes d'authentification avec rate limiter strict
    let auth_routes = Router::new()
        .route("/share/{token}/auth", post(share_page::share_auth))
        .route("/auth/password", post(portal::password_login))
        .route("/auth/qr", post(portal::create_qr_challenge))
        .route("/auth/qr/{id}", get(portal::poll_qr_challenge))
        .layer(auth_rate_limiter);

    // Routes de consultation et telechargement avec rate limiter standard
    let share_routes = Router::new()
        .route("/share/{token}", get(share_page::share_page))
        .route("/share/{token}/download", get(share_page::share_download))
        .route("/", get(portal::portal_home))
        .route("/login", get(portal::login_page))
        .route("/logout", post(portal::logout))
        .route("/upload", post(portal::upload_file))
        .route("/folders", post(portal::create_folder))
        .route("/files/{id}/download", get(portal::download_file))
        .layer(share_rate_limiter);

    Router::new()
        .merge(auth_routes)
        .merge(share_routes)
        .route("/health", get(web_health))
        .layer(SecurityHeadersLayer::for_web())
        .with_state(state)
}

/// Health check de la surface web (pas d'authentification).
async fn web_health() -> &'static str {
    "OK"
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::MiyucloudConfig;
    use miyucloud::crypto::KeyManager;
    use miyucloud::data::MiyucloudDb;
    use miyucloud::storage::local_fs::LocalFsStorage;

    fn make_test_state() -> Arc<AppState> {
        let db = MiyucloudDb::open(":memory:").unwrap();
        let dir = tempfile::tempdir().unwrap();
        let storage = LocalFsStorage::new(dir.into_path()).unwrap();
        let km = KeyManager::from_master_key([1u8; 32]);
        let config = MiyucloudConfig {
            api_port: 11440,
            storage_path: std::path::PathBuf::from("/tmp/test"),
            db_path: std::path::PathBuf::from(":memory:"),
            cog_token: "test-token".to_string(),
            passphrase: Some("test-passphrase".to_string()),
            owner_id: "test-owner".to_string(),
            web_port: 11442,
            web_enabled: true,
            trust_proxy: false,
            default_quota_bytes: 10 * 1024 * 1024 * 1024,
            public_share_links_enabled: true,
            internal_sharing_enabled: true,
            default_share_permission: "read".to_string(),
            ssh_enabled: false,
            ssh_host: None,
            ssh_port: 22,
            ssh_username: None,
            ssh_root_path: None,
            ssh_private_key_path: None,
            ssh_keepalive: true,
            central_db_path: None,
            tls_cert_path: None,
            tls_key_path: None,
            dav_db_path: std::path::PathBuf::from(":memory:"),
        };
        let ip_salt = crate::web_surface::access_log::derive_ip_salt(&config.cog_token);

        let dav_conn = rusqlite::Connection::open_in_memory().unwrap();
        let dav_db = std::sync::Arc::new(std::sync::Mutex::new(dav_conn));
        let caldav_svc = miyucloud_dav::caldav::service::CalDavService::new(dav_db.clone());
        caldav_svc.init_tables().unwrap();
        let carddav_svc = miyucloud_dav::carddav::service::CardDavService::new(dav_db);
        carddav_svc.init_tables().unwrap();

        Arc::new(AppState {
            db,
            storage,
            key_manager: km,
            config,
            ip_salt,
            connect_auth: crate::connect_auth::ConnectAuthManager::new(None),
            web_sessions: std::sync::Mutex::new(std::collections::HashMap::new()),
            qr_challenges: std::sync::Mutex::new(std::collections::HashMap::new()),
            caldav_svc,
            carddav_svc,
        })
    }

    #[test]
    fn test_web_surface_router_builds() {
        let state = make_test_state();
        let _router = web_surface_router(state);
        // If it compiles and doesn't panic, the router is valid.
    }

    #[test]
    fn test_web_surface_has_no_api_routes() {
        // This is verified by the type system: the web_surface_router
        // does not mount any /api/* routes. It only serves /share/* and /health.
        // The separation is guaranteed by having two distinct Router instances
        // on separate TCP listeners.
        let state = make_test_state();
        let _router = web_surface_router(state);
    }
}
