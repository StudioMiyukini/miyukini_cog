//! Erreurs du service KindMother.
//!
//! @id: kindmother_service_errors
//! @do: define_service_errors
//! @layer: core

use thiserror::Error;

/// Erreur du service KindMother.
#[derive(Error, Debug)]
pub enum ServiceError {
    /// Erreur de base de données SQLite.
    #[error("Database error: {0}")]
    Database(String),

    /// Erreur de chiffrement/déchiffrement.
    #[error("Encryption error: {0}")]
    Encryption(String),

    /// Erreur d'authentification.
    #[error("Authentication failed: {0}")]
    Authentication(String),

    /// Erreur d'autorisation (permission refusée).
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Erreur de validation de la requête.
    #[error("Validation error: {0}")]
    Validation(String),

    /// Erreur de quota dépassé.
    #[error("Quota exceeded: {0}")]
    QuotaExceeded(String),

    /// Erreur de transport.
    #[error("Transport error: {0}")]
    Transport(String),

    /// Erreur interne.
    #[error("Internal error: {0}")]
    Internal(String),

    /// Base de données non trouvée.
    #[error("Database not found: {0}")]
    DatabaseNotFound(String),

    /// Opérateur non reconnu.
    #[error("Unknown operator: {0}")]
    UnknownOperator(String),
}

impl From<rusqlite::Error> for ServiceError {
    fn from(err: rusqlite::Error) -> Self {
        ServiceError::Database(err.to_string())
    }
}

impl From<std::io::Error> for ServiceError {
    fn from(err: std::io::Error) -> Self {
        ServiceError::Transport(err.to_string())
    }
}

impl From<serde_json::Error> for ServiceError {
    fn from(err: serde_json::Error) -> Self {
        ServiceError::Validation(err.to_string())
    }
}
