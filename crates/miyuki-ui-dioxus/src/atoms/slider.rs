// @id: MUID-Slider @do: slider-atom @role: component @layer: 6 @human: miyuk

//! Slider/range input atom.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     Slider {
//!         value: 50.0,
//!         min: 0.0,
//!         max: 100.0,
//!         step: 1.0,
//!         on_change: move |v: f64| set_volume(v),
//!         label: Some("Volume".to_string()),
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Slider component for numeric range input.
#[component]
pub fn Slider(
    /// Current value.
    #[props(default = 0.0)]
    value: f64,
    /// Minimum value.
    #[props(default = 0.0)]
    min: f64,
    /// Maximum value.
    #[props(default = 100.0)]
    max: f64,
    /// Step increment.
    #[props(default = 1.0)]
    step: f64,
    /// Value change handler.
    #[props(default)]
    on_change: EventHandler<f64>,
    /// Optional label.
    #[props(default)]
    label: Option<String>,
    /// Disabled state.
    #[props(default)]
    disabled: bool,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;

    let accent = p.accent_primary.to_css();
    let track_bg = p.bg_overlay.to_css();
    let text_color = p.text_primary.to_css();
    let opacity = if disabled { "0.5" } else { "1" };

    let range = max - min;
    let pct = if range > 0.0 {
        ((value - min) / range * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };
    let pct_int = pct as u32;

    let container_style = format!(
        "display: flex; flex-direction: column; gap: 4px; opacity: {opacity}; width: 100%;"
    );

    let track_style = format!(
        "width: 100%; height: 6px; border-radius: 3px; \
         background: linear-gradient(to right, {accent} {pct_int}%, {track_bg} {pct_int}%); \
         cursor: pointer; -webkit-appearance: none; appearance: none; outline: none;"
    );

    let label_text = label.clone().unwrap_or_default();
    let has_label = label.is_some();
    let min_str = format!("{min}");
    let max_str = format!("{max}");
    let step_str = format!("{step}");
    let value_str = format!("{value}");

    rsx! {
        div { style: "{container_style}",
            if has_label {
                div {
                    style: "display: flex; justify-content: space-between; font-size: 12px; color: {text_color};",
                    span { "{label_text}" }
                    span { "{value_str}" }
                }
            }
            input {
                style: "{track_style}",
                r#type: "range",
                min: "{min_str}",
                max: "{max_str}",
                step: "{step_str}",
                value: "{value_str}",
                disabled: disabled,
                oninput: move |evt: FormEvent| {
                    if let Ok(v) = evt.value().parse::<f64>() {
                        on_change.call(v);
                    }
                },
            }
        }
    }
}
