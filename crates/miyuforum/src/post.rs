//! Tools MiyuForum — tool.forum.post.create, list, get, update.
//! Posts : lecture et WriteIntent KindMother pour create/update.

use crate::context::GovernedContext;
use crate::errors::MiyuforumError;

/// @id: miyuforum_tool_forum_post_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un post ; WriteIntent KindMother.
/// @do: forum_post_create_under_governance
pub fn create(ctx: &GovernedContext, _topic_id: &str, _payload: &str) -> Result<String, MiyuforumError> {
    if !ctx.has_mandate() {
        return Err(MiyuforumError::NoMandate);
    }
    Ok(String::new())
}

/// @id: miyuforum_tool_forum_post_list
/// @role: accessor
/// @layer: tool
/// @human: Liste les posts d'un topic.
/// @do: forum_post_list_under_governance
pub fn list(ctx: &GovernedContext, _topic_id: &str, _filters: Option<&str>) -> Result<Vec<String>, MiyuforumError> {
    if !ctx.has_mandate() {
        return Err(MiyuforumError::NoMandate);
    }
    Ok(Vec::new())
}

/// @id: miyuforum_tool_forum_post_get
/// @role: accessor
/// @layer: tool
/// @human: Récupère un post par identifiant.
/// @do: forum_post_get_under_governance
pub fn get(ctx: &GovernedContext, _post_id: &str) -> Result<String, MiyuforumError> {
    if !ctx.has_mandate() {
        return Err(MiyuforumError::NoMandate);
    }
    Ok(String::new())
}

/// @id: miyuforum_tool_forum_post_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour un post ; WriteIntent KindMother.
/// @do: forum_post_update_under_governance
pub fn update(ctx: &GovernedContext, _post_id: &str, _payload: &str) -> Result<(), MiyuforumError> {
    if !ctx.has_mandate() {
        return Err(MiyuforumError::NoMandate);
    }
    Ok(())
}
