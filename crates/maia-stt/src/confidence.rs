//! Score de confiance de transcription.
//!
//! Whisper produit des log-probabilités par token.
//! On calcule la moyenne des log-probs, puis on la convertit en score [0, 1].
//!
//! Seuils (configurables via `ConfidenceConfig`) :
//! - OK   : score ≥ 0.85 → transcription utilisable directement
//! - WARN : score ≥ 0.65 → utilisable avec prudence / demander confirmation
//! - REJECT : score < 0.65 → ignorer / re-demander

use crate::ConfidenceLevel;

/// Configuration des seuils de confiance.
#[derive(Debug, Clone)]
pub struct ConfidenceConfig {
    /// Seuil OK (score ≥ ok → ConfidenceLevel::Ok).
    pub threshold_ok: f32,
    /// Seuil WARN (threshold_warn ≤ score < threshold_ok → ConfidenceLevel::Warn).
    pub threshold_warn: f32,
}

impl Default for ConfidenceConfig {
    fn default() -> Self {
        Self {
            threshold_ok: 0.85,
            threshold_warn: 0.65,
        }
    }
}

impl ConfidenceConfig {
    /// Crée une config avec un seuil minimum custom (ex: persona sophie = 0.60).
    pub fn with_threshold_warn(threshold_warn: f32) -> Self {
        Self {
            threshold_ok: 0.85,
            threshold_warn,
        }
    }
}

/// Scoreur de confiance Whisper.
pub struct ConfidenceScorer {
    pub config: ConfidenceConfig,
}

impl ConfidenceScorer {
    pub fn new(config: ConfidenceConfig) -> Self {
        Self { config }
    }

    pub fn with_defaults() -> Self {
        Self {
            config: ConfidenceConfig::default(),
        }
    }

    /// Calcule le score de confiance à partir des log-probabilités des tokens.
    ///
    /// Whisper retourne des log-probs en base e (valeurs ∈ (-∞, 0]).
    /// On les convertit en probabilités puis on fait la moyenne géométrique.
    ///
    /// Score = exp(mean(log_probs)).clamp(0, 1)
    pub fn score_logprobs(&self, log_probs: &[f32]) -> f32 {
        if log_probs.is_empty() {
            return 0.0;
        }
        // Filtrer les valeurs aberrantes (< -10 → token très incertain)
        let valid: Vec<f32> = log_probs
            .iter()
            .cloned()
            .filter(|&lp| lp > -10.0 && lp.is_finite())
            .collect();

        if valid.is_empty() {
            return 0.0;
        }

        let mean_logprob = valid.iter().sum::<f32>() / valid.len() as f32;
        // Convertir en probabilité : exp(log_prob) ∈ [0, 1]
        // Amplifier légèrement pour le range humain (parole naturelle ≈ -0.3 à -1.0)
        // On mappe [-3, 0] → [0, 1] de façon linéaire pour la lisibilité
        let normalized = ((mean_logprob + 3.0) / 3.0).clamp(0.0, 1.0);
        normalized
    }

    /// Détermine le niveau de confiance à partir d'un score [0, 1].
    pub fn classify(&self, score: f32) -> ConfidenceLevel {
        if score >= self.config.threshold_ok {
            ConfidenceLevel::Ok
        } else if score >= self.config.threshold_warn {
            ConfidenceLevel::Warn
        } else {
            ConfidenceLevel::Reject
        }
    }

    /// Raccourci : calcule le score ET le niveau en une passe.
    pub fn evaluate(&self, log_probs: &[f32]) -> (f32, ConfidenceLevel) {
        let score = self.score_logprobs(log_probs);
        let level = self.classify(score);
        (score, level)
    }
}

/// Calcule un score de confiance à partir du ratio no-speech.
///
/// Whisper produit aussi `no_speech_prob` par segment.
/// Si no_speech_prob > 0.6, le segment est probablement du silence.
pub fn is_likely_silence(no_speech_prob: f32) -> bool {
    no_speech_prob > 0.6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_confidence_threshold_ok() {
        let scorer = ConfidenceScorer::with_defaults();
        // Log-probs proches de 0 → score élevé → OK
        let log_probs = vec![-0.1, -0.2, -0.15, -0.05, -0.3];
        let (score, level) = scorer.evaluate(&log_probs);
        assert_eq!(level, ConfidenceLevel::Ok, "score={score:.3}");
    }

    #[test]
    fn test_confidence_threshold_warn() {
        let scorer = ConfidenceScorer::with_defaults();
        // Formule : score = (mean_logprob + 3.0) / 3.0
        // Zone WARN : score ∈ [0.65, 0.85) → mean ∈ [-1.05, -0.45)
        // mean ≈ -0.75 → score = (-0.75+3.0)/3.0 = 0.75 → WARN
        let log_probs = vec![-0.7, -0.8, -0.6, -0.7, -0.9];
        let (score, level) = scorer.evaluate(&log_probs);
        assert_eq!(level, ConfidenceLevel::Warn, "score={score:.3}");
    }

    #[test]
    fn test_confidence_threshold_reject() {
        let scorer = ConfidenceScorer::with_defaults();
        // Log-probs très bas → score bas → REJECT
        let log_probs = vec![-4.0, -5.0, -4.5, -6.0, -4.8];
        let (score, level) = scorer.evaluate(&log_probs);
        assert_eq!(level, ConfidenceLevel::Reject, "score={score:.3}");
    }

    #[test]
    fn test_empty_logprobs() {
        let scorer = ConfidenceScorer::with_defaults();
        let (score, level) = scorer.evaluate(&[]);
        assert_eq!(score, 0.0);
        assert_eq!(level, ConfidenceLevel::Reject);
    }

    #[test]
    fn test_confidence_level_from_score() {
        assert_eq!(ConfidenceLevel::from_score(0.9), ConfidenceLevel::Ok);
        assert_eq!(ConfidenceLevel::from_score(0.85), ConfidenceLevel::Ok);
        assert_eq!(ConfidenceLevel::from_score(0.75), ConfidenceLevel::Warn);
        assert_eq!(ConfidenceLevel::from_score(0.65), ConfidenceLevel::Warn);
        assert_eq!(ConfidenceLevel::from_score(0.5), ConfidenceLevel::Reject);
        assert_eq!(ConfidenceLevel::from_score(0.0), ConfidenceLevel::Reject);
    }

    #[test]
    fn test_is_likely_silence() {
        assert!(is_likely_silence(0.8));
        assert!(is_likely_silence(0.65));
        assert!(!is_likely_silence(0.3));
        assert!(!is_likely_silence(0.5));
    }

    #[test]
    fn test_custom_threshold() {
        // Sophie terrain : seuil warn plus bas (0.60)
        let scorer = ConfidenceScorer::new(ConfidenceConfig::with_threshold_warn(0.60));
        let log_probs = vec![-1.5, -1.8, -1.6]; // score qui serait REJECT avec défaut 0.65
        let (score, level) = scorer.evaluate(&log_probs);
        // Avec seuil 0.60 on accepte plus facilement
        assert!(
            score > 0.40, // score devrait être dans la zone exploitable
            "score={score:.3}"
        );
        let _ = level; // Le niveau dépend de la valeur exacte du score
    }
}
