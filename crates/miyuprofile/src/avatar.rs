//! Tools MiyuProfile — tool.profile.avatar.get, set, resolve.
//! Store binaire en mémoire (user_id -> bytes).

use crate::context::GovernedContext;
use crate::errors::MiyuprofileError;
use std::collections::HashMap;
use std::sync::Mutex;

static AVATARS: std::sync::OnceLock<Mutex<HashMap<String, Vec<u8>>>> = std::sync::OnceLock::new();

fn avatars() -> &'static Mutex<HashMap<String, Vec<u8>>> {
    AVATARS.get_or_init(|| Mutex::new(HashMap::new()))
}

/// @id: miyuprofile_tool_avatar_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère l'avatar.
/// @do: avatar_get_under_governance
/// tool.profile.avatar.get
pub fn get(ctx: &GovernedContext, user_id: &str) -> Result<Vec<u8>, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let guard = avatars()
        .lock()
        .map_err(|_| MiyuprofileError::InvalidInput("lock".into()))?;
    Ok(guard.get(user_id).cloned().unwrap_or_default())
}

/// @id: miyuprofile_tool_avatar_set
/// @role: mutator
/// @layer: tool
/// @human: Met à jour l'avatar ; WriteIntent KindMother ou MiyuMedia.
/// @do: avatar_set_under_governance
/// tool.profile.avatar.set
pub fn set(ctx: &GovernedContext, user_id: &str, payload: &[u8]) -> Result<(), MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let mut guard = avatars()
        .lock()
        .map_err(|_| MiyuprofileError::InvalidInput("lock".into()))?;
    guard.insert(user_id.to_string(), payload.to_vec());
    Ok(())
}

/// @id: miyuprofile_tool_avatar_resolve
/// @role: mutator
/// @layer: tool
/// @human: Résout avatar (ex. Gravatar) ; exécution seule.
/// @do: avatar_resolve_under_governance
/// tool.profile.avatar.resolve
pub fn resolve(ctx: &GovernedContext, user_id: &str) -> Result<Option<String>, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let guard = avatars()
        .lock()
        .map_err(|_| MiyuprofileError::InvalidInput("lock".into()))?;
    Ok(guard.get(user_id).map(|_| format!("avatar:{user_id}")))
}
