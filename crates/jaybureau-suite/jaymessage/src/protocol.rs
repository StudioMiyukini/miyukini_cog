//! Types protocole pour les messages chiffres.

use crate::{ConversationId, UserId};
use serde::{Deserialize, Serialize};

/// Type de message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    /// Texte normal.
    Text,
    /// Image (via MiyuCloud).
    Image,
    /// Video.
    Video,
    /// Fichier generique.
    File,
    /// Audio (note vocale).
    Audio,
    /// Marqueur "X est en train d'ecrire".
    TypingIndicator,
    /// Marqueur de lecture.
    ReadReceipt,
    /// Distribution d'une cle de groupe (Sender Key).
    GroupKeyDistribution,
}

/// Enveloppe de message en clair (avant chiffrement, cote sender).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEnvelope {
    pub message_id: String,
    pub conversation_id: ConversationId,
    pub sender_id: UserId,
    pub recipient_ids: Vec<UserId>,
    pub message_type: MessageType,
    pub payload: Vec<u8>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Reference a un message parent (pour reply).
    pub reply_to: Option<String>,
}

/// Enveloppe chiffree transmise sur le reseau.
///
/// Le serveur Origin ne voit que ces enveloppes — il ne peut pas dechiffrer
/// le `ciphertext` ni voir le contenu reel du message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedEnvelope {
    pub message_id: String,
    pub conversation_id: ConversationId,
    pub sender_id: UserId,
    /// Destinataire individuel (le sender envoie une enveloppe par destinataire).
    pub recipient_id: UserId,
    /// Nonce ChaCha20 (12 bytes hex).
    pub nonce: String,
    /// Ciphertext + tag Poly1305 (hex).
    pub ciphertext: String,
    /// Signature ed25519 du sender sur (nonce || ciphertext).
    pub signature: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
