//! Types Email, Address, Draft.

use serde::{Deserialize, Serialize};

/// Adresse email + nom optionnel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Address {
    pub email: String,
    pub name: Option<String>,
}

impl Address {
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            name: None,
        }
    }

    pub fn with_name(email: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            email: email.into(),
            name: Some(name.into()),
        }
    }

    /// Format RFC822 ("John Doe <john@example.com>" ou "john@example.com").
    pub fn to_rfc822(&self) -> String {
        match &self.name {
            Some(n) => format!("\"{n}\" <{}>", self.email),
            None => self.email.clone(),
        }
    }
}

/// Email recu (parsed depuis IMAP).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email {
    /// UID IMAP (stable dans un dossier).
    pub uid: u32,
    pub message_id: Option<String>,
    pub from: Address,
    pub to: Vec<Address>,
    pub cc: Vec<Address>,
    pub bcc: Vec<Address>,
    pub subject: String,
    pub date: chrono::DateTime<chrono::Utc>,
    /// Corps en texte brut (toujours present, fallback depuis HTML si necessaire).
    pub body_text: String,
    /// Corps HTML (si disponible).
    pub body_html: Option<String>,
    pub attachments: Vec<Attachment>,
    pub flags: EmailFlags,
}

/// Flags IMAP standard.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailFlags {
    pub seen: bool,
    pub flagged: bool,
    pub answered: bool,
    pub draft: bool,
    pub deleted: bool,
}

/// Piece jointe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attachment {
    pub filename: String,
    pub mime_type: String,
    pub size: u64,
    /// Contenu deja telecharge (si fetch complet) ou index dans le message.
    pub content_id: String,
}

/// Brouillon d'email a envoyer.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmailDraft {
    pub from: Option<Address>,
    pub to: Vec<Address>,
    pub cc: Vec<Address>,
    pub bcc: Vec<Address>,
    pub subject: String,
    pub body_text: String,
    pub body_html: Option<String>,
    pub attachments: Vec<DraftAttachment>,
    pub in_reply_to: Option<String>,
    pub references: Vec<String>,
}

/// Piece jointe d'un brouillon.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftAttachment {
    pub filename: String,
    pub mime_type: String,
    pub data: Vec<u8>,
}

impl EmailDraft {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn to(mut self, addr: Address) -> Self {
        self.to.push(addr);
        self
    }

    pub fn subject(mut self, s: impl Into<String>) -> Self {
        self.subject = s.into();
        self
    }

    pub fn body(mut self, b: impl Into<String>) -> Self {
        self.body_text = b.into();
        self
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.to.is_empty() {
            return Err("Au moins un destinataire requis".into());
        }
        if self.subject.is_empty() {
            return Err("Sujet requis".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_rfc822() {
        let a = Address::with_name("john@example.com", "John Doe");
        assert_eq!(a.to_rfc822(), "\"John Doe\" <john@example.com>");
        let b = Address::new("nobody@example.com");
        assert_eq!(b.to_rfc822(), "nobody@example.com");
    }

    #[test]
    fn draft_validate() {
        let d = EmailDraft::new();
        assert!(d.validate().is_err());

        let d = EmailDraft::new()
            .to(Address::new("test@example.com"))
            .subject("Hello");
        assert!(d.validate().is_ok());
    }
}
