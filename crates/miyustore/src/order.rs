//! Tools MiyuStore — tool.commerce.order.* (create, update, status, list).
//! Commandes : create/update = WriteIntent KindMother ; status/list = lecture.

use crate::context::GovernedContext;
use crate::errors::MiyustoreError;

/// @id: miyustore_tool_commerce_order_create
/// @role: mutator
/// @layer: tool
/// @human: Crée une commande ; WriteIntent KindMother (souvent via checkout.submit).
/// @do: commerce_order_create_under_governance
pub fn create(ctx: &GovernedContext, _payload: &str) -> Result<String, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    Err(MiyustoreError::Unimplemented)
}

/// @id: miyustore_tool_commerce_order_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour une commande (statut, champs) ; WriteIntent KindMother.
/// @do: commerce_order_update_under_governance
pub fn update(ctx: &GovernedContext, _order_id: &str, _payload: &str) -> Result<(), MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    Err(MiyustoreError::Unimplemented)
}

/// @id: miyustore_tool_commerce_order_status
/// @role: accessor
/// @layer: tool
/// @human: Retourne le statut d'une commande ; lecture.
/// @do: commerce_order_status_under_governance
pub fn status(ctx: &GovernedContext, _order_id: &str) -> Result<String, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    Err(MiyustoreError::Unimplemented)
}

/// @id: miyustore_tool_commerce_order_list
/// @role: accessor
/// @layer: tool
/// @human: Liste les commandes ; filtres fournis.
/// @do: commerce_order_list_under_governance
pub fn list(ctx: &GovernedContext, _filters: Option<&str>) -> Result<Vec<String>, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    Err(MiyustoreError::Unimplemented)
}
