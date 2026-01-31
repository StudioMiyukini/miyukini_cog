//! Tools MiyuStore — tool.commerce.product.* (list, resolve, variations, create, update).
//! Produits : lecture ; create/update = WriteIntent KindMother ; décision StrongFather.

use crate::context::GovernedContext;
use crate::errors::MiyustoreError;

/// @id: miyustore_tool_commerce_product_list
/// @role: accessor
/// @layer: tool
/// @human: Liste les produits ; filtres fournis.
/// @do: commerce_product_list_under_governance
pub fn list(ctx: &GovernedContext, _filters: Option<&str>) -> Result<Vec<String>, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    Err(MiyustoreError::Unimplemented)
}

/// @id: miyustore_tool_commerce_product_resolve
/// @role: accessor
/// @layer: tool
/// @human: Résout un produit par identifiant ; lecture.
/// @do: commerce_product_resolve_under_governance
pub fn resolve(ctx: &GovernedContext, _product_id: &str) -> Result<String, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    Err(MiyustoreError::Unimplemented)
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
    Err(MiyustoreError::Unimplemented)
}

/// @id: miyustore_tool_commerce_product_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un produit ; WriteIntent KindMother ; décision StrongFather.
/// @do: commerce_product_create_under_governance
pub fn create(ctx: &GovernedContext, _payload: &str) -> Result<String, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    Err(MiyustoreError::Unimplemented)
}

/// @id: miyustore_tool_commerce_product_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour un produit ; WriteIntent KindMother.
/// @do: commerce_product_update_under_governance
pub fn update(ctx: &GovernedContext, _product_id: &str, _payload: &str) -> Result<(), MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    Err(MiyustoreError::Unimplemented)
}
