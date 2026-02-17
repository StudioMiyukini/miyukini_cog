//! Tool MiyuPosSales — tool.pos.display.push.

use crate::context::GovernedContext;
use crate::errors::MiyupossalesError;

/// @id: miyupossales_tool_display_push
/// @role: mutator
/// @layer: tool
/// @human: Envoie les données à afficher sur l'écran client.
/// @do: display_push_under_governance
/// tool.pos.display.push
pub fn push(
    ctx: &GovernedContext,
    _sale_id: &str,
    _payload: &str,
) -> Result<(), MiyupossalesError> {
    if !ctx.has_mandate() {
        return Err(MiyupossalesError::NoMandate);
    }
    Ok(())
}
