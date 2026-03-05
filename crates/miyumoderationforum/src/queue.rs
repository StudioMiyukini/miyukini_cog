//! Tools MiyuModerationForum — tool.moderation.queue.list, get.

use crate::context::GovernedContext;
use crate::errors::MiyumoderationforumError;
use crate::store;

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
    let guard = store::queue_items()
        .lock()
        .map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?;
    let items = guard
        .iter()
        .map(|(id, (kind, _))| QueueItem {
            id: id.clone(),
            kind: kind.clone(),
        })
        .collect();
    Ok(items)
}

/// @id: miyumoderationforum_tool_queue_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère un élément de la file.
/// @do: moderation_queue_get_under_governance
/// tool.moderation.queue.get
pub fn get(
    ctx: &GovernedContext,
    item_id: &str,
) -> Result<QueueItemDetail, MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    let guard = store::queue_items()
        .lock()
        .map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?;
    let (kind, payload) = guard
        .get(item_id)
        .ok_or_else(|| MiyumoderationforumError::InvalidInput("item not found".into()))?;
    Ok(QueueItemDetail {
        id: item_id.to_string(),
        kind: kind.clone(),
        payload: payload.clone(),
    })
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
