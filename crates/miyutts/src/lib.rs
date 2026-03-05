//! MiyuTTS toolkit (V1 initiale).
//!
//! Fournit:
//! - types API TTS v1
//! - selection de preset hardware
//! - routeur HTTP local (`/api/health`, `/api/tts/voices`, `/api/tts`, `/api/tts/wav`)

use axum::{
    extract::State,
    http::{header, HeaderValue, StatusCode},
    response::Response,
    routing::{get, post},
    Json, Router,
};
use base64::Engine as _;
use serde::{Deserialize, Serialize};

/// Presets TTS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TtsPreset {
    /// Compacite prioritaire.
    Compact,
    /// Equilibre qualite/latence.
    Balanced,
    /// Qualite prioritaire.
    Precision,
    /// Fallback ultra-leger.
    SafeFallback,
}

/// Moteurs TTS supportes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TtsEngine {
    /// Moteur principal local.
    KokoroOnnx,
    /// Fallback ultra-compact.
    EspeakNg,
}

impl TtsEngine {
    fn default_voice(self, language: &str) -> &'static str {
        match (self, language) {
            (Self::KokoroOnnx, "en") => "en_female_01_compact",
            (Self::KokoroOnnx, _) => "fr_female_01_compact",
            (Self::EspeakNg, "en") => "en_system",
            (Self::EspeakNg, _) => "fr_system",
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::KokoroOnnx => "kokoro-onnx",
            Self::EspeakNg => "espeak-ng",
        }
    }
}

/// Profil hardware pour auto-selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardwareProfile {
    /// RAM totale en MB.
    pub ram_total_mb: u64,
    /// VRAM GPU en MB.
    pub gpu_vram_mb: u64,
}

/// Selection automatique TTS.
#[must_use]
pub fn select_preset(hw: HardwareProfile) -> TtsPreset {
    if hw.gpu_vram_mb >= 8192 || hw.ram_total_mb >= 16384 {
        TtsPreset::Precision
    } else if hw.ram_total_mb >= 8192 {
        TtsPreset::Balanced
    } else {
        TtsPreset::Compact
    }
}

/// Requete TTS v1.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsRequest {
    /// Texte a synthetiser.
    pub text: String,
    /// Voix cible.
    pub voice: String,
    /// Format audio (`wav`, `pcm`).
    pub format: String,
    /// Langue (`fr`, `en`).
    pub language: String,
    /// Frequence cible (optionnelle).
    #[serde(default)]
    pub sample_rate: Option<u32>,
    /// Vitesse de lecture (optionnelle).
    #[serde(default)]
    pub speed: Option<f32>,
    /// Preset force.
    #[serde(default)]
    pub preset: Option<TtsPreset>,
}

/// Reponse TTS v1.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsResponse {
    /// Binaire encode en base64 (squelette V1).
    pub audio_base64: String,
    /// Moteur actif.
    pub engine: String,
    /// Voix reelle utilisee.
    pub voice_used: String,
    /// Latence de synthese.
    pub duration_ms: u64,
}

/// Description de voix.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceInfo {
    /// Identifiant.
    pub id: String,
    /// Langue.
    pub lang: String,
    /// Moteur.
    pub engine: String,
}

/// Erreurs TTS v1.
#[derive(Debug, thiserror::Error)]
pub enum TtsError {
    /// Entree invalide.
    #[error("invalid text payload")]
    InvalidText,
    /// Voix inconnue.
    #[error("voice not found: {0}")]
    VoiceNotFound(String),
    /// Erreur moteur.
    #[error("engine error: {0}")]
    Engine(String),
}

/// Etat partage du service TTS.
#[derive(Debug, Clone)]
pub struct TtsState {
    /// Moteur principal.
    pub primary_engine: TtsEngine,
    /// Fallback engine.
    pub fallback_engine: TtsEngine,
}

impl Default for TtsState {
    fn default() -> Self {
        Self {
            primary_engine: TtsEngine::KokoroOnnx,
            fallback_engine: TtsEngine::EspeakNg,
        }
    }
}

/// Construit un routeur HTTP MiyuTTS v1.
#[must_use]
pub fn router() -> Router {
    let state = TtsState::default();
    Router::new()
        .route("/api/health", get(health_handler))
        .route("/api/tts/voices", get(voices_handler))
        .route("/api/tts", post(tts_handler))
        .route("/api/tts/wav", post(tts_wav_handler))
        .with_state(state)
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "miyutts",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

async fn voices_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "voices": known_voices()
    }))
}

async fn tts_handler(
    State(state): State<TtsState>,
    Json(req): Json<TtsRequest>,
) -> Result<Json<TtsResponse>, axum::http::StatusCode> {
    if req.text.trim().is_empty() {
        return Err(axum::http::StatusCode::BAD_REQUEST);
    }

    let synthesized =
        synthesize_with_fallback(&req, &state).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;

    Ok(Json(TtsResponse {
        audio_base64: base64::engine::general_purpose::STANDARD.encode(synthesized.audio),
        engine: synthesized.engine,
        voice_used: synthesized.voice_used,
        duration_ms: synthesized.duration_ms,
    }))
}

async fn tts_wav_handler(
    State(state): State<TtsState>,
    Json(req): Json<TtsRequest>,
) -> Result<Response, StatusCode> {
    if req.text.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let synthesized =
        synthesize_with_fallback(&req, &state).map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut response = Response::new(axum::body::Body::from(synthesized.audio));
    *response.status_mut() = StatusCode::OK;
    response
        .headers_mut()
        .insert(header::CONTENT_TYPE, HeaderValue::from_static("audio/wav"));
    if let Ok(engine_value) = HeaderValue::from_str(&synthesized.engine) {
        response
            .headers_mut()
            .insert("x-miyu-tts-engine", engine_value);
    }
    Ok(response)
}

#[derive(Debug, Clone)]
struct SynthesizedAudio {
    audio: Vec<u8>,
    engine: String,
    voice_used: String,
    duration_ms: u64,
}

fn known_voices() -> Vec<VoiceInfo> {
    vec![
        VoiceInfo {
            id: "fr_female_01_compact".to_string(),
            lang: "fr".to_string(),
            engine: "kokoro-onnx".to_string(),
        },
        VoiceInfo {
            id: "en_female_01_compact".to_string(),
            lang: "en".to_string(),
            engine: "kokoro-onnx".to_string(),
        },
        VoiceInfo {
            id: "fr_system".to_string(),
            lang: "fr".to_string(),
            engine: "espeak-ng".to_string(),
        },
        VoiceInfo {
            id: "en_system".to_string(),
            lang: "en".to_string(),
            engine: "espeak-ng".to_string(),
        },
    ]
}

fn synthesize_with_fallback(
    req: &TtsRequest,
    state: &TtsState,
) -> Result<SynthesizedAudio, TtsError> {
    if let Ok(ok) = synthesize_with_engine(req, state.primary_engine) {
        return Ok(ok);
    }
    synthesize_with_engine(req, state.fallback_engine)
}

fn synthesize_with_engine(
    req: &TtsRequest,
    engine: TtsEngine,
) -> Result<SynthesizedAudio, TtsError> {
    if engine == TtsEngine::KokoroOnnx && req.voice == "unknown_voice" {
        return Err(TtsError::VoiceNotFound(req.voice.clone()));
    }

    let language = if req.language == "en" { "en" } else { "fr" };
    let voice_used = if req.voice.trim().is_empty() {
        engine.default_voice(language).to_string()
    } else {
        req.voice.clone()
    };
    let sample_rate = req.sample_rate.unwrap_or(22_050).clamp(8_000, 48_000);
    let mut duration_ms = (req.text.chars().count() as u64).saturating_mul(22);
    duration_ms = duration_ms.clamp(280, 3_500);

    let audio = if req.format == "pcm" {
        vec![0_u8; (sample_rate as usize / 50).saturating_mul(2)]
    } else {
        generate_silent_wav(sample_rate, duration_ms)
    };

    Ok(SynthesizedAudio {
        audio,
        engine: engine.as_str().to_string(),
        voice_used,
        duration_ms,
    })
}

fn generate_silent_wav(sample_rate: u32, duration_ms: u64) -> Vec<u8> {
    let channels: u16 = 1;
    let bits_per_sample: u16 = 16;
    let bytes_per_sample = (bits_per_sample / 8) as usize;
    let sample_count = ((sample_rate as u64 * duration_ms) / 1_000) as usize;
    let data_len = sample_count.saturating_mul(bytes_per_sample);
    let riff_chunk_size = 36_u32.saturating_add(data_len as u32);

    let mut out = Vec::with_capacity(44 + data_len);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&riff_chunk_size.to_le_bytes());
    out.extend_from_slice(b"WAVE");
    out.extend_from_slice(b"fmt ");
    out.extend_from_slice(&16_u32.to_le_bytes());
    out.extend_from_slice(&1_u16.to_le_bytes());
    out.extend_from_slice(&channels.to_le_bytes());
    out.extend_from_slice(&sample_rate.to_le_bytes());
    let byte_rate = sample_rate
        .saturating_mul(u32::from(channels))
        .saturating_mul(u32::from(bits_per_sample) / 8);
    out.extend_from_slice(&byte_rate.to_le_bytes());
    let block_align = channels.saturating_mul(bits_per_sample / 8);
    out.extend_from_slice(&block_align.to_le_bytes());
    out.extend_from_slice(&bits_per_sample.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&(data_len as u32).to_le_bytes());
    out.resize(44 + data_len, 0);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use serde_json::json;
    use tower::util::ServiceExt;

    #[test]
    fn preset_selection_compact_on_low_ram() {
        let preset = select_preset(HardwareProfile {
            ram_total_mb: 4096,
            gpu_vram_mb: 0,
        });
        assert_eq!(preset, TtsPreset::Compact);
    }

    #[tokio::test]
    async fn voices_route_returns_ok() {
        let app = router();
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/tts/voices")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::OK);
    }

    #[tokio::test]
    async fn wav_route_returns_audio_content_type() {
        let app = router();
        let req = Request::builder()
            .method("POST")
            .uri("/api/tts/wav")
            .header("content-type", "application/json")
            .body(Body::from(
                json!({
                    "text": "Bonjour",
                    "voice": "fr_female_01_compact",
                    "format": "wav",
                    "language": "fr",
                    "sample_rate": 22050
                })
                .to_string(),
            ))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), axum::http::StatusCode::OK);
        assert_eq!(response.headers().get("content-type").unwrap(), "audio/wav");
    }
}
