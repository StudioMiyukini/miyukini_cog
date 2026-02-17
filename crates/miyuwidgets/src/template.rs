//! Tool MiyuWidgets — tool.web.template.resolve.
//! Résout un template par identifiant ; données fournies dans le flux ; pas de lecture base directe.

use crate::context::GovernedContext;
use crate::errors::MiyuwidgetsError;
use crate::store;

/// @id: miyuwidgets_tool_web_template_resolve
/// @role: accessor
/// @layer: tool
/// @human: Résout un template par identifiant ; données fournies dans le flux.
/// @do: web_template_resolve_under_governance
pub fn resolve(ctx: &GovernedContext, template_id: &str) -> Result<String, MiyuwidgetsError> {
    if !ctx.has_mandate() {
        return Err(MiyuwidgetsError::NoMandate);
    }
    let guard = store::templates().lock().map_err(|_| MiyuwidgetsError::InvalidInput("lock".into()))?;
    Ok(guard.get(template_id).cloned().unwrap_or_default())
}

/// Enregistre un template (WriteIntent KindMother). Pour tests et usage applicatif.
pub fn register(ctx: &GovernedContext, template_id: &str, content: &str) -> Result<(), MiyuwidgetsError> {
    if !ctx.has_mandate() {
        return Err(MiyuwidgetsError::NoMandate);
    }
    let mut guard = store::templates().lock().map_err(|_| MiyuwidgetsError::InvalidInput("lock".into()))?;
    guard.insert(template_id.to_string(), content.to_string());
    Ok(())
}
