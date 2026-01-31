//! Tools MiyuPM — tool.pm.conversation.list, tool.pm.conversation.get.
//! Liste et récupération de conversations (fils de messages).

use crate::context::GovernedContext;
use crate::errors::MiyupmError;

/// @id: miyupm_tool_pm_conversation_list
/// @role: accessor
/// @layer: tool
/// @human: Liste les conversations.
/// @do: pm_conversation_list_under_governance
/// tool.pm.conversation.list — lecture.
pub fn list(ctx: &GovernedContext, _filters: Option<&str>) -> Result<Vec<String>, MiyupmError> {
    if !ctx.has_mandate() {
        return Err(MiyupmError::NoMandate);
    }
    Err(MiyupmError::Unimplemented)
}

/// @id: miyupm_tool_pm_conversation_get
/// @role: accessor
/// @layer: tool
/// @human: Récupère une conversation (fil) par identifiant.
/// @do: pm_conversation_get_under_governance
/// tool.pm.conversation.get — lecture.
pub fn get(ctx: &GovernedContext, _conversation_id: &str) -> Result<String, MiyupmError> {
    if !ctx.has_mandate() {
        return Err(MiyupmError::NoMandate);
    }
    Err(MiyupmError::Unimplemented)
}
