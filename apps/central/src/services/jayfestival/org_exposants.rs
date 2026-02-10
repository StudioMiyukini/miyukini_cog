//! ORG-E09/E10/E11/E12/E13/E18 — Exposants d'une édition.
//! - E09: Liste des candidatures
//! - E10: Liste des exposants confirmés
//! - E11: Fiche détaillée exposant
//! - E12: Génération devis
//! - E13: Vue factures
//! - E18: Import CSV

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::components::{Badge, ActionButton};

fn opt_str(s: &Option<String>) -> String {
    s.clone().unwrap_or_default()
}

/// Vue principale des exposants avec onglets.
#[component]
pub fn OrgExposants(edition_id: String) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let mut active_tab = use_signal(|| "candidatures".to_string());
    let mut selected_exposant = use_signal(|| None::<String>);

    // Si un exposant est sélectionné, afficher sa fiche
    if let Some(exp_id) = selected_exposant.read().clone() {
        return rsx! {
            ExposantFiche {
                edition_id: edition_id.clone(),
                exposant_id: exp_id,
                on_back: move |_| { selected_exposant.set(None); },
            }
        };
    }

    let tab = active_tab.read().clone();
    let tab_cand_bg = if tab == "candidatures" { c.accent_blue } else { c.bg_secondary };
    let tab_cand_color = if tab == "candidatures" { "white" } else { c.text_primary };
    let tab_conf_bg = if tab == "confirmes" { c.accent_blue } else { c.bg_secondary };
    let tab_conf_color = if tab == "confirmes" { "white" } else { c.text_primary };
    let tab_devis_bg = if tab == "devis" { c.accent_blue } else { c.bg_secondary };
    let tab_devis_color = if tab == "devis" { "white" } else { c.text_primary };
    let tab_factures_bg = if tab == "factures" { c.accent_blue } else { c.bg_secondary };
    let tab_factures_color = if tab == "factures" { "white" } else { c.text_primary };
    let tab_import_bg = if tab == "import" { c.accent_blue } else { c.bg_secondary };
    let tab_import_color = if tab == "import" { "white" } else { c.text_primary };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // Onglets
            div {
                style: "display: flex; gap: 8px; border-bottom: 1px solid {c.border}; padding-bottom: 12px;",

                button {
                    style: "padding: 8px 16px; background: {tab_cand_bg}; border: none; border-radius: 6px; color: {tab_cand_color}; cursor: pointer; font-size: 13px;",
                    onclick: move |_| { active_tab.set("candidatures".to_string()); },
                    "Candidatures"
                }
                button {
                    style: "padding: 8px 16px; background: {tab_conf_bg}; border: none; border-radius: 6px; color: {tab_conf_color}; cursor: pointer; font-size: 13px;",
                    onclick: move |_| { active_tab.set("confirmes".to_string()); },
                    "Confirmes"
                }
                button {
                    style: "padding: 8px 16px; background: {tab_devis_bg}; border: none; border-radius: 6px; color: {tab_devis_color}; cursor: pointer; font-size: 13px;",
                    onclick: move |_| { active_tab.set("devis".to_string()); },
                    "Devis"
                }
                button {
                    style: "padding: 8px 16px; background: {tab_factures_bg}; border: none; border-radius: 6px; color: {tab_factures_color}; cursor: pointer; font-size: 13px;",
                    onclick: move |_| { active_tab.set("factures".to_string()); },
                    "Factures"
                }
                button {
                    style: "padding: 8px 16px; background: {tab_import_bg}; border: none; border-radius: 6px; color: {tab_import_color}; cursor: pointer; font-size: 13px;",
                    onclick: move |_| { active_tab.set("import".to_string()); },
                    "Import CSV"
                }
            }

            // Contenu selon l'onglet
            if tab == "candidatures" {
                CandidaturesTab {
                    edition_id: edition_id.clone(),
                    on_select: move |id| { selected_exposant.set(Some(id)); },
                }
            }
            if tab == "confirmes" {
                ConfirmesTab {
                    edition_id: edition_id.clone(),
                    on_select: move |id| { selected_exposant.set(Some(id)); },
                }
            }
            if tab == "devis" {
                DevisTab { edition_id: edition_id.clone() }
            }
            if tab == "factures" {
                FacturesTab { edition_id: edition_id.clone() }
            }
            if tab == "import" {
                ImportCsvTab { edition_id: edition_id.clone() }
            }
        }
    }
}

/// ORG-E09: Candidatures en attente.
#[component]
fn CandidaturesTab(edition_id: String, on_select: EventHandler<String>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();

    let participations = {
        let db = &conns.read().jayfestival;
        db.editions_exposants_by_edition(&edition_id).unwrap_or_default()
    };

    let exposants_map: std::collections::HashMap<String, String> = {
        let db = &conns.read().jayfestival;
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

    let pending: Vec<_> = participations
        .iter()
        .filter(|p| p.status_candidature.as_deref() == Some("en_attente"))
        .collect();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 16px;",

            h3 {
                style: "font-size: 18px; color: {c.text_white};",
                "Candidatures en attente ({pending.len()})"
            }

            if pending.is_empty() {
                div {
                    style: "padding: 40px; background: {c.bg_secondary}; border-radius: 8px; color: {c.text_muted}; text-align: center; font-size: 14px;",
                    "Aucune candidature en attente"
                }
            }

            for p in pending.iter() {
                {
                    let name = p.exposant_id.as_ref()
                        .and_then(|eid| exposants_map.get(eid))
                        .cloned()
                        .unwrap_or_else(|| "Exposant inconnu".to_string());
                    let pid = p.id.clone().unwrap_or_default();
                    let exp_id = p.exposant_id.clone().unwrap_or_default();
                    let exp_id_click = exp_id.clone();
                    let pid_accept = pid.clone();
                    let pid_refuse = pid.clone();
                    let conns_a = conns.clone();
                    let conns_r = conns.clone();
                    rsx! {
                        div {
                            style: "display: flex; align-items: center; gap: 12px; background: {c.bg_secondary}; border-radius: 8px; padding: 12px 16px;",

                            div {
                                style: "flex: 1; cursor: pointer;",
                                onclick: move |_| on_select.call(exp_id_click.clone()),
                                span {
                                    style: "color: {c.text_white}; font-size: 14px;",
                                    "{name}"
                                }
                            }

                            Badge {
                                text: "En attente".to_string(),
                                color: c.accent_orange.to_string(),
                            }

                            button {
                                style: "padding: 6px 12px; background: {c.accent_green}; border: none; border-radius: 4px; color: white; cursor: pointer; font-size: 12px;",
                                onclick: move |_| {
                                    let db = &conns_a.read().jayfestival;
                                    let _ = db.editions_exposants_update_status(&pid_accept, "acceptee", None);
                                },
                                "Valider"
                            }
                            button {
                                style: "padding: 6px 12px; background: {c.accent_orange}; border: none; border-radius: 4px; color: white; cursor: pointer; font-size: 12px;",
                                onclick: move |_| {
                                    let db = &conns_r.read().jayfestival;
                                    let _ = db.editions_exposants_update_status(&pid_refuse, "refusee", Some("Refusee par l'organisateur"));
                                },
                                "Refuser"
                            }
                        }
                    }
                }
            }
        }
    }
}

/// ORG-E10: Exposants confirmés.
#[component]
fn ConfirmesTab(edition_id: String, on_select: EventHandler<String>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();

    let participations = {
        let db = &conns.read().jayfestival;
        db.editions_exposants_by_edition(&edition_id).unwrap_or_default()
    };

    let exposants_map: std::collections::HashMap<String, String> = {
        let db = &conns.read().jayfestival;
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

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 16px;",

            h3 {
                style: "font-size: 18px; color: {c.text_white};",
                "Exposants confirmes ({accepted.len()})"
            }

            if accepted.is_empty() {
                div {
                    style: "padding: 40px; background: {c.bg_secondary}; border-radius: 8px; color: {c.text_muted}; text-align: center; font-size: 14px;",
                    "Aucun exposant confirme"
                }
            }

            for p in accepted.iter() {
                {
                    let name = p.exposant_id.as_ref()
                        .and_then(|eid| exposants_map.get(eid))
                        .cloned()
                        .unwrap_or_else(|| "Exposant inconnu".to_string());
                    let stand = p.assigned_stand.clone().unwrap_or_else(|| "Non attribue".to_string());
                    let exp_id = p.exposant_id.clone().unwrap_or_default();
                    let exp_id_click = exp_id.clone();
                    rsx! {
                        div {
                            style: "display: flex; align-items: center; gap: 12px; background: {c.bg_secondary}; border-radius: 8px; padding: 12px 16px; cursor: pointer;",
                            onclick: move |_| on_select.call(exp_id_click.clone()),

                            div {
                                style: "flex: 1;",
                                span {
                                    style: "color: {c.text_white}; font-size: 14px;",
                                    "{name}"
                                }
                                span {
                                    style: "color: {c.text_secondary}; font-size: 12px; margin-left: 12px;",
                                    "Stand: {stand}"
                                }
                            }

                            Badge {
                                text: "Confirme".to_string(),
                                color: c.accent_green.to_string(),
                            }
                        }
                    }
                }
            }
        }
    }
}

/// ORG-E11: Fiche détaillée d'un exposant.
#[component]
fn ExposantFiche(edition_id: String, exposant_id: String, on_back: EventHandler<()>) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();

    // Charger l'exposant
    let exposant = {
        let db = &conns.read().jayfestival;
        db.exposant_by_id(&exposant_id).ok().flatten()
    };

    // Charger la participation
    let participation = {
        let db = &conns.read().jayfestival;
        db.editions_exposants_by_edition(&edition_id)
            .unwrap_or_default()
            .into_iter()
            .find(|p| p.exposant_id.as_deref() == Some(&exposant_id))
    };

    let name = exposant.as_ref().and_then(|e| e.company_name.clone()).unwrap_or_else(|| "Exposant inconnu".to_string());
    let email = exposant.as_ref().and_then(|e| e.contact_email.clone()).unwrap_or_default();
    let phone = exposant.as_ref().and_then(|e| e.contact_phone.clone()).unwrap_or_default();
    let website = exposant.as_ref().and_then(|e| e.site_web.clone()).unwrap_or_default();
    let description = exposant.as_ref().and_then(|e| e.description.clone()).unwrap_or_default();

    let stand = participation.as_ref().and_then(|p| p.assigned_stand.clone()).unwrap_or_else(|| "Non attribue".to_string());
    let status = participation.as_ref().and_then(|p| p.status_candidature.clone()).unwrap_or_default();

    let status_color = match status.as_str() {
        "acceptee" => c.accent_green.to_string(),
        "en_attente" => c.accent_orange.to_string(),
        "refusee" => c.accent_red.to_string(),
        _ => c.text_muted.to_string(),
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // Retour
            button {
                style: "display: flex; align-items: center; gap: 8px; padding: 8px 0; background: none; border: none; color: {c.accent_blue}; cursor: pointer; font-size: 13px;",
                onclick: move |_| on_back.call(()),
                "← Retour a la liste"
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
                        style: "display: flex; gap: 12px;",
                        Badge {
                            text: status.clone(),
                            color: status_color,
                        }
                        span {
                            style: "font-size: 13px; color: {c.text_muted};",
                            "Stand: {stand}"
                        }
                    }
                }

                div {
                    style: "display: flex; gap: 8px;",
                    ActionButton {
                        label: "Generer devis".to_string(),
                        icon: "📄".to_string(),
                        accent: false,
                        onclick: move |_| {},
                    }
                    ActionButton {
                        label: "Envoyer message".to_string(),
                        icon: "✉️".to_string(),
                        accent: true,
                        onclick: move |_| {},
                    }
                }
            }

            // Informations de contact
            section {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                    "Informations de contact"
                }

                div {
                    style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",

                    ContactField { label: "Email", value: email }
                    ContactField { label: "Telephone", value: phone }
                    ContactField { label: "Site web", value: website }
                }
            }

            // Description
            if !description.is_empty() {
                section {
                    style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

                    h3 {
                        style: "font-size: 16px; color: {c.text_white}; margin-bottom: 12px;",
                        "Description"
                    }
                    p {
                        style: "font-size: 14px; color: {c.text_primary}; line-height: 1.6;",
                        "{description}"
                    }
                }
            }

            // Historique des actions
            section {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

                h3 {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                    "Historique"
                }

                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    HistoryEntry { action: "Candidature recue", date: "15/01/2026" }
                    HistoryEntry { action: "Candidature validee", date: "18/01/2026" }
                }
            }
        }
    }
}

#[component]
fn ContactField(label: &'static str, value: String) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let display_value = if value.is_empty() { "Non renseigne".to_string() } else { value };

    rsx! {
        div {
            span {
                style: "font-size: 12px; color: {c.text_muted}; display: block; margin-bottom: 4px;",
                "{label}"
            }
            span {
                style: "font-size: 14px; color: {c.text_primary};",
                "{display_value}"
            }
        }
    }
}

#[component]
fn HistoryEntry(action: &'static str, date: &'static str) -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "display: flex; justify-content: space-between; padding: 8px 0; border-bottom: 1px solid {c.border};",
            span {
                style: "font-size: 13px; color: {c.text_primary};",
                "{action}"
            }
            span {
                style: "font-size: 12px; color: {c.text_muted};",
                "{date}"
            }
        }
    }
}

/// ORG-E12: Génération de devis.
#[component]
fn DevisTab(edition_id: String) -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                h3 {
                    style: "font-size: 18px; color: {c.text_white};",
                    "Devis"
                }

                ActionButton {
                    label: "Nouveau devis".to_string(),
                    icon: "➕".to_string(),
                    accent: true,
                    onclick: move |_| {},
                }
            }

            // Liste des devis
            div {
                style: "display: flex; flex-direction: column; gap: 8px;",

                DevisRow {
                    numero: "DEV-2026-001",
                    exposant: "Exposant A",
                    montant: "450.00 €",
                    status: "envoye",
                    date: "10/01/2026",
                }
                DevisRow {
                    numero: "DEV-2026-002",
                    exposant: "Exposant B",
                    montant: "350.00 €",
                    status: "accepte",
                    date: "12/01/2026",
                }
                DevisRow {
                    numero: "DEV-2026-003",
                    exposant: "Exposant C",
                    montant: "550.00 €",
                    status: "brouillon",
                    date: "15/01/2026",
                }
            }
        }
    }
}

#[component]
fn DevisRow(numero: &'static str, exposant: &'static str, montant: &'static str, status: &'static str, date: &'static str) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let (status_label, status_color) = match status {
        "envoye" => ("Envoye", c.accent_blue),
        "accepte" => ("Accepte", c.accent_green),
        "refuse" => ("Refuse", c.accent_red),
        _ => ("Brouillon", c.text_muted),
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 16px; background: {c.bg_secondary}; border-radius: 8px; padding: 16px;",

            div {
                style: "flex: 1;",
                p {
                    style: "font-size: 14px; color: {c.text_white}; margin-bottom: 4px;",
                    "{numero}"
                }
                p {
                    style: "font-size: 12px; color: {c.text_muted};",
                    "{exposant} • {date}"
                }
            }

            span {
                style: "font-size: 14px; color: {c.text_primary}; font-weight: 600;",
                "{montant}"
            }

            Badge {
                text: status_label.to_string(),
                color: status_color.to_string(),
            }

            div {
                style: "display: flex; gap: 8px;",
                button {
                    style: "padding: 6px 12px; background: {c.bg_hover}; border: none; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 12px;",
                    "Voir"
                }
                button {
                    style: "padding: 6px 12px; background: {c.bg_hover}; border: none; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 12px;",
                    "PDF"
                }
            }
        }
    }
}

/// ORG-E13: Vue des factures.
#[component]
fn FacturesTab(edition_id: String) -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                h3 {
                    style: "font-size: 18px; color: {c.text_white};",
                    "Factures"
                }

                ActionButton {
                    label: "Nouvelle facture".to_string(),
                    icon: "➕".to_string(),
                    accent: true,
                    onclick: move |_| {},
                }
            }

            // Stats rapides
            div {
                style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;",

                StatMini { label: "Total facture", value: "1 350.00 €" }
                StatMini { label: "Paye", value: "800.00 €" }
                StatMini { label: "En attente", value: "550.00 €" }
            }

            // Liste des factures
            div {
                style: "display: flex; flex-direction: column; gap: 8px;",

                FactureRow {
                    numero: "FAC-2026-001",
                    exposant: "Exposant A",
                    montant: "450.00 €",
                    status: "payee",
                    date: "20/01/2026",
                }
                FactureRow {
                    numero: "FAC-2026-002",
                    exposant: "Exposant B",
                    montant: "350.00 €",
                    status: "payee",
                    date: "22/01/2026",
                }
                FactureRow {
                    numero: "FAC-2026-003",
                    exposant: "Exposant C",
                    montant: "550.00 €",
                    status: "en_attente",
                    date: "25/01/2026",
                }
            }
        }
    }
}

#[component]
fn StatMini(label: &'static str, value: &'static str) -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 16px; text-align: center;",
            p {
                style: "font-size: 12px; color: {c.text_muted}; margin-bottom: 4px;",
                "{label}"
            }
            p {
                style: "font-size: 18px; color: {c.text_white}; font-weight: 600;",
                "{value}"
            }
        }
    }
}

#[component]
fn FactureRow(numero: &'static str, exposant: &'static str, montant: &'static str, status: &'static str, date: &'static str) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let (status_label, status_color) = match status {
        "payee" => ("Payee", c.accent_green),
        "en_attente" => ("En attente", c.accent_orange),
        "annulee" => ("Annulee", c.accent_red),
        _ => ("Brouillon", c.text_muted),
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 16px; background: {c.bg_secondary}; border-radius: 8px; padding: 16px;",

            div {
                style: "flex: 1;",
                p {
                    style: "font-size: 14px; color: {c.text_white}; margin-bottom: 4px;",
                    "{numero}"
                }
                p {
                    style: "font-size: 12px; color: {c.text_muted};",
                    "{exposant} • {date}"
                }
            }

            span {
                style: "font-size: 14px; color: {c.text_primary}; font-weight: 600;",
                "{montant}"
            }

            Badge {
                text: status_label.to_string(),
                color: status_color.to_string(),
            }

            div {
                style: "display: flex; gap: 8px;",
                button {
                    style: "padding: 6px 12px; background: {c.bg_hover}; border: none; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 12px;",
                    "Voir"
                }
                button {
                    style: "padding: 6px 12px; background: {c.bg_hover}; border: none; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 12px;",
                    "PDF"
                }
            }
        }
    }
}

/// ORG-E18: Import CSV des exposants.
#[component]
fn ImportCsvTab(edition_id: String) -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            h3 {
                style: "font-size: 18px; color: {c.text_white};",
                "Import CSV"
            }

            // Zone de drop
            div {
                style: "border: 2px dashed {c.border}; border-radius: 8px; padding: 48px; text-align: center;",

                span {
                    style: "font-size: 48px; display: block; margin-bottom: 16px;",
                    "📄"
                }
                p {
                    style: "font-size: 16px; color: {c.text_white}; margin-bottom: 8px;",
                    "Glissez votre fichier CSV ici"
                }
                p {
                    style: "font-size: 13px; color: {c.text_muted}; margin-bottom: 16px;",
                    "ou cliquez pour parcourir"
                }
                button {
                    style: "padding: 10px 20px; background: {c.accent_blue}; border: none; border-radius: 6px; color: white; cursor: pointer; font-size: 14px;",
                    "Selectionner un fichier"
                }
            }

            // Format attendu
            section {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

                h4 {
                    style: "font-size: 14px; color: {c.text_white}; margin-bottom: 12px;",
                    "Format attendu"
                }

                p {
                    style: "font-size: 13px; color: {c.text_primary}; margin-bottom: 12px;",
                    "Le fichier CSV doit contenir les colonnes suivantes :"
                }

                div {
                    style: "font-family: monospace; background: {c.bg_main}; border-radius: 4px; padding: 12px; font-size: 12px; color: {c.text_secondary}; overflow-x: auto;",
                    "nom_entreprise,email,telephone,categorie,description"
                }

                div {
                    style: "margin-top: 16px;",
                    button {
                        style: "padding: 8px 16px; background: {c.bg_hover}; border: none; border-radius: 4px; color: {c.text_primary}; cursor: pointer; font-size: 13px;",
                        "📥 Telecharger le template"
                    }
                }
            }

            // Historique des imports
            section {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

                h4 {
                    style: "font-size: 14px; color: {c.text_white}; margin-bottom: 12px;",
                    "Historique des imports"
                }

                div {
                    style: "color: {c.text_muted}; font-size: 13px; text-align: center; padding: 20px;",
                    "Aucun import realise"
                }
            }
        }
    }
}
