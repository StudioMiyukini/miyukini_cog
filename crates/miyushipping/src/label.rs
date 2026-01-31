//! Tools MiyuShipping — tool.commerce.shipping.label.create, tool.commerce.shipping.label.print.
//! Étiquettes : création (décision StrongFather ; WriteIntent si état géré) ; données d'impression.

use crate::context::GovernedContext;
use crate::errors::MiyushippingError;

/// @id: miyushipping_tool_commerce_shipping_label_create
/// @role: mutator
/// @layer: tool
/// @human: Crée une étiquette d'expédition ; commande/colis fourni ; décision StrongFather ; WriteIntent si état géré.
/// @do: commerce_shipping_label_create_under_governance
pub fn create(ctx: &GovernedContext, _order_or_parcel_ref: &str, _payload: Option<&str>) -> Result<String, MiyushippingError> {
    if !ctx.has_mandate() {
        return Err(MiyushippingError::NoMandate);
    }
    Err(MiyushippingError::Unimplemented)
}

/// @id: miyushipping_tool_commerce_shipping_label_print
/// @role: accessor
/// @layer: tool
/// @human: Produit les données d'impression d'une étiquette ; exécution seule.
/// @do: commerce_shipping_label_print_under_governance
pub fn print(ctx: &GovernedContext, _label_id: &str) -> Result<Vec<u8>, MiyushippingError> {
    if !ctx.has_mandate() {
        return Err(MiyushippingError::NoMandate);
    }
    Err(MiyushippingError::Unimplemented)
}
