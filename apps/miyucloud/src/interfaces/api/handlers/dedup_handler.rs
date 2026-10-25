//! @id     miyucloud_interfaces_api_handlers_dedup_handler
//! @do     definir_global_state
//! @role   orchestration
//! @layer  app
//! @human  miyucloud — module « interfaces api handlers dedup handler ».

use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::{Response, StatusCode, header},
    response::IntoResponse,
};
use serde::Serialize;

use crate::application::ports::dedup_ports::DedupResultDto;
use crate::common::di::AppState;
use crate::interfaces::errors::{AppError, ReponseOu500, reponse_json};
use crate::interfaces::middleware::auth::AuthUser;
use crate::interfaces::upload_spool;
use std::sync::Arc;

/// Global application state for dependency injection
type GlobalState = Arc<AppState>;

/// Réponse JSON d'erreur `{"error": ...}`.
fn json_error(status: StatusCode, message: &str) -> axum::response::Response {
    let body = serde_json::json!({ "error": message }).to_string();
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body))
        .unwrap_or_default()
        .into_response()
}

/// Response for hash check endpoint
#[derive(Debug, Serialize)]
pub struct HashCheckResponse {
    /// Whether a blob with this hash already exists
    pub exists: bool,
    /// The SHA-256 hash that was checked
    pub hash: String,
    /// If exists, the size of the existing blob
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_size: Option<u64>,
    /// If exists, the number of references to this blob
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_count: Option<u32>,
}

/// Response for upload with dedup endpoint
#[derive(Debug, Serialize)]
pub struct DedupUploadResponse {
    /// Whether this was a new file or an existing one
    pub is_new: bool,
    /// The SHA-256 hash of the content
    pub hash: String,
    /// The size of the content in bytes
    pub size: u64,
    /// Bytes saved by deduplication (0 if new file)
    pub bytes_saved: u64,
    /// Current reference count for this blob
    pub ref_count: u32,
}

/// Response for dedup stats endpoint
#[derive(Debug, Serialize)]
pub struct StatsResponse {
    /// Total number of unique blobs stored
    pub unique_blobs: u64,
    /// Total number of references (files pointing to blobs)
    pub total_references: u64,
    /// Total bytes saved by deduplication
    pub bytes_saved: u64,
    /// Total logical bytes (what users think they have)
    pub total_logical_bytes: u64,
    /// Total physical bytes (actual disk usage)
    pub total_physical_bytes: u64,
    /// Deduplication ratio (logical / physical)
    pub dedup_ratio: f64,
    /// Percentage of storage saved
    pub savings_percentage: f64,
}

/// Handler for deduplication-related endpoints
///
/// Provides endpoints for:
/// - Checking if content already exists (by hash)
/// - Uploading files with automatic deduplication
/// - Getting deduplication statistics
pub struct DedupHandler;

impl DedupHandler {
    /// Check if the authenticated user already has a file with the given hash.
    ///
    /// User-scoped: only reveals whether **this user** owns a file that
    /// references the blob — never exposes global existence or ref_count.
    ///
    /// GET /api/dedup/check/{hash}
    pub async fn check_hash(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
        Path(hash): Path<String>,
    ) -> impl IntoResponse {
        let dedup = &state.core.dedup_service;

        // Validate hash format (SHA-256 = 64 hex chars)
        if hash.len() != 64 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    r#"{"error": "Invalid hash format. Expected SHA-256 (64 hex characters)"}"#,
                ))
                .ou_500()
                .into_response();
        }

        // Only reveal whether THIS user has the blob — no global oracle
        let user_has_it = dedup
            .user_owns_blob_reference(&hash, &auth_user.id.to_string())
            .await;

        if user_has_it {
            // Fetch size from metadata (safe — user owns a reference)
            let size = dedup.get_blob_metadata(&hash).await.map(|m| m.size);
            let response = HashCheckResponse {
                exists: true,
                hash,
                existing_size: size,
                ref_count: None, // Never expose global ref_count
            };
            reponse_json(
                Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "application/json"),
                &response,
            )
        } else {
            let response = HashCheckResponse {
                exists: false,
                hash,
                existing_size: None,
                ref_count: None,
            };
            reponse_json(
                Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "application/json"),
                &response,
            )
        }
    }

    /// Upload content with automatic deduplication
    ///
    /// This endpoint calculates the SHA-256 hash of the uploaded content
    /// and either creates a new blob or increments the reference count
    /// of an existing blob.
    ///
    /// POST /api/dedup/upload
    ///
    /// Returns information about whether the content was new or deduplicated.
    pub async fn upload_with_dedup(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
        mut multipart: Multipart,
    ) -> impl IntoResponse {
        // M-04 / M-11 : les blobs déposés ici ne sont rattachés à aucun fichier
        // et la réponse révèle l'existence globale d'un contenu — route
        // réservée aux administrateurs.
        if auth_user.role != "admin" {
            return json_error(StatusCode::FORBIDDEN, "Admin role required");
        }

        let dedup = &state.core.dedup_service;

        // M-04 : taille maximale et quota vérifiés pendant l'écriture en flux.
        let budget = match upload_spool::upload_budget_for(&state, auth_user.id, 0).await {
            Ok(b) => b,
            Err(e) => return e.into_response(),
        };

        // Process multipart form
        loop {
            let field = match multipart.next_field().await {
                Ok(Some(field)) => field,
                Ok(None) => break,
                Err(e) => {
                    return json_error(
                        StatusCode::BAD_REQUEST,
                        &format!("Invalid multipart body: {}", e),
                    );
                }
            };
            if field.name().unwrap_or("") != "file" {
                continue;
            }
            let content_type = field
                .content_type()
                .unwrap_or("application/octet-stream")
                .to_string();

            let temp = match tempfile::NamedTempFile::new() {
                Ok(t) => t,
                Err(e) => {
                    tracing::error!("Dedup upload: temp file creation failed: {}", e);
                    return json_error(StatusCode::INTERNAL_SERVER_ERROR, "Upload failed");
                }
            };
            let spooled = match upload_spool::spool_to_path(field, temp.path(), &budget).await {
                Ok(s) => s,
                Err(e) => {
                    tracing::warn!("Dedup upload refused: {}", e);
                    return AppError::from(e).into_response();
                }
            };
            if spooled.size == 0 {
                return json_error(StatusCode::BAD_REQUEST, "Empty file not allowed");
            }

            // EX-45 : le blob déposé passe par le CDR ; sans ligne de fichier,
            // aucune quarantaine n'est représentable → seuls les contenus
            // reconstruits sont acceptés (400 sinon).
            let issue = match state
                .applications
                .desarmement_service
                .desarmer_sans_quarantaine(temp.path(), &content_type)
                .await
            {
                Ok(issue) => issue,
                Err(e) => {
                    tracing::warn!("Dedup upload refused by CDR: {}", e);
                    return AppError::from(e).into_response();
                }
            };

            // Store with deduplication (streaming from disk, constant RAM).
            return match dedup
                .store_from_file(temp.path(), Some(issue.type_mime), issue.hachage)
                .await
            {
                Ok(result) => {
                    let (is_new, bytes_saved) = match &result {
                        DedupResultDto::NewBlob { .. } => (true, 0),
                        DedupResultDto::ExistingBlob { saved_bytes, .. } => (false, *saved_bytes),
                    };

                    let metadata = dedup.get_blob_metadata(result.hash()).await;

                    let response = DedupUploadResponse {
                        is_new,
                        hash: result.hash().to_string(),
                        size: result.size(),
                        bytes_saved,
                        ref_count: metadata.map(|m| m.ref_count).unwrap_or(1),
                    };

                    tracing::info!(
                        "🔗 Dedup upload: hash={}, new={}, saved={}",
                        result.hash(),
                        is_new,
                        bytes_saved
                    );

                    let status = if is_new {
                        StatusCode::CREATED
                    } else {
                        StatusCode::OK
                    };
                    match serde_json::to_string(&response) {
                        Ok(body) => Response::builder()
                            .status(status)
                            .header(header::CONTENT_TYPE, "application/json")
                            .body(Body::from(body))
                            .unwrap_or_default()
                            .into_response(),
                        Err(_) => json_error(StatusCode::INTERNAL_SERVER_ERROR, "Upload failed"),
                    }
                }
                Err(e) => {
                    tracing::error!("Dedup upload failed: {}", e);
                    json_error(StatusCode::INTERNAL_SERVER_ERROR, "Upload failed")
                }
            };
        }

        json_error(
            StatusCode::BAD_REQUEST,
            "No file field found in multipart form",
        )
    }

    /// Get deduplication statistics
    ///
    /// GET /api/dedup/stats
    ///
    /// Returns comprehensive statistics about the deduplication system including:
    /// - Number of unique blobs
    /// - Total references
    /// - Bytes saved
    /// - Deduplication ratio
    pub async fn get_stats(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
    ) -> impl IntoResponse {
        // Admin-only — global dedup statistics are sensitive infrastructure data
        if auth_user.role != "admin" {
            return Response::builder()
                .status(StatusCode::FORBIDDEN)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{"error": "Admin role required"}"#))
                .ou_500()
                .into_response();
        }

        let dedup = &state.core.dedup_service;
        let stats = dedup.get_stats().await;

        // Calculate savings percentage
        let savings_pct = if stats.total_bytes_referenced > 0 {
            (stats.bytes_saved as f64 / stats.total_bytes_referenced as f64) * 100.0
        } else {
            0.0
        };

        let response = StatsResponse {
            unique_blobs: stats.total_blobs,
            total_references: stats.dedup_hits + stats.total_blobs, // Approximation
            bytes_saved: stats.bytes_saved,
            total_logical_bytes: stats.total_bytes_referenced,
            total_physical_bytes: stats.total_bytes_stored,
            dedup_ratio: stats.dedup_ratio,
            savings_percentage: savings_pct,
        };

        reponse_json(
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json"),
            &response,
        )
    }

    /// Retrieve content by hash (user-scoped).
    ///
    /// GET /api/dedup/blob/{hash}
    ///
    /// Returns the raw content of a blob **only if** the authenticated user
    /// owns at least one file that references it. Returns 404 otherwise
    /// (does not reveal whether the blob exists globally).
    pub async fn get_blob(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
        Path(hash): Path<String>,
    ) -> impl IntoResponse {
        let dedup = &state.core.dedup_service;

        // Validate hash format
        if hash.len() != 64 || !hash.chars().all(|c| c.is_ascii_hexdigit()) {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{"error": "Invalid hash format"}"#))
                .ou_500()
                .into_response();
        }

        // Verify the user owns at least one file referencing this blob, and
        // (EX-45) that this file is disarmed: quarantined or `inconnu` content
        // is never served by hash — same 404 (no oracle).
        if !dedup
            .user_owns_desarmed_blob_reference(&hash, &auth_user.id.to_string())
            .await
        {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{"error": "Blob not found"}"#))
                .ou_500()
                .into_response();
        }

        // Get metadata first for content-type
        let metadata = dedup.get_blob_metadata(&hash).await;
        let content_type = metadata
            .as_ref()
            .and_then(|m| m.content_type.clone())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        // Stream blob in 64 KB chunks — constant memory regardless of size
        let size = match dedup.blob_size(&hash).await {
            Ok(s) => s,
            Err(_) => {
                return Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"error": "Blob not found"}"#))
                    .ou_500()
                    .into_response();
            }
        };

        match dedup.read_blob_stream(&hash).await {
            Ok(stream) => Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, content_type)
                .header(header::CONTENT_LENGTH, size.to_string())
                .header("X-Dedup-Hash", &hash)
                .body(Body::from_stream(stream))
                .ou_500()
                .into_response(),
            Err(_) => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{"error": "Blob not found"}"#))
                .ou_500()
                .into_response(),
        }
    }

    /// Force recalculation of statistics from disk
    ///
    /// POST /api/dedup/recalculate
    ///
    /// Verifies integrity and returns current statistics.
    /// Useful for health checks and auditing.
    pub async fn recalculate_stats(
        State(state): State<GlobalState>,
        auth_user: AuthUser,
    ) -> impl IntoResponse {
        // Admin-only — integrity verification is a privileged operation
        if auth_user.role != "admin" {
            return Response::builder()
                .status(StatusCode::FORBIDDEN)
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(r#"{"error": "Admin role required"}"#))
                .ou_500()
                .into_response();
        }

        let dedup = &state.core.dedup_service;

        // Verify integrity first
        match dedup.verify_integrity().await {
            Ok(issues) => {
                if !issues.is_empty() {
                    tracing::warn!("Dedup integrity issues found: {:?}", issues);
                }
            }
            Err(e) => {
                tracing::error!("Dedup integrity verification failed: {}", e);
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"error": "Verification failed"}"#))
                    .ou_500()
                    .into_response();
            }
        }

        let stats = dedup.get_stats().await;

        // Calculate savings percentage
        let savings_pct = if stats.total_bytes_referenced > 0 {
            (stats.bytes_saved as f64 / stats.total_bytes_referenced as f64) * 100.0
        } else {
            0.0
        };

        let response = StatsResponse {
            unique_blobs: stats.total_blobs,
            total_references: stats.dedup_hits + stats.total_blobs,
            bytes_saved: stats.bytes_saved,
            total_logical_bytes: stats.total_bytes_referenced,
            total_physical_bytes: stats.total_bytes_stored,
            dedup_ratio: stats.dedup_ratio,
            savings_percentage: savings_pct,
        };

        reponse_json(
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json"),
            &response,
        )
    }
}
