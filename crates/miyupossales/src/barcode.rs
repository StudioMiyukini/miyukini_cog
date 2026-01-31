//! Tool MiyuPosSales — tool.pos.barcode.parse.

use crate::context::GovernedContext;
use crate::errors::MiyupossalesError;

/// @id: miyupossales_tool_barcode_parse
/// @role: mutator
/// @layer: tool
/// @human: Parse un code-barres (optionnel : poids) ; retourne item + quantité.
/// @do: barcode_parse_under_governance
/// tool.pos.barcode.parse
pub fn parse(
    ctx: &GovernedContext,
    _barcode: &str,
) -> Result<BarcodeParseResult, MiyupossalesError> {
    if !ctx.has_mandate() {
        return Err(MiyupossalesError::NoMandate);
    }
    Err(MiyupossalesError::Unimplemented)
}

/// Résultat parse code-barres.
#[derive(Debug, Clone, Default)]
pub struct BarcodeParseResult {
    pub article_id: String,
    pub variant_id: Option<String>,
    pub quantity: f64,
}
