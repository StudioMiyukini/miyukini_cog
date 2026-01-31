//! Tools MiyuSocialFeed — tool.social.post.create, update, delete, get.

use crate::context::GovernedContext;
use crate::errors::MiyusocialfeedError;

/// @id: miyusocialfeed_tool_post_create
/// @role: mutator
/// @layer: tool
/// @human: Crée une publication ; WriteIntent KindMother.
/// @do: post_create_under_governance
/// tool.social.post.create
pub fn create(
    ctx: &GovernedContext,
    _content: &str,
    _visibility: &str,
) -> Result<String, MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    Err(MiyusocialfeedError::Unimplemented)
}

/// @id: miyusocialfeed_tool_post_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour une publication ; WriteIntent KindMother.
/// @do: post_update_under_governance
/// tool.social.post.update
pub fn update(
    ctx: &GovernedContext,
    _post_id: &str,
    _content: &str,
) -> Result<(), MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    Err(MiyusocialfeedError::Unimplemented)
}

/// @id: miyusocialfeed_tool_post_delete
/// @role: mutator
/// @layer: tool
/// @human: Supprime une publication ; WriteIntent KindMother.
/// @do: post_delete_under_governance
/// tool.social.post.delete
pub fn delete(ctx: &GovernedContext, _post_id: &str) -> Result<(), MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    Err(MiyusocialfeedError::Unimplemented)
}

/// @id: miyusocialfeed_tool_post_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère une publication.
/// @do: post_get_under_governance
/// tool.social.post.get
pub fn get(ctx: &GovernedContext, _post_id: &str) -> Result<PostItem, MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    Err(MiyusocialfeedError::Unimplemented)
}

/// Élément publication.
#[derive(Debug, Clone)]
pub struct PostItem {
    pub id: String,
    pub author_id: String,
    pub content: String,
}
