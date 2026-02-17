//! Tools MiyuSocialProfile — tool.social.profile.get, update.

use crate::context::GovernedContext;
use crate::errors::MiyusocialprofileError;
use crate::store;
use std::collections::HashMap;

/// @id: miyusocialprofile_tool_profile_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère le profil social.
/// @do: social_profile_get_under_governance
/// tool.social.profile.get
pub fn get(ctx: &GovernedContext, user_id: &str) -> Result<HashMap<String, String>, MiyusocialprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialprofileError::NoMandate);
    }
    let guard = store::profiles().lock().map_err(|_| MiyusocialprofileError::InvalidInput("lock".into()))?;
    Ok(guard.get(user_id).cloned().unwrap_or_default())
}

/// @id: miyusocialprofile_tool_profile_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour le profil social ; WriteIntent KindMother.
/// @do: social_profile_update_under_governance
/// tool.social.profile.update
pub fn update(
    ctx: &GovernedContext,
    user_id: &str,
    data: &HashMap<String, String>,
) -> Result<(), MiyusocialprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyusocialprofileError::NoMandate);
    }
    let mut guard = store::profiles().lock().map_err(|_| MiyusocialprofileError::InvalidInput("lock".into()))?;
    let entry = guard.entry(user_id.to_string()).or_default();
    for (k, v) in data {
        entry.insert(k.clone(), v.clone());
    }
    Ok(())
}
