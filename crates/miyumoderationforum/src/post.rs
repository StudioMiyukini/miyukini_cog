//! Tools MiyuModerationForum — tool.forum.post.edit, lock, delete.

use crate::context::GovernedContext;
use crate::errors::MiyumoderationforumError;
use crate::store;

/// @id: miyumoderationforum_tool_post_edit
/// @role: mutator
/// @layer: tool
/// @human: Édite un post (modération) ; WriteIntent KindMother.
/// @do: post_edit_under_governance
/// tool.forum.post.edit
pub fn edit(
    ctx: &GovernedContext,
    post_id: &str,
    new_content: &str,
) -> Result<(), MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    let mut guard = store::posts().lock().map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?;
    let (content, _locked) = guard.get_mut(post_id).ok_or_else(|| MiyumoderationforumError::InvalidInput("post not found".into()))?;
    *content = new_content.to_string();
    Ok(())
}

/// @id: miyumoderationforum_tool_post_lock
/// @role: mutator
/// @layer: tool
/// @human: Verrouille un post ; WriteIntent KindMother.
/// @do: post_lock_under_governance
/// tool.forum.post.lock
pub fn lock(ctx: &GovernedContext, post_id: &str) -> Result<(), MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    let mut guard = store::posts().lock().map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?;
    let (_, locked) = guard.get_mut(post_id).ok_or_else(|| MiyumoderationforumError::InvalidInput("post not found".into()))?;
    *locked = true;
    Ok(())
}

/// @id: miyumoderationforum_tool_post_delete
/// @role: mutator
/// @layer: tool
/// @human: Supprime un post ; WriteIntent KindMother.
/// @do: post_delete_under_governance
/// tool.forum.post.delete
pub fn delete(ctx: &GovernedContext, post_id: &str) -> Result<(), MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    let mut guard = store::posts().lock().map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?;
    guard.remove(post_id).ok_or_else(|| MiyumoderationforumError::InvalidInput("post not found".into()))?;
    Ok(())
}
