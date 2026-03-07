// @id: MUID-PageHeader @do: page-header-organism @role: component @layer: 6 @human: miyuk

//! PageHeader organism — per-page heading with title, description, breadcrumb, and action slot.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     PageHeader {
//!         title: "Tableau de bord",
//!         description: Some("Vue d'ensemble de vos éditions".to_string()),
//!         action: rsx! {
//!             Button { variant: ButtonVariant::Primary, on_click: handler, "Nouvelle édition" }
//!         },
//!     }
//! }
//! ```
//!
//! @id: muid_page_header @do: render_page_header
//! @role: ui @layer: service
//! @human: PageHeader — en-tête de page avec titre, description optionnelle et slot d'action.

use dioxus::prelude::*;

use crate::context::use_theme;

/// Page-level header organism.
#[component]
pub fn PageHeader(
    /// Page title.
    title: &'static str,
    /// Optional supporting description.
    #[props(default)]
    description: Option<String>,
    /// Optional right-side action slot.
    #[props(default)]
    action: Option<Element>,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;
    let ty = &theme.typography;

    let container_style = format!(
        "display: flex; justify-content: space-between; align-items: flex-start; \
         padding-bottom: 20px; border-bottom: 1px solid {}; margin-bottom: 24px;",
        p.border_default.to_css()
    );

    let text_col_style = "display: flex; flex-direction: column; gap: 4px;";

    let title_style = format!(
        "font-size: {}px; font-weight: 700; color: {}; margin: 0;",
        ty.xl.px,
        p.text_primary.to_css()
    );

    let desc_style = format!(
        "font-size: {}px; color: {}; margin: 0;",
        ty.sm.px,
        p.text_secondary.to_css()
    );

    rsx! {
        div {
            style: "{container_style}",
            div {
                style: "{text_col_style}",
                h1 { style: "{title_style}", "{title}" }
                if let Some(desc) = description {
                    p { style: "{desc_style}", "{desc}" }
                }
            }
            if let Some(slot) = action {
                div { style: "flex-shrink: 0;", {slot} }
            }
        }
    }
}
