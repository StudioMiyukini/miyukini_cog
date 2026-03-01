//! Contexte de configuration runtime pour Alicia Capture.
//!
//! Contient les parametres de capture audio, VAD, et buffer
//! utilises a l'initialisation du toolkit.

// @id: alicia_capture_context
// @role: data
// @layer: toolkit
// @human: Contexte de configuration runtime Alicia Capture.
// @do: hold_voice_capture_context

use serde::{Deserialize, Serialize};

/// Frequence d'echantillonnage cible par defaut (16 kHz mono pour STT).
pub const DEFAULT_SAMPLE_RATE: u32 = 16_000;

/// Nombre de canaux par defaut (mono).
pub const DEFAULT_CHANNELS: u16 = 1;

/// Taille du buffer circulaire par defaut : 30 secondes a 16 kHz mono.
pub const DEFAULT_BUFFER_SIZE: usize = 16_000 * 30; // 480 000 samples

/// Seuil RMS par defaut pour la detection d'activite vocale.
pub const DEFAULT_VAD_THRESHOLD: f32 = 0.02;

/// Duree de debounce VAD par defaut en millisecondes.
pub const DEFAULT_VAD_DEBOUNCE_MS: u64 = 300;

/// Duree du pre-buffer VAD par defaut en millisecondes.
pub const DEFAULT_VAD_PRE_BUFFER_MS: u64 = 500;

/// Nombre maximum de pieces supportees simultanement.
pub const MAX_ROOMS: usize = 4;

/// Contexte de configuration runtime pour le toolkit Alicia Capture.
///
/// Definit les parametres de capture audio, la configuration VAD,
/// et les dimensions du buffer circulaire.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceCaptureContext {
    /// Frequence d'echantillonnage cible (Hz).
    pub sample_rate: u32,
    /// Nombre de canaux (1 = mono, 2 = stereo).
    pub channels: u16,
    /// Taille du buffer circulaire en nombre de samples.
    pub buffer_size: usize,
    /// Seuil RMS pour la detection d'activite vocale.
    pub vad_threshold: f32,
    /// Duree de debounce VAD (ms) avant transition Speech vers Silence.
    pub vad_debounce_ms: u64,
    /// Duree du pre-buffer VAD (ms) pour conserver l'audio avant detection.
    pub vad_pre_buffer_ms: u64,
    /// Identifiant du mandat d'execution (gouvernance).
    pub mandate_id: String,
    /// Niveau de securite (0-4, WorrySentinel).
    pub security_level: u8,
}

impl Default for VoiceCaptureContext {
    fn default() -> Self {
        Self {
            sample_rate: DEFAULT_SAMPLE_RATE,
            channels: DEFAULT_CHANNELS,
            buffer_size: DEFAULT_BUFFER_SIZE,
            vad_threshold: DEFAULT_VAD_THRESHOLD,
            vad_debounce_ms: DEFAULT_VAD_DEBOUNCE_MS,
            vad_pre_buffer_ms: DEFAULT_VAD_PRE_BUFFER_MS,
            mandate_id: String::new(),
            security_level: 0,
        }
    }
}

impl VoiceCaptureContext {
    /// Construit un nouveau contexte avec les parametres par defaut.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Construit un contexte avec un mandat et un niveau de securite.
    #[must_use]
    pub fn with_mandate(mandate_id: String, security_level: u8) -> Self {
        Self {
            mandate_id,
            security_level,
            ..Self::default()
        }
    }

    /// Indique si le contexte a un mandat valide.
    #[must_use]
    pub fn has_mandate(&self) -> bool {
        !self.mandate_id.is_empty()
    }

    /// Modifie le seuil VAD.
    #[must_use]
    pub fn with_vad_threshold(mut self, threshold: f32) -> Self {
        self.vad_threshold = threshold;
        self
    }

    /// Modifie la taille du buffer circulaire.
    #[must_use]
    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    /// Modifie la frequence d'echantillonnage.
    #[must_use]
    pub fn with_sample_rate(mut self, rate: u32) -> Self {
        self.sample_rate = rate;
        self
    }

    /// Calcule le nombre de samples correspondant au pre-buffer VAD.
    #[must_use]
    pub fn vad_pre_buffer_samples(&self) -> usize {
        let ms = self.vad_pre_buffer_ms;
        let rate = u64::from(self.sample_rate);
        (ms * rate / 1000) as usize
    }

    /// Calcule le nombre de samples correspondant au debounce VAD.
    #[must_use]
    pub fn vad_debounce_samples(&self) -> usize {
        let ms = self.vad_debounce_ms;
        let rate = u64::from(self.sample_rate);
        (ms * rate / 1000) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_context() {
        let ctx = VoiceCaptureContext::new();
        assert_eq!(ctx.sample_rate, 16_000);
        assert_eq!(ctx.channels, 1);
        assert_eq!(ctx.buffer_size, 480_000);
        assert!((ctx.vad_threshold - 0.02).abs() < f32::EPSILON);
        assert_eq!(ctx.vad_debounce_ms, 300);
        assert_eq!(ctx.vad_pre_buffer_ms, 500);
        assert!(!ctx.has_mandate());
    }

    #[test]
    fn test_with_mandate() {
        let ctx = VoiceCaptureContext::with_mandate("test-mandate-001".to_string(), 2);
        assert!(ctx.has_mandate());
        assert_eq!(ctx.security_level, 2);
        assert_eq!(ctx.mandate_id, "test-mandate-001");
    }

    #[test]
    fn test_builder_chain() {
        let ctx = VoiceCaptureContext::new()
            .with_vad_threshold(0.05)
            .with_buffer_size(320_000)
            .with_sample_rate(48_000);
        assert!((ctx.vad_threshold - 0.05).abs() < f32::EPSILON);
        assert_eq!(ctx.buffer_size, 320_000);
        assert_eq!(ctx.sample_rate, 48_000);
    }

    #[test]
    fn test_pre_buffer_samples() {
        let ctx = VoiceCaptureContext::new(); // 16kHz, 500ms pre-buffer
        // 500ms * 16000 / 1000 = 8000 samples
        assert_eq!(ctx.vad_pre_buffer_samples(), 8_000);
    }

    #[test]
    fn test_debounce_samples() {
        let ctx = VoiceCaptureContext::new(); // 16kHz, 300ms debounce
        // 300ms * 16000 / 1000 = 4800 samples
        assert_eq!(ctx.vad_debounce_samples(), 4_800);
    }
}
