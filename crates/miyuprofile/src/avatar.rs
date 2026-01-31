//! Tools MiyuProfile — tool.profile.avatar.get, set, resolve.

use crate::context::GovernedContext;
use crate::errors::MiyuprofileError;

/// @id: miyuprofile_tool_avatar_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère l'avatar.
/// @do: avatar_get_under_governance
/// tool.profile.avatar.get
pub fn get(ctx: &GovernedContext, _user_id: &str) -> Result<Vec<u8>, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    Err(MiyuprofileError::Unimplemented)
}

/// @id: miyuprofile_tool_avatar_set
/// @role: mutator
/// @layer: tool
/// @human: Met à jour l'avatar ; WriteIntent KindMother ou MiyuMedia.
/// @do: avatar_set_under_governance
/// tool.profile.avatar.set
pub fn set(ctx: &GovernedContext, _user_id: &str, _payload: &[u8]) -> Result<(), MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    Err(MiyuprofileError::Unimplemented)
}

/// @id: miyuprofile_tool_avatar_resolve
/// @role: mutator
/// @layer: tool
/// @human: Résout avatar (ex. Gravatar) ; exécution seule.
/// @do: avatar_resolve_under_governance
/// tool.profile.avatar.resolve
pub fn resolve(ctx: &GovernedContext, _user_id: &str) -> Result<Option<String>, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    Err(MiyuprofileError::Unimplemented)
}
