//! Tools MiyuForum — tool.forum.readtrack.mark, tool.forum.readtrack.list.
//! Suivi lu : marquer comme lu (WriteIntent KindMother) et lister.

use crate::context::GovernedContext;
use crate::errors::MiyuforumError;

/// @id: miyuforum_tool_forum_readtrack_mark
/// @role: mutator
/// @layer: tool
/// @human: Marque comme lu ; WriteIntent KindMother.
/// @do: forum_readtrack_mark_under_governance
pub fn mark(
    ctx: &GovernedContext,
    _target_ref: &str,
    _target_id: &str,
) -> Result<(), MiyuforumError> {
    if !ctx.has_mandate() {
        return Err(MiyuforumError::NoMandate);
    }
    Err(MiyuforumError::Unimplemented)
}

/// @id: miyuforum_tool_forum_readtrack_list
/// @role: accessor
/// @layer: tool
/// @human: Liste le suivi lu (utilisateur/board).
/// @do: forum_readtrack_list_under_governance
pub fn list(ctx: &GovernedContext, _user_ref: &str, _board_id: Option<&str>) -> Result<Vec<String>, MiyuforumError> {
    if !ctx.has_mandate() {
        return Err(MiyuforumError::NoMandate);
    }
    Err(MiyuforumError::Unimplemented)
}
