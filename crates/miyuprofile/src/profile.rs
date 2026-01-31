//! Tools MiyuProfile — tool.profile.get, tool.profile.update.
//! Décision modification = StrongFather ; WriteIntent KindMother.

use crate::context::GovernedContext;
use crate::errors::MiyuprofileError;
use std::collections::HashMap;

/// @id: miyuprofile_tool_profile_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère le profil.
/// @do: profile_get_under_governance
/// tool.profile.get
pub fn get(ctx: &GovernedContext, _user_id: &str) -> Result<ProfileData, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    Err(MiyuprofileError::Unimplemented)
}

/// @id: miyuprofile_tool_profile_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour le profil ; WriteIntent KindMother.
/// @do: profile_update_under_governance
/// tool.profile.update
pub fn update(
    ctx: &GovernedContext,
    _user_id: &str,
    _data: &HashMap<String, String>,
) -> Result<(), MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    Err(MiyuprofileError::Unimplemented)
}

/// Données profil.
#[derive(Debug, Clone, Default)]
pub struct ProfileData {
    pub user_id: String,
    pub fields: HashMap<String, String>,
}
