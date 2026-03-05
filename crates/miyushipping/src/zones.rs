//! Tool MiyuShipping — tool.commerce.shipping.zones.resolve.
//! Résolution des zones de livraison applicables ; lecture gouvernée.

use crate::context::GovernedContext;
use crate::errors::MiyushippingError;

/// @id: miyushipping_tool_commerce_shipping_zones_resolve
/// @role: accessor
/// @layer: tool
/// @human: Résout les zones de livraison applicables ; lecture gouvernée.
/// @do: commerce_shipping_zones_resolve_under_governance
pub fn resolve(
    ctx: &GovernedContext,
    _context_ref: Option<&str>,
) -> Result<Vec<String>, MiyushippingError> {
    if !ctx.has_mandate() {
        return Err(MiyushippingError::NoMandate);
    }
    Ok(vec!["default".to_string()])
}
