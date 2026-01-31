//! Tool MiyuDiscovery — tool.social.trending.list.
//! Liste les tendances (posts, sujets) ; politique StrongFather.

use crate::context::GovernedContext;
use crate::errors::MiyudiscoveryError;

/// @id: miyudiscovery_tool_trending_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les tendances (posts, sujets) ; politique StrongFather.
/// @do: trending_list_under_governance
/// tool.social.trending.list
pub fn list(ctx: &GovernedContext) -> Result<Vec<TrendingItem>, MiyudiscoveryError> {
    if !ctx.has_mandate() {
        return Err(MiyudiscoveryError::NoMandate);
    }
    Ok(Vec::new())
}

/// Élément tendance.
#[derive(Debug, Clone)]
pub struct TrendingItem {
    pub id: String,
    pub kind: String,
    pub score: f64,
}
