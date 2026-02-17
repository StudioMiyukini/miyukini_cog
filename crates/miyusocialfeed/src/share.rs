//! Tools MiyuSocialFeed — tool.social.share.create, list.

use crate::context::GovernedContext;
use crate::errors::MiyusocialfeedError;
use crate::store;
use miyukini_kernel::{IdGenerator as _, UuidIdGenerator};

/// @id: miyusocialfeed_tool_share_create
/// @role: mutator
/// @layer: tool
/// @human: Crée un partage ; WriteIntent KindMother.
/// @do: share_create_under_governance
/// tool.social.share.create
pub fn create(
    ctx: &GovernedContext,
    post_id: &str,
    target_user_id: Option<&str>,
) -> Result<String, MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    let id = format!("share:{}", UuidIdGenerator.generate());
    let user_id = target_user_id.unwrap_or(&ctx.mandate_id).to_string();
    {
        let mut guard = store::shares().lock().map_err(|_| MiyusocialfeedError::InvalidInput("lock".into()))?;
        guard.insert(id.clone(), (post_id.to_string(), user_id.clone()));
    }
    store::shares_by_post().lock().map_err(|_| MiyusocialfeedError::InvalidInput("lock".into()))?.entry(post_id.to_string()).or_default().push(id.clone());
    Ok(id)
}

/// @id: miyusocialfeed_tool_share_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les partages.
/// @do: share_list_under_governance
/// tool.social.share.list
pub fn list(ctx: &GovernedContext, post_id: &str) -> Result<Vec<ShareItem>, MiyusocialfeedError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialfeedError::NoMandate);
    }
    let guard_ids = store::shares_by_post().lock().map_err(|_| MiyusocialfeedError::InvalidInput("lock".into()))?;
    let ids = guard_ids.get(post_id).cloned().unwrap_or_default();
    drop(guard_ids);
    let guard = store::shares().lock().map_err(|_| MiyusocialfeedError::InvalidInput("lock".into()))?;
    let items: Vec<ShareItem> = ids.into_iter().filter_map(|id| guard.get(&id).map(|(_, user_id)| ShareItem { id, user_id: user_id.clone() })).collect();
    Ok(items)
}

/// Élément partage.
#[derive(Debug, Clone)]
pub struct ShareItem {
    pub id: String,
    pub user_id: String,
}
