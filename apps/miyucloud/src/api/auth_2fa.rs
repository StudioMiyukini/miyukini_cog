//! Handlers API pour sessions et 2FA TOTP.
//!
//! @id: miyucloud_api_auth_2fa
//! @do: expose_session_and_totp_endpoints
//! @role: api
//! @layer: app

use crate::api::auth::error_response;
use crate::web_surface::access_log;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use miyucloud::auth::{sessions, totp};
use miyucloud::utils::sanitize::validate_uuid;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct SetupTotpBody {
    pub account_name: String,
}

#[derive(Deserialize)]
pub struct VerifyTotpBody {
    pub code: String,
    pub session_id: Option<String>,
}

#[derive(Deserialize)]
pub struct VerifyRecoveryBody {
    pub code: String,
    pub session_id: Option<String>,
}

fn ensure_session_for_owner(
    db: &miyucloud::data::MiyucloudDb,
    owner_id: &str,
    session_id: &str,
) -> Result<(), miyucloud::errors::MiyucloudError> {
    let sessions = db.sessions_by_owner(owner_id)?;
    if sessions.iter().any(|s| s.id == session_id && !s.is_revoked) {
        Ok(())
    } else {
        Err(miyucloud::errors::MiyucloudError::NotFound(
            "Session not found for owner".to_string(),
        ))
    }
}

pub async fn create_session(State(state): State<Arc<AppState>>, headers: HeaderMap) -> Response {
    let owner_id = &state.config.owner_id;
    let ip_hash = access_log::extract_client_ip(&headers, state.config.trust_proxy)
        .map(|ip| access_log::hash_ip(&ip, &state.ip_salt));
    let ua =
        access_log::extract_user_agent(&headers).map(|ua| access_log::truncate_user_agent(&ua));

    match sessions::create_session(&state.db, owner_id, ip_hash.as_deref(), ua.as_deref()) {
        Ok(token) => {
            let body = serde_json::json!({
                "session_token": token,
                "owner_id": owner_id
            });
            (StatusCode::CREATED, Json(body)).into_response()
        }
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "SESSION_CREATE_ERROR",
            &e.to_string(),
        ),
    }
}

pub async fn list_sessions(State(state): State<Arc<AppState>>) -> Response {
    let owner_id = &state.config.owner_id;
    match sessions::list_sessions(&state.db, owner_id) {
        Ok(items) => (StatusCode::OK, Json(items)).into_response(),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "SESSION_LIST_ERROR",
            &e.to_string(),
        ),
    }
}

pub async fn revoke_session(
    State(state): State<Arc<AppState>>,
    Path(session_id): Path<String>,
) -> Response {
    if !validate_uuid(&session_id) {
        return error_response(StatusCode::BAD_REQUEST, "INVALID_ID", "Invalid session id");
    }
    let owner_id = &state.config.owner_id;
    if let Err(e) = ensure_session_for_owner(&state.db, owner_id, &session_id) {
        return error_response(StatusCode::NOT_FOUND, "SESSION_NOT_FOUND", &e.to_string());
    }

    match sessions::revoke_session(&state.db, &session_id) {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({"revoked": true}))).into_response(),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "SESSION_REVOKE_ERROR",
            &e.to_string(),
        ),
    }
}

pub async fn revoke_all_sessions(State(state): State<Arc<AppState>>) -> Response {
    let owner_id = &state.config.owner_id;
    match sessions::revoke_all_sessions(&state.db, owner_id) {
        Ok(count) => (
            StatusCode::OK,
            Json(serde_json::json!({"revoked_count": count})),
        )
            .into_response(),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "SESSION_REVOKE_ALL_ERROR",
            &e.to_string(),
        ),
    }
}

pub async fn setup_totp(
    State(state): State<Arc<AppState>>,
    Json(body): Json<SetupTotpBody>,
) -> Response {
    if body.account_name.trim().is_empty() {
        return error_response(
            StatusCode::BAD_REQUEST,
            "INVALID_ACCOUNT",
            "account_name is required",
        );
    }

    match totp::setup_totp(
        &state.db,
        &state.key_manager,
        &state.config.owner_id,
        body.account_name.trim(),
    ) {
        Ok(result) => (StatusCode::OK, Json(result)).into_response(),
        Err(e) => error_response(StatusCode::BAD_REQUEST, "TOTP_SETUP_ERROR", &e.to_string()),
    }
}

pub async fn totp_status(State(state): State<Arc<AppState>>) -> Response {
    match totp::is_totp_enabled(&state.db, &state.config.owner_id) {
        Ok(enabled) => (
            StatusCode::OK,
            Json(serde_json::json!({ "enabled": enabled })),
        )
            .into_response(),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "TOTP_STATUS_ERROR",
            &e.to_string(),
        ),
    }
}

pub async fn verify_totp(
    State(state): State<Arc<AppState>>,
    Json(body): Json<VerifyTotpBody>,
) -> Response {
    if let Some(session_id) = &body.session_id {
        if !validate_uuid(session_id) {
            return error_response(StatusCode::BAD_REQUEST, "INVALID_ID", "Invalid session id");
        }
    }
    match totp::validate_totp(
        &state.db,
        &state.key_manager,
        &state.config.owner_id,
        &body.code,
    ) {
        Ok(true) => {
            if let Some(session_id) = body.session_id {
                if let Err(e) =
                    ensure_session_for_owner(&state.db, &state.config.owner_id, &session_id)
                {
                    return error_response(
                        StatusCode::NOT_FOUND,
                        "SESSION_NOT_FOUND",
                        &e.to_string(),
                    );
                }
                if let Err(e) = sessions::mark_totp_verified(&state.db, &session_id) {
                    return error_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "SESSION_UPDATE_ERROR",
                        &e.to_string(),
                    );
                }
            }
            (StatusCode::OK, Json(serde_json::json!({"valid": true}))).into_response()
        }
        Ok(false) => (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"valid": false})),
        )
            .into_response(),
        Err(e) => error_response(StatusCode::BAD_REQUEST, "TOTP_VERIFY_ERROR", &e.to_string()),
    }
}

pub async fn verify_recovery_code(
    State(state): State<Arc<AppState>>,
    Json(body): Json<VerifyRecoveryBody>,
) -> Response {
    if let Some(session_id) = &body.session_id {
        if !validate_uuid(session_id) {
            return error_response(StatusCode::BAD_REQUEST, "INVALID_ID", "Invalid session id");
        }
    }
    match totp::validate_recovery_code(&state.db, &state.config.owner_id, &body.code) {
        Ok(true) => {
            if let Some(session_id) = body.session_id {
                if let Err(e) =
                    ensure_session_for_owner(&state.db, &state.config.owner_id, &session_id)
                {
                    return error_response(
                        StatusCode::NOT_FOUND,
                        "SESSION_NOT_FOUND",
                        &e.to_string(),
                    );
                }
                if let Err(e) = sessions::mark_totp_verified(&state.db, &session_id) {
                    return error_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "SESSION_UPDATE_ERROR",
                        &e.to_string(),
                    );
                }
            }
            (StatusCode::OK, Json(serde_json::json!({"valid": true}))).into_response()
        }
        Ok(false) => (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({"valid": false})),
        )
            .into_response(),
        Err(e) => error_response(
            StatusCode::BAD_REQUEST,
            "RECOVERY_VERIFY_ERROR",
            &e.to_string(),
        ),
    }
}

pub async fn regenerate_recovery_codes(State(state): State<Arc<AppState>>) -> Response {
    match totp::regenerate_recovery_codes(&state.db, &state.config.owner_id) {
        Ok(codes) => (
            StatusCode::OK,
            Json(serde_json::json!({ "recovery_codes": codes })),
        )
            .into_response(),
        Err(e) => error_response(
            StatusCode::BAD_REQUEST,
            "RECOVERY_REGEN_ERROR",
            &e.to_string(),
        ),
    }
}

pub async fn disable_totp(State(state): State<Arc<AppState>>) -> Response {
    match totp::disable_totp(&state.db, &state.config.owner_id) {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({"disabled": true}))).into_response(),
        Err(e) => error_response(
            StatusCode::BAD_REQUEST,
            "TOTP_DISABLE_ERROR",
            &e.to_string(),
        ),
    }
}
