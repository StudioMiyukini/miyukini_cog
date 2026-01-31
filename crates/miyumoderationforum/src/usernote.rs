//! Tools MiyuModerationForum — tool.moderation.usernote.create, list.

use crate::context::GovernedContext;
use crate::errors::MiyumoderationforumError;

/// @id: miyumoderationforum_tool_usernote_create
/// @role: mutator
/// @layer: tool
/// @human: Crée une note modérateur ; WriteIntent KindMother.
/// @do: usernote_create_under_governance
/// tool.moderation.usernote.create
pub fn create(
    ctx: &GovernedContext,
    _user_id: &str,
    _content: &str,
) -> Result<String, MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    Err(MiyumoderationforumError::Unimplemented)
}

/// @id: miyumoderationforum_tool_usernote_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les notes modérateur.
/// @do: usernote_list_under_governance
/// tool.moderation.usernote.list
pub fn list(
    ctx: &GovernedContext,
    _user_id: &str,
) -> Result<Vec<UsernoteItem>, MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    Err(MiyumoderationforumError::Unimplemented)
}

/// Élément note.
#[derive(Debug, Clone)]
pub struct UsernoteItem {
    pub id: String,
    pub content: String,
}
