//! Tools MiyuBilling — tool.billing.invoice.generate, tool.billing.invoice.list.
//! Factures : génération (WriteIntent KindMother) ; liste (lecture gouvernée).

use crate::context::GovernedContext;
use crate::errors::MiyubillingError;

/// @id: miyubilling_tool_billing_invoice_generate
/// @role: mutator
/// @layer: tool
/// @human: Génère une facture ; règles fournies ; WriteIntent KindMother.
/// @do: billing_invoice_generate_under_governance
pub fn generate(ctx: &GovernedContext, _payload: &str) -> Result<String, MiyubillingError> {
    if !ctx.has_mandate() {
        return Err(MiyubillingError::NoMandate);
    }
    Err(MiyubillingError::Unimplemented)
}

/// @id: miyubilling_tool_billing_invoice_list
/// @role: accessor
/// @layer: tool
/// @human: Liste les factures ; filtres fournis ; lecture gouvernée.
/// @do: billing_invoice_list_under_governance
pub fn list(ctx: &GovernedContext, _filters: Option<&str>) -> Result<Vec<String>, MiyubillingError> {
    if !ctx.has_mandate() {
        return Err(MiyubillingError::NoMandate);
    }
    Err(MiyubillingError::Unimplemented)
}
