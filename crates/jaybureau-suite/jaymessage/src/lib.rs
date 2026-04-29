//! Jay Message — messagerie chiffrée de bout en bout.
//!
//! ## Modèle cryptographique
//!
//! - **Identité** : chaque utilisateur a une paire ed25519 long-terme (signature)
//! - **Sessions 1:1** : X25519 ephémérale → secret partagé → ChaCha20-Poly1305
//! - **Groupes** : Sender Keys (clé symétrique commune), enveloppée par session 1:1 pour
//!   chaque membre, rotation à chaque changement de membership
//! - **Forward secrecy** : nouvelle clé ephémérale par session
//! - **Le serveur Origin ne voit que des EncryptedEnvelope opaques (relay only)**

pub mod attachments;
pub mod conversation;
pub mod groups;
pub mod identity;
pub mod protocol;
pub mod session;

pub use conversation::{Conversation, ConversationKind, Message, MessageStatus};
pub use groups::{GroupKey, SenderKeyDistribution};
pub use identity::IdentityKey;
pub use protocol::{EncryptedEnvelope, MessageEnvelope, MessageType};
pub use session::SessionStore;

/// Identifiant utilisateur.
pub type UserId = String;

/// Identifiant de conversation (1:1 ou groupe).
pub type ConversationId = String;

/// Erreurs.
#[derive(Debug, thiserror::Error)]
pub enum JayMessageError {
    #[error("Crypto: {0}")]
    Crypto(String),
    #[error("Session introuvable: {0}")]
    NoSession(String),
    #[error("Conversation introuvable: {0}")]
    NoConversation(ConversationId),
    #[error("Membre absent: {0}")]
    NotMember(UserId),
    #[error("Format invalide: {0}")]
    Invalid(String),
}

pub type Result<T> = std::result::Result<T, JayMessageError>;
