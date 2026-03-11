//! Endpoints de gestion des modèles.
//!
//! GET    /v1/models                      — modèles chargés (OpenAI-compat)
//! GET    /maia/llm/models                — tous les modèles GGUF disponibles
//! POST   /maia/llm/models/load           — charger un modèle
//! DELETE /maia/llm/models/unload         — décharger le modèle actif
//! POST   /maia/llm/models/:id/hash       — calculer/retourner le SHA-256 (lazy)

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::PathBuf;

use crate::store::GgufModel;
use crate::AppState;

// ── Réponses OpenAI-compat ────────────────────────────────────────

#[derive(Serialize)]
struct OpenAiModelEntry {
    id: String,
    object: &'static str,
    created: u64,
    owned_by: &'static str,
}

#[derive(Serialize)]
struct OpenAiModelsResponse {
    object: &'static str,
    data: Vec<OpenAiModelEntry>,
}

/// GET /v1/models — retourne le modèle chargé au format OpenAI.
pub async fn list_loaded_handler(State(state): State<AppState>) -> impl IntoResponse {
    let data = if let Some(info) = state.engine.model_info() {
        vec![OpenAiModelEntry {
            id: info.name.clone(),
            object: "model",
            created: 0,
            owned_by: "maia-llm",
        }]
    } else {
        vec![]
    };

    Json(OpenAiModelsResponse {
        object: "list",
        data,
    })
}

/// GET /maia/llm/models — tous les GGUF disponibles dans le dossier modèles.
pub async fn list_available_handler(State(state): State<AppState>) -> impl IntoResponse {
    let models: Vec<GgufModelInfo> = state
        .store
        .scan()
        .into_iter()
        .map(|m| {
            let loaded = state
                .engine
                .model_info()
                .map(|i| i.name == m.filename)
                .unwrap_or(false);
            GgufModelInfo {
                filename: m.filename,
                model_id: m.model_id,
                display_name: m.display_name,
                size_display: m.size_display,
                size_bytes: m.size_bytes,
                sha256: m.sha256,
                loaded,
            }
        })
        .collect();

    Json(models)
}

#[derive(Serialize)]
struct GgufModelInfo {
    filename: String,
    model_id: String,
    display_name: String,
    size_display: String,
    size_bytes: u64,
    sha256: Option<String>,
    loaded: bool,
}

// ── Load / Unload ─────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct LoadRequest {
    /// Nom de fichier ou model_id (recherche partielle).
    pub model: String,
}

/// POST /maia/llm/models/load — charge un modèle GGUF.
pub async fn load_handler(
    State(state): State<AppState>,
    Json(req): Json<LoadRequest>,
) -> impl IntoResponse {
    let model_entry: GgufModel = match state.store.find(&req.model) {
        Some(m) => m,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"error": format!("Modèle '{}' non trouvé dans {}", req.model, state.store.models_dir().display())})),
            )
                .into_response();
        }
    };

    let path: PathBuf = model_entry.path.clone();

    let result = tokio::task::spawn_blocking(move || state.engine.load_model(path))
        .await
        .map_err(|e| format!("Task join : {e}"))
        .and_then(|r| r.map_err(|e| e.to_string()));

    match result {
        Ok(info) => Json(json!({
            "loaded": true,
            "model": info.name,
            "gpu_layers": info.gpu_layers,
            "context_size": info.context_size,
        }))
        .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e})),
        )
            .into_response(),
    }
}

/// DELETE /maia/llm/models/unload — décharge le modèle actif.
pub async fn unload_handler(State(state): State<AppState>) -> impl IntoResponse {
    let engine = state.engine.clone();
    let unloaded = tokio::task::spawn_blocking(move || engine.unload())
        .await
        .unwrap_or(false);

    Json(json!({"unloaded": unloaded}))
}

/// POST /maia/llm/models/:id/hash — calcule (et met en cache) le SHA-256 d'un modèle.
///
/// Pour les gros fichiers (>10 GB) ce calcul peut prendre plusieurs minutes.
/// La réponse n'est envoyée qu'une fois le calcul terminé.
/// Le sidecar `.gguf.sha256` est écrit pour les appels futurs.
pub async fn hash_handler(
    State(state): State<AppState>,
    Path(model_id): Path<String>,
) -> impl IntoResponse {
    let model_entry = match state.store.find(&model_id) {
        Some(m) => m,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"error": format!("Modèle '{}' non trouvé", model_id)})),
            )
                .into_response();
        }
    };

    let path = model_entry.path.clone();
    let result = tokio::task::spawn_blocking(move || {
        crate::store::compute_sha256_and_cache(&path)
    })
    .await
    .map_err(|e| format!("Task join : {e}"))
    .and_then(|r| r.map_err(|e| e.to_string()));

    match result {
        Ok(hash) => Json(json!({
            "model_id": model_entry.model_id,
            "filename": model_entry.filename,
            "sha256": hash,
        }))
        .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e})),
        )
            .into_response(),
    }
}
