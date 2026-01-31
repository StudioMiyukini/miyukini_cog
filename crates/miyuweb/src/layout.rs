//! Tool MiyuWeb — tool.web.layout.render.
//! Rendu layout (structure de page) à partir de données fournies.

use crate::context::GovernedContext;
use crate::errors::MiyuwebError;

/// @id: miyuweb_tool_web_layout_render
/// @role: mutator
/// @layer: tool
/// @human: Rend un layout (structure de page) à partir de données fournies.
/// @do: web_layout_render_under_governance
/// tool.web.layout.render — ne décide pas du contenu des zones.
pub fn render(ctx: &GovernedContext, _data: &str) -> Result<String, MiyuwebError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebError::NoMandate);
    }
    Ok(String::new())
}
