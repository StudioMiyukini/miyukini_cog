//! Tools MiyuSocialFeed — tool.social.post.create, update, delete, get.

use crate::context::GovernedContext;
use crate::errors::MiyusocialfeedError;
use crate::store;
use miyukini_kernel::{IdGenerator as _, UuidIdGenerator};

/// @id: miyusocialfeed_tool_post_create
/// @role: mutator
/// @layer: tool
/// @human: Crée une publication ; WriteIntent KindMother.
/// @do: post_create_under_governance
/// tool.social.post.create
pub fn create(
    ctx: &GovernedContext,
    content: &str,
    _visibility: &str,
) -> Result<String, MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    let id = format!("post:{}", UuidIdGenerator.generate());
    let author_id = ctx.mandate_id.clone();
    let mut guard = store::posts().lock().map_err(|_| MiyusocialfeedError::InvalidInput("lock".into()))?;
    guard.insert(id.clone(), (author_id, content.to_string()));
    Ok(id)
}

/// @id: miyusocialfeed_tool_post_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour une publication ; WriteIntent KindMother.
/// @do: post_update_under_governance
/// tool.social.post.update
pub fn update(
    ctx: &GovernedContext,
    post_id: &str,
    content: &str,
) -> Result<(), MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    let mut guard = store::posts().lock().map_err(|_| MiyusocialfeedError::InvalidInput("lock".into()))?;
    let (_, c) = guard.get_mut(post_id).ok_or_else(|| MiyusocialfeedError::InvalidInput("post not found".into()))?;
    *c = content.to_string();
    Ok(())
}

/// @id: miyusocialfeed_tool_post_delete
/// @role: mutator
/// @layer: tool
/// @human: Supprime une publication ; WriteIntent KindMother.
/// @do: post_delete_under_governance
/// tool.social.post.delete
pub fn delete(ctx: &GovernedContext, post_id: &str) -> Result<(), MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    let mut guard = store::posts().lock().map_err(|_| MiyusocialfeedError::InvalidInput("lock".into()))?;
    guard.remove(post_id).ok_or_else(|| MiyusocialfeedError::InvalidInput("post not found".into()))?;
    Ok(())
}

/// @id: miyusocialfeed_tool_post_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère une publication.
/// @do: post_get_under_governance
/// tool.social.post.get
pub fn get(ctx: &GovernedContext, post_id: &str) -> Result<PostItem, MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    let guard = store::posts().lock().map_err(|_| MiyusocialfeedError::InvalidInput("lock".into()))?;
    let (author_id, content) = guard.get(post_id).ok_or_else(|| MiyusocialfeedError::InvalidInput("post not found".into()))?.clone();
    Ok(PostItem { id: post_id.to_string(), author_id, content })
}

/// Élément publication.
#[derive(Debug, Clone)]
pub struct PostItem {
    pub id: String,
    pub author_id: String,
    pub content: String,
}
