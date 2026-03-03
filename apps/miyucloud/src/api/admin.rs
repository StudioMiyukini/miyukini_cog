//! Handlers API pour l'administration (quota, stats, access log).
//!
//! @id: miyucloud_api_admin
//! @do: handle_admin_quota_stats_access_log_endpoints
//! @role: api
//! @layer: app
//!
//! Routes conformes a la spec P1 section 4.1.6.

use crate::api::auth::error_response;
use crate::AppState;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use miyucloud::domain::QuotaOps;
use serde::Deserialize;
use std::sync::Arc;

/// Retourne le quota de l'utilisateur courant.
pub async fn get_quota(State(state): State<Arc<AppState>>) -> Response {
    let db = &state.db;
    let owner_id = &state.config.owner_id;

    match QuotaOps::get_usage(db, owner_id) {
        Ok(quota) => (StatusCode::OK, Json(quota)).into_response(),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            &e.to_string(),
        ),
    }
}

/// Retourne les statistiques de stockage.
pub async fn get_stats(State(app_state): State<Arc<AppState>>) -> Response {
    let db = &app_state.db;

    match db.storage_stats() {
        Ok(storage_stats) => (StatusCode::OK, Json(storage_stats)).into_response(),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            &e.to_string(),
        ),
    }
}

/// Query params pour le journal d'acces.
#[derive(Deserialize)]
pub struct AccessLogQuery {
    /// Nombre max d'entrees a retourner (defaut 50).
    pub limit: Option<u32>,
}

/// Retourne les dernieres entrees du journal d'acces.
pub async fn get_access_log(
    State(state): State<Arc<AppState>>,
    Query(query): Query<AccessLogQuery>,
) -> Response {
    let db = &state.db;
    let limit = query.limit.unwrap_or(50);

    match db.access_log_list(None, limit) {
        Ok(entries) => (StatusCode::OK, Json(entries)).into_response(),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            &e.to_string(),
        ),
    }
}
