//! @id     miyucloud_interfaces_api_handlers_file_handler
//! @do     definir_global_state
//! @role   orchestration
//! @layer  app
//! @human  miyucloud — module « interfaces api handlers file handler ».

use axum::{
    Json,
    body::Body,
    extract::{Multipart, Path, Query, State},
    http::{HeaderMap, Response, StatusCode, header},
    response::IntoResponse,
};
use bytes::Bytes;
use http_range_header::parse_range_header;
use serde::Deserialize;
use std::collections::HashMap;

use crate::application::ports::file_ports::OptimizedFileContent;
use crate::application::ports::file_ports::{
    FileManagementUseCase, FileRetrievalUseCase, FileUploadUseCase,
};
use crate::application::ports::storage_ports::{FileReadPort, StorageUsagePort};
use crate::application::ports::thumbnail_ports::ThumbnailPort;
use crate::application::services::file_retrieval_service::FileRetrievalService;
use crate::common::di::AppState;
use crate::common::errors::{DomainError, ErrorKind};
use crate::domain::entities::etat_cdr::{
    ENTETE_ETAT_CDR, EtatCdr, MESSAGE_QUARANTAINE, PARAM_ACCORD_NON_DESARME, accord_explicite,
};
use crate::interfaces::errors::{AppError, ReponseOu500, reponse_json};
use crate::interfaces::middleware::auth::AuthUser;
use std::sync::Arc;
use uuid::Uuid;

/**
 * Type aliases for dependency injection state.
 */
/// Global application state for dependency injection
type GlobalState = Arc<AppState>;

/**
 * API handler for file-related operations.
 *
 * Acts as a thin HTTP adapter in the hexagonal architecture: it parses requests,
 * delegates business logic to application services, and maps results to HTTP
 * responses.  No infrastructure or strategy logic lives here.
 */
pub struct FileHandler;

impl FileHandler {
    // ═══════════════════════════════════════════════════════════════════════
    //  UPLOAD
    // ═══════════════════════════════════════════════════════════════════════

    /// Streaming file upload — constant ~64 KB RAM regardless of file size.
    ///
    /// **Hash-on-Write**: BLAKE3 is computed while spooling the multipart
    /// body to the temp file. This eliminates the second sequential read
    /// that dedup_service would otherwise need, cutting total I/O in half.
    pub async fn upload_file(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
        multipart: Multipart,
    ) -> impl IntoResponse {
        match Self::upload_file_inner(&state, &auth_user, multipart).await {
            Ok((file, _blob_hash)) => Self::created_json_response(&file).into_response(),
            Err(response) => response.into_response(),
        }
    }

    /// Core upload logic shared by [`Self::upload_file`] and
    /// [`Self::upload_file_with_thumbnails`].
    ///
    /// Returns `(FileDto, blob_hash)` on success.  The blob hash is the
    /// BLAKE3 digest computed during the hash-on-write spool and is
    /// propagated without an extra database round-trip so that callers
    /// (e.g. thumbnail generation) can resolve the physical blob path
    /// immediately.
    async fn upload_file_inner(
        state: &GlobalState,
        auth_user: &AuthUser,
        mut multipart: Multipart,
    ) -> Result<(crate::application::dtos::file_dto::FileDto, String), Response<Body>> {
        let upload_service = &state.applications.file_upload_service;
        let mut folder_id: Option<String> = None;

        tracing::debug!("📤 Processing streaming file upload (hash-on-write)");

        while let Some(field) = multipart.next_field().await.unwrap_or(None) {
            let name = field.name().unwrap_or("").to_string();

            if name == "folder_id" {
                let v = field.text().await.unwrap_or_default();
                if !v.is_empty() {
                    folder_id = Some(v);
                }
                continue;
            }

            if name == "file" {
                let raw_filename = field.file_name().unwrap_or("unnamed").to_string();
                // Browsers send the full relative path (e.g. "Screenshots/file.png")
                // as the filename for folder uploads via webkitRelativePath.
                // Strip path components to get the basename only.
                // This also prevents path-traversal attacks.
                let filename = raw_filename
                    .rsplit('/')
                    .next()
                    .unwrap_or(&raw_filename)
                    .rsplit('\\')
                    .next()
                    .unwrap_or(&raw_filename)
                    .to_string();
                let content_type = field
                    .content_type()
                    .unwrap_or("application/octet-stream")
                    .to_string();

                // ── SECURITY: Verify folder ownership before upload (IDOR V-03 fix) ──
                if let Some(ref fid) = folder_id {
                    use crate::application::ports::inbound::FolderUseCase;
                    let folder_service = &state.applications.folder_service;
                    if folder_service
                        .get_folder_owned(fid, auth_user.id)
                        .await
                        .is_err()
                    {
                        tracing::warn!(
                            "⛔ UPLOAD REJECTED (IDOR): user='{}' attempted upload to folder '{}' owned by another user",
                            auth_user.username,
                            fid,
                        );
                        return Err(Self::domain_error_response(
                            crate::common::errors::DomainError::not_found("Folder", fid),
                        ));
                    }
                }

                // ── Early quota check (before spooling to disk) ──────
                if let Some(storage_svc) = state.storage_usage_service.as_ref() {
                    let estimated_size = field
                        .headers()
                        .get(header::CONTENT_LENGTH)
                        .and_then(|v| v.to_str().ok())
                        .and_then(|s| s.parse::<u64>().ok())
                        .unwrap_or(0);
                    if let Err(err) = storage_svc
                        .check_storage_quota(auth_user.id, estimated_size)
                        .await
                    {
                        tracing::warn!(
                            "⛔ UPLOAD REJECTED (early quota): user={}, file={}, est_size={}",
                            auth_user.username,
                            filename,
                            estimated_size
                        );
                        return Err(Self::quota_error_response(err));
                    }
                }

                // ── Spool multipart field to temp file + hash-on-write ──
                // .dedup_temp is created once by DedupService::initialize() at startup
                let temp_dir = state.core.path_service.get_root_path().join(".dedup_temp");
                let temp_path = temp_dir.join(format!("upload-{}", uuid::Uuid::new_v4()));

                let mut total_size: u64 = 0;
                let mut hasher = blake3::Hasher::new();
                let spool_result: Result<(), String> = async {
                    let file = tokio::fs::File::create(&temp_path)
                        .await
                        .map_err(|e| format!("Failed to create temp file: {}", e))?;

                    // Pre-allocate if Content-Length is known (reduces fragmentation)
                    let hint = field
                        .headers()
                        .get(axum::http::header::CONTENT_LENGTH)
                        .and_then(|v| v.to_str().ok())
                        .and_then(|s| s.parse::<u64>().ok());
                    if let Some(len) = hint {
                        let _ = file.set_len(len).await; // best-effort
                    }

                    // 512 KB buffer — 8× fewer write syscalls than 64 KB
                    let mut writer = tokio::io::BufWriter::with_capacity(524_288, file);
                    let mut field = field;
                    // IMPORTANT: use explicit match instead of `while let Ok(Some(..))`.
                    // The old pattern silently swallowed Err (client disconnect)
                    // and accepted partially received data as a complete upload.
                    loop {
                        match field.chunk().await {
                            Ok(Some(chunk)) => {
                                total_size += chunk.len() as u64;
                                hasher.update(&chunk);
                                tokio::io::AsyncWriteExt::write_all(&mut writer, &chunk)
                                    .await
                                    .map_err(|e| format!("Failed to write chunk: {}", e))?;
                            }
                            Ok(None) => break, // End of field — upload complete
                            Err(e) => {
                                return Err(format!(
                                    "Connection lost during upload (received {} bytes): {}",
                                    total_size, e
                                ));
                            }
                        }
                    }
                    tokio::io::AsyncWriteExt::flush(&mut writer)
                        .await
                        .map_err(|e| format!("Failed to flush temp file: {}", e))?;
                    Ok(())
                }
                .await;

                if let Err(e) = spool_result {
                    let _ = tokio::fs::remove_file(&temp_path).await;
                    tracing::error!("❌ UPLOAD SPOOL FAILED: {} - {}", filename, e);
                    return Err(Self::domain_error_response(
                        crate::common::errors::DomainError::internal_error("FileUpload", e),
                    ));
                }

                // Empty file — use streaming path with the (empty) temp file
                if total_size == 0 {
                    let hash = hasher.finalize().to_hex().to_string();
                    let dto = upload_service
                        .upload_file_streaming(
                            filename,
                            folder_id,
                            content_type,
                            &temp_path,
                            0,
                            Some(hash),
                        )
                        .await
                        .map_err(Self::domain_error_response)?;
                    let stored_hash = dto.etag.clone();
                    return Ok((dto, stored_hash));
                }

                // Finalize hash
                let hash = hasher.finalize().to_hex().to_string();

                // ── MIME validation + detection (magic bytes + blocklist) ─
                let blocked_types = &state.config.security.blocked_mime_types;
                let content_type = match crate::common::mime_detect::validate_upload_mime_from_file(
                    &temp_path,
                    &filename,
                    &content_type,
                    blocked_types,
                )
                .await
                {
                    Ok(validated) => validated,
                    Err(reason) => {
                        let _ = tokio::fs::remove_file(&temp_path).await;
                        tracing::warn!(
                            "⛔ UPLOAD REJECTED (MIME): user='{}', file='{}': {}",
                            auth_user.username,
                            filename,
                            reason
                        );
                        return Err(Self::domain_error_response(
                            crate::common::errors::DomainError::validation_error(reason),
                        ));
                    }
                };

                // ── Quota enforcement ────────────────────────────────
                if let Some(storage_svc) = state.storage_usage_service.as_ref()
                    && let Err(err) = storage_svc
                        .check_storage_quota(auth_user.id, total_size)
                        .await
                {
                    let _ = tokio::fs::remove_file(&temp_path).await;
                    tracing::warn!(
                        "⛔ UPLOAD REJECTED (quota): user={}, file={}, size={}",
                        auth_user.username,
                        filename,
                        total_size
                    );
                    return Err(Self::quota_error_response(err));
                }

                // ── Streaming upload (CDR funnel → blob store) ──────────
                // EX-45 : `upload_file_streaming` passe le fichier dans le CDR
                // avant rangement ; le blob et le hachage stockés sont ceux des
                // octets reconstruits (renvoyés via `file.etag`).
                match upload_service
                    .upload_file_streaming(
                        filename.clone(),
                        folder_id,
                        content_type,
                        &temp_path,
                        total_size,
                        Some(hash.clone()),
                    )
                    .await
                {
                    Ok(file) => {
                        // Emit audit event for successful upload
                        if let Some(ref audit) = state.audit_service {
                            use crate::application::ports::audit_ports::AuditPort;
                            use crate::domain::entities::audit_event::*;
                            audit.record(
                                AuditEvent::new(
                                    AuditAction::FileUpload,
                                    ResourceType::File,
                                    AuditOutcome::Success,
                                )
                                .with_user(auth_user.id, &auth_user.username)
                                .with_resource(&file.id)
                                .with_details(serde_json::json!({
                                    "filename": &filename,
                                    "size": file.size,
                                    "received_size": total_size,
                                    "etat_cdr": &file.etat_cdr,
                                })),
                            );
                        }
                        tracing::info!(
                            "✅ STREAMING UPLOAD: {} ({} bytes, ID: {})",
                            filename,
                            file.size,
                            file.id
                        );
                        let stored_hash = file.etag.clone();
                        return Ok((file, stored_hash));
                    }
                    Err(err) => {
                        let _ = tokio::fs::remove_file(&temp_path).await;
                        tracing::error!("❌ UPLOAD FAILED: {} - {}", filename, err);
                        return Err(Self::domain_error_response(err));
                    }
                }
            }
        }

        Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "No file provided"
            })),
        )
            .into_response())
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  THUMBNAILS
    // ═══════════════════════════════════════════════════════════════════════

    /// Get a thumbnail for a file (image or video).
    ///
    /// **Cache-first**: if the thumbnail already exists in the moka in-memory
    /// cache or on disk, serve it immediately — **zero DB queries**.  The
    /// ownership check was already performed when the thumbnail was first
    /// generated (at upload) or uploaded (PUT by the owner).  UUIDv4 file IDs
    /// have 122 bits of entropy, making enumeration infeasible.
    ///
    /// **ETag / 304**: responses carry an immutable ETag.  If the browser
    /// sends `If-None-Match` matching the ETag, we return 304 Not Modified
    /// without touching cache or DB — pure header round-trip.
    ///
    /// The DB path is only taken on a **cache miss for images** where the
    /// thumbnail hasn't been generated yet (first access after upload if
    /// background generation hasn't finished).
    pub async fn get_thumbnail(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
        headers: HeaderMap,
        Path((id, size)): Path<(String, String)>,
    ) -> impl IntoResponse {
        use crate::application::ports::thumbnail_ports::ThumbnailSize;

        let thumbnail_service = &state.core.thumbnail_service;

        let thumb_size = match size.as_str() {
            "icon" => ThumbnailSize::Icon,
            "preview" => ThumbnailSize::Preview,
            "large" => ThumbnailSize::Large,
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": "Invalid thumbnail size. Use: icon, preview, or large"
                    })),
                )
                    .into_response();
            }
        };

        // ── ETag short-circuit (Solution C) ──────────────────────────
        // Thumbnails are immutable — the ETag never changes for a given
        // (file_id, size) pair.  If the browser already has it, return 304
        // with zero I/O or DB work.
        let etag = format!("\"thumb-{}-{:?}\"", id, thumb_size);
        if let Some(if_none_match) = headers.get(header::IF_NONE_MATCH)
            && let Ok(val) = if_none_match.to_str()
            && (val == etag || val == "*")
        {
            return Response::builder()
                .status(StatusCode::NOT_MODIFIED)
                .header(header::ETAG, &etag)
                .header(header::CACHE_CONTROL, "public, max-age=31536000, immutable")
                .body(Body::empty())
                .ou_500()
                .into_response();
        }

        // ── Propriété puis EX-45, avant tout accès au cache ──────────
        // Anti-IDOR : l'état CDR n'est lu qu'à travers une requête limitée au
        // propriétaire ; un fichier d'autrui répond le même 404 qu'un id
        // inexistant, sans en-tête d'état. Puis aucune vignette pour un
        // fichier non désarmé (quarantaine ou antérieur au CDR), y compris
        // depuis le cache : le contenu a pu être remplacé depuis la génération.
        if let Some(refus) = Self::porte_vignette(
            &*state.applications.file_retrieval_service,
            &id,
            auth_user.id,
        )
        .await
        {
            return refus;
        }

        // ── Cache-first path (Solution A) ────────────────────────────
        // Propriété déjà confirmée ci-dessus : le cache (moka → disque) n'est
        // servi qu'au propriétaire.
        if let Some(data) = thumbnail_service
            .get_cached_thumbnail(&id, thumb_size.into())
            .await
        {
            return Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "image/jpeg")
                .header(header::CONTENT_LENGTH, data.len())
                .header(header::CACHE_CONTROL, "public, max-age=31536000, immutable")
                .header(header::ETAG, &etag)
                .body(Body::from(data))
                .ou_500()
                .into_response();
        }

        // ── Cache miss — need DB for ownership + blob resolution ─────
        let file_retrieval_service = &state.applications.file_retrieval_service;

        let file = match file_retrieval_service
            .get_file_owned(&id, auth_user.id)
            .await
        {
            Ok(f) => f,
            Err(err) => return Self::reponse_erreur_acces(err),
        };

        // Non-image (video, etc.) with no cached thumbnail → 204
        if !thumbnail_service.is_supported_image(&file.mime_type) {
            return Response::builder()
                .status(StatusCode::NO_CONTENT)
                .header(header::CACHE_CONTROL, "no-store")
                .body(Body::empty())
                .ou_500()
                .into_response();
        }

        // Resolve the physical blob path (content-addressable storage)
        let blob_hash = match state
            .repositories
            .file_read_repository
            .get_blob_hash(&id)
            .await
        {
            Ok(hash) => hash,
            Err(_) => {
                return AppError::internal_error("File blob not found").into_response();
            }
        };
        let file_path = state.core.dedup_service.blob_path(&blob_hash);

        match thumbnail_service
            .get_thumbnail(&id, thumb_size.into(), &file_path)
            .await
        {
            Ok(data) => Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "image/jpeg")
                .header(header::CONTENT_LENGTH, data.len())
                .header(header::CACHE_CONTROL, "public, max-age=31536000, immutable")
                .header(header::ETAG, &etag)
                .body(Body::from(data))
                .ou_500()
                .into_response(),
            Err(err) => AppError::internal_error(format!("Thumbnail generation failed: {}", err))
                .into_response(),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  UPLOAD THUMBNAIL (client-generated, e.g. video frames)
    // ═══════════════════════════════════════════════════════════════════════

    /// Accept a client-generated thumbnail (e.g. video frame extracted via
    /// `<video>` + `<canvas>` in the browser) and persist it in the server
    /// cache.  The image is validated, re-encoded to WebP, and stored so
    /// subsequent `GET …/thumbnail/{size}` requests are served instantly.
    ///
    /// **Max body: 512 KB** — thumbnails are small.
    pub async fn upload_thumbnail(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
        Path((id, size)): Path<(String, String)>,
        body: Bytes,
    ) -> impl IntoResponse {
        use crate::application::ports::thumbnail_ports::ThumbnailSize;

        let thumbnail_service = &state.core.thumbnail_service;

        // Validate size
        let thumb_size = match size.as_str() {
            "icon" => ThumbnailSize::Icon,
            "preview" => ThumbnailSize::Preview,
            "large" => ThumbnailSize::Large,
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": "Invalid thumbnail size. Use: icon, preview, or large"
                    })),
                )
                    .into_response();
            }
        };

        // Reject oversized payloads (512 KB)
        if body.len() > 512 * 1024 {
            return (
                StatusCode::PAYLOAD_TOO_LARGE,
                Json(serde_json::json!({ "error": "Thumbnail exceeds 512 KB limit" })),
            )
                .into_response();
        }

        // Propriété (requête limitée au propriétaire) puis EX-45 : pas de
        // vignette pour un fichier non désarmé. Fichier d'autrui ⇒ même 404
        // qu'un id inexistant (anti-IDOR).
        let etat = match Self::etat_cdr_apres_acces(
            &*state.applications.file_retrieval_service,
            &id,
            auth_user.id,
        )
        .await
        {
            Ok(etat) => etat,
            Err(refus) => return *refus,
        };
        if !etat.autorise_traitement_contenu() {
            return AppError::forbidden(format!(
                "Vignette refusée : fichier non désarmé (état CDR « {etat} »)"
            ))
            .into_response();
        }

        // EX-45 : la vignette fournie par le client est un fichier entrant —
        // désarmée par le CDR avant tout décodage par le service de vignettes.
        let body = match state
            .applications
            .desarmement_service
            .desarmer_image(body.to_vec())
            .await
        {
            Ok(reconstruit) => Bytes::from(reconstruit.octets),
            Err(err) => return AppError::from(err).into_response(),
        };

        // Validate, re-encode to WebP, and store
        match thumbnail_service
            .store_external_thumbnail(&id, thumb_size.into(), body)
            .await
        {
            Ok(_) => StatusCode::CREATED.into_response(),
            Err(err) => AppError::internal_error(format!("Failed to store thumbnail: {}", err))
                .into_response(),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  DOWNLOAD
    // ═══════════════════════════════════════════════════════════════════════

    /// Downloads a file with optimized multi-tier strategy.
    ///
    /// The tier selection (write-behind → hot cache → WebP transcode → mmap →
    /// streaming) is fully handled by `FileRetrievalUseCase::get_file_optimized`.
    /// This handler only deals with HTTP concerns: ETag, Range, Content-Disposition,
    /// and optional compression.
    pub async fn download_file(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
        Path(id): Path<String>,
        Query(params): Query<HashMap<String, String>>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        let retrieval = &state.applications.file_retrieval_service;

        // ── Get file metadata (ownership-scoped) ────────────────────────
        // Fichier d'autrui ⇒ même 404 qu'un id inexistant (anti-IDOR).
        let file_dto = match retrieval.get_file_owned(&id, auth_user.id).await {
            Ok(f) => f,
            Err(err) => return Self::reponse_erreur_acces(err),
        };

        // ── EX-45 : état CDR (jamais en cache, suit le contenu) ────────
        // Lu après la propriété, et lui-même limité au propriétaire.
        let etat = match Self::etat_cdr_apres_acces(&**retrieval, &id, auth_user.id).await {
            Ok(etat) => etat,
            Err(refus) => return *refus,
        };

        // ── Metadata-only request ────────────────────────────────────
        if params
            .get("metadata")
            .is_some_and(|v| v == "true" || v == "1")
        {
            return (
                StatusCode::OK,
                Json(serde_json::json!({
                    "id": file_dto.id,
                    "name": file_dto.name,
                    "path": file_dto.path,
                    "size": file_dto.size,
                    "mime_type": file_dto.mime_type,
                    "folder_id": file_dto.folder_id,
                    "created_at": file_dto.created_at,
                    "modified_at": file_dto.modified_at,
                    "etat_cdr": etat.as_str(),
                })),
            )
                .into_response();
        }

        // ── EX-45 : quarantaine → accord explicite obligatoire ───────
        let accord = accord_explicite(params.get(PARAM_ACCORD_NON_DESARME).map(String::as_str));
        if !etat.autorise_telechargement(accord) {
            return Self::refus_non_desarme(etat);
        }
        // Non désarmé : servi en pièce jointe opaque, jamais transcodé ni
        // affiché en ligne (aucun composant COG n'ouvre le contenu).
        let ouvrable = etat.autorise_traitement_contenu();
        let mime_servi: std::sync::Arc<str> = if ouvrable {
            file_dto.mime_type.clone()
        } else {
            std::sync::Arc::from("application/octet-stream")
        };
        let disposition_params = if ouvrable {
            params.clone()
        } else {
            HashMap::new()
        };

        let etag = format!("\"{}-{}\"", id, file_dto.modified_at);

        // ── ETag (304 Not Modified) ──────────────────────────────────
        if let Some(inm) = headers.get(header::IF_NONE_MATCH)
            && let Ok(client_etag) = inm.to_str()
            && (client_etag == etag || client_etag == "*")
        {
            return Response::builder()
                .status(StatusCode::NOT_MODIFIED)
                .header(header::ETAG, &etag)
                .body(Body::empty())
                .ou_500()
                .into_response();
        }

        // ── Range Requests ───────────────────────────────────────────
        if let Some(range_header) = headers.get(header::RANGE)
            && let Ok(range_str) = range_header.to_str()
            && let Ok(ranges) = parse_range_header(range_str)
        {
            let validated = ranges.validate(file_dto.size);
            if let Ok(valid_ranges) = validated {
                if let Some(range) = valid_ranges.first() {
                    let start = *range.start();
                    let end = *range.end();
                    let range_length = end - start + 1;
                    let disposition =
                        Self::content_disposition(&file_dto.name, &mime_servi, &disposition_params);

                    match retrieval
                        .get_file_range_stream_owned(&id, auth_user.id, start, Some(end + 1))
                        .await
                    {
                        Ok(stream) => {
                            return Response::builder()
                                .status(StatusCode::PARTIAL_CONTENT)
                                .header(header::CONTENT_TYPE, &*mime_servi)
                                .header(ENTETE_ETAT_CDR, etat.as_str())
                                .header(header::CONTENT_DISPOSITION, &disposition)
                                .header(header::CONTENT_LENGTH, range_length)
                                .header(
                                    header::CONTENT_RANGE,
                                    format!("bytes {}-{}/{}", start, end, file_dto.size),
                                )
                                .header(header::ACCEPT_RANGES, "bytes")
                                .header(header::ETAG, &etag)
                                .header(
                                    header::CACHE_CONTROL,
                                    "private, max-age=3600, must-revalidate",
                                )
                                .body(Body::from_stream(Box::into_pin(stream)))
                                .ou_500()
                                .into_response();
                        }
                        Err(err) => {
                            tracing::error!("Error creating range stream: {}", err);
                            // fall through to normal download
                        }
                    }
                }
            } else {
                return Response::builder()
                    .status(StatusCode::RANGE_NOT_SATISFIABLE)
                    .header(header::CONTENT_RANGE, format!("bytes */{}", file_dto.size))
                    .body(Body::empty())
                    .ou_500()
                    .into_response();
            }
        }

        // ── Normal download (delegated to service) ───────────────────
        let disposition =
            Self::content_disposition(&file_dto.name, &mime_servi, &disposition_params);

        let accept_webp = ouvrable
            && headers
                .get(header::ACCEPT)
                .and_then(|v| v.to_str().ok())
                .is_some_and(|a| a.contains("image/webp"));
        let prefer_original = !ouvrable
            || params
                .get("original")
                .is_some_and(|v| v == "true" || v == "1");

        // Use the ownership-scoped optimized download.
        // Ownership was already verified by get_file_owned above,
        // so we can safely use the preloaded variant.
        match retrieval
            .get_file_optimized_preloaded(&id, file_dto.clone(), accept_webp, prefer_original)
            .await
        {
            Ok((_file, content)) => match content {
                OptimizedFileContent::Bytes {
                    data, mime_type, ..
                } => {
                    let servi = if ouvrable {
                        mime_type
                    } else {
                        mime_servi.clone()
                    };
                    let mut resp = Self::build_cached_response(data, &servi, &disposition, &etag);
                    if let Ok(v) = header::HeaderValue::from_str(etat.as_str()) {
                        resp.headers_mut().insert(ENTETE_ETAT_CDR, v);
                    }
                    resp.into_response()
                }
                OptimizedFileContent::Mmap(mmap_data) => Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, &*mime_servi)
                    .header(ENTETE_ETAT_CDR, etat.as_str())
                    .header(header::CONTENT_DISPOSITION, &disposition)
                    .header(header::CONTENT_LENGTH, mmap_data.len())
                    .header(header::ETAG, &etag)
                    .header(
                        header::CACHE_CONTROL,
                        "private, max-age=3600, must-revalidate",
                    )
                    .header(header::ACCEPT_RANGES, "bytes")
                    .body(Body::from(mmap_data))
                    .ou_500()
                    .into_response(),
                OptimizedFileContent::Stream(pinned_stream) => Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, &*mime_servi)
                    .header(ENTETE_ETAT_CDR, etat.as_str())
                    .header(header::CONTENT_DISPOSITION, &disposition)
                    .header(header::CONTENT_LENGTH, file_dto.size)
                    .header(header::ETAG, &etag)
                    .header(
                        header::CACHE_CONTROL,
                        "private, max-age=3600, must-revalidate",
                    )
                    .header(header::ACCEPT_RANGES, "bytes")
                    .body(Body::from_stream(pinned_stream))
                    .ou_500()
                    .into_response(),
            },
            Err(err) => AppError::from(err).into_response(),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  LIST
    // ═══════════════════════════════════════════════════════════════════════

    /// Lists files, extracting `folder_id` from query parameters.
    ///
    /// Axum-compatible handler wrapper around [`Self::list_files`].
    pub async fn list_files_query(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
        headers: HeaderMap,
        Query(params): Query<HashMap<String, String>>,
    ) -> impl IntoResponse {
        let folder_id = params.get("folder_id").map(|id| id.as_str());
        tracing::info!("API: Listing files with folder_id: {:?}", folder_id);

        let retrieval = &state.applications.file_retrieval_service;
        match retrieval.list_files_owned(folder_id, auth_user.id).await {
            Ok(files) => {
                // Compute lightweight ETag from max modified_at + count
                let max_mod = files.iter().map(|f| f.modified_at).max().unwrap_or(0);
                let count = files.len();
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                std::hash::Hash::hash(&max_mod, &mut hasher);
                std::hash::Hash::hash(&count, &mut hasher);
                let etag = format!("\"{:x}\"", std::hash::Hasher::finish(&hasher));

                // 304 Not Modified if client already has this version
                if let Some(inm) = headers.get(header::IF_NONE_MATCH)
                    && let Ok(client_etag) = inm.to_str()
                    && client_etag == etag
                {
                    return Response::builder()
                        .status(StatusCode::NOT_MODIFIED)
                        .header(header::ETAG, &etag)
                        .body(Body::empty())
                        .ou_500()
                        .into_response();
                }

                tracing::info!("Found {} files", files.len());
                let mut resp = (StatusCode::OK, Json(files)).into_response();
                // ETag facultatif : une valeur non représentable est omise.
                if let Ok(valeur) = header::HeaderValue::from_str(&etag) {
                    resp.headers_mut().insert(header::ETAG, valeur);
                }
                resp
            }
            Err(err) => AppError::from(err).into_response(),
        }
    }

    /// Uploads a file and generates thumbnails in the background for images.
    ///
    /// Delegates to [`Self::upload_file_inner`] and, on success, spawns
    /// a background task to generate all thumbnail sizes before serialising
    /// the `FileDto` once.
    pub async fn upload_file_with_thumbnails(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
        multipart: Multipart,
    ) -> impl IntoResponse {
        let (file, blob_hash) = match Self::upload_file_inner(&state, &auth_user, multipart).await {
            Ok(pair) => pair,
            Err(response) => return response.into_response(),
        };

        // Generate thumbnails for supported images in background.
        // The blob_hash was already computed during the hash-on-write spool,
        // so we resolve the physical path directly — no extra DB round-trip.
        // EX-45 : seuls les fichiers désarmés sont ouverts pour les vignettes.
        let desarme = file.etat_cdr.as_deref() == Some(EtatCdr::Desarme.as_str());
        if desarme
            && state
                .core
                .thumbnail_service
                .is_supported_image(&file.mime_type)
        {
            let file_id = file.id.clone();
            let thumbnail_service = state.core.thumbnail_service.clone();
            let file_path = state.core.dedup_service.blob_path(&blob_hash);

            tokio::spawn(async move {
                tracing::info!("🖼️ Generating thumbnails for: {}", file_id);
                thumbnail_service.generate_all_sizes_background(file_id, file_path);
            });
        }

        Self::created_json_response(&file).into_response()
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  METADATA
    // ═══════════════════════════════════════════════════════════════════════

    /// Returns EXIF/media metadata for a file.
    ///
    /// Used by the Photos lightbox and for testing EXIF extraction.
    pub async fn get_file_metadata(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
        Path(file_id): Path<String>,
    ) -> impl IntoResponse {
        // Verify ownership
        let file_read = &state.repositories.file_read_repository;
        if let Err(e) = file_read.verify_file_owner(&file_id, auth_user.id).await {
            let msg = e.to_string();
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": msg })),
            )
                .into_response();
        }

        let metadata_repo = &state.repositories.file_metadata_repository;
        match metadata_repo.get(&file_id).await {
            Ok(Some(meta)) => (StatusCode::OK, Json(meta)).into_response(),
            Ok(None) => (
                StatusCode::OK,
                Json(serde_json::json!({
                    "file_id": file_id,
                    "message": "No EXIF metadata available"
                })),
            )
                .into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
                .into_response(),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  DELETE
    // ═══════════════════════════════════════════════════════════════════════

    /// Deletes a file (trash-first with dedup cleanup).
    ///
    /// All logic (trash fallback, dedup ref-count, hash computation) is handled
    /// by `FileManagementUseCase::delete_with_cleanup`.
    ///
    /// When auth is available, uses trash-first deletion; otherwise falls back
    /// to permanent delete so the endpoint works with or without auth.
    pub async fn delete_file(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        let mgmt = &state.applications.file_management_service;

        // Auth required: trash-first with dedup cleanup + ownership verification
        let result = mgmt
            .delete_with_cleanup(&id, auth_user.id)
            .await
            .map(|was_trashed| {
                if was_trashed {
                    tracing::info!("File moved to trash: {}", id);
                } else {
                    tracing::info!("File permanently deleted: {}", id);
                }
            });

        match result {
            Ok(_) => StatusCode::NO_CONTENT.into_response(),
            Err(err) => AppError::from(err).into_response(),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  MOVE
    // ═══════════════════════════════════════════════════════════════════════

    /// Renames a file (ownership-verified)
    pub async fn rename_file(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
        Path(id): Path<String>,
        Json(payload): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let new_name = match payload.get("name").and_then(|v| v.as_str()) {
            Some(name) if !name.trim().is_empty() => name.trim().to_string(),
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": "Missing or empty 'name' field"
                    })),
                )
                    .into_response();
            }
        };

        tracing::info!("Renaming file {} to \"{}\"", id, new_name);
        let mgmt = &state.applications.file_management_service;
        match mgmt.rename_file_owned(&id, auth_user.id, &new_name).await {
            Ok(file_dto) => (StatusCode::OK, Json(file_dto)).into_response(),
            Err(err) => AppError::from(err).into_response(),
        }
    }

    /// Moves a file to a different folder (ownership-verified)
    pub async fn move_file(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
        Path(id): Path<String>,
        Json(payload): Json<MoveFilePayload>,
    ) -> impl IntoResponse {
        tracing::info!("Moving file {} to folder {:?}", id, payload.folder_id);

        let mgmt = &state.applications.file_management_service;

        match mgmt
            .move_file_owned(&id, auth_user.id, payload.folder_id)
            .await
        {
            Ok(file) => (StatusCode::OK, Json(file)).into_response(),
            Err(err) => AppError::from(err).into_response(),
        }
    }

    /// Moves a file to a different folder (simplified payload, ownership-verified)
    pub async fn move_file_simple(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
        Path(id): Path<String>,
        Json(payload): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let folder_id = payload
            .get("folder_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let mgmt = &state.applications.file_management_service;
        match mgmt.move_file_owned(&id, auth_user.id, folder_id).await {
            Ok(file_dto) => (StatusCode::OK, Json(file_dto)).into_response(),
            Err(err) => AppError::from(err).into_response(),
        }
    }

    // ═══════════════════════════════════════════════════════════════════════
    //  PRIVATE HELPERS
    // ═══════════════════════════════════════════════════════════════════════

    /// Build a Content-Disposition header value.
    ///
    /// Uses RFC 5987 `filename*=UTF-8''<percent-encoded>` to safely handle
    /// filenames with quotes, non-ASCII characters, or other special chars.
    /// A sanitised ASCII `filename=` fallback is included for legacy clients.
    fn content_disposition(name: &str, mime: &str, params: &HashMap<String, String>) -> String {
        let force_inline = params
            .get("inline")
            .is_some_and(|v| v == "true" || v == "1");
        let disposition = if force_inline
            || mime.starts_with("image/")
            || mime == "application/pdf"
            || mime.starts_with("video/")
            || mime.starts_with("audio/")
        {
            "inline"
        } else {
            "attachment"
        };

        // RFC 5987 percent-encode for filename* (attr-char safe set)
        use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, utf8_percent_encode};
        // Characters that DON'T need encoding per RFC 5987 attr-char:
        //   ALPHA / DIGIT / "!" / "#" / "$" / "&" / "+" / "-" / "." /
        //   "^" / "_" / "`" / "|" / "~"
        const RFC5987_SET: &AsciiSet = &NON_ALPHANUMERIC
            .remove(b'!')
            .remove(b'#')
            .remove(b'$')
            .remove(b'&')
            .remove(b'+')
            .remove(b'-')
            .remove(b'.')
            .remove(b'^')
            .remove(b'_')
            .remove(b'`')
            .remove(b'|')
            .remove(b'~');
        let encoded = utf8_percent_encode(name, RFC5987_SET).to_string();

        // ASCII fallback: strip anything outside printable ASCII and
        // replace '"' and '\\' to prevent header injection.
        let ascii_safe: String = name
            .chars()
            .filter(|c| c.is_ascii_graphic() || *c == ' ')
            .map(|c| match c {
                '"' | '\\' => '_',
                _ => c,
            })
            .collect();

        format!("{disposition}; filename=\"{ascii_safe}\"; filename*=UTF-8''{encoded}")
    }

    /// Réponse uniforme « introuvable » : même statut, mêmes en-têtes et même
    /// corps pour un fichier inexistant et pour un fichier d'autrui (anti-IDOR).
    /// Aucun en-tête d'état CDR.
    fn reponse_introuvable() -> Response<Body> {
        AppError::not_found("File not found").into_response()
    }

    /// Erreur d'accès → réponse. Introuvable et accès refusé sont confondus
    /// en un 404 uniforme pour ne jamais révéler l'existence d'un fichier.
    fn reponse_erreur_acces(err: DomainError) -> Response<Body> {
        match err.kind {
            ErrorKind::NotFound | ErrorKind::AccessDenied => Self::reponse_introuvable(),
            _ => AppError::from(err).into_response(),
        }
    }

    /// État CDR lu **seulement** à travers la source limitée au propriétaire
    /// (EX-45, anti-IDOR) : un tiers obtient le 404 uniforme, jamais l'état.
    async fn etat_cdr_apres_acces<S: EtatCdrProprietaire>(
        source: &S,
        id: &str,
        utilisateur: Uuid,
    ) -> Result<EtatCdr, Box<Response<Body>>> {
        source
            .etat_cdr_proprietaire(id, utilisateur)
            .await
            .map_err(|err| Box::new(Self::reponse_erreur_acces(err)))
    }

    /// Porte de la vignette : `None` ⇒ poursuivre ; `Some(r)` ⇒ répondre `r`
    /// (404 uniforme si non-propriétaire, 204 + état CDR si non désarmé).
    async fn porte_vignette<S: EtatCdrProprietaire>(
        source: &S,
        id: &str,
        utilisateur: Uuid,
    ) -> Option<Response<Body>> {
        match Self::etat_cdr_apres_acces(source, id, utilisateur).await {
            Ok(etat) if etat.autorise_traitement_contenu() => None,
            Ok(etat) => Some(Self::sans_vignette(etat)),
            Err(refus) => Some(*refus),
        }
    }

    /// Réponse « pas de vignette » pour un fichier non désarmé (EX-45).
    fn sans_vignette(etat: EtatCdr) -> Response<Body> {
        Response::builder()
            .status(StatusCode::NO_CONTENT)
            .header(header::CACHE_CONTROL, "no-store")
            .header(ENTETE_ETAT_CDR, etat.as_str())
            .body(Body::empty())
            .ou_500()
    }

    /// Refus de téléchargement d'un fichier en quarantaine sans accord (EX-45).
    fn refus_non_desarme(etat: EtatCdr) -> Response<Body> {
        let mut resp = (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({
                "error": MESSAGE_QUARANTAINE,
                "error_type": "NonDesarme",
                "etat_cdr": etat.as_str(),
                "parametre_accord": PARAM_ACCORD_NON_DESARME,
            })),
        )
            .into_response();
        if let Ok(v) = header::HeaderValue::from_str(etat.as_str()) {
            resp.headers_mut().insert(ENTETE_ETAT_CDR, v);
        }
        resp
    }

    /// Build a 201 Created JSON response.
    fn created_json_response(file: &crate::application::dtos::file_dto::FileDto) -> Response<Body> {
        reponse_json(
            Response::builder()
                .status(StatusCode::CREATED)
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::CACHE_CONTROL, "no-cache, no-store, must-revalidate"),
            file,
        )
    }

    /// Build error response for DomainError.
    fn domain_error_response(err: crate::common::errors::DomainError) -> Response<Body> {
        AppError::from(err).into_response()
    }

    /// Build a quota-specific error response with 507 status and structured body.
    fn quota_error_response(err: crate::common::errors::DomainError) -> Response<Body> {
        AppError::from(err).into_response()
    }

    /// Build response for cached/small files.
    ///
    /// Compression is handled uniformly by `CompressionLayer` (tower-http)
    /// which negotiates `Accept-Encoding` and applies gzip/brotli in streaming
    /// mode. No manual compression is done here to avoid double-encoding.
    fn build_cached_response(
        content: Bytes,
        mime_type: &str,
        disposition: &str,
        etag: &str,
    ) -> Response<Body> {
        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, mime_type)
            .header(header::CONTENT_DISPOSITION, disposition)
            .header(header::ETAG, etag)
            .header(
                header::CACHE_CONTROL,
                "private, max-age=3600, must-revalidate",
            )
            .header(header::VARY, "Accept-Encoding")
            .header(header::CONTENT_LENGTH, content.len())
            .body(Body::from(content))
            .ou_500()
    }
}

/// Source d'état CDR limitée au propriétaire (EX-45, anti-IDOR). Les
/// gestionnaires ne lisent l'état CDR qu'à travers elle : la propriété est
/// vérifiée avant (dans la même requête que) la lecture de l'état.
pub(crate) trait EtatCdrProprietaire {
    async fn etat_cdr_proprietaire(
        &self,
        id: &str,
        proprietaire: Uuid,
    ) -> Result<EtatCdr, DomainError>;
}

impl EtatCdrProprietaire for FileRetrievalService {
    async fn etat_cdr_proprietaire(
        &self,
        id: &str,
        proprietaire: Uuid,
    ) -> Result<EtatCdr, DomainError> {
        self.etat_cdr_owned(id, proprietaire).await
    }
}

/// Payload for moving a file
#[derive(Debug, Deserialize)]
pub struct MoveFilePayload {
    /// Target folder ID (None means root)
    pub folder_id: Option<String>,
}

#[cfg(test)]
mod tests_cdr {
    use super::*;

    /// Source factice reproduisant la sémantique SQL de
    /// `get_etat_cdr_owned` : `WHERE id = $1 AND user_id = $2`.
    struct SourceFactice {
        fichiers: HashMap<String, (Uuid, EtatCdr)>,
    }

    impl EtatCdrProprietaire for SourceFactice {
        async fn etat_cdr_proprietaire(
            &self,
            id: &str,
            proprietaire: Uuid,
        ) -> Result<EtatCdr, DomainError> {
            match self.fichiers.get(id) {
                Some((p, etat)) if *p == proprietaire => Ok(*etat),
                _ => Err(DomainError::not_found("File", id)),
            }
        }
    }

    const ID_ALICE: &str = "11111111-1111-1111-1111-111111111111";
    const ID_INEXISTANT: &str = "22222222-2222-2222-2222-222222222222";

    fn source_alice(alice: Uuid, etat: EtatCdr) -> SourceFactice {
        SourceFactice {
            fichiers: HashMap::from([(ID_ALICE.to_string(), (alice, etat))]),
        }
    }

    /// Statut, en-têtes et corps complets d'une réponse.
    async fn decomposer(resp: Response<Body>) -> (StatusCode, header::HeaderMap, bytes::Bytes) {
        let (parts, body) = resp.into_parts();
        let corps = axum::body::to_bytes(body, usize::MAX).await.unwrap();
        (parts.status, parts.headers, corps)
    }

    #[tokio::test]
    async fn vignette_fichier_d_autrui_identique_a_inexistant() {
        let (alice, bob) = (Uuid::new_v4(), Uuid::new_v4());
        for etat in [EtatCdr::Quarantaine, EtatCdr::Inconnu, EtatCdr::Desarme] {
            let source = source_alice(alice, etat);
            let autrui = FileHandler::porte_vignette(&source, ID_ALICE, bob)
                .await
                .expect("refus attendu pour un tiers");
            let inexistant = FileHandler::porte_vignette(&source, ID_INEXISTANT, bob)
                .await
                .expect("refus attendu pour un id inexistant");
            let autrui = decomposer(autrui).await;
            assert_eq!(autrui.0, StatusCode::NOT_FOUND);
            assert!(autrui.1.get(ENTETE_ETAT_CDR).is_none());
            assert_eq!(autrui, decomposer(inexistant).await);
        }
    }

    #[tokio::test]
    async fn vignette_proprietaire_recoit_l_etat() {
        let alice = Uuid::new_v4();
        let source = source_alice(alice, EtatCdr::Quarantaine);
        let resp = FileHandler::porte_vignette(&source, ID_ALICE, alice)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::NO_CONTENT);
        assert_eq!(resp.headers().get(ENTETE_ETAT_CDR).unwrap(), "quarantaine");
        let source = source_alice(alice, EtatCdr::Desarme);
        assert!(
            FileHandler::porte_vignette(&source, ID_ALICE, alice)
                .await
                .is_none()
        );
    }

    #[tokio::test]
    async fn telechargement_fichier_d_autrui_identique_a_inexistant() {
        let (alice, bob) = (Uuid::new_v4(), Uuid::new_v4());
        let source = source_alice(alice, EtatCdr::Quarantaine);
        // Même enchaînement que `download_file` : erreur de propriété puis
        // état CDR limité au propriétaire.
        let autrui = FileHandler::etat_cdr_apres_acces(&source, ID_ALICE, bob)
            .await
            .unwrap_err();
        let inexistant = FileHandler::etat_cdr_apres_acces(&source, ID_INEXISTANT, bob)
            .await
            .unwrap_err();
        let autrui = decomposer(*autrui).await;
        assert_eq!(autrui.0, StatusCode::NOT_FOUND);
        assert!(autrui.1.get(ENTETE_ETAT_CDR).is_none());
        assert_eq!(autrui, decomposer(*inexistant).await);

        // L'erreur de `get_file_owned` (NotFound ou AccessDenied) donne la
        // même réponse uniforme.
        for err in [
            DomainError::not_found("File", ID_ALICE),
            DomainError::access_denied("File", "not owner"),
        ] {
            let r = decomposer(FileHandler::reponse_erreur_acces(err)).await;
            assert_eq!(r, autrui);
        }
    }

    #[tokio::test]
    async fn telechargement_proprietaire_quarantaine_refusee_avec_etat() {
        let alice = Uuid::new_v4();
        let source = source_alice(alice, EtatCdr::Quarantaine);
        let etat = FileHandler::etat_cdr_apres_acces(&source, ID_ALICE, alice)
            .await
            .unwrap();
        assert!(!etat.autorise_telechargement(false));
        let resp = FileHandler::refus_non_desarme(etat);
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
        assert_eq!(resp.headers().get(ENTETE_ETAT_CDR).unwrap(), "quarantaine");
    }

    #[test]
    fn quarantaine_sans_accord_refusee_avec_explication() {
        let resp = FileHandler::refus_non_desarme(EtatCdr::Quarantaine);
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
        assert_eq!(resp.headers().get(ENTETE_ETAT_CDR).unwrap(), "quarantaine");
    }

    #[test]
    fn pas_de_vignette_pour_non_desarme() {
        for etat in [EtatCdr::Quarantaine, EtatCdr::Inconnu] {
            let resp = FileHandler::sans_vignette(etat);
            assert_eq!(resp.status(), StatusCode::NO_CONTENT);
            assert_eq!(resp.headers().get(ENTETE_ETAT_CDR).unwrap(), etat.as_str());
        }
    }

    #[test]
    fn non_desarme_servi_en_piece_jointe() {
        // Le gestionnaire passe un type opaque et aucun paramètre `inline`.
        let d = FileHandler::content_disposition(
            "doc.pdf",
            "application/octet-stream",
            &HashMap::new(),
        );
        assert!(d.starts_with("attachment;"));
        assert!(accord_explicite(Some("1")));
        assert!(!EtatCdr::Quarantaine.autorise_telechargement(false));
        assert!(EtatCdr::Quarantaine.autorise_telechargement(true));
    }
}
