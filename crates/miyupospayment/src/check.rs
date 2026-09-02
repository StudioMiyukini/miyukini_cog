//! Tool MiyuPosPayment — tool.pos.payment.check.record.

use crate::context::GovernedContext;
use crate::errors::MiyupospaymentError;

/// @id miyupospayment_tool_check_record
/// @role mutator
/// @layer tool
/// @do check_record_under_governance
//@human / @human Enregistre un paiement chèque ; KindMother.
/// tool.pos.payment.check.record
pub fn record(
    ctx: &GovernedContext,
    _session_id: &str,
    _amount: f64,
    _check_ref: &str,
) -> Result<String, MiyupospaymentError> {
    if !ctx.has_mandate() {
        return Err(MiyupospaymentError::NoMandate);
    }
    Ok("check:stub".to_string())
}
