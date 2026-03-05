//! Tool MiyuSocialFeed — tool.social.feed.list.

use crate::context::GovernedContext;
use crate::errors::MiyusocialfeedError;
use crate::post::PostItem;
use crate::store;

/// @id: miyusocialfeed_tool_feed_list
/// @role: mutator
/// @layer: tool
/// @human: Liste le flux ; filtres fournis.
/// @do: feed_list_under_governance
/// tool.social.feed.list
pub fn list(
    ctx: &GovernedContext,
    filters: &FeedFilters,
) -> Result<Vec<PostItem>, MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    let guard = store::posts()
        .lock()
        .map_err(|_| MiyusocialfeedError::InvalidInput("lock".into()))?;
    let mut items: Vec<PostItem> = guard
        .iter()
        .filter(|(_, (author_id, _))| filters.author_id.as_ref().map_or(true, |a| author_id == a))
        .map(|(id, (author_id, content))| PostItem {
            id: id.clone(),
            author_id: author_id.clone(),
            content: content.clone(),
        })
        .collect();
    if let Some(limit) = filters.limit {
        items.truncate(limit as usize);
    }
    Ok(items)
}

/// Filtres flux.
#[derive(Debug, Clone, Default)]
pub struct FeedFilters {
    pub author_id: Option<String>,
    pub limit: Option<u32>,
}
