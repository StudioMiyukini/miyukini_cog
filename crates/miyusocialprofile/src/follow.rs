//! Tools MiyuSocialProfile — tool.social.follow.add, remove, followers.list, following.list.

use crate::context::GovernedContext;
use crate::errors::MiyusocialprofileError;
use crate::store;

/// @id: miyusocialprofile_tool_follow_add
/// @role: mutator
/// @layer: tool
/// @human: Ajoute un abonnement (follow) ; WriteIntent KindMother.
/// @do: follow_add_under_governance
/// tool.social.follow.add
pub fn add(ctx: &GovernedContext, user_id: &str, target_id: &str) -> Result<(), MiyusocialprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialprofileError::NoMandate);
    }
    store::following().lock().map_err(|_| MiyusocialprofileError::InvalidInput("lock".into()))?.entry(user_id.to_string()).or_default().push(target_id.to_string());
    store::followers().lock().map_err(|_| MiyusocialprofileError::InvalidInput("lock".into()))?.entry(target_id.to_string()).or_default().push(user_id.to_string());
    Ok(())
}

/// @id: miyusocialprofile_tool_follow_remove
/// @role: mutator
/// @layer: tool
/// @human: Supprime un abonnement ; WriteIntent KindMother.
/// @do: follow_remove_under_governance
/// tool.social.follow.remove
pub fn remove(ctx: &GovernedContext, user_id: &str, target_id: &str) -> Result<(), MiyusocialprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialprofileError::NoMandate);
    }
    if let Some(v) = store::following().lock().map_err(|_| MiyusocialprofileError::InvalidInput("lock".into()))?.get_mut(user_id) {
        v.retain(|id| id != target_id);
    }
    if let Some(v) = store::followers().lock().map_err(|_| MiyusocialprofileError::InvalidInput("lock".into()))?.get_mut(target_id) {
        v.retain(|id| id != user_id);
    }
    Ok(())
}

/// @id: miyusocialprofile_tool_followers_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les abonnés.
/// @do: followers_list_under_governance
/// tool.social.followers.list
pub fn followers_list(ctx: &GovernedContext, user_id: &str) -> Result<Vec<String>, MiyusocialprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialprofileError::NoMandate);
    }
    let guard = store::followers().lock().map_err(|_| MiyusocialprofileError::InvalidInput("lock".into()))?;
    Ok(guard.get(user_id).cloned().unwrap_or_default())
}

/// @id: miyusocialprofile_tool_following_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les abonnements.
/// @do: following_list_under_governance
/// tool.social.following.list
pub fn following_list(ctx: &GovernedContext, user_id: &str) -> Result<Vec<String>, MiyusocialprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialprofileError::NoMandate);
    }
    let guard = store::following().lock().map_err(|_| MiyusocialprofileError::InvalidInput("lock".into()))?;
    Ok(guard.get(user_id).cloned().unwrap_or_default())
}
