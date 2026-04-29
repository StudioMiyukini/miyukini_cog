//! Reception de mails via IMAP.
//!
//! Note MVP : implementation simplifiee retournant des emails demo.
//! L'integration complete `imap` + `mail-parser` sera cablee dans une phase ulterieure
//! (necessite un serveur IMAP de test pour valider le flow).

use crate::config::ImapConfig;
use crate::message::{Address, Email, EmailFlags};
use crate::Result;

/// Retourne les N derniers messages de la boite Inbox.
///
/// MVP : retourne 3 emails de demonstration. Pour activer le vrai IMAP,
/// il suffit de remplacer le corps de cette fonction par un appel a
/// `imap::ClientBuilder::new(&config.host, config.port).native_tls()...`
pub fn fetch_inbox(_config: &ImapConfig, _limit: u32) -> Result<Vec<Email>> {
    Ok(demo_emails())
}

/// Recupere un message specifique.
pub fn fetch_message(_config: &ImapConfig, _folder: &str, uid: u32) -> Result<Email> {
    let mut emails = demo_emails();
    if let Some(idx) = emails.iter().position(|e| e.uid == uid) {
        Ok(emails.remove(idx))
    } else {
        Ok(demo_email(uid, "Email introuvable", "Cet email n'existe pas dans le store."))
    }
}

fn demo_emails() -> Vec<Email> {
    vec![
        demo_email(
            1,
            "Bienvenue sur Jay Mail",
            "Bonjour,\n\nJay Mail est votre nouveau client mail dans la suite Jay Bureau.\n\nFeatures :\n- Envoi via SMTP (lettre)\n- Reception via IMAP\n- Integration avec Jay Drive pour les attachements\n- Carnet d'adresses synchronise avec JayClub\n\nA bientot !\n\nL'equipe Miyukini",
        ),
        demo_email(
            2,
            "Reunion lundi 9h",
            "Salut,\n\nN'oublie pas notre reunion lundi 9h. On parle du roadmap Q2.\n\nCordialement,\nJay",
        ),
        demo_email(
            3,
            "Documents Q1 attaches",
            "Hello,\n\nVoici les documents financiers du Q1. Merci de relire.\n\nA bientot.",
        ),
    ]
}

fn demo_email(uid: u32, subject: &str, body: &str) -> Email {
    Email {
        uid,
        message_id: Some(format!("<msg-{uid}@miyukini-cog.com>")),
        from: Address::with_name("noreply@miyukini-cog.com", "Miyukini"),
        to: vec![Address::new("user@miyukini-cog.com")],
        cc: Vec::new(),
        bcc: Vec::new(),
        subject: subject.to_string(),
        date: chrono::Utc::now() - chrono::Duration::hours(uid as i64 * 2),
        body_text: body.to_string(),
        body_html: None,
        attachments: Vec::new(),
        flags: EmailFlags::default(),
    }
}
