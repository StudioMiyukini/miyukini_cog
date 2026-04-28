//! Modele de conversation.

use crate::{ConversationId, UserId};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Type de conversation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConversationKind {
    /// 1:1 entre 2 utilisateurs.
    Direct,
    /// Groupe (3+ membres).
    Group,
}

/// Conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: ConversationId,
    pub kind: ConversationKind,
    /// Participants (user_ids).
    pub members: Vec<UserId>,
    /// Nom (uniquement pour les groupes).
    pub name: Option<String>,
    /// ID MiyuCloud de l'avatar (groupes).
    pub avatar_media_id: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    /// Dernier message (pour preview).
    pub last_message_preview: Option<String>,
    /// Compteur de messages non-lus.
    pub unread_count: u32,
}

impl Conversation {
    pub fn direct(user_a: UserId, user_b: UserId) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: format!("conv:{}", Uuid::new_v4()),
            kind: ConversationKind::Direct,
            members: vec![user_a, user_b],
            name: None,
            avatar_media_id: None,
            created_at: now,
            updated_at: now,
            last_message_preview: None,
            unread_count: 0,
        }
    }

    pub fn group(name: String, members: Vec<UserId>) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: format!("conv:{}", Uuid::new_v4()),
            kind: ConversationKind::Group,
            members,
            name: Some(name),
            avatar_media_id: None,
            created_at: now,
            updated_at: now,
            last_message_preview: None,
            unread_count: 0,
        }
    }
}

/// Statut d'un message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageStatus {
    /// Brouillon (pas encore envoye).
    Draft,
    /// En cours d'envoi.
    Sending,
    /// Envoye au serveur, pas encore confirme par le destinataire.
    Sent,
    /// Recu par le destinataire (ack).
    Delivered,
    /// Lu par le destinataire.
    Read,
    /// Echec d'envoi.
    Failed,
}

/// Message en clair (cote local, apres dechiffrement).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub conversation_id: ConversationId,
    pub sender_id: UserId,
    pub text: String,
    /// IDs des pieces jointes (MiyuCloud).
    pub attachments: Vec<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub status: MessageStatus,
    pub reply_to: Option<String>,
    pub edited_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl Message {
    pub fn new(conversation_id: ConversationId, sender_id: UserId, text: String) -> Self {
        Self {
            id: format!("msg:{}", Uuid::new_v4()),
            conversation_id,
            sender_id,
            text,
            attachments: Vec::new(),
            timestamp: chrono::Utc::now(),
            status: MessageStatus::Sending,
            reply_to: None,
            edited_at: None,
        }
    }
}
