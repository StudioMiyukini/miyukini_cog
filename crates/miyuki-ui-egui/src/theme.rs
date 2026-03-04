// @id: MUIE-Theme @do: theme-application @role: bridge @layer: 6 @human: miyuk

//! Theme application for egui contexts.
//!
//! Provides the high-level [`apply_theme`] function that configures an
//! [`egui::Context`] with a full Miyukini theme (COG or D2).

use egui::Context;
use miyuki_ui_tokens::UiTheme;

use crate::convert;

/// Apply a complete [`UiTheme`] to an egui [`Context`].
///
/// This sets the global style (colors, spacing, rounding) and font definitions.
/// Call this once after creating the context, or whenever the theme changes.
///
/// # Example
///
/// ```rust,no_run
/// use miyuki_ui_tokens::D2_THEME;
/// use miyuki_ui_egui::theme::apply_theme;
///
/// // let ctx = egui::Context::default();
/// // apply_theme(&ctx, &D2_THEME);
/// ```
pub fn apply_theme(ctx: &Context, theme: &UiTheme) {
    let style = convert::theme_to_style(theme);
    ctx.set_style(style);

    // Font definitions -- use egui defaults for now.
    // In production, load custom TTF fonts (Cinzel for D2, Inter for COG).
    let fonts = egui::FontDefinitions::default();
    ctx.set_fonts(fonts);
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyuki_ui_tokens::{COG_THEME, D2_THEME};

    #[test]
    fn test_apply_theme_d2_does_not_panic() {
        let ctx = Context::default();
        apply_theme(&ctx, &D2_THEME);
    }

    #[test]
    fn test_apply_theme_cog_does_not_panic() {
        let ctx = Context::default();
        apply_theme(&ctx, &COG_THEME);
    }

    #[test]
    fn test_apply_theme_sets_dark_mode() {
        let ctx = Context::default();
        apply_theme(&ctx, &D2_THEME);
        let style = ctx.style();
        assert!(style.visuals.dark_mode);
    }
}
