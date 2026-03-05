//! Tools MiyuProfile — tool.profile.preferences.get, set.
//! Même store que profile (préférences = champs profil).

use crate::context::GovernedContext;
use crate::errors::MiyuprofileError;
use crate::profile;
use std::collections::HashMap;

/// @id: miyuprofile_tool_preferences_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère les préférences.
/// @do: preferences_get_under_governance
/// tool.profile.preferences.get
pub fn get(
    ctx: &GovernedContext,
    user_id: &str,
) -> Result<HashMap<String, String>, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let p = profile::get(ctx, user_id)?;
    Ok(p.fields)
}

/// @id: miyuprofile_tool_preferences_set
/// @role: mutator
/// @layer: tool
/// @human: Met à jour les préférences ; WriteIntent KindMother.
/// @do: preferences_set_under_governance
/// tool.profile.preferences.set
pub fn set(
    ctx: &GovernedContext,
    user_id: &str,
    prefs: &HashMap<String, String>,
) -> Result<(), MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    profile::update(ctx, user_id, prefs)
}
