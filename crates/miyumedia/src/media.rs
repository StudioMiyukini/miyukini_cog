//! Tools MiyuMedia — tool.media.upload, tool.media.serve, tool.media.transform.
//! Médias : upload = WriteIntent KindMother ; serve/transform = données fournies dans le flux.

use crate::context::GovernedContext;
use crate::errors::MiyumediaError;

/// @id: miyumedia_tool_media_upload
/// @role: mutator
/// @layer: tool
/// @human: Enregistre un média ; à partir du flux ; persistance KindMother.
/// @do: media_upload_under_governance
pub fn upload(
    ctx: &GovernedContext,
    _payload: &str,
    _blob: Option<&[u8]>,
) -> Result<String, MiyumediaError> {
    if !ctx.has_mandate() {
        return Err(MiyumediaError::NoMandate);
    }
    Ok(format!("med:{}", _payload.len()))
}

/// @id: miyumedia_tool_media_serve
/// @role: accessor
/// @layer: tool
/// @human: Sert un média (stream ou métadonnées) ; données fournies dans le flux.
/// @do: media_serve_under_governance
pub fn serve(
    ctx: &GovernedContext,
    _media_ref: &str,
    _mode: Option<&str>,
) -> Result<Vec<u8>, MiyumediaError> {
    if !ctx.has_mandate() {
        return Err(MiyumediaError::NoMandate);
    }
    Ok(Vec::new())
}

/// @id: miyumedia_tool_media_transform
/// @role: accessor
/// @layer: tool
/// @human: Produit une variante (miniature, recadrage) ; données fournies ; pas de décision politique.
/// @do: media_transform_under_governance
pub fn transform(
    ctx: &GovernedContext,
    _media_ref: &str,
    _options: &str,
) -> Result<Vec<u8>, MiyumediaError> {
    if !ctx.has_mandate() {
        return Err(MiyumediaError::NoMandate);
    }
    Ok(Vec::new())
}
