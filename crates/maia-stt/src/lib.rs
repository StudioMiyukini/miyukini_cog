//! MAIA-STT — Moteur de transcription vocale (whisper.cpp via whisper-rs).
//!
//! # Architecture
//!
//! ```text
//! Audio PCM (f32, any Hz, mono) → resample 16kHz → VAD → normalize → whisper-rs → TranscriptResult
//! ```
//!
//! # Modules
//!
//! - `vad` : Voice Activity Detection avec padding et silence timeout
//! - `confidence` : Score log-prob → OK / WARN / REJECT
//! - `whisper` : Wrapper whisper-rs (feature "whisper" requise pour l'inférence réelle)
//! - `pipeline` : Pipeline complet audio → transcript
//!
//! # Features
//!
//! - `whisper` (optionnel) : Active whisper.cpp via whisper-rs. Nécessite libclang + C++ compiler.
//!   Sans cette feature, `SttPipeline::process()` retourne `SttError::ModelNotLoaded`.
//!
//! # Note sur les dépendances
//!
//! Ce crate est INTENTIONNELLEMENT séparé de `apps/maia` pour éviter le conflit
//! rustpotter (`half ^2.5`) ↔ candle-core (`half =2.4.1`).
//! Ne jamais ajouter `rustpotter` ou `candle-core` dans ce crate.

pub mod confidence;
pub mod pipeline;
pub mod vad;
pub mod whisper;

// ── Types publics ────────────────────────────────────────────────────────

/// Résultat d'une transcription vocale.
#[derive(Debug, Clone)]
pub struct TranscriptResult {
    /// Texte transcrit (sans whitespace superflu).
    pub text: String,
    /// Score de confiance [0.0, 1.0].
    pub confidence: f32,
    /// Niveau de confiance détecté.
    pub level: ConfidenceLevel,
    /// Langue détectée (ex: "fr").
    pub language: String,
    /// Durée de l'audio transcrit en millisecondes.
    pub duration_ms: u32,
}

/// Niveau de confiance de la transcription.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfidenceLevel {
    /// Confiance haute (≥ 0.85) — résultat utilisable directement.
    Ok,
    /// Confiance moyenne (≥ 0.65) — résultat utilisable avec prudence.
    Warn,
    /// Confiance basse (< 0.65) — résultat à rejeter ou re-demander.
    Reject,
}

impl ConfidenceLevel {
    pub fn from_score(score: f32) -> Self {
        if score >= 0.85 {
            Self::Ok
        } else if score >= 0.65 {
            Self::Warn
        } else {
            Self::Reject
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Warn => "warn",
            Self::Reject => "reject",
        }
    }

    pub fn is_usable(&self) -> bool {
        matches!(self, Self::Ok | Self::Warn)
    }
}

/// Erreur du moteur STT.
#[derive(Debug)]
pub enum SttError {
    /// Modèle non chargé ou feature "whisper" non activée.
    ModelNotLoaded,
    /// Erreur d'initialisation whisper.
    WhisperInit(String),
    /// Erreur de transcription.
    Transcription(String),
    /// Audio trop court ou vide (< 100ms ou silence pur).
    AudioTooShort,
}

impl std::fmt::Display for SttError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ModelNotLoaded => write!(
                f,
                "Modèle STT non chargé (compilez avec --features maia-stt/whisper)"
            ),
            Self::WhisperInit(e) => write!(f, "Erreur init whisper : {e}"),
            Self::Transcription(e) => write!(f, "Erreur transcription : {e}"),
            Self::AudioTooShort => write!(f, "Audio trop court ou silence"),
        }
    }
}

impl std::error::Error for SttError {}
