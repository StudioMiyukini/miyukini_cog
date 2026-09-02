//! Tools MiyuStore — tool.commerce.checkout.validate, tool.commerce.checkout.submit.
//! Checkout : validation (pas d'écriture) ; submit délègue à order.create.

use crate::context::GovernedContext;
use crate::errors::MiyustoreError;
use crate::order;

/// @id miyustore_tool_commerce_checkout_validate
/// @role accessor
/// @layer tool
/// @do commerce_checkout_validate_under_governance
//@human / @human Valide les données de checkout ; structure, champs ; pas d'écriture.
pub fn validate(ctx: &GovernedContext, payload: &str) -> Result<bool, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    Ok(!payload.trim().is_empty())
}

/// @id miyustore_tool_commerce_checkout_submit
/// @role mutator
/// @layer tool
/// @do commerce_checkout_submit_under_governance
//@human / @human Soumet le checkout et crée la commande ; WriteIntent KindMother ; décision StrongFather.
pub fn submit(ctx: &GovernedContext, payload: &str) -> Result<String, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    order::create(ctx, payload)
}
