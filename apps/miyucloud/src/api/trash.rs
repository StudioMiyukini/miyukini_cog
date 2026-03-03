//! Handlers API pour la corbeille.
//!
//! @id: miyucloud_api_trash
//! @do: handle_trash_rest_endpoints
//! @role: api
//! @layer: app
//!
//! GET /api/trash : contenu de la corbeille.
//! POST /api/trash/{id}/restore : restaurer un element.
//! DELETE /api/trash/{id} : supprimer definitivement.
//! DELETE /api/trash : vider la corbeille.

use crate::api::auth::error_response;
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use miyucloud::domain::TrashOps;
use std::sync::Arc;

/// Liste le contenu de la corbeille.
pub async fn list_trash(State(state): State<Arc<AppState>>) -> Response {
    let db = &state.db;
    let owner_id = &state.config.owner_id;
    match TrashOps::list_trashed(db, owner_id) {
        Ok(contents) => {
            let body = serde_json::json!({
                "files": contents.files,
                "folders": contents.subfolders,
            });
            (StatusCode::OK, Json(body)).into_response()
        }
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            &e.to_string(),
        ),
    }
}

/// Restaure un element depuis la corbeille.
///
/// Tente d'abord de restaurer comme fichier, puis comme dossier.
pub async fn restore_from_trash(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Response {
    let db = &state.db;
    // Try file first
    if db.file_by_id(&id).ok().flatten().is_some() {
        match TrashOps::restore_file(db, &id) {
            Ok(()) => {
                return (
                    StatusCode::OK,
                    Json(serde_json::json!({"restored": true})),
                )
                    .into_response()
            }
            Err(e) => {
                return error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "RESTORE_ERROR",
                    &e.to_string(),
                )
            }
        }
    }
    // Try folder
    if db.folder_by_id(&id).ok().flatten().is_some() {
        match TrashOps::restore_folder(db, &id) {
            Ok(()) => {
                return (
                    StatusCode::OK,
                    Json(serde_json::json!({"restored": true})),
                )
                    .into_response()
            }
            Err(e) => {
                return error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "RESTORE_ERROR",
                    &e.to_string(),
                )
            }
        }
    }
    error_response(
        StatusCode::NOT_FOUND,
        "NOT_FOUND",
        "Element not found in trash",
    )
}

/// Supprime definitivement un element (purge avec overwrite zeros).
pub async fn purge_from_trash(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Response {
    let db = &state.db;
    let storage = &state.storage;
    // Try file first
    if db.file_by_id(&id).ok().flatten().is_some() {
        match TrashOps::purge_file(db, storage, &id) {
            Ok(()) => {
                return (
                    StatusCode::OK,
                    Json(serde_json::json!({"purged": true})),
                )
                    .into_response()
            }
            Err(e) => {
                return error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "PURGE_ERROR",
                    &e.to_string(),
                )
            }
        }
    }
    // Try folder
    if db.folder_by_id(&id).ok().flatten().is_some() {
        match TrashOps::purge_folder(db, storage, &id) {
            Ok(()) => {
                return (
                    StatusCode::OK,
                    Json(serde_json::json!({"purged": true})),
                )
                    .into_response()
            }
            Err(e) => {
                return error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "PURGE_ERROR",
                    &e.to_string(),
                )
            }
        }
    }
    error_response(
        StatusCode::NOT_FOUND,
        "NOT_FOUND",
        "Element not found in trash",
    )
}

/// Vide la corbeille completement.
pub async fn empty_trash(State(state): State<Arc<AppState>>) -> Response {
    let db = &state.db;
    let storage = &state.storage;
    let owner_id = &state.config.owner_id;
    match TrashOps::empty_trash(db, storage, owner_id) {
        Ok(()) => (
            StatusCode::OK,
            Json(serde_json::json!({"emptied": true})),
        )
            .into_response(),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "EMPTY_TRASH_ERROR",
            &e.to_string(),
        ),
    }
}
