//! @id     miyuvalidate_errors
//! @do     definir_miyu_validate_error
//! @role   rule
//! @layer  core
//! @human  miyuvalidate — module « errors ».

/// Erreurs du toolkit MiyuValidate — une variante par cause de refus (C-11).
///
/// Toute erreur doit être traitée comme un **refus** par l'appelant (C-12).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MiyuValidateError {
    /// Aucun mandat gouverné dans le contexte.
    #[error("Execution refused: no governed mandate")]
    NoMandate,
    /// Fonction non implémentée : refus explicite (C-14).
    #[error("Tool not yet implemented: {0}")]
    Unimplemented(&'static str),
    /// Schéma invalide ou utilisant une construction non supportée
    /// (refus plutôt qu'ignorer une contrainte — fail-closed).
    #[error("Invalid schema: {0}")]
    InvalidSchema(String),
    /// Données invalides (JSON illisible, type attendu absent, nombre non fini…).
    #[error("Invalid data: {0}")]
    InvalidData(String),
    /// Entrée au-delà des bornes (C-13).
    #[error("Input too large: {what} exceeds {max}")]
    InputTooLarge {
        /// Entrée concernée.
        what: &'static str,
        /// Borne appliquée.
        max: usize,
    },
}
