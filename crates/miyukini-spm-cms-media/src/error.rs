//! Types d'erreur du Module Médias.

use std::fmt;

/// Erreurs possibles lors des opérations sur les médias.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MediaError {
    /// Média introuvable.
    NotFound,
    /// Entité introuvable (pour les opérations d'attachement).
    EntityNotFound,
    /// Contrainte violée (ex. média déjà attaché à une entité).
    ConstraintViolation(String),
    /// Erreur générique (validation produit, etc.).
    Other(String),
}

impl fmt::Display for MediaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MediaError::NotFound => write!(f, "media not found"),
            MediaError::EntityNotFound => write!(f, "entity not found"),
            MediaError::ConstraintViolation(msg) => write!(f, "constraint violation: {}", msg),
            MediaError::Other(msg) => write!(f, "error: {}", msg),
        }
    }
}

impl std::error::Error for MediaError {}
