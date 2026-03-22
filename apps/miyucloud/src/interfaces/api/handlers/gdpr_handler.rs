//! GDPR individual rights API handler — RGPD Art. 15-20.
//!
//! Provides actual data export, erasure, and rectification via GdprService.

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

use crate::application::ports::gdpr_ports::{GdprRightsPort, UserRectification};
use crate::common::di::AppState;
use crate::interfaces::middleware::auth::AuthUser;

type GlobalState = Arc<AppState>;

pub struct GdprHandler;

impl GdprHandler {
    /// GET /api/user/gdpr/export — data export (Art. 15 + Art. 20 portability).
    pub async fn export_data(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
    ) -> impl IntoResponse {
        let Some(ref gdpr) = state.gdpr_service else {
            return (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({"error": "GDPR service not available"}))).into_response();
        };

        match gdpr.export_user_data(auth_user.id).await {
            Ok(export) => (StatusCode::OK, Json(serde_json::json!(export))).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
        }
    }

    /// DELETE /api/user/gdpr/erase — account erasure (Art. 17 right to be forgotten).
    pub async fn erase_data(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
    ) -> impl IntoResponse {
        let Some(ref gdpr) = state.gdpr_service else {
            return (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({"error": "GDPR service not available"}))).into_response();
        };

        match gdpr.erase_user_data(auth_user.id, "User-initiated erasure request").await {
            Ok(report) => (StatusCode::OK, Json(serde_json::json!(report))).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
        }
    }

    /// PUT /api/user/gdpr/rectify — update personal data (Art. 16).
    pub async fn rectify_data(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
        Json(updates): Json<UserRectification>,
    ) -> impl IntoResponse {
        let Some(ref gdpr) = state.gdpr_service else {
            return (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({"error": "GDPR service not available"}))).into_response();
        };

        match gdpr.rectify_user_data(auth_user.id, updates).await {
            Ok(()) => (StatusCode::OK, Json(serde_json::json!({"status": "rectification_applied"}))).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e.to_string()}))).into_response(),
        }
    }
}
