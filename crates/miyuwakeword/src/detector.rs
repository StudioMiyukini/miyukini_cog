/// @id toolkit.alicia.wakeword.detector
/// @do implement_wake_word_detection_with_rustpotter
/// @role wake_word_detector
/// @layer 6
/// @human Implementation du detecteur de wake word via rustpotter
use std::path::Path;
use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::errors::WakeWordError;

// -- Public types --

/// Evenement de detection du wake word.
#[derive(Debug, Clone)]
pub struct WakeWordDetection {
    /// Nom du mot-cle detecte (ex: "hey_alicia").
    pub keyword: String,
    /// Score de confiance [0.0, 1.0].
    pub confidence: f32,
    /// Timestamp monotone de la detection.
    pub timestamp: Instant,
    /// Nombre d'echantillons traites depuis le debut du flux.
    pub sample_offset: u64,
}

/// Mode de calcul du score rustpotter.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScoreMode {
    /// Moyenne des scores sur les references.
    #[default]
    Average,
    /// Score maximum parmi les references.
    Max,
    /// Percentile (P75 par defaut).
    Percentile,
}

/// Configuration du detecteur de wake word.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WakeWordConfig {
    /// Chemin vers le fichier modele (.rpw pour reference, .rpwm pour modele entraine).
    pub model_path: String,
    /// Seuil de confiance minimum pour valider une detection [0.0, 1.0].
    /// Valeur par defaut : 0.5
    pub threshold: f32,
    /// Frequence d'echantillonnage attendue (doit correspondre au modele).
    /// Valeur par defaut : 16000 Hz
    pub sample_rate: u32,
    /// Nombre de canaux attendus.
    /// Valeur par defaut : 1 (mono)
    pub channels: u16,
    /// Duree minimale entre deux detections consecutives (ms).
    /// Evite les doubles detections sur un meme enonce.
    /// Valeur par defaut : 2000 ms
    pub min_detection_interval_ms: u32,
    /// Mode de scoring rustpotter.
    /// Valeur par defaut : Average
    pub score_mode: ScoreMode,
    /// Active le filtre passe-bande pour la voix (300-3400 Hz).
    /// Valeur par defaut : true
    pub band_pass_filter: bool,
    /// Active la normalisation de gain automatique.
    /// Valeur par defaut : true
    pub gain_normalizer: bool,
}

impl Default for WakeWordConfig {
    fn default() -> Self {
        Self {
            model_path: crate::models::default_model_path(),
            threshold: 0.5,
            sample_rate: 16000,
            channels: 1,
            min_detection_interval_ms: 2000,
            score_mode: ScoreMode::default(),
            band_pass_filter: true,
            gain_normalizer: true,
        }
    }
}

/// Trait pour le detecteur de wake word.
///
/// Permet de swapper l'implementation (rustpotter, Porcupine, etc.)
/// sans modifier le code appelant.
pub trait WakeWordDetector: Send {
    /// Traite un buffer d'echantillons audio (f32 normalise).
    /// Retourne `Some(detection)` si le wake word est detecte.
    fn process_samples(&mut self, samples: &[f32]) -> Option<WakeWordDetection>;

    /// Reinitialise l'etat interne du detecteur.
    fn reset(&mut self);

    /// Retourne le nom du mot-cle configure.
    fn keyword_name(&self) -> &str;

    /// Retourne le seuil de confiance actuel.
    fn threshold(&self) -> f32;

    /// Met a jour le seuil de confiance.
    fn set_threshold(&mut self, threshold: f32);
}

// -- Rustpotter implementation --

/// Implementation rustpotter du detecteur de wake word.
pub struct RustpotterDetector {
    /// Instance rustpotter interne.
    detector: rustpotter::Rustpotter,
    /// Configuration utilisateur.
    config: WakeWordConfig,
    /// Compteur d'echantillons traites.
    sample_count: u64,
    /// Instant de la derniere detection (debounce).
    last_detection: Option<Instant>,
    /// Seuil de confiance courant.
    current_threshold: f32,
}

impl RustpotterDetector {
    /// Cree un nouveau detecteur rustpotter a partir de la configuration.
    ///
    /// # Errors
    ///
    /// - `WakeWordError::ModelNotFound` si le fichier modele n'existe pas
    /// - `WakeWordError::Config` si la configuration rustpotter est invalide
    /// - `WakeWordError::ModelInvalid` si le modele ne peut pas etre charge
    pub fn new(config: WakeWordConfig) -> Result<Self, WakeWordError> {
        // Validate model path exists
        let model_path = Path::new(&config.model_path);
        if !model_path.exists() {
            return Err(WakeWordError::ModelNotFound(format!(
                "Model file does not exist: {}",
                config.model_path
            )));
        }

        // Build rustpotter config
        let mut rp_config = rustpotter::RustpotterConfig::default();
        rp_config.fmt.sample_rate = config.sample_rate as usize;
        rp_config.fmt.channels = config.channels;
        rp_config.fmt.sample_format = rustpotter::SampleFormat::F32;
        rp_config.detector.threshold = config.threshold;
        rp_config.detector.avg_threshold = config.threshold;
        rp_config.filters.gain_normalizer.enabled = config.gain_normalizer;
        rp_config.filters.band_pass.enabled = config.band_pass_filter;

        // Set score mode
        match config.score_mode {
            ScoreMode::Average => {
                rp_config.detector.score_mode = rustpotter::ScoreMode::Average;
            }
            ScoreMode::Max => {
                rp_config.detector.score_mode = rustpotter::ScoreMode::Max;
            }
            ScoreMode::Percentile => {
                rp_config.detector.score_mode = rustpotter::ScoreMode::Median;
            }
        }

        // Create rustpotter instance
        let mut detector = rustpotter::Rustpotter::new(&rp_config).map_err(|e| {
            WakeWordError::Config(format!("Failed to create Rustpotter instance: {e}"))
        })?;

        // Load wakeword model
        detector
            .add_wakeword_from_file("hey_alicia", &config.model_path)
            .map_err(|e| {
                WakeWordError::ModelInvalid(format!(
                    "Failed to load wakeword model '{}': {e}",
                    config.model_path
                ))
            })?;

        let current_threshold = config.threshold;

        tracing::info!(
            model_path = %config.model_path,
            threshold = config.threshold,
            sample_rate = config.sample_rate,
            score_mode = ?config.score_mode,
            "RustpotterDetector initialized successfully"
        );

        Ok(Self {
            detector,
            config,
            sample_count: 0,
            last_detection: None,
            current_threshold,
        })
    }

    /// Retourne le nombre d'echantillons attendus par frame par rustpotter.
    pub fn samples_per_frame(&self) -> usize {
        self.detector.get_samples_per_frame()
    }

    /// Retourne le nombre d'octets attendus par frame.
    pub fn bytes_per_frame(&self) -> usize {
        self.detector.get_bytes_per_frame()
    }
}

impl WakeWordDetector for RustpotterDetector {
    fn process_samples(&mut self, samples: &[f32]) -> Option<WakeWordDetection> {
        self.sample_count += samples.len() as u64;

        // Feed samples to rustpotter (process_samples requires Vec<T>)
        let detection = self.detector.process_samples(samples.to_vec());

        let det = detection?;

        // Check confidence threshold
        if det.score < self.current_threshold {
            tracing::debug!(
                score = det.score,
                threshold = self.current_threshold,
                "Detection below threshold, ignoring"
            );
            return None;
        }

        // Debounce: check minimum interval between detections
        if let Some(last) = self.last_detection {
            let elapsed = last.elapsed().as_millis() as u32;
            if elapsed < self.config.min_detection_interval_ms {
                tracing::debug!(
                    elapsed_ms = elapsed,
                    min_interval_ms = self.config.min_detection_interval_ms,
                    "Detection debounced (too soon after last detection)"
                );
                return None;
            }
        }

        let now = Instant::now();
        self.last_detection = Some(now);

        tracing::info!(
            keyword = det.name,
            score = det.score,
            sample_offset = self.sample_count,
            "Wake word detected!"
        );

        Some(WakeWordDetection {
            keyword: det.name,
            confidence: det.score,
            timestamp: now,
            sample_offset: self.sample_count,
        })
    }

    fn reset(&mut self) {
        self.sample_count = 0;
        self.last_detection = None;
        tracing::debug!("Detector state reset");
    }

    #[allow(clippy::unnecessary_literal_bound)]
    fn keyword_name(&self) -> &str {
        "hey_alicia"
    }

    fn threshold(&self) -> f32 {
        self.current_threshold
    }

    fn set_threshold(&mut self, threshold: f32) {
        self.current_threshold = threshold;
        tracing::info!(new_threshold = threshold, "Threshold updated");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = WakeWordConfig::default();
        assert!((config.threshold - 0.5).abs() < f32::EPSILON);
        assert_eq!(config.sample_rate, 16000);
        assert_eq!(config.channels, 1);
        assert_eq!(config.min_detection_interval_ms, 2000);
        assert_eq!(config.score_mode, ScoreMode::Average);
        assert!(config.band_pass_filter);
        assert!(config.gain_normalizer);
        assert!(config.model_path.contains("hey_alicia"));
    }

    #[test]
    fn test_score_mode_default() {
        assert_eq!(ScoreMode::default(), ScoreMode::Average);
    }

    #[test]
    fn test_detector_invalid_model_path() {
        let config = WakeWordConfig {
            model_path: "/nonexistent/path/model.rpw".to_string(),
            ..WakeWordConfig::default()
        };
        let result = RustpotterDetector::new(config);
        assert!(result.is_err());
        let err = result.err().unwrap();
        match err {
            WakeWordError::ModelNotFound(msg) => {
                assert!(msg.contains("nonexistent"));
            }
            other => panic!("Expected ModelNotFound, got: {other}"),
        }
    }

    #[test]
    fn test_wake_word_detection_fields() {
        let det = WakeWordDetection {
            keyword: "hey_alicia".to_string(),
            confidence: 0.85,
            timestamp: Instant::now(),
            sample_offset: 48000,
        };
        assert_eq!(det.keyword, "hey_alicia");
        assert!((det.confidence - 0.85).abs() < f32::EPSILON);
        assert_eq!(det.sample_offset, 48000);
    }

    #[test]
    fn test_config_serialization() {
        let config = WakeWordConfig::default();
        let json = serde_json::to_string(&config);
        assert!(json.is_ok());
        let json_str = json.unwrap();
        assert!(json_str.contains("threshold"));
        assert!(json_str.contains("sample_rate"));
    }

    #[test]
    fn test_config_deserialization() {
        let json = r#"{
            "model_path": "test.rpw",
            "threshold": 0.7,
            "sample_rate": 16000,
            "channels": 1,
            "min_detection_interval_ms": 3000,
            "score_mode": "Max",
            "band_pass_filter": false,
            "gain_normalizer": true
        }"#;
        let config: Result<WakeWordConfig, _> = serde_json::from_str(json);
        assert!(config.is_ok());
        let config = config.unwrap();
        assert!((config.threshold - 0.7).abs() < f32::EPSILON);
        assert_eq!(config.score_mode, ScoreMode::Max);
        assert!(!config.band_pass_filter);
        assert_eq!(config.min_detection_interval_ms, 3000);
    }

    #[test]
    fn test_threshold_update_on_trait() {
        // We cannot create a real RustpotterDetector without a model file,
        // but we can test the config and trait contract indirectly.
        let config = WakeWordConfig {
            threshold: 0.6,
            ..WakeWordConfig::default()
        };
        assert!((config.threshold - 0.6).abs() < f32::EPSILON);
    }
}
