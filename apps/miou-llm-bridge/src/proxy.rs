//! @id     miou_llm_bridge_proxy
//! @do     definir_proxy_state
//! @role   orchestration
//! @layer  app
//! @human  miou-llm-bridge — module « proxy ».

//! Routeur HTTP — proxy passthrough + API agents/skills/contextes.

use axum::{
    body::Body,
    extract::{ConnectInfo, Path, State},
    http::{header, HeaderMap, HeaderValue, Method, Request, StatusCode, Uri},
    middleware::Next,
    response::{Html, IntoResponse},
    routing::{any, delete, get, post, put},
    Json, Router,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

use crate::agents::custom::{CreateAgentRequest, UpdateAgentRequest};
use crate::agents::AgentRegistry;
use crate::config::SecurityConfig;
use crate::context::{ContextRegistry, CreateContextRequest, UpdateContextRequest};
use crate::fallback;
use crate::hardware::HardwareInfo;
use crate::inference::InferenceRouter;
use crate::mode::MaiaMode;
use crate::model_manager::ModelManager;
use crate::persona::PersonaConfig;
use crate::security::{AuditLog, RateLimiter, SecurityAuditLog};
use crate::skills::{SkillExecRequest, SkillRegistry};

/// État partagé entre les handlers.
#[derive(Clone)]
pub struct ProxyState {
    pub client: reqwest::Client,
    pub upstream_url: String,
    /// Appelants épinglés (requêtes signées ; aucun secret partagé).
    pub gardes: std::sync::Arc<crate::signature::GardesMaia>,
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
    /// Persona active (None si aucun --profile passé).
    pub active_persona: Option<PersonaConfig>,
    /// Mode de fonctionnement (standalone / pro).
    pub maia_mode: MaiaMode,
    /// Timestamp de démarrage (epoch secondes).
    pub started_at: u64,
    /// Enregistré auprès du hub TEAM.
    pub team_registered: bool,
    /// Actions d'outils suspendues en attente de confirmation (K-02).
    pub confirmations: std::sync::Arc<crate::confirmations::RegistreConfirmations>,
    /// Porte StrongFather obligatoire (outils, administration des agents — K-11, K-9).
    pub gouvernance: std::sync::Arc<crate::gouvernance::GouvernanceMaia>,
}

/// Construit la couche CORS à partir d'une liste explicite d'origines.
/// Liste vide (défaut) => aucun en-tête CORS (pas d'accès cross-origin navigateur).
/// Le joker `*` et les valeurs invalides sont ignorés.
pub fn build_cors(origins: &[String]) -> CorsLayer {
    let values: Vec<HeaderValue> = origins
        .iter()
        .filter(|o| o.trim() != "*")
        .filter_map(|o| HeaderValue::from_str(o.trim()).ok())
        .collect();
    let layer = CorsLayer::new();
    if values.is_empty() {
        return layer;
    }
    layer
        .allow_origin(values)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            axum::http::HeaderName::from_static(miyauth::http::ENTETE_SIGNATAIRE),
            axum::http::HeaderName::from_static(miyauth::http::ENTETE_HORODATAGE),
            axum::http::HeaderName::from_static(miyauth::http::ENTETE_NONCE),
            axum::http::HeaderName::from_static(miyauth::http::ENTETE_SIGNATURE),
        ])
}

/// Construit le routeur axum avec toutes les routes.
///
/// Le serveur DOIT être lancé avec `into_make_service_with_connect_info::<SocketAddr>()`
/// (sinon le middleware de sécurité refuse toutes les requêtes protégées).
pub fn proxy_router(state: ProxyState) -> Router {
    let cors = build_cors(&state.security_config.cors_allowed_origins);

    // Routes publiques (health + UI — pas de vérification de sécurité)
    let public_routes = Router::new()
        .route("/", get(ui_handler))
        .route("/health", get(health_handler))
        // MAIA health endpoint — état complet pour TEAM et monitoring
        .route("/maia/health", get(crate::health::health_handler))
        // AI Act art. 50 : présentation publique de MAIA comme IA.
        .route("/maia/presentation", get(crate::mention_ia::presentation_handler));

    // Routes protégées (sécurité : origine IP + requête signée par un appelant épinglé)
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
        // ── Confirmations d'actions suspendues (K-02) ────────────
        .route("/v1/confirmations", get(list_confirmations))
        .route("/v1/confirmations/{id}", post(decide_confirmation))
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
        // ── Middleware de sécurité (origine + requête signée) ────
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

/// Page de l'UI intégrée. Elle est publique, mais les appels qu'elle fait à
/// l'API (`/v1/…`, `/status`) sont protégés : un navigateur ne détient pas de clé
/// épinglée, donc ils reçoivent 401 (ou 503 sans appelant épinglé). Central
/// n'affiche pas cette UI ; pour l'utiliser, la servir derrière un backend qui
/// signe les requêtes (usage `maia/api`) avec une clé épinglée dans
/// `[security] cles_appelants`.
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

/// Réponse 403 d'un refus StrongFather.
fn refus_gouvernance(motif: String) -> axum::response::Response {
    (
        StatusCode::FORBIDDEN,
        Json(serde_json::json!({ "error": motif })),
    )
        .into_response()
}

async fn create_agent(
    State(state): State<ProxyState>,
    Json(req): Json<CreateAgentRequest>,
) -> impl IntoResponse {
    let agent = req.into_agent_def();
    let id = agent.id.clone();
    // K-9 : création soumise à StrongFather (pas d'écrasement, pas d'agent au-delà de Sandboxed).
    let existant = state.agent_registry.get_agent(&id);
    let intent = crate::gouvernance::intention_agent(
        crate::gouvernance::OperationAgent::Creer,
        &agent,
        existant.as_ref(),
        crate::gouvernance::APPELANT_LOCAL,
    );
    let registre = &state.agent_registry;
    // Insertion conditionnelle : un agent apparu depuis la décision n'est pas écrasé.
    match state
        .gouvernance
        .executer(&intent, |_| registre.inserer_nouvel_agent(agent))
    {
        Ok(Ok(())) => (
            StatusCode::CREATED,
            Json(serde_json::json!({ "created": id })),
        )
            .into_response(),
        Ok(Err(msg)) => (
            StatusCode::CONFLICT,
            Json(serde_json::json!({ "error": msg })),
        )
            .into_response(),
        Err(motif) => refus_gouvernance(motif),
    }
}

async fn update_agent(
    State(state): State<ProxyState>,
    Path(id): Path<String>,
    Json(req): Json<UpdateAgentRequest>,
) -> impl IntoResponse {
    match state.agent_registry.get_agent(&id) {
        Some(existant) => {
            let mut agent = existant.clone();
            req.apply_to(&mut agent);
            // K-9 : built-in non réécrit, aucune élévation (niveau, skills, dispatch).
            let intent = crate::gouvernance::intention_agent(
                crate::gouvernance::OperationAgent::Modifier,
                &agent,
                Some(&existant),
                crate::gouvernance::APPELANT_LOCAL,
            );
            let registre = &state.agent_registry;
            // Remplacement conditionnel : la décision porte sur `existant`.
            match state.gouvernance.executer(&intent, |_| {
                registre.remplacer_si_inchange(&existant, agent)
            }) {
                Ok(Ok(())) => Json(serde_json::json!({ "updated": id })).into_response(),
                Ok(Err(msg)) => (
                    StatusCode::CONFLICT,
                    Json(serde_json::json!({ "error": msg })),
                )
                    .into_response(),
                Err(motif) => refus_gouvernance(motif),
            }
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
    let Some(existant) = state.agent_registry.get_agent(&id) else {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("Agent '{id}' introuvable") })),
        )
            .into_response();
    };
    // Suppression soumise à StrongFather, puis conditionnelle à l'état décidé.
    let intent = crate::gouvernance::intention_agent(
        crate::gouvernance::OperationAgent::Supprimer,
        &existant,
        Some(&existant),
        crate::gouvernance::APPELANT_LOCAL,
    );
    let registre = &state.agent_registry;
    match state
        .gouvernance
        .executer(&intent, |_| registre.supprimer_si_inchange(&existant))
    {
        Ok(Ok(())) => Json(serde_json::json!({ "deleted": id })).into_response(),
        Ok(Err(msg)) => (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({ "error": msg })),
        )
            .into_response(),
        Err(motif) => refus_gouvernance(motif),
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
    /// Joint la mémoire de l'utilisateur (lot 7) : opt-in, jamais pour le hub TEAM.
    #[serde(default)]
    memoire: bool,
}

fn default_true() -> bool {
    true
}

async fn chat_with_agent(
    State(state): State<ProxyState>,
    Path(id): Path<String>,
    usage: Option<axum::Extension<crate::signature::UsageAppelant>>,
    Json(req): Json<AgentChatRequest>,
) -> impl IntoResponse {
    let Some(agent) = state.agent_registry.get_agent(&id) else {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("Agent '{id}' introuvable") })),
        )
            .into_response();
    };

    // Vérifier la disponibilité LLM (native → upstream → fallback)
    let availability = fallback::probe_availability(&state.inference)
    .await;

    if availability.is_degraded() {
        let fb = fallback::fallback_respond(&req.message);
        return Json(crate::mention_ia::marquer(serde_json::json!({
            "agent": agent.display_name,
            "role": agent.role,
            "icon": agent.icon,
            "content": if fb.answered { fb.content } else { fb.refusal_reason.unwrap_or_default() },
            "answered": fb.answered,
            "availability": fb.availability,
            "fallback": true,
        }), false))
        .into_response();
    }

    // Construire le contexte enrichi
    let context_text = if req.context_ids.is_empty() {
        String::new()
    } else {
        state
            .context_registry
            .compile_context(&req.context_ids, 4096)
    };

    // Identité d'IA d'abord (AI Act art. 50 §1), puis la consigne de l'agent.
    let mut system_prompt = crate::mention_ia::consigne_systeme(&agent.display_name);
    system_prompt.push_str(&agent.system_prompt);
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

    // Mémoire de l'utilisateur (EX-50..56) : appelant API signé et demande explicite seulement.
    let avec_memoire = req.memoire
        && matches!(usage, Some(axum::Extension(u)) if u.0 == crate::signature::USAGE_API);
    let bloc = if avec_memoire {
        state.context_registry.bloc_memoire(&req.message).await
    } else {
        None
    };
    // Outil `memoire_noter` (A-009) : seulement si la mémoire est jointe à ce tour.
    let carnet = if avec_memoire {
        state.context_registry.carnet_memoire()
    } else {
        None
    };
    let messages = vec![
        serde_json::json!({ "role": "system", "content": system_prompt }),
        serde_json::json!({ "role": "user", "content": crate::memoire::avec_memoire(bloc, &req.message) }),
    ];

    let model = req.model.unwrap_or_else(|| "default".into());

    // Tool calling : construire les tools si activé et si l'agent a des skills
    let mut tools = if req.enable_tools {
        crate::tools::agent_tools(&agent)
    } else {
        Vec::new()
    };
    // Compétence `memoire` déclarée par l'agent (F6) : sinon l'outil n'est pas proposé.
    if req.enable_tools && carnet.is_some() && crate::outil_memoire::competence_memoire(&agent) {
        tools.push(crate::outil_memoire::definition_noter());
    }

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

                Json(crate::mention_ia::marquer(serde_json::json!({
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
                }), true))
                .into_response()
            }
            Err(e) => {
                let fb = fallback::fallback_respond(&req.message);
                Json(crate::mention_ia::marquer(serde_json::json!({
                    "agent": agent.display_name,
                    "icon": agent.icon,
                    "content": if fb.answered { fb.content } else { fb.refusal_reason.unwrap_or_default() },
                    "answered": fb.answered,
                    "availability": "fallback",
                    "fallback": true,
                    "error": format!("{e}"),
                }), false))
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
            &state.confirmations,
            &state.gouvernance,
            carnet,
        )
        .await
        {
            Ok(result) => Json(crate::mention_ia::marquer(serde_json::json!({
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
                "foreign_content": result.contenu_etranger,
                "pending_confirmations": result.confirmations_en_attente,
            }), true))
            .into_response(),

            Err(crate::tools::ToolChatError::MaxIterationsExceeded) => Json(crate::mention_ia::marquer(serde_json::json!({
                "agent": agent.display_name,
                "agent_id": agent.id,
                "icon": agent.icon,
                "content": "L'agent a atteint la limite d'itérations d'outils (5). \
                           La tâche est peut-être trop complexe pour une seule interaction.",
                "error": "max_tool_iterations",
                "availability": availability,
                "fallback": false,
            }), false))
            .into_response(),

            Err(crate::tools::ToolChatError::UpstreamError(msg)) => {
                let fb = fallback::fallback_respond(&req.message);
                Json(crate::mention_ia::marquer(serde_json::json!({
                    "agent": agent.display_name,
                    "icon": agent.icon,
                    "content": if fb.answered { fb.content } else { fb.refusal_reason.unwrap_or_default() },
                    "answered": fb.answered,
                    "availability": "fallback",
                    "fallback": true,
                    "upstream_error": msg,
                }), false))
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
    // Fail-closed : un agent_id connu et autorisé pour ce skill est obligatoire.
    if let Err(reason) = crate::security::authorize_skill_request(
        &state.agent_registry,
        req.agent_id.as_deref(),
        &id,
    ) {
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({ "error": reason })),
        )
            .into_response();
    }

    // Appel direct par l'utilisateur (hors boucle du modèle) : une écriture
    // de service exige `confirm: true` explicite dans la requête.
    let autorisation = if req.confirm {
        crate::skills::Autorisation::EcritureConfirmee
    } else {
        crate::skills::Autorisation::Standard
    };
    // Agent déjà validé par `authorize_skill_request` (fail-closed).
    let Some(agent) = req
        .agent_id
        .as_deref()
        .map(str::trim)
        .and_then(|a| state.agent_registry.get_agent(a))
    else {
        return refus_gouvernance("agent inconnu — exécution refusée".into());
    };
    let demande = crate::gouvernance::DemandeOutil {
        agent: &agent,
        skill_id: &id,
        params: &req.params,
        autorisation,
        contenu_etranger: false,
        mode: crate::gouvernance::ModeExecution::Direct,
        appelant: crate::gouvernance::APPELANT_LOCAL,
    };
    let result = match crate::tools::executer_outil_gouverne(
        &state.gouvernance,
        &crate::tools::ExecuteurRegistre,
        &demande,
    )
    .await
    {
        Ok((result, _regle)) => result,
        Err(motif) => {
            state.audit_log.log(crate::security::AuditEntry {
                timestamp: crate::security::epoch_now(),
                agent_id: agent.id.clone(),
                skill_id: id.clone(),
                action: "refus_strongfather".into(),
                details: serde_json::to_string(&req.params).unwrap_or_default(),
                success: false,
                client_id: String::new(),
            });
            return refus_gouvernance(motif);
        }
    };

    state.audit_log.log(crate::security::AuditEntry {
        timestamp: crate::security::epoch_now(),
        agent_id: req.agent_id.unwrap_or_default(),
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
// Confirmations d'actions suspendues (K-02)
// ═══════════════════════════════════════════════════════════════════════

/// Liste les actions suspendues, avec l'action exacte qui sera exécutée.
async fn list_confirmations(State(state): State<ProxyState>) -> impl IntoResponse {
    let actions = state.confirmations.lister();
    Json(serde_json::json!({ "pending_confirmations": actions, "count": actions.len() }))
}

#[derive(serde::Deserialize)]
struct ConfirmationDecision {
    /// `true` : exécuter l'action affichée ; `false` : l'abandonner.
    confirm: bool,
}

/// Confirme (exécute) ou rejette une action suspendue. Usage unique.
async fn decide_confirmation(
    State(state): State<ProxyState>,
    Path(id): Path<String>,
    Json(decision): Json<ConfirmationDecision>,
) -> impl IntoResponse {
    if !decision.confirm {
        return match state.confirmations.prendre(&id) {
            Ok(action) => Json(serde_json::json!({
                "rejected": action.id,
                "action": action.action,
            }))
            .into_response(),
            Err(e) => (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": e.to_string() })),
            )
                .into_response(),
        };
    }
    match crate::tools::executer_confirmation(
        &id,
        &state.confirmations,
        &state.agent_registry,
        &state.audit_log,
        &crate::tools::ExecuteurRegistre,
        &state.gouvernance,
    )
    .await
    {
        Ok((action, result)) => Json(serde_json::json!({
            "confirmed": action.id,
            "action": action.action,
            "result": result,
        }))
        .into_response(),
        Err(e) => (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({ "error": e })),
        )
            .into_response(),
    }
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
    let Some(team) = state.agent_registry.get_team(&id) else {
        return (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": format!("Équipe '{id}' introuvable") })),
        )
            .into_response();
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
    let availability = fallback::probe_availability(&state.inference)
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
    let Some(model) = state.model_manager.find_model(&req.filename) else {
        return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "error": format!("Modèle '{}' introuvable dans {}", req.filename, state.model_manager.models_dir().display()),
                    "hint": "Utilisez GET /v1/models/local pour voir les modèles disponibles",
                })),
            )
                .into_response();
    };

    let path = model.path.clone();
    let display = model.display_name.clone();

    match state.inference.native.load_model(path).await {
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
    if state.inference.native.unload_model().await {
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
        fallback::probe_availability(&state.inference).await;

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
// Middleware de sécurité (origine + requête signée)
// ═══════════════════════════════════════════════════════════════════════

/// Middleware de sécurité appliqué sur toutes les routes protégées.
///
/// Vérifie dans l'ordre (fail-closed) :
/// 0. IP source = adresse TCP réelle (XFF seulement via trusted_proxies)
/// 1. Validation d'origine (IP source dans allowed_origins ou loopback)
/// 2. Requête signée par un appelant épinglé (`crate::signature`) : en-têtes
///    contrôlés avant la lecture du corps ; aucun appelant épinglé ⇒ 503.
///    Plus de jeton Bearer ni de loopback ouvert (lot 2, K-09).
async fn security_middleware(
    State(state): State<ProxyState>,
    req: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    let headers = req.headers().clone();
    let path = req.uri().path().to_string();

    // IP source = adresse réelle de la connexion (ConnectInfo). X-Forwarded-For
    // n'est lu que si la connexion vient d'un proxy listé dans `trusted_proxies`.
    // Sans ConnectInfo (serveur mal lancé) : refus (fail-closed).
    let Some(peer) = req
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ci| ci.0.ip())
    else {
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({
                "error": "Accès refusé",
                "reason": "no_connect_info",
            })),
        )
            .into_response();
    };
    let source_ip =
        crate::security::resolve_client_ip(peer, &headers, &state.security_config.trusted_proxies)
            .to_string();

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

    // 2. Requête signée par un appelant épinglé
    match crate::signature::verifier(&state.gardes, req).await {
        Ok(req) => next.run(req).await.into_response(),
        Err(refus) => {
            state
                .security_audit
                .log_event(crate::security::SecurityEvent {
                    timestamp: crate::security::epoch_now(),
                    kind: if refus.motif == "rejeu" {
                        crate::security::SecurityEventKind::HmacReplay
                    } else {
                        crate::security::SecurityEventKind::AuthFailed
                    },
                    source_ip,
                    path,
                    details: refus.motif.to_string(),
                });
            (
                refus.statut,
                Json(serde_json::json!({
                    "error": "Authentification requise (requête signée)",
                    "reason": refus.motif,
                })),
            )
                .into_response()
        }
    }
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
            // Requêtes signées Ed25519 par des appelants épinglés, toujours exigées.
            "signed_requests": true,
            "callers_pinned": cfg.cles_appelants.len(),
            "team_hubs_pinned": cfg.cles_hub_team.len(),
            "shared_secret": false,
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

/// Passthrough vers l'upstream. L'appelant est déjà authentifié par
/// `security_middleware` (requête signée) ; ses en-têtes d'authentification ne
/// sont jamais relayés.
async fn proxy_handler(State(state): State<ProxyState>, req: Request<Body>) -> impl IntoResponse {
    // Le relais vers un moteur hors COG n'existe que si la politique l'autorise
    // (jamais en production : `maia-inference-externe-refusee`).
    if !state.inference.upstream.est_autorise() {
        return (
            StatusCode::FORBIDDEN,
            Json(serde_json::json!({
                "error": crate::inference::InferenceError::UpstreamRefuse.to_string(),
            })),
        )
            .into_response();
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
            // En-têtes de transport et d'authentification : jamais relayés.
            "host" | "connection" | "transfer-encoding" | "authorization" => {}
            n if n.starts_with("x-miyu-") => {}
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
        Method::POST => reqwest::Method::POST,
        Method::PUT => reqwest::Method::PUT,
        Method::DELETE => reqwest::Method::DELETE,
        Method::PATCH => reqwest::Method::PATCH,
        Method::HEAD => reqwest::Method::HEAD,
        Method::OPTIONS => reqwest::Method::OPTIONS,
        _ => reqwest::Method::GET,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::signature::{GardesMaia, USAGE_API, USAGE_TEAM};
    use miyauth::http::service::{entetes_signees, maintenant_unix};
    use miyusceau::CleSignature;
    use tower::ServiceExt;

    fn cle_appelant() -> CleSignature {
        CleSignature::depuis_graine(&[3_u8; 32])
    }
    fn cle_hub() -> CleSignature {
        CleSignature::depuis_graine(&[4_u8; 32])
    }

    fn etat(gardes: GardesMaia) -> ProxyState {
        let client = reqwest::Client::new();
        // Backends injoignables (port 9) : seule l'authentification est testée.
        let maia =
            crate::inference::MaiaLlmClient::new("http://127.0.0.1:9".into(), None, client.clone());
        let amont =
            crate::inference::UpstreamBackend::new(client.clone(), "http://127.0.0.1:9".into());
        ProxyState {
            client: client.clone(),
            upstream_url: "http://127.0.0.1:9".into(),
            gardes: std::sync::Arc::new(gardes),
            log_requests: false,
            hardware: crate::hardware::detect_hardware(),
            agent_registry: AgentRegistry::new(),
            context_registry: ContextRegistry::new(),
            rate_limiter: RateLimiter::new(crate::security::RateLimitConfig::default()),
            audit_log: AuditLog::new(100),
            inference: InferenceRouter::new(maia, amont, true),
            model_manager: ModelManager::new(
                std::env::temp_dir().join("maia-test-modeles-absents"),
            ),
            security_config: SecurityConfig::default(),
            security_audit: SecurityAuditLog::new(100),
            active_persona: None,
            maia_mode: MaiaMode::Standalone,
            started_at: 0,
            team_registered: false,
            confirmations: std::sync::Arc::new(crate::confirmations::RegistreConfirmations::new()),
            gouvernance: std::sync::Arc::new(crate::gouvernance::GouvernanceMaia::depuis_fichier(
                std::env::temp_dir().join("maia-test-politiques-absentes.toml"),
            )),
        }
    }

    fn securise() -> Router {
        proxy_router(etat(GardesMaia::new(
            vec![cle_appelant().cle_publique()],
            vec![cle_hub().cle_publique()],
        )))
    }

    fn requete(
        methode: &str,
        chemin: &str,
        corps: &str,
        cle: Option<(&CleSignature, &str)>,
    ) -> Request<Body> {
        let mut b = Request::builder()
            .method(methode)
            .uri(chemin)
            .header(header::CONTENT_TYPE, "application/json");
        if let Some((cle, usage)) = cle {
            for (n, v) in entetes_signees(cle, usage, methode, chemin, corps.as_bytes()) {
                b = b.header(n, v);
            }
        }
        let mut r = b.body(Body::from(corps.to_string())).unwrap();
        r.extensions_mut()
            .insert(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 40000))));
        r
    }

    async fn statut(app: &Router, r: Request<Body>) -> StatusCode {
        app.clone().oneshot(r).await.unwrap().status()
    }

    #[tokio::test]
    async fn appelant_epingle_accepte() {
        let app = securise();
        let r = requete("GET", "/v1/agents", "", Some((&cle_appelant(), USAGE_API)));
        assert_eq!(statut(&app, r).await, StatusCode::OK);
        let r = requete(
            "GET",
            "/v1/security/status",
            "",
            Some((&cle_appelant(), USAGE_API)),
        );
        assert_eq!(statut(&app, r).await, StatusCode::OK);
    }

    #[tokio::test]
    async fn non_signe_cle_inconnue_bearer_refuses() {
        let app = securise();
        assert_eq!(
            statut(&app, requete("GET", "/v1/agents", "", None)).await,
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            statut(&app, requete("GET", "/status", "", None)).await,
            StatusCode::UNAUTHORIZED
        );
        let mut r = requete("GET", "/v1/agents", "", None);
        r.headers_mut()
            .insert(header::AUTHORIZATION, "Bearer secret".parse().unwrap());
        assert_eq!(statut(&app, r).await, StatusCode::UNAUTHORIZED);
        let intrus = CleSignature::generer();
        let r = requete("GET", "/v1/agents", "", Some((&intrus, USAGE_API)));
        assert_eq!(statut(&app, r).await, StatusCode::UNAUTHORIZED);
        // Passthrough proxy : même porte.
        let r = requete("POST", "/v1/chat/completions", "{}", None);
        assert_eq!(statut(&app, r).await, StatusCode::UNAUTHORIZED);
        // Santé publique.
        assert_eq!(
            statut(&app, requete("GET", "/health", "", None)).await,
            StatusCode::OK
        );
    }

    #[tokio::test]
    async fn hub_team_limite_au_chat_d_agent() {
        let app = securise();
        let r = requete("GET", "/v1/agents", "", Some((&cle_hub(), USAGE_TEAM)));
        assert_eq!(statut(&app, r).await, StatusCode::UNAUTHORIZED);
        let r = requete("GET", "/v1/agents", "", Some((&cle_hub(), USAGE_API)));
        assert_eq!(statut(&app, r).await, StatusCode::UNAUTHORIZED);
        // Usage d'appelant sur la route TEAM : signature invalide.
        let r = requete(
            "POST",
            "/v1/agents/inconnu/chat",
            "{}",
            Some((&cle_hub(), USAGE_API)),
        );
        assert_eq!(statut(&app, r).await, StatusCode::UNAUTHORIZED);
        // Route TEAM avec l'usage TEAM : authentifié (l'agent inconnu donne 404, pas 401).
        let r = requete(
            "POST",
            "/v1/agents/inconnu/chat",
            "{\"messages\":[]}",
            Some((&cle_hub(), USAGE_TEAM)),
        );
        let s = statut(&app, r).await;
        assert!(
            s != StatusCode::UNAUTHORIZED && s != StatusCode::SERVICE_UNAVAILABLE,
            "{s}"
        );
    }

    #[tokio::test]
    async fn rejeu_perime_corps_altere_refuses() {
        let app = securise();
        let corps = "{\"name\":\"x\"}";
        let signe = requete(
            "POST",
            "/v1/contexts",
            corps,
            Some((&cle_appelant(), USAGE_API)),
        );
        let entetes = signe.headers().clone();
        assert_ne!(statut(&app, signe).await, StatusCode::UNAUTHORIZED);
        let mut rejeu = requete("POST", "/v1/contexts", corps, None);
        rejeu.headers_mut().extend(entetes);
        assert_eq!(statut(&app, rejeu).await, StatusCode::UNAUTHORIZED);
        let signe = requete(
            "POST",
            "/v1/contexts",
            corps,
            Some((&cle_appelant(), USAGE_API)),
        );
        let mut altere = requete("POST", "/v1/contexts", "{\"name\":\"y\"}", None);
        altere.headers_mut().extend(signe.headers().clone());
        assert_eq!(statut(&app, altere).await, StatusCode::UNAUTHORIZED);
        let vieux = miyauth::http::signer_requete(
            &cle_appelant(),
            USAGE_API,
            "GET",
            "/v1/agents",
            b"",
            maintenant_unix() - 600,
        );
        let mut r = requete("GET", "/v1/agents", "", None);
        for (n, v) in vieux.en_entetes() {
            r.headers_mut().insert(n, v.parse().unwrap());
        }
        assert_eq!(statut(&app, r).await, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn aucune_cle_epinglee_tout_refuse_meme_en_loopback() {
        let app = proxy_router(etat(GardesMaia::default()));
        let mut r = requete("GET", "/v1/agents", "", Some((&cle_appelant(), USAGE_API)));
        r.headers_mut()
            .insert(header::HOST, "127.0.0.1:11435".parse().unwrap());
        assert_eq!(statut(&app, r).await, StatusCode::SERVICE_UNAVAILABLE);
        let mut r = requete("GET", "/v1/agents", "", None);
        r.headers_mut()
            .insert(header::HOST, "localhost:11435".parse().unwrap());
        assert_eq!(statut(&app, r).await, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(
            statut(&app, requete("GET", "/health", "", None)).await,
            StatusCode::OK
        );
    }
}
