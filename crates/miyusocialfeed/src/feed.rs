//! Tool MiyuSocialFeed — tool.social.feed.list.

use crate::context::GovernedContext;
use crate::errors::MiyusocialfeedError;
use crate::post::PostItem;

/// @id: miyusocialfeed_tool_feed_list
/// @role: mutator
/// @layer: tool
/// @human: Liste le flux ; filtres fournis.
/// @do: feed_list_under_governance
/// tool.social.feed.list
pub fn list(
    ctx: &GovernedContext,
    _filters: &FeedFilters,
) -> Result<Vec<PostItem>, MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    Err(MiyusocialfeedError::Unimplemented)
}

/// Filtres flux.
#[derive(Debug, Clone, Default)]
pub struct FeedFilters {
    pub author_id: Option<String>,
    pub limit: Option<u32>,
}
