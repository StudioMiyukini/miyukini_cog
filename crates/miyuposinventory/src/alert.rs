//! Tool MiyuPosInventory — tool.inventory.alert.low.evaluate.

use crate::context::GovernedContext;
use crate::errors::MiyuposinventoryError;

/// @id: miyuposinventory_tool_alert_low_evaluate
/// @role: mutator
/// @layer: tool
/// @human: Évalue les articles sous seuil bas (données seuils fournies).
/// @do: alert_low_evaluate_under_governance
/// tool.inventory.alert.low.evaluate
pub fn low_evaluate(
    ctx: &GovernedContext,
    _site_id: &str,
) -> Result<Vec<AlertItem>, MiyuposinventoryError> {
    if !ctx.has_mandate() {
        return Err(MiyuposinventoryError::NoMandate);
    }
    Ok(Vec::new())
}

/// Élément alerte.
#[derive(Debug, Clone)]
pub struct AlertItem {
    pub article_id: String,
    pub quantity: i64,
    pub threshold: i64,
}
