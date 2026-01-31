//! Tool MiyuWeb — tool.web.theme.resolve.
//! Résolution thème (couleurs, styles) pour un contexte donné.

use crate::context::GovernedContext;
use crate::errors::MiyuwebError;

/// @id: miyuweb_tool_web_theme_resolve
/// @role: accessor
/// @layer: tool
/// @human: Résout le thème applicable (couleurs, styles) pour un contexte donné.
/// @do: web_theme_resolve_under_governance
/// tool.web.theme.resolve — ne décide pas de la politique de thème.
pub fn resolve(ctx: &GovernedContext, _context_ref: &str) -> Result<String, MiyuwebError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebError::NoMandate);
    }
    Err(MiyuwebError::Unimplemented)
}
