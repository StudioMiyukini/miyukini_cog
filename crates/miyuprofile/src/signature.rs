//! Tools MiyuProfile — tool.profile.signature.get, set, handwritten.
//! Stocké dans le champ "signature" du profil (texte) + store binaire (manuscrite).

use crate::context::GovernedContext;
use crate::errors::MiyuprofileError;
use crate::profile;
use std::collections::HashMap;
use std::sync::Mutex;

// ── Signature texte (rétro-compatible) ───────────────────────────

/// @id: miyuprofile_tool_signature_get
/// @role: mutator
/// @layer: tool
/// @human: Récupère la signature texte.
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
/// @human: Met à jour la signature texte ; WriteIntent KindMother.
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

// ── Signature manuscrite (binaire : PNG/SVG) ─────────────────────

static HANDWRITTEN_STORE: std::sync::OnceLock<Mutex<HashMap<String, HandwrittenSignature>>> =
    std::sync::OnceLock::new();

fn handwritten_store() -> &'static Mutex<HashMap<String, HandwrittenSignature>> {
    HANDWRITTEN_STORE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// Signature manuscrite stockée.
#[derive(Debug, Clone)]
pub struct HandwrittenSignature {
    /// Données binaires (PNG, SVG, ou autre format image).
    pub data: Vec<u8>,
    /// Type MIME (image/png, image/svg+xml…).
    pub mime_type: String,
}

/// tool.profile.signature.handwritten.get — Récupère la signature manuscrite.
pub fn handwritten_get(
    ctx: &GovernedContext,
    user_id: &str,
) -> Result<Option<HandwrittenSignature>, MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let guard = handwritten_store()
        .lock()
        .map_err(|_| MiyuprofileError::InvalidInput("handwritten lock".into()))?;
    Ok(guard.get(user_id).cloned())
}

/// tool.profile.signature.handwritten.set — Stocke la signature manuscrite.
pub fn handwritten_set(
    ctx: &GovernedContext,
    user_id: &str,
    data: &[u8],
    mime_type: &str,
) -> Result<(), MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let mut guard = handwritten_store()
        .lock()
        .map_err(|_| MiyuprofileError::InvalidInput("handwritten lock".into()))?;
    guard.insert(
        user_id.to_string(),
        HandwrittenSignature {
            data: data.to_vec(),
            mime_type: mime_type.to_string(),
        },
    );
    Ok(())
}

/// tool.profile.signature.handwritten.delete — Supprime la signature manuscrite.
pub fn handwritten_delete(
    ctx: &GovernedContext,
    user_id: &str,
) -> Result<(), MiyuprofileError> {
    if !ctx.has_mandate() {
        return Err(MiyuprofileError::NoMandate);
    }
    let mut guard = handwritten_store()
        .lock()
        .map_err(|_| MiyuprofileError::InvalidInput("handwritten lock".into()))?;
    guard.remove(user_id);
    Ok(())
}
