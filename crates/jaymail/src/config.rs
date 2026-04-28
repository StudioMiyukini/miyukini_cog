//! Configuration de comptes mail.

use serde::{Deserialize, Serialize};

/// Mode TLS pour la connexion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TlsMode {
    /// TLS direct (port 465 SMTPS, 993 IMAPS).
    Tls,
    /// STARTTLS (port 587 SMTP submission, 143 IMAP).
    StartTls,
    /// Pas de chiffrement (deconseille).
    None,
}

/// Configuration SMTP.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub tls: TlsMode,
}

impl SmtpConfig {
    /// Defaults Miyukini (mail.miyukini.com:587 STARTTLS).
    pub fn miyukini_default(username: String, password: String) -> Self {
        Self {
            host: "mail.miyukini.com".into(),
            port: 587,
            username,
            password,
            tls: TlsMode::StartTls,
        }
    }
}

/// Configuration IMAP.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImapConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub tls: TlsMode,
}

impl ImapConfig {
    /// Defaults Miyukini (mail.miyukini.com:993 TLS direct).
    pub fn miyukini_default(username: String, password: String) -> Self {
        Self {
            host: "mail.miyukini.com".into(),
            port: 993,
            username,
            password,
            tls: TlsMode::Tls,
        }
    }
}

/// Compte mail complet (SMTP + IMAP).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub email: String,
    pub display_name: String,
    pub smtp: SmtpConfig,
    pub imap: ImapConfig,
}

impl Account {
    /// Compte Miyukini standard avec defaults serveur.
    pub fn miyukini(email: String, display_name: String, password: String) -> Self {
        Self {
            email: email.clone(),
            display_name,
            smtp: SmtpConfig::miyukini_default(email.clone(), password.clone()),
            imap: ImapConfig::miyukini_default(email, password),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn miyukini_account_uses_correct_servers() {
        let acc = Account::miyukini(
            "user@miyukini-cog.com".into(),
            "User".into(),
            "pass".into(),
        );
        assert_eq!(acc.smtp.host, "mail.miyukini.com");
        assert_eq!(acc.smtp.port, 587);
        assert_eq!(acc.imap.host, "mail.miyukini.com");
        assert_eq!(acc.imap.port, 993);
        assert_eq!(acc.smtp.tls, TlsMode::StartTls);
        assert_eq!(acc.imap.tls, TlsMode::Tls);
    }
}
