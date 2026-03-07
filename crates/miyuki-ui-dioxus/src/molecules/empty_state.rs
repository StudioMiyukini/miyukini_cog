// @id: MUID-EmptyState @do: empty-state-molecule @role: component @layer: 6 @human: miyuk

//! EmptyState molecule — placeholder display when a list or view has no content.
//!
//! # Example
//!
//! ```rust,ignore
//! rsx! {
//!     EmptyState {
//!         icon: "📭",
//!         title: "Aucun exposant",
//!         description: Some("Ajoutez votre premier exposant pour commencer.".to_string()),
//!         action_label: Some("Ajouter".to_string()),
//!         on_action: Some(EventHandler::new(|_| { /* navigate */ })),
//!     }
//! }
//! ```
//!
//! @id: muid_empty_state @do: render_empty_state
//! @role: ui @layer: service
//! @human: EmptyState — affiche un état vide avec icône, titre, description optionnelle et CTA optionnel.

use dioxus::prelude::*;

use crate::atoms::button::{Button, ButtonVariant};
use crate::context::use_theme;

/// Empty state placeholder molecule.
#[component]
pub fn EmptyState(
    /// Icon or emoji displayed above the title.
    icon: &'static str,
    /// Main title text.
    title: &'static str,
    /// Optional supporting description.
    #[props(default)]
    description: Option<String>,
    /// Optional action button label.
    #[props(default)]
    action_label: Option<String>,
    /// Optional action button handler.
    #[props(default)]
    on_action: Option<EventHandler<MouseEvent>>,
) -> Element {
    let theme = use_theme();
    let p = &theme.palette;
    let ty = &theme.typography;

    let container_style = format!(
        "display: flex; flex-direction: column; align-items: center; justify-content: center; \
         padding: 48px 24px; gap: 12px; text-align: center;"
    );

    let icon_style = format!("font-size: 40px; line-height: 1;");

    let title_style = format!(
        "font-size: {}px; font-weight: 600; color: {}; margin: 0;",
        ty.lg.px,
        p.text_primary.to_css()
    );

    let desc_style = format!(
        "font-size: {}px; color: {}; margin: 0; max-width: 320px;",
        ty.sm.px,
        p.text_secondary.to_css()
    );

    rsx! {
        div {
            style: "{container_style}",
            span { style: "{icon_style}", "{icon}" }
            p { style: "{title_style}", "{title}" }
            if let Some(desc) = &description {
                p { style: "{desc_style}", "{desc}" }
            }
            if let (Some(label), Some(handler)) = (action_label, on_action) {
                Button {
                    variant: ButtonVariant::Primary,
                    on_click: handler,
                    "{label}"
                }
            }
        }
    }
}
