//! Tool MiyuBilling — tool.billing.tenant.resolve.
//! Résolution du contexte tenant ; isolation multi-tenant ; pas d'écriture.

use crate::context::GovernedContext;
use crate::errors::MiyubillingError;

/// @id: miyubilling_tool_billing_tenant_resolve
/// @role: accessor
/// @layer: tool
/// @human: Résout le contexte tenant ; isolation multi-tenant ; pas d'écriture.
/// @do: billing_tenant_resolve_under_governance
/// Stub : retourne le context_ref tel quel (tenant par défaut).
pub fn resolve(ctx: &GovernedContext, context_ref: &str) -> Result<String, MiyubillingError> {
    if !ctx.has_mandate() {
        return Err(MiyubillingError::NoMandate);
    }
    Ok(if context_ref.is_empty() {
        "default".to_string()
    } else {
        context_ref.to_string()
    })
}
