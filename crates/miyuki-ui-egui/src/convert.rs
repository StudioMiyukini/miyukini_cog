// @id: MUIE-Convert @do: token-conversion @role: bridge @layer: 6 @human: miyuk

//! Conversion from [`miyuki_ui_tokens`] types to egui types.
//!
//! This module is the bridge between the agnostic design tokens and the
//! concrete egui style system. All egui visual configuration should flow
//! through these functions rather than hard-coding colors or sizes.

use egui::{style::Spacing, vec2, Color32, Margin, Rounding, Shadow, Stroke, Style, Visuals};
use miyuki_ui_tokens::{Rgba, UiTheme};

/// Convert a token [`Rgba`] to an egui [`Color32`].
///
/// The conversion is a direct channel mapping (no premultiplication).
pub const fn rgba_to_color32(c: &Rgba) -> Color32 {
    Color32::from_rgba_premultiplied(c.r, c.g, c.b, c.a)
}

/// Build a complete egui [`Style`] from a [`UiTheme`].
///
/// Configures visuals (colors, rounding, strokes), spacing, and text styles
/// to match the given theme. Call [`egui::Context::set_style`] with the result.
pub fn theme_to_style(theme: &UiTheme) -> Style {
    let p = &theme.palette;

    let visuals = Visuals {
        dark_mode: true,

        // Panel and window backgrounds
        panel_fill: rgba_to_color32(&p.bg_secondary),
        window_fill: rgba_to_color32(&p.bg_secondary),
        window_stroke: Stroke::new(2.0, rgba_to_color32(&p.border_default)),
        window_rounding: Rounding::same(theme.radius.sm),
        window_shadow: Shadow {
            offset: vec2(theme.shadow.md.offset_x, theme.shadow.md.offset_y),
            blur: theme.shadow.md.blur,
            spread: theme.shadow.md.spread,
            color: rgba_to_color32(&theme.shadow.md.color),
        },

        // Override text color
        override_text_color: Some(rgba_to_color32(&p.text_primary)),

        // Widget visuals
        widgets: egui::style::Widgets {
            noninteractive: egui::style::WidgetVisuals {
                bg_fill: rgba_to_color32(&p.bg_surface),
                weak_bg_fill: rgba_to_color32(&p.bg_primary),
                bg_stroke: Stroke::new(1.0, rgba_to_color32(&p.border_subtle)),
                rounding: Rounding::same(theme.radius.sm),
                fg_stroke: Stroke::new(1.0, rgba_to_color32(&p.text_primary)),
                expansion: 0.0,
            },
            inactive: egui::style::WidgetVisuals {
                bg_fill: rgba_to_color32(&p.bg_surface),
                weak_bg_fill: rgba_to_color32(&p.bg_surface),
                bg_stroke: Stroke::new(1.0, rgba_to_color32(&p.border_default)),
                rounding: Rounding::same(theme.radius.sm),
                fg_stroke: Stroke::new(1.0, rgba_to_color32(&p.text_primary)),
                expansion: 0.0,
            },
            hovered: egui::style::WidgetVisuals {
                bg_fill: rgba_to_color32(&p.bg_elevated),
                weak_bg_fill: rgba_to_color32(&p.bg_elevated),
                bg_stroke: Stroke::new(1.0, rgba_to_color32(&p.border_strong)),
                rounding: Rounding::same(theme.radius.sm),
                fg_stroke: Stroke::new(1.5, rgba_to_color32(&p.accent_primary_hover)),
                expansion: 1.0,
            },
            active: egui::style::WidgetVisuals {
                bg_fill: rgba_to_color32(&p.accent_primary_active),
                weak_bg_fill: rgba_to_color32(&p.accent_primary_active),
                bg_stroke: Stroke::new(1.0, rgba_to_color32(&p.border_accent)),
                rounding: Rounding::same(theme.radius.sm),
                fg_stroke: Stroke::new(2.0, rgba_to_color32(&p.text_high)),
                expansion: 1.0,
            },
            open: egui::style::WidgetVisuals {
                bg_fill: rgba_to_color32(&p.bg_elevated),
                weak_bg_fill: rgba_to_color32(&p.bg_elevated),
                bg_stroke: Stroke::new(1.0, rgba_to_color32(&p.border_strong)),
                rounding: Rounding::same(theme.radius.sm),
                fg_stroke: Stroke::new(1.0, rgba_to_color32(&p.text_high)),
                expansion: 0.0,
            },
        },

        // Selection
        selection: egui::style::Selection {
            bg_fill: rgba_to_color32(&p.accent_primary_subtle),
            stroke: Stroke::new(1.0, rgba_to_color32(&p.accent_primary)),
        },

        // Hyperlinks
        hyperlink_color: rgba_to_color32(&p.accent_primary),

        // Faint bg color
        faint_bg_color: rgba_to_color32(&p.bg_primary),

        // Extreme bg color
        extreme_bg_color: rgba_to_color32(&p.bg_base),

        // Code bg color
        code_bg_color: rgba_to_color32(&p.bg_overlay),

        ..Visuals::dark()
    };

    let spacing = Spacing {
        item_spacing: vec2(theme.spacing.space_1, theme.spacing.space_1),
        window_margin: Margin::same(theme.spacing.space_2),
        button_padding: vec2(theme.spacing.space_2, theme.spacing.space_1),
        indent: theme.spacing.space_4,
        interact_size: vec2(theme.spacing.space_10, theme.spacing.space_5),
        slider_width: 100.0,
        combo_width: 100.0,
        text_edit_width: 200.0,
        icon_width: theme.spacing.space_4,
        icon_width_inner: theme.spacing.space_3,
        icon_spacing: theme.spacing.space_1,
        tooltip_width: 220.0,
        menu_margin: Margin::same(theme.spacing.space_1),
        ..Spacing::default()
    };

    Style {
        visuals,
        spacing,
        ..Style::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyuki_ui_tokens::{COG_THEME, D2_THEME};

    #[test]
    fn test_rgba_to_color32_opaque() {
        let c = Rgba::new(255, 0, 0);
        let result = rgba_to_color32(&c);
        assert_eq!(result, Color32::from_rgba_premultiplied(255, 0, 0, 255));
    }

    #[test]
    fn test_rgba_to_color32_with_alpha() {
        let c = Rgba::with_alpha(100, 200, 50, 128);
        let result = rgba_to_color32(&c);
        assert_eq!(result, Color32::from_rgba_premultiplied(100, 200, 50, 128));
    }

    #[test]
    fn test_rgba_to_color32_black() {
        let c = Rgba::new(0, 0, 0);
        let result = rgba_to_color32(&c);
        assert_eq!(result, Color32::from_rgba_premultiplied(0, 0, 0, 255));
    }

    #[test]
    fn test_theme_to_style_cog_does_not_panic() {
        let _style = theme_to_style(&COG_THEME);
    }

    #[test]
    fn test_theme_to_style_d2_does_not_panic() {
        let _style = theme_to_style(&D2_THEME);
    }

    #[test]
    fn test_theme_to_style_d2_is_dark_mode() {
        let style = theme_to_style(&D2_THEME);
        assert!(style.visuals.dark_mode);
    }

    #[test]
    fn test_theme_to_style_d2_panel_fill() {
        let style = theme_to_style(&D2_THEME);
        let expected = rgba_to_color32(&D2_THEME.palette.bg_secondary);
        assert_eq!(style.visuals.panel_fill, expected);
    }

    #[test]
    fn test_theme_to_style_cog_panel_fill() {
        let style = theme_to_style(&COG_THEME);
        let expected = rgba_to_color32(&COG_THEME.palette.bg_secondary);
        assert_eq!(style.visuals.panel_fill, expected);
    }

    #[test]
    fn test_theme_to_style_d2_spacing() {
        let style = theme_to_style(&D2_THEME);
        assert!(style.spacing.item_spacing.x > 0.0);
        assert!(style.spacing.item_spacing.y > 0.0);
    }
}
