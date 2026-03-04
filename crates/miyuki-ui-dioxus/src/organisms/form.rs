// @id: MUID-Form @do: form-organism @role: component @layer: 6 @human: miyuk

//! Form organism with title, fields, and submit/cancel actions.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     Form {
//!         title: Some("Create Account".to_string()),
//!         submit_label: "Create".to_string(),
//!         on_submit: move |_| { /* submit form */ },
//!         FormField { label: "Username".to_string(), TextInput { /* ... */ } }
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Form organism.
#[component]
pub fn Form(
    /// Optional form title.
    #[props(default)]
    title: Option<String>,
    /// Submit handler.
    #[props(default)]
    on_submit: EventHandler<()>,
    /// Submit button label.
    #[props(default = "Submit".to_string())]
    submit_label: String,
    /// Optional cancel button label (hidden if None).
    #[props(default)]
    cancel_label: Option<String>,
    /// Optional cancel handler.
    #[props(default)]
    on_cancel: Option<EventHandler<()>>,
    /// Form fields (children).
    children: Element,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;
    let sp = &theme.spacing;

    let title_color = p.text_high.to_css();
    let border_color = p.border_subtle.to_css();
    let accent = p.accent_primary.to_css();
    let text_on_accent = p.text_high.to_css();
    let secondary_bg = p.bg_overlay.to_css();
    let secondary_text = p.text_primary.to_css();
    let pad = sp.space_4;
    let radius = theme.radius.md;

    let form_style = format!(
        "display: flex; flex-direction: column; gap: {pad}px;"
    );

    let has_title = title.is_some();
    let title_text = title.unwrap_or_default();
    let has_cancel = cancel_label.is_some() && on_cancel.is_some();
    let cancel_text = cancel_label.unwrap_or_default();

    let title_style = format!(
        "font-size: 20px; font-weight: 600; color: {title_color}; \
         padding-bottom: {pad}px; border-bottom: 1px solid {border_color}; margin: 0;"
    );

    let actions_style = format!(
        "display: flex; justify-content: flex-end; gap: 8px; \
         padding-top: {pad}px; border-top: 1px solid {border_color};"
    );

    let submit_style = format!(
        "padding: 8px 20px; background: {accent}; color: {text_on_accent}; \
         border: none; border-radius: {radius}px; font-size: 14px; \
         font-weight: 500; cursor: pointer; transition: opacity 150ms;"
    );

    let cancel_style = format!(
        "padding: 8px 20px; background: {secondary_bg}; color: {secondary_text}; \
         border: none; border-radius: {radius}px; font-size: 14px; \
         font-weight: 500; cursor: pointer; transition: opacity 150ms;"
    );

    rsx! {
        div { style: "{form_style}",
            if has_title {
                div { style: "{title_style}", "{title_text}" }
            }
            {children}
            div { style: "{actions_style}",
                if has_cancel {
                    button {
                        style: "{cancel_style}",
                        onclick: move |_| {
                            if let Some(ref handler) = on_cancel {
                                handler.call(());
                            }
                        },
                        "{cancel_text}"
                    }
                }
                button {
                    style: "{submit_style}",
                    onclick: move |_| on_submit.call(()),
                    "{submit_label}"
                }
            }
        }
    }
}
