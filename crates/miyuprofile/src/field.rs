//! Tools MiyuProfile — tool.profile.field.list, get, set.

use crate::context::GovernedContext;
use crate::errors::MiyuprofileError;

/// @id: miyuprofile_tool_field_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les champs (schéma).
/// @do: field_list_under_governance
/// tool.profile.field.list
pub fn list(ctx: &GovernedContext) -> Result<Vec<String>, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    Err(MiyuprofileError::Unimplemented)
}

/// @id: miyuprofile_tool_field_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère un champ.
/// @do: field_get_under_governance
/// tool.profile.field.get
pub fn get(
    ctx: &GovernedContext,
    _user_id: &str,
    _field_name: &str,
) -> Result<String, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    Err(MiyuprofileError::Unimplemented)
}

/// @id: miyuprofile_tool_field_set
/// @role: mutator
/// @layer: tool
/// @human: Met à jour un champ ; WriteIntent KindMother.
/// @do: field_set_under_governance
/// tool.profile.field.set
pub fn set(
    ctx: &GovernedContext,
    _user_id: &str,
    _field_name: &str,
    _value: &str,
) -> Result<(), MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    Err(MiyuprofileError::Unimplemented)
}
