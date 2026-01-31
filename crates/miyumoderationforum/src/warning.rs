//! Tools MiyuModerationForum — tool.moderation.warning.create, list.

use crate::context::GovernedContext;
use crate::errors::MiyumoderationforumError;

/// @id: miyumoderationforum_tool_warning_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un avertissement ; WriteIntent KindMother.
/// @do: warning_create_under_governance
/// tool.moderation.warning.create
pub fn create(
    ctx: &GovernedContext,
    _user_id: &str,
    _reason: &str,
) -> Result<String, MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    Err(MiyumoderationforumError::Unimplemented)
}

/// @id: miyumoderationforum_tool_warning_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les avertissements.
/// @do: warning_list_under_governance
/// tool.moderation.warning.list
pub fn list(ctx: &GovernedContext) -> Result<Vec<WarningItem>, MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    Err(MiyumoderationforumError::Unimplemented)
}

/// Élément avertissement.
#[derive(Debug, Clone)]
pub struct WarningItem {
    pub id: String,
    pub user_id: String,
    pub reason: String,
}
