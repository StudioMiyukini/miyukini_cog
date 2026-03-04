// @id: MUID-Card @do: card-molecule @role: component @layer: 6 @human: miyuk

//! Card molecule with optional header, body, and footer.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     Card {
//!         title: Some("My Card".to_string()),
//!         on_click: Some(EventHandler::new(|_| { /* navigate */ })),
//!         p { "Card content goes here" }
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Card container molecule.
#[component]
pub fn Card(
    /// Optional title in the header.
    #[props(default)]
    title: Option<String>,
    /// Optional description subtitle.
    #[props(default)]
    description: Option<String>,
    /// Optional header icon (emoji/text).
    #[props(default)]
    header_icon: Option<String>,
    /// Optional click handler (makes the card interactive).
    #[props(default)]
    on_click: Option<EventHandler<MouseEvent>>,
    /// Optional footer element.
    #[props(default)]
    footer: Option<Element>,
    /// Card body content.
    children: Element,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;
    let sp = &theme.spacing;
    let rad = &theme.radius;
    let shd = &theme.shadow;

    let bg_color = p.bg_surface.to_css();
    let border_color = p.border_subtle.to_css();
    let text_color = p.text_primary.to_css();
    let title_color = p.text_high.to_css();
    let desc_color = p.text_secondary.to_css();
    let radius_val = rad.lg;
    let shadow_css = shd.sm.to_css();
    let pad = sp.space_4;

    let is_clickable = on_click.is_some();
    let cursor = if is_clickable { "pointer" } else { "default" };

    let card_style = format!(
        "display: flex; flex-direction: column; \
         background: {bg_color}; border: 1px solid {border_color}; \
         border-radius: {radius_val}px; box-shadow: {shadow_css}; \
         overflow: hidden; cursor: {cursor}; \
         transition: box-shadow 200ms, border-color 200ms;"
    );

    let has_header = title.is_some() || header_icon.is_some();
    let has_footer = footer.is_some();
    let title_text = title.unwrap_or_default();
    let desc_text = description.unwrap_or_default();
    let has_desc = !desc_text.is_empty();
    let icon_text = header_icon.unwrap_or_default();
    let has_icon = !icon_text.is_empty();

    let header_style = format!(
        "display: flex; align-items: center; gap: 8px; \
         padding: {pad}px; border-bottom: 1px solid {border_color};"
    );

    let body_style = format!(
        "padding: {pad}px; color: {text_color}; flex: 1;"
    );

    let footer_style = format!(
        "padding: {pad}px; border-top: 1px solid {border_color};"
    );

    rsx! {
        div {
            style: "{card_style}",
            onclick: move |evt| {
                if let Some(ref handler) = on_click {
                    handler.call(evt);
                }
            },
            if has_header {
                div { style: "{header_style}",
                    if has_icon {
                        span { style: "font-size: 20px;", "{icon_text}" }
                    }
                    div { style: "display: flex; flex-direction: column;",
                        div { style: "font-size: 16px; font-weight: 600; color: {title_color};", "{title_text}" }
                        if has_desc {
                            div { style: "font-size: 12px; color: {desc_color}; margin-top: 2px;", "{desc_text}" }
                        }
                    }
                }
            }
            div { style: "{body_style}",
                {children}
            }
            if has_footer {
                div { style: "{footer_style}",
                    {footer}
                }
            }
        }
    }
}
