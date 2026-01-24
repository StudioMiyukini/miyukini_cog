//! Types d'erreur du Module Contenu.

use std::fmt;

/// Erreurs possibles lors des opérations sur les contenus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentError {
    /// Contenu introuvable.
    NotFound,
    /// Statut invalide (transition non autorisée).
    InvalidStatus,
    /// Relation invalide (ex. cycle, contenu inexistant).
    InvalidRelation,
    /// Version introuvable.
    VersionNotFound,
    /// Versioning non activé pour ce contenu.
    VersioningNotEnabled,
    /// Contrainte violée (ex. suppression avec contenus enfants).
    ConstraintViolation(String),
    /// Erreur générique (validation produit, etc.).
    Other(String),
}

impl fmt::Display for ContentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContentError::NotFound => write!(f, "content not found"),
            ContentError::InvalidStatus => write!(f, "invalid status"),
            ContentError::InvalidRelation => write!(f, "invalid relation"),
            ContentError::VersionNotFound => write!(f, "version not found"),
            ContentError::VersioningNotEnabled => write!(f, "versioning not enabled"),
            ContentError::ConstraintViolation(msg) => write!(f, "constraint violation: {}", msg),
            ContentError::Other(msg) => write!(f, "error: {}", msg),
        }
    }
}

impl std::error::Error for ContentError {}
