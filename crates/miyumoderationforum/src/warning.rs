//! Tools MiyuModerationForum — tool.moderation.warning.create, list.

use crate::context::GovernedContext;
use crate::errors::MiyumoderationforumError;
use crate::store;
use miyukini_kernel::{IdGenerator as _, UuidIdGenerator};

/// @id: miyumoderationforum_tool_warning_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un avertissement ; WriteIntent KindMother.
/// @do: warning_create_under_governance
/// tool.moderation.warning.create
pub fn create(
    ctx: &GovernedContext,
    user_id: &str,
    reason: &str,
) -> Result<String, MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    let id = format!("warn:{}", UuidIdGenerator.generate());
    let mut guard = store::warnings().lock().map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?;
    guard.insert(id.clone(), (user_id.to_string(), reason.to_string()));
    Ok(id)
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
    let guard = store::warnings().lock().map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?;
    let items = guard.iter().map(|(id, (user_id, reason))| WarningItem {
        id: id.clone(),
        user_id: user_id.clone(),
        reason: reason.clone(),
    }).collect();
    Ok(items)
}

/// Élément avertissement.
#[derive(Debug, Clone)]
pub struct WarningItem {
    pub id: String,
    pub user_id: String,
    pub reason: String,
}
