//! Endpoints internes pour approuver les connexions web MiyuCloud.

use crate::api::auth::error_response;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use std::sync::Arc;

pub async fn list_challenges(State(state): State<Arc<AppState>>) -> Response {
    let mut challenges = state
        .qr_challenges
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    challenges.retain(|_, challenge| !challenge.is_expired() && !challenge.rejected);

    let items: Vec<_> = challenges
        .values()
        .filter(|challenge| challenge.approved_identity.is_none())
        .map(|challenge| challenge.summary())
        .collect();
    (StatusCode::OK, Json(items)).into_response()
}

pub async fn approve_challenge(
    State(state): State<Arc<AppState>>,
    Path(challenge_id): Path<String>,
) -> Response {
    let approved_identity = match state.connect_auth.approve_qr_for_owner(&state.config.owner_id) {
        Ok(identity) => identity,
        Err(err) => {
            return error_response(StatusCode::BAD_REQUEST, "QR_APPROVE_ERROR", &err);
        }
    };

    let mut challenges = state
        .qr_challenges
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let Some(challenge) = challenges.get_mut(&challenge_id) else {
        return error_response(
            StatusCode::NOT_FOUND,
            "QR_CHALLENGE_NOT_FOUND",
            "Challenge QR introuvable.",
        );
    };
    if challenge.is_expired() {
        return error_response(
            StatusCode::GONE,
            "QR_CHALLENGE_EXPIRED",
            "Challenge QR expiré.",
        );
    }
    challenge.approved_identity = Some(approved_identity);

    (StatusCode::OK, Json(serde_json::json!({ "approved": true }))).into_response()
}

pub async fn reject_challenge(
    State(state): State<Arc<AppState>>,
    Path(challenge_id): Path<String>,
) -> Response {
    let mut challenges = state
        .qr_challenges
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    let Some(challenge) = challenges.get_mut(&challenge_id) else {
        return error_response(
            StatusCode::NOT_FOUND,
            "QR_CHALLENGE_NOT_FOUND",
            "Challenge QR introuvable.",
        );
    };
    challenge.rejected = true;

    (StatusCode::OK, Json(serde_json::json!({ "rejected": true }))).into_response()
}
