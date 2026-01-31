//! Tool MiyuPosPayment — tool.pos.payment.cash.record.

use crate::context::GovernedContext;
use crate::errors::MiyupospaymentError;

/// @id: miyupospayment_tool_cash_record
/// @role: mutator
/// @layer: tool
/// @human: Enregistre un paiement espèces ; montant + session ; KindMother.
/// @do: cash_record_under_governance
/// tool.pos.payment.cash.record
pub fn record(
    ctx: &GovernedContext,
    _session_id: &str,
    _amount: f64,
) -> Result<String, MiyupospaymentError> {
    if !ctx.has_mandate() {
        return Err(MiyupospaymentError::NoMandate);
    }
    Err(MiyupospaymentError::Unimplemented)
}
