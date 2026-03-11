//! Routes d'enregistrement et heartbeat des nœuds MAIA.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::nodes::registry::{NodeHealth, NodeInfo};
use crate::AppState;

/// POST /team/register — enregistrement d'un nœud MAIA.
pub async fn register_node(
    State(state): State<AppState>,
    Json(info): Json<NodeInfo>,
) -> impl IntoResponse {
    let id = info.id.clone();
    state.registry.register(info);
    tracing::info!("Nœud enregistré : {id}");
    Json(json!({
        "registered": true,
        "id": id,
        "heartbeat_interval_secs": state.config.health_interval_secs,
    }))
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
