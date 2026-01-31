//! Tools MiyuStore — tool.commerce.checkout.validate, tool.commerce.checkout.submit.
//! Checkout : validation (pas d'écriture) ; submit = WriteIntent KindMother ; décision StrongFather.

use crate::context::GovernedContext;
use crate::errors::MiyustoreError;

/// @id: miyustore_tool_commerce_checkout_validate
/// @role: accessor
/// @layer: tool
/// @human: Valide les données de checkout ; structure, champs ; pas d'écriture.
/// @do: commerce_checkout_validate_under_governance
pub fn validate(ctx: &GovernedContext, _payload: &str) -> Result<bool, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    Err(MiyustoreError::Unimplemented)
}

/// @id: miyustore_tool_commerce_checkout_submit
/// @role: mutator
/// @layer: tool
/// @human: Soumet le checkout et crée la commande ; WriteIntent KindMother ; décision StrongFather.
/// @do: commerce_checkout_submit_under_governance
pub fn submit(ctx: &GovernedContext, _payload: &str) -> Result<String, MiyustoreError> {
    if !ctx.has_mandate() {
        return Err(MiyustoreError::NoMandate);
    }
    Err(MiyustoreError::Unimplemented)
}
