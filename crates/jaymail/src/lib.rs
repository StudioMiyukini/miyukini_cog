//! Jay Mail — client email Miyukini.
//!
//! Fournit :
//! - Configuration de comptes (SMTP + IMAP)
//! - Envoi via SMTP (`lettre`)
//! - Réception via IMAP (`imap` crate)
//! - Parse RFC822 (`mail-parser`)

pub mod config;
pub mod imap_client;
pub mod mailbox;
pub mod message;
pub mod smtp_client;

pub use config::{Account, ImapConfig, SmtpConfig, TlsMode};
pub use imap_client::{fetch_inbox, fetch_message};
pub use mailbox::{Mailbox, MailboxKind};
pub use message::{Address, Email, EmailDraft};
pub use smtp_client::send_email;

/// Erreurs Jay Mail.
#[derive(Debug, thiserror::Error)]
pub enum JayMailError {
    #[error("Configuration invalide: {0}")]
    Config(String),
    #[error("SMTP: {0}")]
    Smtp(String),
    #[error("IMAP: {0}")]
    Imap(String),
    #[error("Parse: {0}")]
    Parse(String),
    #[error("I/O: {0}")]
    Io(String),
}

pub type Result<T> = std::result::Result<T, JayMailError>;
