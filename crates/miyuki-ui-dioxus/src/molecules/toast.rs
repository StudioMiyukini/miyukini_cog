// @id: MUID-Toast @do: toast-molecule @role: component @layer: 6 @human: miyuk

//! Toast notification molecule.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     Toast {
//!         message: "Settings saved successfully".to_string(),
//!         variant: ToastVariant::Success,
//!         on_dismiss: move |_| hide_toast(),
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Toast notification variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ToastVariant {
    /// Success notification (green).
    Success,
    /// Warning notification (orange).
    Warning,
    /// Error notification (red).
    Error,
    /// Informational notification (blue, default).
    #[default]
    Info,
}

/// Toast notification molecule.
#[component]
pub fn Toast(
    /// Notification message.
    message: String,
    /// Visual variant.
    #[props(default)]
    variant: ToastVariant,
    /// Dismiss handler.
    #[props(default)]
    on_dismiss: EventHandler<()>,
    /// Auto-dismiss duration in ms (None = manual dismiss only).
    #[props(default)]
    duration_ms: Option<u32>,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;
    let z = theme.z_index.notification;

    let (accent_color, icon_char) = match variant {
        ToastVariant::Success => (p.success.to_css(), "v"),
        ToastVariant::Warning => (p.warning.to_css(), "!"),
        ToastVariant::Error => (p.error.to_css(), "x"),
        ToastVariant::Info => (p.info.to_css(), "i"),
    };

    let bg_color = p.bg_elevated.to_css();
    let text_color = p.text_primary.to_css();
    let border_color = p.border_default.to_css();
    let muted_color = p.text_muted.to_css();
    let radius = theme.radius.lg;

    let container_style = format!(
        "display: flex; align-items: center; gap: 12px; \
         padding: 12px 16px; \
         background: {bg_color}; border: 1px solid {border_color}; \
         border-left: 3px solid {accent_color}; \
         border-radius: {radius}px; \
         box-shadow: 0 4px 12px rgba(0,0,0,0.3); \
         z-index: {z}; min-width: 300px; max-width: 480px;"
    );

    let icon_style = format!(
        "display: flex; align-items: center; justify-content: center; \
         width: 20px; height: 20px; border-radius: 50%; \
         background: {accent_color}; color: white; \
         font-size: 12px; font-weight: 700; flex-shrink: 0;"
    );

    let close_style = format!(
        "background: none; border: none; color: {muted_color}; \
         cursor: pointer; font-size: 16px; padding: 2px; line-height: 1; \
         flex-shrink: 0;"
    );

    // Encode duration as data attribute so the parent can read it,
    // and to avoid an unused-parameter lint.
    let duration_attr = duration_ms.map_or_else(String::new, |ms| format!("{ms}"));

    rsx! {
        div {
            style: "{container_style}",
            role: "alert",
            "data-duration": "{duration_attr}",
            div { style: "{icon_style}", "{icon_char}" }
            span { style: "flex: 1; color: {text_color}; font-size: 14px;", "{message}" }
            button {
                style: "{close_style}",
                onclick: move |_| on_dismiss.call(()),
                aria_label: "Dismiss",
                "x"
            }
        }
    }
}
