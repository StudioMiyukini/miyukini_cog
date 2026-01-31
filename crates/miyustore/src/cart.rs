//! Tools MiyuStore — tool.commerce.cart.* (add, update, remove, compute).
//! Panier : WriteIntent KindMother ; données fournies dans le flux.

use crate::context::GovernedContext;
use crate::errors::MiyustoreError;

/// @id: miyustore_tool_commerce_cart_add
/// @role: mutator
/// @layer: tool
/// @human: Ajoute une ligne au panier ; WriteIntent KindMother.
/// @do: commerce_cart_add_under_governance
pub fn add(ctx: &GovernedContext, _cart_ref: &str, _payload: &str) -> Result<String, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    Err(MiyustoreError::Unimplemented)
}

/// @id: miyustore_tool_commerce_cart_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour une ligne du panier ; WriteIntent KindMother.
/// @do: commerce_cart_update_under_governance
pub fn update(ctx: &GovernedContext, _line_id: &str, _payload: &str) -> Result<(), MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    Err(MiyustoreError::Unimplemented)
}

/// @id: miyustore_tool_commerce_cart_remove
/// @role: mutator
/// @layer: tool
/// @human: Supprime une ligne du panier ; WriteIntent KindMother.
/// @do: commerce_cart_remove_under_governance
pub fn remove(ctx: &GovernedContext, _line_id: &str) -> Result<(), MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    Err(MiyustoreError::Unimplemented)
}

/// @id: miyustore_tool_commerce_cart_compute
/// @role: accessor
/// @layer: tool
/// @human: Calcule totaux, taxes, livraison du panier ; règles fournies dans le flux.
/// @do: commerce_cart_compute_under_governance
pub fn compute(ctx: &GovernedContext, _cart_ref: &str, _rules: Option<&str>) -> Result<String, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    Err(MiyustoreError::Unimplemented)
}
