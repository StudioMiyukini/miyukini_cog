//! Tools MiyuModerationForum — tool.moderation.queue.list, get.

use crate::context::GovernedContext;
use crate::errors::MiyumoderationforumError;

/// @id: miyumoderationforum_tool_queue_list
/// @role: mutator
/// @layer: tool
/// @human: Liste la file d'attente de modération.
/// @do: moderation_queue_list_under_governance
/// tool.moderation.queue.list
pub fn list(ctx: &GovernedContext) -> Result<Vec<QueueItem>, MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    Err(MiyumoderationforumError::Unimplemented)
}

/// @id: miyumoderationforum_tool_queue_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère un élément de la file.
/// @do: moderation_queue_get_under_governance
/// tool.moderation.queue.get
pub fn get(
    ctx: &GovernedContext,
    _item_id: &str,
) -> Result<QueueItemDetail, MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    Err(MiyumoderationforumError::Unimplemented)
}

/// Élément file.
#[derive(Debug, Clone)]
pub struct QueueItem {
    pub id: String,
    pub kind: String,
}

/// Détail élément file.
#[derive(Debug, Clone, Default)]
pub struct QueueItemDetail {
    pub id: String,
    pub kind: String,
    pub payload: String,
}
