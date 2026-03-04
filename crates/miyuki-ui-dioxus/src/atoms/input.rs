// @id: MUID-TextInput @do: text-input-atom @role: component @layer: 6 @human: miyuk

//! Text input atom with type variants and error state.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     TextInput {
//!         placeholder: "Enter your name".to_string(),
//!         value: name_signal.read().clone(),
//!         on_input: move |evt: FormEvent| name_signal.set(evt.value()),
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Input type variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputType {
    /// Standard text input.
    #[default]
    Text,
    /// Password input (masked).
    Password,
    /// Numeric input.
    Number,
    /// Search input.
    Search,
}

impl InputType {
    /// HTML input type attribute value.
    fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Password => "password",
            Self::Number => "number",
            Self::Search => "search",
        }
    }
}

/// Text input component.
#[component]
pub fn TextInput(
    /// The type of input.
    #[props(default)]
    input_type: InputType,
    /// Placeholder text.
    #[props(default)]
    placeholder: String,
    /// Current value.
    #[props(default)]
    value: String,
    /// Input change handler.
    #[props(default)]
    on_input: EventHandler<FormEvent>,
    /// Disabled state.
    #[props(default)]
    disabled: bool,
    /// Error message (shows error border when Some).
    #[props(default)]
    error: Option<String>,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;
    let sp = &theme.spacing;
    let rad = &theme.radius;

    let has_error = error.is_some();
    let border_color = if has_error {
        p.error.to_css()
    } else {
        p.border_default.to_css()
    };
    let bg_color = p.bg_surface.to_css();
    let text_color = p.text_primary.to_css();
    let placeholder_note = p.text_muted.to_css();
    let pad_v = sp.space_2;
    let pad_h = sp.space_3;
    let radius_val = rad.md;
    let font_px = theme.typography.body.px;
    let opacity = if disabled { "0.5" } else { "1" };
    let cursor = if disabled { "not-allowed" } else { "text" };
    let type_str = input_type.as_str();

    let style_str = format!(
        "width: 100%; padding: {pad_v}px {pad_h}px; \
         background: {bg_color}; color: {text_color}; \
         border: 1px solid {border_color}; border-radius: {radius_val}px; \
         font-size: {font_px}px; font-family: inherit; \
         outline: none; transition: border-color 200ms; \
         opacity: {opacity}; cursor: {cursor}; box-sizing: border-box;"
    );

    let error_color = p.error.to_css();
    let error_text = error.unwrap_or_default();
    let show_error = has_error;

    rsx! {
        div { style: "display: flex; flex-direction: column; width: 100%;",
            input {
                style: "{style_str}",
                r#type: "{type_str}",
                placeholder: "{placeholder}",
                value: "{value}",
                disabled: disabled,
                oninput: move |evt| on_input.call(evt),
            }
            if show_error {
                div {
                    style: "color: {error_color}; font-size: 12px; margin-top: 4px; padding-left: {placeholder_note}",
                    "{error_text}"
                }
            }
        }
    }
}
