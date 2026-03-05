//! Handlers API pour les fichiers.
//!
//! @id: miyucloud_api_files
//! @do: handle_file_crud_rest_endpoints
//! @role: api
//! @layer: app
//!
//! CRUD fichiers : upload (multipart), download (stream), list, get, update, delete, search.

use crate::api::auth::error_response;
use crate::AppState;
use axum::body::Body;
use axum::extract::{Multipart, Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use miyucloud::domain::FileOps;
use miyucloud::utils::sanitize::{sanitize_mime_type, validate_uuid};
use serde::Deserialize;
use std::sync::Arc;

/// Query params pour la liste des fichiers.
#[derive(Deserialize)]
pub struct ListFilesQuery {
    /// UUID du dossier parent (absent = racine).
    pub parent_id: Option<String>,
}

/// Liste le contenu d'un dossier.
pub async fn list_files(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListFilesQuery>,
) -> Response {
    let db = &state.db;
    let owner_id = &state.config.owner_id;
    match FileOps::list_contents(db, query.parent_id.as_deref(), owner_id) {
        Ok(contents) => (StatusCode::OK, Json(contents)).into_response(),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            &e.to_string(),
        ),
    }
}

/// Recupere les metadonnees d'un fichier.
pub async fn get_file(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> Response {
    if !validate_uuid(&id) {
        return error_response(StatusCode::BAD_REQUEST, "INVALID_ID", "Invalid file id");
    }
    let db = &state.db;
    match db.file_by_id(&id) {
        Ok(Some(file)) => (StatusCode::OK, Json(file)).into_response(),
        Ok(None) => error_response(
            StatusCode::NOT_FOUND,
            "FILE_NOT_FOUND",
            "Le fichier demande n'existe pas.",
        ),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            &e.to_string(),
        ),
    }
}

/// Telecharge un fichier (dechiffre).
pub async fn download_file(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> Response {
    if !validate_uuid(&id) {
        return error_response(StatusCode::BAD_REQUEST, "INVALID_ID", "Invalid file id");
    }
    let db = &state.db;
    let storage = &state.storage;
    let key_manager = &state.key_manager;
    match FileOps::download_file(db, storage, key_manager, &id) {
        Ok(data) => {
            // Get file entry for mime type
            let mime = db
                .file_by_id(&id)
                .ok()
                .flatten()
                .map_or_else(|| "application/octet-stream".to_string(), |f| f.mime_type);
            let safe_mime = sanitize_mime_type(&mime);
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", safe_mime)
                .header("Content-Length", data.len().to_string())
                .body(Body::from(data))
                .unwrap_or_else(|_| {
                    error_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "INTERNAL_ERROR",
                        "Failed to build response",
                    )
                })
        }
        Err(e) => match &e {
            miyucloud::errors::MiyucloudError::NotFound(_) => {
                error_response(StatusCode::NOT_FOUND, "FILE_NOT_FOUND", &e.to_string())
            }
            _ => error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                &e.to_string(),
            ),
        },
    }
}

/// Upload un fichier via multipart/form-data.
pub async fn upload_file(State(state): State<Arc<AppState>>, mut multipart: Multipart) -> Response {
    let db = &state.db;
    let storage = &state.storage;
    let key_manager = &state.key_manager;
    let owner_id = state.config.owner_id.clone();

    let mut file_data: Option<Vec<u8>> = None;
    let mut file_name = String::from("unnamed");
    let mut mime_type = String::from("application/octet-stream");
    let mut parent_id: Option<String> = None;

    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "file" => {
                if let Some(fname) = field.file_name() {
                    file_name = fname.to_string();
                }
                if let Some(ct) = field.content_type() {
                    mime_type = sanitize_mime_type(ct);
                }
                match field.bytes().await {
                    Ok(bytes) => file_data = Some(bytes.to_vec()),
                    Err(e) => {
                        return error_response(
                            StatusCode::BAD_REQUEST,
                            "UPLOAD_ERROR",
                            &format!("Failed to read file data: {e}"),
                        );
                    }
                }
            }
            "parent_id" => {
                if let Ok(text) = field.text().await {
                    if !text.is_empty() {
                        parent_id = Some(text);
                    }
                }
            }
            _ => {}
        }
    }

    let Some(data) = file_data else {
        return error_response(
            StatusCode::BAD_REQUEST,
            "MISSING_FILE",
            "No file field in multipart request",
        );
    };

    match FileOps::upload_file(
        db,
        storage,
        key_manager,
        &owner_id,
        parent_id.as_deref(),
        &file_name,
        &sanitize_mime_type(&mime_type),
        &data,
    ) {
        Ok(entry) => (StatusCode::CREATED, Json(entry)).into_response(),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "UPLOAD_ERROR",
            &e.to_string(),
        ),
    }
}

/// Body pour la mise a jour d'un fichier (renommage/deplacement).
#[derive(Deserialize)]
pub struct UpdateFileBody {
    /// Nouveau nom (optionnel).
    pub name: Option<String>,
    /// Nouveau dossier parent (optionnel).
    pub parent_id: Option<String>,
}

/// Met a jour un fichier (renommage/deplacement).
pub async fn update_file(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Json(body): Json<UpdateFileBody>,
) -> Response {
    if !validate_uuid(&id) {
        return error_response(StatusCode::BAD_REQUEST, "INVALID_ID", "Invalid file id");
    }
    if let Some(pid) = &body.parent_id {
        if !pid.is_empty() && !validate_uuid(pid) {
            return error_response(
                StatusCode::BAD_REQUEST,
                "INVALID_PARENT_ID",
                "Invalid parent id",
            );
        }
    }
    let db = &state.db;
    if let Some(name) = &body.name {
        if let Err(e) = FileOps::rename_file(db, &id, name) {
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "UPDATE_ERROR",
                &e.to_string(),
            );
        }
    }
    if let Some(pid) = &body.parent_id {
        let new_parent = if pid.is_empty() {
            None
        } else {
            Some(pid.as_str())
        };
        if let Err(e) = FileOps::move_file(db, &id, new_parent) {
            return error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "UPDATE_ERROR",
                &e.to_string(),
            );
        }
    }
    match db.file_by_id(&id) {
        Ok(Some(file)) => (StatusCode::OK, Json(file)).into_response(),
        Ok(None) => error_response(
            StatusCode::NOT_FOUND,
            "FILE_NOT_FOUND",
            "Le fichier demande n'existe pas.",
        ),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            &e.to_string(),
        ),
    }
}

/// Supprime un fichier (soft delete -> corbeille).
pub async fn delete_file(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> Response {
    if !validate_uuid(&id) {
        return error_response(StatusCode::BAD_REQUEST, "INVALID_ID", "Invalid file id");
    }
    let db = &state.db;
    match FileOps::delete_file(db, &id) {
        Ok(()) => {
            let body = serde_json::json!({"trashed": true});
            (StatusCode::OK, Json(body)).into_response()
        }
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "DELETE_ERROR",
            &e.to_string(),
        ),
    }
}

/// Restaure un fichier depuis la corbeille.
pub async fn restore_file(State(state): State<Arc<AppState>>, Path(id): Path<String>) -> Response {
    if !validate_uuid(&id) {
        return error_response(StatusCode::BAD_REQUEST, "INVALID_ID", "Invalid file id");
    }
    let db = &state.db;
    match miyucloud::domain::TrashOps::restore_file(db, &id) {
        Ok(()) => match db.file_by_id(&id) {
            Ok(Some(file)) => (StatusCode::OK, Json(file)).into_response(),
            _ => (StatusCode::OK, Json(serde_json::json!({"restored": true}))).into_response(),
        },
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "RESTORE_ERROR",
            &e.to_string(),
        ),
    }
}

/// Query params pour la recherche.
#[derive(Deserialize)]
pub struct SearchQuery {
    /// Terme de recherche.
    pub q: String,
}

/// Recherche des fichiers par nom.
pub async fn search_files(
    State(state): State<Arc<AppState>>,
    Query(query): Query<SearchQuery>,
) -> Response {
    let db = &state.db;
    let owner_id = &state.config.owner_id;
    match db.file_search(owner_id, &query.q) {
        Ok(files) => (StatusCode::OK, Json(files)).into_response(),
        Err(e) => error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "SEARCH_ERROR",
            &e.to_string(),
        ),
    }
}
