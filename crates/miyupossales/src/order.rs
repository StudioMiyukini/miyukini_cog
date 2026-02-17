//! Tool MiyuPosSales — tool.pos.order.service_type.set.

use crate::context::GovernedContext;
use crate::errors::MiyupossalesError;

/// @id: miyupossales_tool_order_service_type_set
/// @role: mutator
/// @layer: tool
/// @human: Définit le type de service (sur place / à emporter / livraison).
/// @do: order_service_type_set_under_governance
/// tool.pos.order.service_type.set
pub fn service_type_set(
    ctx: &GovernedContext,
    _ticket_id: &str,
    _service_type: &str,
) -> Result<(), MiyupossalesError> {
    if !ctx.has_mandate() {
        return Err(MiyupossalesError::NoMandate);
    }
    Ok(())
}
