// @id: MUID-SearchBar @do: search-bar-molecule @role: component @layer: 6 @human: miyuk

//! Search bar molecule composing a text input with a search icon.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     SearchBar {
//!         value: query(),
//!         on_change: move |v| set_query(v),
//!         placeholder: "Search services...".to_string(),
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Search bar molecule.
#[component]
pub fn SearchBar(
    /// Current search text.
    #[props(default)]
    value: String,
    /// Change handler for the search text.
    #[props(default)]
    on_change: EventHandler<String>,
    /// Placeholder text.
    #[props(default = "Search...".to_string())]
    placeholder: String,
    /// Optional clear handler (shows clear button when Some).
    #[props(default)]
    on_clear: Option<EventHandler<()>>,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;
    let sp = &theme.spacing;
    let rad = &theme.radius;

    let bg_color = p.bg_surface.to_css();
    let border_color = p.border_default.to_css();
    let text_color = p.text_primary.to_css();
    let muted_color = p.text_muted.to_css();
    let radius_val = rad.md;
    let pad_v = sp.space_2;
    let pad_h = sp.space_3;
    let font_px = theme.typography.body.px;

    let container_style = format!(
        "display: flex; align-items: center; gap: 8px; \
         padding: {pad_v}px {pad_h}px; \
         background: {bg_color}; border: 1px solid {border_color}; \
         border-radius: {radius_val}px; width: 100%; box-sizing: border-box;"
    );

    let input_style = format!(
        "flex: 1; background: transparent; border: none; outline: none; \
         color: {text_color}; font-size: {font_px}px; font-family: inherit;"
    );

    let icon_style = format!("color: {muted_color}; font-size: 16px; flex-shrink: 0;");

    let has_value = !value.is_empty();
    let has_clear = on_clear.is_some() && has_value;

    let clear_style = format!(
        "background: none; border: none; color: {muted_color}; \
         cursor: pointer; font-size: 14px; padding: 2px 4px; line-height: 1;"
    );

    rsx! {
        div { style: "{container_style}",
            span { style: "{icon_style}", "search" }
            input {
                style: "{input_style}",
                r#type: "search",
                placeholder: "{placeholder}",
                value: "{value}",
                oninput: move |evt: FormEvent| on_change.call(evt.value()),
            }
            if has_clear {
                button {
                    style: "{clear_style}",
                    onclick: move |_| {
                        if let Some(ref handler) = on_clear {
                            handler.call(());
                        }
                    },
                    "x"
                }
            }
        }
    }
}
