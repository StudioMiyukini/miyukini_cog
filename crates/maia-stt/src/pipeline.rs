//! Pipeline STT complet : audio PCM → VAD → normalize → whisper → TranscriptResult.
//!
//! ## Flux
//! ```text
//! PCM (any rate, mono) → resample 16kHz → VAD → normalize → WhisperEngine → TranscriptResult
//! ```
//!
//! ## Usage minimal
//! ```rust,no_run
//! use maia_stt::pipeline::{SttPipeline, PipelineConfig};
//!
//! let config = PipelineConfig::default_cog_fr("/path/to/ggml-medium.bin");
//! let pipeline = SttPipeline::new(config).unwrap();
//! let audio: Vec<f32> = vec![0.0; 44100]; // 1s audio 44kHz
//! let result = pipeline.process(&audio, 44100).unwrap();
//! ```

use crate::confidence::{ConfidenceConfig, ConfidenceScorer};
use crate::vad::{normalize, resample_linear, VadConfig, VadProcessor};
use crate::whisper::{WhisperConfig, WhisperEngine};
use crate::{ConfidenceLevel, SttError, TranscriptResult};

/// Configuration du pipeline STT.
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    /// Configuration Whisper (chemin modèle, langue, prompt...).
    pub whisper: WhisperConfig,
    /// Durée minimale d'audio valide (ms). En dessous → AudioTooShort.
    pub min_audio_ms: u64,
    /// Si true, rejette les transcriptions vides.
    pub reject_empty: bool,
    /// Seuil no-speech : si Whisper détecte no_speech_prob > seuil → AudioTooShort.
    pub no_speech_threshold: f32,
}

impl PipelineConfig {
    /// Config par défaut COG francophone.
    pub fn default_cog_fr(model_path: impl Into<String>) -> Self {
        Self {
            whisper: WhisperConfig::default_cog_fr(model_path),
            min_audio_ms: 100,
            reject_empty: true,
            no_speech_threshold: 0.6,
        }
    }
}

/// Pipeline STT complet.
pub struct SttPipeline {
    engine: WhisperEngine,
    vad: VadProcessor,
    config: PipelineConfig,
}

impl SttPipeline {
    /// Crée et initialise le pipeline (charge le modèle Whisper).
    pub fn new(config: PipelineConfig) -> Result<Self, SttError> {
        let vad_config = config.whisper.vad.clone();
        let engine = WhisperEngine::load(config.whisper.clone())?;
        Ok(Self {
            engine,
            vad: VadProcessor::new(vad_config),
            config,
        })
    }

    /// Traite un buffer audio PCM f32 et retourne le transcript.
    ///
    /// - `samples` : audio PCM f32 (mono, n'importe quel taux).
    /// - `src_sample_rate` : taux d'échantillonnage de l'entrée (ex: 44100).
    ///
    /// Retourne `SttError::AudioTooShort` si l'audio est trop court ou sans voix.
    pub fn process(
        &self,
        samples: &[f32],
        src_sample_rate: u32,
    ) -> Result<TranscriptResult, SttError> {
        // 1. Resample → 16kHz
        let audio_16k = resample_linear(samples, src_sample_rate, 16_000);

        // 2. Vérification durée minimale
        let duration_ms = audio_16k.len() * 1000 / 16_000;
        if duration_ms < self.config.min_audio_ms as usize {
            return Err(SttError::AudioTooShort);
        }

        // 3. VAD : trouver les segments de voix
        let segments = self.vad.detect(&audio_16k);
        if segments.is_empty() {
            return Err(SttError::AudioTooShort);
        }

        // 4. Fusionner tous les segments (pour une requête unique)
        let merged: Vec<f32> = segments
            .into_iter()
            .flat_map(|s| s.samples)
            .collect();

        // 5. Normalisation
        let normalized = normalize(&merged);

        // 6. Transcription Whisper
        let result = self.engine.transcribe(&normalized)?;

        // 7. Rejeter les résultats vides si configuré
        if self.config.reject_empty && result.text.trim().is_empty() {
            return Err(SttError::AudioTooShort);
        }

        Ok(result)
    }

    /// Vérifie si le moteur est prêt.
    pub fn is_ready(&self) -> bool {
        self.engine.is_loaded()
    }

    /// Config active.
    pub fn config(&self) -> &PipelineConfig {
        &self.config
    }
}

/// Pipeline STT sans état (traitement direct sans instance).
///
/// Utile pour les tests ou les traitements one-shot.
pub fn process_audio_direct(
    samples: &[f32],
    src_sample_rate: u32,
    vad_config: Option<VadConfig>,
    confidence_config: Option<ConfidenceConfig>,
) -> Result<ProcessedAudio, SttError> {
    let vad = VadProcessor::new(vad_config.unwrap_or_default());
    let scorer = ConfidenceScorer::new(confidence_config.unwrap_or_default());

    // Resample → 16kHz
    let audio_16k = resample_linear(samples, src_sample_rate, 16_000);
    if audio_16k.len() < 1600 {
        return Err(SttError::AudioTooShort);
    }

    // VAD
    let segments = vad.detect(&audio_16k);
    if segments.is_empty() {
        return Err(SttError::AudioTooShort);
    }

    // Merge + normalize
    let merged: Vec<f32> = segments.iter().flat_map(|s| s.samples.iter().cloned()).collect();
    let normalized = normalize(&merged);

    let total_duration_ms = segments.iter().map(|s| s.duration_ms).sum();

    Ok(ProcessedAudio {
        samples: normalized,
        duration_ms: total_duration_ms,
        segments_count: segments.len(),
    })
}

/// Audio traité par le pipeline (post-VAD + normalize), prêt pour Whisper.
#[derive(Debug)]
pub struct ProcessedAudio {
    /// Samples PCM f32 à 16kHz, normalisés.
    pub samples: Vec<f32>,
    /// Durée totale des segments de voix (ms).
    pub duration_ms: u32,
    /// Nombre de segments de voix détectés.
    pub segments_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_speech_audio(sr: u32, duration_ms: u64) -> Vec<f32> {
        let n = (sr as usize * duration_ms as usize) / 1000;
        (0..n)
            .map(|i| 0.5 * (2.0 * std::f32::consts::PI * 440.0 * i as f32 / sr as f32).sin())
            .collect()
    }

    #[test]
    fn test_process_audio_direct_speech() {
        let audio = make_speech_audio(44_100, 600);
        let result = process_audio_direct(&audio, 44_100, None, None);
        assert!(result.is_ok(), "Doit traiter sans erreur : {:?}", result.err());
        let processed = result.unwrap();
        assert!(!processed.samples.is_empty());
        assert!(processed.duration_ms > 0);
        assert!(processed.segments_count >= 1);
    }

    #[test]
    fn test_process_audio_direct_silence() {
        let silence = vec![0.0f32; 44_100]; // 1s silence
        let result = process_audio_direct(&silence, 44_100, None, None);
        assert!(matches!(result, Err(SttError::AudioTooShort)));
    }

    #[test]
    fn test_process_audio_direct_too_short() {
        let short = make_speech_audio(16_000, 50); // 50ms — trop court
        let result = process_audio_direct(&short, 16_000, None, None);
        assert!(matches!(result, Err(SttError::AudioTooShort)));
    }

    #[test]
    fn test_normalization_in_pipeline() {
        let audio = make_speech_audio(16_000, 500);
        let result = process_audio_direct(&audio, 16_000, None, None).unwrap();
        // Vérifier que le pic est ~0.9 (normalisé)
        let peak = result.samples.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
        assert!(
            (peak - 0.9).abs() < 0.05,
            "Pic normalisé inattendu : {peak:.3}"
        );
    }
}
