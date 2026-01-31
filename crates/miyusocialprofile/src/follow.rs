//! Tools MiyuSocialProfile — tool.social.follow.add, remove, followers.list, following.list.

use crate::context::GovernedContext;
use crate::errors::MiyusocialprofileError;

/// @id: miyusocialprofile_tool_follow_add
/// @role: mutator
/// @layer: tool
/// @human: Ajoute un abonnement (follow) ; WriteIntent KindMother.
/// @do: follow_add_under_governance
/// tool.social.follow.add
pub fn add(ctx: &GovernedContext, _user_id: &str, _target_id: &str) -> Result<(), MiyusocialprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialprofileError::NoMandate);
    }
    Err(MiyusocialprofileError::Unimplemented)
}

/// @id: miyusocialprofile_tool_follow_remove
/// @role: mutator
/// @layer: tool
/// @human: Supprime un abonnement ; WriteIntent KindMother.
/// @do: follow_remove_under_governance
/// tool.social.follow.remove
pub fn remove(ctx: &GovernedContext, _user_id: &str, _target_id: &str) -> Result<(), MiyusocialprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialprofileError::NoMandate);
    }
    Err(MiyusocialprofileError::Unimplemented)
}

/// @id: miyusocialprofile_tool_followers_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les abonnés.
/// @do: followers_list_under_governance
/// tool.social.followers.list
pub fn followers_list(ctx: &GovernedContext, _user_id: &str) -> Result<Vec<String>, MiyusocialprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialprofileError::NoMandate);
    }
    Err(MiyusocialprofileError::Unimplemented)
}

/// @id: miyusocialprofile_tool_following_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les abonnements.
/// @do: following_list_under_governance
/// tool.social.following.list
pub fn following_list(ctx: &GovernedContext, _user_id: &str) -> Result<Vec<String>, MiyusocialprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialprofileError::NoMandate);
    }
    Err(MiyusocialprofileError::Unimplemented)
}
