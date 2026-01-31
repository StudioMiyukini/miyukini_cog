//! Tools MiyuPosSales — tool.pos.item.variant.resolve, modifier.apply.

use crate::context::GovernedContext;
use crate::errors::MiyupossalesError;

/// @id: miyupossales_tool_variant_resolve
/// @role: mutator
/// @layer: tool
/// @human: Résout une variante (taille, couleur, etc.) pour un article.
/// @do: variant_resolve_under_governance
/// tool.pos.item.variant.resolve
pub fn variant_resolve(
    ctx: &GovernedContext,
    _article_id: &str,
    _variant_attrs: &std::collections::HashMap<String, String>,
) -> Result<VariantResult, MiyupossalesError> {
    if !ctx.has_mandate() {
        return Err(MiyupossalesError::NoMandate);
    }
    Err(MiyupossalesError::Unimplemented)
}

/// @id: miyupossales_tool_modifier_apply
/// @role: mutator
/// @layer: tool
/// @human: Applique des modificateurs (add-ons) à une ligne.
/// @do: modifier_apply_under_governance
/// tool.pos.item.modifier.apply
pub fn modifier_apply(
    ctx: &GovernedContext,
    _line_id: &str,
    _modifier_ids: &[String],
) -> Result<(), MiyupossalesError> {
    if !ctx.has_mandate() {
        return Err(MiyupossalesError::NoMandate);
    }
    Err(MiyupossalesError::Unimplemented)
}

/// Résultat variante.
#[derive(Debug, Clone, Default)]
pub struct VariantResult {
    pub variant_id: String,
    pub sku: String,
}
