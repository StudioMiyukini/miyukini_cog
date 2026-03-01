//! Handler pour GET /history.

// @id: service.alicia.api.handlers.history
// @role: history_handler
// @layer: 7
// @human: Endpoint historique des commandes Alicia API.
// @do: handle_history_requests

use std::sync::Arc;

use axum::extract::{Query, State};
use axum::Json;

use crate::auth::{JwtAuth, JwtScope};
use crate::dto::{HistoryEntryDto, HistoryParams, HistoryResponse};
use crate::errors::ApiError;
use crate::router::AppState;

/// GET /api/v1/alicia/history
///
/// Retourne l'historique des commandes avec pagination. JWT scope History requis.
pub async fn list(
    JwtAuth(claims): JwtAuth,
    Query(params): Query<HistoryParams>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<HistoryResponse>, ApiError> {
    claims.require_scope(&JwtScope::History)?;

    // Limiter a 500 max
    let limit = params.limit.min(500);

    let all_entries = state.alicia.recent_activity(MAX_HISTORY_ENTRIES).await;

    // Filtrer par source si specifie
    let filtered: Vec<_> = all_entries
        .into_iter()
        .filter(|entry| {
            if let Some(ref src) = params.source {
                let entry_source = format!("{:?}", entry.source).to_lowercase();
                entry_source == src.to_lowercase()
            } else {
                true
            }
        })
        .filter(|entry| {
            if let Some(device_id) = params.device_id {
                entry.device_id == Some(device_id)
            } else {
                true
            }
        })
        .collect();

    let total = filtered.len() as u64;
    let offset = params.offset as usize;
    let entries: Vec<HistoryEntryDto> = filtered
        .into_iter()
        .skip(offset)
        .take(limit as usize)
        .map(|e| HistoryEntryDto {
            id: e.id,
            source: format!("{:?}", e.source).to_lowercase(),
            source_detail: e.source_detail,
            device_id: e.device_id,
            device_name: e.device_name,
            command: e.command,
            success: e.success,
            error_message: e.error_message,
            latency_ms: e.latency_ms,
            executed_at: e.executed_at,
        })
        .collect();

    Ok(Json(HistoryResponse {
        entries,
        total,
        limit,
        offset: params.offset,
    }))
}

/// Nombre maximum d'entrees a recuperer en memoire.
const MAX_HISTORY_ENTRIES: usize = 500;
