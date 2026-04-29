//! Pieces jointes des messages — uploadees chiffrees dans MiyuCloud.

use serde::{Deserialize, Serialize};

/// Piece jointe d'un message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAttachment {
    /// ID MiyuCloud du fichier (deja chiffre at-rest).
    pub media_id: String,
    pub filename: String,
    pub mime_type: String,
    pub size_bytes: u64,
    /// Cle de dechiffrement par-message (pour eviter de rendre la piece accessible aux autres).
    pub decryption_key_hex: Option<String>,
    /// Aperçu (thumbnail base64, pour les images).
    pub thumbnail_data: Option<String>,
}
