//! Tool MiyuPosAnalytics — tool.analytics.cash.discrepancy.

use crate::context::GovernedContext;
use crate::errors::MiyuposanalyticsError;

/// @id: miyuposanalytics_tool_cash_discrepancy
/// @role: mutator
/// @layer: tool
/// @human: Retourne l'écart caisse pour un shift.
/// @do: cash_discrepancy_under_governance
/// tool.analytics.cash.discrepancy
pub fn discrepancy(ctx: &GovernedContext, _shift_id: &str) -> Result<CashDiscrepancyResult, MiyuposanalyticsError> {
    if !ctx.has_mandate() {
        return Err(MiyuposanalyticsError::NoMandate);
    }
    Ok(CashDiscrepancyResult::default())
}

/// Résultat écart caisse.
#[derive(Debug, Clone, Default)]
pub struct CashDiscrepancyResult {
    pub expected: f64,
    pub actual: f64,
    pub discrepancy: f64,
}
