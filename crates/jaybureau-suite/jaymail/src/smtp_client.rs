//! Envoi de mails via SMTP avec `lettre`.

use crate::config::{SmtpConfig, TlsMode};
use crate::message::EmailDraft;
use crate::{JayMailError, Result};
use lettre::message::header::ContentType;
use lettre::message::{Mailbox, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor};

/// Envoie un email via SMTP.
pub async fn send_email(config: &SmtpConfig, draft: &EmailDraft) -> Result<()> {
    draft.validate().map_err(JayMailError::Config)?;

    // Constructeur de message lettre
    let from_email = draft
        .from
        .as_ref()
        .map(|a| a.email.clone())
        .unwrap_or_else(|| config.username.clone());

    let from_mb: Mailbox = from_email
        .parse()
        .map_err(|e: lettre::address::AddressError| JayMailError::Config(e.to_string()))?;

    let mut builder = Message::builder().from(from_mb).subject(&draft.subject);

    for to in &draft.to {
        let mb: Mailbox = to
            .email
            .parse()
            .map_err(|e: lettre::address::AddressError| JayMailError::Config(e.to_string()))?;
        builder = builder.to(mb);
    }
    for cc in &draft.cc {
        let mb: Mailbox = cc
            .email
            .parse()
            .map_err(|e: lettre::address::AddressError| JayMailError::Config(e.to_string()))?;
        builder = builder.cc(mb);
    }
    for bcc in &draft.bcc {
        let mb: Mailbox = bcc
            .email
            .parse()
            .map_err(|e: lettre::address::AddressError| JayMailError::Config(e.to_string()))?;
        builder = builder.bcc(mb);
    }

    if let Some(reply) = &draft.in_reply_to {
        builder = builder.in_reply_to(reply.clone());
    }

    // Body : multipart si HTML present, sinon texte simple
    let message = if let Some(html) = &draft.body_html {
        builder
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_PLAIN)
                            .body(draft.body_text.clone()),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(ContentType::TEXT_HTML)
                            .body(html.clone()),
                    ),
            )
            .map_err(|e| JayMailError::Smtp(e.to_string()))?
    } else {
        builder
            .body(draft.body_text.clone())
            .map_err(|e| JayMailError::Smtp(e.to_string()))?
    };

    // Construire le transport selon le mode TLS
    let creds = Credentials::new(config.username.clone(), config.password.clone());

    let mailer: AsyncSmtpTransport<Tokio1Executor> = match config.tls {
        TlsMode::Tls => AsyncSmtpTransport::<Tokio1Executor>::relay(&config.host)
            .map_err(|e| JayMailError::Smtp(e.to_string()))?
            .port(config.port)
            .credentials(creds)
            .build(),
        TlsMode::StartTls => AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.host)
            .map_err(|e| JayMailError::Smtp(e.to_string()))?
            .port(config.port)
            .credentials(creds)
            .build(),
        TlsMode::None => AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.host)
            .port(config.port)
            .credentials(creds)
            .build(),
    };

    mailer
        .send(message)
        .await
        .map_err(|e| JayMailError::Smtp(e.to_string()))?;

    tracing::info!("Email envoyé: {} → {} destinataires", draft.subject, draft.to.len());
    Ok(())
}
