//! Tools MiyuPosKitchen — tool.pos.kitchen.print, order.push, order.update_status.

use crate::context::GovernedContext;
use crate::errors::MiyuposkitchenError;

/// @id: miyuposkitchen_tool_kitchen_print
/// @role: mutator
/// @layer: tool
/// @human: Envoie la commande à l'imprimante cuisine (données fournies).
/// @do: kitchen_print_under_governance
/// tool.pos.kitchen.print
pub fn print(
    ctx: &GovernedContext,
    _order_id: &str,
    _payload: &[u8],
) -> Result<(), MiyuposkitchenError> {
    if !ctx.has_mandate() {
        return Err(MiyuposkitchenError::NoMandate);
    }
    Ok(())
}

/// @id: miyuposkitchen_tool_order_push
/// @role: mutator
/// @layer: tool
/// @human: Envoie un ordre à l'affichage cuisine.
/// @do: order_push_under_governance
/// tool.pos.kitchen.order.push
pub fn order_push(ctx: &GovernedContext, _order_id: &str) -> Result<(), MiyuposkitchenError> {
    if !ctx.has_mandate() {
        return Err(MiyuposkitchenError::NoMandate);
    }
    Ok(())
}

/// @id: miyuposkitchen_tool_order_update_status
/// @role: mutator
/// @layer: tool
/// @human: Met à jour le statut d'un ordre cuisine (en cours, prêt).
/// @do: order_update_status_under_governance
/// tool.pos.kitchen.order.update_status
pub fn order_update_status(
    ctx: &GovernedContext,
    _order_id: &str,
    _status: &str,
) -> Result<(), MiyuposkitchenError> {
    if !ctx.has_mandate() {
        return Err(MiyuposkitchenError::NoMandate);
    }
    Ok(())
}
