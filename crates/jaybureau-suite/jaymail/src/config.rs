//! @id     jaybureau_suite_jaymail_config
//! @do     definir_tls_mode
//! @role   ui
//! @layer  core
//! @human  jaybureau-suite — module « jaymail config ».

//! Configuration de comptes mail.

use crate::{JayMailError, Result};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use zeroize::Zeroizing;

/// Mode TLS pour la connexion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TlsMode {
    /// TLS direct (port 465 SMTPS, 993 IMAPS).
    Tls,
    /// STARTTLS (port 587 SMTP submission, 143 IMAP).
    StartTls,
    /// Pas de chiffrement : refuse sauf adresse IP de bouclage **et**
    /// `allow_plaintext_loopback = true` (relais local de test).
    None,
}

/// Vrai si `host` est une adresse IP littérale de bouclage (127.0.0.0/8, ::1).
///
/// Les noms (y compris `localhost`) ne sont pas acceptés : leur résolution
/// n'est pas maîtrisée (fail-closed, C-12).
pub fn is_loopback_host(host: &str) -> bool {
    let h = host.trim_start_matches('[').trim_end_matches(']');
    h.parse::<IpAddr>().is_ok_and(|ip| ip.is_loopback())
}

/// Contrôle commun du mode TLS (R-011).
fn check_tls(host: &str, tls: TlsMode, allow_plaintext_loopback: bool) -> Result<()> {
    match tls {
        TlsMode::Tls | TlsMode::StartTls => Ok(()),
        TlsMode::None if allow_plaintext_loopback && is_loopback_host(host) => Ok(()),
        TlsMode::None => Err(JayMailError::Config(format!(
            "connexion sans TLS refusée vers {host} (autorisée uniquement en bouclage avec allow_plaintext_loopback)"
        ))),
    }
}

/// Configuration SMTP.
///
/// Le mot de passe est effacé de la mémoire à la destruction et masqué dans
/// `Debug` (C-25).
#[derive(Clone, Serialize, Deserialize)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Zeroizing<String>,
    pub tls: TlsMode,
    /// Autorise `TlsMode::None` vers une IP de bouclage (désactivé par défaut).
    #[serde(default)]
    pub allow_plaintext_loopback: bool,
}

impl std::fmt::Debug for SmtpConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SmtpConfig")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("username", &self.username)
            .field("password", &"<redacted>")
            .field("tls", &self.tls)
            .field("allow_plaintext_loopback", &self.allow_plaintext_loopback)
            .finish()
    }
}

impl SmtpConfig {
    /// Defaults Miyukini (mail.miyukini.com:587 STARTTLS).
    pub fn miyukini_default(username: String, password: String) -> Self {
        Self {
            host: "mail.miyukini.com".into(),
            port: 587,
            username,
            password: Zeroizing::new(password),
            tls: TlsMode::StartTls,
            allow_plaintext_loopback: false,
        }
    }

    /// Vérifie la configuration avant connexion (TLS obligatoire hors bouclage).
    pub fn validate(&self) -> Result<()> {
        if self.host.is_empty() {
            return Err(JayMailError::Config("hôte SMTP vide".into()));
        }
        check_tls(&self.host, self.tls, self.allow_plaintext_loopback)
    }
}

/// Configuration IMAP.
#[derive(Clone, Serialize, Deserialize)]
pub struct ImapConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Zeroizing<String>,
    pub tls: TlsMode,
    /// Autorise `TlsMode::None` vers une IP de bouclage (désactivé par défaut).
    #[serde(default)]
    pub allow_plaintext_loopback: bool,
}

impl std::fmt::Debug for ImapConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ImapConfig")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("username", &self.username)
            .field("password", &"<redacted>")
            .field("tls", &self.tls)
            .field("allow_plaintext_loopback", &self.allow_plaintext_loopback)
            .finish()
    }
}

impl ImapConfig {
    /// Defaults Miyukini (mail.miyukini.com:993 TLS direct).
    pub fn miyukini_default(username: String, password: String) -> Self {
        Self {
            host: "mail.miyukini.com".into(),
            port: 993,
            username,
            password: Zeroizing::new(password),
            tls: TlsMode::Tls,
            allow_plaintext_loopback: false,
        }
    }

    /// Vérifie la configuration avant connexion (TLS obligatoire hors bouclage).
    pub fn validate(&self) -> Result<()> {
        if self.host.is_empty() {
            return Err(JayMailError::Config("hôte IMAP vide".into()));
        }
        check_tls(&self.host, self.tls, self.allow_plaintext_loopback)
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
        assert!(acc.smtp.validate().is_ok());
        assert!(acc.imap.validate().is_ok());
    }

    #[test]
    fn debug_does_not_contain_password() {
        let acc = Account::miyukini(
            "user@miyukini-cog.com".into(),
            "User".into(),
            "s3cr3t-P@ss".into(),
        );
        for dbg in [
            format!("{acc:?}"),
            format!("{:?}", acc.smtp),
            format!("{:?}", acc.imap),
            format!("{acc:#?}"),
        ] {
            assert!(!dbg.contains("s3cr3t-P@ss"), "mot de passe dans Debug: {dbg}");
            assert!(dbg.contains("<redacted>"));
        }
    }

    #[test]
    fn tls_none_to_remote_host_refused() {
        let mut smtp = SmtpConfig::miyukini_default("u".into(), "p".into());
        smtp.tls = TlsMode::None;
        assert!(smtp.validate().is_err());
        // Même avec l'option : hôte distant refusé.
        smtp.allow_plaintext_loopback = true;
        assert!(smtp.validate().is_err());
        // `localhost` (nom) refusé : seule une IP de bouclage est admise.
        smtp.host = "localhost".into();
        assert!(smtp.validate().is_err());
        smtp.host = "127.0.0.1.evil.com".into();
        assert!(smtp.validate().is_err());

        let mut imap = ImapConfig::miyukini_default("u".into(), "p".into());
        imap.tls = TlsMode::None;
        imap.allow_plaintext_loopback = true;
        assert!(imap.validate().is_err());
    }

    #[test]
    fn tls_none_loopback_requires_opt_in() {
        let mut smtp = SmtpConfig::miyukini_default("u".into(), "p".into());
        smtp.tls = TlsMode::None;
        smtp.host = "127.0.0.1".into();
        assert!(smtp.validate().is_err());
        smtp.allow_plaintext_loopback = true;
        assert!(smtp.validate().is_ok());
        smtp.host = "[::1]".into();
        assert!(smtp.validate().is_ok());
    }

    #[tokio::test]
    async fn send_email_refuses_plaintext_remote() {
        let mut smtp = SmtpConfig::miyukini_default("u@miyukini.com".into(), "p".into());
        smtp.tls = TlsMode::None;
        smtp.host = "192.0.2.1".into();
        let draft = crate::EmailDraft {
            to: vec![crate::Address {
                email: "dest@example.com".into(),
                name: None,
            }],
            subject: "s".into(),
            body_text: "b".into(),
            ..Default::default()
        };
        let err = crate::send_email(&smtp, &draft).await;
        assert!(matches!(err, Err(JayMailError::Config(_))));
    }
}
