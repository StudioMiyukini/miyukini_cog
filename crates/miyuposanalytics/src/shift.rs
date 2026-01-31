//! Tool MiyuPosAnalytics — tool.pos.shift.close.

use crate::context::GovernedContext;
use crate::errors::MiyuposanalyticsError;

/// @id: miyuposanalytics_tool_shift_close
/// @role: mutator
/// @layer: tool
/// @human: Clôture un shift caisse (comptage, écart) ; autorisation = StrongFather.
/// @do: shift_close_under_governance
/// tool.pos.shift.close
pub fn close(
    ctx: &GovernedContext,
    _shift_id: &str,
    _count: &std::collections::HashMap<String, f64>,
) -> Result<String, MiyuposanalyticsError> {
    if !ctx.has_mandate() {
        return Err(MiyuposanalyticsError::NoMandate);
    }
    Err(MiyuposanalyticsError::Unimplemented)
}
