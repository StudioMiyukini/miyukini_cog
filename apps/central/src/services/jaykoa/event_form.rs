//! Formulaire de création/édition d'événement JayKoa.

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::state::use_app_state;
use jaykoa::data::{Agenda, TemporalEntry, EntryType, TemporalStatus};
use chrono::Local;

/// Props pour le modal de formulaire d'événement.
#[derive(Props, Clone, PartialEq)]
pub struct EventFormModalProps {
    /// Date/heure initiale (optionnelle, format ISO).
    pub initial_datetime: Option<String>,
    /// Liste des agendas disponibles.
    pub agendas: Vec<Agenda>,
    /// Callback pour fermer le modal.
    pub on_close: EventHandler<()>,
    /// Callback pour sauvegarder l'événement.
    pub on_save: EventHandler<TemporalEntry>,
}

/// Modal de création d'événement.
#[component]
pub fn EventFormModal(props: EventFormModalProps) -> Element {
    let state = use_app_state();
    let p = use_palette();
    
    // État du formulaire
    let mut title = use_signal(String::new);
    let mut description = use_signal(String::new);
    let mut location = use_signal(String::new);
    let mut all_day = use_signal(|| false);
    
    // Date et heure par défaut
    let default_date = props.initial_datetime.as_ref()
        .and_then(|s| s.get(..10).map(String::from))
        .unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());
    let default_time = props.initial_datetime.as_ref()
        .and_then(|s| s.get(11..16).map(String::from))
        .unwrap_or_else(|| "09:00".to_string());
    
    let mut start_date = use_signal(|| default_date.clone());
    let mut start_time = use_signal(|| default_time.clone());
    let mut end_date = use_signal(|| default_date.clone());
    let mut end_time = use_signal(|| {
        // Par défaut 1h après
        let hour: u32 = default_time[..2].parse().unwrap_or(9);
        format!("{:02}:00", (hour + 1).min(23))
    });
    
    // Agenda sélectionné
    let default_agenda = props.agendas.iter()
        .find(|a| a.is_default)
        .or_else(|| props.agendas.first())
        .and_then(|a| a.id.clone())
        .unwrap_or_default();
    let mut selected_agenda = use_signal(|| default_agenda);
    
    // Couleur
    let agenda_color = props.agendas.iter()
        .find(|a| a.id.as_ref() == Some(&selected_agenda.read().clone()))
        .and_then(|a| a.color.clone())
        .unwrap_or_else(|| "#4285F4".to_string());
    let mut selected_color = use_signal(|| agenda_color);
    
    // Validation
    let can_save = !title.read().is_empty();
    
    // Handler de sauvegarde
    let save = move |_| {
        let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        
        let (start_dt, end_dt) = if *all_day.read() {
            (
                format!("{}T00:00:00", start_date.read()),
                format!("{}T23:59:59", end_date.read()),
            )
        } else {
            (
                format!("{}T{}:00", start_date.read(), start_time.read()),
                format!("{}T{}:00", end_date.read(), end_time.read()),
            )
        };
        
        let entry = TemporalEntry {
            id: Some(uuid::Uuid::new_v4().to_string()),
            agenda_id: Some(selected_agenda.read().clone()),
            title: Some(title.read().clone()),
            description: if description.read().is_empty() { None } else { Some(description.read().clone()) },
            start_datetime: Some(start_dt),
            end_datetime: Some(end_dt),
            all_day: *all_day.read(),
            location: if location.read().is_empty() { None } else { Some(location.read().clone()) },
            status: Some(TemporalStatus::Confirmed.as_str().to_string()),
            entry_type: Some(EntryType::Internal.as_str().to_string()),
            source_service: None,
            source_event_id: None,
            color: Some(selected_color.read().clone()),
            recurrence_rule: None,
            reminders_json: None,
            created_at: Some(now.clone()),
            updated_at: Some(now),
            last_synced_at: None,
        };
        
        props.on_save.call(entry);
    };

    rsx! {
        // Backdrop
        div {
            style: "position: fixed; inset: 0; z-index: 1000; display: flex; align-items: center; justify-content: center; background: rgba(0,0,0,0.6);",
            onclick: move |_| props.on_close.call(()),
            
            // Modal
            div {
                style: "background: {p.bg_surface}; border-radius: 12px; width: 480px; max-width: 90vw; max-height: 90vh; overflow-y: auto; box-shadow: 0 16px 48px rgba(0,0,0,0.3);",
                onclick: move |e| e.stop_propagation(),
                
                // Header
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; padding: 16px 20px; border-bottom: 1px solid {p.border_default};",
                    
                    h2 {
                        style: "font-size: 18px; font-weight: 500; color: {p.text_high}; margin: 0;",
                        "Nouvel événement"
                    }
                    
                    button {
                        style: "width: 32px; height: 32px; display: flex; align-items: center; justify-content: center; background: transparent; border: none; color: {p.text_secondary}; cursor: pointer; border-radius: 4px; font-size: 18px;",
                        onclick: move |_| props.on_close.call(()),
                        "✕"
                    }
                }
                
                // Corps du formulaire
                div {
                    style: "padding: 20px;",
                    
                    // Titre
                    div {
                        style: "margin-bottom: 16px;",
                        input {
                            style: "width: 100%; padding: 12px; background: {p.bg_secondary}; border: 1px solid {p.border_default}; border-radius: 6px; color: {p.text_primary}; font-size: 16px; box-sizing: border-box;",
                            placeholder: "Ajouter un titre",
                            value: "{title}",
                            oninput: move |e| title.set(e.value()),
                        }
                    }
                    
                    // Journée entière
                    div {
                        style: "display: flex; align-items: center; gap: 8px; margin-bottom: 16px;",
                        input {
                            r#type: "checkbox",
                            checked: "{all_day}",
                            onchange: move |e| all_day.set(e.checked()),
                        }
                        label {
                            style: "font-size: 13px; color: {p.text_secondary};",
                            "Journée entière"
                        }
                    }
                    
                    // Date et heure de début
                    div {
                        style: "display: flex; gap: 12px; margin-bottom: 12px;",
                        
                        div {
                            style: "flex: 1;",
                            label {
                                style: "display: block; font-size: 11px; color: {p.text_muted}; margin-bottom: 4px;",
                                "Début"
                            }
                            input {
                                r#type: "date",
                                style: "width: 100%; padding: 10px; background: {p.bg_secondary}; border: 1px solid {p.border_default}; border-radius: 6px; color: {p.text_primary}; font-size: 13px; box-sizing: border-box;",
                                value: "{start_date}",
                                oninput: move |e| start_date.set(e.value()),
                            }
                        }
                        
                        if !*all_day.read() {
                            div {
                                style: "width: 120px;",
                                label {
                                    style: "display: block; font-size: 11px; color: {p.text_muted}; margin-bottom: 4px;",
                                    "Heure"
                                }
                                input {
                                    r#type: "time",
                                    style: "width: 100%; padding: 10px; background: {p.bg_secondary}; border: 1px solid {p.border_default}; border-radius: 6px; color: {p.text_primary}; font-size: 13px; box-sizing: border-box;",
                                    value: "{start_time}",
                                    oninput: move |e| start_time.set(e.value()),
                                }
                            }
                        }
                    }
                    
                    // Date et heure de fin
                    div {
                        style: "display: flex; gap: 12px; margin-bottom: 16px;",
                        
                        div {
                            style: "flex: 1;",
                            label {
                                style: "display: block; font-size: 11px; color: {p.text_muted}; margin-bottom: 4px;",
                                "Fin"
                            }
                            input {
                                r#type: "date",
                                style: "width: 100%; padding: 10px; background: {p.bg_secondary}; border: 1px solid {p.border_default}; border-radius: 6px; color: {p.text_primary}; font-size: 13px; box-sizing: border-box;",
                                value: "{end_date}",
                                oninput: move |e| end_date.set(e.value()),
                            }
                        }
                        
                        if !*all_day.read() {
                            div {
                                style: "width: 120px;",
                                label {
                                    style: "display: block; font-size: 11px; color: {p.text_muted}; margin-bottom: 4px;",
                                    "Heure"
                                }
                                input {
                                    r#type: "time",
                                    style: "width: 100%; padding: 10px; background: {p.bg_secondary}; border: 1px solid {p.border_default}; border-radius: 6px; color: {p.text_primary}; font-size: 13px; box-sizing: border-box;",
                                    value: "{end_time}",
                                    oninput: move |e| end_time.set(e.value()),
                                }
                            }
                        }
                    }
                    
                    // Lieu
                    div {
                        style: "margin-bottom: 16px;",
                        label {
                            style: "display: flex; align-items: center; gap: 8px; font-size: 11px; color: {p.text_muted}; margin-bottom: 4px;",
                            span { "📍" }
                            "Lieu"
                        }
                        input {
                            style: "width: 100%; padding: 10px; background: {p.bg_secondary}; border: 1px solid {p.border_default}; border-radius: 6px; color: {p.text_primary}; font-size: 13px; box-sizing: border-box;",
                            placeholder: "Ajouter un lieu",
                            value: "{location}",
                            oninput: move |e| location.set(e.value()),
                        }
                    }
                    
                    // Description
                    div {
                        style: "margin-bottom: 16px;",
                        label {
                            style: "display: flex; align-items: center; gap: 8px; font-size: 11px; color: {p.text_muted}; margin-bottom: 4px;",
                            span { "≡" }
                            "Description"
                        }
                        textarea {
                            style: "width: 100%; padding: 10px; background: {p.bg_secondary}; border: 1px solid {p.border_default}; border-radius: 6px; color: {p.text_primary}; font-size: 13px; box-sizing: border-box; min-height: 80px; resize: vertical;",
                            placeholder: "Ajouter une description",
                            value: "{description}",
                            oninput: move |e| description.set(e.value()),
                        }
                    }
                    
                    // Sélecteur d'agenda
                    div {
                        style: "margin-bottom: 16px;",
                        label {
                            style: "display: flex; align-items: center; gap: 8px; font-size: 11px; color: {p.text_muted}; margin-bottom: 4px;",
                            span { "📅" }
                            "Agenda"
                        }
                        select {
                            style: "width: 100%; padding: 10px; background: {p.bg_secondary}; border: 1px solid {p.border_default}; border-radius: 6px; color: {p.text_primary}; font-size: 13px;",
                            value: "{selected_agenda}",
                            onchange: move |e| {
                                selected_agenda.set(e.value());
                                // Mettre à jour la couleur
                                if let Some(agenda) = props.agendas.iter().find(|a| a.id.as_deref() == Some(&e.value())) {
                                    if let Some(color) = &agenda.color {
                                        selected_color.set(color.clone());
                                    }
                                }
                            },
                            for agenda in props.agendas.iter().filter(|a| a.source_service.is_none()) {
                                option {
                                    value: "{agenda.id.as_deref().unwrap_or(\"\")}",
                                    "{agenda.name.as_deref().unwrap_or(\"Sans nom\")}"
                                }
                            }
                        }
                    }
                    
                    // Sélecteur de couleur
                    div {
                        style: "margin-bottom: 24px;",
                        label {
                            style: "display: flex; align-items: center; gap: 8px; font-size: 11px; color: {p.text_muted}; margin-bottom: 8px;",
                            span { "🎨" }
                            "Couleur"
                        }
                        div {
                            style: "display: flex; gap: 8px; flex-wrap: wrap;",
                            for color in ["#4285F4", "#33B679", "#E67C73", "#F6BF26", "#8E24AA", "#039BE5", "#7986CB", "#616161"] {
                                {
                                    let is_selected = *selected_color.read() == color;
                                    let border_style = if is_selected { "3px solid white".to_string() } else { "none".to_string() };
                                    let shadow_style = if is_selected { format!("0 0 0 2px {color}") } else { "none".to_string() };
                                    rsx! {
                                        button {
                                            key: "{color}",
                                            style: "width: 28px; height: 28px; border-radius: 50%; background: {color}; border: {border_style}; cursor: pointer; box-shadow: {shadow_style};",
                                            onclick: move |_| selected_color.set(color.to_string()),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Footer
                div {
                    style: "display: flex; justify-content: flex-end; gap: 12px; padding: 16px 20px; border-top: 1px solid {p.border_default};",
                    
                    button {
                        style: "padding: 10px 20px; background: transparent; border: 1px solid {p.border_default}; border-radius: 6px; color: {p.text_secondary}; font-size: 13px; cursor: pointer;",
                        onclick: move |_| props.on_close.call(()),
                        "Annuler"
                    }
                    
                    {
                        let save_bg = if can_save { p.accent_primary.to_string() } else { p.bg_overlay.to_string() };
                        let save_cursor = if can_save { "pointer" } else { "not-allowed" };
                        rsx! {
                            button {
                                style: "padding: 10px 24px; background: {save_bg}; border: none; border-radius: 6px; color: white; font-size: 13px; cursor: {save_cursor}; font-weight: 500;",
                                disabled: !can_save,
                                onclick: save,
                                "Enregistrer"
                            }
                        }
                    }
                }
            }
        }
    }
}
