//! Tool MiyuShipping — tool.commerce.shipping.tracking.get.
//! Statut de suivi d'un envoi ; identifiant fourni.

use crate::context::GovernedContext;
use crate::errors::MiyushippingError;

/// @id: miyushipping_tool_commerce_shipping_tracking_get
/// @role: accessor
/// @layer: tool
/// @human: Retourne le statut de suivi d'un envoi ; identifiant fourni.
/// @do: commerce_shipping_tracking_get_under_governance
pub fn get(ctx: &GovernedContext, _tracking_id: &str) -> Result<String, MiyushippingError> {
    if !ctx.has_mandate() {
        return Err(MiyushippingError::NoMandate);
    }
    Ok("{}".to_string())
}
