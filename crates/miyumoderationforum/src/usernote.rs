//! Tools MiyuModerationForum — tool.moderation.usernote.create, list.

use crate::context::GovernedContext;
use crate::errors::MiyumoderationforumError;
use crate::store;
use miyukini_kernel::{IdGenerator as _, UuidIdGenerator};

/// @id: miyumoderationforum_tool_usernote_create
/// @role: mutator
/// @layer: tool
/// @human: Crée une note modérateur ; WriteIntent KindMother.
/// @do: usernote_create_under_governance
/// tool.moderation.usernote.create
pub fn create(
    ctx: &GovernedContext,
    user_id: &str,
    content: &str,
) -> Result<String, MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    let id = format!("note:{}", UuidIdGenerator.generate());
    {
        let mut guard = store::usernotes()
            .lock()
            .map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?;
        guard.insert(id.clone(), (user_id.to_string(), content.to_string()));
    }
    {
        let mut guard = store::usernotes_by_user()
            .lock()
            .map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?;
        guard
            .entry(user_id.to_string())
            .or_default()
            .push(id.clone());
    }
    Ok(id)
}

/// @id: miyumoderationforum_tool_usernote_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les notes modérateur.
/// @do: usernote_list_under_governance
/// tool.moderation.usernote.list
pub fn list(
    ctx: &GovernedContext,
    user_id: &str,
) -> Result<Vec<UsernoteItem>, MiyumoderationforumError> {
    if !ctx.has_mandate() {
        return Err(MiyumoderationforumError::NoMandate);
    }
    let guard_ids = store::usernotes_by_user()
        .lock()
        .map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?;
    let ids = guard_ids.get(user_id).cloned().unwrap_or_default();
    drop(guard_ids);
    let guard_notes = store::usernotes()
        .lock()
        .map_err(|_| MiyumoderationforumError::InvalidInput("lock".into()))?;
    let items: Vec<UsernoteItem> = ids
        .into_iter()
        .filter_map(|id| {
            guard_notes.get(&id).map(|(_, content)| UsernoteItem {
                id,
                content: content.clone(),
            })
        })
        .collect();
    Ok(items)
}

/// Élément note.
#[derive(Debug, Clone)]
pub struct UsernoteItem {
    pub id: String,
    pub content: String,
}
