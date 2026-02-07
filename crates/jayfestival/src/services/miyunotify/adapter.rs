//! Adapter JayFestival → Miyunotify (annonces, notifications ciblées).
//!
//! @id: miyunotify_send_announcement
//! @do: envoie une annonce (broadcast) aux destinataires d'une édition ou d'un canal
//! @layer: domain
//!
//! @id: miyunotify_send_targeted
//! @do: envoie une notification ciblée (email, push ou inbox) à un ou plusieurs destinataires
//! @layer: domain
//!
//! Alpha : appelle Miyunotify (email_send, push_send, inbox_write).

use miyunotify::context::GovernedContext;
use miyunotify::email;
use miyunotify::push;
use miyunotify::inbox;
use miyunotify::errors::MiyunotifyError;
use std::fmt;

/// Contexte gouverné alpha pour les appels Miyunotify depuis JayFestival.
fn jayfestival_notify_ctx() -> GovernedContext {
    GovernedContext::new("jayfestival_alpha".to_string(), 1)
}

/// Erreur d'intégration Miyunotify.
#[derive(Debug)]
pub struct MiyunotifyAdapterError {
    /// Message d'erreur (réseau, payload, service).
    pub message: String,
}

impl fmt::Display for MiyunotifyAdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MiyunotifyAdapterError {}

impl From<MiyunotifyError> for MiyunotifyAdapterError {
    fn from(e: MiyunotifyError) -> Self {
        MiyunotifyAdapterError {
            message: format!("{e:?}"),
        }
    }
}

/// Envoie une annonce (broadcast) aux destinataires d'une édition ou d'un canal.
///
/// @id: miyunotify_send_announcement
/// @do: envoie une annonce (broadcast) aux destinataires d'une édition ou d'un canal
/// @layer: domain
pub fn miyunotify_send_announcement(
    to_emails: &[String],
    subject: &str,
    body: &str,
) -> Result<(), MiyunotifyAdapterError> {
    let ctx = jayfestival_notify_ctx();
    for to in to_emails {
        email::send(&ctx, to, subject, body, None)?;
    }
    Ok(())
}

/// Envoie une notification ciblée (email, push ou inbox) à un ou plusieurs destinataires.
///
/// @id: miyunotify_send_targeted
/// @do: envoie une notification ciblée (email, push ou inbox) à un ou plusieurs destinataires
/// @layer: domain
pub fn miyunotify_send_targeted(
    channel: MiyunotifyTargetChannel,
    recipient: &str,
    subject_or_channel: &str,
    body_or_payload: &str,
) -> Result<(), MiyunotifyAdapterError> {
    let ctx = jayfestival_notify_ctx();
    match channel {
        MiyunotifyTargetChannel::Email => {
            email::send(&ctx, recipient, subject_or_channel, body_or_payload, None)?;
        }
        MiyunotifyTargetChannel::Push => {
            push::send(&ctx, subject_or_channel, body_or_payload)?;
        }
        MiyunotifyTargetChannel::Inbox => {
            let _ = inbox::write(&ctx, recipient, body_or_payload, None)?;
        }
    }
    Ok(())
}

/// Canal de notification ciblée (email, push, inbox).
#[derive(Debug, Clone, Copy)]
pub enum MiyunotifyTargetChannel {
    /// Notification par email.
    Email,
    /// Notification push (device/channel).
    Push,
    /// Écriture dans la boîte de réception (inbox).
    Inbox,
}
