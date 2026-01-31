//! Tool MiyuPosKitchen — tool.pos.order.service_type.set.

use crate::context::GovernedContext;
use crate::errors::MiyuposkitchenError;

/// @id: miyuposkitchen_tool_service_type_set
/// @role: mutator
/// @layer: tool
/// @human: Définit le type de service (sur place / à emporter / livraison).
/// @do: service_type_set_under_governance
/// tool.pos.order.service_type.set
pub fn set(
    ctx: &GovernedContext,
    _ticket_id: &str,
    _service_type: &str,
) -> Result<(), MiyuposkitchenError> {
    if !ctx.has_mandate() {
        return Err(MiyuposkitchenError::NoMandate);
    }
    Err(MiyuposkitchenError::Unimplemented)
}
