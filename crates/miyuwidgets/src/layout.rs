//! Tool MiyuWidgets — tool.web.layout.apply.
//! Applique une modification de layout ; structure à partir de données fournies ; exécution seule.

use crate::context::GovernedContext;
use crate::errors::MiyuwidgetsError;
use crate::store;
use miyukini_kernel::{IdGenerator as _, UuidIdGenerator};

/// @id: miyuwidgets_tool_web_layout_apply
/// @role: mutator
/// @layer: tool
/// @human: Applique une modification de layout ; structure à partir de données fournies.
/// @do: web_layout_apply_under_governance
pub fn apply(ctx: &GovernedContext, payload: &str) -> Result<String, MiyuwidgetsError> {
    if !ctx.has_mandate() {
        return Err(MiyuwidgetsError::NoMandate);
    }
    let id = format!("layout:{}", UuidIdGenerator.generate());
    let mut guard = store::layouts().lock().map_err(|_| MiyuwidgetsError::InvalidInput("lock".into()))?;
    guard.insert(id.clone(), payload.to_string());
    Ok(id)
}
