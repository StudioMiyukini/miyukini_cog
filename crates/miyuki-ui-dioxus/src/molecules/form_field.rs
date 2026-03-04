// @id: MUID-FormField @do: form-field-molecule @role: component @layer: 6 @human: miyuk

//! Form field molecule composing a label, input, and error message.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     FormField {
//!         label: "Email".to_string(),
//!         required: true,
//!         error: email_error(),
//!         TextInput { value: email(), on_input: move |e| set_email(e) }
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Form field molecule wrapping a label + input + error.
#[component]
pub fn FormField(
    /// Field label text.
    label: String,
    /// Error message (shown below the input).
    #[props(default)]
    error: Option<String>,
    /// Whether the field is required (shows * marker).
    #[props(default)]
    required: bool,
    /// The input component (children).
    children: Element,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;

    let label_color = p.text_secondary.to_css();
    let error_color = p.error.to_css();
    let required_color = p.error.to_css();
    let font_px = theme.typography.sm.px;

    let container_style = "display: flex; flex-direction: column; gap: 4px; width: 100%;";

    let label_style = format!(
        "font-size: {font_px}px; color: {label_color}; font-weight: 500;"
    );

    let has_error = error.is_some();
    let error_text = error.unwrap_or_default();
    let error_style = format!(
        "font-size: 12px; color: {error_color}; margin-top: 2px;"
    );

    rsx! {
        div { style: "{container_style}",
            label { style: "{label_style}",
                "{label}"
                if required {
                    span { style: "color: {required_color}; margin-left: 2px;", " *" }
                }
            }
            {children}
            if has_error {
                div { style: "{error_style}", "{error_text}" }
            }
        }
    }
}
