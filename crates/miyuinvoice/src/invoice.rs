//! Tools MiyuInvoice — tool.invoice.create, tool.invoice.send.
//! Facture : création (WriteIntent KindMother) ; envoi par canal fourni. Store en mémoire.

use crate::context::GovernedContext;
use crate::errors::MiyuinvoiceError;
use crate::store;
use miyukini_kernel::{IdGenerator as _, UuidIdGenerator};

/// @id: miyuinvoice_tool_invoice_create
/// @role: mutator
/// @layer: tool
/// @human: Crée une facture (métier indépendant) à partir de données fournies ; WriteIntent KindMother.
/// @do: invoice_create_under_governance
pub fn create(ctx: &GovernedContext, payload: &str) -> Result<String, MiyuinvoiceError> {
    if !ctx.has_mandate() {
        return Err(MiyuinvoiceError::NoMandate);
    }
    let id = format!("inv:{}", UuidIdGenerator.generate());
    let mut guard = store::invoices().lock().map_err(|_| MiyuinvoiceError::InvalidInput("lock".into()))?;
    guard.insert(id.clone(), payload.to_string());
    Ok(id)
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
    Ok(())
}
