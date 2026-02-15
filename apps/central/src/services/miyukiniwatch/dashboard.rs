//! Écran Tableau de bord MiyukiniWatch.

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use miyukiniwatch::MiyukiniWatchPresenter;

#[component]
pub fn DashboardScreen(
    profile_id: String,
    has_data: bool,
    is_collecting: bool,
) -> Element {
    let conns = use_service_connections();
    let state = use_app_state();
    let c = state.read().current_theme.palette();

    let db = &conns.read().miyukiniwatch;
    let presenter = MiyukiniWatchPresenter::new(std::sync::Arc::clone(db));
    let aggregates = presenter.get_aggregates(&profile_id).unwrap_or_default();

    let session_summary = aggregates.iter().find(|a| a.id == "AGG_SESSION_SUMMARY");
    let session_return = aggregates.iter().find(|a| a.id == "AGG_SESSION_RETURN");
    let activity_level = aggregates.iter().find(|a| a.id == "AGG_ACTIVITY_LEVEL");

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px; max-width: 800px;",

            // Bloc périmètre
            div {
                style: "padding: 16px; background: {c.bg_secondary}; border-radius: 8px;",
                p {
                    style: "font-size: 13px; color: {c.text_secondary}; margin-bottom: 8px;",
                    "Dimensions collectées :"
                }
                ul {
                    style: "list-style: none; font-size: 13px; color: {c.text_primary};",
                    li { "✓ Quand (horodatages, durées)" }
                    li { "✓ Où (services, onglets)" }
                    li { "✓ Qui (identifiants de contacts)" }
                    li { "✓ Combien (clics, sessions)" }
                }
            }

            if !has_data && is_collecting {
                div {
                    style: "padding: 24px; background: {c.bg_card}; border-radius: 8px; color: {c.text_secondary};",
                    p {
                        "MiyukiniWatch vient de commencer à mesurer tes habitudes. "
                        "Reviens dans quelques jours pour voir tes premières statistiques."
                    }
                }
            } else if !has_data {
                div {
                    style: "padding: 24px; background: {c.bg_card}; border-radius: 8px; color: {c.text_secondary};",
                    p {
                        "Aucune donnée disponible. La collecte (si active) reconstituera progressivement le contexte."
                    }
                }
            } else {
                // Cartes
                div {
                    style: "display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 16px;",

                    // Carte Sessions
                    div {
                        style: "padding: 20px; background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border};",
                        h3 { style: "font-size: 14px; color: {c.text_secondary}; margin-bottom: 12px;", "Sessions" }
                        if let Some(agg) = session_return {
                            if let Some(days) = agg.data.get("days_away").and_then(|v| v.as_u64()) {
                                p { style: "color: {c.text_primary};", "Jours depuis la dernière visite : {days}" }
                            }
                        }
                        if let Some(agg) = session_summary {
                            if let Some(total) = agg.data.get("total_sessions").and_then(|v| v.as_i64()) {
                                p { style: "color: {c.text_primary}; margin-top: 4px;", "Total des sessions : {total}" }
                            }
                        }
                    }

                    // Carte Activité
                    div {
                        style: "padding: 20px; background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border};",
                        h3 { style: "font-size: 14px; color: {c.text_secondary}; margin-bottom: 12px;", "Activité" }
                        if let Some(agg) = activity_level {
                            if let Some(sessions) = agg.data.get("sessions_week").and_then(|v| v.as_i64()) {
                                p { style: "color: {c.text_primary};", "Sessions cette semaine : {sessions}" }
                            }
                        }
                    }
                }
            }

            // Pied
            div {
                style: "margin-top: 24px; padding-top: 16px; border-top: 1px solid {c.border};",
                p {
                    style: "font-size: 12px; color: {c.text_muted};",
                    "MiyukiniWatch enregistre uniquement quand, où, qui et combien. Il ne lit pas le contenu de tes messages."
                }
            }
        }
    }
}
