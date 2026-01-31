//! Tools MiyuSocialFeed — tool.social.comment.create, list, delete.

use crate::context::GovernedContext;
use crate::errors::MiyusocialfeedError;

/// @id: miyusocialfeed_tool_comment_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un commentaire ; WriteIntent KindMother.
/// @do: comment_create_under_governance
/// tool.social.comment.create
pub fn create(
    ctx: &GovernedContext,
    _post_id: &str,
    _content: &str,
) -> Result<String, MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    Err(MiyusocialfeedError::Unimplemented)
}

/// @id: miyusocialfeed_tool_comment_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les commentaires.
/// @do: comment_list_under_governance
/// tool.social.comment.list
pub fn list(ctx: &GovernedContext, _post_id: &str) -> Result<Vec<CommentItem>, MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    Err(MiyusocialfeedError::Unimplemented)
}

/// @id: miyusocialfeed_tool_comment_delete
/// @role: mutator
/// @layer: tool
/// @human: Supprime un commentaire ; WriteIntent KindMother.
/// @do: comment_delete_under_governance
/// tool.social.comment.delete
pub fn delete(ctx: &GovernedContext, _comment_id: &str) -> Result<(), MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    Err(MiyusocialfeedError::Unimplemented)
}

/// Élément commentaire.
#[derive(Debug, Clone)]
pub struct CommentItem {
    pub id: String,
    pub author_id: String,
    pub content: String,
}
