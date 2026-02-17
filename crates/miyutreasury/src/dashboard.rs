//! Tool MiyuTreasury — tool.treasury.dashboard.aggregate.
//! Tableau de bord trésorerie ; agrège les indicateurs ; données = KindMother.

use crate::context::GovernedContext;
use crate::errors::MiyutreasuryError;

/// @id: miyutreasury_tool_treasury_dashboard_aggregate
/// @role: accessor
/// @layer: tool
/// @human: Agrège les indicateurs pour le tableau de bord trésorerie ; données = KindMother.
/// @do: treasury_dashboard_aggregate_under_governance
pub fn aggregate(ctx: &GovernedContext, _context_ref: Option<&str>) -> Result<String, MiyutreasuryError> {
    if !ctx.has_mandate() {
        return Err(MiyutreasuryError::NoMandate);
    }
    Ok("{}".to_string())
}
