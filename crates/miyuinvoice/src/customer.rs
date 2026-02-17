//! Tools MiyuInvoice — tool.invoice.customer.resolve, tool.invoice.customer.list.
//! Client facturation : résolution par identifiant ; liste (filtres fournis). Store en mémoire.

use crate::context::GovernedContext;
use crate::errors::MiyuinvoiceError;
use crate::store;

/// Enregistre un client (pour alimentation par flux / MiyukiniSales). Pas de décision métier.
pub fn register(ctx: &GovernedContext, customer_id: &str, metadata: &str) -> Result<(), MiyuinvoiceError> {
    if !ctx.has_mandate() {
        return Err(MiyuinvoiceError::NoMandate);
    }
    let mut guard = store::customers().lock().map_err(|_| MiyuinvoiceError::InvalidInput("lock".into()))?;
    guard.insert(customer_id.to_string(), metadata.to_string());
    Ok(())
}

/// @id: miyuinvoice_tool_invoice_customer_resolve
/// @role: accessor
/// @layer: tool
/// @human: Résout un client (facturation) par identifiant ; lecture.
/// @do: invoice_customer_resolve_under_governance
pub fn resolve(ctx: &GovernedContext, customer_id: &str) -> Result<String, MiyuinvoiceError> {
    if !ctx.has_mandate() {
        return Err(MiyuinvoiceError::NoMandate);
    }
    let guard = store::customers().lock().map_err(|_| MiyuinvoiceError::InvalidInput("lock".into()))?;
    let payload = guard
        .get(customer_id)
        .cloned()
        .ok_or_else(|| MiyuinvoiceError::InvalidInput("customer not found".into()))?;
    Ok(payload)
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
    let guard = store::customers().lock().map_err(|_| MiyuinvoiceError::InvalidInput("lock".into()))?;
    let ids: Vec<String> = guard.keys().cloned().collect();
    Ok(ids)
}
