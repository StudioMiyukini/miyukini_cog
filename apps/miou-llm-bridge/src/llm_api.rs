//! API LLM partagée — endpoints pour les services COG.
//!
//! Permet aux autres services de l'écosystème Miyukini (JayKonta, JayKoa,
//! MiyukiniWatch, etc.) d'utiliser le LLM via des endpoints simples,
//! sans avoir besoin de leur propre client LLM.
//! Utilise InferenceRouter : natif GGUF d'abord, upstream en backup.

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::catalog::CATALOG;
use crate::fallback;
use crate::proxy::ProxyState;
use crate::recommend;

// ── Types requête ───────────────────────────────────────────────────

/// POST /v1/llm/chat — Chat simple (pas d'agent, pas de tools).
#[derive(Debug, Deserialize)]
pub struct LlmChatRequest {
    /// Modèle à utiliser (ou "default" pour le premier chargé).
    #[serde(default = "default_model")]
    pub model: String,
    /// Messages au format OpenAI.
    pub messages: Vec<serde_json::Value>,
    /// Température (0.0–2.0).
    #[serde(default = "default_temperature")]
    pub temperature: f64,
    /// Tokens maximum.
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
}

/// POST /v1/llm/complete — Complétion texte simple.
#[derive(Debug, Deserialize)]
pub struct LlmCompleteRequest {
    #[serde(default = "default_model")]
    pub model: String,
    /// Prompt texte à compléter.
    pub prompt: String,
    #[serde(default = "default_temperature")]
    pub temperature: f64,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    /// Séquences de stop optionnelles.
    #[serde(default)]
    pub stop: Option<Vec<String>>,
}

fn default_model() -> String {
    "default".into()
}
fn default_temperature() -> f64 {
    0.7
}
fn default_max_tokens() -> u32 {
    2048
}

// ── Types réponse ───────────────────────────────────────────────────

/// Entrée modèle enrichie pour GET /v1/llm/models.
#[derive(Debug, Serialize)]
pub struct LlmModelEntry {
    pub id: String,
    pub display_name: String,
    pub loaded: bool,
    pub backend: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantization: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_score: Option<u8>,
}

// ── Handlers ────────────────────────────────────────────────────────

/// POST /v1/llm/chat — Chat via InferenceRouter (natif ou upstream).
pub async fn llm_chat(
    State(state): State<ProxyState>,
    Json(req): Json<LlmChatRequest>,
) -> impl IntoResponse {
    // Rate limiting
    if let Err(retry_after) = state.rate_limiter.check("llm_api") {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            Json(serde_json::json!({
                "error": "Rate limit exceeded",
                "retry_after_secs": retry_after,
            })),
        )
            .into_response();
    }

    // Disponibilité
    let availability = fallback::probe_availability(
        &state.client,
        &state.upstream_url,
        state.inference.native.is_loaded(),
    )
    .await;
    if availability.is_degraded() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": "Aucun LLM disponible",
                "availability": availability,
            })),
        )
            .into_response();
    }

    // Construire et envoyer via InferenceRouter
    let body = serde_json::json!({
        "model": req.model,
        "messages": req.messages,
        "temperature": req.temperature,
        "max_tokens": req.max_tokens,
    });

    match state.inference.chat_completion(body).await {
        Ok(resp) => {
            let content = resp.choices.first()
                .and_then(|c| c.message.get("content"))
                .and_then(|c| c.as_str())
                .unwrap_or("");

            Json(serde_json::json!({
                "content": content,
                "model": resp.model,
                "backend": resp.backend,
                "usage": resp.usage,
                "availability": availability,
            }))
            .into_response()
        }
        Err(e) => (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({ "error": format!("{e}") })),
        )
            .into_response(),
    }
}

/// POST /v1/llm/complete — Complétion texte via InferenceRouter.
pub async fn llm_complete(
    State(state): State<ProxyState>,
    Json(req): Json<LlmCompleteRequest>,
) -> impl IntoResponse {
    // Rate limiting
    if let Err(retry_after) = state.rate_limiter.check("llm_api") {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            Json(serde_json::json!({
                "error": "Rate limit exceeded",
                "retry_after_secs": retry_after,
            })),
        )
            .into_response();
    }

    // Disponibilité
    let availability = fallback::probe_availability(
        &state.client,
        &state.upstream_url,
        state.inference.native.is_loaded(),
    )
    .await;
    if availability.is_degraded() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "error": "Aucun LLM disponible",
                "availability": availability,
            })),
        )
            .into_response();
    }

    // Convertir prompt en format chat
    let mut body = serde_json::json!({
        "model": req.model,
        "messages": [{ "role": "user", "content": req.prompt }],
        "temperature": req.temperature,
        "max_tokens": req.max_tokens,
    });

    if let Some(ref stop) = req.stop {
        body["stop"] = serde_json::to_value(stop).unwrap_or_default();
    }

    match state.inference.chat_completion(body).await {
        Ok(resp) => {
            let text = resp.choices.first()
                .and_then(|c| c.message.get("content"))
                .and_then(|c| c.as_str())
                .unwrap_or("");

            Json(serde_json::json!({
                "text": text,
                "model": resp.model,
                "backend": resp.backend,
                "usage": resp.usage,
                "availability": availability,
            }))
            .into_response()
        }
        Err(e) => (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({ "error": format!("{e}") })),
        )
            .into_response(),
    }
}

/// GET /v1/llm/models — Liste des modèles (natif + upstream), enrichie avec catalogue.
pub async fn llm_models(State(state): State<ProxyState>) -> impl IntoResponse {
    let mut models: Vec<LlmModelEntry> = Vec::new();

    // 1. Modèle natif chargé
    if let Some(native_name) = state.inference.native.loaded_model_name() {
        let native_lower = native_name.to_lowercase();
        let catalog_match = CATALOG
            .iter()
            .find(|e| native_lower.contains(e.match_pattern));

        models.push(LlmModelEntry {
            id: native_name.clone(),
            display_name: catalog_match
                .map(|c| c.display_name.to_string())
                .unwrap_or_else(|| native_name.clone()),
            loaded: true,
            backend: "native".into(),
            params: catalog_match.map(|c| c.params.to_string()),
            quantization: catalog_match.map(|c| c.quantization.to_string()),
            quality_score: catalog_match.map(|c| c.quality_score),
        });
    }

    // 2. Modèles upstream (LM Studio)
    let upstream_ids = recommend::fetch_loaded_model_ids(&state).await;
    for model_id in &upstream_ids {
        let model_lower = model_id.to_lowercase();
        let catalog_match = CATALOG
            .iter()
            .find(|e| model_lower.contains(e.match_pattern));

        models.push(LlmModelEntry {
            id: model_id.clone(),
            display_name: catalog_match
                .map(|c| c.display_name.to_string())
                .unwrap_or_else(|| model_id.clone()),
            loaded: true,
            backend: "upstream".into(),
            params: catalog_match.map(|c| c.params.to_string()),
            quantization: catalog_match.map(|c| c.quantization.to_string()),
            quality_score: catalog_match.map(|c| c.quality_score),
        });
    }

    // 3. Modèles GGUF locaux non chargés
    let local_models = state.model_manager.scan_models();
    let native_name = state.inference.native.loaded_model_name();
    for local in &local_models {
        let already_listed = native_name.as_ref().map_or(false, |n| *n == local.filename);
        if !already_listed {
            models.push(LlmModelEntry {
                id: local.filename.clone(),
                display_name: local.display_name.clone(),
                loaded: false,
                backend: "local".into(),
                params: None,
                quantization: None,
                quality_score: None,
            });
        }
    }

    let count = models.len();
    Json(serde_json::json!({
        "models": models,
        "count": count,
    }))
}

/// GET /v1/llm/status — État complet (disponibilité, backends, modèles).
pub async fn llm_status(State(state): State<ProxyState>) -> impl IntoResponse {
    let native_loaded = state.inference.native.is_loaded();
    let availability = fallback::probe_availability(
        &state.client,
        &state.upstream_url,
        native_loaded,
    )
    .await;

    let upstream_ids = recommend::fetch_loaded_model_ids(&state).await;
    let local_models = state.model_manager.scan_models();
    let active = state.inference.active_backend().await;

    Json(serde_json::json!({
        "availability": availability,
        "availability_label": availability.label(),
        "degraded": availability.is_degraded(),
        "has_llm": availability.has_llm(),
        "active_backend": active,
        "native": {
            "loaded": native_loaded,
            "model": state.inference.native.loaded_model_name(),
        },
        "upstream": {
            "loaded_models": upstream_ids,
            "count": upstream_ids.len(),
            "url": state.upstream_url,
        },
        "local_gguf": {
            "models": local_models,
            "count": local_models.len(),
        },
        "hardware_tier": state.hardware.tier.label(),
    }))
}
