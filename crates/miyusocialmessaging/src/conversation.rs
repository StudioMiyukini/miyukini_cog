//! Tools MiyuSocialMessaging — tool.social.conversation.list, get.

use crate::context::GovernedContext;
use crate::errors::MiyusocialmessagingError;
use crate::store;

/// @id: miyusocialmessaging_tool_conversation_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les conversations.
/// @do: conversation_list_under_governance
/// tool.social.conversation.list
pub fn list(ctx: &GovernedContext) -> Result<Vec<ConversationItem>, MiyusocialmessagingError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmessagingError::NoMandate);
    }
    let guard = store::conversations()
        .lock()
        .map_err(|_| MiyusocialmessagingError::InvalidInput("lock".into()))?;
    let items = guard
        .iter()
        .filter(|(_, participants)| participants.contains(&ctx.mandate_id))
        .map(|(id, participants)| ConversationItem {
            id: id.clone(),
            participant_ids: participants.clone(),
        })
        .collect();
    Ok(items)
}

/// @id: miyusocialmessaging_tool_conversation_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère une conversation (fil).
/// @do: conversation_get_under_governance
/// tool.social.conversation.get
pub fn get(
    ctx: &GovernedContext,
    conversation_id: &str,
) -> Result<ConversationDetail, MiyusocialmessagingError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmessagingError::NoMandate);
    }
    let guard_conv = store::conversations()
        .lock()
        .map_err(|_| MiyusocialmessagingError::InvalidInput("lock".into()))?;
    let participant_ids = guard_conv
        .get(conversation_id)
        .ok_or_else(|| MiyusocialmessagingError::InvalidInput("conversation not found".into()))?
        .clone();
    drop(guard_conv);
    let message_ids = store::conversation_messages()
        .lock()
        .map_err(|_| MiyusocialmessagingError::InvalidInput("lock".into()))?
        .get(conversation_id)
        .cloned()
        .unwrap_or_default();
    Ok(ConversationDetail {
        id: conversation_id.to_string(),
        participant_ids,
        message_ids,
    })
}

/// Élément conversation.
#[derive(Debug, Clone)]
pub struct ConversationItem {
    pub id: String,
    pub participant_ids: Vec<String>,
}

/// Détail conversation.
#[derive(Debug, Clone, Default)]
pub struct ConversationDetail {
    pub id: String,
    pub participant_ids: Vec<String>,
    pub message_ids: Vec<String>,
}
