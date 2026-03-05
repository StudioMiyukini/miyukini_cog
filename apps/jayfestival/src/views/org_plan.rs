//! ORG-E14/E15/E16 — Plan de salle complet.
//! - E14: Définition des zones
//! - E15: Attribution des stands
//! - E16: Visualisation et export

use super::components::{ActionButton, Badge};
use dioxus::prelude::*;
use miyukini_service_ui::use_palette;

/// Vue principale du plan de salle avec onglets.
#[component]
pub fn OrgPlan(edition_id: String) -> Element {
    let c = use_palette();
    let mut active_tab = use_signal(|| "visualisation".to_string());

    let tab = active_tab.read().clone();
    let tab_visu_bg = if tab == "visualisation" {
        c.accent_blue
    } else {
        c.bg_secondary
    };
    let tab_visu_color = if tab == "visualisation" {
        "white"
    } else {
        c.text_primary
    };
    let tab_zones_bg = if tab == "zones" {
        c.accent_blue
    } else {
        c.bg_secondary
    };
    let tab_zones_color = if tab == "zones" {
        "white"
    } else {
        c.text_primary
    };
    let tab_attrib_bg = if tab == "attribution" {
        c.accent_blue
    } else {
        c.bg_secondary
    };
    let tab_attrib_color = if tab == "attribution" {
        "white"
    } else {
        c.text_primary
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // En-tête
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                h3 {
                    style: "font-size: 18px; color: {c.text_white};",
                    "Plan de salle"
                }

                div {
                    style: "display: flex; gap: 8px;",
                    ActionButton {
                        label: "Exporter PDF".to_string(),
                        icon: "📄".to_string(),
                        accent: false,
                        onclick: move |_| {},
                    }
                    ActionButton {
                        label: "Exporter PNG".to_string(),
                        icon: "🖼️".to_string(),
                        accent: false,
                        onclick: move |_| {},
                    }
                }
            }

            // Onglets
            div {
                style: "display: flex; gap: 8px; border-bottom: 1px solid {c.border}; padding-bottom: 12px;",

                button {
                    style: "padding: 8px 16px; background: {tab_visu_bg}; border: none; border-radius: 6px; color: {tab_visu_color}; cursor: pointer; font-size: 13px;",
                    onclick: move |_| { active_tab.set("visualisation".to_string()); },
                    "Visualisation"
                }
                button {
                    style: "padding: 8px 16px; background: {tab_zones_bg}; border: none; border-radius: 6px; color: {tab_zones_color}; cursor: pointer; font-size: 13px;",
                    onclick: move |_| { active_tab.set("zones".to_string()); },
                    "Zones"
                }
                button {
                    style: "padding: 8px 16px; background: {tab_attrib_bg}; border: none; border-radius: 6px; color: {tab_attrib_color}; cursor: pointer; font-size: 13px;",
                    onclick: move |_| { active_tab.set("attribution".to_string()); },
                    "Attribution"
                }
            }

            // Contenu selon l'onglet
            if tab == "visualisation" {
                PlanVisualisationTab { edition_id: edition_id.clone() }
            }
            if tab == "zones" {
                PlanZonesTab { edition_id: edition_id.clone() }
            }
            if tab == "attribution" {
                PlanAttributionTab { edition_id: edition_id.clone() }
            }
        }
    }
}

/// ORG-E16: Visualisation du plan.
#[component]
fn PlanVisualisationTab(edition_id: String) -> Element {
    let c = use_palette();
    // Charger les exposants de l'édition pour la légende
    let participations = {
        let db = crate::use_db();
        db.editions_exposants_by_edition(&edition_id)
            .unwrap_or_default()
    };

    let assigned_count = participations
        .iter()
        .filter(|p| p.assigned_stand.is_some())
        .count();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px;",

            // Légende
            div {
                style: "display: flex; gap: 16px; font-size: 12px;",

                LegendItem { color: c.accent_green.to_string(), label: format!("Attribue ({})", assigned_count) }
                LegendItem { color: c.accent_orange.to_string(), label: "Reserve".to_string() }
                LegendItem { color: c.bg_hover.to_string(), label: "Libre".to_string() }
            }

            // Grille du plan
            PlanGrid {}

            // Info
            p {
                style: "font-size: 13px; color: {c.text_muted}; font-style: italic;",
                "Cliquez sur un stand pour voir les details ou l'attribuer."
            }
        }
    }
}

#[component]
fn LegendItem(color: String, label: String) -> Element {
    let c = use_palette();
    let bg_color = format!("{color}40");

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 6px;",
            div {
                style: "width: 14px; height: 14px; background: {bg_color}; border: 1px solid {color}; border-radius: 2px;",
            }
            span {
                style: "color: {c.text_secondary};",
                "{label}"
            }
        }
    }
}

#[component]
fn PlanGrid() -> Element {
    let selected_stand = use_signal(|| None::<String>);

    rsx! {
        div {
            style: "display: grid; grid-template-columns: repeat(6, 1fr); gap: 8px; max-width: 600px;",

            // Rangée A
            StandCell { id: "A1", status: "libre", selected: selected_stand }
            StandCell { id: "A2", status: "libre", selected: selected_stand }
            StandCell { id: "A3", status: "libre", selected: selected_stand }
            StandCell { id: "A4", status: "libre", selected: selected_stand }
            StandCell { id: "A5", status: "libre", selected: selected_stand }
            StandCell { id: "A6", status: "libre", selected: selected_stand }

            // Rangée B
            StandCell { id: "B1", status: "libre", selected: selected_stand }
            StandCell { id: "B2", status: "reserve", selected: selected_stand }
            StandCell { id: "B3", status: "reserve", selected: selected_stand }
            StandCell { id: "B4", status: "libre", selected: selected_stand }
            StandCell { id: "B5", status: "libre", selected: selected_stand }
            StandCell { id: "B6", status: "libre", selected: selected_stand }

            // Rangée C
            StandCell { id: "C1", status: "attribue", selected: selected_stand }
            StandCell { id: "C2", status: "attribue", selected: selected_stand }
            StandCell { id: "C3", status: "libre", selected: selected_stand }
            StandCell { id: "C4", status: "libre", selected: selected_stand }
            StandCell { id: "C5", status: "reserve", selected: selected_stand }
            StandCell { id: "C6", status: "libre", selected: selected_stand }

            // Rangée D
            StandCell { id: "D1", status: "libre", selected: selected_stand }
            StandCell { id: "D2", status: "libre", selected: selected_stand }
            StandCell { id: "D3", status: "attribue", selected: selected_stand }
            StandCell { id: "D4", status: "libre", selected: selected_stand }
            StandCell { id: "D5", status: "libre", selected: selected_stand }
            StandCell { id: "D6", status: "libre", selected: selected_stand }
        }
    }
}

#[component]
fn StandCell(id: &'static str, status: &'static str, selected: Signal<Option<String>>) -> Element {
    let c = use_palette();
    let is_selected = selected.read().as_deref() == Some(id);

    let (bg, border_color) = match status {
        "attribue" => (format!("{}40", c.accent_green), c.accent_green.to_string()),
        "reserve" => (
            format!("{}40", c.accent_orange),
            c.accent_orange.to_string(),
        ),
        _ => (c.bg_hover.to_string(), c.border.to_string()),
    };

    let border = if is_selected {
        format!("2px solid {}", c.accent_blue)
    } else {
        format!("1px solid {border_color}")
    };

    rsx! {
        div {
            style: "aspect-ratio: 1; background: {bg}; border: {border}; border-radius: 4px; display: flex; align-items: center; justify-content: center; cursor: pointer; font-size: 12px; color: {c.text_secondary}; font-weight: 500; transition: all 0.2s;",
            onclick: move |_| {
                selected.set(Some(id.to_string()));
            },
            "{id}"
        }
    }
}

/// ORG-E14: Définition des zones.
#[component]
fn PlanZonesTab(edition_id: String) -> Element {
    let c = use_palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // Actions
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                p {
                    style: "font-size: 14px; color: {c.text_secondary};",
                    "Definissez les differentes zones de votre espace d'exposition."
                }

                ActionButton {
                    label: "Ajouter une zone".to_string(),
                    icon: "➕".to_string(),
                    accent: true,
                    onclick: move |_| {},
                }
            }

            // Liste des zones
            div {
                style: "display: flex; flex-direction: column; gap: 12px;",

                ZoneCard {
                    name: "Zone A - Entree principale",
                    stands_count: 6,
                    surface: "120 m²",
                    color: "#4CAF50",
                }
                ZoneCard {
                    name: "Zone B - Espace central",
                    stands_count: 6,
                    surface: "150 m²",
                    color: "#2196F3",
                }
                ZoneCard {
                    name: "Zone C - Espace premium",
                    stands_count: 6,
                    surface: "100 m²",
                    color: "#9C27B0",
                }
                ZoneCard {
                    name: "Zone D - Espace artisans",
                    stands_count: 6,
                    surface: "80 m²",
                    color: "#FF9800",
                }
            }

            // Configuration globale
            section {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px; margin-top: 16px;",

                h4 {
                    style: "font-size: 14px; color: {c.text_white}; margin-bottom: 16px;",
                    "Configuration globale"
                }

                div {
                    style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",

                    div {
                        label {
                            style: "font-size: 12px; color: {c.text_muted}; display: block; margin-bottom: 4px;",
                            "Surface totale"
                        }
                        input {
                            r#type: "text",
                            style: "width: 100%; padding: 8px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; font-size: 13px;",
                            value: "450 m²",
                        }
                    }
                    div {
                        label {
                            style: "font-size: 12px; color: {c.text_muted}; display: block; margin-bottom: 4px;",
                            "Nombre total de stands"
                        }
                        input {
                            r#type: "number",
                            style: "width: 100%; padding: 8px 12px; background: {c.bg_main}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_primary}; font-size: 13px;",
                            value: "24",
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ZoneCard(
    name: &'static str,
    stands_count: usize,
    surface: &'static str,
    color: &'static str,
) -> Element {
    let c = use_palette();

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 16px; background: {c.bg_secondary}; border-radius: 8px; padding: 16px; border-left: 4px solid {color};",

            div {
                style: "flex: 1;",
                h4 {
                    style: "font-size: 14px; color: {c.text_white}; margin-bottom: 4px;",
                    "{name}"
                }
                div {
                    style: "display: flex; gap: 16px; font-size: 12px; color: {c.text_muted};",
                    span { "{stands_count} stands" }
                    span { "{surface}" }
                }
            }

            div {
                style: "display: flex; gap: 8px;",
                button {
                    style: "padding: 6px 12px; background: {c.bg_hover}; border: none; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 12px;",
                    "Modifier"
                }
                button {
                    style: "padding: 6px 12px; background: transparent; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_muted}; cursor: pointer; font-size: 12px;",
                    "Supprimer"
                }
            }
        }
    }
}

/// ORG-E15: Attribution des stands.
#[component]
fn PlanAttributionTab(edition_id: String) -> Element {
    let c = use_palette();
    // Charger les participations
    let participations = {
        let db = crate::use_db();
        db.editions_exposants_by_edition(&edition_id)
            .unwrap_or_default()
    };

    // Charger les noms des exposants
    let exposants_map: std::collections::HashMap<String, String> = {
        let db = crate::use_db();
        db.exposants_list(false)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|e| {
                let id = e.id?;
                let name = e.company_name.unwrap_or_else(|| "Sans nom".to_string());
                Some((id, name))
            })
            .collect()
    };

    let accepted: Vec<_> = participations
        .iter()
        .filter(|p| p.status_candidature.as_deref() == Some("acceptee"))
        .collect();

    let assigned_count = accepted
        .iter()
        .filter(|p| p.assigned_stand.is_some())
        .count();
    let unassigned_count = accepted.len() - assigned_count;

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // Stats
            div {
                style: "display: flex; gap: 16px;",

                div {
                    style: "background: {c.bg_secondary}; border-radius: 8px; padding: 16px; flex: 1; text-align: center;",
                    p {
                        style: "font-size: 24px; color: {c.accent_green}; font-weight: 600;",
                        "{assigned_count}"
                    }
                    p {
                        style: "font-size: 12px; color: {c.text_muted};",
                        "Stands attribues"
                    }
                }
                div {
                    style: "background: {c.bg_secondary}; border-radius: 8px; padding: 16px; flex: 1; text-align: center;",
                    p {
                        style: "font-size: 24px; color: {c.accent_orange}; font-weight: 600;",
                        "{unassigned_count}"
                    }
                    p {
                        style: "font-size: 12px; color: {c.text_muted};",
                        "En attente"
                    }
                }
            }

            // Instructions
            p {
                style: "font-size: 14px; color: {c.text_secondary};",
                "Selectionnez un exposant puis cliquez sur un stand libre pour l'attribuer."
            }

            // Liste des exposants à attribuer
            h4 {
                style: "font-size: 16px; color: {c.text_white};",
                "Exposants en attente d'attribution"
            }

            if unassigned_count == 0 {
                div {
                    style: "padding: 20px; background: {c.bg_secondary}; border-radius: 8px; color: {c.text_muted}; text-align: center; font-size: 14px;",
                    "Tous les exposants ont un stand attribue"
                }
            }

            for p in accepted.iter().filter(|p| p.assigned_stand.is_none()) {
                {
                    let name = p.exposant_id.as_ref()
                        .and_then(|eid| exposants_map.get(eid))
                        .cloned()
                        .unwrap_or_else(|| "Exposant inconnu".to_string());
                    rsx! {
                        div {
                            style: "display: flex; align-items: center; gap: 12px; background: {c.bg_secondary}; border-radius: 8px; padding: 12px 16px;",

                            span {
                                style: "flex: 1; font-size: 14px; color: {c.text_white};",
                                "{name}"
                            }

                            Badge {
                                text: "Sans stand".to_string(),
                                color: c.accent_orange.to_string(),
                            }

                            button {
                                style: "padding: 6px 12px; background: {c.accent_blue}; border: none; border-radius: 4px; color: white; cursor: pointer; font-size: 12px;",
                                "Attribuer"
                            }
                        }
                    }
                }
            }

            // Exposants avec stand
            h4 {
                style: "font-size: 16px; color: {c.text_white}; margin-top: 16px;",
                "Exposants avec stand attribue"
            }

            for p in accepted.iter().filter(|p| p.assigned_stand.is_some()) {
                {
                    let name = p.exposant_id.as_ref()
                        .and_then(|eid| exposants_map.get(eid))
                        .cloned()
                        .unwrap_or_else(|| "Exposant inconnu".to_string());
                    let stand = p.assigned_stand.clone().unwrap_or_default();
                    rsx! {
                        div {
                            style: "display: flex; align-items: center; gap: 12px; background: {c.bg_secondary}; border-radius: 8px; padding: 12px 16px;",

                            span {
                                style: "flex: 1; font-size: 14px; color: {c.text_white};",
                                "{name}"
                            }

                            Badge {
                                text: format!("Stand {}", stand),
                                color: c.accent_green.to_string(),
                            }

                            button {
                                style: "padding: 6px 12px; background: {c.bg_hover}; border: none; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 12px;",
                                "Modifier"
                            }
                        }
                    }
                }
            }
        }
    }
}
