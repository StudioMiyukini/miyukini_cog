//! @id     miyuwebway_tracker_errors
//! @do     definir_miyuwebway_tracker_error
//! @role   orchestration
//! @layer  core
//! @human  miyuwebway tracker — module « errors ».

/// Erreurs du toolkit MiyuWebwayTracker — une variante par cause de refus (C-11).
///
/// Toute erreur doit être traitée comme un **refus** par l'appelant (C-12).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MiyuwebwayTrackerError {
    /// Aucun mandat gouverné dans le contexte.
    #[error("Execution refused: no governed mandate")]
    NoMandate,
    /// Outil non implémenté : refus explicite (C-14). Porte le ToolId.
    #[error("Tool not yet implemented: {0}")]
    Unimplemented(&'static str),
    /// Tracker (Origin) ou état local indisponible.
    #[error("Tracker unavailable: {0}")]
    TrackerUnavailable(String),
    /// Entrée de liste COG invalide (vide, trop longue, caractères interdits).
    #[error("Invalid COG entry: {0}")]
    InvalidEntry(&'static str),
    /// Liste locale pleine (borne mémoire, C-13).
    #[error("COG list capacity exceeded ({max} entries)")]
    CapacityExceeded {
        /// Nombre maximal d'entrées.
        max: usize,
    },
    /// Hôte invalide pour la résolution d'adresse.
    #[error("Invalid host: {0}")]
    InvalidHost(&'static str),
}
