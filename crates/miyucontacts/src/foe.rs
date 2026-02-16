//! Tools MiyuContacts — tool.contacts.foe.add, remove, list.

use crate::context::GovernedContext;
use crate::errors::MiyucontactsError;
use crate::friend::ContactItem;
use crate::store;

/// @id: miyucontacts_tool_foe_add
/// @role: mutator
/// @layer: tool
/// @human: Ajoute un ennemi ; WriteIntent KindMother.
/// @do: foe_add_under_governance
/// tool.contacts.foe.add
pub fn add(ctx: &GovernedContext, target_id: &str) -> Result<(), MiyucontactsError> {
    if !ctx.has_mandate() {
        return Err(MiyucontactsError::NoMandate);
    }
    let st = store::default_store();
    let mut guard = st
        .lock()
        .map_err(|_| MiyucontactsError::InvalidInput("store lock".into()))?;
    guard
        .foes
        .entry(ctx.mandate_id.clone())
        .or_default()
        .insert(target_id.to_string());
    Ok(())
}

/// @id: miyucontacts_tool_foe_remove
/// @role: mutator
/// @layer: tool
/// @human: Supprime un ennemi ; WriteIntent KindMother.
/// @do: foe_remove_under_governance
/// tool.contacts.foe.remove
pub fn remove(ctx: &GovernedContext, target_id: &str) -> Result<(), MiyucontactsError> {
    if !ctx.has_mandate() {
        return Err(MiyucontactsError::NoMandate);
    }
    let st = store::default_store();
    let mut guard = st
        .lock()
        .map_err(|_| MiyucontactsError::InvalidInput("store lock".into()))?;
    if let Some(set) = guard.foes.get_mut(&ctx.mandate_id) {
        set.remove(target_id);
    }
    Ok(())
}

/// @id: miyucontacts_tool_foe_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les ennemis.
/// @do: foe_list_under_governance
/// tool.contacts.foe.list
pub fn list(ctx: &GovernedContext) -> Result<Vec<ContactItem>, MiyucontactsError> {
    if !ctx.has_mandate() {
        return Err(MiyucontactsError::NoMandate);
    }
    let st = store::default_store();
    let guard = st
        .lock()
        .map_err(|_| MiyucontactsError::InvalidInput("store lock".into()))?;
    let items: Vec<ContactItem> = guard
        .foes
        .get(&ctx.mandate_id)
        .map(|set| {
            set.iter()
                .map(|id| ContactItem {
                    id: id.clone(),
                    contact_type: "foe".to_string(),
                })
                .collect()
        })
        .unwrap_or_default();
    Ok(items)
}
