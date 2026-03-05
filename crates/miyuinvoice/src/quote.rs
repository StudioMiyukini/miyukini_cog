//! Tools MiyuInvoice — tool.invoice.quote.* (create, update, to_invoice).
//! Devis : create/update = WriteIntent KindMother ; to_invoice = décision StrongFather. Store en mémoire.

use crate::context::GovernedContext;
use crate::errors::MiyuinvoiceError;
use crate::invoice::create as invoice_create;
use crate::store;
use miyukini_kernel::{IdGenerator as _, UuidIdGenerator};

/// @id: miyuinvoice_tool_invoice_quote_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un devis à partir de données fournies ; persistance = KindMother.
/// @do: invoice_quote_create_under_governance
pub fn create(ctx: &GovernedContext, payload: &str) -> Result<String, MiyuinvoiceError> {
    if !ctx.has_mandate() {
        return Err(MiyuinvoiceError::NoMandate);
    }
    let id = format!("quo:{}", UuidIdGenerator.generate());
    let mut guard = store::quotes()
        .lock()
        .map_err(|_| MiyuinvoiceError::InvalidInput("lock".into()))?;
    guard.insert(id.clone(), payload.to_string());
    Ok(id)
}

/// @id: miyuinvoice_tool_invoice_quote_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour un devis existant ; WriteIntent KindMother.
/// @do: invoice_quote_update_under_governance
pub fn update(
    ctx: &GovernedContext,
    quote_id: &str,
    payload: &str,
) -> Result<(), MiyuinvoiceError> {
    if !ctx.has_mandate() {
        return Err(MiyuinvoiceError::NoMandate);
    }
    let mut guard = store::quotes()
        .lock()
        .map_err(|_| MiyuinvoiceError::InvalidInput("lock".into()))?;
    guard.insert(quote_id.to_string(), payload.to_string());
    Ok(())
}

/// @id: miyuinvoice_tool_invoice_quote_to_invoice
/// @role: mutator
/// @layer: tool
/// @human: Convertit un devis en facture ; décision = StrongFather.
/// @do: invoice_quote_to_invoice_under_governance
pub fn to_invoice(ctx: &GovernedContext, quote_id: &str) -> Result<String, MiyuinvoiceError> {
    if !ctx.has_mandate() {
        return Err(MiyuinvoiceError::NoMandate);
    }
    let payload = {
        let guard = store::quotes()
            .lock()
            .map_err(|_| MiyuinvoiceError::InvalidInput("lock".into()))?;
        guard
            .get(quote_id)
            .cloned()
            .ok_or_else(|| MiyuinvoiceError::InvalidInput("quote not found".into()))?
    };
    invoice_create(ctx, &payload)
}
