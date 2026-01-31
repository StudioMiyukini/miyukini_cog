//! Tools MiyuModerationForum — tool.forum.topic.lock, move, merge, split, delete, copy.
//! Décision StrongFather ; WriteIntent KindMother.

use crate::context::GovernedContext;
use crate::errors::MiyumoderationforumError;

/// @id: miyumoderationforum_tool_topic_lock
/// @role: mutator
/// @layer: tool
/// @human: Verrouille un topic ; WriteIntent KindMother.
/// @do: topic_lock_under_governance
/// tool.forum.topic.lock
pub fn lock(ctx: &GovernedContext, _topic_id: &str) -> Result<(), MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    Err(MiyumoderationforumError::Unimplemented)
}

/// @id: miyumoderationforum_tool_topic_move
/// @role: mutator
/// @layer: tool
/// @human: Déplace un topic ; WriteIntent KindMother.
/// @do: topic_move_under_governance
/// tool.forum.topic.move
pub fn r#move(
    ctx: &GovernedContext,
    _topic_id: &str,
    _target_board_id: &str,
) -> Result<(), MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    Err(MiyumoderationforumError::Unimplemented)
}

/// @id: miyumoderationforum_tool_topic_merge
/// @role: mutator
/// @layer: tool
/// @human: Fusionne des topics ; WriteIntent KindMother.
/// @do: topic_merge_under_governance
/// tool.forum.topic.merge
pub fn merge(
    ctx: &GovernedContext,
    _source_topic_ids: &[String],
    _target_topic_id: &str,
) -> Result<(), MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    Err(MiyumoderationforumError::Unimplemented)
}

/// @id: miyumoderationforum_tool_topic_split
/// @role: mutator
/// @layer: tool
/// @human: Scinde un topic ; WriteIntent KindMother.
/// @do: topic_split_under_governance
/// tool.forum.topic.split
pub fn split(
    ctx: &GovernedContext,
    _topic_id: &str,
    _post_ids_after_split: &[String],
) -> Result<String, MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    Err(MiyumoderationforumError::Unimplemented)
}

/// @id: miyumoderationforum_tool_topic_delete
/// @role: mutator
/// @layer: tool
/// @human: Supprime un topic ; WriteIntent KindMother.
/// @do: topic_delete_under_governance
/// tool.forum.topic.delete
pub fn delete(ctx: &GovernedContext, _topic_id: &str) -> Result<(), MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    Err(MiyumoderationforumError::Unimplemented)
}

/// @id: miyumoderationforum_tool_topic_copy
/// @role: mutator
/// @layer: tool
/// @human: Copie un topic ; WriteIntent KindMother.
/// @do: topic_copy_under_governance
/// tool.forum.topic.copy
pub fn copy(
    ctx: &GovernedContext,
    _topic_id: &str,
    _target_board_id: &str,
) -> Result<String, MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    Err(MiyumoderationforumError::Unimplemented)
}
