//! Tools MiyuModerationForum — tool.forum.post.edit, lock, delete.

use crate::context::GovernedContext;
use crate::errors::MiyumoderationforumError;

/// @id: miyumoderationforum_tool_post_edit
/// @role: mutator
/// @layer: tool
/// @human: Édite un post (modération) ; WriteIntent KindMother.
/// @do: post_edit_under_governance
/// tool.forum.post.edit
pub fn edit(
    ctx: &GovernedContext,
    _post_id: &str,
    _new_content: &str,
) -> Result<(), MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    Err(MiyumoderationforumError::Unimplemented)
}

/// @id: miyumoderationforum_tool_post_lock
/// @role: mutator
/// @layer: tool
/// @human: Verrouille un post ; WriteIntent KindMother.
/// @do: post_lock_under_governance
/// tool.forum.post.lock
pub fn lock(ctx: &GovernedContext, _post_id: &str) -> Result<(), MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    Err(MiyumoderationforumError::Unimplemented)
}

/// @id: miyumoderationforum_tool_post_delete
/// @role: mutator
/// @layer: tool
/// @human: Supprime un post ; WriteIntent KindMother.
/// @do: post_delete_under_governance
/// tool.forum.post.delete
pub fn delete(ctx: &GovernedContext, _post_id: &str) -> Result<(), MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    Err(MiyumoderationforumError::Unimplemented)
}
