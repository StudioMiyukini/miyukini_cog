//! Handlers API pour les dossiers.
//!
//! @id: miyucloud_api_folders
//! @do: handle_folder_crud_rest_endpoints
//! @role: api
//! @layer: app

use crate::api::auth::error_response;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use miyucloud::domain::FileOps;
use serde::Deserialize;
use std::sync::Arc;

/// Body pour la creation d'un dossier.
#[derive(Deserialize)]
pub struct CreateFolderBody {
    /// Nom du dossier.
    pub name: String,
    /// UUID du dossier parent (optionnel, None = racine).
    pub parent_id: Option<String>,
}

/// Cree un nouveau dossier.
pub async fn create_folder(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateFolderBody>,
) -> Response {
    if body.name.is_empty() {
        return error_response(
            StatusCode::BAD_REQUEST,
            "INVALID_INPUT",
            "Folder name cannot be empty",
        );
    }
    let db = &state.db;
    let owner_id = &state.config.owner_id;
    match FileOps::create_folder(db, owner_id, body.parent_id.as_deref(), &body.name) {
        Ok(folder) => (StatusCode::CREATED, Json(folder)).into_response(),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "CREATE_ERROR",
            &e.to_string(),
        ),
    }
}

/// Body pour la mise a jour d'un dossier.
#[derive(Deserialize)]
pub struct UpdateFolderBody {
    /// Nouveau nom (optionnel).
    pub name: Option<String>,
    /// Nouveau parent (optionnel).
    pub parent_id: Option<String>,
}

/// Met a jour un dossier (renommage/deplacement).
pub async fn update_folder(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(body): Json<UpdateFolderBody>,
) -> Response {
    let db = &state.db;
    if let Some(name) = &body.name {
        if let Err(e) = FileOps::rename_folder(db, &id, name) {
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "UPDATE_ERROR",
                &e.to_string(),
            );
        }
    }
    if let Some(pid) = &body.parent_id {
        // Move folder by updating its parent_id directly
        if let Ok(Some(mut folder)) = db.folder_by_id(&id) {
            folder.parent_id = if pid.is_empty() {
                None
            } else {
                Some(pid.clone())
            };
            if let Err(e) = db.folder_update(&folder) {
                return error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "UPDATE_ERROR",
                    &e.to_string(),
                );
            }
        }
    }
    match db.folder_by_id(&id) {
        Ok(Some(folder)) => (StatusCode::OK, Json(folder)).into_response(),
        Ok(None) => error_response(
            StatusCode::NOT_FOUND,
            "FOLDER_NOT_FOUND",
            "Le dossier demande n'existe pas.",
        ),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            &e.to_string(),
        ),
    }
}

/// Supprime un dossier (soft delete -> corbeille).
pub async fn delete_folder(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Response {
    let db = &state.db;
    match FileOps::delete_folder(db, &id) {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({"trashed": true}))).into_response(),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "DELETE_ERROR",
            &e.to_string(),
        ),
    }
}
