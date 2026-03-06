use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::common::dav_error::DavError;
use crate::common::dav_store::DavResource;
use crate::wopi::discovery::{WopiQuery, WopiState};

/// GET /wopi/files/{file_id}/contents — GetFile
pub async fn handle_get_file(
    State(state): State<WopiState>,
    Path(file_id): Path<String>,
    Query(query): Query<WopiQuery>,
) -> Result<Response, DavError> {
    let (validated_file_id, owner_id) = state.token_validator.validate_token(&query.access_token)?;
    if validated_file_id != file_id {
        return Err(DavError::Forbidden("Token file_id mismatch".into()));
    }

    let resource = state.store.resolve_path(&owner_id, &format!("/{file_id}"))?;
    let file = match resource {
        DavResource::File(f) => f,
        _ => return Err(DavError::NotFound("File not found".into())),
    };

    let data = state.store.get_file(&file)?;

    Ok((
        StatusCode::OK,
        [("content-type", file.mime_type.as_str())],
        data,
    )
        .into_response())
}

/// POST /wopi/files/{file_id}/contents — PutFile
pub async fn handle_put_file(
    State(state): State<WopiState>,
    Path(file_id): Path<String>,
    Query(query): Query<WopiQuery>,
    body: axum::body::Bytes,
) -> Result<Response, DavError> {
    let (validated_file_id, owner_id) = state.token_validator.validate_token(&query.access_token)?;
    if validated_file_id != file_id {
        return Err(DavError::Forbidden("Token file_id mismatch".into()));
    }

    let resource = state.store.resolve_path(&owner_id, &format!("/{file_id}"))?;
    let file = match resource {
        DavResource::File(f) => f,
        _ => return Err(DavError::NotFound("File not found".into())),
    };

    // Delete old, create new (same approach as WebDAV PUT)
    state.store.delete_file(&file.id)?;
    state.store.put_file(
        &owner_id,
        file.parent_id.as_deref(),
        &file.name,
        &body,
    )?;

    Ok(StatusCode::OK.into_response())
}
