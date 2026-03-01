//! @id toolkit.alicia.wakeword.errors
//! @do define_error_types_for_wake_word_detection
//! @role error_definition
//! @layer 6
//! @human Types d'erreur explicites pour la detection de wake word

/// Erreurs possibles lors de la detection de wake word.
#[derive(Debug, Clone, thiserror::Error)]
pub enum WakeWordError {
    /// Aucun mandat de gouvernance fourni.
    #[error("Execution refused: no governed mandate")]
    NoMandate,

    /// Fichier modele introuvable.
    #[error("Model file not found: {0}")]
    ModelNotFound(String),

    /// Fichier modele corrompu ou version incompatible.
    #[error("Model file invalid: {0}")]
    ModelInvalid(String),

    /// Erreur de configuration du detecteur.
    #[error("Detector configuration error: {0}")]
    Config(String),

    /// Erreur lors du traitement audio.
    #[error("Audio processing error: {0}")]
    AudioProcessing(String),

    /// Fonctionnalite non implementee.
    #[error("Feature not yet implemented")]
    Unimplemented,
}
