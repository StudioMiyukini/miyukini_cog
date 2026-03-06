//! Routeur API REST MiyuCloud.
//!
//! @id: miyucloud_api_router
//! @do: define_api_routes_and_middleware
//! @role: api
//! @layer: app
//!
//! Toutes les routes `/api/*` exigent le header `X-COG-Token`.
//! La route `/health` est accessible sans authentification.

pub mod admin;
pub mod auth;
pub mod auth_2fa;
pub mod caldav_api;
pub mod carddav_api;
pub mod files;
pub mod folders;
pub mod onboarding;
pub mod shares;
pub mod sync_api;
pub mod trash;
pub mod web_auth;

use crate::api::auth::CogTokenLayer;
use crate::security_headers::SecurityHeadersLayer;
use axum::routing::{delete, get, post, put};
use axum::Router;
use std::sync::Arc;

use crate::AppState;

/// Construit le routeur API complet.
pub fn api_router(state: Arc<AppState>) -> Router {
    let protected = Router::new()
        // Files
        .route("/files", get(files::list_files))
        .route("/files/upload", post(files::upload_file))
        .route("/files/search", get(files::search_files))
        .route("/files/{id}", get(files::get_file))
        .route("/files/{id}", put(files::update_file))
        .route("/files/{id}", delete(files::delete_file))
        .route("/files/{id}/download", get(files::download_file))
        .route("/files/{id}/restore", post(files::restore_file))
        // Folders
        .route("/folders", post(folders::create_folder))
        .route("/folders/{id}", put(folders::update_folder))
        .route("/folders/{id}", delete(folders::delete_folder))
        // Trash
        .route("/trash", get(trash::list_trash))
        .route("/trash", delete(trash::empty_trash))
        .route("/trash/{id}/restore", post(trash::restore_from_trash))
        .route("/trash/{id}", delete(trash::purge_from_trash))
        // Shares (B07)
        .route("/shares/link", post(shares::create_share_link))
        .route("/shares/links", get(shares::list_share_links))
        .route("/shares/links/{id}", delete(shares::revoke_share_link))
        .route("/shares/permission", post(shares::create_share_permission))
        .route(
            "/shares/resources/{resource_id}/permissions",
            get(shares::list_permissions),
        )
        .route(
            "/shares/permissions/{id}",
            delete(shares::delete_permission),
        )
        // Sync (C09)
        .route("/sync/peers", get(sync_api::list_peers))
        .route("/sync/peers", post(sync_api::register_peer))
        .route("/sync/peers/{id}/trust", post(sync_api::trust_peer))
        .route("/sync/peers/{id}/untrust", post(sync_api::untrust_peer))
        .route("/sync/status", get(sync_api::sync_status))
        .route("/sync/conflicts", get(sync_api::list_conflicts))
        .route(
            "/sync/conflicts/{id}/resolve",
            post(sync_api::resolve_conflict),
        )
        .route("/sync/trigger", post(sync_api::trigger_sync))
        // CalDAV JSON API (E8)
        .route("/calendars", get(caldav_api::list_calendars))
        .route("/calendars", post(caldav_api::create_calendar))
        .route("/calendars/{id}/events", get(caldav_api::list_events))
        // CardDAV JSON API (E8)
        .route("/contacts/books", get(carddav_api::list_addressbooks))
        .route("/contacts/books", post(carddav_api::create_addressbook))
        .route("/contacts/books/{id}/contacts", get(carddav_api::list_contacts))
        // Admin (B19)
        .route("/admin/quota", get(admin::get_quota))
        .route("/admin/quota", put(admin::set_quota))
        .route("/admin/stats", get(admin::get_stats))
        .route("/admin/access-log", get(admin::get_access_log))
        .route("/admin/health", get(onboarding::admin_health))
        // Web auth approvals (local only)
        .route("/web-auth/challenges", get(web_auth::list_challenges))
        .route(
            "/web-auth/challenges/{id}/approve",
            post(web_auth::approve_challenge),
        )
        .route(
            "/web-auth/challenges/{id}/reject",
            post(web_auth::reject_challenge),
        )
        // Onboarding (V3/V4)
        .route("/onboarding/status", get(onboarding::onboarding_status))
        .route(
            "/onboarding/complete",
            post(onboarding::onboarding_complete),
        )
        .route("/onboarding/reset", post(onboarding::onboarding_reset))
        // Auth sessions + 2FA (V2/V4)
        .route("/auth/session", post(auth_2fa::create_session))
        .route("/auth/sessions", get(auth_2fa::list_sessions))
        .route("/auth/sessions", delete(auth_2fa::revoke_all_sessions))
        .route("/auth/sessions/{id}", delete(auth_2fa::revoke_session))
        .route("/auth/totp/status", get(auth_2fa::totp_status))
        .route("/auth/totp/setup", post(auth_2fa::setup_totp))
        .route("/auth/totp/verify", post(auth_2fa::verify_totp))
        .route(
            "/auth/totp/recovery/verify",
            post(auth_2fa::verify_recovery_code),
        )
        .route(
            "/auth/totp/recovery/regenerate",
            post(auth_2fa::regenerate_recovery_codes),
        )
        .route("/auth/totp", delete(auth_2fa::disable_totp))
        .layer(CogTokenLayer::new(state.config.cog_token.clone()))
        .layer(SecurityHeadersLayer::for_api())
        .with_state(state.clone());

    Router::new()
        .route("/health", get(health))
        .nest("/api", protected)
}

/// Health check endpoint (pas d'authentification requise).
async fn health() -> &'static str {
    "OK"
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bootstrap_crypto;
    use crate::config::MiyucloudConfig;
    use crate::connect_auth::ConnectAuthManager;
    use miyucloud::data::MiyucloudDb;
    use miyucloud::storage::local_fs::LocalFsStorage;
    use miyucloud_dav::caldav::service::CalDavService;
    use miyucloud_dav::carddav::service::CardDavService;
    use std::collections::HashMap;

    #[test]
    fn api_router_builds_without_route_conflicts() {
        let dir = tempfile::tempdir().expect("temp dir");
        let storage_path = dir.path().join("storage");
        std::fs::create_dir_all(&storage_path).expect("storage dir");

        let db = MiyucloudDb::open(":memory:").expect("db");
        let config = MiyucloudConfig {
            api_port: 11440,
            storage_path: storage_path.clone(),
            db_path: dir.path().join("miyucloud.db"),
            cog_token: "test-token".to_string(),
            passphrase: Some("test-passphrase".to_string()),
            owner_id: "owner".to_string(),
            web_port: 11442,
            web_enabled: false,
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
            dav_db_path: dir.path().join("dav.db"),
        };
        let key_manager = bootstrap_crypto(&db, &config).expect("bootstrap");
        let storage = LocalFsStorage::new(storage_path).expect("storage");

        let dav_conn = rusqlite::Connection::open_in_memory().expect("dav conn");
        let dav_db = std::sync::Arc::new(std::sync::Mutex::new(dav_conn));
        let caldav_svc = CalDavService::new(dav_db.clone());
        caldav_svc.init_tables().expect("caldav tables");
        let carddav_svc = CardDavService::new(dav_db);
        carddav_svc.init_tables().expect("carddav tables");

        let state = Arc::new(AppState {
            db,
            storage,
            key_manager,
            config,
            ip_salt: [0; 32],
            connect_auth: ConnectAuthManager::new(None),
            web_sessions: std::sync::Mutex::new(HashMap::new()),
            qr_challenges: std::sync::Mutex::new(HashMap::new()),
            caldav_svc,
            carddav_svc,
        });

        let _router = api_router(state);
    }
}
