//! @id     jaybureau_suite_jaymessage_lib
//! @do     declarer_module_attachments
//! @role   ui
//! @layer  core
//! @human  jaybureau-suite — module « jaymessage lib ».

//! Jay Message — messagerie chiffrée de bout en bout.
//!
//! ## Modèle cryptographique
//!
//! - **Identité** : chaque utilisateur a une paire ed25519 long-terme (signature)
//! - **Sessions 1:1** : X25519 ephémérale **signée par l'identité ed25519**
//!   (`verify_strict` avant dérivation) → HKDF-SHA256 → deux clés
//!   directionnelles XChaCha20-Poly1305, nonce aléatoire 192 bits interne
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
pub use session::{HandshakeOffer, PendingHandshake, SessionKeys, SessionStore};

/// Version du protocole de messagerie (2 : handshake signé, XChaCha20, R-009).
/// Les enveloppes et offres de la v1 (X25519 non signé, nonce 12 octets)
/// ne sont plus acceptées.
pub const PROTOCOL_VERSION: u8 = 2;

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
