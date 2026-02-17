//! Tool MiyuPosSales — tool.pos.context.store.resolve.

use crate::context::GovernedContext;
use crate::errors::MiyupossalesError;

/// @id: miyupossales_tool_store_resolve
/// @role: mutator
/// @layer: tool
/// @human: Résout le magasin/point de vente courant pour le contexte.
/// @do: store_resolve_under_governance
/// tool.pos.context.store.resolve
pub fn store_resolve(ctx: &GovernedContext) -> Result<StoreResult, MiyupossalesError> {
    if !ctx.has_mandate() {
        return Err(MiyupossalesError::NoMandate);
    }
    Ok(StoreResult::default())
}

/// Résultat magasin.
#[derive(Debug, Clone, Default)]
pub struct StoreResult {
    pub store_id: String,
    pub name: String,
}
