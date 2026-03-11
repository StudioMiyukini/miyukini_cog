//! GET /maia/llm/health — état du moteur d'inférence.

use axum::{extract::State, response::IntoResponse, Json};
use serde::Serialize;

use crate::AppState;

#[derive(Debug, Serialize)]
pub struct LlmHealth {
    pub status: &'static str,
    pub model_loaded: bool,
    pub model_name: Option<String>,
    pub model_path: Option<String>,
    pub size_bytes: Option<u64>,
    pub gpu_layers: Option<u32>,
    pub context_size: Option<u32>,
    pub models_available: usize,
    pub uptime_secs: u64,
}

pub async fn health_handler(State(state): State<AppState>) -> impl IntoResponse {
    let model_info = state.engine.model_info();
    let models_available = state.store.scan().len();

    let uptime_secs = state
        .started_at
        .elapsed()
        .unwrap_or_default()
        .as_secs();

    let health = LlmHealth {
        status: if model_info.is_some() { "ready" } else { "idle" },
        model_loaded: model_info.is_some(),
        model_name: model_info.as_ref().map(|m| m.name.clone()),
        model_path: model_info.as_ref().map(|m| m.path.clone()),
        size_bytes: model_info.as_ref().map(|m| m.size_bytes),
        gpu_layers: model_info.as_ref().map(|m| m.gpu_layers),
        context_size: model_info.as_ref().map(|m| m.context_size),
        models_available,
        uptime_secs,
    };

    Json(health)
}
