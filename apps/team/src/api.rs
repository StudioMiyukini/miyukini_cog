//! @id     team_api
//! @do     register_node
//! @role   orchestration
//! @layer  app
//! @human  team — module « api ».

//! Routes d'enregistrement et heartbeat des nœuds MAIA.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::mws::policy::validate_node_url;
use crate::nodes::registry::{NodeHealth, NodeInfo};
use crate::AppState;

/// Nombre maximal de nœuds dans le registre (C-13).
pub const MAX_NODES: usize = 256;
/// Longueur maximale des champs texte courts d'un nœud (octets).
pub const MAX_FIELD_LEN: usize = 256;

/// Vérifie les bornes des champs texte d'un `NodeInfo` (C-13).
fn node_fields_within_bounds(info: &NodeInfo) -> bool {
    let short = [
        Some(info.id.as_str()),
        Some(info.name.as_str()),
        Some(info.version.as_str()),
        Some(info.cpu.as_str()),
        info.gpu.as_deref(),
        info.model_active.as_deref(),
        info.persona.as_deref(),
    ];
    !info.id.trim().is_empty()
        && short.iter().flatten().all(|f| f.len() <= MAX_FIELD_LEN)
}

fn bad_request(msg: &str) -> axum::response::Response {
    (StatusCode::BAD_REQUEST, Json(json!({ "error": msg }))).into_response()
}

/// POST /team/register — enregistrement d'un nœud MAIA.
///
/// L'URL du nœud est validée (anti-SSRF) : http(s), politique d'isolation,
/// loopback ou `node_allowed_hosts`. Refus → 400.
pub async fn register_node(
    State(state): State<AppState>,
    Json(info): Json<NodeInfo>,
) -> axum::response::Response {
    if !node_fields_within_bounds(&info) {
        return bad_request("champs de nœud invalides ou trop longs");
    }
    if let Err(e) = validate_node_url(
        &info.maia_url,
        state.config.isolation_policy,
        &state.config.node_allowed_hosts,
    ) {
        tracing::warn!("Enregistrement refusé ({}) : {e}", info.id);
        return bad_request(&format!("maia_url refusée : {e}"));
    }
    let id = info.id.clone();
    if !state.registry.register_bounded(info, MAX_NODES) {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "error": "registre de nœuds plein" })),
        )
            .into_response();
    }
    tracing::info!("Nœud enregistré : {id}");
    Json(json!({
        "registered": true,
        "id": id,
        "heartbeat_interval_secs": state.config.health_interval_secs,
    }))
    .into_response()
}

/// POST /team/heartbeat — heartbeat d'un nœud MAIA.
#[derive(Deserialize)]
pub struct HeartbeatRequest {
    pub id: String,
    #[serde(default)]
    pub model_active: Option<String>,
    #[serde(default)]
    pub persona: Option<String>,
    // Health optionnel (si le nœud l'envoie directement)
    #[serde(default)]
    pub health: Option<NodeHealth>,
}

pub async fn node_heartbeat(
    State(state): State<AppState>,
    Json(req): Json<HeartbeatRequest>,
) -> impl IntoResponse {
    let health = req.health.unwrap_or(NodeHealth {
        model: req.model_active.clone(),
        backend: None,
        hardware_tier: String::new(),
        uptime_secs: 0,
        capabilities_agents: 0,
        capabilities_tts: false,
    });

    state.registry.heartbeat(&req.id, health, req.model_active, req.persona);
    tracing::debug!("Heartbeat reçu : {}", req.id);
    Json(json!({ "ack": true }))
}

/// DELETE /team/nodes/:id — désenregistrement propre.
pub async fn unregister_node(
    State(state): State<AppState>,
    Path(node_id): Path<String>,
) -> impl IntoResponse {
    state.registry.unregister(&node_id);
    tracing::info!("Nœud désenregistré : {node_id}");
    Json(json!({ "unregistered": true }))
}

/// GET /team/nodes — liste des nœuds (vue interne).
pub async fn list_nodes_internal(State(state): State<AppState>) -> impl IntoResponse {
    crate::mws::routes::list_nodes(State(state)).await
}
