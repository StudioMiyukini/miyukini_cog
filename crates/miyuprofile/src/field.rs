//! Tools MiyuProfile — tool.profile.field.list, get, set.
//! Utilise le même store que profile (champs = ProfileData.fields).

use crate::context::GovernedContext;
use crate::errors::MiyuprofileError;
use crate::profile;

/// @id: miyuprofile_tool_field_list
/// @role: mutator
/// @layer: tool
/// @human: Liste les noms de champs du profil (utilisateur = mandate).
/// @do: field_list_under_governance
/// tool.profile.field.list
pub fn list(ctx: &GovernedContext) -> Result<Vec<String>, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let p = profile::get(ctx, &ctx.mandate_id)?;
    let mut names: Vec<String> = p.fields.keys().cloned().collect();
    names.sort();
    Ok(names)
}

/// @id: miyuprofile_tool_field_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère un champ.
/// @do: field_get_under_governance
/// tool.profile.field.get
pub fn get(
    ctx: &GovernedContext,
    user_id: &str,
    field_name: &str,
) -> Result<String, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let p = profile::get(ctx, user_id)?;
    Ok(p.fields
        .get(field_name)
        .cloned()
        .unwrap_or_default())
}

/// @id: miyuprofile_tool_field_set
/// @role: mutator
/// @layer: tool
/// @human: Met à jour un champ ; WriteIntent KindMother.
/// @do: field_set_under_governance
/// tool.profile.field.set
pub fn set(
    ctx: &GovernedContext,
    user_id: &str,
    field_name: &str,
    value: &str,
) -> Result<(), MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let mut data = std::collections::HashMap::new();
    data.insert(field_name.to_string(), value.to_string());
    profile::update(ctx, user_id, &data)
}
