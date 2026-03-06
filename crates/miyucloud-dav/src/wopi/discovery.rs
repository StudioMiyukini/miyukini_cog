use axum::extract::{Path, Query, State};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::common::dav_error::DavError;
use crate::common::dav_store::{DavResource, DavStore};
use crate::wopi::auth::WopiTokenValidator;

#[derive(Debug, Serialize)]
pub struct CheckFileInfoResponse {
    #[serde(rename = "BaseFileName")]
    pub base_file_name: String,
    #[serde(rename = "Size")]
    pub size: u64,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "UserCanWrite")]
    pub user_can_write: bool,
    #[serde(rename = "UserFriendlyName")]
    pub user_friendly_name: String,
    #[serde(rename = "OwnerId")]
    pub owner_id: String,
    #[serde(rename = "SHA256")]
    pub sha256: String,
}

#[derive(Deserialize)]
pub struct WopiQuery {
    pub access_token: String,
}

/// WOPI shared state
#[derive(Clone)]
pub struct WopiState {
    pub store: DavStore,
    pub token_validator: WopiTokenValidator,
}

/// GET /wopi/files/{file_id} — CheckFileInfo
pub async fn handle_check_file_info(
    State(state): State<WopiState>,
    Path(file_id): Path<String>,
    Query(query): Query<WopiQuery>,
) -> Result<Response, DavError> {
    let (validated_file_id, owner_id) = state.token_validator.validate_token(&query.access_token)?;
    if validated_file_id != file_id {
        return Err(DavError::Forbidden("Token file_id mismatch".into()));
    }

    // Resolve the file. The file_id in WOPI is the DB file ID, not a path.
    // We search by looking up the file directly using the store's resolve approach.
    // For WOPI, we use a simplified path: /{file_id} which the store treats as a name lookup at root.
    let resource = state.store.resolve_path(&owner_id, &format!("/{file_id}"))?;
    let file = match resource {
        DavResource::File(f) => f,
        _ => return Err(DavError::NotFound("File not found".into())),
    };

    Ok(Json(CheckFileInfoResponse {
        base_file_name: file.name.clone(),
        size: file.size_bytes,
        version: file.checksum_sha256.clone(),
        user_can_write: true,
        user_friendly_name: owner_id,
        owner_id: file.owner_id.clone(),
        sha256: file.checksum_sha256.clone(),
    })
    .into_response())
}
