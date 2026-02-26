//! EXP-E05/E06 — Participations exposant (liste et détail).

use dioxus::prelude::*;
use miyukini_service_ui::use_palette;
use super::components::{Badge, ActionButton};
#[allow(unused_imports)]
use super::components::EmptyState;

fn opt_str(s: &Option<String>) -> String {
    s.clone().unwrap_or_default()
}

/// Liste des participations de l'exposant.
#[component]
pub fn ExpParticipations() -> Element {
    let c = use_palette();
    let mut selected_participation = use_signal(|| None::<String>);

    // Charger l'exposant courant (premier de la DB pour la démo)
    let exposant = {
        let db = crate::use_db();
        db.exposants_list(false).unwrap_or_default().into_iter().next()
    };

    let exp_id = exposant.as_ref().and_then(|e| e.id.clone()).unwrap_or_default();

    // Charger les participations de cet exposant via les éditions
    let participations: Vec<jayfestival::data::EditionExposant> = {
        let db = crate::use_db();
        // Récupérer toutes les éditions puis filtrer les participations
        let editions = db.editions_list().unwrap_or_default();
        let mut all_participations = Vec::new();
        for ed in editions {
            if let Some(ref eid) = ed.id {
                if let Ok(parts) = db.editions_exposants_by_edition(eid) {
                    for p in parts {
                        if p.exposant_id.as_deref() == Some(exp_id.as_str()) {
                            all_participations.push(p);
                        }
                    }
                }
            }
        }
        all_participations
    };

    // Si une participation est sélectionnée, afficher le détail
    if let Some(part_id) = selected_participation.read().clone() {
        return rsx! {
            ParticipationDetail {
                participation_id: part_id,
                on_back: move |()| { selected_participation.set(None); },
            }
        };
    }

    // Séparer les participations par statut
    let confirmed: Vec<_> = participations
        .iter()
        .filter(|p| p.status_candidature.as_deref() == Some("acceptee"))
        .collect();

    let pending: Vec<_> = participations
        .iter()
        .filter(|p| p.status_candidature.as_deref() == Some("en_attente"))
        .collect();

    let past: Vec<_> = participations
        .iter()
        .filter(|p| p.status_candidature.as_deref() == Some("terminee"))
        .collect();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            h2 {
                style: "font-size: 24px; color: {c.text_white};",
                "Mes participations"
            }

            // Participations confirmées
            section {
                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 12px;",
                    "Participations confirmees ({confirmed.len()})"
                }

                if confirmed.is_empty() {
                    div {
                        style: "padding: 20px; background: {c.bg_secondary}; border-radius: 8px; color: {c.text_muted}; text-align: center; font-size: 14px;",
                        "Aucune participation confirmee"
                    }
                }

                for p in confirmed.iter() {
                    {
                        let pid = opt_str(&p.id);
                        let edition_id = opt_str(&p.edition_id);
                        let stand = opt_str(&p.assigned_stand);
                        let stand_display = if stand.is_empty() { "Non attribue".to_string() } else { stand };
                        let pid_click = pid.clone();
                        rsx! {
                            ParticipationCard {
                                edition_id: edition_id,
                                stand: stand_display,
                                status: "confirmee",
                                onclick: move |_| { selected_participation.set(Some(pid_click.clone())); },
                            }
                        }
                    }
                }
            }

            // Candidatures en attente
            section {
                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 12px;",
                    "Candidatures en attente ({pending.len()})"
                }

                if pending.is_empty() {
                    div {
                        style: "padding: 20px; background: {c.bg_secondary}; border-radius: 8px; color: {c.text_muted}; text-align: center; font-size: 14px;",
                        "Aucune candidature en attente"
                    }
                }

                for p in pending.iter() {
                    {
                        let pid = opt_str(&p.id);
                        let edition_id = opt_str(&p.edition_id);
                        let pid_click = pid.clone();
                        rsx! {
                            ParticipationCard {
                                edition_id: edition_id,
                                stand: "En attente".to_string(),
                                status: "en_attente",
                                onclick: move |_| { selected_participation.set(Some(pid_click.clone())); },
                            }
                        }
                    }
                }
            }

            // Participations passées
            if !past.is_empty() {
                section {
                    h3 {
                        style: "font-size: 16px; color: {c.text_white}; margin-bottom: 12px;",
                        "Participations passees ({past.len()})"
                    }

                    for p in past.iter() {
                        {
                            let pid = opt_str(&p.id);
                            let edition_id = opt_str(&p.edition_id);
                            let stand = opt_str(&p.assigned_stand);
                            let pid_click = pid.clone();
                            rsx! {
                                ParticipationCard {
                                    edition_id: edition_id,
                                    stand: stand,
                                    status: "terminee",
                                    onclick: move |_| { selected_participation.set(Some(pid_click.clone())); },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ParticipationCard(
    edition_id: String,
    stand: String,
    status: &'static str,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let c = use_palette();

    // Charger les infos de l'édition
    let edition = {
        let db = crate::use_db();
        db.edition_by_id(&edition_id).ok().flatten()
    };

    let name = edition.as_ref().and_then(|e| e.name.clone()).unwrap_or_else(|| "Evenement inconnu".to_string());
    let location = edition.as_ref().and_then(|e| e.location.clone()).unwrap_or_default();
    let date = edition.as_ref().and_then(|e| e.start_date.clone()).unwrap_or_default();

    let (status_label, status_color) = match status {
        "confirmee" => ("Confirmee", c.accent_green),
        "en_attente" => ("En attente", c.accent_orange),
        "terminee" => ("Terminee", c.text_muted),
        _ => ("Inconnu", c.text_muted),
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px; background: {c.bg_secondary}; border-radius: 8px; padding: 16px; cursor: pointer;",
            onclick: move |evt| onclick.call(evt),

            span { style: "font-size: 28px;", "🎪" }

            div {
                style: "flex: 1;",
                p {
                    style: "font-size: 14px; color: {c.text_white}; font-weight: 500; margin-bottom: 4px;",
                    "{name}"
                }
                div {
                    style: "display: flex; gap: 16px; font-size: 12px; color: {c.text_muted};",
                    if !date.is_empty() {
                        span { "📆 {date}" }
                    }
                    if !location.is_empty() {
                        span { "📍 {location}" }
                    }
                    span { "🪑 {stand}" }
                }
            }

            Badge {
                text: status_label.to_string(),
                color: status_color.to_string(),
            }
        }
    }
}

/// Détail d'une participation.
#[component]
fn ParticipationDetail(participation_id: String, on_back: EventHandler<()>) -> Element {
    let c = use_palette();

    // Charger la participation
    let participation = {
        let db = crate::use_db();
        // Note: On devrait avoir une méthode edition_exposant_by_id
        db.editions_exposants_by_edition(&participation_id).ok().and_then(|v| v.into_iter().next())
    };

    // Charger l'édition associée
    let edition_id = participation.as_ref().and_then(|p| p.edition_id.clone()).unwrap_or_default();
    let edition = {
        let db = crate::use_db();
        db.edition_by_id(&edition_id).ok().flatten()
    };

    let name = edition.as_ref().and_then(|e| e.name.clone()).unwrap_or_else(|| "Evenement".to_string());
    let location = edition.as_ref().and_then(|e| e.location.clone()).unwrap_or_default();
    let date = edition.as_ref().and_then(|e| e.start_date.clone()).unwrap_or_default();
    let stand = participation.as_ref().and_then(|p| p.assigned_stand.clone()).unwrap_or_else(|| "Non attribue".to_string());
    let status = participation.as_ref().and_then(|p| p.status_candidature.clone()).unwrap_or_default();

    let status_color = match status.as_str() {
        "acceptee" => c.accent_green.to_string(),
        "en_attente" => c.accent_orange.to_string(),
        _ => c.text_muted.to_string(),
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // Retour
            button {
                style: "display: flex; align-items: center; gap: 8px; padding: 8px 0; background: none; border: none; color: {c.accent_blue}; cursor: pointer; font-size: 13px;",
                onclick: move |_| on_back.call(()),
                "← Retour aux participations"
            }

            // En-tête
            div {
                style: "display: flex; justify-content: space-between; align-items: flex-start;",

                div {
                    h2 {
                        style: "font-size: 24px; color: {c.text_white}; margin-bottom: 8px;",
                        "{name}"
                    }
                    div {
                        style: "display: flex; gap: 12px; font-size: 13px; color: {c.text_muted};",
                        if !date.is_empty() {
                            span { "📆 {date}" }
                        }
                        if !location.is_empty() {
                            span { "📍 {location}" }
                        }
                    }
                }

                Badge {
                    text: status.clone(),
                    color: status_color,
                }
            }

            // Informations stand
            section {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                    "Mon stand"
                }

                div {
                    style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",

                    InfoField { label: "Emplacement", value: stand }
                    InfoField { label: "Surface", value: "9 m²".to_string() }
                    InfoField { label: "Type", value: "Standard".to_string() }
                    InfoField { label: "Electricite", value: "220V inclus".to_string() }
                }
            }

            // Documents
            section {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                    "Documents"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    DocumentItem { name: "Contrat de participation", status: "signe" }
                    DocumentItem { name: "Fiche technique stand", status: "a_remplir" }
                    DocumentItem { name: "Reglement interieur", status: "lu" }
                }
            }

            // Actions
            div {
                style: "display: flex; gap: 12px;",

                ActionButton {
                    label: "Contacter l'organisateur".to_string(),
                    icon: "✉️".to_string(),
                    accent: true,
                    onclick: move |_| {},
                }
                ActionButton {
                    label: "Voir le plan".to_string(),
                    icon: "🗺️".to_string(),
                    accent: false,
                    onclick: move |_| {},
                }
            }
        }
    }
}

#[component]
fn InfoField(label: &'static str, value: String) -> Element {
    let c = use_palette();

    rsx! {
        div {
            span {
                style: "font-size: 12px; color: {c.text_muted}; display: block; margin-bottom: 4px;",
                "{label}"
            }
            span {
                style: "font-size: 14px; color: {c.text_primary};",
                "{value}"
            }
        }
    }
}

#[component]
fn DocumentItem(name: &'static str, status: &'static str) -> Element {
    let c = use_palette();

    let (icon, status_label, status_color) = match status {
        "signe" => ("✅", "Signe", c.accent_green),
        "a_remplir" => ("📝", "A remplir", c.accent_orange),
        "lu" => ("👁️", "Lu", c.text_muted),
        _ => ("📄", "En attente", c.text_muted),
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px; padding: 12px; background: {c.bg_main}; border-radius: 6px;",

            span { style: "font-size: 16px;", "{icon}" }

            span {
                style: "flex: 1; font-size: 13px; color: {c.text_primary};",
                "{name}"
            }

            Badge {
                text: status_label.to_string(),
                color: status_color.to_string(),
            }

            button {
                style: "padding: 6px 12px; background: {c.bg_hover}; border: none; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 12px;",
                "Voir"
            }
        }
    }
}
