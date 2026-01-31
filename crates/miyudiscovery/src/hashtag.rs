//! Tools MiyuDiscovery — tool.social.hashtag.list, get, trending.
//! Lecture ; politique StrongFather.

use crate::context::GovernedContext;
use crate::errors::MiyudiscoveryError;

/// @id: miyudiscovery_tool_hashtag_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les hashtags.
/// @do: hashtag_list_under_governance
/// tool.social.hashtag.list
pub fn list(ctx: &GovernedContext) -> Result<Vec<HashtagItem>, MiyudiscoveryError> {
    if !ctx.has_mandate() {
        return Err(MiyudiscoveryError::NoMandate);
    }
    Ok(Vec::new())
}

/// @id: miyudiscovery_tool_hashtag_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère un hashtag et ses posts.
/// @do: hashtag_get_under_governance
/// tool.social.hashtag.get
pub fn get(
    ctx: &GovernedContext,
    hashtag: &str,
) -> Result<HashtagDetail, MiyudiscoveryError> {
    if !ctx.has_mandate() {
        return Err(MiyudiscoveryError::NoMandate);
    }
    Ok(HashtagDetail {
        tag: hashtag.trim().to_string(),
        post_ids: Vec::new(),
    })
}

/// @id: miyudiscovery_tool_hashtag_trending
/// @role: mutator
/// @layer: tool
/// @human: Liste les hashtags tendance ; politique StrongFather.
/// @do: hashtag_trending_under_governance
/// tool.social.hashtag.trending
pub fn trending(ctx: &GovernedContext) -> Result<Vec<HashtagItem>, MiyudiscoveryError> {
    if !ctx.has_mandate() {
        return Err(MiyudiscoveryError::NoMandate);
    }
    Ok(Vec::new())
}

/// Élément hashtag.
#[derive(Debug, Clone)]
pub struct HashtagItem {
    pub tag: String,
    pub count: u64,
}

/// Détail hashtag + posts.
#[derive(Debug, Clone, Default)]
pub struct HashtagDetail {
    pub tag: String,
    pub post_ids: Vec<String>,
}
