//! Handlers HTTP pour le sous-système des capacités services Alicia (P1-P7).
//!
//! Tous les endpoints requièrent que `AliciaService::enable_capabilities_runtime`
//! ait été appelé au démarrage. Sinon, ils renvoient
//! `503 Service Unavailable` avec un message clair.

// @id service.alicia.api.handlers.capabilities
// @role http_handlers_capabilities
// @layer 7
// @do handle_capabilities_api
// @human Endpoints REST pour skills, dispatch, learnings, persona active, hot reload.

use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

use miyualicia::capabilities_runtime::QueuedDispatch;
use miyualicia::CapabilitiesRuntime;
use miyualicia_dispatcher::Decision;

use crate::auth::{JwtAuth, JwtScope};
use crate::errors::ApiError;
use crate::router::AppState;

/// Récupère le `CapabilitiesRuntime` actif ou renvoie 503.
async fn capabilities_or_503(
    state: &Arc<AppState>,
) -> Result<Arc<CapabilitiesRuntime>, ApiError> {
    state.alicia.capabilities().await.ok_or_else(|| {
        ApiError::InternalError(
            "sous-système capacités désactivé (enable_capabilities_runtime non appelé)".into(),
        )
    })
}

// ----------------------------------------------------------------------
// GET /capabilities/tools — liste des ToolDefinition pour LLM
// ----------------------------------------------------------------------

/// Renvoie la liste de tools OpenAI-compatible exposables au LLM.
pub async fn tools(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Read)?;
    let runtime = capabilities_or_503(&state).await?;
    let tools = runtime.list_tools().await;
    Ok(Json(serde_json::json!({ "tools": tools })))
}

// ----------------------------------------------------------------------
// GET /capabilities/services — service_ids couverts par un client
// ----------------------------------------------------------------------

/// Liste les `service_id` pour lesquels un `ServiceClient` est enregistré.
pub async fn services(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Read)?;
    let runtime = capabilities_or_503(&state).await?;
    let services = runtime.registered_services().await;
    Ok(Json(serde_json::json!({ "services": services })))
}

// ----------------------------------------------------------------------
// POST /capabilities/dispatch — exécute une capability
// ----------------------------------------------------------------------

/// Body POST `/capabilities/dispatch` (et variantes confirm/cancel).
#[derive(Debug, Deserialize)]
pub struct DispatchBody {
    /// Identifiant qualifié de la capability (`service_id.intent`).
    pub qualified_id: String,
    /// Persona active.
    pub persona: String,
    /// Phrase utilisateur pour audit.
    pub user_phrase: String,
    /// Paramètres JSON (objet ou null).
    #[serde(default)]
    pub params: Value,
}

/// Représentation JSON d'une `Decision`.
fn decision_to_json(d: Decision) -> Value {
    match d {
        Decision::Executed(r) => serde_json::json!({
            "kind": "executed",
            "qualified_id": r.qualified_id,
            "service_result": {
                "success": r.service_result.success,
                "data":    r.service_result.data,
                "message": r.service_result.message,
            },
        }),
        Decision::PendingConfirmation { qualified_id, prompt } => serde_json::json!({
            "kind": "pending_confirmation",
            "qualified_id": qualified_id,
            "prompt": prompt,
        }),
    }
}

pub async fn dispatch(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Json(body): Json<DispatchBody>,
) -> Result<Json<Value>, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Write)?;
    let runtime = capabilities_or_503(&state).await?;
    let decision = runtime
        .dispatch_capability(&body.qualified_id, &body.params, &body.persona, &body.user_phrase)
        .await?;
    Ok(Json(decision_to_json(decision)))
}

pub async fn dispatch_confirm(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Json(body): Json<DispatchBody>,
) -> Result<Json<Value>, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Write)?;
    let runtime = capabilities_or_503(&state).await?;
    let result = runtime
        .dispatch_confirmed(&body.qualified_id, &body.params, &body.persona, &body.user_phrase)
        .await?;
    Ok(Json(serde_json::json!({
        "kind": "executed",
        "qualified_id": result.qualified_id,
        "service_result": {
            "success": result.service_result.success,
            "data":    result.service_result.data,
            "message": result.service_result.message,
        },
    })))
}

pub async fn dispatch_cancel(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Json(body): Json<DispatchBody>,
) -> Result<StatusCode, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Write)?;
    let runtime = capabilities_or_503(&state).await?;
    runtime
        .dispatch_cancelled(&body.qualified_id, &body.params, &body.persona, &body.user_phrase)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

// ----------------------------------------------------------------------
// Apprentissage — pending / accept / reject / detect
// ----------------------------------------------------------------------

pub async fn learnings_pending(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Path(persona): Path<String>,
) -> Result<Json<Value>, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Read)?;
    let runtime = capabilities_or_503(&state).await?;
    let pending = runtime
        .memory()
        .pending_learned_skills(&persona)
        .map_err(|e| ApiError::InternalError(format!("memory: {e}")))?;
    Ok(Json(serde_json::json!({ "pending": pending })))
}

pub async fn learnings_active(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Path(persona): Path<String>,
) -> Result<Json<Value>, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Read)?;
    let runtime = capabilities_or_503(&state).await?;
    let active = runtime
        .memory()
        .active_learned_skills(&persona)
        .map_err(|e| ApiError::InternalError(format!("memory: {e}")))?;
    Ok(Json(serde_json::json!({ "active": active })))
}

pub async fn learnings_accept(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Write)?;
    let runtime = capabilities_or_503(&state).await?;
    runtime
        .memory()
        .accept_learned_skill(id)
        .map_err(|e| ApiError::InternalError(format!("memory: {e}")))?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn learnings_reject(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Write)?;
    let runtime = capabilities_or_503(&state).await?;
    runtime
        .memory()
        .reject_learned_skill(id)
        .map_err(|e| ApiError::InternalError(format!("memory: {e}")))?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn learnings_detect(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Path(persona): Path<String>,
) -> Result<Json<Value>, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Write)?;
    let runtime = capabilities_or_503(&state).await?;
    let patterns = runtime.detect_patterns(&persona).await?;
    Ok(Json(serde_json::json!({ "patterns": patterns })))
}

// ----------------------------------------------------------------------
// Persona active
// ----------------------------------------------------------------------

pub async fn persona_active(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Read)?;
    let runtime = capabilities_or_503(&state).await?;
    let id = runtime
        .memory()
        .active_persona_id()
        .map_err(|e| ApiError::InternalError(format!("memory: {e}")))?;
    Ok(Json(serde_json::json!({ "persona_id": id })))
}

pub async fn persona_activate(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Write)?;
    let runtime = capabilities_or_503(&state).await?;
    runtime
        .memory()
        .set_active_persona(&id)
        .map_err(|e| match e {
            miyualicia_memory::MemoryError::UnknownPersona(p) => ApiError::NotFound(format!("persona '{p}'")),
            other => ApiError::InternalError(format!("memory: {other}")),
        })?;
    Ok(StatusCode::NO_CONTENT)
}

// ----------------------------------------------------------------------
// Hot reload manuel
// ----------------------------------------------------------------------

pub async fn manifest_reload(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Admin)?;
    let runtime = capabilities_or_503(&state).await?;
    match runtime.tick_manifest_watcher().await? {
        None => Ok(Json(serde_json::json!({ "outcome": "unchanged" }))),
        Some(report) => Ok(Json(serde_json::json!({
            "outcome": "reloaded",
            "added":   report.added,
            "kept":    report.kept,
            "removed": report.removed,
            "errors":  report.errors.iter().map(|e| e.to_string()).collect::<Vec<_>>(),
        }))),
    }
}

// ----------------------------------------------------------------------
// Episodes récents (lecture mémoire)
// ----------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct EpisodesQuery {
    /// Limite (default 20).
    #[serde(default)]
    pub limit: Option<usize>,
}

pub async fn episodes_recent(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Query(query): Query<EpisodesQuery>,
) -> Result<Json<Value>, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Read)?;
    let runtime = capabilities_or_503(&state).await?;
    let limit = query.limit.unwrap_or(20).min(200);
    let episodes = runtime
        .memory()
        .recent_episodes(limit)
        .map_err(|e| ApiError::InternalError(format!("memory: {e}")))?;
    Ok(Json(serde_json::json!({
        "episodes": episodes,
        "count":    episodes.len(),
    })))
}

// ----------------------------------------------------------------------
// Queue async des PendingConfirmation
// ----------------------------------------------------------------------

/// POST `/capabilities/dispatch_and_queue` — comme `/dispatch` mais
/// **enregistre automatiquement** la confirmation dans la queue si la
/// décision est `PendingConfirmation`. Renvoie l'`id` pour résolution
/// ultérieure.
pub async fn dispatch_and_queue(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Json(body): Json<DispatchBody>,
) -> Result<Json<Value>, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Write)?;
    let runtime = capabilities_or_503(&state).await?;
    let outcome = runtime
        .dispatch_and_queue(&body.qualified_id, &body.params, &body.persona, &body.user_phrase)
        .await?;
    let body = match outcome {
        QueuedDispatch::Executed(r) => serde_json::json!({
            "kind": "executed",
            "qualified_id": r.qualified_id,
            "service_result": {
                "success": r.service_result.success,
                "data":    r.service_result.data,
                "message": r.service_result.message,
            },
        }),
        QueuedDispatch::Queued { id, qualified_id, prompt } => serde_json::json!({
            "kind": "queued",
            "id": id.to_string(),
            "qualified_id": qualified_id,
            "prompt": prompt,
        }),
    };
    Ok(Json(body))
}

/// GET `/capabilities/pending_confirmations` — liste les pending confirmations
/// actives, optionnellement filtrées par `?persona=...`.
#[derive(Debug, Deserialize)]
pub struct PendingFilterQuery {
    /// Filtre persona (`None` = toutes personas).
    pub persona: Option<String>,
}

pub async fn pending_list(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Query(q): Query<PendingFilterQuery>,
) -> Result<Json<Value>, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Read)?;
    let runtime = capabilities_or_503(&state).await?;
    let pending = match q.persona.as_deref() {
        Some(p) => runtime.pending_confirmations_for(p).await?,
        None => runtime.list_pending_confirmations().await?,
    };
    Ok(Json(serde_json::json!({
        "pending": pending,
        "count":   pending.len(),
    })))
}

pub async fn pending_confirm(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Write)?;
    let runtime = capabilities_or_503(&state).await?;
    let result = runtime.confirm_queued(id).await?;
    Ok(Json(serde_json::json!({
        "kind": "executed",
        "qualified_id": result.qualified_id,
        "service_result": {
            "success": result.service_result.success,
            "data":    result.service_result.data,
            "message": result.service_result.message,
        },
    })))
}

pub async fn pending_cancel(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Write)?;
    let runtime = capabilities_or_503(&state).await?;
    runtime.cancel_queued(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
