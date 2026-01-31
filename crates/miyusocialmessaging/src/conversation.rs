//! Tools MiyuSocialMessaging — tool.social.conversation.list, get.

use crate::context::GovernedContext;
use crate::errors::MiyusocialmessagingError;

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
    Err(MiyusocialmessagingError::Unimplemented)
}

/// @id: miyusocialmessaging_tool_conversation_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère une conversation (fil).
/// @do: conversation_get_under_governance
/// tool.social.conversation.get
pub fn get(
    ctx: &GovernedContext,
    _conversation_id: &str,
) -> Result<ConversationDetail, MiyusocialmessagingError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmessagingError::NoMandate);
    }
    Err(MiyusocialmessagingError::Unimplemented)
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
