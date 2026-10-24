//! @id     team_api
//! @do     register_node
//! @role   orchestration
//! @layer  app
//! @human  team — module « api ».

//! Routes d'enregistrement et heartbeat des nœuds MAIA.

use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::auth::Signataire;
use crate::mws::policy::validate_node_url;
use crate::nodes::registry::{NodeHealth, NodeInfo, RefusEnregistrement};
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
/// loopback ou `node_allowed_hosts`. Refus → 400. Le nœud est lié à la clé
/// qui a signé la requête : un identifiant déjà lié à une autre clé → 409.
pub async fn register_node(
    State(state): State<AppState>,
    Extension(Signataire(signataire)): Extension<Signataire>,
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
    match state.registry.register_signed(info, MAX_NODES, signataire) {
        Ok(()) => {}
        Err(RefusEnregistrement::Plein) => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({ "error": "registre de nœuds plein" })),
            )
                .into_response();
        }
        Err(RefusEnregistrement::AutreSignataire) => {
            tracing::warn!("Enregistrement refusé ({id}) : identifiant lié à une autre clé");
            return (
                StatusCode::CONFLICT,
                Json(json!({ "error": "identifiant de nœud lié à une autre clé" })),
            )
                .into_response();
        }
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
    Extension(Signataire(signataire)): Extension<Signataire>,
    Json(req): Json<HeartbeatRequest>,
) -> axum::response::Response {
    let health = req.health.unwrap_or(NodeHealth {
        model: req.model_active.clone(),
        backend: None,
        hardware_tier: String::new(),
        uptime_secs: 0,
        capabilities_agents: 0,
        capabilities_tts: false,
    });

    if !state
        .registry
        .heartbeat_signed(&req.id, signataire, health, req.model_active, req.persona)
    {
        return (StatusCode::NOT_FOUND, Json(json!({ "error": "nœud inconnu pour cette clé" }))).into_response();
    }
    tracing::debug!("Heartbeat reçu : {}", req.id);
    Json(json!({ "ack": true })).into_response()
}

/// DELETE /team/nodes/:id — désenregistrement propre.
pub async fn unregister_node(
    State(state): State<AppState>,
    Extension(Signataire(signataire)): Extension<Signataire>,
    Path(node_id): Path<String>,
) -> axum::response::Response {
    if !state.registry.unregister_signed(&node_id, signataire) {
        return (StatusCode::NOT_FOUND, Json(json!({ "error": "nœud inconnu pour cette clé" }))).into_response();
    }
    tracing::info!("Nœud désenregistré : {node_id}");
    Json(json!({ "unregistered": true })).into_response()
}

/// GET /team/nodes — liste des nœuds (vue interne).
pub async fn list_nodes_internal(State(state): State<AppState>) -> impl IntoResponse {
    crate::mws::routes::list_nodes(State(state)).await
}
