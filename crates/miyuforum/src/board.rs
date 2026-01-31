//! Tools MiyuForum — tool.forum.board.list, get, create, update.
//! Boards/forums : lecture et WriteIntent KindMother pour create/update.

use crate::context::GovernedContext;
use crate::errors::MiyuforumError;

/// @id: miyuforum_tool_forum_board_list
/// @role: accessor
/// @layer: tool
/// @human: Liste les forums/boards.
/// @do: forum_board_list_under_governance
/// tool.forum.board.list — lecture.
pub fn list(ctx: &GovernedContext, _category_id: Option<&str>) -> Result<Vec<String>, MiyuforumError> {
    if !ctx.has_mandate() {
        return Err(MiyuforumError::NoMandate);
    }
    Ok(Vec::new())
}

/// @id: miyuforum_tool_forum_board_get
/// @role: accessor
/// @layer: tool
/// @human: Récupère un board par identifiant.
/// @do: forum_board_get_under_governance
/// tool.forum.board.get — lecture.
pub fn get(ctx: &GovernedContext, _board_id: &str) -> Result<String, MiyuforumError> {
    if !ctx.has_mandate() {
        return Err(MiyuforumError::NoMandate);
    }
    Ok(String::new())
}

/// @id: miyuforum_tool_forum_board_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un board ; WriteIntent KindMother.
/// @do: forum_board_create_under_governance
/// tool.forum.board.create — ne décide pas ; WriteIntent.
pub fn create(ctx: &GovernedContext, _payload: &str) -> Result<String, MiyuforumError> {
    if !ctx.has_mandate() {
        return Err(MiyuforumError::NoMandate);
    }
    Ok(String::new())
}

/// @id: miyuforum_tool_forum_board_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour un board ; WriteIntent KindMother.
/// @do: forum_board_update_under_governance
/// tool.forum.board.update — ne décide pas ; WriteIntent.
pub fn update(
    ctx: &GovernedContext,
    _board_id: &str,
    _payload: &str,
) -> Result<(), MiyuforumError> {
    if !ctx.has_mandate() {
        return Err(MiyuforumError::NoMandate);
    }
    Ok(())
}
