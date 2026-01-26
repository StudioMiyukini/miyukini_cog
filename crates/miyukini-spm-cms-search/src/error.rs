//! Types d'erreur du Module Recherche & Indexation.

use std::fmt;

/// Erreurs possibles lors des opérations de recherche et d'indexation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchError {
    /// Entité non indexée.
    EntityNotIndexed,
    /// Champ introuvable dans l'index.
    FieldNotFound,
    /// Opérateur de recherche invalide pour le type de champ.
    InvalidOperator,
    /// Requête invalide (ex. critères vides).
    InvalidQuery(String),
    /// Erreur générique (validation produit, etc.).
    Other(String),
}

impl fmt::Display for SearchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchError::EntityNotIndexed => write!(f, "entity not indexed"),
            SearchError::FieldNotFound => write!(f, "field not found"),
            SearchError::InvalidOperator => write!(f, "invalid operator"),
            SearchError::InvalidQuery(msg) => write!(f, "invalid query: {}", msg),
            SearchError::Other(msg) => write!(f, "error: {}", msg),
        }
    }
}

impl std::error::Error for SearchError {}
