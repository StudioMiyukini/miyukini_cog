//! @id     miyucloud_interfaces_nextcloud_webdav_handler
//! @do     definir_trait_xml_result_ext
//! @role   orchestration
//! @layer  app
//! @human  miyucloud — module « interfaces nextcloud webdav handler ».

use axum::{
    body::{self, Body},
    http::{HeaderName, Request, StatusCode, header},
    response::Response,
};
use bytes::Buf;
use chrono::Utc;
use quick_xml::{
    Writer,
    events::{BytesEnd, BytesStart, BytesText, Event},
};
use std::collections::HashSet;
use std::sync::Arc;

use crate::application::adapters::webdav_adapter::{PropFindRequest, WebDavAdapter};
use crate::application::ports::favorites_ports::FavoritesUseCase;
use crate::application::ports::file_ports::{
    FileManagementUseCase, FileRetrievalUseCase, FileUploadUseCase,
};
use crate::application::ports::inbound::FolderUseCase;
use crate::application::ports::trash_ports::TrashUseCase;
use crate::common::di::AppState;
use crate::common::mime_detect::{filename_from_path, refine_content_type};
use crate::domain::services::home_folder::is_reserved_home_name;
use crate::interfaces::errors::AppError;
use crate::interfaces::middleware::auth::{AuthUser, CurrentUser};
use crate::interfaces::nextcloud::owned_paths::{self, OwnedResource};
use crate::interfaces::upload_spool::{self, MIME_SNIFF_BYTES};

/// Extension trait to map XML write errors to `String` concisely.
trait XmlResultExt<T> {
    fn xml_err(self) -> Result<T, String>;
}

impl<T, E: std::fmt::Display> XmlResultExt<T> for Result<T, E> {
    fn xml_err(self) -> Result<T, String> {
        self.map_err(|e| e.to_string())
    }
}

/// Convert a `u64` timestamp to `i64` safely, falling back to 0 on overflow.
fn timestamp_to_i64(ts: u64) -> i64 {
    i64::try_from(ts).unwrap_or(0)
}

const HEADER_DAV: HeaderName = HeaderName::from_static("dav");

/// Resolve the internal Miyukini Cloud path from a Nextcloud DAV subpath.
///
/// Nextcloud: /remote.php/dav/files/{user}/{subpath}
/// Internal:  My Folder - {username}/{subpath}
///
/// An empty subpath maps to the user's home folder root.
pub fn nc_to_internal_path(username: &str, subpath: &str) -> Result<String, AppError> {
    let home = format!("My Folder - {}", username);
    let subpath = subpath.trim_matches('/');
    if subpath.is_empty() {
        return Ok(home);
    }
    // Reject path traversal attempts.
    if subpath.split('/').any(|seg| seg == ".." || seg == ".") {
        return Err(AppError::bad_request("Invalid path: traversal not allowed"));
    }
    Ok(format!("{}/{}", home, subpath))
}

/// Build the Nextcloud DAV href for a resource.
///
/// Each path segment is URL-encoded individually so filenames with spaces,
/// `#`, `%`, or non-ASCII characters produce valid PROPFIND hrefs.
pub fn nc_href(username: &str, subpath: &str) -> String {
    let subpath = subpath.trim_matches('/');
    let encoded_user = urlencoding::encode(username);
    if subpath.is_empty() {
        format!("/remote.php/dav/files/{}/", encoded_user)
    } else {
        let encoded_segments: Vec<_> = subpath
            .split('/')
            .map(|seg| urlencoding::encode(seg))
            .collect();
        format!(
            "/remote.php/dav/files/{}/{}",
            encoded_user,
            encoded_segments.join("/")
        )
    }
}

/// Dispatch Nextcloud WebDAV request to the appropriate handler.
///
/// `subpath` is everything after `/remote.php/dav/files/{user}/`.
pub async fn handle_nc_webdav(
    state: Arc<AppState>,
    req: Request<Body>,
    user: AuthUser,
    subpath: String,
) -> Result<Response<Body>, AppError> {
    let method = req.method().clone();
    match method.as_str() {
        "OPTIONS" => handle_options(),
        "PROPFIND" => handle_propfind(state, req, &user, &subpath).await,
        "GET" => handle_get(state, &user, &subpath).await,
        "PUT" => handle_put(state, req, &user, &subpath).await,
        "MKCOL" => handle_mkcol(state, &user, &subpath).await,
        "DELETE" => handle_delete(state, &user, &subpath).await,
        "MOVE" => handle_move(state, req, &user, &subpath).await,
        "HEAD" => handle_head(state, &user, &subpath).await,
        "PROPPATCH" => handle_proppatch(state, req, &user, &subpath).await,
        "REPORT" | "SEARCH" => {
            crate::interfaces::nextcloud::report_handler::handle_nc_report(
                state, req, &user, &subpath,
            )
            .await
        }
        _ => Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .unwrap()),
    }
}

// ──────────────────── OPTIONS ────────────────────

fn handle_options() -> Result<Response<Body>, AppError> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(HEADER_DAV, "1, 2, 3")
        .header(
            header::ALLOW,
            "OPTIONS, GET, HEAD, PUT, DELETE, MKCOL, MOVE, PROPFIND, PROPPATCH, REPORT, SEARCH",
        )
        .body(Body::empty())
        .unwrap())
}

// ──────────────────── PROPFIND ────────────────────

async fn handle_propfind(
    state: Arc<AppState>,
    req: Request<Body>,
    user: &CurrentUser,
    subpath: &str,
) -> Result<Response<Body>, AppError> {
    let depth = req
        .headers()
        .get("depth")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("1")
        .to_string();

    // Parse the PROPFIND XML body (or assume allprop if empty).
    let body_bytes = body::to_bytes(req.into_body(), 64 * 1024)
        .await
        .map_err(|e| AppError::bad_request(format!("Failed to read body: {}", e)))?;

    let propfind = if body_bytes.is_empty() {
        PropFindRequest {
            prop_find_type: crate::application::adapters::webdav_adapter::PropFindType::AllProp,
        }
    } else {
        WebDavAdapter::parse_propfind(body_bytes.reader())
            .map_err(|e| AppError::bad_request(format!("Invalid PROPFIND XML: {}", e)))?
    };

    let folder_service = &state.applications.folder_service;
    let file_service = &state.applications.file_retrieval_service;

    // M-02 : résolution sous la racine de l'appelant uniquement.
    let resource = owned_paths::resolve_owned(&state, user, subpath).await?;

    if let Some(OwnedResource::Folder(folder)) = resource.clone() {
        // It's a folder.
        let (files, subfolders) = if depth != "0" {
            let files = file_service
                .list_files_owned(Some(&folder.id), user.id)
                .await
                .unwrap_or_default();
            let subfolders = folder_service
                .list_folders_for_owner(Some(&folder.id), user.id)
                .await
                .unwrap_or_default();
            (files, subfolders)
        } else {
            (vec![], vec![])
        };

        // Batch-check favorites for all items in this listing.
        let favorite_ids = if let Some(fav_svc) = state.favorites_service.as_ref() {
            let mut items: Vec<(&str, &str)> = Vec::new();
            items.push((&folder.id, "folder"));
            for f in &files {
                items.push((&f.id, "file"));
            }
            for sf in &subfolders {
                items.push((&sf.id, "folder"));
            }
            fav_svc
                .batch_check_favorites(user.id, &items)
                .await
                .unwrap_or_default()
        } else {
            HashSet::new()
        };

        // Generate Nextcloud-aware XML.
        let nc = state.nextcloud.as_ref();
        let file_id_svc = nc.map(|n| &n.file_ids);

        let mut buf = Vec::new();
        write_nc_multistatus(
            &mut buf,
            Some(&folder),
            &files,
            &subfolders,
            &propfind,
            &depth,
            &user.username,
            subpath,
            file_id_svc,
            &favorite_ids,
        )
        .await
        .map_err(|e| AppError::internal_error(format!("XML generation failed: {}", e)))?;

        return Ok(Response::builder()
            .status(StatusCode::MULTI_STATUS)
            .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
            .body(Body::from(buf))
            .unwrap());
    }

    // Not a folder — try as a file.
    if let Some(OwnedResource::File(file)) = resource {
        // Batch-check favorites for this single file.
        let favorite_ids = if let Some(fav_svc) = state.favorites_service.as_ref() {
            let items: Vec<(&str, &str)> = vec![(&file.id, "file")];
            fav_svc
                .batch_check_favorites(user.id, &items)
                .await
                .unwrap_or_default()
        } else {
            HashSet::new()
        };

        let nc = state.nextcloud.as_ref();
        let file_id_svc = nc.map(|n| &n.file_ids);

        let mut buf = Vec::new();
        write_nc_multistatus(
            &mut buf,
            None,
            &[file],
            &[],
            &propfind,
            "0",
            &user.username,
            subpath,
            file_id_svc,
            &favorite_ids,
        )
        .await
        .map_err(|e| AppError::internal_error(format!("XML generation failed: {}", e)))?;

        return Ok(Response::builder()
            .status(StatusCode::MULTI_STATUS)
            .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
            .body(Body::from(buf))
            .unwrap());
    }

    Err(AppError::not_found("Resource not found"))
}

// ──────────────────── GET ────────────────────

async fn handle_get(
    state: Arc<AppState>,
    user: &CurrentUser,
    subpath: &str,
) -> Result<Response<Body>, AppError> {
    // GET on root folder — NC clients use this as an existence check
    if subpath.is_empty() || subpath == "/" {
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("DAV", "1, 3")
            .body(Body::empty())
            .unwrap());
    }

    let file_service = &state.applications.file_retrieval_service;

    // M-02 : résolution sous la racine de l'appelant uniquement.
    let file = match owned_paths::resolve_owned(&state, user, subpath).await? {
        // NC clients use GET on folders as an existence check.
        Some(OwnedResource::Folder(_)) => {
            return Ok(Response::builder()
                .status(StatusCode::OK)
                .header("DAV", "1, 3")
                .body(Body::empty())
                .unwrap());
        }
        Some(OwnedResource::File(file)) => file,
        None => return Err(AppError::not_found("File not found")),
    };

    let stream = file_service
        .get_file_stream_owned(&file.id, user.id)
        .await
        .map_err(|e| AppError::internal_error(format!("Failed to read file: {}", e)))?;

    let modified_at =
        chrono::DateTime::<Utc>::from_timestamp(timestamp_to_i64(file.modified_at), 0)
            .unwrap_or_else(Utc::now);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, file.mime_type.as_ref())
        .header(header::CONTENT_LENGTH, file.size)
        .header(header::ETAG, format!("\"{}\"", file.id))
        .header(header::LAST_MODIFIED, modified_at.to_rfc2822())
        .body(Body::from_stream(std::pin::Pin::from(stream)))
        .unwrap())
}

// ──────────────────── HEAD ────────────────────

async fn handle_head(
    state: Arc<AppState>,
    user: &CurrentUser,
    subpath: &str,
) -> Result<Response<Body>, AppError> {
    // HEAD on root folder — NC clients use this as an existence check
    if subpath.is_empty() || subpath == "/" {
        return Ok(Response::builder()
            .status(StatusCode::OK)
            .header("DAV", "1, 3")
            .body(Body::empty())
            .unwrap());
    }

    // M-02 : résolution sous la racine de l'appelant uniquement.
    let file = match owned_paths::resolve_owned(&state, user, subpath).await? {
        // NC clients use HEAD on folders as an existence check.
        Some(OwnedResource::Folder(_)) => {
            return Ok(Response::builder()
                .status(StatusCode::OK)
                .header("DAV", "1, 3")
                .body(Body::empty())
                .unwrap());
        }
        Some(OwnedResource::File(file)) => file,
        None => return Err(AppError::not_found("File not found")),
    };

    let modified_at =
        chrono::DateTime::<Utc>::from_timestamp(timestamp_to_i64(file.modified_at), 0)
            .unwrap_or_else(Utc::now);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, file.mime_type.as_ref())
        .header(header::CONTENT_LENGTH, file.size)
        .header(header::ETAG, format!("\"{}\"", file.id))
        .header(header::LAST_MODIFIED, modified_at.to_rfc2822())
        .body(Body::empty())
        .unwrap())
}

// ──────────────────── PROPPATCH ────────────────────

async fn handle_proppatch(
    state: Arc<AppState>,
    req: Request<Body>,
    user: &CurrentUser,
    subpath: &str,
) -> Result<Response<Body>, AppError> {
    let body_bytes = body::to_bytes(req.into_body(), 64 * 1024)
        .await
        .map_err(|e| AppError::bad_request(format!("Failed to read body: {}", e)))?;

    let body_str = String::from_utf8_lossy(&body_bytes);

    // Parse oc:favorite value from PROPPATCH XML.
    let favorite_value = parse_proppatch_favorite(&body_str);

    if let Some(value) = favorite_value {
        // Determine item_id and item_type (M-02 : ressources de l'appelant).
        let (item_id, item_type) = match owned_paths::resolve_owned(&state, user, subpath).await? {
            Some(OwnedResource::File(file)) => (file.id, "file"),
            Some(OwnedResource::Folder(folder)) => (folder.id, "folder"),
            None => return Err(AppError::not_found("Resource not found")),
        };

        if let Some(fav_svc) = state.favorites_service.as_ref() {
            if value == 1 {
                fav_svc
                    .add_to_favorites(user.id, &item_id, item_type)
                    .await
                    .map_err(|e| {
                        AppError::internal_error(format!("Failed to add favorite: {}", e))
                    })?;
            } else {
                fav_svc
                    .remove_from_favorites(user.id, &item_id, item_type)
                    .await
                    .map_err(|e| {
                        AppError::internal_error(format!("Failed to remove favorite: {}", e))
                    })?;
            }
        }
    }

    // Return 207 Multi-Status with success response using quick_xml for safe escaping.
    let href = nc_href(&user.username, subpath);
    let mut buf = Vec::new();
    {
        let mut xml = Writer::new(&mut buf);
        xml.write_event(Event::Text(BytesText::new(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>",
        )))
        .map_err(|e| AppError::internal_error(format!("XML write failed: {}", e)))?;

        let mut ms = BytesStart::new("d:multistatus");
        ms.push_attribute(("xmlns:d", "DAV:"));
        ms.push_attribute(("xmlns:oc", "http://owncloud.org/ns"));
        xml.write_event(Event::Start(ms))
            .map_err(|e| AppError::internal_error(format!("XML: {}", e)))?;

        xml.write_event(Event::Start(BytesStart::new("d:response")))
            .map_err(|e| AppError::internal_error(format!("XML: {}", e)))?;
        write_text_element(&mut xml, "d:href", &href)
            .map_err(|e| AppError::internal_error(format!("XML: {}", e)))?;
        xml.write_event(Event::Start(BytesStart::new("d:propstat")))
            .map_err(|e| AppError::internal_error(format!("XML: {}", e)))?;
        xml.write_event(Event::Start(BytesStart::new("d:prop")))
            .map_err(|e| AppError::internal_error(format!("XML: {}", e)))?;
        xml.write_event(Event::Empty(BytesStart::new("oc:favorite")))
            .map_err(|e| AppError::internal_error(format!("XML: {}", e)))?;
        xml.write_event(Event::End(BytesEnd::new("d:prop")))
            .map_err(|e| AppError::internal_error(format!("XML: {}", e)))?;
        write_text_element(&mut xml, "d:status", "HTTP/1.1 200 OK")
            .map_err(|e| AppError::internal_error(format!("XML: {}", e)))?;
        xml.write_event(Event::End(BytesEnd::new("d:propstat")))
            .map_err(|e| AppError::internal_error(format!("XML: {}", e)))?;
        xml.write_event(Event::End(BytesEnd::new("d:response")))
            .map_err(|e| AppError::internal_error(format!("XML: {}", e)))?;
        xml.write_event(Event::End(BytesEnd::new("d:multistatus")))
            .map_err(|e| AppError::internal_error(format!("XML: {}", e)))?;
    }

    Ok(Response::builder()
        .status(StatusCode::MULTI_STATUS)
        .header(header::CONTENT_TYPE, "application/xml; charset=utf-8")
        .body(Body::from(buf))
        .unwrap())
}

/// Parse the oc:favorite value from a PROPPATCH XML body using quick_xml.
fn parse_proppatch_favorite(body: &str) -> Option<u8> {
    use quick_xml::Reader;

    let mut reader = Reader::from_str(body);
    let mut inside_favorite = false;

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                let local = e.local_name();
                if local.as_ref() == b"favorite" {
                    inside_favorite = true;
                }
            }
            Ok(Event::Text(ref e)) if inside_favorite => {
                let text = e.decode().ok()?;
                return text.trim().parse::<u8>().ok();
            }
            Ok(Event::End(ref e)) => {
                if e.local_name().as_ref() == b"favorite" {
                    inside_favorite = false;
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
    }
    None
}

// ──────────────────── PUT ────────────────────

/// Cible d'un PUT : fichier existant de l'appelant ou nouveau fichier dans
/// un dossier de l'appelant (M-02).
enum PutTarget {
    Existing { file_id: String, size: u64 },
    New { folder_id: String, name: String },
}

async fn handle_put(
    state: Arc<AppState>,
    req: Request<Body>,
    user: &CurrentUser,
    subpath: &str,
) -> Result<Response<Body>, AppError> {
    let upload_service = &state.applications.file_upload_service;

    if subpath.trim_matches('/').is_empty() {
        return Err(AppError::bad_request("Cannot PUT to root folder"));
    }

    let claimed_type = req
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream")
        .to_string();

    let oc_mtime = req
        .headers()
        .get("x-oc-mtime")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<i64>().ok());

    let declared = upload_spool::declared_length(req.headers());

    // M-02 : la cible est résolue sous la racine de l'appelant uniquement.
    let target = match owned_paths::resolve_owned(&state, user, subpath).await? {
        Some(OwnedResource::Folder(_)) => {
            return Err(AppError::conflict("Cannot PUT to a directory"));
        }
        Some(OwnedResource::File(file)) => PutTarget::Existing {
            file_id: file.id,
            size: file.size,
        },
        None => {
            let (parent_sub, name) = owned_paths::split_parent(subpath);
            if name.is_empty() {
                return Err(AppError::bad_request("Missing file name"));
            }
            let parent = owned_paths::resolve_owned_folder(&state, user, parent_sub)
                .await?
                .ok_or_else(|| AppError::conflict("Parent folder not found"))?;
            PutTarget::New {
                folder_id: parent.id,
                name: name.to_string(),
            }
        }
    };

    // M-04 : taille maximale et quota vérifiés avant puis pendant l'écriture,
    // corps écrit en flux sur disque (jamais entièrement en mémoire).
    let replaced = match &target {
        PutTarget::Existing { size, .. } => *size,
        PutTarget::New { .. } => 0,
    };
    let budget = upload_spool::upload_budget_for(&state, user.id, replaced).await?;
    budget.check_declared(declared)?;

    let temp = tempfile::NamedTempFile::new()
        .map_err(|e| AppError::internal_error(format!("Failed to create temp file: {}", e)))?;
    let spooled =
        upload_spool::spool_to_path(req.into_body().into_data_stream(), temp.path(), &budget)
            .await?;

    // Detect real MIME type via magic bytes + extension, falling back to client header.
    let filename = filename_from_path(subpath);
    let head = &spooled.head[..spooled.head.len().min(MIME_SNIFF_BYTES)];
    let content_type = refine_content_type(head, filename, &claimed_type);

    match target {
        PutTarget::Existing { file_id, .. } => {
            // Update existing file — returns FileDto with fresh content-hash etag.
            let updated = upload_service
                .update_file_content_by_id(
                    &file_id,
                    temp.path(),
                    spooled.size,
                    &content_type,
                    Some(spooled.hash),
                    oc_mtime,
                )
                .await
                .map_err(|e| AppError::internal_error(format!("Failed to update file: {}", e)))?;

            Ok(Response::builder()
                .status(StatusCode::NO_CONTENT)
                .header(header::ETAG, format!("\"{}\"", updated.etag))
                .header("oc-etag", format!("\"{}\"", updated.etag))
                .body(Body::empty())
                .unwrap())
        }
        PutTarget::New { folder_id, name } => {
            let file_dto = upload_service
                .upload_file_streaming(
                    name,
                    Some(folder_id),
                    content_type,
                    temp.path(),
                    spooled.size,
                    Some(spooled.hash),
                )
                .await
                .map_err(|e| AppError::internal_error(format!("Failed to create file: {}", e)))?;

            Ok(Response::builder()
                .status(StatusCode::CREATED)
                .header(header::ETAG, format!("\"{}\"", file_dto.etag))
                .header("oc-etag", format!("\"{}\"", file_dto.etag))
                .body(Body::empty())
                .unwrap())
        }
    }
}

// ──────────────────── MKCOL ────────────────────

async fn handle_mkcol(
    state: Arc<AppState>,
    user: &CurrentUser,
    subpath: &str,
) -> Result<Response<Body>, AppError> {
    use crate::application::dtos::folder_dto::CreateFolderDto;

    let folder_service = &state.applications.folder_service;

    // If the resource already exists, return 405 per RFC 4918 §9.3.1
    if owned_paths::resolve_owned(&state, user, subpath)
        .await?
        .is_some()
    {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .unwrap());
    }

    // Collect path segments that need to be created (walk from root to leaf)
    let segments: Vec<&str> = subpath.split('/').filter(|s| !s.is_empty()).collect();
    if segments.iter().any(|seg| *seg == ".." || *seg == ".") {
        return Err(AppError::bad_request("Invalid path: traversal not allowed"));
    }

    // M-02 : création uniquement sous le dossier personnel de l'appelant.
    let mut parent = owned_paths::resolve_home(&state, user).await?;
    let mut current_sub = String::new();

    for segment in &segments {
        if is_reserved_home_name(segment) {
            return Err(AppError::forbidden("This folder name is reserved"));
        }
        current_sub = if current_sub.is_empty() {
            segment.to_string()
        } else {
            format!("{}/{}", current_sub, segment)
        };
        match owned_paths::resolve_owned(&state, user, &current_sub).await? {
            Some(OwnedResource::Folder(existing)) => {
                parent = existing;
            }
            Some(OwnedResource::File(_)) => {
                return Err(AppError::conflict("A file exists at this path"));
            }
            None => {
                let dto = CreateFolderDto {
                    name: segment.to_string(),
                    parent_id: Some(parent.id.clone()),
                };
                match folder_service.create_folder(dto).await {
                    Ok(created) => {
                        parent = created;
                    }
                    Err(e)
                        if e.message.contains("already exists")
                            || e.message.contains("Already Exists") =>
                    {
                        // Race condition — folder created concurrently
                        parent = owned_paths::resolve_owned_folder(&state, user, &current_sub)
                            .await?
                            .ok_or_else(|| {
                                AppError::internal_error("Folder exists but cannot be found")
                            })?;
                    }
                    Err(e) => return Err(AppError::from(e)),
                }
            }
        }
    }

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::empty())
        .unwrap())
}

// ──────────────────── DELETE ────────────────────

async fn handle_delete(
    state: Arc<AppState>,
    user: &CurrentUser,
    subpath: &str,
) -> Result<Response<Body>, AppError> {
    // Le dossier personnel lui-même ne se supprime pas.
    if subpath.trim_matches('/').is_empty() {
        return Err(AppError::forbidden("The home folder cannot be deleted"));
    }

    // M-02 : ressource de l'appelant uniquement.
    let resource = owned_paths::resolve_owned(&state, user, subpath)
        .await?
        .ok_or_else(|| AppError::not_found("Resource not found"))?;

    // Prefer soft-delete (move to trash) when trash service is available.
    // This is what Nextcloud clients expect — items appear in the trashbin.
    if let Some(trash_svc) = state.trash_service.as_ref() {
        let (id, kind) = match &resource {
            OwnedResource::Folder(folder) => (folder.id.as_str(), "folder"),
            OwnedResource::File(file) => (file.id.as_str(), "file"),
        };
        trash_svc
            .move_to_trash(id, kind, user.id)
            .await
            .map_err(|e| AppError::internal_error(format!("Failed to trash {}: {}", kind, e)))?;
        return Ok(Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(Body::empty())
            .unwrap());
    }

    // Fallback: hard delete when trash service is not available.
    match resource {
        OwnedResource::Folder(folder) => {
            state
                .applications
                .folder_service
                .delete_folder(&folder.id, user.id)
                .await
                .map_err(AppError::from)?;
        }
        OwnedResource::File(file) => {
            state
                .applications
                .file_management_service
                .delete_file_owned(&file.id, user.id)
                .await
                .map_err(AppError::from)?;
        }
    }

    Ok(Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(Body::empty())
        .unwrap())
}

// ──────────────────── MOVE ────────────────────

async fn handle_move(
    state: Arc<AppState>,
    req: Request<Body>,
    user: &CurrentUser,
    subpath: &str,
) -> Result<Response<Body>, AppError> {
    use crate::application::dtos::folder_dto::{MoveFolderDto, RenameFolderDto};

    let destination = req
        .headers()
        .get("destination")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::bad_request("Missing Destination header"))?
        .to_string();

    // Parse destination path: extract subpath after /remote.php/dav/files/{user}/
    let dest_subpath = extract_nc_subpath_from_dest(&destination, &user.username)
        .ok_or_else(|| AppError::bad_request("Invalid Destination URL"))?;

    if subpath.trim_matches('/').is_empty() {
        return Err(AppError::forbidden("The home folder cannot be moved"));
    }

    let (dest_parent_sub, dest_name) = owned_paths::split_parent(&dest_subpath);
    if dest_name.is_empty() {
        return Err(AppError::bad_request("Invalid Destination URL"));
    }
    let (src_parent_sub, _) = owned_paths::split_parent(subpath);

    let folder_service = &state.applications.folder_service;
    let file_mgmt = &state.applications.file_management_service;

    // M-02/M-03 : source ET dossier de destination résolus sous la racine
    // de l'appelant ; les opérations `_owned` revérifient le propriétaire.
    let source = owned_paths::resolve_owned(&state, user, subpath)
        .await?
        .ok_or_else(|| AppError::not_found("Source resource not found"))?;

    let dest_parent_id = if src_parent_sub == dest_parent_sub {
        None
    } else {
        let parent = owned_paths::resolve_owned_folder(&state, user, dest_parent_sub)
            .await?
            .ok_or_else(|| AppError::conflict("Destination folder not found"))?;
        Some(parent.id)
    };

    match source {
        OwnedResource::File(file) => {
            if let Some(parent_id) = dest_parent_id {
                file_mgmt
                    .move_file_owned(&file.id, user.id, Some(parent_id))
                    .await
                    .map_err(AppError::from)?;
            }
            if file.name != dest_name {
                file_mgmt
                    .rename_file_owned(&file.id, user.id, dest_name)
                    .await
                    .map_err(AppError::from)?;
            }

            // Return ETag and OC-ETag so Nextcloud clients can track the moved file.
            let mut builder = Response::builder().status(StatusCode::CREATED);
            if let Some(moved) =
                owned_paths::resolve_owned_file(&state, user, &dest_subpath).await?
            {
                builder = builder
                    .header(header::ETAG, format!("\"{}\"", moved.id))
                    .header("oc-etag", format!("\"{}\"", moved.id));
            }
            Ok(builder.body(Body::empty()).unwrap())
        }
        OwnedResource::Folder(folder) => {
            if is_reserved_home_name(dest_name) {
                return Err(AppError::forbidden("This folder name is reserved"));
            }
            if let Some(parent_id) = dest_parent_id {
                folder_service
                    .move_folder(
                        &folder.id,
                        MoveFolderDto {
                            parent_id: Some(parent_id),
                        },
                        user.id,
                    )
                    .await
                    .map_err(AppError::from)?;
            }
            if folder.name != dest_name {
                folder_service
                    .rename_folder(
                        &folder.id,
                        RenameFolderDto {
                            name: dest_name.to_string(),
                        },
                        user.id,
                    )
                    .await
                    .map_err(AppError::from)?;
            }

            Ok(Response::builder()
                .status(StatusCode::CREATED)
                .body(Body::empty())
                .unwrap())
        }
    }
}

/// Extract the subpath from a Destination header URL.
///
/// Only accepts relative paths or absolute URLs whose path starts with the
/// expected DAV prefix.  For full URLs the host is ignored — the path alone is
/// used — so an attacker cannot redirect the server to a different host.
fn extract_nc_subpath_from_dest(dest: &str, username: &str) -> Option<String> {
    let prefix = format!("/remote.php/dav/files/{}/", username);
    // For full URLs, extract the path portion (everything after the authority).
    let path = if dest.starts_with("http://") || dest.starts_with("https://") {
        // Find the start of the path after "scheme://host".
        let after_scheme = dest.split_once("://")?.1;
        let path_start = after_scheme.find('/').unwrap_or(after_scheme.len());
        &after_scheme[path_start..]
    } else {
        dest
    };
    let decoded = urlencoding::decode(path).ok()?;
    let decoded = decoded.trim_end_matches('/');
    decoded
        .strip_prefix(prefix.trim_end_matches('/'))
        .map(|s| s.trim_start_matches('/').to_string())
}

// ────────────── Nextcloud PROPFIND XML Generation ──────────────

use crate::application::dtos::file_dto::FileDto;
use crate::application::dtos::folder_dto::FolderDto;
use crate::application::services::nextcloud_file_id_service::NextcloudFileIdService;

/// Generate a complete Nextcloud-compatible multistatus XML response.
#[allow(clippy::too_many_arguments)]
async fn write_nc_multistatus<W: std::io::Write>(
    writer: W,
    folder: Option<&FolderDto>,
    files: &[FileDto],
    subfolders: &[FolderDto],
    _request: &PropFindRequest,
    depth: &str,
    username: &str,
    subpath: &str,
    file_id_svc: Option<&Arc<NextcloudFileIdService>>,
    favorite_ids: &HashSet<String>,
) -> Result<(), String> {
    let mut xml = Writer::new(writer);

    // Root element with all required namespaces.
    let mut ms = BytesStart::new("d:multistatus");
    ms.push_attribute(("xmlns:d", "DAV:"));
    ms.push_attribute(("xmlns:oc", "http://owncloud.org/ns"));
    ms.push_attribute(("xmlns:nc", "http://nextcloud.org/ns"));
    ms.push_attribute(("xmlns:ocs", "http://open-collaboration-services.org/ns"));
    xml.write_event(Event::Start(ms)).xml_err()?;

    // Current folder entry.
    if let Some(f) = folder {
        let href = nc_href(username, subpath);
        let file_id = resolve_folder_id(file_id_svc, &f.id).await;
        let oc_id = file_id.map(|id| format_oc_id(id, file_id_svc));
        write_folder_response(
            &mut xml,
            f,
            &href,
            file_id,
            oc_id.as_deref(),
            username,
            favorite_ids,
        )?;
    }

    // When folder is None, files are the target resource itself (single-file
    // PROPFIND) and must always be emitted. When folder is Some, files/subfolders
    // are children and should only be listed when depth > 0.
    let emit_children = folder.is_none() || depth != "0";

    if emit_children {
        // Files.
        for file in files {
            let child_sub = if folder.is_none() {
                // Single-file PROPFIND — subpath already points to the file.
                subpath.to_string()
            } else if subpath.is_empty() {
                file.name.clone()
            } else {
                format!("{}/{}", subpath.trim_end_matches('/'), file.name)
            };
            let href = nc_href(username, &child_sub);
            let file_id = resolve_file_id(file_id_svc, &file.id).await;
            let oc_id = file_id.map(|id| format_oc_id(id, file_id_svc));
            write_file_response(
                &mut xml,
                file,
                &href,
                file_id,
                oc_id.as_deref(),
                username,
                favorite_ids,
            )?;
        }

        // Subfolders.
        for sf in subfolders {
            let child_sub = if subpath.is_empty() {
                sf.name.clone()
            } else {
                format!("{}/{}", subpath.trim_end_matches('/'), sf.name)
            };
            let href = format!("{}/", nc_href(username, &child_sub));
            let file_id = resolve_folder_id(file_id_svc, &sf.id).await;
            let oc_id = file_id.map(|id| format_oc_id(id, file_id_svc));
            write_folder_response(
                &mut xml,
                sf,
                &href,
                file_id,
                oc_id.as_deref(),
                username,
                favorite_ids,
            )?;
        }
    }

    xml.write_event(Event::End(BytesEnd::new("d:multistatus")))
        .xml_err()?;

    Ok(())
}

pub fn write_folder_response<W: std::io::Write>(
    xml: &mut Writer<W>,
    folder: &FolderDto,
    href: &str,
    file_id: Option<i64>,
    oc_id: Option<&str>,
    owner: &str,
    favorite_ids: &HashSet<String>,
) -> Result<(), String> {
    xml.write_event(Event::Start(BytesStart::new("d:response")))
        .xml_err()?;

    // href
    write_text_element(xml, "d:href", href)?;

    xml.write_event(Event::Start(BytesStart::new("d:propstat")))
        .xml_err()?;
    xml.write_event(Event::Start(BytesStart::new("d:prop")))
        .xml_err()?;

    // resourcetype
    xml.write_event(Event::Start(BytesStart::new("d:resourcetype")))
        .xml_err()?;
    xml.write_event(Event::Empty(BytesStart::new("d:collection")))
        .xml_err()?;
    xml.write_event(Event::End(BytesEnd::new("d:resourcetype")))
        .xml_err()?;

    write_text_element(xml, "d:displayname", &folder.name)?;

    let created_at =
        chrono::DateTime::<Utc>::from_timestamp(timestamp_to_i64(folder.created_at), 0)
            .unwrap_or_else(Utc::now);
    let modified_at =
        chrono::DateTime::<Utc>::from_timestamp(timestamp_to_i64(folder.modified_at), 0)
            .unwrap_or_else(Utc::now);

    write_text_element(xml, "d:getlastmodified", &modified_at.to_rfc2822())?;
    write_text_element(xml, "d:getetag", &format!("\"{}\"", folder.id))?;
    write_text_element(xml, "d:getcontenttype", "httpd/unix-directory")?;
    write_text_element(xml, "d:getcontentlength", "0")?;
    write_text_element(xml, "d:creationdate", &created_at.to_rfc3339())?;

    // Nextcloud/ownCloud properties
    if let Some(id) = file_id {
        write_text_element(xml, "oc:fileid", &id.to_string())?;
    }
    if let Some(oid) = oc_id {
        write_text_element(xml, "oc:id", oid)?;
    }
    write_text_element(xml, "oc:permissions", "RGDNVCK")?;
    // Numeric share-permissions bitmask: Read=1 + Update=2 + Create=4 + Delete=8 + Share=16 = 31
    write_text_element(xml, "ocs:share-permissions", "31")?;
    write_text_element(xml, "oc:size", "0")?;
    write_text_element(xml, "oc:owner-id", owner)?;
    write_text_element(xml, "oc:owner-display-name", owner)?;
    write_text_element(xml, "nc:has-preview", "false")?;
    write_text_element(xml, "nc:is-encrypted", "0")?;
    write_text_element(xml, "nc:mount-type", "")?;

    let is_fav = if favorite_ids.contains(&folder.id) {
        "1"
    } else {
        "0"
    };
    write_text_element(xml, "oc:favorite", is_fav)?;
    // Empty share-types (no sharing API yet)
    xml.write_event(Event::Empty(BytesStart::new("oc:share-types")))
        .xml_err()?;

    xml.write_event(Event::End(BytesEnd::new("d:prop")))
        .xml_err()?;
    write_text_element(xml, "d:status", "HTTP/1.1 200 OK")?;
    xml.write_event(Event::End(BytesEnd::new("d:propstat")))
        .xml_err()?;

    xml.write_event(Event::End(BytesEnd::new("d:response")))
        .xml_err()?;

    Ok(())
}

pub fn write_file_response<W: std::io::Write>(
    xml: &mut Writer<W>,
    file: &FileDto,
    href: &str,
    file_id: Option<i64>,
    oc_id: Option<&str>,
    owner: &str,
    favorite_ids: &HashSet<String>,
) -> Result<(), String> {
    xml.write_event(Event::Start(BytesStart::new("d:response")))
        .xml_err()?;

    write_text_element(xml, "d:href", href)?;

    xml.write_event(Event::Start(BytesStart::new("d:propstat")))
        .xml_err()?;
    xml.write_event(Event::Start(BytesStart::new("d:prop")))
        .xml_err()?;

    // resourcetype (empty for files)
    xml.write_event(Event::Empty(BytesStart::new("d:resourcetype")))
        .xml_err()?;

    write_text_element(xml, "d:displayname", &file.name)?;
    write_text_element(xml, "d:getcontenttype", &file.mime_type)?;
    write_text_element(xml, "d:getcontentlength", &file.size.to_string())?;

    let created_at = chrono::DateTime::<Utc>::from_timestamp(timestamp_to_i64(file.created_at), 0)
        .unwrap_or_else(Utc::now);
    let modified_at =
        chrono::DateTime::<Utc>::from_timestamp(timestamp_to_i64(file.modified_at), 0)
            .unwrap_or_else(Utc::now);

    write_text_element(xml, "d:getlastmodified", &modified_at.to_rfc2822())?;
    write_text_element(xml, "d:getetag", &format!("\"{}\"", file.etag))?;
    write_text_element(xml, "d:creationdate", &created_at.to_rfc3339())?;

    // Nextcloud/ownCloud properties
    if let Some(id) = file_id {
        write_text_element(xml, "oc:fileid", &id.to_string())?;
    }
    if let Some(oid) = oc_id {
        write_text_element(xml, "oc:id", oid)?;
    }
    write_text_element(xml, "oc:permissions", "RGDNVW")?;
    // Numeric share-permissions bitmask: Read=1 + Update=2 + Delete=8 + Share=16 = 27
    write_text_element(xml, "ocs:share-permissions", "27")?;
    write_text_element(xml, "oc:size", &file.size.to_string())?;
    write_text_element(xml, "oc:owner-id", owner)?;
    write_text_element(xml, "oc:owner-display-name", owner)?;

    let is_fav = if favorite_ids.contains(&file.id) {
        "1"
    } else {
        "0"
    };
    write_text_element(xml, "oc:favorite", is_fav)?;
    // Empty share-types (no sharing API yet)
    xml.write_event(Event::Empty(BytesStart::new("oc:share-types")))
        .xml_err()?;

    // Check if file is an image that can have previews
    let has_preview = matches!(
        &*file.mime_type,
        "image/jpeg" | "image/jpg" | "image/png" | "image/gif" | "image/webp"
    );
    write_text_element(
        xml,
        "nc:has-preview",
        if has_preview { "true" } else { "false" },
    )?;

    write_text_element(xml, "nc:is-encrypted", "0")?;
    write_text_element(xml, "nc:mount-type", "")?;
    write_text_element(xml, "nc:creation_time", &file.created_at.to_string())?;
    write_text_element(xml, "nc:upload_time", &file.modified_at.to_string())?;

    xml.write_event(Event::End(BytesEnd::new("d:prop")))
        .xml_err()?;
    write_text_element(xml, "d:status", "HTTP/1.1 200 OK")?;
    xml.write_event(Event::End(BytesEnd::new("d:propstat")))
        .xml_err()?;

    xml.write_event(Event::End(BytesEnd::new("d:response")))
        .xml_err()?;

    Ok(())
}

pub fn write_text_element<W: std::io::Write>(
    xml: &mut Writer<W>,
    tag: &str,
    value: &str,
) -> Result<(), String> {
    xml.write_event(Event::Start(BytesStart::new(tag)))
        .xml_err()?;
    xml.write_event(Event::Text(BytesText::new(value)))
        .xml_err()?;
    xml.write_event(Event::End(BytesEnd::new(tag))).xml_err()?;
    Ok(())
}

pub async fn resolve_file_id(
    svc: Option<&Arc<NextcloudFileIdService>>,
    file_uuid: &str,
) -> Option<i64> {
    let svc = svc?;
    svc.get_or_create_file_id(file_uuid).await.ok()
}

pub async fn resolve_folder_id(
    svc: Option<&Arc<NextcloudFileIdService>>,
    folder_uuid: &str,
) -> Option<i64> {
    let svc = svc?;
    svc.get_or_create_folder_id(folder_uuid).await.ok()
}

pub fn format_oc_id(id: i64, svc: Option<&Arc<NextcloudFileIdService>>) -> String {
    match svc {
        Some(s) => s.format_oc_id(id),
        None => format!("{:08}ocnca", id),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── nc_to_internal_path ──

    #[test]
    fn test_empty_subpath_returns_home() {
        assert_eq!(
            nc_to_internal_path("alice", "").unwrap(),
            "My Folder - alice"
        );
    }

    #[test]
    fn test_subpath_appended() {
        assert_eq!(
            nc_to_internal_path("alice", "Documents/work").unwrap(),
            "My Folder - alice/Documents/work"
        );
    }

    #[test]
    fn test_strips_surrounding_slashes() {
        assert_eq!(
            nc_to_internal_path("alice", "/Photos/").unwrap(),
            "My Folder - alice/Photos"
        );
    }

    #[test]
    fn test_rejects_dot_dot_traversal() {
        assert!(nc_to_internal_path("alice", "../etc/passwd").is_err());
    }

    #[test]
    fn test_rejects_single_dot() {
        assert!(nc_to_internal_path("alice", "foo/./bar").is_err());
    }

    // ── nc_href ──

    #[test]
    fn test_href_root() {
        assert_eq!(nc_href("alice", ""), "/remote.php/dav/files/alice/");
    }

    #[test]
    fn test_href_encodes_spaces() {
        assert_eq!(
            nc_href("alice", "My Photos/vacation pic.jpg"),
            "/remote.php/dav/files/alice/My%20Photos/vacation%20pic.jpg"
        );
    }

    #[test]
    fn test_href_encodes_special_chars() {
        let href = nc_href("alice", "file#1.txt");
        assert!(href.contains("file%231.txt"));
    }

    // ── extract_nc_subpath_from_dest ──

    #[test]
    fn test_extract_relative_path() {
        let result = extract_nc_subpath_from_dest(
            "/remote.php/dav/files/alice/Documents/moved.txt",
            "alice",
        );
        assert_eq!(result.as_deref(), Some("Documents/moved.txt"));
    }

    #[test]
    fn test_extract_absolute_url() {
        let result = extract_nc_subpath_from_dest(
            "https://cloud.example.com/remote.php/dav/files/alice/new.txt",
            "alice",
        );
        assert_eq!(result.as_deref(), Some("new.txt"));
    }

    #[test]
    fn test_extract_url_encoded() {
        let result = extract_nc_subpath_from_dest(
            "/remote.php/dav/files/alice/My%20Folder/file.txt",
            "alice",
        );
        assert_eq!(result.as_deref(), Some("My Folder/file.txt"));
    }

    #[test]
    fn test_extract_wrong_user_returns_none() {
        let result = extract_nc_subpath_from_dest("/remote.php/dav/files/bob/secret.txt", "alice");
        assert!(result.is_none());
    }

    // ── timestamp_to_i64 ──

    #[test]
    fn test_timestamp_normal() {
        assert_eq!(timestamp_to_i64(1700000000), 1700000000i64);
    }

    #[test]
    fn test_timestamp_overflow_returns_zero() {
        assert_eq!(timestamp_to_i64(u64::MAX), 0);
    }
}
