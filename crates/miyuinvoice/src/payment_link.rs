//! Tool MiyuInvoice — tool.invoice.payment.link.generate.
//! Génération lien de paiement pour une facture.

use crate::context::GovernedContext;
use crate::errors::MiyuinvoiceError;

/// @id: miyuinvoice_tool_invoice_payment_link_generate
/// @role: mutator
/// @layer: tool
/// @human: Génère un lien de paiement pour une facture.
/// @do: invoice_payment_link_generate_under_governance
pub fn generate(ctx: &GovernedContext, invoice_id: &str) -> Result<String, MiyuinvoiceError> {
    if !ctx.has_mandate() {
        return Err(MiyuinvoiceError::NoMandate);
    }
    Ok(format!("https://pay.example/{invoice_id}"))
}
