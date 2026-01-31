//! Tool MiyuPosInventory — tool.pos.label.print.

use crate::context::GovernedContext;
use crate::errors::MiyuposinventoryError;

/// @id: miyuposinventory_tool_label_print
/// @role: mutator
/// @layer: tool
/// @human: Imprime une étiquette code-barres (données fournies).
/// @do: label_print_under_governance
/// tool.pos.label.print
pub fn print(
    ctx: &GovernedContext,
    _article_id: &str,
    _barcode: &str,
    _label_data: &str,
) -> Result<(), MiyuposinventoryError> {
    if !ctx.has_mandate() {
        return Err(MiyuposinventoryError::NoMandate);
    }
    Err(MiyuposinventoryError::Unimplemented)
}
