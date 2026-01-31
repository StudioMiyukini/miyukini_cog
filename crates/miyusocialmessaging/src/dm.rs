//! Tools MiyuSocialMessaging — tool.social.dm.send, list, get, reaction.add, reaction.remove, readmark.set.

use crate::context::GovernedContext;
use crate::errors::MiyusocialmessagingError;

/// @id: miyusocialmessaging_tool_dm_send
/// @role: mutator
/// @layer: tool
/// @human: Envoie un DM ; WriteIntent KindMother.
/// @do: dm_send_under_governance
/// tool.social.dm.send
pub fn send(
    ctx: &GovernedContext,
    _conversation_id: &str,
    _content: &str,
) -> Result<String, MiyusocialmessagingError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmessagingError::NoMandate);
    }
    Err(MiyusocialmessagingError::Unimplemented)
}

/// @id: miyusocialmessaging_tool_dm_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les messages d'une conversation.
/// @do: dm_list_under_governance
/// tool.social.dm.list
pub fn list(
    ctx: &GovernedContext,
    _conversation_id: &str,
) -> Result<Vec<DmItem>, MiyusocialmessagingError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmessagingError::NoMandate);
    }
    Err(MiyusocialmessagingError::Unimplemented)
}

/// @id: miyusocialmessaging_tool_dm_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère un message.
/// @do: dm_get_under_governance
/// tool.social.dm.get
pub fn get(ctx: &GovernedContext, _message_id: &str) -> Result<DmItem, MiyusocialmessagingError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmessagingError::NoMandate);
    }
    Err(MiyusocialmessagingError::Unimplemented)
}

/// @id: miyusocialmessaging_tool_dm_reaction_add
/// @role: mutator
/// @layer: tool
/// @human: Ajoute une réaction à un DM ; WriteIntent KindMother.
/// @do: dm_reaction_add_under_governance
/// tool.social.dm.reaction.add
pub fn reaction_add(
    ctx: &GovernedContext,
    _message_id: &str,
    _reaction_type: &str,
) -> Result<(), MiyusocialmessagingError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmessagingError::NoMandate);
    }
    Err(MiyusocialmessagingError::Unimplemented)
}

/// @id: miyusocialmessaging_tool_dm_reaction_remove
/// @role: mutator
/// @layer: tool
/// @human: Supprime une réaction ; WriteIntent KindMother.
/// @do: dm_reaction_remove_under_governance
/// tool.social.dm.reaction.remove
pub fn reaction_remove(
    ctx: &GovernedContext,
    _message_id: &str,
    _reaction_type: &str,
) -> Result<(), MiyusocialmessagingError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmessagingError::NoMandate);
    }
    Err(MiyusocialmessagingError::Unimplemented)
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
    Err(MiyusocialmessagingError::Unimplemented)
}

/// Élément message.
#[derive(Debug, Clone)]
pub struct DmItem {
    pub id: String,
    pub sender_id: String,
    pub content: String,
}
