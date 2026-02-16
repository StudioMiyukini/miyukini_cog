//! Tools MiyuProfile — tool.profile.signature.get, set.
//! Stocké dans le champ "signature" du profil.

use crate::context::GovernedContext;
use crate::errors::MiyuprofileError;
use crate::profile;
use std::collections::HashMap;

/// @id: miyuprofile_tool_signature_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère la signature.
/// @do: signature_get_under_governance
/// tool.profile.signature.get
pub fn get(ctx: &GovernedContext, user_id: &str) -> Result<String, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let p = profile::get(ctx, user_id)?;
    Ok(p.fields.get("signature").cloned().unwrap_or_default())
}

/// @id: miyuprofile_tool_signature_set
/// @role: mutator
/// @layer: tool
/// @human: Met à jour la signature ; WriteIntent KindMother.
/// @do: signature_set_under_governance
/// tool.profile.signature.set
pub fn set(ctx: &GovernedContext, user_id: &str, content: &str) -> Result<(), MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let mut data = HashMap::new();
    data.insert("signature".to_string(), content.to_string());
    profile::update(ctx, user_id, &data)
}
