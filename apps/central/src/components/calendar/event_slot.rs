//! Rendu d'un événement dans la grille calendrier.
//!
//! @id: central.components.calendar.event_slot
//! @do: render_calendar_event_block
//! @role: ui
//! @layer: ui
//! @human: Composant de rendu d'un événement dans les grilles jour/semaine

use dioxus::prelude::*;
use crate::state::use_app_state;

/// Props pour le slot d'événement.
#[derive(Props, Clone, PartialEq)]
pub struct EventSlotProps {
    /// Titre de l'événement.
    pub title: String,
    /// Couleur de fond (hex).
    pub color: String,
    /// Libellé horaire (ex: "09:00 — 10:30").
    #[props(default)]
    pub time_label: String,
    /// Lieu optionnel.
    pub location: Option<String>,
    /// Décalage vertical en pixels.
    #[props(default = 0)]
    pub top_px: u32,
    /// Hauteur en pixels.
    #[props(default = 60)]
    pub height_px: u32,
    /// Largeur en pourcentage.
    #[props(default = 95)]
    pub width_percent: u32,
    /// Décalage horizontal en pourcentage.
    #[props(default = 2)]
    pub left_percent: u32,
    /// L'événement est en conflit.
    #[props(default = false)]
    pub has_conflict: bool,
    /// L'événement est en lecture seule.
    #[props(default = false)]
    pub is_readonly: bool,
    /// Icône de source.
    pub source_icon: Option<String>,
    /// Callback au clic.
    pub on_click: Option<EventHandler<()>>,
}

/// Bloc d'événement dans la grille du calendrier.
#[component]
pub fn EventSlot(props: EventSlotProps) -> Element {
    let state = use_app_state();
    let c = state.read().current_theme.palette();
    
    // Style de bordure pour les conflits
    let border = if props.has_conflict {
        format!("3px solid {}", c.accent_orange)
    } else {
        "none".to_string()
    };

    rsx! {
        div {
            style: "position: absolute; top: {props.top_px}px; left: {props.left_percent}%; width: calc({props.width_percent}% - 4px); height: {props.height_px}px; background: {props.color}; border-radius: 4px; padding: 4px 8px; overflow: hidden; cursor: pointer; transition: transform 0.1s, box-shadow 0.2s; border-left: {border};",
            onclick: move |_| {
                if let Some(handler) = &props.on_click {
                    handler.call(());
                }
            },
            
            // Titre
            div {
                style: "display: flex; align-items: center; gap: 4px;",
                span {
                    style: "font-size: 12px; font-weight: 500; color: white; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                    "{props.title}"
                }
                if let Some(icon) = &props.source_icon {
                    span {
                        style: "font-size: 10px;",
                        "{icon}"
                    }
                }
                if props.is_readonly {
                    span {
                        style: "font-size: 10px; opacity: 0.7;",
                        "🔒"
                    }
                }
            }
            
            // Heure
            if !props.time_label.is_empty() && props.height_px > 40 {
                span {
                    style: "font-size: 10px; color: rgba(255,255,255,0.8);",
                    "{props.time_label}"
                }
            }
            
            // Lieu
            if let Some(loc) = &props.location {
                if props.height_px > 60 && !loc.is_empty() {
                    div {
                        style: "font-size: 10px; color: rgba(255,255,255,0.7); margin-top: 2px;",
                        "📍 {loc}"
                    }
                }
            }
        }
    }
}

/// Props pour un item d'agenda (liste).
#[derive(Props, Clone, PartialEq)]
pub struct AgendaListItemProps {
    /// Titre de l'événement.
    pub title: String,
    /// Heure formatée (ex: "14:00").
    pub time: String,
    /// Lieu optionnel.
    pub location: Option<String>,
    /// Type d'événement (pour icône/couleur).
    #[props(default = "default".to_string())]
    pub event_type: String,
    /// Callback au clic.
    pub on_click: Option<EventHandler<()>>,
}

/// Item d'événement pour les vues liste/agenda.
#[component]
pub fn AgendaListItem(props: AgendaListItemProps) -> Element {
    let state = use_app_state();
    let c = state.read().current_theme.palette();

    let (icon, color) = match props.event_type.as_str() {
        "visite" => ("🎪", c.accent_blue),
        "preparation" => ("🔧", c.accent_orange),
        "evenement" => ("🎪", c.accent_blue),
        "animation" => ("🎤", c.accent_green),
        "rdv" => ("📅", c.accent_blue),
        "reservation" => ("📅", c.accent_green),
        "conference" => ("🎤", c.accent_orange),
        "concours" => ("🏆", c.accent_blue),
        _ => ("📌", c.text_muted),
    };

    rsx! {
        div {
            style: "display: flex; gap: 16px; background: {c.bg_secondary}; border-radius: 8px; padding: 16px; border-left: 3px solid {color}; cursor: pointer;",
            onclick: move |_| {
                if let Some(handler) = &props.on_click {
                    handler.call(());
                }
            },
            
            // Heure
            div {
                style: "width: 60px; text-align: center;",
                span {
                    style: "font-size: 16px; color: {c.text_white}; font-weight: 600;",
                    "{props.time}"
                }
            }
            
            // Icône
            span { style: "font-size: 24px;", "{icon}" }
            
            // Contenu
            div {
                style: "flex: 1;",
                h4 {
                    style: "font-size: 14px; color: {c.text_white}; margin-bottom: 4px;",
                    "{props.title}"
                }
                if let Some(loc) = &props.location {
                    p {
                        style: "font-size: 12px; color: {c.text_muted};",
                        "📍 {loc}"
                    }
                }
            }
            
            // Action
            button {
                style: "padding: 6px 12px; background: {c.bg_hover}; border: none; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 12px;",
                "Voir"
            }
        }
    }
}
