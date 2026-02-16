//! Tools MiyuProfile — tool.profile.get, tool.profile.update.
//! Décision modification = StrongFather ; WriteIntent KindMother.
//! Store par défaut en mémoire (persistance réelle déléguée à KindMother côté produit).

use crate::context::GovernedContext;
use crate::errors::MiyuprofileError;
use std::collections::HashMap;
use std::sync::Mutex;

/// Données profil.
#[derive(Debug, Clone, Default)]
pub struct ProfileData {
    pub user_id: String,
    pub fields: HashMap<String, String>,
}

static DEFAULT_STORE: std::sync::OnceLock<Mutex<HashMap<String, ProfileData>>> =
    std::sync::OnceLock::new();

fn default_store() -> &'static Mutex<HashMap<String, ProfileData>> {
    DEFAULT_STORE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// @id: miyuprofile_tool_profile_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère le profil.
/// @do: profile_get_under_governance
/// tool.profile.get
pub fn get(ctx: &GovernedContext, user_id: &str) -> Result<ProfileData, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let store = default_store();
    let guard = store
        .lock()
        .map_err(|_| MiyuprofileError::InvalidInput("store lock".into()))?;
    let profile = guard.get(user_id).cloned().unwrap_or_else(|| ProfileData {
        user_id: user_id.to_string(),
        fields: HashMap::new(),
    });
    Ok(profile)
}

/// @id: miyuprofile_tool_profile_update
/// @role: mutator
/// @layer: tool
/// @human: Met à jour le profil ; WriteIntent KindMother.
/// @do: profile_update_under_governance
/// tool.profile.update
pub fn update(
    ctx: &GovernedContext,
    user_id: &str,
    data: &HashMap<String, String>,
) -> Result<(), MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let store = default_store();
    let mut guard = store
        .lock()
        .map_err(|_| MiyuprofileError::InvalidInput("store lock".into()))?;
    let entry = guard
        .entry(user_id.to_string())
        .or_insert_with(|| ProfileData {
            user_id: user_id.to_string(),
            fields: HashMap::new(),
        });
    entry.fields.extend(data.iter().map(|(k, v)| (k.clone(), v.clone())));
    Ok(())
}
