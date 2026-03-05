//! Tools MiyuSocialModeration — tool.social.block.add, remove, list.

use crate::context::GovernedContext;
use crate::errors::MiyusocialmoderationError;
use crate::store;

/// @id: miyusocialmoderation_tool_block_add
/// @role: mutator
/// @layer: tool
/// @human: Bloque un utilisateur ; WriteIntent KindMother.
/// @do: block_add_under_governance
/// tool.social.block.add
pub fn add(
    ctx: &GovernedContext,
    user_id: &str,
    blocked_id: &str,
) -> Result<(), MiyusocialmoderationError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmoderationError::NoMandate);
    }
    let mut guard = store::blocks()
        .lock()
        .map_err(|_| MiyusocialmoderationError::InvalidInput("lock".into()))?;
    let v = guard.entry(user_id.to_string()).or_default();
    if !v.contains(&blocked_id.to_string()) {
        v.push(blocked_id.to_string());
    }
    Ok(())
}

/// @id: miyusocialmoderation_tool_block_remove
/// @role: mutator
/// @layer: tool
/// @human: Débloque un utilisateur ; WriteIntent KindMother.
/// @do: block_remove_under_governance
/// tool.social.block.remove
pub fn remove(
    ctx: &GovernedContext,
    user_id: &str,
    blocked_id: &str,
) -> Result<(), MiyusocialmoderationError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmoderationError::NoMandate);
    }
    if let Some(v) = store::blocks()
        .lock()
        .map_err(|_| MiyusocialmoderationError::InvalidInput("lock".into()))?
        .get_mut(user_id)
    {
        v.retain(|id| id != blocked_id);
    }
    Ok(())
}

/// @id: miyusocialmoderation_tool_block_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les utilisateurs bloqués.
/// @do: block_list_under_governance
/// tool.social.block.list
pub fn list(
    ctx: &GovernedContext,
    user_id: &str,
) -> Result<Vec<String>, MiyusocialmoderationError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmoderationError::NoMandate);
    }
    let guard = store::blocks()
        .lock()
        .map_err(|_| MiyusocialmoderationError::InvalidInput("lock".into()))?;
    Ok(guard.get(user_id).cloned().unwrap_or_default())
}
