//! Tools MiyuSocialFeed — tool.social.comment.create, list, delete.

use crate::context::GovernedContext;
use crate::errors::MiyusocialfeedError;
use crate::store;
use miyukini_kernel::{IdGenerator as _, UuidIdGenerator};

/// @id: miyusocialfeed_tool_comment_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un commentaire ; WriteIntent KindMother.
/// @do: comment_create_under_governance
/// tool.social.comment.create
pub fn create(
    ctx: &GovernedContext,
    post_id: &str,
    content: &str,
) -> Result<String, MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    let id = format!("comment:{}", UuidIdGenerator.generate());
    let author_id = ctx.mandate_id.clone();
    {
        let mut guard = store::comments()
            .lock()
            .map_err(|_| MiyusocialfeedError::InvalidInput("lock".into()))?;
        guard.insert(
            id.clone(),
            (post_id.to_string(), author_id.clone(), content.to_string()),
        );
    }
    store::comments_by_post()
        .lock()
        .map_err(|_| MiyusocialfeedError::InvalidInput("lock".into()))?
        .entry(post_id.to_string())
        .or_default()
        .push(id.clone());
    Ok(id)
}

/// @id: miyusocialfeed_tool_comment_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les commentaires.
/// @do: comment_list_under_governance
/// tool.social.comment.list
pub fn list(ctx: &GovernedContext, post_id: &str) -> Result<Vec<CommentItem>, MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    let guard_ids = store::comments_by_post()
        .lock()
        .map_err(|_| MiyusocialfeedError::InvalidInput("lock".into()))?;
    let ids = guard_ids.get(post_id).cloned().unwrap_or_default();
    drop(guard_ids);
    let guard = store::comments()
        .lock()
        .map_err(|_| MiyusocialfeedError::InvalidInput("lock".into()))?;
    let items: Vec<CommentItem> = ids
        .into_iter()
        .filter_map(|id| {
            guard.get(&id).map(|(_, author_id, content)| CommentItem {
                id,
                author_id: author_id.clone(),
                content: content.clone(),
            })
        })
        .collect();
    Ok(items)
}

/// @id: miyusocialfeed_tool_comment_delete
/// @role: mutator
/// @layer: tool
/// @human: Supprime un commentaire ; WriteIntent KindMother.
/// @do: comment_delete_under_governance
/// tool.social.comment.delete
pub fn delete(ctx: &GovernedContext, comment_id: &str) -> Result<(), MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    let post_id = {
        let mut guard = store::comments()
            .lock()
            .map_err(|_| MiyusocialfeedError::InvalidInput("lock".into()))?;
        guard
            .remove(comment_id)
            .ok_or_else(|| MiyusocialfeedError::InvalidInput("comment not found".into()))
            .map(|(p, _, _)| p)?
    };
    if let Ok(mut by_post) = store::comments_by_post().lock() {
        if let Some(ids) = by_post.get_mut(&post_id) {
            ids.retain(|id| id != comment_id);
        }
    }
    Ok(())
}

/// Élément commentaire.
#[derive(Debug, Clone)]
pub struct CommentItem {
    pub id: String,
    pub author_id: String,
    pub content: String,
}
