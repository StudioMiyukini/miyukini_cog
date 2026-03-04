// @id: MUID-AuthLayout @do: auth-template @role: component @layer: 6 @human: miyuk

//! Authentication layout with centered card on a fullscreen background.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     AuthLayout {
//!         logo: Some(rsx! { img { src: "/logo.png" } }),
//!         Form { /* login form */ }
//!     }
//! }
//! ```

use dioxus::prelude::*;

use crate::context::use_theme;

/// Authentication page layout template.
#[component]
pub fn AuthLayout(
    /// Optional logo element above the card.
    #[props(default)]
    logo: Option<Element>,
    /// Card content (children, usually a form).
    children: Element,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;

    let bg_color = p.bg_base.to_css();
    let card_bg = p.bg_surface.to_css();
    let border_color = p.border_default.to_css();
    let radius = theme.radius.lg;
    let shadow = theme.shadow.lg.to_css();
    let has_logo = logo.is_some();

    let container_style = format!(
        "display: flex; align-items: center; justify-content: center; \
         min-height: 100vh; background: {bg_color}; padding: 24px;"
    );

    let wrapper_style = "display: flex; flex-direction: column; align-items: center; gap: 24px;";

    let card_style = format!(
        "width: 100%; max-width: 420px; \
         background: {card_bg}; border: 1px solid {border_color}; \
         border-radius: {radius}px; box-shadow: {shadow}; \
         padding: 32px; box-sizing: border-box;"
    );

    rsx! {
        div { style: "{container_style}",
            div { style: "{wrapper_style}",
                if has_logo {
                    {logo}
                }
                div { style: "{card_style}",
                    {children}
                }
            }
        }
    }
}
