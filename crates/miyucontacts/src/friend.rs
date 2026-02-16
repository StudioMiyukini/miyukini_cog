//! Tools MiyuContacts — tool.contacts.friend.add, remove, list.
//! Autorisation StrongFather ; WriteIntent KindMother.

use crate::context::GovernedContext;
use crate::errors::MiyucontactsError;
use crate::store;

/// Élément contact (réponse).
#[derive(Debug, Clone)]
pub struct ContactItem {
    pub id: String,
    pub contact_type: String,
}

/// @id: miyucontacts_tool_friend_add
/// @role: mutator
/// @layer: tool
/// @human: Ajoute un ami ; WriteIntent KindMother.
/// @do: friend_add_under_governance
/// tool.contacts.friend.add
pub fn add(ctx: &GovernedContext, target_id: &str) -> Result<(), MiyucontactsError> {
    if !ctx.has_mandate() {
        return Err(MiyucontactsError::NoMandate);
    }
    let st = store::default_store();
    let mut guard = st
        .lock()
        .map_err(|_| MiyucontactsError::InvalidInput("store lock".into()))?;
    guard
        .friends
        .entry(ctx.mandate_id.clone())
        .or_default()
        .insert(target_id.to_string());
    Ok(())
}

/// @id: miyucontacts_tool_friend_remove
/// @role: mutator
/// @layer: tool
/// @human: Supprime un ami ; WriteIntent KindMother.
/// @do: friend_remove_under_governance
/// tool.contacts.friend.remove
pub fn remove(ctx: &GovernedContext, target_id: &str) -> Result<(), MiyucontactsError> {
    if !ctx.has_mandate() {
        return Err(MiyucontactsError::NoMandate);
    }
    let st = store::default_store();
    let mut guard = st
        .lock()
        .map_err(|_| MiyucontactsError::InvalidInput("store lock".into()))?;
    if let Some(set) = guard.friends.get_mut(&ctx.mandate_id) {
        set.remove(target_id);
    }
    Ok(())
}

/// @id: miyucontacts_tool_friend_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les amis.
/// @do: friend_list_under_governance
/// tool.contacts.friend.list
pub fn list(ctx: &GovernedContext) -> Result<Vec<ContactItem>, MiyucontactsError> {
    if !ctx.has_mandate() {
        return Err(MiyucontactsError::NoMandate);
    }
    let st = store::default_store();
    let guard = st
        .lock()
        .map_err(|_| MiyucontactsError::InvalidInput("store lock".into()))?;
    let items: Vec<ContactItem> = guard
        .friends
        .get(&ctx.mandate_id)
        .map(|set| {
            set.iter()
                .map(|id| ContactItem {
                    id: id.clone(),
                    contact_type: "friend".to_string(),
                })
                .collect()
        })
        .unwrap_or_default();
    Ok(items)
}
