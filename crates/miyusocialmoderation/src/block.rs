//! Tools MiyuSocialModeration — tool.social.block.add, remove, list.

use crate::context::GovernedContext;
use crate::errors::MiyusocialmoderationError;

/// @id: miyusocialmoderation_tool_block_add
/// @role: mutator
/// @layer: tool
/// @human: Bloque un utilisateur ; WriteIntent KindMother.
/// @do: block_add_under_governance
/// tool.social.block.add
pub fn add(ctx: &GovernedContext, _user_id: &str, _blocked_id: &str) -> Result<(), MiyusocialmoderationError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmoderationError::NoMandate);
    }
    Err(MiyusocialmoderationError::Unimplemented)
}

/// @id: miyusocialmoderation_tool_block_remove
/// @role: mutator
/// @layer: tool
/// @human: Débloque un utilisateur ; WriteIntent KindMother.
/// @do: block_remove_under_governance
/// tool.social.block.remove
pub fn remove(ctx: &GovernedContext, _user_id: &str, _blocked_id: &str) -> Result<(), MiyusocialmoderationError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmoderationError::NoMandate);
    }
    Err(MiyusocialmoderationError::Unimplemented)
}

/// @id: miyusocialmoderation_tool_block_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les utilisateurs bloqués.
/// @do: block_list_under_governance
/// tool.social.block.list
pub fn list(ctx: &GovernedContext, _user_id: &str) -> Result<Vec<String>, MiyusocialmoderationError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmoderationError::NoMandate);
    }
    Err(MiyusocialmoderationError::Unimplemented)
}
