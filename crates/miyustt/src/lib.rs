//! @id     miyustt_lib
//! @do     definir_stt_preset
//! @role   orchestration
//! @layer  core
//! @human  miyustt — module « lib ».

//! MiyuSTT toolkit (V1 initiale).
//!
//! Fournit:
//! - types API STT v1
//! - selection de preset hardware
//! - routeur HTTP local (`/api/health`, `/api/stt`, `/api/stt/stream`)
//!
//! Securite (R-010, lot 2 #10) : requetes signees par des appelants epingles
//! (aucun secret partage ; usage [`USAGE`]), origine locale stricte, corps borne,
//! et [`validate_bind`] qui refuse une ecoute hors loopback sans appelant epingle.
//! Le controle s'execute en middleware : en-tetes verifies avant la lecture du corps.

mod securite;

use std::net::SocketAddr;
use std::sync::Arc;

use miyauth::http::service::GardeSignee;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        DefaultBodyLimit, Request, State,
    },
    http::StatusCode,
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

pub use securite::{is_local_origin, SecurityConfigError, USAGE};

/// Taille max du corps HTTP `/api/stt` (octets). Au-dela : 413.
pub const MAX_STT_BODY_BYTES: usize = 16 * 1024 * 1024;
/// Duree audio max acceptee par requete (secondes).
pub const MAX_AUDIO_SECONDS: u32 = 60;
/// Frequence d'echantillonnage minimale acceptee (Hz).
pub const MIN_SAMPLE_RATE: u32 = 8_000;
/// Frequence d'echantillonnage maximale acceptee (Hz).
pub const MAX_SAMPLE_RATE: u32 = 48_000;
/// Nombre max d'indices de langue.
pub const MAX_LANGUAGE_HINTS: usize = 8;
/// Longueur max d'un identifiant court (langue, modele).
pub const MAX_IDENT_LEN: usize = 128;
/// Taille max d'un message WebSocket entrant (octets).
pub const MAX_WS_MESSAGE_BYTES: usize = 64 * 1024;

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
            (Self::FasterWhisper, SttPreset::Balanced) => "small-int8",
            (Self::FasterWhisper, SttPreset::Precision) => "medium-fp16",
            (Self::FasterWhisper, SttPreset::Compact | SttPreset::SafeFallback) => "base-int8",
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
    /// Configuration de securite endpoint.
    pub security: SttSecurity,
}

/// Configuration securite de MiyuSTT.
///
/// `appelants` : garde des requetes signees (cles publiques des appelants
/// epingles, usage [`USAGE`]), exigee sur toute route sauf `/api/health`, y
/// compris en loopback. Aucune cle epinglee : toute requete protegee est
/// refusee (503). Hors loopback, au moins une cle est exigee ([`validate_bind`]).
#[derive(Clone, Default, Debug)]
pub struct SttSecurity {
    /// Appelants autorises (cles epinglees + registre anti-rejeu).
    pub appelants: Arc<GardeSignee>,
    /// Restreint les requetes avec `Origin` non-local.
    pub enforce_local_origin: bool,
}

impl SttSecurity {
    /// Securite avec les cles publiques Ed25519 des appelants autorises.
    pub fn avec_appelants(cles: impl IntoIterator<Item = [u8; 32]>, enforce_local_origin: bool) -> Self {
        Self {
            appelants: Arc::new(GardeSignee::new(cles)),
            enforce_local_origin,
        }
    }
}

/// Valide l'adresse d'ecoute au regard de la configuration de securite.
///
/// A appeler avant d'ouvrir le port. Refuse (fail-closed) :
/// - une adresse non analysable comme `IP:port` ;
/// - une adresse non-loopback sans appelant epingle.
///
/// # Errors
/// [`SecurityConfigError`] decrivant la cause du refus.
pub fn validate_bind(
    bind_addr: &str,
    security: &SttSecurity,
) -> Result<SocketAddr, SecurityConfigError> {
    securite::check_bind(bind_addr, &security.appelants)
}

impl Default for SttState {
    fn default() -> Self {
        Self {
            primary_engine: SttEngine::FasterWhisper,
            fallback_engines: vec![SttEngine::WhisperCpp, SttEngine::SherpaOnnx],
            security: SttSecurity {
                appelants: Arc::default(),
                enforce_local_origin: true,
            },
        }
    }
}

/// Construit un routeur HTTP MiyuSTT v1.
pub fn router() -> Router {
    router_with_state(SttState::default())
}

/// Construit un routeur HTTP MiyuSTT v1 avec etat personnalise.
///
/// `/api/health` reste public ; les autres routes passent par le controle
/// origine + requete signee (en-tetes avant lecture du corps).
pub fn router_with_state(state: SttState) -> Router {
    let protected = Router::new()
        .route("/api/stt", post(stt_handler))
        .route("/api/stt/stream", get(stt_stream_handler))
        .route_layer(middleware::from_fn_with_state(
            state.security.clone(),
            security_middleware,
        ))
        .layer(DefaultBodyLimit::max(MAX_STT_BODY_BYTES))
        .with_state(state);
    Router::new()
        .route("/api/health", get(health_handler))
        .merge(protected)
}

async fn security_middleware(
    State(security): State<SttSecurity>,
    req: Request,
    next: Next,
) -> Response {
    match securite::check_request(
        req,
        &security.appelants,
        security.enforce_local_origin,
        MAX_STT_BODY_BYTES,
    )
    .await
    {
        Ok(req) => next.run(req).await,
        Err(status) => status.into_response(),
    }
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "miyustt",
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

/// Validation bornee d'une requete STT (C-13).
fn validate_request(req: &SttRequest) -> Result<(), StatusCode> {
    if req.samples.is_empty() || !(MIN_SAMPLE_RATE..=MAX_SAMPLE_RATE).contains(&req.sample_rate) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let max_samples = u64::from(req.sample_rate) * u64::from(MAX_AUDIO_SECONDS);
    if req.samples.len() as u64 > max_samples {
        return Err(StatusCode::PAYLOAD_TOO_LARGE);
    }
    if !matches!(req.language.as_str(), "auto" | "fr" | "en") {
        return Err(StatusCode::BAD_REQUEST);
    }
    if req.languages_hint.len() > MAX_LANGUAGE_HINTS
        || req.languages_hint.iter().any(|h| h.len() > MAX_IDENT_LEN)
        || req.model.as_ref().is_some_and(|m| m.len() > MAX_IDENT_LEN)
    {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(())
}

async fn stt_handler(
    State(state): State<SttState>,
    Json(mut req): Json<SttRequest>,
) -> Result<Json<SttResponse>, StatusCode> {
    if let Err(status) = validate_request(&req) {
        req.samples.fill(0.0);
        return Err(status);
    }

    let response = transcribe_with_fallback(&req, &state);

    // Purge best-effort des buffers audio apres traitement.
    req.samples.fill(0.0);

    response
        .map(Json)
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)
}

async fn stt_stream_handler(ws: WebSocketUpgrade, State(state): State<SttState>) -> Response {
    ws.max_message_size(MAX_WS_MESSAGE_BYTES)
        .max_frame_size(MAX_WS_MESSAGE_BYTES)
        .on_upgrade(move |socket| stream_session(socket, state))
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
    use serde_json::json;
    use tower::util::ServiceExt;
    use miyauth::http::service::entetes_signees;
    use miyusceau::CleSignature;

    /// Cle de l'appelant de test.
    fn test_key() -> CleSignature {
        CleSignature::depuis_graine(&[7_u8; 32])
    }

    /// Securite epinglant la cle de test.
    fn test_security(enforce_local_origin: bool) -> SttSecurity {
        SttSecurity::avec_appelants([test_key().cle_publique()], enforce_local_origin)
    }

    /// Signe une requete avec `cle` (corps lu puis remis).
    async fn sign_with(cle: &CleSignature, req: Request<Body>) -> Request<Body> {
        let (mut parts, body) = req.into_parts();
        let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
        let path = parts.uri.path_and_query().unwrap().as_str().to_string();
        for (name, value) in entetes_signees(cle, USAGE, parts.method.as_str(), &path, &bytes) {
            parts.headers.insert(name, value.parse().unwrap());
        }
        Request::from_parts(parts, Body::from(bytes))
    }

    async fn signed(req: Request<Body>) -> Request<Body> {
        sign_with(&test_key(), req).await
    }

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
        let app = secured_app(false);
        let req = Request::builder()
            .uri("/api/stt/stream")
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(signed(req).await).await.unwrap();
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

    #[tokio::test]
    async fn stt_requires_signature_when_configured() {
        let status = post_stt_raw(secured_app(false), &[], stt_body()).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        // Ancien jeton bearer : sans effet.
        let status = post_stt_raw(secured_app(false), &[("authorization", "Bearer secret")], stt_body()).await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn no_pinned_caller_refuses_everything() {
        // Pas de cle epinglee : refus (503), meme signee, meme en loopback.
        let app = router_with_state(SttState {
            security: SttSecurity { enforce_local_origin: false, ..SttSecurity::default() },
            ..SttState::default()
        });
        assert_eq!(post_stt(app.clone(), &[], stt_body()).await, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(post_stt_raw(app, &[], stt_body()).await, StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn stt_rejects_non_local_origin_by_default() {
        let app = router();
        let body = serde_json::json!({
            "samples": [0.1, -0.1],
            "sample_rate": 16000,
            "language": "fr",
            "languages_hint": ["fr", "en"]
        })
        .to_string();
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/stt")
                    .header("content-type", "application/json")
                    .header("origin", "https://evil.example")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn stt_accepts_signed_request_with_local_origin() {
        let status = post_stt(secured_app(true), &[("origin", "http://localhost:8080")], stt_body()).await;
        assert_eq!(status, StatusCode::OK);
    }

    fn secured_app(enforce_local_origin: bool) -> Router {
        router_with_state(SttState {
            security: test_security(enforce_local_origin),
            ..SttState::default()
        })
    }

    fn stt_body() -> String {
        json!({
            "samples": [0.1, -0.1],
            "sample_rate": 16000,
            "language": "fr",
            "languages_hint": ["fr"]
        })
        .to_string()
    }

    fn stt_request(headers: &[(&str, &str)], body: String) -> Request<Body> {
        let mut builder = Request::builder()
            .method("POST")
            .uri("/api/stt")
            .header("content-type", "application/json");
        for (k, v) in headers {
            builder = builder.header(*k, *v);
        }
        builder.body(Body::from(body)).unwrap()
    }

    /// POST `/api/stt` signe par la cle de test.
    async fn post_stt(app: Router, headers: &[(&str, &str)], body: String) -> StatusCode {
        let req = signed(stt_request(headers, body)).await;
        app.oneshot(req).await.unwrap().status()
    }

    /// POST `/api/stt` non signe.
    async fn post_stt_raw(app: Router, headers: &[(&str, &str)], body: String) -> StatusCode {
        app.oneshot(stt_request(headers, body)).await.unwrap().status()
    }

    #[test]
    fn startup_refuses_non_loopback_without_caller() {
        let err = validate_bind("0.0.0.0:3003", &SttSecurity::default()).unwrap_err();
        assert!(matches!(err, SecurityConfigError::CallerRequired(_)));
        assert!(validate_bind("0.0.0.0:3003", &test_security(true)).is_ok());
        assert!(validate_bind("127.0.0.1:3003", &SttSecurity::default()).is_ok());
    }

    #[tokio::test]
    async fn unknown_key_replay_stale_and_tampered_are_401() {
        let app = secured_app(false);
        // Cle inconnue.
        let other = CleSignature::generer();
        let req = sign_with(&other, stt_request(&[], stt_body())).await;
        assert_eq!(app.clone().oneshot(req).await.unwrap().status(), StatusCode::UNAUTHORIZED);
        // Rejeu de la meme requete signee.
        let req = signed(stt_request(&[], stt_body())).await;
        let headers = req.headers().clone();
        assert_eq!(app.clone().oneshot(req).await.unwrap().status(), StatusCode::OK);
        let mut replay = stt_request(&[], stt_body());
        replay.headers_mut().extend(headers.clone());
        assert_eq!(app.clone().oneshot(replay).await.unwrap().status(), StatusCode::UNAUTHORIZED);
        // Corps altere.
        let req = signed(stt_request(&[], stt_body())).await;
        let mut tampered = stt_request(&[], stt_body().replace("0.1", "0.2"));
        tampered.headers_mut().extend(req.headers().clone());
        assert_eq!(app.clone().oneshot(tampered).await.unwrap().status(), StatusCode::UNAUTHORIZED);
        // Horodatage perime.
        let old = miyauth::http::signer_requete(
            &test_key(),
            USAGE,
            "POST",
            "/api/stt",
            stt_body().as_bytes(),
            miyauth::http::service::maintenant_unix() - 600,
        );
        let mut stale = stt_request(&[], stt_body());
        for (name, value) in old.en_entetes() {
            stale.headers_mut().insert(name, value.parse().unwrap());
        }
        assert_eq!(app.oneshot(stale).await.unwrap().status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn signed_request_is_200() {
        let status = post_stt(secured_app(false), &[], stt_body()).await;
        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn lookalike_origin_is_refused() {
        for origin in [
            "http://localhost.evil.com",
            "http://127.0.0.1.evil.com:8080",
        ] {
            let status = post_stt(
                secured_app(true),
                &[("origin", origin)],
                stt_body(),
            )
            .await;
            assert_eq!(status, StatusCode::FORBIDDEN, "{origin}");
        }
    }

    #[tokio::test]
    async fn local_origin_with_port_is_accepted() {
        let status = post_stt(
            secured_app(true),
            &[("origin", "http://localhost:5173")],
            stt_body(),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn oversize_body_is_413() {
        let big = format!(
            "{{\"samples\":[{}0],\"sample_rate\":16000,\"language\":\"fr\"}}",
            "0,".repeat(MAX_STT_BODY_BYTES / 2 + 16)
        );
        let status = post_stt(
            secured_app(false),
            &[],
            big,
        )
        .await;
        assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
    }

    #[tokio::test]
    async fn too_long_audio_is_413() {
        let samples = vec![0.0_f32; (8_000 * MAX_AUDIO_SECONDS + 1) as usize];
        let body = json!({ "samples": samples, "sample_rate": 8000, "language": "fr" }).to_string();
        let status = post_stt(
            secured_app(false),
            &[],
            body,
        )
        .await;
        assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
    }

    #[tokio::test]
    async fn stream_requires_signature_when_configured() {
        let response = secured_app(false)
            .oneshot(
                Request::builder()
                    .uri("/api/stt/stream")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn health_stays_public() {
        let response = secured_app(true)
            .oneshot(
                Request::builder()
                    .uri("/api/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }



    #[tokio::test]
    async fn latency_snapshot_three_presets() {
        let app = router_with_state(SttState {
            security: test_security(false),
            ..SttState::default()
        });

        for preset in ["compact", "balanced", "precision"] {
            let body = json!({
                "samples": [0.1, -0.1, 0.05, -0.02],
                "sample_rate": 16000,
                "language": "fr",
                "preset": preset,
                "languages_hint": ["fr", "en"]
            })
            .to_string();

            let req = signed(
                Request::builder()
                    .method("POST")
                    .uri("/api/stt")
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await;
            let start = std::time::Instant::now();
            let response = app.clone().oneshot(req).await.unwrap();
            let elapsed_ms = start.elapsed().as_millis();
            println!("stt_latency_ms preset={preset}: {elapsed_ms}");
            assert_eq!(response.status(), StatusCode::OK);
            assert!(elapsed_ms < 200);
        }
    }
}
