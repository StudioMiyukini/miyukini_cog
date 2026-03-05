//! MiyuSTT toolkit (V1 initiale).
//!
//! Fournit:
//! - types API STT v1
//! - selection de preset hardware
//! - routeur HTTP local (`/api/health`, `/api/stt`, `/api/stt/stream`)

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

/// Niveaux de preset STT.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SttPreset {
    /// Priorite compacite/latence.
    Compact,
    /// Equilibre qualite/performance.
    Balanced,
    /// Priorite qualite transcription.
    Precision,
    /// Fallback de securite en cas d'erreur.
    SafeFallback,
}

/// Moteurs STT supportes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SttEngine {
    /// Moteur principal.
    FasterWhisper,
    /// Fallback robuste local.
    WhisperCpp,
    /// Option temps reel.
    SherpaOnnx,
}

impl SttEngine {
    fn as_str(self) -> &'static str {
        match self {
            Self::FasterWhisper => "faster-whisper",
            Self::WhisperCpp => "whisper.cpp",
            Self::SherpaOnnx => "sherpa-onnx",
        }
    }

    fn default_model(self, preset: SttPreset) -> &'static str {
        match (self, preset) {
            (Self::FasterWhisper, SttPreset::Compact) => "base-int8",
            (Self::FasterWhisper, SttPreset::Balanced) => "small-int8",
            (Self::FasterWhisper, SttPreset::Precision) => "medium-fp16",
            (Self::FasterWhisper, SttPreset::SafeFallback) => "base-int8",
            (Self::WhisperCpp, _) => "base",
            (Self::SherpaOnnx, _) => "streaming-small",
        }
    }
}

/// Meta hardware minimum pour selection auto.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardwareProfile {
    /// Coeurs CPU logiques.
    pub cpu_cores: u32,
    /// RAM totale en MB.
    pub ram_total_mb: u64,
    /// VRAM GPU en MB (0 si absent).
    pub gpu_vram_mb: u64,
}

/// Selection automatique de preset STT.
#[must_use]
pub fn select_preset(hw: HardwareProfile) -> SttPreset {
    if hw.gpu_vram_mb >= 8192 || hw.ram_total_mb >= 16384 {
        SttPreset::Precision
    } else if hw.cpu_cores >= 6 || hw.ram_total_mb >= 8192 {
        SttPreset::Balanced
    } else {
        SttPreset::Compact
    }
}

/// Requete STT batch v1.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttRequest {
    /// Echantillons PCM normalises.
    pub samples: Vec<f32>,
    /// Frequence d'echantillonnage.
    pub sample_rate: u32,
    /// Langue demandee (`auto`, `fr`, `en`).
    pub language: String,
    /// Modele demande (optionnel).
    #[serde(default)]
    pub model: Option<String>,
    /// Hint de langues prioritaires.
    #[serde(default)]
    pub languages_hint: Vec<String>,
    /// Preset force (optionnel).
    #[serde(default)]
    pub preset: Option<SttPreset>,
}

/// Reponse STT batch v1.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttResponse {
    /// Texte transcrit.
    pub transcript: String,
    /// Confiance (0.0..1.0).
    pub confidence: f32,
    /// Duree de traitement.
    pub duration_ms: u64,
    /// Moteur ayant produit la sortie.
    pub engine: String,
    /// Modele utilise.
    pub model_used: String,
    /// Langue detectee.
    pub language_detected: String,
}

/// Erreurs STT v1.
#[derive(Debug, thiserror::Error)]
pub enum SttError {
    /// Donnees audio invalides.
    #[error("invalid audio payload")]
    InvalidAudio,
    /// Aucune transcription disponible.
    #[error("no transcript available")]
    EmptyTranscript,
    /// Erreur moteur.
    #[error("engine error: {0}")]
    Engine(String),
}

/// Etat partage du service STT.
#[derive(Debug, Clone)]
pub struct SttState {
    /// Moteur principal.
    pub primary_engine: SttEngine,
    /// Chaine fallback locale.
    pub fallback_engines: Vec<SttEngine>,
}

impl Default for SttState {
    fn default() -> Self {
        Self {
            primary_engine: SttEngine::FasterWhisper,
            fallback_engines: vec![SttEngine::WhisperCpp, SttEngine::SherpaOnnx],
        }
    }
}

/// Construit un routeur HTTP MiyuSTT v1.
#[must_use]
pub fn router() -> Router {
    let state = SttState::default();
    Router::new()
        .route("/api/health", get(health_handler))
        .route("/api/stt", post(stt_handler))
        .route("/api/stt/stream", get(stt_stream_handler))
        .with_state(state)
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "miyustt",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

async fn stt_handler(
    State(state): State<SttState>,
    Json(req): Json<SttRequest>,
) -> Result<Json<SttResponse>, axum::http::StatusCode> {
    if req.samples.is_empty() || req.sample_rate < 8_000 {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }
    if !matches!(req.language.as_str(), "auto" | "fr" | "en") {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }

    transcribe_with_fallback(&req, &state)
        .map(Json)
        .map_err(|_| axum::http::StatusCode::SERVICE_UNAVAILABLE)
}

async fn stt_stream_handler(ws: WebSocketUpgrade, State(state): State<SttState>) -> Response {
    ws.on_upgrade(move |socket| stream_session(socket, state))
}

async fn stream_session(mut socket: WebSocket, state: SttState) {
    let partial = serde_json::json!({
        "event": "partial",
        "engine": state.primary_engine,
        "text": "bonjour",
    });
    let final_msg = serde_json::json!({
        "event": "final",
        "engine": state.primary_engine,
        "text": "bonjour ceci est une transcription en streaming",
        "confidence": 0.9
    });
    let _ = socket.send(Message::Text(partial.to_string().into())).await;
    let _ = socket
        .send(Message::Text(final_msg.to_string().into()))
        .await;
    let _ = socket.send(Message::Close(None)).await;
}

fn transcribe_with_fallback(req: &SttRequest, state: &SttState) -> Result<SttResponse, SttError> {
    let preset = req.preset.unwrap_or(SttPreset::Balanced);
    let mut chain = Vec::with_capacity(state.fallback_engines.len() + 1);
    chain.push(state.primary_engine);
    chain.extend(state.fallback_engines.iter().copied());

    for engine in chain {
        if let Ok(response) = transcribe_with_engine(req, preset, engine) {
            return Ok(response);
        }
    }

    Err(SttError::Engine(
        "all local engines failed for this request".to_string(),
    ))
}

fn transcribe_with_engine(
    req: &SttRequest,
    preset: SttPreset,
    engine: SttEngine,
) -> Result<SttResponse, SttError> {
    // Permet de valider la chaine fallback en test/integration.
    if engine == SttEngine::FasterWhisper && req.model.as_deref() == Some("fail-primary") {
        return Err(SttError::Engine("forced primary failure".to_string()));
    }

    let language_detected = detect_language(req);
    let transcript = if language_detected == "en" {
        "this is a placeholder transcription".to_string()
    } else {
        "ceci est une transcription de demonstration".to_string()
    };

    let (confidence, duration_ms) = match engine {
        SttEngine::FasterWhisper => (0.93, 120),
        SttEngine::WhisperCpp => (0.89, 180),
        SttEngine::SherpaOnnx => (0.86, 95),
    };

    Ok(SttResponse {
        transcript,
        confidence,
        duration_ms,
        engine: engine.as_str().to_string(),
        model_used: req
            .model
            .clone()
            .unwrap_or_else(|| engine.default_model(preset).to_string()),
        language_detected,
    })
}

fn detect_language(req: &SttRequest) -> String {
    if req.language != "auto" {
        return req.language.clone();
    }
    if req.languages_hint.iter().any(|lang| lang == "en")
        && !req.languages_hint.iter().any(|lang| lang == "fr")
    {
        "en".to_string()
    } else {
        "fr".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use tower::util::ServiceExt;

    #[test]
    fn preset_selection_is_deterministic() {
        let compact = select_preset(HardwareProfile {
            cpu_cores: 4,
            ram_total_mb: 4096,
            gpu_vram_mb: 0,
        });
        assert_eq!(compact, SttPreset::Compact);
    }

    #[tokio::test]
    async fn health_route_returns_ok() {
        let app = router();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }

    #[tokio::test]
    async fn stream_route_exists_and_requires_upgrade() {
        let app = router();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/stt/stream")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::BAD_REQUEST);
    }

    #[test]
    fn fallback_switches_engine_when_primary_fails() {
        let state = SttState::default();
        let req = SttRequest {
            samples: vec![0.1, -0.1, 0.2, -0.2],
            sample_rate: 16_000,
            language: "fr".to_string(),
            model: Some("fail-primary".to_string()),
            languages_hint: vec!["fr".to_string(), "en".to_string()],
            preset: Some(SttPreset::Balanced),
        };
        let response = transcribe_with_fallback(&req, &state).expect("fallback should succeed");
        assert_eq!(response.engine, "whisper.cpp");
    }
}
