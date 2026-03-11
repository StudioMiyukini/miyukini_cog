//! Routes MWS (Miyukini Web Service) gateway.
//!
//! POST /api/task        — soumet une tâche
//! GET  /api/task/:id    — état d'une tâche
//! GET  /api/agents      — liste des agents disponibles sur les nœuds
//! GET  /api/nodes       — liste des nœuds enregistrés
//! GET  /api/sequences   — liste des séquences MIPOWER

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::orchestrator::scheduler::{Task, TaskKind, PRIORITY_NORMAL};
use crate::AppState;

/// POST /api/task — soumet une tâche au scheduler.
pub async fn submit_task(
    State(state): State<AppState>,
    Json(req): Json<TaskSubmitRequest>,
) -> impl IntoResponse {
    let task = Task {
        id: String::new(),
        priority: req.priority.unwrap_or(PRIORITY_NORMAL),
        kind: req.kind.unwrap_or(TaskKind::AgentChat),
        payload: req.payload,
        min_tier: req.min_tier,
        model_pattern: req.model_pattern,
        timeout_secs: req.timeout_secs.unwrap_or(120),
    };

    let id = state.scheduler.submit(task);
    Json(json!({ "task_id": id, "status": "pending" }))
}

#[derive(Deserialize)]
pub struct TaskSubmitRequest {
    pub payload: serde_json::Value,
    #[serde(default)]
    pub priority: Option<u8>,
    #[serde(default)]
    pub kind: Option<TaskKind>,
    #[serde(default)]
    pub min_tier: Option<u8>,
    #[serde(default)]
    pub model_pattern: Option<String>,
    #[serde(default)]
    pub timeout_secs: Option<u64>,
}

/// GET /api/task/:id — état d'une tâche.
pub async fn get_task(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> impl IntoResponse {
    match state.scheduler.get_result(&task_id) {
        Some(result) => Json(json!(result)).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(json!({"error": format!("Tâche '{}' inconnue", task_id)})),
        )
            .into_response(),
    }
}

/// GET /api/nodes — liste tous les nœuds avec leur statut.
pub async fn list_nodes(State(state): State<AppState>) -> impl IntoResponse {
    let nodes: Vec<serde_json::Value> = state
        .registry
        .all_nodes()
        .into_iter()
        .map(|n| {
            json!({
                "id": n.info.id,
                "name": n.info.name,
                "url": n.info.maia_url,
                "status": n.status.to_string(),
                "tier": n.info.hardware_tier,
                "model_active": n.info.model_active,
                "model_expected": n.info.model_active,
                "match": true, // Simplification S3-E02
                "persona": n.info.persona,
                "last_seen_ago_secs": n.age().as_secs(),
            })
        })
        .collect();

    Json(json!({
        "nodes": nodes,
        "count": nodes.len(),
        "online": state.registry.online_count(),
    }))
}

/// GET /api/sequences — liste les séquences MIPOWER disponibles.
pub async fn list_sequences(State(state): State<AppState>) -> impl IntoResponse {
    let sequences: Vec<serde_json::Value> = state
        .sequences
        .iter()
        .map(|s| {
            json!({
                "name": s.name,
                "steps_count": s.steps.len(),
                "brief_preview": s.brief.lines().next().unwrap_or("").trim_start_matches('#').trim(),
            })
        })
        .collect();

    Json(json!({
        "sequences": sequences,
        "count": sequences.len(),
    }))
}

/// GET /api/status — état global du hub TEAM.
pub async fn team_status(State(state): State<AppState>) -> impl IntoResponse {
    Json(json!({
        "service": "team",
        "version": env!("CARGO_PKG_VERSION"),
        "nodes_online": state.registry.online_count(),
        "nodes_total": state.registry.all_nodes().len(),
        "tasks_pending": state.scheduler.pending_count(),
        "sequences_loaded": state.sequences.len(),
        "isolation_policy": state.config.isolation_policy.to_string(),
    }))
}
