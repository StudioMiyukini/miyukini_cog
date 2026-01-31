//! Tool MiyuPosInventory — tool.inventory.import.items.

use crate::context::GovernedContext;
use crate::errors::MiyuposinventoryError;

/// @id: miyuposinventory_tool_import_items
/// @role: mutator
/// @layer: tool
/// @human: Importe des articles à partir d'un flux structuré (ex. CSV) ; persistance = KindMother.
/// @do: import_items_under_governance
/// tool.inventory.import.items
pub fn items(
    ctx: &GovernedContext,
    _payload: &[u8],
    _format: &str,
) -> Result<ImportResult, MiyuposinventoryError> {
    if !ctx.has_mandate() {
        return Err(MiyuposinventoryError::NoMandate);
    }
    Err(MiyuposinventoryError::Unimplemented)
}

/// Résultat import.
#[derive(Debug, Clone, Default)]
pub struct ImportResult {
    pub imported: u32,
    pub errors: Vec<String>,
}
