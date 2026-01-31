//! Tools MiyuForum — tool.forum.category.list, get, create, update.
//! Catégories : lecture et WriteIntent KindMother pour create/update.

use crate::context::GovernedContext;
use crate::errors::MiyuforumError;

/// @id: miyuforum_tool_forum_category_list
/// @role: accessor
/// @layer: tool
/// @human: Liste les catégories.
/// @do: forum_category_list_under_governance
/// tool.forum.category.list — lecture.
pub fn list(ctx: &GovernedContext) -> Result<Vec<String>, MiyuforumError> {
    if !ctx.has_mandate() {
        return Err(MiyuforumError::NoMandate);
    }
    Err(MiyuforumError::Unimplemented)
}

/// @id: miyuforum_tool_forum_category_get
/// @role: accessor
/// @layer: tool
/// @human: Récupère une catégorie par identifiant.
/// @do: forum_category_get_under_governance
/// tool.forum.category.get — lecture.
pub fn get(ctx: &GovernedContext, _category_id: &str) -> Result<String, MiyuforumError> {
    if !ctx.has_mandate() {
        return Err(MiyuforumError::NoMandate);
    }
    Err(MiyuforumError::Unimplemented)
}

/// @id: miyuforum_tool_forum_category_create
/// @role: mutator
/// @layer: tool
/// @human: Crée une catégorie ; WriteIntent KindMother.
/// @do: forum_category_create_under_governance
/// tool.forum.category.create — ne décide pas ; WriteIntent.
pub fn create(ctx: &GovernedContext, _payload: &str) -> Result<String, MiyuforumError> {
    if !ctx.has_mandate() {
        return Err(MiyuforumError::NoMandate);
    }
    Err(MiyuforumError::Unimplemented)
}

/// @id: miyuforum_tool_forum_category_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour une catégorie ; WriteIntent KindMother.
/// @do: forum_category_update_under_governance
/// tool.forum.category.update — ne décide pas ; WriteIntent.
pub fn update(
    ctx: &GovernedContext,
    _category_id: &str,
    _payload: &str,
) -> Result<(), MiyuforumError> {
    if !ctx.has_mandate() {
        return Err(MiyuforumError::NoMandate);
    }
    Err(MiyuforumError::Unimplemented)
}
