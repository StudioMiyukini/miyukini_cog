//! Erreurs du kit MiyuWebway Participant.

use thiserror::Error;

/// Erreurs possibles du kit MWS Participant.
#[derive(Debug, Clone, Error)]
pub enum MiyuwebwayParticipantError {
    /// Pas de mandat de gouvernance.
    #[error("Execution refused: no governed mandate")]
    NoMandate,

    /// Fonctionnalité non implémentée.
    #[error("Tool not yet implemented")]
    Unimplemented,

    /// Échec de connexion.
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    /// Connexion fermée.
    #[error("Connection closed by remote")]
    ConnectionClosed,

    /// Erreur TLS.
    #[error("TLS error: {0}")]
    TlsError(String),

    /// Domaine invalide.
    #[error("Invalid domain name")]
    InvalidDomain,

    /// Erreur d'envoi.
    #[error("Send error: {0}")]
    SendError(String),

    /// Erreur de lecture.
    #[error("Read error: {0}")]
    ReadError(String),

    /// Payload invalide.
    #[error("Invalid payload format")]
    InvalidPayload,

    /// Message inattendu.
    #[error("Unexpected message type")]
    UnexpectedMessage,

    /// Enregistrement rejeté.
    #[error("Registration rejected: {0}")]
    RegistrationRejected(String),

    /// Annonce échouée.
    #[error("Announce failed: {0}")]
    AnnounceFailed(String),

    /// Erreur du Tracker.
    #[error("Tracker error: {0}")]
    TrackerError(String),

    /// Non connecté au réseau.
    #[error("Not connected to MWS network")]
    NotConnected,

    /// Timeout.
    #[error("Operation timed out")]
    Timeout,

    /// Erreur de protocole.
    #[error("Protocol error: {0}")]
    ProtocolError(String),

    /// Permis invalide ou expiré.
    #[error("Invalid or expired permis")]
    InvalidPermis,

    /// Lobby non trouvé.
    #[error("Lobby not found")]
    LobbyNotFound,

    /// Lobby complet.
    #[error("Lobby is full")]
    LobbyFull,

    /// Mot de passe incorrect.
    #[error("Wrong password")]
    WrongPassword,

    /// COG non trouvé.
    #[error("COG not found")]
    CogNotFound,
}
