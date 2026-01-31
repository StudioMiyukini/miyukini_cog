//! Tool MiyuSocialModeration — tool.social.post.delete (visibilité).

use crate::context::GovernedContext;
use crate::errors::MiyusocialmoderationError;

/// @id: miyusocialmoderation_tool_post_delete
/// @role: mutator
/// @layer: tool
/// @human: Supprime un post (visibilité) ; WriteIntent KindMother.
/// @do: post_delete_under_governance
/// tool.social.post.delete
pub fn delete(ctx: &GovernedContext, _post_id: &str) -> Result<(), MiyusocialmoderationError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialmoderationError::NoMandate);
    }
    Err(MiyusocialmoderationError::Unimplemented)
}
