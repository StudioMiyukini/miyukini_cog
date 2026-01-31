//! Tools MiyuCMS — tool.media.* (upload, serve, transform).
//! Médias : upload = WriteIntent KindMother ; serve/transform = données fournies dans le flux.

use crate::context::GovernedContext;
use crate::errors::MiyucmsError;

/// @id: miyucms_tool_media_upload
/// @role: mutator
/// @layer: tool
/// @human: Enregistre un média ; persistance KindMother.
/// @do: cms_media_upload_under_governance
pub fn upload(ctx: &GovernedContext, _payload: &str, _blob: Option<&[u8]>) -> Result<String, MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    Err(MiyucmsError::Unimplemented)
}

/// @id: miyucms_tool_media_serve
/// @role: accessor
/// @layer: tool
/// @human: Sert un média (stream ou métadonnées) ; données fournies dans le flux.
/// @do: cms_media_serve_under_governance
pub fn serve(ctx: &GovernedContext, _media_ref: &str, _mode: Option<&str>) -> Result<Vec<u8>, MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    Err(MiyucmsError::Unimplemented)
}

/// @id: miyucms_tool_media_transform
/// @role: accessor
/// @layer: tool
/// @human: Produit une variante (miniature, recadrage) ; données fournies ; pas de persistance directe.
/// @do: cms_media_transform_under_governance
pub fn transform(ctx: &GovernedContext, _media_ref: &str, _options: &str) -> Result<Vec<u8>, MiyucmsError> {
    if !ctx.has_mandate() {
        return Err(MiyucmsError::NoMandate);
    }
    Err(MiyucmsError::Unimplemented)
}
