//! Tool MiyuPosAnalytics — tool.analytics.tax.report.

use crate::context::GovernedContext;
use crate::errors::MiyuposanalyticsError;

/// @id: miyuposanalytics_tool_tax_report
/// @role: mutator
/// @layer: tool
/// @human: Retourne rapport taxes (période, filtres).
/// @do: tax_report_under_governance
/// tool.analytics.tax.report
pub fn report(
    ctx: &GovernedContext,
    _period_start: &str,
    _period_end: &str,
) -> Result<Vec<u8>, MiyuposanalyticsError> {
    if !ctx.has_mandate() {
        return Err(MiyuposanalyticsError::NoMandate);
    }
    Ok(Vec::new())
}
