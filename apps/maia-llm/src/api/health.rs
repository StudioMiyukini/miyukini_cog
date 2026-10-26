//! @id     maia_llm_api_health
//! @do     definir_llm_health
//! @role   orchestration
//! @layer  app
//! @human  maia-llm — module « api health ».

//! GET /maia/llm/health — état du moteur d'inférence.
//! Route publique : ne divulgue volontairement pas `model_path` (chemins disque).

use axum::{extract::State, response::IntoResponse, Json};
use serde::Serialize;

use crate::AppState;

#[derive(Debug, Serialize)]
pub struct LlmHealth {
    pub status: &'static str,
    pub model_loaded: bool,
    pub model_name: Option<String>,
    pub size_bytes: Option<u64>,
    pub gpu_layers: Option<u32>,
    pub context_size: Option<u32>,
    pub models_available: usize,
    pub uptime_secs: u64,
    /// Date de coupure des connaissances (manifeste signé du modèle chargé).
    pub date_coupure: Option<String>,
    /// Licence du modèle chargé (manifeste signé).
    pub licence: Option<String>,
    /// GPU utilisable par ce binaire (détection llama.cpp, sans outil externe).
    pub gpu_utilisable: bool,
    /// Profil matériel recommandé (P1/P2/P3).
    pub recommandation: crate::materiel::Recommandation,
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
        size_bytes: model_info.as_ref().map(|m| m.size_bytes),
        gpu_layers: model_info.as_ref().map(|m| m.gpu_layers),
        context_size: model_info.as_ref().map(|m| m.context_size),
        models_available,
        uptime_secs,
        date_coupure: model_info.as_ref().and_then(|m| m.date_coupure.clone()),
        licence: model_info.as_ref().and_then(|m| m.licence.clone()),
        gpu_utilisable: state.engine.gpu_utilisable(),
        recommandation: crate::materiel::recommander_ici(),
    };

    Json(health)
}
