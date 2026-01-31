//! Tools MiyuProfile — tool.profile.preferences.get, set.

use crate::context::GovernedContext;
use crate::errors::MiyuprofileError;
use std::collections::HashMap;

/// @id: miyuprofile_tool_preferences_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère les préférences.
/// @do: preferences_get_under_governance
/// tool.profile.preferences.get
pub fn get(ctx: &GovernedContext, _user_id: &str) -> Result<HashMap<String, String>, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    Err(MiyuprofileError::Unimplemented)
}

/// @id: miyuprofile_tool_preferences_set
/// @role: mutator
/// @layer: tool
/// @human: Met à jour les préférences ; WriteIntent KindMother.
/// @do: preferences_set_under_governance
/// tool.profile.preferences.set
pub fn set(
    ctx: &GovernedContext,
    _user_id: &str,
    _prefs: &HashMap<String, String>,
) -> Result<(), MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    Err(MiyuprofileError::Unimplemented)
}
