// @id: MUID-CommandPalette @do: command-palette-organism @role: component @layer: 6 @human: miyuk

//! Command palette organism for quick actions.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     CommandPalette {
//!         query: query_val(),
//!         on_query: move |q| set_query(q),
//!         actions: vec![
//!             CommandAction { id: "settings".into(), label: "Open Settings".into(), shortcut: Some("Ctrl+,".into()) },
//!         ],
//!         on_select: move |id| handle_action(id),
//!         on_close: move |_| close_palette(),
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// A command palette action.
#[derive(Debug, Clone, PartialEq)]
pub struct CommandAction {
    /// Unique identifier.
    pub id: String,
    /// Display label.
    pub label: String,
    /// Optional keyboard shortcut.
    pub shortcut: Option<String>,
}

/// Command palette organism.
#[component]
pub fn CommandPalette(
    /// Current search query.
    #[props(default)]
    query: String,
    /// Query change handler.
    #[props(default)]
    on_query: EventHandler<String>,
    /// Available actions.
    #[props(default)]
    actions: Vec<CommandAction>,
    /// Action selection handler (receives action id).
    #[props(default)]
    on_select: EventHandler<String>,
    /// Close handler.
    #[props(default)]
    on_close: EventHandler<()>,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;
    let z = theme.z_index;

    let backdrop_z = z.overlay;
    let palette_z = z.modal;
    let bg_color = p.bg_surface.to_css();
    let border_color = p.border_default.to_css();
    let text_color = p.text_primary.to_css();
    let muted_color = p.text_muted.to_css();
    let input_bg = p.bg_primary.to_css();
    let radius = theme.radius.lg;
    let shadow = theme.shadow.xl.to_css();

    let backdrop_style = format!(
        "position: fixed; inset: 0; \
         background: rgba(0, 0, 0, 0.5); \
         display: flex; align-items: flex-start; justify-content: center; \
         padding-top: 120px; z-index: {backdrop_z};"
    );

    let palette_style = format!(
        "display: flex; flex-direction: column; \
         width: 100%; max-width: 560px; max-height: 400px; \
         background: {bg_color}; border: 1px solid {border_color}; \
         border-radius: {radius}px; box-shadow: {shadow}; \
         z-index: {palette_z}; overflow: hidden;"
    );

    let input_style = format!(
        "width: 100%; padding: 14px 16px; border: none; outline: none; \
         background: {input_bg}; color: {text_color}; font-size: 16px; \
         border-bottom: 1px solid {border_color}; box-sizing: border-box;"
    );

    let list_style = "flex: 1; overflow-y: auto; padding: 4px 0;";

    // Filter actions by query
    let query_lower = query.to_lowercase();
    let filtered: Vec<&CommandAction> = actions
        .iter()
        .filter(|a| {
            query_lower.is_empty() || a.label.to_lowercase().contains(&query_lower)
        })
        .collect();

    rsx! {
        div {
            style: "{backdrop_style}",
            onclick: move |_| on_close.call(()),
            div {
                style: "{palette_style}",
                onclick: move |evt| evt.stop_propagation(),
                input {
                    style: "{input_style}",
                    placeholder: "Type a command...",
                    value: "{query}",
                    oninput: move |evt: FormEvent| on_query.call(evt.value()),
                }
                div { style: "{list_style}",
                    for action in filtered.iter() {
                        {
                            let action_id = action.id.clone();
                            let has_shortcut = action.shortcut.is_some();
                            let shortcut_text = action.shortcut.clone().unwrap_or_default();
                            let item_style = format!(
                                "display: flex; align-items: center; justify-content: space-between; \
                                 padding: 8px 16px; color: {text_color}; font-size: 14px; \
                                 cursor: pointer; transition: background 100ms;"
                            );
                            rsx! {
                                div {
                                    style: "{item_style}",
                                    onclick: move |_| {
                                        on_select.call(action_id.clone());
                                    },
                                    span { "{action.label}" }
                                    if has_shortcut {
                                        span { style: "font-size: 12px; color: {muted_color};", "{shortcut_text}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
