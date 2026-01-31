//! Tools MiyuModerationForum — tool.moderation.ban.create, list.

use crate::context::GovernedContext;
use crate::errors::MiyumoderationforumError;

/// @id: miyumoderationforum_tool_ban_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un bannissement ; WriteIntent KindMother.
/// @do: ban_create_under_governance
/// tool.moderation.ban.create
pub fn create(
    ctx: &GovernedContext,
    _user_id: &str,
    _reason: &str,
    _until: Option<&str>,
) -> Result<String, MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    Err(MiyumoderationforumError::Unimplemented)
}

/// @id: miyumoderationforum_tool_ban_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les bannissements.
/// @do: ban_list_under_governance
/// tool.moderation.ban.list
pub fn list(ctx: &GovernedContext) -> Result<Vec<BanItem>, MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    Err(MiyumoderationforumError::Unimplemented)
}

/// Élément bannissement.
#[derive(Debug, Clone)]
pub struct BanItem {
    pub id: String,
    pub user_id: String,
    pub reason: String,
    pub until: Option<String>,
}
