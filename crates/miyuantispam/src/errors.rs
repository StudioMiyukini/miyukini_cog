//! @id     miyuantispam_errors
//! @do     definir_miyuantispam_error
//! @role   orchestration
//! @layer  core
//! @human  miyuantispam — module « errors ».

/// Erreurs du toolkit MiyuAntiSpam — une variante par cause de refus (C-11).
///
/// Toute erreur doit être traitée comme un **refus** par l'appelant (C-12).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MiyuantispamError {
    /// Aucun mandat gouverné dans le contexte.
    #[error("Execution refused: no governed mandate")]
    NoMandate,
    /// Outil non implémenté : refus explicite (C-14).
    #[error("Tool not yet implemented: {0}")]
    Unimplemented(&'static str),
    /// Politique de limitation invalide (seuil nul, fenêtre nulle, etc.).
    #[error("Invalid rate limit policy: {0}")]
    InvalidPolicy(&'static str),
    /// Clé de limitation vide ou trop longue.
    #[error("Invalid rate limit key: {0}")]
    InvalidKey(&'static str),
    /// Table des clés suivies pleine (aucune entrée expirée à purger) :
    /// refus fail-closed plutôt qu'éviction d'un compteur actif.
    #[error("Rate limiter capacity exceeded ({max} tracked keys)")]
    CapacityExceeded {
        /// Nombre maximal de clés suivies.
        max: usize,
    },
    /// État interne indisponible (verrou empoisonné) : refus.
    #[error("Rate limiter internal state unavailable")]
    StateUnavailable,
}
