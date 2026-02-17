//! Tool MiyuPosInventory — tool.inventory.history.list.

use crate::context::GovernedContext;
use crate::errors::MiyuposinventoryError;

/// @id: miyuposinventory_tool_history_list
/// @role: mutator
/// @layer: tool
/// @human: Liste l'historique des mouvements (filtres fournis).
/// @do: history_list_under_governance
/// tool.inventory.history.list
pub fn list(
    ctx: &GovernedContext,
    _article_id: Option<&str>,
    _site_id: Option<&str>,
    _from: &str,
    _to: &str,
) -> Result<Vec<MovementItem>, MiyuposinventoryError> {
    if !ctx.has_mandate() {
        return Err(MiyuposinventoryError::NoMandate);
    }
    Ok(Vec::new())
}

/// Élément mouvement.
#[derive(Debug, Clone)]
pub struct MovementItem {
    pub id: String,
    pub article_id: String,
    pub site_id: String,
    pub delta: i64,
    pub kind: String,
}
