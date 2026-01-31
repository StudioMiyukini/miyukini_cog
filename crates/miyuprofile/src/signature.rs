//! Tools MiyuProfile — tool.profile.signature.get, set.

use crate::context::GovernedContext;
use crate::errors::MiyuprofileError;

/// @id: miyuprofile_tool_signature_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère la signature.
/// @do: signature_get_under_governance
/// tool.profile.signature.get
pub fn get(ctx: &GovernedContext, _user_id: &str) -> Result<String, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    Err(MiyuprofileError::Unimplemented)
}

/// @id: miyuprofile_tool_signature_set
/// @role: mutator
/// @layer: tool
/// @human: Met à jour la signature ; WriteIntent KindMother.
/// @do: signature_set_under_governance
/// tool.profile.signature.set
pub fn set(
    ctx: &GovernedContext,
    _user_id: &str,
    _content: &str,
) -> Result<(), MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    Err(MiyuprofileError::Unimplemented)
}
