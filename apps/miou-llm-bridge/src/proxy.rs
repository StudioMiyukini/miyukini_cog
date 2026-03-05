//! Routeur HTTP — proxy passthrough + API agents/skills/contextes.

use axum::{
    body::Body,
    extract::{ConnectInfo, Path, State},
    http::{header, HeaderMap, Method, Request, StatusCode, Uri},
    middleware::Next,
    response::{Html, IntoResponse},
    routing::{any, delete, get, post, put},
    Json, Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::agents::custom::{CreateAgentRequest, UpdateAgentRequest};
use crate::agents::AgentRegistry;
use crate::config::SecurityConfig;
use crate::context::{ContextRegistry, CreateContextRequest, UpdateContextRequest};
use crate::fallback;
use crate::hardware::HardwareInfo;
use crate::inference::InferenceRouter;
use crate::model_manager::ModelManager;
use crate::security::{AuditLog, RateLimiter, SecurityAuditLog};
use crate::skills::{SkillExecRequest, SkillRegistry};

/// État partagé entre les handlers.
#[derive(Clone)]
pub struct ProxyState {
    pub client: reqwest::Client,
    pub upstream_url: String,
    pub auth_token: Option<String>,
    pub log_requests: bool,
    pub hardware: HardwareInfo,
    pub agent_registry: AgentRegistry,
    pub context_registry: ContextRegistry,
    pub rate_limiter: RateLimiter,
    pub audit_log: AuditLog,
    pub inference: InferenceRouter,
    pub model_manager: ModelManager,
    pub security_config: SecurityConfig,
    pub security_audit: SecurityAuditLog,
}

/// Construit le routeur axum avec toutes les routes.
pub fn proxy_router(state: ProxyState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Routes publiques (health + UI — pas de vérification de sécurité)
    let public_routes = Router::new()
        .route("/", get(ui_handler))
        .route("/health", get(health_handler));

    // Routes protégées (sécurité : origin + auth + HMAC)
    let protected_routes = Router::new()
        // ── Status ────────────────────────────────────────────────
        .route("/status", get(status_handler))
        // ── Hardware & Recommandation ────────────────────────────
        .route("/v1/hardware", get(crate::recommend::hardware_handler))
        .route("/v1/recommend", get(crate::recommend::recommend_handler))
        // ── Agents CRUD ──────────────────────────────────────────
        .route("/v1/agents", get(list_agents).post(create_agent))
        .route(
            "/v1/agents/{id}",
            get(get_agent).put(update_agent).delete(delete_agent),
        )
        .route("/v1/agents/{id}/chat", post(chat_with_agent))
        // ── Skills ───────────────────────────────────────────────
        .route("/v1/skills", get(list_skills))
        .route("/v1/skills/{id}/execute", post(execute_skill))
        // ── Contextes CRUD ───────────────────────────────────────
        .route("/v1/contexts", get(list_contexts).post(create_context))
        .route(
            "/v1/contexts/{id}",
            get(get_context).put(update_context).delete(delete_context),
        )
        // ── Équipes ──────────────────────────────────────────────
        .route("/v1/teams", get(list_teams).post(create_team))
        .route("/v1/teams/{id}/task", post(team_task))
        // ── Fallback / Disponibilité ─────────────────────────────
        .route("/v1/availability", get(availability_handler))
        // ── Modèles locaux GGUF ──────────────────────────────────
        .route("/v1/models/local", get(list_local_models))
        .route("/v1/models/load", post(load_model))
        .route("/v1/models/unload", post(unload_model))
        .route("/v1/inference/status", get(inference_status))
        // ── API LLM partagée (pour les services COG) ───────────────
        .route("/v1/llm/chat", post(crate::llm_api::llm_chat))
        .route("/v1/llm/complete", post(crate::llm_api::llm_complete))
        .route("/v1/llm/models", get(crate::llm_api::llm_models))
        .route("/v1/llm/status", get(crate::llm_api::llm_status))
        // ── Sécurité / Audit ──────────────────────────────────────
        .route("/v1/security/audit", get(security_audit_handler))
        .route("/v1/security/status", get(security_status_handler))
        // ── Catch-all proxy passthrough vers LM Studio (backup) ──
        .route("/v1/{*path}", any(proxy_handler))
        // ── Middleware de sécurité (origin + auth + HMAC) ────────
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            security_middleware,
        ));

    public_routes
        .merge(protected_routes)
        .layer(cors)
        .with_state(state)
}

// ═══════════════════════════════════════════════════════════════════════
// UI
// ═══════════════════════════════════════════════════════════════════════

async fn ui_handler() -> impl IntoResponse {
    Html(include_str!("../static/index.html"))
}

// ═══════════════════════════════════════════════════════════════════════
// Health & Status
// ═══════════════════════════════════════════════════════════════════════

async fn health_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "ok",
        "service": "miyukini-ai-studio",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

async fn status_handler(State(state): State<ProxyState>) -> impl IntoResponse {
    let native_loaded = state.inference.native.is_loaded();
    let upstream_ok = state.inference.upstream.is_available().await;

    let agents = state.agent_registry.list_agents();
    let skills = SkillRegistry::list_skills();
    let local_models = state.model_manager.scan_models();

    Json(serde_json::json!({
        "service": "miyukini-ai-studio",
        "status": "ok",
        "native_model": state.inference.native.loaded_model_name(),
        "native_loaded": native_loaded,
        "upstream": if upstream_ok { "connected" } else { "unreachable" },
        "upstream_url": state.upstream_url,
        "hardware_tier": state.hardware.tier.label(),
        "agents_count": agents.len(),
        "skills_count": skills.len(),
        "local_models_count": local_models.len(),
    }))
}

// ═══════════════════════════════════════════════════════════════════════
// Agents CRUD
// ═══════════════════════════════════════════════════════════════════════

async fn list_agents(State(state): State<ProxyState>) -> impl IntoResponse {
    let agents = state.agent_registry.list_agents();
    Json(serde_json::json!({ "agents": agents, "count": agents.len() }))
}

async fn get_agent(State(state): State<ProxyState>, Path(id): Path<String>) -> impl IntoResponse {
    match state.agent_registry.get_agent(&id) {
        Some(agent) => Json(serde_json::json!({
            "agent": {
                "id": agent.id,
                "display_name": agent.display_name,
                "role": agent.role,
                "description": agent.description,
                "system_prompt": agent.system_prompt,
                "skill_ids": agent.skill_ids,
                "model_preference": agent.model_preference,
                "security_level": agent.security_level,
                "is_builtin": agent.is_builtin,
                "icon": agent.icon,
                "can_dispatch": agent.can_dispatch,
            }
        }))
        .into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("Agent '{id}' introuvable") })),
        )
            .into_response(),
    }
}

async fn create_agent(
    State(state): State<ProxyState>,
    Json(req): Json<CreateAgentRequest>,
) -> impl IntoResponse {
    let agent = req.into_agent_def();
    let id = agent.id.clone();
    state.agent_registry.upsert_agent(agent);
    (
        StatusCode::CREATED,
        Json(serde_json::json!({ "created": id })),
    )
}

async fn update_agent(
    State(state): State<ProxyState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateAgentRequest>,
) -> impl IntoResponse {
    match state.agent_registry.get_agent(&id) {
        Some(mut agent) => {
            req.apply_to(&mut agent);
            state.agent_registry.upsert_agent(agent);
            Json(serde_json::json!({ "updated": id })).into_response()
        }
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("Agent '{id}' introuvable") })),
        )
            .into_response(),
    }
}

async fn delete_agent(
    State(state): State<ProxyState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match state.agent_registry.delete_agent(&id) {
        Ok(()) => Json(serde_json::json!({ "deleted": id })).into_response(),
        Err(msg) => (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({ "error": msg })),
        )
            .into_response(),
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Chat avec un agent
// ═══════════════════════════════════════════════════════════════════════

#[derive(serde::Deserialize)]
struct AgentChatRequest {
    message: String,
    #[serde(default)]
    context_ids: Vec<String>,
    #[serde(default)]
    model: Option<String>,
    /// Active le tool calling si le modèle le supporte (défaut: true).
    #[serde(default = "default_true")]
    enable_tools: bool,
}

fn default_true() -> bool {
    true
}

async fn chat_with_agent(
    State(state): State<ProxyState>,
    Path(id): Path<String>,
    Json(req): Json<AgentChatRequest>,
) -> impl IntoResponse {
    let agent = match state.agent_registry.get_agent(&id) {
        Some(a) => a,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": format!("Agent '{id}' introuvable") })),
            )
                .into_response();
        }
    };

    // Vérifier la disponibilité LLM (native → upstream → fallback)
    let availability = fallback::probe_availability(
        &state.client,
        &state.upstream_url,
        state.inference.native.is_loaded(),
    )
    .await;

    if availability.is_degraded() {
        let fb = fallback::fallback_respond(&req.message);
        return Json(serde_json::json!({
            "agent": agent.display_name,
            "role": agent.role,
            "icon": agent.icon,
            "content": if fb.answered { fb.content } else { fb.refusal_reason.unwrap_or_default() },
            "answered": fb.answered,
            "availability": fb.availability,
            "fallback": true,
        }))
        .into_response();
    }

    // Construire le contexte enrichi
    let context_text = if !req.context_ids.is_empty() {
        state
            .context_registry
            .compile_context(&req.context_ids, 4096)
    } else {
        String::new()
    };

    let mut system_prompt = agent.system_prompt.clone();
    if !context_text.is_empty() {
        system_prompt.push_str("\n\n# Contexte additionnel\n");
        system_prompt.push_str(&context_text);
    }

    // Instructions de grounding anti-hallucination pour les petits modèles locaux
    system_prompt.push_str(concat!(
        "\n\n# Règles ABSOLUES\n",
        "- Tu es un assistant IA. L'utilisateur est un humain qui te pose des questions.\n",
        "- Ne te fais JAMAIS passer pour l'utilisateur. Tu réponds TOUJOURS en tant qu'assistant.\n",
        "- Si tu ne connais pas la réponse, dis-le honnêtement. N'invente JAMAIS d'informations.\n",
        "- Ne fabrique pas de détails sur des produits, entreprises, ou technologies que tu ne connais pas.\n",
        "- Réponds de manière concise et utile. Reste dans ton rôle d'assistant.",
    ));

    let messages = vec![
        serde_json::json!({ "role": "system", "content": system_prompt }),
        serde_json::json!({ "role": "user", "content": req.message }),
    ];

    let model = req.model.unwrap_or_else(|| "default".into());

    // Tool calling : construire les tools si activé et si l'agent a des skills
    let tools = if req.enable_tools {
        crate::tools::agent_tools(&agent)
    } else {
        Vec::new()
    };

    if tools.is_empty() {
        // Pas de tools — flux classique via InferenceRouter
        // Température basse pour réduire l'hallucination des petits modèles
        let chat_body = serde_json::json!({
            "model": model,
            "messages": messages,
            "temperature": 0.4,
            "max_tokens": 2048,
        });

        match state.inference.chat_completion(chat_body).await {
            Ok(resp) => {
                let content = resp
                    .choices
                    .first()
                    .and_then(|c| c.message.get("content"))
                    .and_then(|c| c.as_str())
                    .unwrap_or("Pas de réponse");

                Json(serde_json::json!({
                    "agent": agent.display_name,
                    "agent_id": agent.id,
                    "role": agent.role,
                    "icon": agent.icon,
                    "content": content,
                    "model": resp.model,
                    "backend": resp.backend,
                    "availability": availability,
                    "fallback": false,
                    "tool_calls": [],
                    "iterations": 1,
                }))
                .into_response()
            }
            Err(e) => {
                let fb = fallback::fallback_respond(&req.message);
                Json(serde_json::json!({
                    "agent": agent.display_name,
                    "icon": agent.icon,
                    "content": if fb.answered { fb.content } else { fb.refusal_reason.unwrap_or_default() },
                    "answered": fb.answered,
                    "availability": "fallback",
                    "fallback": true,
                    "error": format!("{e}"),
                }))
                .into_response()
            }
        }
    } else {
        // Flux avec tool calling via InferenceRouter
        match crate::tools::chat_with_tools(
            &state.inference,
            &model,
            messages,
            tools,
            &agent,
            &state.audit_log,
        )
        .await
        {
            Ok(result) => Json(serde_json::json!({
                "agent": agent.display_name,
                "agent_id": agent.id,
                "role": agent.role,
                "icon": agent.icon,
                "content": result.content,
                "model": model,
                "availability": availability,
                "fallback": false,
                "tool_calls": result.tool_calls_log,
                "iterations": result.iterations,
            }))
            .into_response(),

            Err(crate::tools::ToolChatError::MaxIterationsExceeded) => Json(serde_json::json!({
                "agent": agent.display_name,
                "agent_id": agent.id,
                "icon": agent.icon,
                "content": "L'agent a atteint la limite d'itérations d'outils (5). \
                           La tâche est peut-être trop complexe pour une seule interaction.",
                "error": "max_tool_iterations",
                "availability": availability,
                "fallback": false,
            }))
            .into_response(),

            Err(crate::tools::ToolChatError::UpstreamError(msg)) => {
                let fb = fallback::fallback_respond(&req.message);
                Json(serde_json::json!({
                    "agent": agent.display_name,
                    "icon": agent.icon,
                    "content": if fb.answered { fb.content } else { fb.refusal_reason.unwrap_or_default() },
                    "answered": fb.answered,
                    "availability": "fallback",
                    "fallback": true,
                    "upstream_error": msg,
                }))
                .into_response()
            }

            Err(crate::tools::ToolChatError::ParseError(msg)) => (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({ "error": format!("Erreur parsing LLM : {msg}") })),
            )
                .into_response(),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Skills
// ═══════════════════════════════════════════════════════════════════════

async fn list_skills() -> impl IntoResponse {
    let skills = SkillRegistry::list_skills();
    Json(serde_json::json!({ "skills": skills, "count": skills.len() }))
}

async fn execute_skill(
    State(state): State<ProxyState>,
    Path(id): Path<String>,
    Json(req): Json<SkillExecRequest>,
) -> impl IntoResponse {
    if let Some(ref agent_id) = req.agent_id {
        if let Some(agent) = state.agent_registry.get_agent(agent_id) {
            match crate::security::check_agent_skill_permission(&agent, &id) {
                crate::security::PermissionCheck::Allowed => {}
                crate::security::PermissionCheck::Denied(reason) => {
                    return (
                        StatusCode::FORBIDDEN,
                        Json(serde_json::json!({ "error": reason })),
                    )
                        .into_response();
                }
            }
        }
    }

    let result = SkillRegistry::execute(&id, &req.params).await;

    state.audit_log.log(crate::security::AuditEntry {
        timestamp: crate::security::epoch_now(),
        agent_id: req.agent_id.unwrap_or_else(|| "anonymous".into()),
        skill_id: id.clone(),
        action: req
            .params
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("execute")
            .into(),
        details: serde_json::to_string(&req.params).unwrap_or_default(),
        success: result.success,
        client_id: String::new(),
    });

    Json(serde_json::json!({ "result": result })).into_response()
}

// ═══════════════════════════════════════════════════════════════════════
// Contextes CRUD
// ═══════════════════════════════════════════════════════════════════════

async fn list_contexts(State(state): State<ProxyState>) -> impl IntoResponse {
    let bases = state.context_registry.list_bases();
    Json(serde_json::json!({ "contexts": bases, "count": bases.len() }))
}

async fn get_context(State(state): State<ProxyState>, Path(id): Path<String>) -> impl IntoResponse {
    match state.context_registry.get_base(&id) {
        Some(base) => Json(serde_json::json!({ "context": base })).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("Contexte '{id}' introuvable") })),
        )
            .into_response(),
    }
}

async fn create_context(
    State(state): State<ProxyState>,
    Json(req): Json<CreateContextRequest>,
) -> impl IntoResponse {
    let base = state.context_registry.create_base(req);
    (
        StatusCode::CREATED,
        Json(serde_json::json!({ "created": base.id, "context": base })),
    )
}

async fn update_context(
    State(state): State<ProxyState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateContextRequest>,
) -> impl IntoResponse {
    match state.context_registry.update_base(&id, req) {
        Some(base) => Json(serde_json::json!({ "updated": id, "context": base })).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("Contexte '{id}' introuvable") })),
        )
            .into_response(),
    }
}

async fn delete_context(
    State(state): State<ProxyState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if state.context_registry.delete_base(&id) {
        Json(serde_json::json!({ "deleted": id })).into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("Contexte '{id}' introuvable") })),
        )
            .into_response()
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Équipes
// ═══════════════════════════════════════════════════════════════════════

async fn list_teams(State(state): State<ProxyState>) -> impl IntoResponse {
    let teams = state.agent_registry.list_teams();
    Json(serde_json::json!({ "teams": teams, "count": teams.len() }))
}

async fn create_team(
    State(state): State<ProxyState>,
    Json(team): Json<crate::agents::AgentTeam>,
) -> impl IntoResponse {
    let id = team.id.clone();
    state.agent_registry.create_team(team);
    (
        StatusCode::CREATED,
        Json(serde_json::json!({ "created": id })),
    )
}

#[derive(serde::Deserialize)]
struct TeamTaskRequest {
    description: String,
    #[serde(default)]
    context_ids: Vec<String>,
    #[serde(default)]
    target_agent_id: Option<String>,
}

async fn team_task(
    State(state): State<ProxyState>,
    Path(id): Path<String>,
    Json(req): Json<TeamTaskRequest>,
) -> impl IntoResponse {
    let team = match state.agent_registry.get_team(&id) {
        Some(t) => t,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": format!("Équipe '{id}' introuvable") })),
            )
                .into_response();
        }
    };

    let target_id = req
        .target_agent_id
        .unwrap_or_else(|| team.coordinator_id.clone());

    Json(serde_json::json!({
        "team": team.name,
        "dispatched_to": target_id,
        "task": req.description,
        "status": "dispatched",
    }))
    .into_response()
}

// ═══════════════════════════════════════════════════════════════════════
// Disponibilité / Fallback
// ═══════════════════════════════════════════════════════════════════════

async fn availability_handler(State(state): State<ProxyState>) -> impl IntoResponse {
    let availability = fallback::probe_availability(
        &state.client,
        &state.upstream_url,
        state.inference.native.is_loaded(),
    )
    .await;
    Json(serde_json::json!({
        "availability": availability,
        "label": availability.label(),
        "degraded": availability.is_degraded(),
        "has_llm": availability.has_llm(),
    }))
}

// ═══════════════════════════════════════════════════════════════════════
// Modèles locaux GGUF
// ═══════════════════════════════════════════════════════════════════════

async fn list_local_models(State(state): State<ProxyState>) -> impl IntoResponse {
    let models = state.model_manager.scan_models();
    let loaded = state.inference.native.loaded_model_name();
    Json(serde_json::json!({
        "models": models,
        "count": models.len(),
        "loaded_model": loaded,
        "models_dir": state.model_manager.models_dir().to_string_lossy(),
    }))
}

#[derive(serde::Deserialize)]
struct LoadModelRequest {
    /// Nom du fichier GGUF ou recherche partielle.
    filename: String,
}

async fn load_model(
    State(state): State<ProxyState>,
    Json(req): Json<LoadModelRequest>,
) -> impl IntoResponse {
    let model = match state.model_manager.find_model(&req.filename) {
        Some(m) => m,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": format!("Modèle '{}' introuvable dans {}", req.filename, state.model_manager.models_dir().display()),
                    "hint": "Utilisez GET /v1/models/local pour voir les modèles disponibles",
                })),
            )
                .into_response();
        }
    };

    let path = model.path.clone();
    let display = model.display_name.clone();

    match state.inference.native.load_model(path) {
        Ok(name) => Json(serde_json::json!({
            "loaded": name,
            "display_name": display,
            "size": model.size_display,
        }))
        .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": format!("{e}") })),
        )
            .into_response(),
    }
}

async fn unload_model(State(state): State<ProxyState>) -> impl IntoResponse {
    let name = state.inference.native.loaded_model_name();
    if state.inference.native.unload_model() {
        Json(serde_json::json!({ "unloaded": name }))
    } else {
        Json(serde_json::json!({ "message": "Aucun modèle chargé" }))
    }
}

async fn inference_status(State(state): State<ProxyState>) -> impl IntoResponse {
    let native_loaded = state.inference.native.is_loaded();
    let native_model = state.inference.native.loaded_model_name();
    let upstream_available = state.inference.upstream.is_available().await;
    let active = state.inference.active_backend().await;
    let local_models = state.model_manager.scan_models();

    let availability =
        fallback::probe_availability(&state.client, &state.upstream_url, native_loaded).await;

    Json(serde_json::json!({
        "availability": availability,
        "availability_label": availability.label(),
        "native": {
            "loaded": native_loaded,
            "model": native_model,
            "local_models_count": local_models.len(),
        },
        "upstream": {
            "available": upstream_available,
            "url": state.inference.upstream.upstream_url(),
        },
        "active_backend": active,
        "hardware_tier": state.hardware.tier.label(),
    }))
}

// ═══════════════════════════════════════════════════════════════════════
// Middleware de sécurité (origin + auth globale + HMAC)
// ═══════════════════════════════════════════════════════════════════════

/// Middleware de sécurité appliqué sur toutes les routes protégées.
///
/// Vérifie dans l'ordre :
/// 1. Validation d'origine (IP source dans allowed_origins ou loopback)
/// 2. Auth Bearer globale (si require_auth_all est activé)
/// 3. Signature HMAC (si le header X-COG-Signature est présent)
async fn security_middleware(
    State(state): State<ProxyState>,
    req: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    let headers = req.headers().clone();
    let path = req.uri().path().to_string();
    let method = req.method().to_string();

    // Extraire l'IP source (ConnectInfo n'est pas toujours disponible)
    let source_ip = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .map(|v| v.split(',').next().unwrap_or(v).trim().to_string())
        .or_else(|| {
            headers
                .get("x-real-ip")
                .and_then(|v| v.to_str().ok())
                .map(String::from)
        })
        .unwrap_or_else(|| "127.0.0.1".into());

    // 1. Validation d'origine
    match crate::security::check_origin(Some(&source_ip), &headers, &state.security_config) {
        crate::security::OriginCheck::Allowed => {}
        crate::security::OriginCheck::Blocked(reason) => {
            state
                .security_audit
                .log_event(crate::security::SecurityEvent {
                    timestamp: crate::security::epoch_now(),
                    kind: crate::security::SecurityEventKind::OriginBlocked,
                    source_ip: source_ip.clone(),
                    path: path.clone(),
                    details: reason.clone(),
                });
            return (
                StatusCode::FORBIDDEN,
                Json(serde_json::json!({
                    "error": "Accès refusé",
                    "reason": "origin_blocked",
                })),
            )
                .into_response();
        }
    }

    // 2. Auth Bearer globale (si activée)
    if state.security_config.require_auth_all {
        if let Err(msg) = crate::security::verify_auth_token(&headers, &state.auth_token) {
            state
                .security_audit
                .log_event(crate::security::SecurityEvent {
                    timestamp: crate::security::epoch_now(),
                    kind: crate::security::SecurityEventKind::AuthFailed,
                    source_ip: source_ip.clone(),
                    path: path.clone(),
                    details: msg.to_string(),
                });
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": msg,
                    "reason": "auth_required",
                })),
            )
                .into_response();
        }
    }

    // 3. Vérification HMAC (si le header X-COG-Signature est présent)
    if headers.get(crate::security::HEADER_COG_SIGNATURE).is_some() {
        // Pour la vérification HMAC, on a besoin du body — on ne peut pas le lire ici
        // sans consommer la requête. On vérifie seulement le timestamp pour l'anti-replay.
        // La vérification complète du body est faite dans les handlers qui en ont besoin.
        if let Some(ts_str) = headers
            .get(crate::security::HEADER_COG_TIMESTAMP)
            .and_then(|v| v.to_str().ok())
        {
            if let Ok(timestamp) = ts_str.parse::<u64>() {
                let now = crate::security::epoch_now();
                let age = if now > timestamp {
                    now - timestamp
                } else {
                    timestamp - now
                };

                if age > state.security_config.hmac_max_age_secs {
                    state
                        .security_audit
                        .log_event(crate::security::SecurityEvent {
                            timestamp: now,
                            kind: crate::security::SecurityEventKind::HmacReplay,
                            source_ip: source_ip.clone(),
                            path: path.clone(),
                            details: format!(
                                "Timestamp expiré (age: {age}s, max: {}s)",
                                state.security_config.hmac_max_age_secs
                            ),
                        });
                    return (
                        StatusCode::FORBIDDEN,
                        Json(serde_json::json!({
                            "error": "Requête expirée (anti-replay)",
                            "reason": "hmac_replay",
                        })),
                    )
                        .into_response();
                }
            }
        }
    }

    // Passer au handler suivant
    next.run(req).await.into_response()
}

// ═══════════════════════════════════════════════════════════════════════
// Sécurité — Endpoints d'audit et status
// ═══════════════════════════════════════════════════════════════════════

/// GET /v1/security/audit — Journal des événements de sécurité.
async fn security_audit_handler(State(state): State<ProxyState>) -> impl IntoResponse {
    let events = state.security_audit.recent(100);
    let blocked_ips = state.security_audit.blocked_ips();

    Json(serde_json::json!({
        "events": events,
        "count": events.len(),
        "blocked_ips": blocked_ips,
        "blocked_ips_count": blocked_ips.len(),
    }))
}

/// GET /v1/security/status — État de la configuration de sécurité.
async fn security_status_handler(State(state): State<ProxyState>) -> impl IntoResponse {
    let cfg = &state.security_config;

    Json(serde_json::json!({
        "origin_validation": {
            "enabled": !cfg.allowed_origins.is_empty() || cfg.strict_user_agent,
            "allowed_origins": cfg.allowed_origins,
            "strict_user_agent": cfg.strict_user_agent,
        },
        "authentication": {
            "bearer_token_configured": state.auth_token.is_some(),
            "require_auth_all": cfg.require_auth_all,
        },
        "hmac_signing": {
            "enabled": cfg.hmac_secret.is_some(),
            "max_age_secs": cfg.hmac_max_age_secs,
        },
        "encryption": {
            "enabled": cfg.encryption_key.is_some(),
            "algorithm": "AES-256-GCM",
        },
        "recent_security_events": state.security_audit.recent(10).len(),
        "blocked_ips": state.security_audit.blocked_ips(),
    }))
}

// ═══════════════════════════════════════════════════════════════════════
// Proxy passthrough (catch-all vers LM Studio)
// ═══════════════════════════════════════════════════════════════════════

async fn proxy_handler(State(state): State<ProxyState>, req: Request<Body>) -> impl IntoResponse {
    if let Some(ref expected_token) = state.auth_token {
        let authorized = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .map(|v| v.strip_prefix("Bearer ").unwrap_or(v) == expected_token.as_str())
            .unwrap_or(false);

        if !authorized {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "error": "Unauthorized" })),
            )
                .into_response();
        }
    }

    let method = req.method().clone();
    let uri = req.uri().clone();
    let headers = req.headers().clone();
    let upstream_uri = build_upstream_uri(&state.upstream_url, &uri);

    if state.log_requests {
        tracing::info!("{method} {uri} → {upstream_uri}");
    }

    let body_bytes = match axum::body::to_bytes(req.into_body(), 10 * 1024 * 1024).await {
        Ok(b) => b,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({ "error": format!("Body read error: {e}") })),
            )
                .into_response();
        }
    };

    let mut upstream_req = state
        .client
        .request(to_reqwest_method(&method), &upstream_uri);

    for (name, value) in &headers {
        match name.as_str() {
            "host" | "connection" | "transfer-encoding" => continue,
            _ => {
                if let Ok(v) = value.to_str() {
                    upstream_req = upstream_req.header(name.as_str(), v);
                }
            }
        }
    }

    if !body_bytes.is_empty() {
        upstream_req = upstream_req.body(body_bytes);
    }

    match upstream_req.send().await {
        Ok(upstream_resp) => {
            let status = StatusCode::from_u16(upstream_resp.status().as_u16())
                .unwrap_or(StatusCode::BAD_GATEWAY);

            let mut response_headers = HeaderMap::new();
            for (name, value) in upstream_resp.headers() {
                if name.as_str() != "transfer-encoding" {
                    response_headers.insert(name.clone(), value.clone());
                }
            }

            match upstream_resp.bytes().await {
                Ok(body) => (status, response_headers, body).into_response(),
                Err(e) => (
                    StatusCode::BAD_GATEWAY,
                    Json(serde_json::json!({ "error": format!("Upstream body error: {e}") })),
                )
                    .into_response(),
            }
        }
        Err(e) => {
            tracing::error!("Upstream error: {e}");
            (
                StatusCode::BAD_GATEWAY,
                Json(serde_json::json!({
                    "error": format!("Upstream unreachable: {e}"),
                    "upstream_url": state.upstream_url,
                })),
            )
                .into_response()
        }
    }
}

fn build_upstream_uri(upstream_base: &str, uri: &Uri) -> String {
    let path = uri.path();
    let base = upstream_base.trim_end_matches('/');
    match uri.query() {
        Some(q) => format!("{base}{path}?{q}"),
        None => format!("{base}{path}"),
    }
}

fn to_reqwest_method(method: &Method) -> reqwest::Method {
    match *method {
        Method::GET => reqwest::Method::GET,
        Method::POST => reqwest::Method::POST,
        Method::PUT => reqwest::Method::PUT,
        Method::DELETE => reqwest::Method::DELETE,
        Method::PATCH => reqwest::Method::PATCH,
        Method::HEAD => reqwest::Method::HEAD,
        Method::OPTIONS => reqwest::Method::OPTIONS,
        _ => reqwest::Method::GET,
    }
}
