//! Tool MiyuWeb — tool.web.asset.serve.
//! Service d'un asset (image, CSS, etc.) à partir de données fournies dans le flux.

use crate::context::GovernedContext;
use crate::errors::MiyuwebError;

/// @id: miyuweb_tool_web_asset_serve
/// @role: accessor
/// @layer: tool
/// @human: Sert un asset (image, CSS, binaire) à partir de données fournies dans le flux.
/// @do: web_asset_serve_under_governance
/// tool.web.asset.serve — ne lit pas la base ; données fournies en entrée.
pub fn serve(
    ctx: &GovernedContext,
    _asset_ref: &str,
    _metadata: Option<&str>,
) -> Result<Vec<u8>, MiyuwebError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebError::NoMandate);
    }
    Err(MiyuwebError::Unimplemented)
}
