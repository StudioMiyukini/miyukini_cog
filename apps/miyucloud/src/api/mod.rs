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
pub mod files;
pub mod folders;
pub mod shares;
pub mod sync_api;
pub mod trash;

use crate::api::auth::CogTokenLayer;
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
            "/shares/permissions/{resource_id}",
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
        // Admin (B19)
        .route("/admin/quota", get(admin::get_quota))
        .route("/admin/stats", get(admin::get_stats))
        .route("/admin/access-log", get(admin::get_access_log))
        .layer(CogTokenLayer::new(state.config.cog_token.clone()))
        .with_state(state.clone());

    Router::new()
        .route("/health", get(health))
        .nest("/api", protected)
}

/// Health check endpoint (pas d'authentification requise).
async fn health() -> &'static str {
    "OK"
}
