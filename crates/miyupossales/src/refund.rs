//! Tool MiyuPosSales — tool.pos.refund.record.

use crate::context::GovernedContext;
use crate::errors::MiyupossalesError;

/// @id: miyupossales_tool_refund_record
/// @role: mutator
/// @layer: tool
/// @human: Enregistre un remboursement (item ou reçu) ; autorisation = StrongFather ; WriteIntent KindMother.
/// @do: refund_record_under_governance
/// tool.pos.refund.record
pub fn record(
    ctx: &GovernedContext,
    _sale_id: &str,
    _refund_type: &str,
    _line_ids: &[String],
) -> Result<String, MiyupossalesError> {
    if !ctx.has_mandate() {
        return Err(MiyupossalesError::NoMandate);
    }
    Ok("refund:stub".to_string())
}
