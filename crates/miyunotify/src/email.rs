//! Tool MiyuNotify — tool.notify.email.send.
//! Envoi email (destinataire, sujet, corps, pièces jointes fournis). Autorisation = StrongFather.

use crate::context::GovernedContext;
use crate::errors::MiyunotifyError;

/// @id: miyunotify_tool_email_send
/// @role: mutator
/// @layer: tool
/// @human: Envoie un email ; autorisation fournie par le flux (StrongFather).
/// @do: email_send_under_governance
/// tool.notify.email.send — ne décide pas de l'autorisation.
pub fn send(
    ctx: &GovernedContext,
    _to: &str,
    _subject: &str,
    _body: &str,
    _attachments: Option<&[String]>,
) -> Result<(), MiyunotifyError> {
    if !ctx.has_mandate() {
        return Err(MiyunotifyError::NoMandate);
    }
    Err(MiyunotifyError::Unimplemented)
}
