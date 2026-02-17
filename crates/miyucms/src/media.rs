//! Tools MiyuCMS — tool.media.* (upload, serve, transform).
//! Médias : upload = WriteIntent KindMother ; serve/transform = données fournies dans le flux.

use crate::context::GovernedContext;
use crate::errors::MiyucmsError;
use crate::store;
use miyukini_kernel::{IdGenerator as _, UuidIdGenerator};

/// @id: miyucms_tool_media_upload
/// @role: mutator
/// @layer: tool
/// @human: Enregistre un média ; persistance KindMother.
/// @do: cms_media_upload_under_governance
pub fn upload(ctx: &GovernedContext, payload: &str, blob: Option<&[u8]>) -> Result<String, MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    let id = format!("med:{}", UuidIdGenerator.generate());
    let data = blob.unwrap_or_default().to_vec();
    let mut guard = store::media().lock().map_err(|_| MiyucmsError::InvalidInput("lock".into()))?;
    guard.insert(id.clone(), (payload.to_string(), data));
    Ok(id)
}

/// @id: miyucms_tool_media_serve
/// @role: accessor
/// @layer: tool
/// @human: Sert un média (stream ou métadonnées) ; données fournies dans le flux.
/// @do: cms_media_serve_under_governance
pub fn serve(ctx: &GovernedContext, media_ref: &str, _mode: Option<&str>) -> Result<Vec<u8>, MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    let guard = store::media().lock().map_err(|_| MiyucmsError::InvalidInput("lock".into()))?;
    let (_, blob) = guard.get(media_ref).ok_or_else(|| MiyucmsError::InvalidInput("media not found".into()))?;
    Ok(blob.clone())
}

/// @id: miyucms_tool_media_transform
/// @role: accessor
/// @layer: tool
/// @human: Produit une variante (miniature, recadrage) ; données fournies ; pas de persistance directe.
/// @do: cms_media_transform_under_governance
pub fn transform(ctx: &GovernedContext, media_ref: &str, _options: &str) -> Result<Vec<u8>, MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    let guard = store::media().lock().map_err(|_| MiyucmsError::InvalidInput("lock".into()))?;
    let (_, blob) = guard.get(media_ref).ok_or_else(|| MiyucmsError::InvalidInput("media not found".into()))?;
    Ok(blob.clone())
}
