use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;

use crate::common::dav_error::DavError;
use crate::common::dav_store::DavStore;
use crate::thumbnails::cache::ThumbnailCache;

#[derive(Deserialize)]
pub struct ThumbnailParams {
    #[serde(default = "default_size")]
    pub width: u32,
    #[serde(default = "default_size")]
    pub height: u32,
}

fn default_size() -> u32 {
    256
}

/// Shared state for the thumbnail handler.
#[derive(Clone)]
pub struct ThumbnailState {
    pub store: DavStore,
    pub cache: ThumbnailCache,
}

/// GET /api/files/{owner_id}/{file_id}/thumbnail?width=256&height=256
pub async fn handle_thumbnail(
    State(state): State<ThumbnailState>,
    Path((owner_id, file_id)): Path<(String, String)>,
    Query(params): Query<ThumbnailParams>,
) -> Result<Response, DavError> {
    let width = params.width.clamp(32, 1024);
    let height = params.height.clamp(32, 1024);

    // Check cache first
    if let Some(cached) = state.cache.get(&file_id, width, height) {
        return Ok((
            StatusCode::OK,
            [
                ("content-type", "image/png"),
                ("cache-control", "public, max-age=3600"),
            ],
            cached,
        )
            .into_response());
    }

    // Resolve file from store
    let resource = state.store.resolve_path(&owner_id, &format!("/{file_id}"))?;
    let file = match resource {
        crate::common::dav_store::DavResource::File(f) => f,
        _ => return Err(DavError::BadRequest("Not a file".into())),
    };

    // Check if thumbnailable
    if !miyucloud::domain::thumbnail_ops::is_thumbnailable(&file.mime_type) {
        return Err(DavError::BadRequest(format!(
            "Cannot generate thumbnail for MIME type: {}",
            file.mime_type
        )));
    }

    // Download file data
    let data = state.store.get_file(&file)?;

    // Generate thumbnail
    let thumbnail = miyucloud::domain::thumbnail_ops::generate_thumbnail(&data, width, height)
        .map_err(|e| DavError::Internal(format!("Thumbnail generation failed: {e}")))?;

    // Cache it
    state.cache.insert(&file_id, width, height, thumbnail.clone());

    Ok((
        StatusCode::OK,
        [
            ("content-type", "image/png"),
            ("cache-control", "public, max-age=3600"),
        ],
        thumbnail,
    )
        .into_response())
}
