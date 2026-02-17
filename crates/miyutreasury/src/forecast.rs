//! Tool MiyuTreasury — tool.treasury.forecast.compute.
//! Prévisionnel trésorerie ; calcule à partir de données fournies.

use crate::context::GovernedContext;
use crate::errors::MiyutreasuryError;

/// @id: miyutreasury_tool_treasury_forecast_compute
/// @role: accessor
/// @layer: tool
/// @human: Calcule un prévisionnel à partir de données fournies ; exécution seule.
/// @do: treasury_forecast_compute_under_governance
pub fn compute(ctx: &GovernedContext, _payload: &str) -> Result<String, MiyutreasuryError> {
    if !ctx.has_mandate() {
        return Err(MiyutreasuryError::NoMandate);
    }
    Ok("{}".to_string())
}
