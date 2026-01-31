//! Tool MiyuWeb — tool.web.input.capture.
//! Capture d'une entrée utilisateur (clic, saisie) pour le flux gouverné.

use crate::context::GovernedContext;
use crate::errors::MiyuwebError;

/// @id: miyuweb_tool_web_input_capture
/// @role: mutator
/// @layer: tool
/// @human: Capture une entrée utilisateur (clic, saisie, touche) et la transmet au flux gouverné.
/// @do: web_input_capture_under_governance
/// tool.web.input.capture — ne décide pas de l'usage.
pub fn capture(
    ctx: &GovernedContext,
    _input_type: &str,
    _value: &str,
) -> Result<String, MiyuwebError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebError::NoMandate);
    }
    Ok(String::new())
}
