//! Tool MiyuWidgets — tool.web.template.resolve.
//! Résout un template par identifiant ; données fournies dans le flux ; pas de lecture base directe.

use crate::context::GovernedContext;
use crate::errors::MiyuwidgetsError;

/// @id: miyuwidgets_tool_web_template_resolve
/// @role: accessor
/// @layer: tool
/// @human: Résout un template par identifiant ; données fournies dans le flux.
/// @do: web_template_resolve_under_governance
pub fn resolve(ctx: &GovernedContext, _template_id: &str) -> Result<String, MiyuwidgetsError> {
    if !ctx.has_mandate() {
        return Err(MiyuwidgetsError::NoMandate);
    }
    Err(MiyuwidgetsError::Unimplemented)
}
