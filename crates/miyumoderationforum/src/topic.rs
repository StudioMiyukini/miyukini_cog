//! Tools MiyuModerationForum — tool.forum.topic.lock, move, merge, split, delete, copy.
//! Décision StrongFather ; WriteIntent KindMother.

use crate::context::GovernedContext;
use crate::errors::MiyumoderationforumError;
use crate::store;
use miyukini_kernel::{IdGenerator as _, UuidIdGenerator};

/// @id: miyumoderationforum_tool_topic_lock
/// @role: mutator
/// @layer: tool
/// @human: Verrouille un topic ; WriteIntent KindMother.
/// @do: topic_lock_under_governance
/// tool.forum.topic.lock
pub fn lock(ctx: &GovernedContext, topic_id: &str) -> Result<(), MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    let mut guard = store::topics()
        .lock()
        .map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?;
    let (_, locked) = guard
        .get_mut(topic_id)
        .ok_or_else(|| MiyumoderationforumError::InvalidInput("topic not found".into()))?;
    *locked = true;
    Ok(())
}

/// @id: miyumoderationforum_tool_topic_move
/// @role: mutator
/// @layer: tool
/// @human: Déplace un topic ; WriteIntent KindMother.
/// @do: topic_move_under_governance
/// tool.forum.topic.move
pub fn r#move(
    ctx: &GovernedContext,
    topic_id: &str,
    target_board_id: &str,
) -> Result<(), MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    let mut guard = store::topics()
        .lock()
        .map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?;
    let (board_id, _) = guard
        .get_mut(topic_id)
        .ok_or_else(|| MiyumoderationforumError::InvalidInput("topic not found".into()))?;
    *board_id = target_board_id.to_string();
    Ok(())
}

/// @id: miyumoderationforum_tool_topic_merge
/// @role: mutator
/// @layer: tool
/// @human: Fusionne des topics ; WriteIntent KindMother.
/// @do: topic_merge_under_governance
/// tool.forum.topic.merge
pub fn merge(
    ctx: &GovernedContext,
    source_topic_ids: &[String],
    _target_topic_id: &str,
) -> Result<(), MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    let mut guard = store::topics()
        .lock()
        .map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?;
    for id in source_topic_ids {
        guard.remove(id);
    }
    Ok(())
}

/// @id: miyumoderationforum_tool_topic_split
/// @role: mutator
/// @layer: tool
/// @human: Scinde un topic ; WriteIntent KindMother.
/// @do: topic_split_under_governance
/// tool.forum.topic.split
pub fn split(
    ctx: &GovernedContext,
    topic_id: &str,
    _post_ids_after_split: &[String],
) -> Result<String, MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    let guard = store::topics()
        .lock()
        .map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?;
    let (board_id, locked) = guard
        .get(topic_id)
        .ok_or_else(|| MiyumoderationforumError::InvalidInput("topic not found".into()))?
        .clone();
    drop(guard);
    let new_id = format!("topic:{}", UuidIdGenerator.generate());
    store::topics()
        .lock()
        .map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?
        .insert(new_id.clone(), (board_id, locked));
    Ok(new_id)
}

/// @id: miyumoderationforum_tool_topic_delete
/// @role: mutator
/// @layer: tool
/// @human: Supprime un topic ; WriteIntent KindMother.
/// @do: topic_delete_under_governance
/// tool.forum.topic.delete
pub fn delete(ctx: &GovernedContext, topic_id: &str) -> Result<(), MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    let mut guard = store::topics()
        .lock()
        .map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?;
    guard
        .remove(topic_id)
        .ok_or_else(|| MiyumoderationforumError::InvalidInput("topic not found".into()))?;
    Ok(())
}

/// @id: miyumoderationforum_tool_topic_copy
/// @role: mutator
/// @layer: tool
/// @human: Copie un topic ; WriteIntent KindMother.
/// @do: topic_copy_under_governance
/// tool.forum.topic.copy
pub fn copy(
    ctx: &GovernedContext,
    topic_id: &str,
    target_board_id: &str,
) -> Result<String, MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    let guard = store::topics()
        .lock()
        .map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?;
    let (_, locked) = guard
        .get(topic_id)
        .ok_or_else(|| MiyumoderationforumError::InvalidInput("topic not found".into()))?
        .clone();
    drop(guard);
    let new_id = format!("topic:{}", UuidIdGenerator.generate());
    store::topics()
        .lock()
        .map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?
        .insert(new_id.clone(), (target_board_id.to_string(), locked));
    Ok(new_id)
}
