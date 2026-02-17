//! Tools MiyuShipping — tool.commerce.shipping.shipment.create, tool.commerce.shipping.shipment.list.
//! Expéditions : création (WriteIntent si état commande géré ; décision StrongFather) ; liste.

use crate::context::GovernedContext;
use crate::errors::MiyushippingError;

/// @id: miyushipping_tool_commerce_shipping_shipment_create
/// @role: mutator
/// @layer: tool
/// @human: Crée une expédition (colis) ; WriteIntent si état commande géré ; décision StrongFather.
/// @do: commerce_shipping_shipment_create_under_governance
pub fn create(ctx: &GovernedContext, order_id: &str, _payload: &str) -> Result<String, MiyushippingError> {
    if !ctx.has_mandate() {
        return Err(MiyushippingError::NoMandate);
    }
    Ok(format!("ship:{}", order_id))
}

/// @id: miyushipping_tool_commerce_shipping_shipment_list
/// @role: accessor
/// @layer: tool
/// @human: Liste les expéditions d'une commande ; lecture gouvernée.
/// @do: commerce_shipping_shipment_list_under_governance
pub fn list(ctx: &GovernedContext, _order_id: &str) -> Result<Vec<String>, MiyushippingError> {
    if !ctx.has_mandate() {
        return Err(MiyushippingError::NoMandate);
    }
    Ok(Vec::new())
}
