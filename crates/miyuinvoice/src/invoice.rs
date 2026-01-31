//! Tools MiyuInvoice — tool.invoice.create, tool.invoice.send.
//! Facture : création (WriteIntent KindMother) ; envoi par canal fourni.

use crate::context::GovernedContext;
use crate::errors::MiyuinvoiceError;

/// @id: miyuinvoice_tool_invoice_create
/// @role: mutator
/// @layer: tool
/// @human: Crée une facture (métier indépendant) à partir de données fournies ; WriteIntent KindMother.
/// @do: invoice_create_under_governance
pub fn create(ctx: &GovernedContext, _payload: &str) -> Result<String, MiyuinvoiceError> {
    if !ctx.has_mandate() {
        return Err(MiyuinvoiceError::NoMandate);
    }
    Err(MiyuinvoiceError::Unimplemented)
}

/// @id: miyuinvoice_tool_invoice_send
/// @role: mutator
/// @layer: tool
/// @human: Envoie une facture par canal fourni (email, etc.).
/// @do: invoice_send_under_governance
pub fn send(ctx: &GovernedContext, _invoice_id: &str, _channel: &str, _payload: Option<&str>) -> Result<(), MiyuinvoiceError> {
    if !ctx.has_mandate() {
        return Err(MiyuinvoiceError::NoMandate);
    }
    Err(MiyuinvoiceError::Unimplemented)
}
