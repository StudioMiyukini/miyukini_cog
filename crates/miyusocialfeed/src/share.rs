//! Tools MiyuSocialFeed — tool.social.share.create, list.

use crate::context::GovernedContext;
use crate::errors::MiyusocialfeedError;

/// @id: miyusocialfeed_tool_share_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un partage ; WriteIntent KindMother.
/// @do: share_create_under_governance
/// tool.social.share.create
pub fn create(
    ctx: &GovernedContext,
    _post_id: &str,
    _target_user_id: Option<&str>,
) -> Result<String, MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    Err(MiyusocialfeedError::Unimplemented)
}

/// @id: miyusocialfeed_tool_share_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les partages.
/// @do: share_list_under_governance
/// tool.social.share.list
pub fn list(ctx: &GovernedContext, _post_id: &str) -> Result<Vec<ShareItem>, MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    Err(MiyusocialfeedError::Unimplemented)
}

/// Élément partage.
#[derive(Debug, Clone)]
pub struct ShareItem {
    pub id: String,
    pub user_id: String,
}
