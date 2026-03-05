//! Tools MiyuModerationForum — tool.moderation.ban.create, list.

use crate::context::GovernedContext;
use crate::errors::MiyumoderationforumError;
use crate::store;
use miyukini_kernel::{IdGenerator as _, UuidIdGenerator};

/// @id: miyumoderationforum_tool_ban_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un bannissement ; WriteIntent KindMother.
/// @do: ban_create_under_governance
/// tool.moderation.ban.create
pub fn create(
    ctx: &GovernedContext,
    user_id: &str,
    reason: &str,
    until: Option<&str>,
) -> Result<String, MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    let id = format!("ban:{}", UuidIdGenerator.generate());
    let until_opt = until.map(String::from);
    let mut guard = store::bans()
        .lock()
        .map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?;
    guard.insert(
        id.clone(),
        (user_id.to_string(), reason.to_string(), until_opt),
    );
    Ok(id)
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
    let guard = store::bans()
        .lock()
        .map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?;
    let items = guard
        .iter()
        .map(|(id, (user_id, reason, until))| BanItem {
            id: id.clone(),
            user_id: user_id.clone(),
            reason: reason.clone(),
            until: until.clone(),
        })
        .collect();
    Ok(items)
}

/// Élément bannissement.
#[derive(Debug, Clone)]
pub struct BanItem {
    pub id: String,
    pub user_id: String,
    pub reason: String,
    pub until: Option<String>,
}
