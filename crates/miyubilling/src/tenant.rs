//! Tool MiyuBilling — tool.billing.tenant.resolve.
//! Résolution du contexte tenant ; isolation multi-tenant ; pas d'écriture.

use crate::context::GovernedContext;
use crate::errors::MiyubillingError;

/// @id: miyubilling_tool_billing_tenant_resolve
/// @role: accessor
/// @layer: tool
/// @human: Résout le contexte tenant ; isolation multi-tenant ; pas d'écriture.
/// @do: billing_tenant_resolve_under_governance
pub fn resolve(ctx: &GovernedContext, _context_ref: &str) -> Result<String, MiyubillingError> {
    if !ctx.has_mandate() {
        return Err(MiyubillingError::NoMandate);
    }
    Err(MiyubillingError::Unimplemented)
}
