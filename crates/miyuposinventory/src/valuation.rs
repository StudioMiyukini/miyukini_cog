//! Tool MiyuPosInventory — tool.inventory.valuation.report.

use crate::context::GovernedContext;
use crate::errors::MiyuposinventoryError;

/// @id: miyuposinventory_tool_valuation_report
/// @role: mutator
/// @layer: tool
/// @human: Retourne un rapport de valorisation (coût / marge potentielle) ; lecture seule.
/// @do: valuation_report_under_governance
/// tool.inventory.valuation.report
pub fn report(
    ctx: &GovernedContext,
    _site_id: &str,
    _as_of: &str,
) -> Result<Vec<u8>, MiyuposinventoryError> {
    if !ctx.has_mandate() {
        return Err(MiyuposinventoryError::NoMandate);
    }
    Ok(Vec::new())
}
