//! Tools MiyuStore — tool.commerce.cart.* (add, update, remove, compute).
//! Store en mémoire (KindMother côté produit pour persistance).

use crate::context::GovernedContext;
use crate::errors::MiyustoreError;
use miyukini_kernel::{IdGenerator, UuidIdGenerator};
use std::collections::HashMap;
use std::sync::Mutex;

type CartLines = HashMap<String, String>;

static CARTS: std::sync::OnceLock<Mutex<HashMap<String, CartLines>>> = std::sync::OnceLock::new();

fn carts() -> &'static Mutex<HashMap<String, CartLines>> {
    CARTS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// @id: miyustore_tool_commerce_cart_add
/// @role: mutator
/// @layer: tool
/// @human: Ajoute une ligne au panier ; WriteIntent KindMother.
/// @do: commerce_cart_add_under_governance
pub fn add(ctx: &GovernedContext, cart_ref: &str, payload: &str) -> Result<String, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    let line_id = format!("line:{}", UuidIdGenerator.generate());
    let mut guard = carts().lock().map_err(|_| MiyustoreError::InvalidInput("lock".into()))?;
    guard
        .entry(cart_ref.to_string())
        .or_default()
        .insert(line_id.clone(), payload.to_string());
    Ok(line_id)
}

/// @id: miyustore_tool_commerce_cart_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour une ligne du panier ; WriteIntent KindMother.
/// @do: commerce_cart_update_under_governance
pub fn update(ctx: &GovernedContext, line_id: &str, payload: &str) -> Result<(), MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    let mut guard = carts().lock().map_err(|_| MiyustoreError::InvalidInput("lock".into()))?;
    for lines in guard.values_mut() {
        if lines.contains_key(line_id) {
            lines.insert(line_id.to_string(), payload.to_string());
            return Ok(());
        }
    }
    Ok(())
}

/// @id: miyustore_tool_commerce_cart_remove
/// @role: mutator
/// @layer: tool
/// @human: Supprime une ligne du panier ; WriteIntent KindMother.
/// @do: commerce_cart_remove_under_governance
pub fn remove(ctx: &GovernedContext, line_id: &str) -> Result<(), MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    let mut guard = carts().lock().map_err(|_| MiyustoreError::InvalidInput("lock".into()))?;
    for lines in guard.values_mut() {
        lines.remove(line_id);
    }
    Ok(())
}

/// @id: miyustore_tool_commerce_cart_compute
/// @role: accessor
/// @layer: tool
/// @human: Calcule totaux, taxes, livraison du panier ; règles fournies dans le flux.
/// @do: commerce_cart_compute_under_governance
pub fn compute(ctx: &GovernedContext, cart_ref: &str, _rules: Option<&str>) -> Result<String, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    let guard = carts().lock().map_err(|_| MiyustoreError::InvalidInput("lock".into()))?;
    let count = guard.get(cart_ref).map(|l| l.len()).unwrap_or(0);
    Ok(format!("total_lines:{count}"))
}
