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

use crate::auth::{JwtAuth, JwtClaims, JwtScope};
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

/// Résout la persona d'une requête à partir de l'identité authentifiée
/// (R-002, C-20) — jamais d'un champ de requête seul.
///
/// - `requested = Some(p)` : `p` doit figurer dans `claims.personas` (403 sinon) ;
/// - `requested = None` : persona active côté serveur si elle est autorisée,
///   sinon l'unique persona autorisée du client, sinon 403.
///
/// Fail-closed : un client sans persona configurée ne peut rien dispatcher,
/// y compris avec le scope `admin`.
fn resolve_persona(
    claims: &JwtClaims,
    requested: Option<&str>,
    runtime: &CapabilitiesRuntime,
) -> Result<String, ApiError> {
    if let Some(p) = requested {
        claims.require_persona(p)?;
        return Ok(p.to_string());
    }
    let active = runtime
        .memory()
        .active_persona_id()
        .map_err(|e| ApiError::InternalError(format!("memory: {e}")))?;
    if let Some(a) = active {
        if claims.allows_persona(&a) {
            return Ok(a);
        }
    }
    match claims.personas.as_slice() {
        [only] => Ok(only.clone()),
        _ => Err(ApiError::PersonaForbidden {
            persona: "(aucune persona autorisée résolue)".into(),
        }),
    }
}

/// Vérifie que la `PendingConfirmation` `id` existe et appartient à une
/// persona autorisée pour le client (404 / 403 sinon).
async fn require_pending_owned(
    claims: &JwtClaims,
    runtime: &CapabilitiesRuntime,
    id: Uuid,
) -> Result<(), ApiError> {
    let pc = runtime
        .pending_confirmation(id)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("confirmation en attente {id}")))?;
    claims.require_persona(&pc.persona)
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

/// Body POST `/capabilities/dispatch` et `/capabilities/dispatch_and_queue`.
#[derive(Debug, Deserialize)]
pub struct DispatchBody {
    /// Identifiant qualifié de la capability (`service_id.intent`).
    pub qualified_id: String,
    /// Persona demandée (optionnelle). Elle n'est qu'une **sélection** parmi
    /// les personas autorisées par le token (R-002) : toute valeur hors de
    /// cette liste est refusée (403). Absente : persona active serveur.
    #[serde(default)]
    pub persona: Option<String>,
    /// Phrase utilisateur pour audit.
    pub user_phrase: String,
    /// Paramètres JSON (objet ou null).
    #[serde(default)]
    pub params: Value,
}

/// Body POST `/capabilities/dispatch/confirm` et `/capabilities/dispatch/cancel`.
///
/// R-016 : seule la référence d'une `PendingConfirmation` **stockée** est
/// acceptée. `qualified_id` / `params` / `persona` fournis par le client sont
/// rejetés (champ inconnu → 422) : seul l'enregistrement stocké est exécuté.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PendingRefBody {
    /// Identifiant de la `PendingConfirmation` (renvoyé par `/dispatch`).
    pub id: Uuid,
}

/// Représentation JSON d'un résultat d'exécution.
fn executed_to_json(r: &miyualicia_dispatcher::ExecutionResult) -> Value {
    serde_json::json!({
        "kind": "executed",
        "qualified_id": r.qualified_id,
        "service_result": {
            "success": r.service_result.success,
            "data":    r.service_result.data,
            "message": r.service_result.message,
        },
    })
}

/// POST `/capabilities/dispatch` — exécute une capability, ou **enregistre**
/// la `PendingConfirmation` (R-016) et renvoie son `id` : la confirmation ne
/// peut ensuite se faire que par cet `id`.
pub async fn dispatch(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Json(body): Json<DispatchBody>,
) -> Result<Json<Value>, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Write)?;
    let runtime = capabilities_or_503(&state).await?;
    let persona = resolve_persona(&claims, body.persona.as_deref(), &runtime)?;
    let outcome = runtime
        .dispatch_and_queue(&body.qualified_id, &body.params, &persona, &body.user_phrase)
        .await?;
    Ok(Json(match outcome {
        QueuedDispatch::Executed(r) => executed_to_json(&r),
        QueuedDispatch::Queued { id, qualified_id, prompt } => serde_json::json!({
            "kind": "pending_confirmation",
            "id": id.to_string(),
            "qualified_id": qualified_id,
            "prompt": prompt,
        }),
    }))
}

/// POST `/capabilities/dispatch/confirm` — body `{ "id": <uuid> }` uniquement.
/// Exécute l'enregistrement stocké (réclamation atomique, R-017).
pub async fn dispatch_confirm(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Json(body): Json<PendingRefBody>,
) -> Result<Json<Value>, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Write)?;
    let runtime = capabilities_or_503(&state).await?;
    require_pending_owned(&claims, &runtime, body.id).await?;
    let result = runtime.confirm_queued(body.id).await?;
    Ok(Json(executed_to_json(&result)))
}

/// POST `/capabilities/dispatch/cancel` — body `{ "id": <uuid> }` uniquement.
pub async fn dispatch_cancel(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Json(body): Json<PendingRefBody>,
) -> Result<StatusCode, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Write)?;
    let runtime = capabilities_or_503(&state).await?;
    require_pending_owned(&claims, &runtime, body.id).await?;
    runtime.cancel_queued(body.id).await?;
    Ok(StatusCode::NO_CONTENT)
}

// ----------------------------------------------------------------------
// Apprentissage — pending / accept / reject / detect
// ----------------------------------------------------------------------

/// Vérifie qu'une skill apprise existe et appartient à une persona autorisée.
fn require_learned_skill_owned(
    claims: &JwtClaims,
    runtime: &CapabilitiesRuntime,
    id: Uuid,
) -> Result<(), ApiError> {
    let skill = runtime
        .memory()
        .learned_skill(id)
        .map_err(|e| ApiError::InternalError(format!("memory: {e}")))?
        .ok_or_else(|| ApiError::NotFound(format!("skill apprise {id}")))?;
    claims.require_persona(&skill.persona)
}

/// GET `/capabilities/learnings/pending/{persona}` — skills proposées (persona autorisée).
pub async fn learnings_pending(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Path(persona): Path<String>,
) -> Result<Json<Value>, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Read)?;
    claims.require_persona(&persona)?;
    let runtime = capabilities_or_503(&state).await?;
    let pending = runtime
        .memory()
        .pending_learned_skills(&persona)
        .map_err(|e| ApiError::InternalError(format!("memory: {e}")))?;
    Ok(Json(serde_json::json!({ "pending": pending })))
}

/// GET `/capabilities/learnings/active/{persona}` — skills actives (persona autorisée).
pub async fn learnings_active(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Path(persona): Path<String>,
) -> Result<Json<Value>, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Read)?;
    claims.require_persona(&persona)?;
    let runtime = capabilities_or_503(&state).await?;
    let active = runtime
        .memory()
        .active_learned_skills(&persona)
        .map_err(|e| ApiError::InternalError(format!("memory: {e}")))?;
    Ok(Json(serde_json::json!({ "active": active })))
}

/// POST `/capabilities/learnings/{id}/accept` — accepte une skill d'une persona autorisée.
pub async fn learnings_accept(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Write)?;
    let runtime = capabilities_or_503(&state).await?;
    require_learned_skill_owned(&claims, &runtime, id)?;
    runtime
        .memory()
        .accept_learned_skill(id)
        .map_err(|e| ApiError::InternalError(format!("memory: {e}")))?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST `/capabilities/learnings/{id}/reject` — rejette une skill d'une persona autorisée.
pub async fn learnings_reject(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Write)?;
    let runtime = capabilities_or_503(&state).await?;
    require_learned_skill_owned(&claims, &runtime, id)?;
    runtime
        .memory()
        .reject_learned_skill(id)
        .map_err(|e| ApiError::InternalError(format!("memory: {e}")))?;
    Ok(StatusCode::NO_CONTENT)
}

/// POST `/capabilities/learnings/detect/{persona}` — détection de patterns (persona autorisée).
pub async fn learnings_detect(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Path(persona): Path<String>,
) -> Result<Json<Value>, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Write)?;
    claims.require_persona(&persona)?;
    let runtime = capabilities_or_503(&state).await?;
    let patterns = runtime.detect_patterns(&persona).await?;
    Ok(Json(serde_json::json!({ "patterns": patterns })))
}

// ----------------------------------------------------------------------
// Persona active
// ----------------------------------------------------------------------

/// GET `/capabilities/persona/active` — persona active côté serveur.
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

/// POST `/capabilities/persona/{id}/activate` — active une persona autorisée pour le client.
pub async fn persona_activate(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Write)?;
    // Un client ne peut activer qu'une persona qui lui est autorisée (R-002).
    claims.require_persona(&id)?;
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

/// POST `/capabilities/manifest/reload` — rechargement manuel des manifestes (admin).
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
            "errors":  report.errors.iter().map(ToString::to_string).collect::<Vec<_>>(),
        }))),
    }
}

// ----------------------------------------------------------------------
// Episodes récents (lecture mémoire)
// ----------------------------------------------------------------------

/// Paramètres de `GET /capabilities/episodes/recent`.
#[derive(Debug, Deserialize)]
pub struct EpisodesQuery {
    /// Limite (default 20).
    #[serde(default)]
    pub limit: Option<usize>,
}

/// GET `/capabilities/episodes/recent` — derniers épisodes d'audit.
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
    let persona = resolve_persona(&claims, body.persona.as_deref(), &runtime)?;
    let outcome = runtime
        .dispatch_and_queue(&body.qualified_id, &body.params, &persona, &body.user_phrase)
        .await?;
    let body = match outcome {
        QueuedDispatch::Executed(r) => executed_to_json(&r),
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

/// GET `/capabilities/pending_confirmations` — pending des personas autorisées.
pub async fn pending_list(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Query(q): Query<PendingFilterQuery>,
) -> Result<Json<Value>, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Read)?;
    let runtime = capabilities_or_503(&state).await?;
    // Seules les pending des personas autorisées pour le client sont visibles.
    let pending = match q.persona.as_deref() {
        Some(p) => {
            claims.require_persona(p)?;
            runtime.pending_confirmations_for(p).await?
        }
        None => runtime
            .list_pending_confirmations()
            .await?
            .into_iter()
            .filter(|pc| claims.allows_persona(&pc.persona))
            .collect(),
    };
    Ok(Json(serde_json::json!({
        "pending": pending,
        "count":   pending.len(),
    })))
}

/// POST `/capabilities/pending_confirmations/{id}/confirm` — exécute l'enregistrement stocké.
pub async fn pending_confirm(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Write)?;
    let runtime = capabilities_or_503(&state).await?;
    require_pending_owned(&claims, &runtime, id).await?;
    let result = runtime.confirm_queued(id).await?;
    Ok(Json(executed_to_json(&result)))
}

/// POST `/capabilities/pending_confirmations/{id}/cancel` — annule l'enregistrement stocké.
pub async fn pending_cancel(
    JwtAuth(claims): JwtAuth,
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    // Controle de scope fail-closed (audit C9).
    claims.require_scope(&JwtScope::Write)?;
    let runtime = capabilities_or_503(&state).await?;
    require_pending_owned(&claims, &runtime, id).await?;
    runtime.cancel_queued(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
