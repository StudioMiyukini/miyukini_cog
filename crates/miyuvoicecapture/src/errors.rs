//! Types d'erreur explicites pour Alicia Capture.
//!
//! Erreurs de capture audio, enumeration devices, VAD, buffer.
//! Pas de `unwrap()` en production -- chaque erreur est typee et propagee.

// @id: alicia_capture_errors
// @role: infrastructure
// @layer: toolkit
// @human: Types d'erreur contractuels Alicia Capture.
// @do: represent_voice_capture_errors

use thiserror::Error;

/// Erreur principale du toolkit Alicia Capture.
///
/// Chaque variante correspond a une categorie d'erreur distincte,
/// permettant un traitement precis par l'appelant.
#[derive(Debug, Error)]
pub enum VoiceCaptureError {
    /// Aucun peripherique audio d'entree disponible.
    #[error("No input audio device available")]
    NoInputDevice,

    /// Aucun peripherique audio de sortie disponible.
    #[error("No output audio device available")]
    NoOutputDevice,

    /// Peripherique audio introuvable par son identifiant.
    #[error("Audio device not found: {0}")]
    DeviceNotFound(String),

    /// Erreur lors de l'enumeration des peripheriques audio.
    #[error("Device enumeration failed: {0}")]
    DeviceEnumeration(String),

    /// Configuration audio non supportee par le peripherique.
    #[error("Unsupported audio config: {0}")]
    UnsupportedConfig(String),

    /// Erreur lors de la construction du stream audio.
    #[error("Stream build failed: {0}")]
    StreamBuild(String),

    /// Erreur lors de la lecture/ecriture du stream audio.
    #[error("Stream playback/capture error: {0}")]
    StreamError(String),

    /// Le buffer circulaire est plein (overflow).
    #[error("Ring buffer overflow: {0} samples lost")]
    BufferOverflow(usize),

    /// Contexte de capture non initialise.
    #[error("Capture context not initialized")]
    ContextNotInitialized,

    /// Configuration VAD invalide.
    #[error("Invalid VAD configuration: {0}")]
    InvalidVadConfig(String),

    /// Erreur interne inattendue.
    #[error("Internal error: {0}")]
    Internal(String),
}
