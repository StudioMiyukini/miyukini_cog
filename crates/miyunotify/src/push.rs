//! Tool MiyuNotify — tool.notify.push.send.
//! Envoi notification push (device/channel, payload fournis). Autorisation = StrongFather.

use crate::context::GovernedContext;
use crate::errors::MiyunotifyError;

/// @id: miyunotify_tool_push_send
/// @role: mutator
/// @layer: tool
/// @human: Envoie une notification push ; autorisation fournie par le flux.
/// @do: push_send_under_governance
/// tool.notify.push.send — ne décide pas de l'autorisation.
pub fn send(ctx: &GovernedContext, _channel: &str, _payload: &str) -> Result<(), MiyunotifyError> {
    if !ctx.has_mandate() {
        return Err(MiyunotifyError::NoMandate);
    }
    Err(MiyunotifyError::Unimplemented)
}
