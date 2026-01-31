//! Tools MiyuContacts — tool.contacts.foe.add, remove, list.
//! Autorisation StrongFather ; WriteIntent KindMother.

use crate::context::GovernedContext;
use crate::errors::MiyucontactsError;

/// @id: miyucontacts_tool_foe_add
/// @role: mutator
/// @layer: tool
/// @human: Ajoute un ennemi ; WriteIntent KindMother.
/// @do: foe_add_under_governance
/// tool.contacts.foe.add
pub fn add(ctx: &GovernedContext, _target_id: &str) -> Result<(), MiyucontactsError> {
    if !ctx.has_mandate() {
        return Err(MiyucontactsError::NoMandate);
    }
    Err(MiyucontactsError::Unimplemented)
}

/// @id: miyucontacts_tool_foe_remove
/// @role: mutator
/// @layer: tool
/// @human: Supprime un ennemi ; WriteIntent KindMother.
/// @do: foe_remove_under_governance
/// tool.contacts.foe.remove
pub fn remove(ctx: &GovernedContext, _target_id: &str) -> Result<(), MiyucontactsError> {
    if !ctx.has_mandate() {
        return Err(MiyucontactsError::NoMandate);
    }
    Err(MiyucontactsError::Unimplemented)
}

/// @id: miyucontacts_tool_foe_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les ennemis.
/// @do: foe_list_under_governance
/// tool.contacts.foe.list
pub fn list(ctx: &GovernedContext) -> Result<Vec<crate::friend::ContactItem>, MiyucontactsError> {
    if !ctx.has_mandate() {
        return Err(MiyucontactsError::NoMandate);
    }
    Err(MiyucontactsError::Unimplemented)
}
