//! Types d'erreur centraux pour MiyuCloud.
//!
//! @id: miyucloud_errors
//! @do: define_miyucloud_error_types
//! @role: error
//! @layer: infra
//!
//! Enum central `MiyucloudError` couvrant toutes les couches du service :
//! IO, persistance, chiffrement, integrite, permissions, quotas, validation.

use thiserror::Error;

/// Erreur centrale du crate miyucloud.
///
/// Chaque variante mappe un domaine d'erreur specifique afin que les couches
/// superieures (API, domaine) puissent pattern-matcher sans perdre le contexte.
#[derive(Debug, Error)]
pub enum MiyucloudError {
    /// Erreur d'entree/sortie (filesystem, reseau).
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Erreur de base de donnees (KindMother / SQLite).
    #[error("Database error: {0}")]
    Db(String),

    /// Erreur de chiffrement ou dechiffrement.
    #[error("Crypto error: {0}")]
    Crypto(String),

    /// Ressource introuvable (fichier, dossier, lien de partage).
    #[error("Not found: {0}")]
    NotFound(String),

    /// Acces refuse (permissions insuffisantes).
    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    /// Quota de stockage depasse.
    #[error("Quota exceeded: {0}")]
    QuotaExceeded(String),

    /// Donnees d'entree invalides.
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Verification d'integrite echouee (SHA-256, HMAC).
    #[error("Integrity check failed: {0}")]
    Integrity(String),

    /// Erreur de serialisation/deserialisation.
    #[error("Serialization error: {0}")]
    Serialization(String),
}

#[cfg(feature = "legacy-sqlite")]
impl From<rusqlite::Error> for MiyucloudError {
    fn from(e: rusqlite::Error) -> Self {
        Self::Db(e.to_string())
    }
}

impl From<serde_json::Error> for MiyucloudError {
    fn from(e: serde_json::Error) -> Self {
        Self::Serialization(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display_io() {
        let err = MiyucloudError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file missing",
        ));
        assert!(err.to_string().contains("IO error"));
        assert!(err.to_string().contains("file missing"));
    }

    #[test]
    fn error_display_db() {
        let err = MiyucloudError::Db("connection refused".into());
        assert_eq!(err.to_string(), "Database error: connection refused");
    }

    #[test]
    fn error_display_crypto() {
        let err = MiyucloudError::Crypto("bad key".into());
        assert_eq!(err.to_string(), "Crypto error: bad key");
    }

    #[test]
    fn error_display_not_found() {
        let err = MiyucloudError::NotFound("file abc".into());
        assert_eq!(err.to_string(), "Not found: file abc");
    }

    #[test]
    fn error_display_permission_denied() {
        let err = MiyucloudError::PermissionDenied("read only".into());
        assert_eq!(err.to_string(), "Permission denied: read only");
    }

    #[test]
    fn error_display_quota_exceeded() {
        let err = MiyucloudError::QuotaExceeded("10GB limit".into());
        assert_eq!(err.to_string(), "Quota exceeded: 10GB limit");
    }

    #[test]
    fn error_display_invalid_input() {
        let err = MiyucloudError::InvalidInput("empty name".into());
        assert_eq!(err.to_string(), "Invalid input: empty name");
    }

    #[test]
    fn error_display_integrity() {
        let err = MiyucloudError::Integrity("checksum mismatch".into());
        assert_eq!(err.to_string(), "Integrity check failed: checksum mismatch");
    }

    #[test]
    fn error_display_serialization() {
        let err = MiyucloudError::Serialization("invalid json".into());
        assert_eq!(err.to_string(), "Serialization error: invalid json");
    }
}
