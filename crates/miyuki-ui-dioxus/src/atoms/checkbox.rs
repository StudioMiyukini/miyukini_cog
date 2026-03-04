// @id: MUID-Checkbox @do: checkbox-atom @role: component @layer: 6 @human: miyuk

//! Checkbox atom with label and indeterminate state.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     Checkbox {
//!         checked: is_checked(),
//!         on_change: move |val: bool| set_checked(val),
//!         label: Some("Accept terms".to_string()),
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Checkbox component.
#[component]
pub fn Checkbox(
    /// Current checked state.
    #[props(default)]
    checked: bool,
    /// Change handler receiving the new checked state.
    #[props(default)]
    on_change: EventHandler<bool>,
    /// Optional text label.
    #[props(default)]
    label: Option<String>,
    /// Disabled state.
    #[props(default)]
    disabled: bool,
    /// Indeterminate state (overrides checked visually).
    #[props(default)]
    indeterminate: bool,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;

    let border_color = if checked || indeterminate {
        p.accent_primary.to_css()
    } else {
        p.border_default.to_css()
    };
    let bg_color = if checked || indeterminate {
        p.accent_primary.to_css()
    } else {
        "transparent".to_string()
    };
    let text_color = p.text_primary.to_css();
    let opacity = if disabled { "0.5" } else { "1" };
    let cursor = if disabled {
        "not-allowed"
    } else {
        "pointer"
    };

    let indicator = if indeterminate {
        "-"
    } else if checked {
        "v"
    } else {
        ""
    };
    let indicator_color = p.text_high.to_css();

    let container_style = format!(
        "display: inline-flex; align-items: center; gap: 8px; \
         cursor: {cursor}; opacity: {opacity}; user-select: none;"
    );

    let box_style = format!(
        "display: inline-flex; align-items: center; justify-content: center; \
         width: 18px; height: 18px; border: 2px solid {border_color}; \
         border-radius: 3px; background: {bg_color}; \
         color: {indicator_color}; font-size: 12px; font-weight: 700; \
         transition: all 150ms; flex-shrink: 0;"
    );

    let label_style = format!(
        "font-size: 14px; color: {text_color}; line-height: 1.4;"
    );

    let label_text = label.clone().unwrap_or_default();
    let has_label = label.is_some();

    rsx! {
        div {
            style: "{container_style}",
            role: "checkbox",
            aria_checked: if indeterminate { "mixed".to_string() } else { checked.to_string() },
            onclick: move |_| {
                if !disabled {
                    on_change.call(!checked);
                }
            },
            div {
                style: "{box_style}",
                "{indicator}"
            }
            if has_label {
                span {
                    style: "{label_style}",
                    "{label_text}"
                }
            }
        }
    }
}
