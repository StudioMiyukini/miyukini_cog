//! Tools MiyuWidgets — tool.web.widget.*.render (text, image, button, grid, container).
//! Rendu de blocs widget ; données fournies dans le flux.

use crate::context::GovernedContext;
use crate::errors::MiyuwidgetsError;

/// @id miyuwidgets_tool_web_widget_text_render
/// @role accessor
/// @layer tool
/// @do web_widget_text_render_under_governance
//@human / @human Rend un bloc texte ; données fournies dans le flux.
pub fn text_render(ctx: &GovernedContext, payload: &str) -> Result<String, MiyuwidgetsError> {
    if !ctx.has_mandate() {
        return Err(MiyuwidgetsError::NoMandate);
    }
    Ok(format!("<span>{}</span>", escape(payload)))
}

/// @id miyuwidgets_tool_web_widget_image_render
/// @role accessor
/// @layer tool
/// @do web_widget_image_render_under_governance
//@human / @human Rend un bloc image ; données fournies dans le flux.
pub fn image_render(ctx: &GovernedContext, payload: &str) -> Result<String, MiyuwidgetsError> {
    if !ctx.has_mandate() {
        return Err(MiyuwidgetsError::NoMandate);
    }
    Ok(format!("<img data-payload=\"{}\"/>", escape(payload)))
}

/// @id miyuwidgets_tool_web_widget_button_render
/// @role accessor
/// @layer tool
/// @do web_widget_button_render_under_governance
//@human / @human Rend un bloc bouton ; données fournies dans le flux.
pub fn button_render(ctx: &GovernedContext, payload: &str) -> Result<String, MiyuwidgetsError> {
    if !ctx.has_mandate() {
        return Err(MiyuwidgetsError::NoMandate);
    }
    Ok(format!("<button>{}</button>", escape(payload)))
}

/// @id miyuwidgets_tool_web_widget_grid_render
/// @role accessor
/// @layer tool
/// @do web_widget_grid_render_under_governance
//@human / @human Rend une grille de blocs ; données fournies dans le flux.
pub fn grid_render(ctx: &GovernedContext, payload: &str) -> Result<String, MiyuwidgetsError> {
    if !ctx.has_mandate() {
        return Err(MiyuwidgetsError::NoMandate);
    }
    Ok(format!(
        "<div class=\"grid\" data-payload=\"{}\"></div>",
        escape(payload)
    ))
}

/// @id miyuwidgets_tool_web_widget_container_render
/// @role accessor
/// @layer tool
/// @do web_widget_container_render_under_governance
//@human / @human Rend un conteneur (section/colonnes) ; données fournies dans le flux.
pub fn container_render(ctx: &GovernedContext, payload: &str) -> Result<String, MiyuwidgetsError> {
    if !ctx.has_mandate() {
        return Err(MiyuwidgetsError::NoMandate);
    }
    Ok(format!(
        "<div class=\"container\" data-payload=\"{}\"></div>",
        escape(payload)
    ))
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
