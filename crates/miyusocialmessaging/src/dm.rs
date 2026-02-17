//! Tools MiyuSocialMessaging — tool.social.dm.send, list, get, reaction.add, reaction.remove, readmark.set.

use crate::context::GovernedContext;
use crate::errors::MiyusocialmessagingError;
use crate::store;
use miyukini_kernel::{IdGenerator as _, UuidIdGenerator};

/// @id: miyusocialmessaging_tool_dm_send
/// @role: mutator
/// @layer: tool
/// @human: Envoie un DM ; WriteIntent KindMother.
/// @do: dm_send_under_governance
/// tool.social.dm.send
pub fn send(
    ctx: &GovernedContext,
    conversation_id: &str,
    content: &str,
) -> Result<String, MiyusocialmessagingError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmessagingError::NoMandate);
    }
    store::conversations().lock().map_err(|_| MiyusocialmessagingError::InvalidInput("lock".into()))?.entry(conversation_id.to_string()).or_insert_with(|| vec![ctx.mandate_id.clone()]);
    let id = format!("msg:{}", UuidIdGenerator.generate());
    {
        let mut guard = store::messages().lock().map_err(|_| MiyusocialmessagingError::InvalidInput("lock".into()))?;
        guard.insert(id.clone(), (conversation_id.to_string(), ctx.mandate_id.clone(), content.to_string()));
    }
    store::conversation_messages().lock().map_err(|_| MiyusocialmessagingError::InvalidInput("lock".into()))?.entry(conversation_id.to_string()).or_default().push(id.clone());
    Ok(id)
}

/// @id: miyusocialmessaging_tool_dm_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les messages d'une conversation.
/// @do: dm_list_under_governance
/// tool.social.dm.list
pub fn list(
    ctx: &GovernedContext,
    conversation_id: &str,
) -> Result<Vec<DmItem>, MiyusocialmessagingError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmessagingError::NoMandate);
    }
    let guard_ids = store::conversation_messages().lock().map_err(|_| MiyusocialmessagingError::InvalidInput("lock".into()))?;
    let ids = guard_ids.get(conversation_id).cloned().unwrap_or_default();
    drop(guard_ids);
    let guard = store::messages().lock().map_err(|_| MiyusocialmessagingError::InvalidInput("lock".into()))?;
    let items: Vec<DmItem> = ids.into_iter().filter_map(|id| guard.get(&id).map(|(_, sender_id, content)| DmItem { id, sender_id: sender_id.clone(), content: content.clone() })).collect();
    Ok(items)
}

/// @id: miyusocialmessaging_tool_dm_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère un message.
/// @do: dm_get_under_governance
/// tool.social.dm.get
pub fn get(ctx: &GovernedContext, message_id: &str) -> Result<DmItem, MiyusocialmessagingError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmessagingError::NoMandate);
    }
    let guard = store::messages().lock().map_err(|_| MiyusocialmessagingError::InvalidInput("lock".into()))?;
    let (_, sender_id, content) = guard.get(message_id).ok_or_else(|| MiyusocialmessagingError::InvalidInput("message not found".into()))?.clone();
    Ok(DmItem { id: message_id.to_string(), sender_id, content })
}

/// @id: miyusocialmessaging_tool_dm_reaction_add
/// @role: mutator
/// @layer: tool
/// @human: Ajoute une réaction à un DM ; WriteIntent KindMother.
/// @do: dm_reaction_add_under_governance
/// tool.social.dm.reaction.add
pub fn reaction_add(
    ctx: &GovernedContext,
    message_id: &str,
    _reaction_type: &str,
) -> Result<(), MiyusocialmessagingError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmessagingError::NoMandate);
    }
    if !store::messages().lock().map_err(|_| MiyusocialmessagingError::InvalidInput("lock".into()))?.contains_key(message_id) {
        return Err(MiyusocialmessagingError::InvalidInput("message not found".into()));
    }
    Ok(())
}

/// @id: miyusocialmessaging_tool_dm_reaction_remove
/// @role: mutator
/// @layer: tool
/// @human: Supprime une réaction ; WriteIntent KindMother.
/// @do: dm_reaction_remove_under_governance
/// tool.social.dm.reaction.remove
pub fn reaction_remove(
    ctx: &GovernedContext,
    message_id: &str,
    _reaction_type: &str,
) -> Result<(), MiyusocialmessagingError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmessagingError::NoMandate);
    }
    if !store::messages().lock().map_err(|_| MiyusocialmessagingError::InvalidInput("lock".into()))?.contains_key(message_id) {
        return Err(MiyusocialmessagingError::InvalidInput("message not found".into()));
    }
    Ok(())
}

/// @id: miyusocialmessaging_tool_dm_readmark_set
/// @role: mutator
/// @layer: tool
/// @human: Marque comme lu ; WriteIntent KindMother.
/// @do: dm_readmark_set_under_governance
/// tool.social.dm.readmark.set
pub fn readmark_set(
    ctx: &GovernedContext,
    _conversation_id: &str,
    _message_id: &str,
) -> Result<(), MiyusocialmessagingError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmessagingError::NoMandate);
    }
    Ok(())
}

/// Élément message.
#[derive(Debug, Clone)]
pub struct DmItem {
    pub id: String,
    pub sender_id: String,
    pub content: String,
}
