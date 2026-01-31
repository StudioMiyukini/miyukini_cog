//! Tools MiyuContacts — tool.contacts.friend.add, remove, list.
//! Autorisation StrongFather ; WriteIntent KindMother.

use crate::context::GovernedContext;
use crate::errors::MiyucontactsError;

/// @id: miyucontacts_tool_friend_add
/// @role: mutator
/// @layer: tool
/// @human: Ajoute un ami ; WriteIntent KindMother.
/// @do: friend_add_under_governance
/// tool.contacts.friend.add
pub fn add(ctx: &GovernedContext, _target_id: &str) -> Result<(), MiyucontactsError> {
    if !ctx.has_mandate() {
        return Err(MiyucontactsError::NoMandate);
    }
    Err(MiyucontactsError::Unimplemented)
}

/// @id: miyucontacts_tool_friend_remove
/// @role: mutator
/// @layer: tool
/// @human: Supprime un ami ; WriteIntent KindMother.
/// @do: friend_remove_under_governance
/// tool.contacts.friend.remove
pub fn remove(ctx: &GovernedContext, _target_id: &str) -> Result<(), MiyucontactsError> {
    if !ctx.has_mandate() {
        return Err(MiyucontactsError::NoMandate);
    }
    Err(MiyucontactsError::Unimplemented)
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
    Err(MiyucontactsError::Unimplemented)
}

/// Élément contact (réponse).
#[derive(Debug, Clone)]
pub struct ContactItem {
    pub id: String,
    pub contact_type: String,
}
