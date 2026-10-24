//! @id     miyutts_lib
//! @do     definir_tts_preset
//! @role   orchestration
//! @layer  core
//! @human  miyutts — module « lib ».

//! MiyuTTS toolkit (V1 initiale).
//!
//! Fournit:
//! - types API TTS v1
//! - selection de preset hardware
//! - routeur HTTP local (`/api/health`, `/api/tts/voices`, `/api/tts`, `/api/tts/wav`)
//!
//! Securite (R-010) : jeton bearer compare en temps constant, origine locale
//! stricte, corps borne, et [`validate_bind`] qui refuse une ecoute hors loopback
//! sans jeton. Le controle s'execute en middleware, avant la lecture du corps.

mod securite;

use std::net::SocketAddr;

use axum::{
    extract::{DefaultBodyLimit, Request, State},
    http::{header, HeaderValue, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use base64::Engine as _;
use serde::{Deserialize, Serialize};

pub use securite::{is_local_origin, SecurityConfigError};

/// Taille max du corps HTTP des routes TTS (octets). Au-dela : 413.
pub const MAX_TTS_BODY_BYTES: usize = 64 * 1024;
/// Nombre max de caracteres de texte a synthetiser.
pub const MAX_TTS_TEXT_CHARS: usize = 4_000;
/// Longueur max d'un identifiant court (voix, format, langue).
pub const MAX_IDENT_LEN: usize = 128;

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
    /// Configuration securite endpoint.
    pub security: TtsSecurity,
}

/// Configuration securite de MiyuTTS.
///
/// `bearer_token` : s'il est present, il est exige sur toute route sauf `/api/health`,
/// y compris en loopback. Son absence n'est admise qu'en ecoute loopback
/// (voir [`validate_bind`]). Un jeton vide refuse toute requete.
#[derive(Clone, Default)]
pub struct TtsSecurity {
    /// Token bearer attendu (obligatoire hors loopback).
    pub bearer_token: Option<String>,
    /// Restreint les requetes avec `Origin` non-local.
    pub enforce_local_origin: bool,
}

impl std::fmt::Debug for TtsSecurity {
    // Le jeton n'est jamais affiche (C-25).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TtsSecurity")
            .field(
                "bearer_token",
                &self.bearer_token.as_ref().map(|_| "<masque>"),
            )
            .field("enforce_local_origin", &self.enforce_local_origin)
            .finish()
    }
}

/// Valide l'adresse d'ecoute au regard de la configuration de securite.
///
/// A appeler avant d'ouvrir le port. Refuse (fail-closed) :
/// - une adresse non analysable comme `IP:port` ;
/// - un jeton configure mais vide ;
/// - une adresse non-loopback sans jeton.
///
/// # Errors
/// [`SecurityConfigError`] decrivant la cause du refus.
pub fn validate_bind(
    bind_addr: &str,
    security: &TtsSecurity,
) -> Result<SocketAddr, SecurityConfigError> {
    securite::check_bind(bind_addr, security.bearer_token.as_deref())
}

impl Default for TtsState {
    fn default() -> Self {
        Self {
            primary_engine: TtsEngine::KokoroOnnx,
            fallback_engine: TtsEngine::EspeakNg,
            security: TtsSecurity {
                bearer_token: None,
                enforce_local_origin: true,
            },
        }
    }
}

/// Construit un routeur HTTP MiyuTTS v1.
pub fn router() -> Router {
    router_with_state(TtsState::default())
}

/// Construit un routeur HTTP MiyuTTS v1 avec etat personnalise.
///
/// `/api/health` reste public ; les autres routes passent par le controle
/// origine + jeton avant toute lecture du corps.
pub fn router_with_state(state: TtsState) -> Router {
    let protected = Router::new()
        .route("/api/tts/voices", get(voices_handler))
        .route("/api/tts", post(tts_handler))
        .route("/api/tts/wav", post(tts_wav_handler))
        .route_layer(middleware::from_fn_with_state(
            state.security.clone(),
            security_middleware,
        ))
        .layer(DefaultBodyLimit::max(MAX_TTS_BODY_BYTES))
        .with_state(state);
    Router::new()
        .route("/api/health", get(health_handler))
        .merge(protected)
}

async fn security_middleware(
    State(security): State<TtsSecurity>,
    req: Request,
    next: Next,
) -> Response {
    match securite::check_request(
        req.headers(),
        security.bearer_token.as_deref(),
        security.enforce_local_origin,
    ) {
        Ok(()) => next.run(req).await,
        Err(status) => status.into_response(),
    }
}

/// Validation bornee d'une requete TTS (C-13).
fn validate_request(req: &TtsRequest) -> Result<(), StatusCode> {
    if req.text.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    if req.text.chars().count() > MAX_TTS_TEXT_CHARS {
        return Err(StatusCode::PAYLOAD_TOO_LARGE);
    }
    if req.voice.len() > MAX_IDENT_LEN
        || req.format.len() > MAX_IDENT_LEN
        || req.language.len() > MAX_IDENT_LEN
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(())
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
    Json(mut req): Json<TtsRequest>,
) -> Result<Json<TtsResponse>, axum::http::StatusCode> {
    if let Err(status) = validate_request(&req) {
        scrub_text(&mut req.text);
        return Err(status);
    }

    let synthesized =
        synthesize_with_fallback(&req, &state).map_err(|_| axum::http::StatusCode::BAD_REQUEST);

    scrub_text(&mut req.text);
    let synthesized = synthesized?;

    Ok(Json(TtsResponse {
        audio_base64: base64::engine::general_purpose::STANDARD.encode(synthesized.audio),
        engine: synthesized.engine,
        voice_used: synthesized.voice_used,
        duration_ms: synthesized.duration_ms,
    }))
}

async fn tts_wav_handler(
    State(state): State<TtsState>,
    Json(mut req): Json<TtsRequest>,
) -> Result<Response, StatusCode> {
    if let Err(status) = validate_request(&req) {
        scrub_text(&mut req.text);
        return Err(status);
    }
    let synthesized = synthesize_with_fallback(&req, &state).map_err(|_| StatusCode::BAD_REQUEST);

    scrub_text(&mut req.text);
    let synthesized = synthesized?;

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

fn scrub_text(text: &mut String) {
    if text.is_empty() {
        return;
    }
    // Purge best-effort du buffer en memoire.
    let mask = " ".repeat(text.len());
    text.replace_range(.., &mask);
    text.clear();
    text.shrink_to_fit();
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

    #[tokio::test]
    async fn tts_requires_bearer_when_configured() {
        let app = router_with_state(TtsState {
            security: TtsSecurity {
                bearer_token: Some("secret".to_string()),
                enforce_local_origin: false,
            },
            ..TtsState::default()
        });
        let req = Request::builder()
            .method("POST")
            .uri("/api/tts")
            .header("content-type", "application/json")
            .body(Body::from(
                json!({
                    "text": "Bonjour",
                    "voice": "fr_female_01_compact",
                    "format": "wav",
                    "language": "fr"
                })
                .to_string(),
            ))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn tts_rejects_non_local_origin_by_default() {
        let app = router();
        let req = Request::builder()
            .method("POST")
            .uri("/api/tts")
            .header("content-type", "application/json")
            .header("origin", "https://evil.example")
            .body(Body::from(
                json!({
                    "text": "Bonjour",
                    "voice": "fr_female_01_compact",
                    "format": "wav",
                    "language": "fr"
                })
                .to_string(),
            ))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn tts_accepts_valid_bearer_and_local_origin() {
        let app = router_with_state(TtsState {
            security: TtsSecurity {
                bearer_token: Some("secret".to_string()),
                enforce_local_origin: true,
            },
            ..TtsState::default()
        });
        let req = Request::builder()
            .method("POST")
            .uri("/api/tts")
            .header("content-type", "application/json")
            .header("origin", "http://localhost:8080")
            .header("authorization", "Bearer secret")
            .body(Body::from(
                json!({
                    "text": "Bonjour",
                    "voice": "fr_female_01_compact",
                    "format": "wav",
                    "language": "fr"
                })
                .to_string(),
            ))
            .unwrap();

        let response = app.oneshot(req).await.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }

    fn secured_app(enforce_local_origin: bool) -> Router {
        router_with_state(TtsState {
            security: TtsSecurity {
                bearer_token: Some("secret".to_string()),
                enforce_local_origin,
            },
            ..TtsState::default()
        })
    }

    fn tts_body(text: &str) -> String {
        json!({
            "text": text,
            "voice": "fr_female_01_compact",
            "format": "wav",
            "language": "fr"
        })
        .to_string()
    }

    async fn post_tts(app: Router, headers: &[(&str, &str)], body: String) -> StatusCode {
        let mut builder = Request::builder()
            .method("POST")
            .uri("/api/tts")
            .header("content-type", "application/json");
        for (k, v) in headers {
            builder = builder.header(*k, *v);
        }
        app.oneshot(builder.body(Body::from(body)).unwrap())
            .await
            .unwrap()
            .status()
    }

    #[test]
    fn startup_refuses_non_loopback_without_token() {
        let err = validate_bind("0.0.0.0:3004", &TtsSecurity::default()).unwrap_err();
        assert!(matches!(err, SecurityConfigError::TokenRequired(_)));
        let ok = validate_bind(
            "0.0.0.0:3004",
            &TtsSecurity {
                bearer_token: Some("secret".into()),
                enforce_local_origin: true,
            },
        );
        assert!(ok.is_ok());
        assert!(validate_bind("127.0.0.1:3004", &TtsSecurity::default()).is_ok());
    }

    #[tokio::test]
    async fn bad_token_is_401() {
        let status = post_tts(
            secured_app(false),
            &[("authorization", "Bearer secreu")],
            tts_body("Bonjour"),
        )
        .await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn good_token_is_200() {
        let status = post_tts(
            secured_app(false),
            &[("authorization", "Bearer secret")],
            tts_body("Bonjour"),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn lookalike_origin_is_refused() {
        for origin in ["http://localhost.evil.com", "http://[::1].evil.com"] {
            let status = post_tts(
                secured_app(true),
                &[("origin", origin), ("authorization", "Bearer secret")],
                tts_body("Bonjour"),
            )
            .await;
            assert_eq!(status, StatusCode::FORBIDDEN, "{origin}");
        }
    }

    #[tokio::test]
    async fn local_origin_with_port_is_accepted() {
        let status = post_tts(
            secured_app(true),
            &[
                ("origin", "http://localhost:5173"),
                ("authorization", "Bearer secret"),
            ],
            tts_body("Bonjour"),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn oversize_body_is_413() {
        let text = "a".repeat(MAX_TTS_BODY_BYTES + 1);
        let status = post_tts(
            secured_app(false),
            &[("authorization", "Bearer secret")],
            tts_body(&text),
        )
        .await;
        assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
    }

    #[tokio::test]
    async fn too_long_text_is_413() {
        let text = "a".repeat(MAX_TTS_TEXT_CHARS + 1);
        let status = post_tts(
            secured_app(false),
            &[("authorization", "Bearer secret")],
            tts_body(&text),
        )
        .await;
        assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
    }

    #[tokio::test]
    async fn voices_require_token_and_health_stays_public() {
        let voices = secured_app(false)
            .oneshot(
                Request::builder()
                    .uri("/api/tts/voices")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(voices.status(), StatusCode::UNAUTHORIZED);
        let health = secured_app(true)
            .oneshot(
                Request::builder()
                    .uri("/api/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(health.status(), StatusCode::OK);
    }

    #[test]
    fn debug_masks_token() {
        let sec = TtsSecurity {
            bearer_token: Some("super-secret".into()),
            enforce_local_origin: true,
        };
        let shown = format!(
            "{:?}",
            TtsState {
                security: sec,
                ..TtsState::default()
            }
        );
        assert!(!shown.contains("super-secret"));
    }

    #[tokio::test]
    async fn latency_snapshot_three_presets() {
        let app = router_with_state(TtsState {
            security: TtsSecurity {
                bearer_token: None,
                enforce_local_origin: false,
            },
            ..TtsState::default()
        });

        for preset in ["compact", "balanced", "precision"] {
            let req = Request::builder()
                .method("POST")
                .uri("/api/tts")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "text": "Bonjour Miyukini",
                        "voice": "fr_female_01_compact",
                        "format": "wav",
                        "language": "fr",
                        "preset": preset
                    })
                    .to_string(),
                ))
                .unwrap();

            let start = std::time::Instant::now();
            let response = app.clone().oneshot(req).await.unwrap();
            let elapsed_ms = start.elapsed().as_millis();
            println!("tts_latency_ms preset={preset}: {elapsed_ms}");
            assert_eq!(response.status(), StatusCode::OK);
            assert!(elapsed_ms < 200);
        }
    }
}
