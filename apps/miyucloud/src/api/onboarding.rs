//! Endpoints onboarding et health detaille.
//!
//! @id: miyucloud_api_onboarding
//! @do: expose_onboarding_and_health_endpoints
//! @role: api
//! @layer: app

use crate::api::auth::error_response;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use std::sync::Arc;

/// Retourne le statut onboarding consolide.
pub async fn onboarding_status(State(state): State<Arc<AppState>>) -> Response {
    let owner_id = &state.config.owner_id;
    let storage_path = state.config.storage_path.to_string_lossy().to_string();

    let status = match miyucloud::domain::onboarding::get_status(&state.db, owner_id) {
        Ok(s) => s,
        Err(e) => {
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "ONBOARDING_STATUS_ERROR",
                &e.to_string(),
            )
        }
    };

    let passphrase_set = match miyucloud::domain::onboarding::check_passphrase(&state.db, owner_id)
    {
        Ok(v) => v,
        Err(e) => {
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "ONBOARDING_CHECK_PASSPHRASE_ERROR",
                &e.to_string(),
            )
        }
    };

    let totp_set = match miyucloud::domain::onboarding::check_2fa(&state.db, owner_id) {
        Ok(v) => v,
        Err(e) => {
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "ONBOARDING_CHECK_2FA_ERROR",
                &e.to_string(),
            )
        }
    };

    let storage_verified = match miyucloud::domain::onboarding::verify_storage(&storage_path) {
        Ok(v) => v,
        Err(e) => {
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "ONBOARDING_VERIFY_STORAGE_ERROR",
                &e.to_string(),
            )
        }
    };

    (
        StatusCode::OK,
        Json(json!({
            "current_step": status.current_step,
            "passphrase_set": passphrase_set,
            "totp_set": totp_set,
            "storage_verified": storage_verified,
            "completed": status.completed,
            "completed_at": status.completed_at,
        })),
    )
        .into_response()
}

/// Marque l'onboarding comme termine.
pub async fn onboarding_complete(State(state): State<Arc<AppState>>) -> Response {
    let owner_id = &state.config.owner_id;
    match miyucloud::domain::onboarding::complete(&state.db, owner_id) {
        Ok(()) => (StatusCode::OK, Json(json!({ "completed": true }))).into_response(),
        Err(e) => error_response(
            StatusCode::BAD_REQUEST,
            "ONBOARDING_COMPLETE_ERROR",
            &e.to_string(),
        ),
    }
}

/// Reinitialise l'onboarding.
pub async fn onboarding_reset(State(state): State<Arc<AppState>>) -> Response {
    let owner_id = &state.config.owner_id;
    match miyucloud::domain::onboarding::reset(&state.db, owner_id) {
        Ok(()) => (StatusCode::OK, Json(json!({ "reset": true }))).into_response(),
        Err(e) => error_response(
            StatusCode::BAD_REQUEST,
            "ONBOARDING_RESET_ERROR",
            &e.to_string(),
        ),
    }
}

/// Retourne le health check detaille et les metriques.
pub async fn admin_health(State(state): State<Arc<AppState>>) -> Response {
    let storage_path = state.config.storage_path.to_string_lossy().to_string();

    let health = match miyucloud::monitoring::health_check(&state.db, &storage_path) {
        Ok(h) => h,
        Err(e) => {
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "HEALTH_CHECK_ERROR",
                &e.to_string(),
            )
        }
    };

    let metrics = match miyucloud::monitoring::collect_metrics(&state.db) {
        Ok(m) => m,
        Err(e) => {
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "HEALTH_METRICS_ERROR",
                &e.to_string(),
            )
        }
    };

    (
        StatusCode::OK,
        Json(json!({
            "status": health.status,
            "db_accessible": health.db_accessible,
            "storage_accessible": health.storage_accessible,
            "storage_path": health.storage_path,
            "disk_free_bytes": health.disk_free_bytes,
            "disk_total_bytes": health.disk_total_bytes,
            "file_count": health.file_count,
            "total_size_bytes": health.total_size_bytes,
            "uptime_seconds": health.uptime_seconds,
            "metrics": {
                "total_files": metrics.total_files,
                "total_folders": metrics.total_folders,
                "total_size_bytes": metrics.total_size_bytes,
                "active_share_links": metrics.active_share_links,
                "active_sessions": metrics.active_sessions,
                "sync_peers": metrics.sync_peers,
            }
        })),
    )
        .into_response()
}
