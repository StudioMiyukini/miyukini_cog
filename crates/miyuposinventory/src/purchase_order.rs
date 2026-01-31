//! Tools MiyuPosInventory — tool.inventory.purchase_order.create, update, track.

use crate::context::GovernedContext;
use crate::errors::MiyuposinventoryError;
use std::collections::HashMap;

/// @id: miyuposinventory_tool_purchase_order_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un bon de commande fournisseur à partir de données fournies ; WriteIntent KindMother.
/// @do: purchase_order_create_under_governance
/// tool.inventory.purchase_order.create
pub fn create(
    ctx: &GovernedContext,
    _supplier_id: &str,
    _lines: &[(String, u32)],
) -> Result<String, MiyuposinventoryError> {
    if !ctx.has_mandate() {
        return Err(MiyuposinventoryError::NoMandate);
    }
    Err(MiyuposinventoryError::Unimplemented)
}

/// @id: miyuposinventory_tool_purchase_order_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour un bon de commande (réception partielle, etc.) ; WriteIntent KindMother.
/// @do: purchase_order_update_under_governance
/// tool.inventory.purchase_order.update
pub fn update(
    ctx: &GovernedContext,
    _po_id: &str,
    _updates: &HashMap<String, String>,
) -> Result<(), MiyuposinventoryError> {
    if !ctx.has_mandate() {
        return Err(MiyuposinventoryError::NoMandate);
    }
    Err(MiyuposinventoryError::Unimplemented)
}

/// @id: miyuposinventory_tool_purchase_order_track
/// @role: mutator
/// @layer: tool
/// @human: Retourne le statut / suivi d'un bon de commande.
/// @do: purchase_order_track_under_governance
/// tool.inventory.purchase_order.track
pub fn track(
    ctx: &GovernedContext,
    _po_id: &str,
) -> Result<PurchaseOrderStatus, MiyuposinventoryError> {
    if !ctx.has_mandate() {
        return Err(MiyuposinventoryError::NoMandate);
    }
    Err(MiyuposinventoryError::Unimplemented)
}

/// Statut bon de commande.
#[derive(Debug, Clone, Default)]
pub struct PurchaseOrderStatus {
    pub status: String,
    pub received_count: u32,
}
