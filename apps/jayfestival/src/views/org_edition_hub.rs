//! ORG-E07 — Hub d'une édition (navigation par onglets).

use super::components::TabButton;
use super::org_annonces::OrgAnnonces;
use super::org_budget::OrgBudget;
use super::org_documents::OrgDocuments;
use super::org_exposants::OrgExposants;
use super::org_parametres::OrgParametres;
use super::org_plan::OrgPlan;
use super::org_programme::OrgProgramme;
use super::org_publication::OrgPublication;
use super::org_services::OrgServices;
use super::{JayFestivalState, OrgEditionTab};
use dioxus::prelude::*;
use miyukini_service_ui::use_palette;

#[component]
pub fn OrgEditionHub(
    edition_id: String,
    tab: OrgEditionTab,
    state: Signal<JayFestivalState>,
) -> Element {
    let c = use_palette();
    // Charger l'édition
    let edition = {
        let db = crate::use_db();
        db.edition_by_id(&edition_id).ok().flatten()
    };

    let edition_name = edition
        .as_ref()
        .and_then(|e| e.name.clone())
        .unwrap_or_else(|| "Edition".to_string());

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px;",

            // En-tête
            div {
                style: "display: flex; align-items: center; gap: 12px;",

                button {
                    style: "padding: 6px 12px; background: {c.bg_hover}; border: 1px solid {c.border}; border-radius: 4px; color: {c.text_secondary}; cursor: pointer; font-size: 12px;",
                    onclick: move |_| {
                        state.write().org_section = super::OrgSection::Editions;
                    },
                    "← Retour"
                }

                h2 {
                    style: "font-size: 24px; color: {c.text_white};",
                    "🎪 {edition_name}"
                }
            }

            // Onglets
            div {
                style: "display: flex; gap: 4px; border-bottom: 1px solid {c.border}; padding-bottom: 0; flex-wrap: wrap;",

                TabButton {
                    label: "Vue d'ensemble".to_string(),
                    is_active: tab == OrgEditionTab::Overview,
                    onclick: move |_| { state.write().edition_tab = OrgEditionTab::Overview; },
                }
                TabButton {
                    label: "Parametres".to_string(),
                    is_active: tab == OrgEditionTab::Parametres,
                    onclick: move |_| { state.write().edition_tab = OrgEditionTab::Parametres; },
                }
                TabButton {
                    label: "Exposants".to_string(),
                    is_active: tab == OrgEditionTab::Exposants,
                    onclick: move |_| { state.write().edition_tab = OrgEditionTab::Exposants; },
                }
                TabButton {
                    label: "Programme".to_string(),
                    is_active: tab == OrgEditionTab::Programme,
                    onclick: move |_| { state.write().edition_tab = OrgEditionTab::Programme; },
                }
                TabButton {
                    label: "Budget".to_string(),
                    is_active: tab == OrgEditionTab::Budget,
                    onclick: move |_| { state.write().edition_tab = OrgEditionTab::Budget; },
                }
                TabButton {
                    label: "Plan".to_string(),
                    is_active: tab == OrgEditionTab::Plan,
                    onclick: move |_| { state.write().edition_tab = OrgEditionTab::Plan; },
                }
                TabButton {
                    label: "Documents".to_string(),
                    is_active: tab == OrgEditionTab::Documents,
                    onclick: move |_| { state.write().edition_tab = OrgEditionTab::Documents; },
                }
                TabButton {
                    label: "Annonces".to_string(),
                    is_active: tab == OrgEditionTab::Annonces,
                    onclick: move |_| { state.write().edition_tab = OrgEditionTab::Annonces; },
                }
                TabButton {
                    label: "Services".to_string(),
                    is_active: tab == OrgEditionTab::Services,
                    onclick: move |_| { state.write().edition_tab = OrgEditionTab::Services; },
                }
                TabButton {
                    label: "Publier".to_string(),
                    is_active: tab == OrgEditionTab::Publish,
                    onclick: move |_| { state.write().edition_tab = OrgEditionTab::Publish; },
                }
            }

            // Contenu de l'onglet
            match tab {
                OrgEditionTab::Overview => rsx! {
                    EditionOverview { edition_id: edition_id.clone() }
                },
                OrgEditionTab::Parametres => rsx! {
                    OrgParametres { edition_id: edition_id.clone() }
                },
                OrgEditionTab::Exposants => rsx! {
                    OrgExposants { edition_id: edition_id.clone() }
                },
                OrgEditionTab::Programme => rsx! {
                    OrgProgramme { edition_id: edition_id.clone() }
                },
                OrgEditionTab::Budget => rsx! {
                    OrgBudget { edition_id: edition_id.clone() }
                },
                OrgEditionTab::Plan => rsx! {
                    OrgPlan { edition_id: edition_id.clone() }
                },
                OrgEditionTab::Documents => rsx! {
                    OrgDocuments { edition_id: edition_id.clone() }
                },
                OrgEditionTab::Annonces => rsx! {
                    OrgAnnonces { edition_id: edition_id.clone() }
                },
                OrgEditionTab::Services => rsx! {
                    OrgServices { edition_id: edition_id.clone() }
                },
                OrgEditionTab::Publish => rsx! {
                    OrgPublication { edition_id: edition_id.clone() }
                },
            }
        }
    }
}

/// Vue d'ensemble d'une édition (stats synthétiques).
#[component]
fn EditionOverview(edition_id: String) -> Element {
    let c = use_palette();
    let (exposants_count, animations_count, budget_summary) = {
        let db = crate::use_db();
        let exp = db
            .editions_exposants_by_edition(&edition_id)
            .map(|v| v.len())
            .unwrap_or(0);
        let anim = db
            .animations_by_edition(&edition_id)
            .map(|v| v.len())
            .unwrap_or(0);
        let budget = db.budget_summary(&edition_id).unwrap_or_default();
        (exp, anim, budget)
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 20px;",

            // Stats de l'édition
            div {
                style: "display: grid; grid-template-columns: repeat(4, 1fr); gap: 16px;",

                super::components::StatCard {
                    label: "Exposants".to_string(),
                    value: exposants_count.to_string(),
                    icon: "🏪".to_string(),
                    color: c.accent_blue.to_string(),
                }
                super::components::StatCard {
                    label: "Animations".to_string(),
                    value: animations_count.to_string(),
                    icon: "🎭".to_string(),
                    color: c.accent_green.to_string(),
                }
                super::components::StatCard {
                    label: "Revenus".to_string(),
                    value: format!("{:.0} EUR", budget_summary.total_revenus),
                    icon: "💰".to_string(),
                    color: c.accent_green.to_string(),
                }
                super::components::StatCard {
                    label: "Balance".to_string(),
                    value: format!("{:.0} EUR", budget_summary.balance),
                    icon: "📊".to_string(),
                    color: if budget_summary.balance >= 0.0 { c.accent_green.to_string() } else { c.accent_orange.to_string() },
                }
            }
        }
    }
}
