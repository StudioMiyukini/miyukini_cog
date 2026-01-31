//! Tool MiyuWeb — tool.web.html.render.
//! Rendu HTML à partir de données et template fournis ; ne décide pas du contenu.

use crate::context::GovernedContext;
use crate::errors::MiyuwebError;

/// @id: miyuweb_tool_web_html_render
/// @role: mutator
/// @layer: tool
/// @human: Rend du HTML à partir de données et template fournis dans le flux.
/// @do: web_html_render_under_governance
/// tool.web.html.render — ne décide pas du contenu.
pub fn render(
    ctx: &GovernedContext,
    _template: &str,
    _data: &str,
) -> Result<String, MiyuwebError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebError::NoMandate);
    }
    Err(MiyuwebError::Unimplemented)
}
