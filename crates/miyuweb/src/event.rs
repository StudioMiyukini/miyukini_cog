//! Tool MiyuWeb — tool.web.event.dispatch.
//! Dispatch d'un événement dans le flux gouverné.

use crate::context::GovernedContext;
use crate::errors::MiyuwebError;

/// @id: miyuweb_tool_web_event_dispatch
/// @role: mutator
/// @layer: tool
/// @human: Dispatche un événement dans le flux gouverné (ex. clic, soumission).
/// @do: web_event_dispatch_under_governance
/// tool.web.event.dispatch — ne décide pas du traitement.
pub fn dispatch(
    ctx: &GovernedContext,
    _event_type: &str,
    _payload: &str,
) -> Result<(), MiyuwebError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebError::NoMandate);
    }
    Ok(())
}
