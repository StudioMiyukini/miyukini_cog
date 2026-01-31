//! Tools MiyuSocialProfile — tool.social.profile.get, update.

use crate::context::GovernedContext;
use crate::errors::MiyusocialprofileError;
use std::collections::HashMap;

/// @id: miyusocialprofile_tool_profile_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère le profil social.
/// @do: social_profile_get_under_governance
/// tool.social.profile.get
pub fn get(ctx: &GovernedContext, _user_id: &str) -> Result<HashMap<String, String>, MiyusocialprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialprofileError::NoMandate);
    }
    Err(MiyusocialprofileError::Unimplemented)
}

/// @id: miyusocialprofile_tool_profile_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour le profil social ; WriteIntent KindMother.
/// @do: social_profile_update_under_governance
/// tool.social.profile.update
pub fn update(
    ctx: &GovernedContext,
    _user_id: &str,
    _data: &HashMap<String, String>,
) -> Result<(), MiyusocialprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialprofileError::NoMandate);
    }
    Err(MiyusocialprofileError::Unimplemented)
}
