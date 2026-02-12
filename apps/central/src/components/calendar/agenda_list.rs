//! Composant liste d'agenda — Affichage en liste des événements.
//!
//! @id: central.components.calendar.agenda_list
//! @do: render_agenda_list_item
//! @role: ui
//! @layer: ui
//! @human: Composant item de liste d'agenda

use dioxus::prelude::*;
use crate::state::use_app_state;

/// Props pour un item de liste d'agenda.
#[derive(Props, Clone, PartialEq)]
pub struct AgendaListItemProps {
    /// Titre de l'événement.
    pub title: String,
    /// Heure affichée.
    pub time: String,
    /// Lieu optionnel.
    #[props(default)]
    pub location: Option<String>,
    /// Type d'événement (pour la couleur).
    #[props(default)]
    pub event_type: String,
}

/// Item de liste d'agenda.
#[component]
pub fn AgendaListItem(props: AgendaListItemProps) -> Element {
    let c = use_app_state().read().current_theme.palette();
    
    // Couleur selon le type
    let indicator_color = match props.event_type.as_str() {
        "preparation" => "#F59E0B",
        "evenement" | "conference" => "#3B82F6",
        "animation" => "#10B981",
        "rdv" => "#8B5CF6",
        "visite" => "#EC4899",
        "reservation" => "#F97316",
        "concours" => "#14B8A6",
        _ => c.accent_blue,
    };
    
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px; padding: 12px; background: {c.bg_secondary}; border-radius: 8px; border-left: 4px solid {indicator_color};",
            
            // Heure
            div {
                style: "min-width: 60px; font-size: 14px; font-weight: 600; color: {c.text_white};",
                "{props.time}"
            }
            
            // Contenu
            div {
                style: "flex: 1;",
                
                p {
                    style: "font-size: 14px; color: {c.text_white}; font-weight: 500;",
                    "{props.title}"
                }
                
                if let Some(loc) = &props.location {
                    p {
                        style: "font-size: 12px; color: {c.text_muted}; margin-top: 2px;",
                        "📍 {loc}"
                    }
                }
            }
        }
    }
}
