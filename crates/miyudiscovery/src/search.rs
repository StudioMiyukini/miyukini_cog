//! Tool MiyuDiscovery — tool.social.search.
//! Recherche sociale (scope=social) ; lecture.

use crate::context::GovernedContext;
use crate::errors::MiyudiscoveryError;

/// @id miyudiscovery_tool_search
/// @role mutator
/// @layer tool
/// @do social_search_under_governance
//@human / @human Recherche sociale (scope=social) ; lecture.
/// tool.social.search
pub fn search(
    ctx: &GovernedContext,
    _query: &str,
    _limit: u32,
) -> Result<SearchResult, MiyudiscoveryError> {
    if !ctx.has_mandate() {
        return Err(MiyudiscoveryError::NoMandate);
    }
    Ok(SearchResult {
        post_ids: Vec::new(),
        hashtag_ids: Vec::new(),
    })
}

/// Résultat recherche sociale.
#[derive(Debug, Clone, Default)]
pub struct SearchResult {
    pub post_ids: Vec<String>,
    pub hashtag_ids: Vec<String>,
}
