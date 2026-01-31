//! Tools MiyuPosSales — tool.pos.sale.create, add_item, remove_item.

use crate::context::GovernedContext;
use crate::errors::MiyupossalesError;

/// @id: miyupossales_tool_sale_create
/// @role: mutator
/// @layer: tool
/// @human: Crée une vente (ticket) à partir du contexte fourni ; WriteIntent KindMother.
/// @do: sale_create_under_governance
/// tool.pos.sale.create
pub fn create(ctx: &GovernedContext, _store_id: &str) -> Result<String, MiyupossalesError> {
    if !ctx.has_mandate() {
        return Err(MiyupossalesError::NoMandate);
    }
    Err(MiyupossalesError::Unimplemented)
}

/// @id: miyupossales_tool_sale_add_item
/// @role: mutator
/// @layer: tool
/// @human: Ajoute une ligne (article, variante, modificateurs, qté) à une vente ; WriteIntent KindMother.
/// @do: sale_add_item_under_governance
/// tool.pos.sale.add_item
pub fn add_item(
    ctx: &GovernedContext,
    _sale_id: &str,
    _article_id: &str,
    _quantity: u32,
    _variant_id: Option<&str>,
    _modifier_ids: &[String],
) -> Result<String, MiyupossalesError> {
    if !ctx.has_mandate() {
        return Err(MiyupossalesError::NoMandate);
    }
    Err(MiyupossalesError::Unimplemented)
}

/// @id: miyupossales_tool_sale_remove_item
/// @role: mutator
/// @layer: tool
/// @human: Retire une ligne d'une vente ; WriteIntent KindMother.
/// @do: sale_remove_item_under_governance
/// tool.pos.sale.remove_item
pub fn remove_item(
    ctx: &GovernedContext,
    _sale_id: &str,
    _line_id: &str,
) -> Result<(), MiyupossalesError> {
    if !ctx.has_mandate() {
        return Err(MiyupossalesError::NoMandate);
    }
    Err(MiyupossalesError::Unimplemented)
}
