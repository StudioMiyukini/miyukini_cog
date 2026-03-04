// @id: MUID-Icon @do: icon-atom @role: component @layer: 6 @human: miyuk

//! Icon atom rendering a text/emoji glyph at configurable sizes.
//!
//! Uses emoji or text glyphs since Dioxus desktop does not support
//! arbitrary SVG injection. For pixel-perfect icons, compose with
//! img elements instead.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     Icon { name: "search".to_string(), size: IconSize::Md }
//! }
//! ```

use dioxus::prelude::*;
use miyuki_ui_tokens::Rgba;

use crate::context::use_theme;

/// Icon display size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IconSize {
    /// 12px.
    Xs,
    /// 16px.
    Sm,
    /// 20px (default).
    #[default]
    Md,
    /// 24px.
    Lg,
    /// 32px.
    Xl,
}

impl IconSize {
    /// Size in logical pixels.
    fn px(self) -> f32 {
        match self {
            Self::Xs => 12.0,
            Self::Sm => 16.0,
            Self::Md => 20.0,
            Self::Lg => 24.0,
            Self::Xl => 32.0,
        }
    }
}

/// Icon component displaying a named glyph.
#[component]
pub fn Icon(
    /// Icon name (emoji or short text).
    name: String,
    /// Display size.
    #[props(default)]
    size: IconSize,
    /// Optional color override.
    #[props(default)]
    color: Option<Rgba>,
) -> Element {
    let theme = use_theme();
    let px = size.px();
    let color_css = match color {
        Some(c) => c.to_css(),
        None => theme.palette.text_primary.to_css(),
    };

    let style_str = format!(
        "display: inline-flex; align-items: center; justify-content: center; \
         width: {px}px; height: {px}px; font-size: {px}px; \
         color: {color_css}; line-height: 1; flex-shrink: 0;"
    );

    rsx! {
        span {
            style: "{style_str}",
            aria_hidden: "true",
            "{name}"
        }
    }
}
