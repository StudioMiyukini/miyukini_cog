//! Handlers API pour le partage (liens externes + permissions internes).
//!
//! @id: miyucloud_api_shares
//! @do: handle_share_link_and_permission_rest_endpoints
//! @role: api
//! @layer: app
//!
//! Routes conformes a la spec P1 section 4.1.3.

use crate::api::auth::error_response;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use miyucloud::data::types::{GranteeType, PermissionLevel};
use miyucloud::domain::{ExternalShareOps, SharingOps};
use miyucloud::utils::sanitize::validate_uuid;
use serde::Deserialize;
use std::sync::Arc;

// -----------------------------------------------------------------------
// Share Links (external)
// -----------------------------------------------------------------------

/// Body pour la creation d'un lien de partage.
#[derive(Deserialize)]
pub struct CreateShareLinkBody {
    /// UUID du fichier a partager.
    pub file_id: Option<String>,
    /// UUID du dossier a partager.
    pub folder_id: Option<String>,
    /// Mot de passe optionnel.
    pub password: Option<String>,
    /// Duree d'expiration en heures.
    pub expires_in_hours: u32,
    /// Nombre max de telechargements.
    pub max_downloads: Option<u32>,
}

/// Cree un lien de partage externe.
pub async fn create_share_link(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateShareLinkBody>,
) -> Response {
    if let Some(file_id) = &body.file_id {
        if !validate_uuid(file_id) {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_FILE_ID",
                "Invalid file id",
            );
        }
    }
    if let Some(folder_id) = &body.folder_id {
        if !validate_uuid(folder_id) {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_FOLDER_ID",
                "Invalid folder id",
            );
        }
    }
    let db = &state.db;
    let creator_id = &state.config.owner_id;

    match ExternalShareOps::create_link(
        db,
        body.file_id.as_deref(),
        body.folder_id.as_deref(),
        creator_id,
        body.password.as_deref(),
        body.expires_in_hours,
        body.max_downloads,
    ) {
        Ok(link) => (StatusCode::CREATED, Json(link)).into_response(),
        Err(e) => match &e {
            miyucloud::errors::MiyucloudError::NotFound(_) => {
                error_response(StatusCode::NOT_FOUND, "NOT_FOUND", &e.to_string())
            }
            miyucloud::errors::MiyucloudError::InvalidInput(_) => {
                error_response(StatusCode::BAD_REQUEST, "INVALID_INPUT", &e.to_string())
            }
            _ => error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                &e.to_string(),
            ),
        },
    }
}

/// Liste les liens de partage du createur.
pub async fn list_share_links(State(state): State<Arc<AppState>>) -> Response {
    let db = &state.db;
    let creator_id = &state.config.owner_id;

    match ExternalShareOps::list_links(db, creator_id) {
        Ok(links) => (StatusCode::OK, Json(links)).into_response(),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            &e.to_string(),
        ),
    }
}

/// Revoque un lien de partage.
pub async fn revoke_share_link(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Response {
    if !validate_uuid(&id) {
        return error_response(
            StatusCode::BAD_REQUEST,
            "INVALID_ID",
            "Invalid share link id",
        );
    }
    let db = &state.db;

    match ExternalShareOps::revoke_link(db, &id) {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({"revoked": true}))).into_response(),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            &e.to_string(),
        ),
    }
}

// -----------------------------------------------------------------------
// Share Permissions (internal Tribu)
// -----------------------------------------------------------------------

/// Body pour la creation d'une permission de partage.
#[derive(Deserialize)]
pub struct CreateSharePermissionBody {
    /// UUID du fichier.
    pub file_id: Option<String>,
    /// UUID du dossier.
    pub folder_id: Option<String>,
    /// UUID du beneficiaire.
    pub grantee_id: String,
    /// Type de beneficiaire.
    pub grantee_type: String,
    /// Niveau de permission.
    pub permission: String,
}

/// Cree une permission de partage interne.
pub async fn create_share_permission(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateSharePermissionBody>,
) -> Response {
    if let Some(file_id) = &body.file_id {
        if !validate_uuid(file_id) {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_FILE_ID",
                "Invalid file id",
            );
        }
    }
    if let Some(folder_id) = &body.folder_id {
        if !validate_uuid(folder_id) {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_FOLDER_ID",
                "Invalid folder id",
            );
        }
    }
    let db = &state.db;
    let grantee_type = GranteeType::from_str_value(&body.grantee_type);
    let permission = PermissionLevel::from_str_value(&body.permission);

    match SharingOps::share(
        db,
        body.file_id.as_deref(),
        body.folder_id.as_deref(),
        &body.grantee_id,
        grantee_type,
        permission,
    ) {
        Ok(perm) => (StatusCode::CREATED, Json(perm)).into_response(),
        Err(e) => match &e {
            miyucloud::errors::MiyucloudError::NotFound(_) => {
                error_response(StatusCode::NOT_FOUND, "NOT_FOUND", &e.to_string())
            }
            miyucloud::errors::MiyucloudError::InvalidInput(_) => {
                error_response(StatusCode::BAD_REQUEST, "INVALID_INPUT", &e.to_string())
            }
            _ => error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                &e.to_string(),
            ),
        },
    }
}

/// Liste les permissions pour une ressource.
pub async fn list_permissions(
    State(state): State<Arc<AppState>>,
    Path(resource_id): Path<String>,
) -> Response {
    if !validate_uuid(&resource_id) {
        return error_response(
            StatusCode::BAD_REQUEST,
            "INVALID_RESOURCE_ID",
            "Invalid resource id",
        );
    }
    let db = &state.db;

    match SharingOps::list_shares_for_resource(db, &resource_id) {
        Ok(perms) => (StatusCode::OK, Json(perms)).into_response(),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            &e.to_string(),
        ),
    }
}

/// Supprime une permission de partage.
pub async fn delete_permission(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Response {
    if !validate_uuid(&id) {
        return error_response(
            StatusCode::BAD_REQUEST,
            "INVALID_ID",
            "Invalid permission id",
        );
    }
    let db = &state.db;

    match SharingOps::unshare(db, &id) {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({"deleted": true}))).into_response(),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            &e.to_string(),
        ),
    }
}
