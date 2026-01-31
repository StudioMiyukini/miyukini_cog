//! Tools MiyuInvoice — tool.invoice.customer.resolve, tool.invoice.customer.list.
//! Client facturation : résolution par identifiant ; liste (filtres fournis).

use crate::context::GovernedContext;
use crate::errors::MiyuinvoiceError;

/// @id: miyuinvoice_tool_invoice_customer_resolve
/// @role: accessor
/// @layer: tool
/// @human: Résout un client (facturation) par identifiant ; lecture.
/// @do: invoice_customer_resolve_under_governance
pub fn resolve(ctx: &GovernedContext, _customer_id: &str) -> Result<String, MiyuinvoiceError> {
    if !ctx.has_mandate() {
        return Err(MiyuinvoiceError::NoMandate);
    }
    Err(MiyuinvoiceError::Unimplemented)
}

/// @id: miyuinvoice_tool_invoice_customer_list
/// @role: accessor
/// @layer: tool
/// @human: Liste les clients (filtres fournis) pour facturation ; lecture.
/// @do: invoice_customer_list_under_governance
pub fn list(ctx: &GovernedContext, _filters: Option<&str>) -> Result<Vec<String>, MiyuinvoiceError> {
    if !ctx.has_mandate() {
        return Err(MiyuinvoiceError::NoMandate);
    }
    Err(MiyuinvoiceError::Unimplemented)
}
