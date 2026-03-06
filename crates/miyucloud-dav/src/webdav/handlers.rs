use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

use crate::common::dav_error::DavError;
use crate::common::dav_store::{DavResource, DavStore};
use crate::common::path_validator;
use crate::webdav::parser::{self, Depth};
use crate::webdav::response::{self, PropfindResponse};
use crate::webdav::properties;

const DAV_PREFIX: &str = "/dav";

fn extract_user_path(uri_path: &str) -> Result<(&str, &str), DavError> {
    // Path format: /dav/{owner_id}/{relative_path...}
    let after_dav = uri_path
        .strip_prefix(DAV_PREFIX)
        .unwrap_or(uri_path)
        .trim_start_matches('/');

    if after_dav.is_empty() {
        return Err(DavError::BadRequest("Missing owner in DAV path".into()));
    }

    let (owner_id, relative) = match after_dav.find('/') {
        Some(pos) => (&after_dav[..pos], &after_dav[pos..]),
        None => (after_dav, "/"),
    };

    Ok((owner_id, relative))
}

fn resource_to_propfind_response(resource: &DavResource, href: &str) -> PropfindResponse {
    PropfindResponse {
        href: href.to_string(),
        properties: properties::resource_properties(
            resource.display_name(),
            resource.content_type(),
            resource.content_length(),
            resource.created_at(),
            resource.last_modified(),
            &resource.etag(),
            resource.is_collection(),
        ),
        not_found_properties: vec![],
    }
}

/// Handle PROPFIND requests
pub async fn handle_propfind(
    State(store): State<DavStore>,
    req: Request,
) -> Result<Response, DavError> {
    let path = req.uri().path().to_string();
    let validated_path = path_validator::validate_dav_path(&path)?;

    let depth = Depth::from_header(
        req.headers()
            .get("depth")
            .and_then(|v| v.to_str().ok()),
    );

    let body_bytes = axum::body::to_bytes(req.into_body(), 1_048_576)
        .await
        .map_err(|e| DavError::BadRequest(format!("Failed to read body: {e}")))?;

    let _propfind_req = parser::parse_propfind(&body_bytes)?;

    let (owner_id, relative_path) = extract_user_path(&validated_path)?;
    let resource = store.resolve_path(owner_id, relative_path)?;

    let mut responses = vec![resource_to_propfind_response(&resource, &validated_path)];

    // Depth 1 or infinity: include children
    if resource.is_collection() && depth != Depth::Zero {
        let parent_id = match &resource {
            DavResource::Collection(f) => Some(f.id.as_str()),
            DavResource::Root { .. } => None,
            _ => None,
        };

        let children = store.list_children(owner_id, parent_id)?;
        for child in &children {
            let child_href = if validated_path.ends_with('/') {
                format!("{}{}", validated_path, child.display_name())
            } else {
                format!("{}/{}", validated_path, child.display_name())
            };
            responses.push(resource_to_propfind_response(child, &child_href));
        }
    }

    let xml = response::build_multistatus(&responses);

    Ok((
        StatusCode::MULTI_STATUS,
        [("content-type", "application/xml; charset=utf-8")],
        xml,
    )
        .into_response())
}

/// Handle GET requests (file download)
pub async fn handle_get(
    State(store): State<DavStore>,
    req: Request,
) -> Result<Response, DavError> {
    let path = req.uri().path().to_string();
    let validated_path = path_validator::validate_dav_path(&path)?;
    let (owner_id, relative_path) = extract_user_path(&validated_path)?;

    let resource = store.resolve_path(owner_id, relative_path)?;

    match resource {
        DavResource::File(file) => {
            let data = store.get_file(&file)?;
            Ok((
                StatusCode::OK,
                [
                    ("content-type", file.mime_type.as_str()),
                    ("etag", &format!("\"{}\"", file.checksum_sha256)),
                    ("content-length", &data.len().to_string()),
                ],
                data,
            )
                .into_response())
        }
        DavResource::Collection(_) | DavResource::Root { .. } => {
            Err(DavError::MethodNotAllowed)
        }
    }
}

/// Handle PUT requests (file upload)
pub async fn handle_put(
    State(store): State<DavStore>,
    req: Request,
) -> Result<Response, DavError> {
    let path = req.uri().path().to_string();
    let validated_path = path_validator::validate_dav_path(&path)?;
    let (owner_id, relative_path) = extract_user_path(&validated_path)?;

    let body_bytes = axum::body::to_bytes(req.into_body(), 512 * 1024 * 1024) // 512 MB max
        .await
        .map_err(|e| DavError::BadRequest(format!("Failed to read body: {e}")))?;

    // Extract parent path and filename
    let (parent_path, filename) = match relative_path.rfind('/') {
        Some(pos) if pos > 0 => (&relative_path[..pos], &relative_path[pos + 1..]),
        _ => ("/", relative_path.trim_start_matches('/')),
    };

    if filename.is_empty() {
        return Err(DavError::BadRequest("Cannot PUT to a collection".into()));
    }

    // Resolve parent to get parent_id
    let parent_id = if parent_path == "/" || parent_path.is_empty() {
        None
    } else {
        match store.resolve_path(owner_id, parent_path)? {
            DavResource::Collection(f) => Some(f.id),
            DavResource::Root { .. } => None,
            _ => return Err(DavError::Conflict("Parent is not a collection".into())),
        }
    };

    // Check if file already exists (update vs create)
    let existing = store.resolve_path(owner_id, relative_path).ok();
    let is_update = existing.is_some();

    if let Some(DavResource::File(existing_file)) = existing {
        // Delete old version, then create new
        store.delete_file(&existing_file.id)?;
    }

    let _file = store.put_file(
        owner_id,
        parent_id.as_deref(),
        filename,
        &body_bytes,
    )?;

    let status = if is_update {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::CREATED
    };

    Ok(status.into_response())
}

/// Handle DELETE requests
pub async fn handle_delete(
    State(store): State<DavStore>,
    req: Request,
) -> Result<Response, DavError> {
    let path = req.uri().path().to_string();
    let validated_path = path_validator::validate_dav_path(&path)?;
    let (owner_id, relative_path) = extract_user_path(&validated_path)?;

    let resource = store.resolve_path(owner_id, relative_path)?;

    match resource {
        DavResource::File(file) => {
            store.delete_file(&file.id)?;
        }
        DavResource::Collection(folder) => {
            store.delete_collection(&folder.id)?;
        }
        DavResource::Root { .. } => {
            return Err(DavError::Forbidden("Cannot delete root".into()));
        }
    }

    Ok(StatusCode::NO_CONTENT.into_response())
}

/// Handle MKCOL requests (create collection/directory)
pub async fn handle_mkcol(
    State(store): State<DavStore>,
    req: Request,
) -> Result<Response, DavError> {
    let path = req.uri().path().to_string();
    let validated_path = path_validator::validate_dav_path(&path)?;
    let (owner_id, relative_path) = extract_user_path(&validated_path)?;

    // Check if already exists
    if store.resolve_path(owner_id, relative_path).is_ok() {
        return Err(DavError::MethodNotAllowed);
    }

    // Extract parent path and folder name
    let (parent_path, folder_name) = match relative_path.rfind('/') {
        Some(pos) if pos > 0 => (&relative_path[..pos], &relative_path[pos + 1..]),
        _ => ("/", relative_path.trim_start_matches('/')),
    };

    if folder_name.is_empty() {
        return Err(DavError::BadRequest("Empty collection name".into()));
    }

    let parent_id = if parent_path == "/" || parent_path.is_empty() {
        None
    } else {
        match store.resolve_path(owner_id, parent_path)? {
            DavResource::Collection(f) => Some(f.id),
            DavResource::Root { .. } => None,
            _ => return Err(DavError::Conflict("Parent is not a collection".into())),
        }
    };

    store.create_collection(owner_id, parent_id.as_deref(), folder_name)?;

    Ok(StatusCode::CREATED.into_response())
}

/// Handle COPY requests
pub async fn handle_copy(
    State(store): State<DavStore>,
    req: Request,
) -> Result<Response, DavError> {
    let path = req.uri().path().to_string();
    let validated_path = path_validator::validate_dav_path(&path)?;
    let (owner_id, relative_path) = extract_user_path(&validated_path)?;

    let destination = req
        .headers()
        .get("destination")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| DavError::BadRequest("Missing Destination header".into()))?;

    let dest_path = path_validator::validate_dav_path(destination)?;
    let (_, dest_relative) = extract_user_path(&dest_path)?;

    let resource = store.resolve_path(owner_id, relative_path)?;

    match resource {
        DavResource::File(file) => {
            let data = store.get_file(&file)?;

            let (parent_path, filename) = match dest_relative.rfind('/') {
                Some(pos) if pos > 0 => (&dest_relative[..pos], &dest_relative[pos + 1..]),
                _ => ("/", dest_relative.trim_start_matches('/')),
            };

            let parent_id = if parent_path == "/" || parent_path.is_empty() {
                None
            } else {
                match store.resolve_path(owner_id, parent_path)? {
                    DavResource::Collection(f) => Some(f.id),
                    DavResource::Root { .. } => None,
                    _ => return Err(DavError::Conflict("Destination parent is not a collection".into())),
                }
            };

            store.put_file(owner_id, parent_id.as_deref(), filename, &data)?;
            Ok(StatusCode::CREATED.into_response())
        }
        _ => Err(DavError::BadRequest("COPY only supported for files".into())),
    }
}

/// Handle MOVE requests
pub async fn handle_move(
    State(store): State<DavStore>,
    req: Request,
) -> Result<Response, DavError> {
    let path = req.uri().path().to_string();
    let validated_path = path_validator::validate_dav_path(&path)?;
    let (owner_id, relative_path) = extract_user_path(&validated_path)?;

    let destination = req
        .headers()
        .get("destination")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| DavError::BadRequest("Missing Destination header".into()))?;

    let dest_path = path_validator::validate_dav_path(destination)?;
    let (_, dest_relative) = extract_user_path(&dest_path)?;

    let resource = store.resolve_path(owner_id, relative_path)?;

    match resource {
        DavResource::File(file) => {
            let (parent_path, filename) = match dest_relative.rfind('/') {
                Some(pos) if pos > 0 => (&dest_relative[..pos], &dest_relative[pos + 1..]),
                _ => ("/", dest_relative.trim_start_matches('/')),
            };

            let parent_id = if parent_path == "/" || parent_path.is_empty() {
                None
            } else {
                match store.resolve_path(owner_id, parent_path)? {
                    DavResource::Collection(f) => Some(f.id),
                    DavResource::Root { .. } => None,
                    _ => return Err(DavError::Conflict("Destination parent is not a collection".into())),
                }
            };

            store.move_file(&file.id, parent_id.as_deref(), filename)?;
            Ok(StatusCode::CREATED.into_response())
        }
        _ => Err(DavError::BadRequest("MOVE only supported for files".into())),
    }
}

/// Handle OPTIONS requests (DAV capabilities)
pub async fn handle_options() -> Response {
    (
        StatusCode::OK,
        [
            ("allow", "OPTIONS, GET, HEAD, PUT, DELETE, PROPFIND, PROPPATCH, MKCOL, COPY, MOVE"),
            ("dav", "1, 2"),
            ("ms-author-via", "DAV"),
        ],
        Body::empty(),
    )
        .into_response()
}
