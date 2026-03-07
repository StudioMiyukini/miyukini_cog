//! ORG-E25 — Publication et cloture (workflow de validation).
//!
//! @id: jf_org_publication @do: render_org_publication
//! @role: ui @layer: service
//! @human: Ecran ORG-E25 JayFestival: publication et cloture (workflow de validation).

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use super::components::Badge;

/// Workflow de publication d'une edition.
#[component]
pub fn OrgPublication(edition_id: String) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();

    // Charger l'édition
    let edition = {
        let db = &conns.read().jayfestival;
        db.edition_by_id(&edition_id).ok().flatten()
    };

    let status = edition.as_ref().and_then(|e| e.status.clone()).unwrap_or_default();

    // Compteurs pour checklist
    let (exposants_count, animations_count) = {
        let db = &conns.read().jayfestival;
        let exp = db.editions_exposants_by_edition(&edition_id).map(|v| v.len()).unwrap_or(0);
        let anim = db.animations_by_edition(&edition_id).map(|v| v.len()).unwrap_or(0);
        (exp, anim)
    };

    let is_publishable = exposants_count > 0 && animations_count > 0;

    // Calcul de la couleur du badge de statut
    let status_color = match status.as_str() {
        "publie" | "en_cours" => c.accent_green.to_string(),
        "brouillon" | "en_preparation" => c.accent_orange.to_string(),
        _ => c.text_muted.to_string(),
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px;",

            // En-tête
            div {
                style: "display: flex; justify-content: space-between; align-items: center;",

                div {
                    h3 {
                        style: "font-size: 18px; color: {c.text_white}; margin-bottom: 4px;",
                        "Publication de l'evenement"
                    }
                    div {
                        style: "display: flex; align-items: center; gap: 8px;",
                        span {
                            style: "font-size: 13px; color: {c.text_muted};",
                            "Statut actuel :"
                        }
                        Badge {
                            text: status.clone(),
                            color: status_color,
                        }
                    }
                }
            }

            // Checklist avant publication
            PublicationChecklist {
                exposants_count: exposants_count,
                animations_count: animations_count,
            }

            // Actions de publication
            PublicationActions {
                is_publishable: is_publishable,
                status: status.clone(),
                edition_id: edition_id.clone(),
            }

            // Historique des actions
            PublicationHistory {}
        }
    }
}

#[component]
fn PublicationChecklist(exposants_count: usize, animations_count: usize) -> Element {
    let c = use_app_state().read().current_theme.palette();

    let exp_desc = format!("{exposants_count} exposant(s) inscrit(s)");
    let anim_desc = format!("{animations_count} animation(s) programmee(s)");

    rsx! {
        section {
            h4 {
                style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                "Checklist avant publication"
            }

            div {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

                div {
                    style: "display: flex; flex-direction: column; gap: 12px;",

                    // Check 1 : Informations de base
                    ChecklistItem {
                        label: "Informations de base renseignees",
                        description: "Nom, dates, lieu, description".to_string(),
                        checked: true,
                    }

                    // Check 2 : Exposants
                    ChecklistItem {
                        label: "Au moins un exposant valide",
                        description: exp_desc,
                        checked: exposants_count > 0,
                    }

                    // Check 3 : Programme
                    ChecklistItem {
                        label: "Programme defini",
                        description: anim_desc,
                        checked: animations_count > 0,
                    }

                    // Check 4 : Documents
                    ChecklistItem {
                        label: "Documents legaux telecharges",
                        description: "Reglement, conditions, assurance".to_string(),
                        checked: false,
                    }

                    // Check 5 : Plan
                    ChecklistItem {
                        label: "Plan de salle configure",
                        description: "Stands attribues aux exposants".to_string(),
                        checked: false,
                    }
                }
            }
        }
    }
}

#[component]
fn PublicationActions(is_publishable: bool, status: String, edition_id: String) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let conns = use_service_connections();

    let publish_border = if is_publishable {
        format!("2px solid {}", c.accent_green)
    } else {
        "none".to_string()
    };

    let publish_message = if is_publishable {
        "L'evenement est pret a etre publie"
    } else {
        "Completez la checklist pour pouvoir publier"
    };

    let publish_label = if status == "publie" {
        "Deja publie"
    } else {
        "Publier maintenant"
    };

    let is_publish_active = is_publishable && status != "publie";

    let publish_btn_bg = if is_publish_active { c.accent_blue } else { c.bg_hover };
    let publish_btn_color = if is_publish_active { "white" } else { c.text_muted };

    rsx! {
        section {
            h4 {
                style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                "Actions"
            }

            div {
                style: "display: grid; grid-template-columns: repeat(2, 1fr); gap: 16px;",

                // Prévisualisation
                div {
                    style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px;",

                    div {
                        style: "display: flex; align-items: center; gap: 12px; margin-bottom: 12px;",
                        span { style: "font-size: 24px;", "👁️" }
                        h5 {
                            style: "font-size: 14px; color: {c.text_white};",
                            "Previsualiser"
                        }
                    }
                    p {
                        style: "font-size: 13px; color: {c.text_muted}; margin-bottom: 16px;",
                        "Voir l'evenement tel qu'il apparaitra pour les visiteurs"
                    }
                    button {
                        style: "display: flex; align-items: center; gap: 8px; padding: 10px 16px; background: {c.bg_hover}; border: none; border-radius: 6px; color: {c.text_primary}; cursor: pointer; font-size: 13px;",
                        "🔍 Ouvrir apercu"
                    }
                }

                // Publication
                div {
                    style: "background: {c.bg_secondary}; border-radius: 8px; padding: 20px; border: {publish_border};",

                    div {
                        style: "display: flex; align-items: center; gap: 12px; margin-bottom: 12px;",
                        span { style: "font-size: 24px;", "🚀" }
                        h5 {
                            style: "font-size: 14px; color: {c.text_white};",
                            "Publier"
                        }
                    }
                    p {
                        style: "font-size: 13px; color: {c.text_muted}; margin-bottom: 16px;",
                        "{publish_message}"
                    }
                    button {
                        style: "display: flex; align-items: center; gap: 8px; padding: 10px 16px; background: {publish_btn_bg}; border: none; border-radius: 6px; color: {publish_btn_color}; cursor: pointer; font-size: 13px;",
                        onclick: move |_| {
                            if is_publish_active {
                                let db = &conns.read().jayfestival;
                                if let Ok(Some(mut ed)) = db.edition_by_id(&edition_id) {
                                    ed.status = Some("publie".to_string());
                                    let _ = db.edition_update(&ed);
                                }
                            }
                        },
                        "📤 {publish_label}"
                    }
                }
            }
        }
    }
}

#[component]
fn PublicationHistory() -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        section {
            style: "margin-top: 24px; padding-top: 24px; border-top: 1px solid {c.border};",

            h4 {
                style: "font-size: 16px; color: {c.text_white}; margin-bottom: 16px;",
                "Historique des actions"
            }

            div {
                style: "background: {c.bg_secondary}; border-radius: 8px; padding: 16px;",

                div {
                    style: "display: flex; flex-direction: column; gap: 8px;",

                    HistoryItem {
                        action: "Edition creee",
                        date: "01/01/2026 10:30",
                        user: "Admin",
                    }
                    HistoryItem {
                        action: "Brouillon enregistre",
                        date: "05/01/2026 14:15",
                        user: "Admin",
                    }
                }
            }
        }
    }
}

#[component]
fn HistoryItem(action: &'static str, date: &'static str, user: &'static str) -> Element {
    let c = use_app_state().read().current_theme.palette();

    rsx! {
        div {
            style: "display: flex; justify-content: space-between; align-items: center; padding: 8px 0; border-bottom: 1px solid {c.border};",

            span {
                style: "font-size: 13px; color: {c.text_primary};",
                "{action}"
            }
            span {
                style: "font-size: 12px; color: {c.text_muted};",
                "{date} • {user}"
            }
        }
    }
}

/// Item de checklist.
#[component]
fn ChecklistItem(label: &'static str, description: String, checked: bool) -> Element {
    let c = use_app_state().read().current_theme.palette();
    let icon = if checked { "✅" } else { "⬜" };
    let color = if checked { c.accent_green } else { c.text_muted };

    rsx! {
        div {
            style: "display: flex; align-items: flex-start; gap: 12px; padding: 8px 0;",

            span {
                style: "font-size: 20px;",
                "{icon}"
            }
            div {
                p {
                    style: "font-size: 14px; color: {color};",
                    "{label}"
                }
                p {
                    style: "font-size: 12px; color: {c.text_muted};",
                    "{description}"
                }
            }
        }
    }
}
