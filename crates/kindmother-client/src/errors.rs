//! Erreurs du client KindMother.
//!
//! @id: kindmother_client_errors
//! @do: define_client_errors
//! @layer: toolkit

use thiserror::Error;

/// Erreur du client KindMother.
#[derive(Error, Debug)]
pub enum ClientError {
    /// Erreur de connexion au service.
    #[error("Connection error: {0}")]
    Connection(String),

    /// Erreur de transport.
    #[error("Transport error: {0}")]
    Transport(String),

    /// Erreur retournée par le service.
    #[error("Service error: {0}")]
    Service(String),

    /// Erreur de sérialisation/désérialisation.
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Permission refusée.
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Base de données non trouvée.
    #[error("Database not found: {0}")]
    DatabaseNotFound(String),

    /// Quota dépassé.
    #[error("Quota exceeded: {0}")]
    QuotaExceeded(String),

    /// Client non connecté.
    #[error("Client not connected")]
    NotConnected,

    /// Requête invalide.
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Timeout.
    #[error("Timeout: {0}")]
    Timeout(String),
}

impl From<std::io::Error> for ClientError {
    fn from(err: std::io::Error) -> Self {
        ClientError::Transport(err.to_string())
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(err: serde_json::Error) -> Self {
        ClientError::Serialization(err.to_string())
    }
}
