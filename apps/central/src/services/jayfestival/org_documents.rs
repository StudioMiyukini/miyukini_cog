//! ORG-E22 — Documents et legal (upload/telechargement).
//!
//! @id: jf_org_documents @do: render_org_documents
//! @role: ui @layer: service
//! @human: Ecran ORG-E22 JayFestival: documents legaux organisateur (upload/telechargement).

use dioxus::prelude::*;
use miyuki_ui_dioxus::context::use_palette;
use crate::state::use_app_state;

/// Liste des documents d'une edition.
#[component]
pub fn OrgDocuments(edition_id: String) -> Element {
    let p = use_palette();
    let selected_cat = use_signal(|| "Tous".to_string());

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // En-tête
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                h3 {
                    style: "font-size: 18px; color: {p.text_high};",
                    "Documents de l'edition"
                }

                button {
                    style: "display: flex; align-items: center; gap: 8px; padding: 10px 16px; background: {p.accent_primary}; border: none; border-radius: 6px; color: white; cursor: pointer; font-size: 13px;",
                    "📄 Ajouter un document"
                }
            }

            // Catégories
            DocumentCategories { selected: selected_cat }

            // Liste des documents
            DocumentsList {}

            // Section Templates
            DocumentTemplates {}
        }
    }
}

#[component]
fn DocumentCategories(selected: Signal<String>) -> Element {
    let p = use_palette();
    let current = selected.read().clone();

    let cat_tous_bg = if current == "Tous" { p.accent_primary } else { p.bg_secondary };
    let cat_tous_color = if current == "Tous" { "white" } else { p.text_primary };
    let cat_reg_bg = if current == "Reglementaire" { p.accent_primary } else { p.bg_secondary };
    let cat_reg_color = if current == "Reglementaire" { "white" } else { p.text_primary };
    let cat_con_bg = if current == "Contrats" { p.accent_primary } else { p.bg_secondary };
    let cat_con_color = if current == "Contrats" { "white" } else { p.text_primary };
    let cat_com_bg = if current == "Communication" { p.accent_primary } else { p.bg_secondary };
    let cat_com_color = if current == "Communication" { "white" } else { p.text_primary };

    rsx! {
        div {
            style: "display: flex; gap: 8px;",

            button {
                style: "padding: 8px 16px; background: {cat_tous_bg}; border: none; border-radius: 6px; color: {cat_tous_color}; cursor: pointer; font-size: 13px;",
                onclick: move |_| { selected.set("Tous".to_string()); },
                "Tous"
            }
            button {
                style: "padding: 8px 16px; background: {cat_reg_bg}; border: none; border-radius: 6px; color: {cat_reg_color}; cursor: pointer; font-size: 13px;",
                onclick: move |_| { selected.set("Reglementaire".to_string()); },
                "Reglementaire"
            }
            button {
                style: "padding: 8px 16px; background: {cat_con_bg}; border: none; border-radius: 6px; color: {cat_con_color}; cursor: pointer; font-size: 13px;",
                onclick: move |_| { selected.set("Contrats".to_string()); },
                "Contrats"
            }
            button {
                style: "padding: 8px 16px; background: {cat_com_bg}; border: none; border-radius: 6px; color: {cat_com_color}; cursor: pointer; font-size: 13px;",
                onclick: move |_| { selected.set("Communication".to_string()); },
                "Communication"
            }
        }
    }
}

#[component]
fn DocumentsList() -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 8px;",

            DocumentRow {
                name: "Reglement interieur",
                doc_icon: "📕",
                filename: "reglement.pdf",
                size: "15 Ko",
            }
            DocumentRow {
                name: "Contrat exposant",
                doc_icon: "📕",
                filename: "contrat.pdf",
                size: "42 Ko",
            }
            DocumentRow {
                name: "Plan de securite",
                doc_icon: "📕",
                filename: "securite.pdf",
                size: "128 Ko",
            }
        }
    }
}

#[component]
fn DocumentRow(name: &'static str, doc_icon: &'static str, filename: &'static str, size: &'static str) -> Element {
    let p = use_palette();

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 16px; background: {p.bg_secondary}; border-radius: 8px; padding: 16px;",

            // Icône
            div {
                style: "width: 48px; height: 48px; background: {p.bg_overlay}; border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 20px;",
                "{doc_icon}"
            }

            // Infos
            div {
                style: "flex: 1;",
                p {
                    style: "font-size: 14px; color: {p.text_high};",
                    "{name}"
                }
                div {
                    style: "display: flex; gap: 12px; font-size: 12px; color: {p.text_muted};",
                    span { "{filename}" }
                    span { "{size}" }
                }
            }

            // Actions
            div {
                style: "display: flex; gap: 8px;",

                button {
                    style: "padding: 8px 12px; background: {p.bg_overlay}; border: none; border-radius: 4px; color: {p.text_primary}; cursor: pointer; font-size: 12px;",
                    "Telecharger"
                }
                button {
                    style: "padding: 8px 12px; background: transparent; border: 1px solid {p.border_default}; border-radius: 4px; color: {p.text_muted}; cursor: pointer; font-size: 12px;",
                    "Supprimer"
                }
            }
        }
    }
}

#[component]
fn DocumentTemplates() -> Element {
    let p = use_palette();

    rsx! {
        section {
            style: "margin-top: 24px; padding-top: 24px; border-top: 1px solid {p.border_default};",

            h3 {
                style: "font-size: 16px; color: {p.text_high}; margin-bottom: 16px;",
                "Templates disponibles"
            }

            div {
                style: "display: grid; grid-template-columns: repeat(3, 1fr); gap: 16px;",

                TemplateCard {
                    name: "Contrat exposant type",
                    description: "Modele de contrat standard",
                }
                TemplateCard {
                    name: "Fiche technique",
                    description: "Formulaire technique stand",
                }
                TemplateCard {
                    name: "Badge exposant",
                    description: "Template badge A4",
                }
            }
        }
    }
}

#[component]
fn TemplateCard(name: &'static str, description: &'static str) -> Element {
    let p = use_palette();

    rsx! {
        div {
            style: "background: {p.bg_secondary}; border-radius: 8px; padding: 16px;",

            p {
                style: "font-size: 14px; color: {p.text_high}; margin-bottom: 4px;",
                "{name}"
            }
            p {
                style: "font-size: 12px; color: {p.text_muted}; margin-bottom: 12px;",
                "{description}"
            }
            button {
                style: "padding: 6px 12px; background: {p.bg_overlay}; border: none; border-radius: 4px; color: {p.text_primary}; cursor: pointer; font-size: 12px;",
                "Utiliser"
            }
        }
    }
}
