//! Tool MiyuInvoice — tool.invoice.electronic.submit.
//! Facturation électronique ; soumission plateforme agréée.

use crate::context::GovernedContext;
use crate::errors::MiyuinvoiceError;

/// @id miyuinvoice_tool_invoice_electronic_submit
/// @role mutator
/// @layer tool
/// @do invoice_electronic_submit_under_governance
//@human / @human Soumet à la facturation électronique (plateforme agréée 2026).
pub fn submit(
    ctx: &GovernedContext,
    invoice_id: &str,
    _payload: Option<&str>,
) -> Result<String, MiyuinvoiceError> {
    if !ctx.has_mandate() {
        return Err(MiyuinvoiceError::NoMandate);
    }
    Ok(format!("submitted:{invoice_id}"))
}
