// @id: MUID-Spinner @do: spinner-atom @role: component @layer: 6 @human: miyuk

//! Spinner/loading indicator atom.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     Spinner { size: SpinnerSize::Md }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Spinner size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpinnerSize {
    /// 16px.
    Sm,
    /// 24px (default).
    #[default]
    Md,
    /// 40px.
    Lg,
}

impl SpinnerSize {
    /// Size in logical pixels.
    fn px(self) -> f32 {
        match self {
            Self::Sm => 16.0,
            Self::Md => 24.0,
            Self::Lg => 40.0,
        }
    }
}

/// Spinner loading indicator.
#[component]
pub fn Spinner(
    /// Size variant.
    #[props(default)]
    size: SpinnerSize,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;

    let px = size.px();
    let border_w = (px / 8.0).max(2.0);
    let accent = p.accent_primary.to_css();
    let track = p.bg_overlay.to_css();

    // CSS animation for rotation is defined inline.
    // Dioxus desktop uses webview, so CSS animations work.
    let style_str = format!(
        "display: inline-block; width: {px}px; height: {px}px; \
         border: {border_w}px solid {track}; \
         border-top-color: {accent}; \
         border-radius: 50%; \
         animation: miyuki-spin 0.8s linear infinite;"
    );

    // We inject the keyframes via a style element once.
    let keyframes = "@keyframes miyuki-spin { to { transform: rotate(360deg); } }";

    rsx! {
        style { "{keyframes}" }
        div {
            style: "{style_str}",
            role: "status",
            aria_label: "Loading",
        }
    }
}
