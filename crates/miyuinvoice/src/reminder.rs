//! Tool MiyuInvoice — tool.invoice.reminder.send.
//! Relance facture ; règles = StrongFather.

use crate::context::GovernedContext;
use crate::errors::MiyuinvoiceError;

/// @id: miyuinvoice_tool_invoice_reminder_send
/// @role: mutator
/// @layer: tool
/// @human: Envoie une relance ; règles = StrongFather.
/// @do: invoice_reminder_send_under_governance
pub fn send(ctx: &GovernedContext, _invoice_id: &str, _payload: Option<&str>) -> Result<(), MiyuinvoiceError> {
    if !ctx.has_mandate() {
        return Err(MiyuinvoiceError::NoMandate);
    }
    Ok(())
}
