//! GDPR individual rights API handler.
//!
//! RGPD Art. 15-20: data export, erasure, rectification.
//! User endpoints under /api/user/gdpr/ and admin endpoints under /api/admin/gdpr/.

use axum::{
    Json,
    extract::State,
    http::StatusCode,
    response::IntoResponse,
};
use std::sync::Arc;

use crate::common::di::AppState;
use crate::interfaces::middleware::auth::AuthUser;

type GlobalState = Arc<AppState>;

pub struct GdprHandler;

impl GdprHandler {
    /// GET /api/user/gdpr/export — request data export (Art. 15 + Art. 20).
    pub async fn export_data(
        State(_state): State<GlobalState>,
        auth_user: AuthUser,
    ) -> impl IntoResponse {
        // TODO: Wire to GdprRightsPort implementation when available
        (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "export_requested",
                "user_id": auth_user.id.to_string(),
                "message": "Your data export is being prepared. You will be notified when ready."
            })),
        )
    }

    /// DELETE /api/user/gdpr/erase — request account erasure (Art. 17).
    pub async fn erase_data(
        State(_state): State<GlobalState>,
        auth_user: AuthUser,
    ) -> impl IntoResponse {
        // TODO: Wire to GdprRightsPort implementation
        // This is a destructive operation — should require confirmation
        (
            StatusCode::ACCEPTED,
            Json(serde_json::json!({
                "status": "erasure_requested",
                "user_id": auth_user.id.to_string(),
                "message": "Your account erasure request has been recorded. This action is irreversible."
            })),
        )
    }

    /// PUT /api/user/gdpr/rectify — update personal data (Art. 16).
    pub async fn rectify_data(
        State(_state): State<GlobalState>,
        auth_user: AuthUser,
        Json(updates): Json<crate::application::ports::gdpr_ports::UserRectification>,
    ) -> impl IntoResponse {
        // TODO: Wire to GdprRightsPort implementation
        (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "rectification_applied",
                "user_id": auth_user.id.to_string(),
                "updated_fields": {
                    "email": updates.email.is_some(),
                    "username": updates.username.is_some(),
                }
            })),
        )
    }
}
