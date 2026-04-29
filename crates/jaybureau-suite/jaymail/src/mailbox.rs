//! Gestion des dossiers de mail (Inbox, Sent, Drafts, Trash).

use serde::{Deserialize, Serialize};

/// Type de dossier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MailboxKind {
    Inbox,
    Sent,
    Drafts,
    Trash,
    Spam,
    Archive,
    Custom,
}

impl MailboxKind {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Inbox => "Boîte de réception",
            Self::Sent => "Envoyés",
            Self::Drafts => "Brouillons",
            Self::Trash => "Corbeille",
            Self::Spam => "Spam",
            Self::Archive => "Archivés",
            Self::Custom => "Dossier",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Self::Inbox => "📥",
            Self::Sent => "📤",
            Self::Drafts => "📝",
            Self::Trash => "🗑",
            Self::Spam => "🚫",
            Self::Archive => "📦",
            Self::Custom => "📁",
        }
    }

    pub fn imap_name(&self) -> &'static str {
        match self {
            Self::Inbox => "INBOX",
            Self::Sent => "Sent",
            Self::Drafts => "Drafts",
            Self::Trash => "Trash",
            Self::Spam => "Junk",
            Self::Archive => "Archive",
            Self::Custom => "",
        }
    }

    pub fn standard() -> &'static [MailboxKind] {
        &[
            Self::Inbox,
            Self::Sent,
            Self::Drafts,
            Self::Spam,
            Self::Trash,
        ]
    }
}

/// Boite mail (dossier).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mailbox {
    pub kind: MailboxKind,
    pub name: String,
    pub unread_count: u32,
    pub total_count: u32,
}
