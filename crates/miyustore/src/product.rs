//! Tools MiyuStore — tool.commerce.product.* (list, resolve, variations, create, update).
//! Store en mémoire (persistance réelle = KindMother côté produit).

use crate::context::GovernedContext;
use crate::errors::MiyustoreError;
use miyukini_kernel::{IdGenerator, UuidIdGenerator};
use std::collections::HashMap;
use std::sync::Mutex;

static PRODUCTS: std::sync::OnceLock<Mutex<HashMap<String, String>>> = std::sync::OnceLock::new();

fn products() -> &'static Mutex<HashMap<String, String>> {
    PRODUCTS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// @id: miyustore_tool_commerce_product_list
/// @role: accessor
/// @layer: tool
/// @human: Liste les produits ; filtres fournis.
/// @do: commerce_product_list_under_governance
pub fn list(ctx: &GovernedContext, _filters: Option<&str>) -> Result<Vec<String>, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    let guard = products().lock().map_err(|_| MiyustoreError::InvalidInput("lock".into()))?;
    Ok(guard.keys().cloned().collect())
}

/// @id: miyustore_tool_commerce_product_resolve
/// @role: accessor
/// @layer: tool
/// @human: Résout un produit par identifiant ; lecture.
/// @do: commerce_product_resolve_under_governance
pub fn resolve(ctx: &GovernedContext, product_id: &str) -> Result<String, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    let guard = products().lock().map_err(|_| MiyustoreError::InvalidInput("lock".into()))?;
    Ok(guard.get(product_id).cloned().unwrap_or_default())
}

/// @id: miyustore_tool_commerce_product_variations
/// @role: accessor
/// @layer: tool
/// @human: Liste les variations d'un produit ; lecture.
/// @do: commerce_product_variations_under_governance
pub fn variations(ctx: &GovernedContext, _product_id: &str) -> Result<Vec<String>, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    Ok(Vec::new())
}

/// @id: miyustore_tool_commerce_product_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un produit ; WriteIntent KindMother ; décision StrongFather.
/// @do: commerce_product_create_under_governance
pub fn create(ctx: &GovernedContext, payload: &str) -> Result<String, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    let id = format!("prod:{}", UuidIdGenerator.generate());
    let mut guard = products().lock().map_err(|_| MiyustoreError::InvalidInput("lock".into()))?;
    guard.insert(id.clone(), payload.to_string());
    Ok(id)
}

/// @id: miyustore_tool_commerce_product_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour un produit ; WriteIntent KindMother.
/// @do: commerce_product_update_under_governance
pub fn update(ctx: &GovernedContext, product_id: &str, payload: &str) -> Result<(), MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    let mut guard = products().lock().map_err(|_| MiyustoreError::InvalidInput("lock".into()))?;
    guard.insert(product_id.to_string(), payload.to_string());
    Ok(())
}
