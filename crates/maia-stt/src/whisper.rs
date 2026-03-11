//! Wrapper whisper-rs pour la transcription MAIA-STT.
//!
//! Compilé seulement avec la feature "whisper" (nécessite whisper.cpp compilé).
//! Sans la feature, seuls les types publics et les stubs sont disponibles.

use crate::{SttError, TranscriptResult};
use crate::confidence::{ConfidenceConfig, ConfidenceScorer};
use crate::vad::VadConfig;

/// Configuration du moteur Whisper.
#[derive(Debug, Clone)]
pub struct WhisperConfig {
    /// Chemin vers le fichier de modèle whisper.cpp (.bin).
    pub model_path: String,
    /// Code langue (ex: "fr", "en", "auto").
    pub language: String,
    /// Prompt initial pour guider la transcription (vocabulaire COG).
    pub initial_prompt: String,
    /// Beam size pour le décodage (plus élevé = meilleur mais plus lent).
    pub beam_size: u32,
    /// Threads CPU à utiliser (0 = automatique).
    pub n_threads: u32,
    /// Activer la traduction en anglais (false = transcription dans la langue source).
    pub translate: bool,
    /// Configuration de confiance.
    pub confidence: ConfidenceConfig,
    /// Configuration VAD.
    pub vad: VadConfig,
}

impl WhisperConfig {
    /// Config par défaut pour l'usage COG francophone.
    pub fn default_cog_fr(model_path: impl Into<String>) -> Self {
        Self {
            model_path: model_path.into(),
            language: "fr".into(),
            initial_prompt: "Miyukini, COG, MAIA, Alicia, Miou, Sophie. Vocabulaire technique français.".into(),
            beam_size: 5,
            n_threads: 0,
            translate: false,
            confidence: ConfidenceConfig::default(),
            vad: VadConfig::default(),
        }
    }
}

// ── WhisperEngine ────────────────────────────────────────────────────────

/// Moteur de transcription Whisper.
///
/// Avec feature "whisper" → appels réels à whisper.cpp via whisper-rs.
/// Sans feature "whisper" → erreur SttError::ModelNotLoaded.
pub struct WhisperEngine {
    config: WhisperConfig,
    #[cfg(feature = "whisper")]
    ctx: std::sync::Arc<std::sync::Mutex<whisper_rs::WhisperContext>>,
    _loaded: bool,
}

impl WhisperEngine {
    /// Charge le modèle Whisper depuis `config.model_path`.
    pub fn load(config: WhisperConfig) -> Result<Self, SttError> {
        #[cfg(feature = "whisper")]
        {
            let params = whisper_rs::WhisperContextParameters::default();
            let ctx = whisper_rs::WhisperContext::new_with_params(&config.model_path, params)
                .map_err(|e| SttError::WhisperInit(e.to_string()))?;
            Ok(Self {
                config,
                ctx: std::sync::Arc::new(std::sync::Mutex::new(ctx)),
                _loaded: true,
            })
        }
        #[cfg(not(feature = "whisper"))]
        {
            tracing::warn!(
                "Feature 'whisper' non activée — WhisperEngine::load retourne stub. \
                 Compilez avec --features maia-stt/whisper pour l'inférence réelle."
            );
            Ok(Self {
                config,
                _loaded: false,
            })
        }
    }

    /// Transcrit un buffer audio PCM f32 à 16kHz mono.
    ///
    /// - Avec feature "whisper" : transcription réelle via whisper.cpp.
    /// - Sans feature : retourne `SttError::ModelNotLoaded`.
    pub fn transcribe(&self, audio: &[f32]) -> Result<TranscriptResult, SttError> {
        if audio.is_empty() || audio.len() < 1600 {
            // < 100ms d'audio → trop court
            return Err(SttError::AudioTooShort);
        }

        #[cfg(feature = "whisper")]
        {
            self.transcribe_real(audio)
        }
        #[cfg(not(feature = "whisper"))]
        {
            Err(SttError::ModelNotLoaded)
        }
    }

    /// Indique si le moteur est chargé et prêt.
    pub fn is_loaded(&self) -> bool {
        self._loaded
    }

    /// Config active.
    pub fn config(&self) -> &WhisperConfig {
        &self.config
    }

    // ── Implémentation réelle (feature whisper uniquement) ────────────────

    #[cfg(feature = "whisper")]
    fn transcribe_real(&self, audio: &[f32]) -> Result<TranscriptResult, SttError> {
        let ctx = self.ctx.lock().map_err(|e| {
            SttError::Transcription(format!("Mutex poisoned : {e}"))
        })?;

        let mut state = ctx.create_state().map_err(|e| {
            SttError::Transcription(format!("Création state : {e}"))
        })?;

        let mut params = whisper_rs::FullParams::new(whisper_rs::SamplingStrategy::BeamSearch {
            beam_size: self.config.beam_size as i32,
            patience: -1.0,
        });

        params.set_language(if self.config.language == "auto" {
            None
        } else {
            Some(&self.config.language)
        });
        params.set_initial_prompt(&self.config.initial_prompt);
        params.set_translate(self.config.translate);
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        if self.config.n_threads > 0 {
            params.set_n_threads(self.config.n_threads as i32);
        }

        state.full(params, audio).map_err(|e| {
            SttError::Transcription(format!("Transcription : {e}"))
        })?;

        // Collecter les segments et log-probs
        let n_segments = state.full_n_segments().map_err(|e| {
            SttError::Transcription(format!("n_segments : {e}"))
        })?;

        let mut text = String::new();
        let mut all_logprobs: Vec<f32> = Vec::new();

        for i in 0..n_segments {
            if let Ok(seg_text) = state.full_get_segment_text(i) {
                text.push_str(seg_text.trim());
                text.push(' ');
            }
            // Collecter log-probs des tokens de ce segment
            if let Ok(n_tokens) = state.full_n_tokens(i) {
                for t in 0..n_tokens {
                    if let Ok(data) = state.full_get_token_data(i, t) {
                        all_logprobs.push(data.p.ln()); // p ∈ [0,1] → log-prob
                    }
                }
            }
        }

        let text = text.trim().to_string();
        let scorer = ConfidenceScorer::new(self.config.confidence.clone());
        let (confidence, level) = scorer.evaluate(&all_logprobs);

        let duration_ms = (audio.len() * 1000 / 16_000) as u32;

        // Détecter la langue si mode auto
        let language = if self.config.language == "auto" {
            state.full_lang_id()
                .ok()
                .and_then(|id| whisper_rs::get_lang_str(id).ok())
                .map(|s| s.to_string())
                .unwrap_or_else(|| "fr".into())
        } else {
            self.config.language.clone()
        };

        Ok(TranscriptResult {
            text,
            confidence,
            level,
            language,
            duration_ms,
        })
    }
}
