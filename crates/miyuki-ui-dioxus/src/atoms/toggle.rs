// @id: MUID-Toggle @do: toggle-atom @role: component @layer: 6 @human: miyuk

//! Toggle switch atom.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     Toggle {
//!         checked: is_enabled(),
//!         on_change: move |val| set_enabled(val),
//!         label: Some("Dark mode".to_string()),
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Toggle size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToggleSize {
    /// Small toggle (32x18px).
    Sm,
    /// Medium toggle (44x24px, default).
    #[default]
    Md,
}

/// Toggle switch component.
#[component]
pub fn Toggle(
    /// Current state.
    #[props(default)]
    checked: bool,
    /// Change handler.
    #[props(default)]
    on_change: EventHandler<bool>,
    /// Optional label.
    #[props(default)]
    label: Option<String>,
    /// Size variant.
    #[props(default)]
    size: ToggleSize,
    /// Disabled state.
    #[props(default)]
    disabled: bool,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;

    let (track_w, track_h, knob_size, knob_offset) = match size {
        ToggleSize::Sm => (32.0_f32, 18.0_f32, 14.0_f32, 2.0_f32),
        ToggleSize::Md => (44.0_f32, 24.0_f32, 20.0_f32, 2.0_f32),
    };

    let track_bg = if checked {
        p.accent_primary.to_css()
    } else {
        p.bg_overlay.to_css()
    };
    let knob_color = p.text_high.to_css();
    let text_color = p.text_primary.to_css();
    let opacity = if disabled { "0.5" } else { "1" };
    let cursor = if disabled { "not-allowed" } else { "pointer" };

    let knob_x = if checked {
        track_w - knob_size - knob_offset
    } else {
        knob_offset
    };
    let half_h = track_h / 2.0;

    let container_style = format!(
        "display: inline-flex; align-items: center; gap: 8px; \
         cursor: {cursor}; opacity: {opacity}; user-select: none;"
    );

    let track_style = format!(
        "position: relative; width: {track_w}px; height: {track_h}px; \
         background: {track_bg}; border-radius: {half_h}px; \
         transition: background 200ms; flex-shrink: 0;"
    );

    let knob_top = knob_offset;
    let knob_style = format!(
        "position: absolute; top: {knob_top}px; left: {knob_x}px; \
         width: {knob_size}px; height: {knob_size}px; \
         background: {knob_color}; border-radius: 50%; \
         transition: left 200ms;"
    );

    let label_text = label.clone().unwrap_or_default();
    let has_label = label.is_some();

    rsx! {
        div {
            style: "{container_style}",
            role: "switch",
            aria_checked: "{checked}",
            onclick: move |_| {
                if !disabled {
                    on_change.call(!checked);
                }
            },
            div {
                style: "{track_style}",
                div { style: "{knob_style}" }
            }
            if has_label {
                span {
                    style: "font-size: 14px; color: {text_color};",
                    "{label_text}"
                }
            }
        }
    }
}
