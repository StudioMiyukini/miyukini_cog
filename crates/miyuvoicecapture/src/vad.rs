//! Voice Activity Detection (VAD) base sur l'energie RMS.
//!
//! Detecte les segments de parole dans un flux audio continu en calculant
//! l'energie RMS (Root Mean Square) par trame et en appliquant une machine
//! a etats avec debounce pour eviter les faux positifs.
//!
//! ## Machine a etats VAD
//!
//! ```text
//! Silence ---[RMS > seuil]---> MaybeSpeech
//! MaybeSpeech ---[RMS > seuil pendant confirmation_frames]---> Speech
//! MaybeSpeech ---[RMS <= seuil]---> Silence
//! Speech ---[RMS <= seuil]---> MaybeEnd
//! MaybeEnd ---[RMS <= seuil pendant debounce_frames]---> Silence
//! MaybeEnd ---[RMS > seuil]---> Speech
//! ```

// @id: alicia_capture_vad
// @role: processing
// @layer: toolkit
// @human: Voice Activity Detection base sur energie RMS avec debounce et pre-buffer.
// @do: detect_voice_activity

use serde::{Deserialize, Serialize};

use crate::errors::VoiceCaptureError;

/// Etats de la machine VAD.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VadState {
    /// Aucune activite vocale detectee.
    Silence,
    /// RMS au-dessus du seuil, en attente de confirmation.
    MaybeSpeech,
    /// Parole confirmee, capture active.
    Speech,
    /// RMS sous le seuil apres parole, en attente de debounce.
    MaybeEnd,
}

impl std::fmt::Display for VadState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Silence => write!(f, "Silence"),
            Self::MaybeSpeech => write!(f, "MaybeSpeech"),
            Self::Speech => write!(f, "Speech"),
            Self::MaybeEnd => write!(f, "MaybeEnd"),
        }
    }
}

/// Configuration du detecteur d'activite vocale.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VadConfig {
    /// Seuil RMS au-dessus duquel on considere qu'il y a de la parole.
    pub rms_threshold: f32,
    /// Nombre de trames consecutives au-dessus du seuil pour confirmer la parole.
    pub confirmation_frames: usize,
    /// Nombre de trames consecutives sous le seuil pour confirmer la fin de parole (debounce).
    pub debounce_frames: usize,
    /// Taille du pre-buffer en nombre de samples (pour garder l'audio avant detection).
    pub pre_buffer_samples: usize,
}

impl Default for VadConfig {
    fn default() -> Self {
        // Valeurs par defaut pour 16kHz, trame de 480 samples (30ms)
        // Debounce ~300ms = 10 trames de 30ms
        // Confirmation = 2 trames (~60ms)
        // Pre-buffer = 8000 samples (~500ms a 16kHz)
        Self {
            rms_threshold: 0.02,
            confirmation_frames: 2,
            debounce_frames: 10,
            pre_buffer_samples: 8_000,
        }
    }
}

impl VadConfig {
    /// Construit une configuration VAD a partir des parametres du contexte.
    ///
    /// Calcule automatiquement les trames de debounce et de pre-buffer
    /// en fonction de la frequence d'echantillonnage et de la taille de trame.
    pub fn from_context_params(
        rms_threshold: f32,
        debounce_ms: u64,
        pre_buffer_ms: u64,
        sample_rate: u32,
        frame_size: usize,
    ) -> Result<Self, VoiceCaptureError> {
        if rms_threshold <= 0.0 || rms_threshold >= 1.0 {
            return Err(VoiceCaptureError::InvalidVadConfig(
                format!("RMS threshold must be in (0.0, 1.0), got {rms_threshold}"),
            ));
        }
        if frame_size == 0 {
            return Err(VoiceCaptureError::InvalidVadConfig(
                "Frame size must be > 0".to_string(),
            ));
        }
        if sample_rate == 0 {
            return Err(VoiceCaptureError::InvalidVadConfig(
                "Sample rate must be > 0".to_string(),
            ));
        }

        let samples_per_ms = f64::from(sample_rate) / 1000.0;
        let debounce_samples = (debounce_ms as f64 * samples_per_ms) as usize;
        let pre_buffer_samples = (pre_buffer_ms as f64 * samples_per_ms) as usize;

        let debounce_frames = if frame_size > 0 {
            debounce_samples / frame_size
        } else {
            0
        };

        Ok(Self {
            rms_threshold,
            confirmation_frames: 2,
            debounce_frames: debounce_frames.max(1),
            pre_buffer_samples,
        })
    }

    /// Valide la configuration.
    pub fn validate(&self) -> Result<(), VoiceCaptureError> {
        if self.rms_threshold <= 0.0 || self.rms_threshold >= 1.0 {
            return Err(VoiceCaptureError::InvalidVadConfig(
                format!(
                    "RMS threshold must be in (0.0, 1.0), got {}",
                    self.rms_threshold
                ),
            ));
        }
        if self.debounce_frames == 0 {
            return Err(VoiceCaptureError::InvalidVadConfig(
                "Debounce frames must be > 0".to_string(),
            ));
        }
        Ok(())
    }
}

/// Detecteur d'activite vocale (VAD) base sur l'energie RMS.
///
/// Traite des trames audio et maintient une machine a etats pour
/// distinguer les segments de parole du silence. Inclut un pre-buffer
/// circulaire pour conserver l'audio juste avant la detection.
pub struct VoiceActivityDetector {
    /// Configuration du VAD.
    config: VadConfig,
    /// Etat courant de la machine a etats.
    state: VadState,
    /// Compteur de trames dans l'etat transitoire courant.
    transition_counter: usize,
    /// Pre-buffer circulaire pour garder les samples avant detection.
    pre_buffer: Vec<f32>,
    /// Position d'ecriture dans le pre-buffer (index circulaire).
    pre_buffer_write_pos: usize,
    /// Nombre de samples actuellement dans le pre-buffer.
    pre_buffer_count: usize,
    /// Derniere valeur RMS calculee.
    last_rms: f32,
}

impl VoiceActivityDetector {
    /// Cree un nouveau detecteur avec la configuration par defaut.
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(VadConfig::default())
    }

    /// Cree un nouveau detecteur avec la configuration specifiee.
    #[must_use]
    pub fn with_config(config: VadConfig) -> Self {
        let pre_buffer_size = config.pre_buffer_samples;
        Self {
            config,
            state: VadState::Silence,
            transition_counter: 0,
            pre_buffer: vec![0.0; pre_buffer_size],
            pre_buffer_write_pos: 0,
            pre_buffer_count: 0,
            last_rms: 0.0,
        }
    }

    /// Retourne l'etat courant du VAD.
    #[must_use]
    pub fn state(&self) -> VadState {
        self.state
    }

    /// Retourne la derniere valeur RMS calculee.
    #[must_use]
    pub fn last_rms(&self) -> f32 {
        self.last_rms
    }

    /// Retourne la configuration du VAD.
    #[must_use]
    pub fn config(&self) -> &VadConfig {
        &self.config
    }

    /// Indique si le VAD detecte actuellement de la parole.
    ///
    /// Retourne `true` pour les etats `Speech` et `MaybeEnd`
    /// (pendant le debounce, on considere toujours qu'il y a de la parole).
    #[must_use]
    pub fn is_speech(&self) -> bool {
        matches!(self.state, VadState::Speech | VadState::MaybeEnd)
    }

    /// Calcule l'energie RMS (Root Mean Square) d'une trame audio.
    ///
    /// RMS = sqrt(sum(sample^2) / N)
    ///
    /// Retourne 0.0 pour une trame vide.
    #[must_use]
    pub fn compute_rms(samples: &[f32]) -> f32 {
        if samples.is_empty() {
            return 0.0;
        }
        let sum_sq: f64 = samples
            .iter()
            .map(|&s| f64::from(s) * f64::from(s))
            .sum();
        let mean_sq = sum_sq / samples.len() as f64;
        mean_sq.sqrt() as f32
    }

    /// Traite une trame audio et met a jour l'etat du VAD.
    ///
    /// Retourne le nouvel etat apres traitement de la trame.
    /// Alimente egalement le pre-buffer circulaire.
    pub fn process_frame(&mut self, samples: &[f32]) -> VadState {
        // Alimenter le pre-buffer
        self.feed_pre_buffer(samples);

        // Calculer le RMS de la trame
        let rms = Self::compute_rms(samples);
        self.last_rms = rms;
        let above_threshold = rms > self.config.rms_threshold;

        // Machine a etats
        match self.state {
            VadState::Silence => {
                if above_threshold {
                    self.state = VadState::MaybeSpeech;
                    self.transition_counter = 1;
                    tracing::debug!(rms = %rms, "VAD: Silence -> MaybeSpeech");
                }
            }
            VadState::MaybeSpeech => {
                if above_threshold {
                    self.transition_counter += 1;
                    if self.transition_counter >= self.config.confirmation_frames {
                        self.state = VadState::Speech;
                        self.transition_counter = 0;
                        tracing::info!(rms = %rms, "VAD: MaybeSpeech -> Speech (confirmed)");
                    }
                } else {
                    // Faux positif, retour au silence
                    self.state = VadState::Silence;
                    self.transition_counter = 0;
                    tracing::debug!(rms = %rms, "VAD: MaybeSpeech -> Silence (false positive)");
                }
            }
            VadState::Speech => {
                if !above_threshold {
                    self.state = VadState::MaybeEnd;
                    self.transition_counter = 1;
                    tracing::debug!(rms = %rms, "VAD: Speech -> MaybeEnd");
                }
            }
            VadState::MaybeEnd => {
                if above_threshold {
                    // Reprise de parole pendant le debounce
                    self.state = VadState::Speech;
                    self.transition_counter = 0;
                    tracing::debug!(rms = %rms, "VAD: MaybeEnd -> Speech (resumed)");
                } else {
                    self.transition_counter += 1;
                    if self.transition_counter >= self.config.debounce_frames {
                        self.state = VadState::Silence;
                        self.transition_counter = 0;
                        tracing::info!(rms = %rms, "VAD: MaybeEnd -> Silence (debounced)");
                    }
                }
            }
        }

        self.state
    }

    /// Reinitialise le VAD a l'etat Silence.
    pub fn reset(&mut self) {
        self.state = VadState::Silence;
        self.transition_counter = 0;
        self.last_rms = 0.0;
        self.pre_buffer.fill(0.0);
        self.pre_buffer_write_pos = 0;
        self.pre_buffer_count = 0;
    }

    /// Recupere le contenu du pre-buffer (audio avant la detection).
    ///
    /// Retourne les samples dans l'ordre chronologique.
    #[must_use]
    pub fn drain_pre_buffer(&self) -> Vec<f32> {
        if self.pre_buffer.is_empty() {
            return Vec::new();
        }

        let len = self.pre_buffer.len();
        let count = self.pre_buffer_count.min(len);

        if count == 0 {
            return Vec::new();
        }

        let mut result = Vec::with_capacity(count);

        if count < len {
            // Le buffer n'est pas encore plein
            let start = self.pre_buffer_write_pos.saturating_sub(count);
            result.extend_from_slice(&self.pre_buffer[start..self.pre_buffer_write_pos]);
        } else {
            // Le buffer est plein, lecture circulaire
            result.extend_from_slice(&self.pre_buffer[self.pre_buffer_write_pos..]);
            result.extend_from_slice(&self.pre_buffer[..self.pre_buffer_write_pos]);
        }

        result
    }

    /// Alimente le pre-buffer circulaire avec de nouveaux samples.
    fn feed_pre_buffer(&mut self, samples: &[f32]) {
        if self.pre_buffer.is_empty() {
            return;
        }

        let buf_len = self.pre_buffer.len();
        for &sample in samples {
            self.pre_buffer[self.pre_buffer_write_pos] = sample;
            self.pre_buffer_write_pos = (self.pre_buffer_write_pos + 1) % buf_len;
            if self.pre_buffer_count < buf_len {
                self.pre_buffer_count += 1;
            }
        }
    }
}

impl Default for VoiceActivityDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_silence(len: usize) -> Vec<f32> {
        vec![0.0; len]
    }

    fn make_speech(len: usize, amplitude: f32) -> Vec<f32> {
        // Signal sinusoidal simple
        (0..len)
            .map(|i| amplitude * (2.0 * std::f32::consts::PI * 440.0 * i as f32 / 16000.0).sin())
            .collect()
    }

    #[test]
    fn test_rms_silence() {
        let samples = make_silence(480);
        let rms = VoiceActivityDetector::compute_rms(&samples);
        assert!(rms.abs() < f32::EPSILON, "Silence RMS should be 0.0");
    }

    #[test]
    fn test_rms_nonzero() {
        let samples = make_speech(480, 0.5);
        let rms = VoiceActivityDetector::compute_rms(&samples);
        assert!(rms > 0.0, "Speech RMS should be > 0.0");
        assert!(rms < 1.0, "Speech RMS should be < 1.0");
    }

    #[test]
    fn test_rms_empty() {
        let rms = VoiceActivityDetector::compute_rms(&[]);
        assert!(rms.abs() < f32::EPSILON);
    }

    #[test]
    fn test_rms_known_value() {
        // RMS of constant signal = absolute value of the constant
        let samples = vec![0.5_f32; 100];
        let rms = VoiceActivityDetector::compute_rms(&samples);
        assert!((rms - 0.5).abs() < 0.001, "RMS of constant 0.5 should be ~0.5, got {rms}");
    }

    #[test]
    fn test_vad_starts_in_silence() {
        let vad = VoiceActivityDetector::new();
        assert_eq!(vad.state(), VadState::Silence);
        assert!(!vad.is_speech());
    }

    #[test]
    fn test_vad_silence_stays_silent() {
        let mut vad = VoiceActivityDetector::new();
        let silence = make_silence(480);
        for _ in 0..10 {
            vad.process_frame(&silence);
        }
        assert_eq!(vad.state(), VadState::Silence);
    }

    #[test]
    fn test_vad_speech_detection() {
        let config = VadConfig {
            rms_threshold: 0.02,
            confirmation_frames: 2,
            debounce_frames: 3,
            pre_buffer_samples: 1000,
        };
        let mut vad = VoiceActivityDetector::with_config(config);

        // Envoyer des trames de parole (amplitude elevee)
        let speech = make_speech(480, 0.3);
        vad.process_frame(&speech); // -> MaybeSpeech
        assert_eq!(vad.state(), VadState::MaybeSpeech);

        vad.process_frame(&speech); // -> Speech (confirmation_frames = 2)
        assert_eq!(vad.state(), VadState::Speech);
        assert!(vad.is_speech());
    }

    #[test]
    fn test_vad_false_positive_rejection() {
        let config = VadConfig {
            rms_threshold: 0.02,
            confirmation_frames: 2,
            debounce_frames: 3,
            pre_buffer_samples: 1000,
        };
        let mut vad = VoiceActivityDetector::with_config(config);

        let speech = make_speech(480, 0.3);
        let silence = make_silence(480);

        vad.process_frame(&speech); // -> MaybeSpeech
        assert_eq!(vad.state(), VadState::MaybeSpeech);

        vad.process_frame(&silence); // -> Silence (faux positif)
        assert_eq!(vad.state(), VadState::Silence);
    }

    #[test]
    fn test_vad_debounce() {
        let config = VadConfig {
            rms_threshold: 0.02,
            confirmation_frames: 2,
            debounce_frames: 3,
            pre_buffer_samples: 1000,
        };
        let mut vad = VoiceActivityDetector::with_config(config);

        let speech = make_speech(480, 0.3);
        let silence = make_silence(480);

        // Entrer en Speech
        vad.process_frame(&speech);
        vad.process_frame(&speech);
        assert_eq!(vad.state(), VadState::Speech);

        // Premier silence -> MaybeEnd
        vad.process_frame(&silence);
        assert_eq!(vad.state(), VadState::MaybeEnd);
        assert!(vad.is_speech()); // Toujours considere comme parole pendant debounce

        // Deuxieme silence -> toujours MaybeEnd (debounce_frames = 3)
        vad.process_frame(&silence);
        assert_eq!(vad.state(), VadState::MaybeEnd);

        // Troisieme silence -> Silence (debounce complete)
        vad.process_frame(&silence);
        assert_eq!(vad.state(), VadState::Silence);
        assert!(!vad.is_speech());
    }

    #[test]
    fn test_vad_speech_resume_during_debounce() {
        let config = VadConfig {
            rms_threshold: 0.02,
            confirmation_frames: 2,
            debounce_frames: 5,
            pre_buffer_samples: 1000,
        };
        let mut vad = VoiceActivityDetector::with_config(config);

        let speech = make_speech(480, 0.3);
        let silence = make_silence(480);

        // Entrer en Speech
        vad.process_frame(&speech);
        vad.process_frame(&speech);
        assert_eq!(vad.state(), VadState::Speech);

        // Debut du debounce
        vad.process_frame(&silence);
        assert_eq!(vad.state(), VadState::MaybeEnd);

        // Reprise de parole -> retour a Speech
        vad.process_frame(&speech);
        assert_eq!(vad.state(), VadState::Speech);
    }

    #[test]
    fn test_vad_reset() {
        let mut vad = VoiceActivityDetector::new();
        let speech = make_speech(480, 0.3);

        vad.process_frame(&speech);
        vad.process_frame(&speech);
        assert_eq!(vad.state(), VadState::Speech);

        vad.reset();
        assert_eq!(vad.state(), VadState::Silence);
        assert!(vad.last_rms().abs() < f32::EPSILON);
    }

    #[test]
    fn test_pre_buffer_stores_samples() {
        let config = VadConfig {
            rms_threshold: 0.02,
            confirmation_frames: 2,
            debounce_frames: 3,
            pre_buffer_samples: 100,
        };
        let mut vad = VoiceActivityDetector::with_config(config);

        let frame: Vec<f32> = (0..50).map(|i| i as f32 / 50.0).collect();
        vad.process_frame(&frame);

        let pre = vad.drain_pre_buffer();
        assert_eq!(pre.len(), 50);
    }

    #[test]
    fn test_pre_buffer_wraps_around() {
        let config = VadConfig {
            rms_threshold: 0.02,
            confirmation_frames: 2,
            debounce_frames: 3,
            pre_buffer_samples: 100,
        };
        let mut vad = VoiceActivityDetector::with_config(config);

        // Ecrire 150 samples dans un buffer de 100
        let frame1: Vec<f32> = (0..80).map(|i| i as f32).collect();
        let frame2: Vec<f32> = (80..150).map(|i| i as f32).collect();
        vad.process_frame(&frame1);
        vad.process_frame(&frame2);

        let pre = vad.drain_pre_buffer();
        // Le buffer circulaire de 100 contient les 100 derniers samples (50..150)
        assert_eq!(pre.len(), 100);
        // Le premier sample doit etre 50.0 (les 50 premiers ont ete ecrases)
        assert!((pre[0] - 50.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_vad_config_validation() {
        let mut config = VadConfig::default();
        assert!(config.validate().is_ok());

        config.rms_threshold = 0.0;
        assert!(config.validate().is_err());

        config.rms_threshold = 1.0;
        assert!(config.validate().is_err());

        config.rms_threshold = 0.5;
        config.debounce_frames = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_vad_config_from_context() {
        let config = VadConfig::from_context_params(
            0.03,    // threshold
            300,     // debounce ms
            500,     // pre-buffer ms
            16_000,  // sample rate
            480,     // frame size (30ms)
        );
        assert!(config.is_ok());
        let config = config.unwrap();
        assert!((config.rms_threshold - 0.03).abs() < f32::EPSILON);
        // 300ms / 30ms per frame = 10 frames
        assert_eq!(config.debounce_frames, 10);
        // 500ms * 16 samples/ms = 8000 samples
        assert_eq!(config.pre_buffer_samples, 8_000);
    }

    #[test]
    fn test_vad_state_display() {
        assert_eq!(VadState::Silence.to_string(), "Silence");
        assert_eq!(VadState::MaybeSpeech.to_string(), "MaybeSpeech");
        assert_eq!(VadState::Speech.to_string(), "Speech");
        assert_eq!(VadState::MaybeEnd.to_string(), "MaybeEnd");
    }
}
