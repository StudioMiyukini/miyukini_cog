//! Tools MiyuShipping — tool.commerce.shipping.rate, tool.commerce.shipping.rates.compare.
//! Tarifs livraison : calcul (panier/zone fourni) ; comparaison transporteurs.

use crate::context::GovernedContext;
use crate::errors::MiyushippingError;

/// @id: miyushipping_tool_commerce_shipping_rate
/// @role: accessor
/// @layer: tool
/// @human: Calcule le tarif de livraison ; panier/zone fourni ; règles KindMother ou flux.
/// @do: commerce_shipping_rate_under_governance
pub fn rate(ctx: &GovernedContext, _cart_or_parcel_ref: &str, _zone_ref: &str) -> Result<String, MiyushippingError> {
    if !ctx.has_mandate() {
        return Err(MiyushippingError::NoMandate);
    }
    Err(MiyushippingError::Unimplemented)
}

/// @id: miyushipping_tool_commerce_shipping_rates_compare
/// @role: accessor
/// @layer: tool
/// @human: Compare les tarifs de plusieurs transporteurs ; colis donné ; lecture.
/// @do: commerce_shipping_rates_compare_under_governance
pub fn rates_compare(ctx: &GovernedContext, _parcel_ref: &str) -> Result<Vec<String>, MiyushippingError> {
    if !ctx.has_mandate() {
        return Err(MiyushippingError::NoMandate);
    }
    Err(MiyushippingError::Unimplemented)
}
