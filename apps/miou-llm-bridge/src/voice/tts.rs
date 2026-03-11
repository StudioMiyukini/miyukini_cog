//! Gestion TTS — synthèse vocale via le sous-processus Piper.
//!
//! Piper lit le texte sur stdin et écrit du PCM WAV sur stdout.
//! On pilotera via un `std::process::Command` + `std::io::Write`.
//!
//! Binaire attendu : `piper` dans le PATH ou chemin configuré.
//! Modèle de voix : fichier `.onnx` + fichier `.onnx.json` (config).

use std::io::Write;
use std::path::PathBuf;
use std::process::{Child, ChildStdin, Command, Stdio};

/// Configuration du sous-processus Piper.
#[derive(Debug, Clone)]
pub struct PiperConfig {
    /// Chemin vers le binaire piper (ex: "/usr/local/bin/piper" ou "piper").
    pub binary: String,
    /// Chemin vers le modèle ONNX (ex: "fr_FR-siwis-medium.onnx").
    pub model_path: PathBuf,
    /// Vitesse de synthèse (1.0 = normal, 0.8 = lent, 1.2 = rapide).
    pub speed: f64,
    /// Taux d'échantillonnage de sortie (Hz) — doit correspondre au modèle.
    pub output_sample_rate: u32,
}

impl Default for PiperConfig {
    fn default() -> Self {
        Self {
            binary: "piper".into(),
            model_path: PathBuf::from("fr_FR-siwis-medium.onnx"),
            speed: 1.0,
            output_sample_rate: 22_050,
        }
    }
}

/// Erreur TTS.
#[derive(Debug)]
pub enum TtsError {
    ProcessSpawn(String),
    Io(String),
    ProcessFailed(String),
    AudioDecode(String),
}

impl std::fmt::Display for TtsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProcessSpawn(e) => write!(f, "Échec lancement piper : {e}"),
            Self::Io(e) => write!(f, "Erreur I/O piper : {e}"),
            Self::ProcessFailed(e) => write!(f, "Piper échoué : {e}"),
            Self::AudioDecode(e) => write!(f, "Décodage audio piper : {e}"),
        }
    }
}

/// Synthétise du texte en audio WAV via Piper.
///
/// Retourne les bytes WAV bruts produits par piper sur stdout.
/// L'appelant peut décoder le WAV ou le jouer directement.
pub fn synthesize_wav(text: &str, config: &PiperConfig) -> Result<Vec<u8>, TtsError> {
    let mut child: Child = Command::new(&config.binary)
        .arg("--model")
        .arg(&config.model_path)
        .arg("--output_raw") // PCM raw sur stdout (pas de header WAV)
        .arg("--length_scale")
        .arg((1.0 / config.speed).to_string()) // length_scale = 1/speed
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|e| TtsError::ProcessSpawn(e.to_string()))?;

    // Écrire le texte sur stdin et fermer
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(text.as_bytes())
            .map_err(|e| TtsError::Io(e.to_string()))?;
        // stdin fermé à la fin du bloc → piper reçoit EOF et commence le rendu
    }

    let output = child
        .wait_with_output()
        .map_err(|e| TtsError::Io(e.to_string()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(TtsError::ProcessFailed(format!(
            "exit={} stderr={}",
            output.status,
            stderr.trim()
        )));
    }

    Ok(output.stdout)
}

/// Convertit les bytes PCM 16-bit LE (raw piper --output_raw) en f32 mono.
pub fn pcm16_to_f32(raw: &[u8]) -> Vec<f32> {
    raw.chunks_exact(2)
        .map(|c| {
            let sample = i16::from_le_bytes([c[0], c[1]]);
            sample as f32 / 32768.0
        })
        .collect()
}

/// Lance une instance Piper en mode streaming (stdin ouvert).
///
/// Permet d'envoyer plusieurs segments de texte successifs
/// sans relancer le processus. Appeler `finish()` pour obtenir l'audio.
pub struct PiperStream {
    child: Child,
    stdin: Option<ChildStdin>,
}

impl PiperStream {
    /// Démarre Piper en mode streaming.
    pub fn start(config: &PiperConfig) -> Result<Self, TtsError> {
        let mut child = Command::new(&config.binary)
            .arg("--model")
            .arg(&config.model_path)
            .arg("--output_raw")
            .arg("--length_scale")
            .arg((1.0 / config.speed).to_string())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| TtsError::ProcessSpawn(e.to_string()))?;

        let stdin = child.stdin.take();
        Ok(Self { child, stdin })
    }

    /// Envoie un segment de texte à synthétiser.
    pub fn send(&mut self, text: &str) -> Result<(), TtsError> {
        if let Some(ref mut stdin) = self.stdin {
            stdin
                .write_all(text.as_bytes())
                .map_err(|e| TtsError::Io(e.to_string()))?;
            stdin
                .write_all(b"\n")
                .map_err(|e| TtsError::Io(e.to_string()))?;
        }
        Ok(())
    }

    /// Ferme stdin et attend la fin du processus. Retourne les bytes PCM raw.
    pub fn finish(mut self) -> Result<Vec<u8>, TtsError> {
        // Fermer stdin → piper reçoit EOF
        drop(self.stdin.take());

        let output = self
            .child
            .wait_with_output()
            .map_err(|e| TtsError::Io(e.to_string()))?;

        if !output.status.success() {
            return Err(TtsError::ProcessFailed(format!("exit={}", output.status)));
        }

        Ok(output.stdout)
    }
}
