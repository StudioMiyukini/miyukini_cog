//! Tool MiyuPosPayment — tool.pos.payment.split.

use crate::context::GovernedContext;
use crate::errors::MiyupospaymentError;
use std::collections::HashMap;

/// @id: miyupospayment_tool_split
/// @role: mutator
/// @layer: tool
/// @human: Répartit le paiement entre plusieurs moyens ; autorisation = StrongFather.
/// @do: payment_split_under_governance
/// tool.pos.payment.split
pub fn split(
    ctx: &GovernedContext,
    _session_id: &str,
    _total: f64,
    _amounts_by_method: &HashMap<String, f64>,
) -> Result<Vec<String>, MiyupospaymentError> {
    if !ctx.has_mandate() {
        return Err(MiyupospaymentError::NoMandate);
    }
    Ok(Vec::new())
}
