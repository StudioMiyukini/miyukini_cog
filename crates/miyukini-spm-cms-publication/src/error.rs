//! Types d'erreur du Module Publication.

use std::fmt;

/// Erreurs possibles lors des opérations sur les publications.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicationError {
    /// Publication introuvable.
    NotFound,
    /// Transition d'état invalide.
    InvalidTransition,
    /// Date de publication invalide (passée).
    InvalidSchedule,
    /// Erreur générique (validation produit, etc.).
    Other(String),
}

impl fmt::Display for PublicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PublicationError::NotFound => write!(f, "publication not found"),
            PublicationError::InvalidTransition => write!(f, "invalid transition"),
            PublicationError::InvalidSchedule => write!(f, "invalid schedule"),
            PublicationError::Other(msg) => write!(f, "error: {}", msg),
        }
    }
}

impl std::error::Error for PublicationError {}
