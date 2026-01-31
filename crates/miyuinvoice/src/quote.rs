//! Tools MiyuInvoice — tool.invoice.quote.* (create, update, to_invoice).
//! Devis : create/update = WriteIntent KindMother ; to_invoice = décision StrongFather.

use crate::context::GovernedContext;
use crate::errors::MiyuinvoiceError;

/// @id: miyuinvoice_tool_invoice_quote_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un devis à partir de données fournies ; persistance = KindMother.
/// @do: invoice_quote_create_under_governance
pub fn create(ctx: &GovernedContext, _payload: &str) -> Result<String, MiyuinvoiceError> {
    if !ctx.has_mandate() {
        return Err(MiyuinvoiceError::NoMandate);
    }
    Err(MiyuinvoiceError::Unimplemented)
}

/// @id: miyuinvoice_tool_invoice_quote_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour un devis existant ; WriteIntent KindMother.
/// @do: invoice_quote_update_under_governance
pub fn update(ctx: &GovernedContext, _quote_id: &str, _payload: &str) -> Result<(), MiyuinvoiceError> {
    if !ctx.has_mandate() {
        return Err(MiyuinvoiceError::NoMandate);
    }
    Err(MiyuinvoiceError::Unimplemented)
}

/// @id: miyuinvoice_tool_invoice_quote_to_invoice
/// @role: mutator
/// @layer: tool
/// @human: Convertit un devis en facture ; décision = StrongFather.
/// @do: invoice_quote_to_invoice_under_governance
pub fn to_invoice(ctx: &GovernedContext, _quote_id: &str) -> Result<String, MiyuinvoiceError> {
    if !ctx.has_mandate() {
        return Err(MiyuinvoiceError::NoMandate);
    }
    Err(MiyuinvoiceError::Unimplemented)
}
