// @id: MUID-Progress @do: progress-bar-atom @role: component @layer: 6 @human: miyuk

//! Progress bar atom with linear variant.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     ProgressBar { value: 0.75, variant: ProgressVariant::Linear }
//! }
//! ```

use dioxus::prelude::*;
use miyuki_ui_tokens::Rgba;

use crate::context::use_theme;

/// Progress bar display variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProgressVariant {
    /// Horizontal bar.
    #[default]
    Linear,
}

/// Progress bar component.
#[component]
pub fn ProgressBar(
    /// Progress value from 0.0 to 1.0.
    #[props(default = 0.0)]
    value: f64,
    /// Display variant.
    #[props(default)]
    variant: ProgressVariant,
    /// Optional fill color override.
    #[props(default)]
    color: Option<Rgba>,
    /// Optional height in pixels.
    #[props(default)]
    height: Option<f32>,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;

    let clamped = value.clamp(0.0, 1.0);
    let pct = (clamped * 100.0) as u32;
    let bar_height = height.unwrap_or(6.0);
    let radius = bar_height / 2.0;

    let fill_color = match color {
        Some(c) => c.to_css(),
        None => p.accent_primary.to_css(),
    };
    let track_color = p.bg_overlay.to_css();

    let track_style = format!(
        "width: 100%; height: {bar_height}px; background: {track_color}; \
         border-radius: {radius}px; overflow: hidden;"
    );

    let fill_style = format!(
        "width: {pct}%; height: 100%; background: {fill_color}; \
         border-radius: {radius}px; transition: width 300ms;"
    );

    rsx! {
        div {
            style: "{track_style}",
            role: "progressbar",
            aria_valuenow: "{pct}",
            aria_valuemin: "0",
            aria_valuemax: "100",
            div { style: "{fill_style}" }
        }
    }
}
