//! EXP-E09/E10 — Documents exposant (liste et upload).

use dioxus::prelude::*;
use crate::state::use_app_state;
use super::components::{Badge, ActionButton};

/// Gestion des documents exposant.
#[component]
pub fn ExpDocuments() -> Element {
    let c = use_app_state().read().current_theme.palette();
    let mut active_tab = use_signal(|| "tous".to_string());

    let tab = active_tab.read().clone();
    let tab_tous_bg = if tab == "tous" { c.accent_blue } else { c.bg_secondary };
    let tab_tous_color = if tab == "tous" { "white" } else { c.text_primary };
    let tab_contrats_bg = if tab == "contrats" { c.accent_blue } else { c.bg_secondary };
    let tab_contrats_color = if tab == "contrats" { "white" } else { c.text_primary };
    let tab_factures_bg = if tab == "factures" { c.accent_blue } else { c.bg_secondary };
    let tab_factures_color = if tab == "factures" { "white" } else { c.text_primary };
    let tab_autres_bg = if tab == "autres" { c.accent_blue } else { c.bg_secondary };
    let tab_autres_color = if tab == "autres" { "white" } else { c.text_primary };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // En-tête
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                h2 {
                    style: "font-size: 24px; color: {c.text_white};",
                    "Mes documents"
                }

                ActionButton {
                    label: "Telecharger un document".to_string(),
                    icon: "📤".to_string(),
                    accent: true,
                    onclick: move |_| {},
                }
            }

            // Onglets
            div {
                style: "display: flex; gap: 8px;",

                button {
                    style: "padding: 8px 16px; background: {tab_tous_bg}; border: none; border-radius: 6px; color: {tab_tous_color}; cursor: pointer; font-size: 13px;",
                    onclick: move |_| { active_tab.set("tous".to_string()); },
                    "Tous"
                }
                button {
                    style: "padding: 8px 16px; background: {tab_contrats_bg}; border: none; border-radius: 6px; color: {tab_contrats_color}; cursor: pointer; font-size: 13px;",
                    onclick: move |_| { active_tab.set("contrats".to_string()); },
                    "Contrats"
                }
                button {
                    style: "padding: 8px 16px; background: {tab_factures_bg}; border: none; border-radius: 6px; color: {tab_factures_color}; cursor: pointer; font-size: 13px;",
                    onclick: move |_| { active_tab.set("factures".to_string()); },
                    "Factures"
                }
                button {
                    style: "padding: 8px 16px; background: {tab_autres_bg}; border: none; border-radius: 6px; color: {tab_autres_color}; cursor: pointer; font-size: 13px;",
                    onclick: move |_| { active_tab.set("autres".to_string()); },
                    "Autres"
                }
            }

            // Liste des documents
            div {
                style: "display: flex; flex-direction: column; gap: 8px;",

                DocumentRow {
                    name: "Contrat participation - Salon 2026",
                    doc_type: "contrat",
                    date: "10/01/2026",
                    status: "signe",
                }
                DocumentRow {
                    name: "Facture FA-2026-0042",
                    doc_type: "facture",
                    date: "15/01/2026",
                    status: "payee",
                }
                DocumentRow {
                    name: "Fiche technique stand",
                    doc_type: "technique",
                    date: "18/01/2026",
                    status: "a_completer",
                }
                DocumentRow {
                    name: "Reglement interieur",
                    doc_type: "legal",
                    date: "01/01/2026",
                    status: "lu",
                }
                DocumentRow {
                    name: "Attestation assurance RC",
                    doc_type: "assurance",
                    date: "05/01/2026",
                    status: "valide",
                }
            }

            // Zone d'upload
            UploadZone {}

            // Documents requis
            RequiredDocuments {}
        }
    }
}

#[component]
fn DocumentRow(name: &'static str, doc_type: &'static str, date: &'static str, status: &'static str) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let icon = match doc_type {
        "contrat" => "📝",
        "facture" => "💰",
        "technique" => "🔧",
        "legal" => "⚖️",
        "assurance" => "🛡️",
        _ => "📄",
    };

    let (status_label, status_color) = match status {
        "signe" => ("Signe", c.accent_green),
        "payee" => ("Payee", c.accent_green),
        "a_completer" => ("A completer", c.accent_orange),
        "lu" => ("Lu", c.text_muted),
        "valide" => ("Valide", c.accent_green),
        "en_attente" => ("En attente", c.accent_orange),
        _ => ("Inconnu", c.text_muted),
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px; background: {c.bg_secondary}; border-radius: 8px; padding: 16px;",

            span { style: "font-size: 24px;", "{icon}" }

            div {
                style: "flex: 1;",
                p {
                    style: "font-size: 14px; color: {c.text_white}; margin-bottom: 4px;",
                    "{name}"
                }
                p {
                    style: "font-size: 12px; color: {c.text_muted};",
                    "Ajoute le {date}"
                }
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
                    "📥"
                }
            }
        }
    }
}

#[component]
fn UploadZone() -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "border: 2px dashed {c.border}; border-radius: 8px; padding: 32px; text-align: center;",

            span {
                style: "font-size: 40px; display: block; margin-bottom: 12px;",
                "📤"
            }
            p {
                style: "font-size: 14px; color: {c.text_white}; margin-bottom: 8px;",
                "Glissez vos fichiers ici"
            }
            p {
                style: "font-size: 12px; color: {c.text_muted}; margin-bottom: 16px;",
                "ou cliquez pour parcourir (PDF, JPG, PNG - max 10 Mo)"
            }
            button {
                style: "padding: 10px 20px; background: {c.accent_blue}; border: none; border-radius: 6px; color: white; cursor: pointer; font-size: 14px;",
                "Parcourir"
            }
        }
    }
}

#[component]
fn RequiredDocuments() -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        section {
            style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

            h3 {
                style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                "Documents requis pour votre prochaine participation"
            }

            div {
                style: "display: flex; flex-direction: column; gap: 8px;",

                RequiredItem { name: "Attestation assurance RC Pro", provided: true }
                RequiredItem { name: "Extrait Kbis (moins de 3 mois)", provided: false }
                RequiredItem { name: "Fiche technique completee", provided: false }
            }
        }
    }
}

#[component]
fn RequiredItem(name: &'static str, provided: bool) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let icon = if provided { "✅" } else { "⬜" };
    let color = if provided { c.accent_green } else { c.text_muted };

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 12px; padding: 8px 0;",

            span { style: "font-size: 18px;", "{icon}" }

            span {
                style: "flex: 1; font-size: 14px; color: {color};",
                "{name}"
            }

            if !provided {
                button {
                    style: "padding: 6px 12px; background: {c.accent_blue}; border: none; border-radius: 4px; color: white; cursor: pointer; font-size: 12px;",
                    "Ajouter"
                }
            }
        }
    }
}
