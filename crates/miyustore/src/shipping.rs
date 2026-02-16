//! Tools MiyuStore — tool.commerce.shipping.rate, tool.commerce.shipping.zones.resolve.
//! Stub : tarif fixe ; zones par défaut.

use crate::context::GovernedContext;
use crate::errors::MiyustoreError;

/// @id: miyustore_tool_commerce_shipping_rate
/// @role: accessor
/// @layer: tool
/// @human: Calcule le tarif de livraison ; panier/zone fourni.
/// @do: commerce_shipping_rate_under_governance
pub fn rate(ctx: &GovernedContext, _cart_ref: &str, _zone_ref: &str) -> Result<String, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    Ok("0.00".to_string())
}

/// @id: miyustore_tool_commerce_shipping_zones_resolve
/// @role: accessor
/// @layer: tool
/// @human: Résout les zones de livraison applicables ; lecture gouvernée.
/// @do: commerce_shipping_zones_resolve_under_governance
pub fn zones_resolve(ctx: &GovernedContext, _context_ref: Option<&str>) -> Result<Vec<String>, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    Ok(vec!["default".to_string()])
}
