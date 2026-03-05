//! Ecran Tableau de bord MiyukiniWatch.

use crate::{use_db, use_profile_id};
use dioxus::prelude::*;
use miyukini_service_ui::use_palette;
use miyukiniwatch::MiyukiniWatchPresenter;

#[component]
pub fn DashboardScreen(has_data: bool, is_collecting: bool) -> Element {
    let c = use_palette();
    let db = use_db();
    let profile_id = use_profile_id();

    let presenter = MiyukiniWatchPresenter::new(db);
    let aggregates = presenter.get_aggregates(&profile_id).unwrap_or_default();

    let session_summary = aggregates.iter().find(|a| a.id == "AGG_SESSION_SUMMARY");
    let session_return = aggregates.iter().find(|a| a.id == "AGG_SESSION_RETURN");
    let activity_level = aggregates.iter().find(|a| a.id == "AGG_ACTIVITY_LEVEL");

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px; max-width: 800px;",

            // Bloc perimetre
            div {
                style: "padding: 16px; background: {c.bg_secondary}; border-radius: 8px;",
                p {
                    style: "font-size: 13px; color: {c.text_secondary}; margin-bottom: 8px;",
                    "Dimensions collectees :"
                }
                ul {
                    style: "list-style: none; font-size: 13px; color: {c.text_primary};",
                    li { "\u{2713} Quand (horodatages, durees)" }
                    li { "\u{2713} Ou (services, onglets)" }
                    li { "\u{2713} Qui (identifiants de contacts)" }
                    li { "\u{2713} Combien (clics, sessions)" }
                }
            }

            if !has_data && is_collecting {
                div {
                    style: "padding: 24px; background: {c.bg_card}; border-radius: 8px; color: {c.text_secondary};",
                    p {
                        "MiyukiniWatch vient de commencer a mesurer tes habitudes. "
                        "Reviens dans quelques jours pour voir tes premieres statistiques."
                    }
                }
            } else if !has_data {
                div {
                    style: "padding: 24px; background: {c.bg_card}; border-radius: 8px; color: {c.text_secondary};",
                    p {
                        "Aucune donnee disponible. La collecte (si active) reconstituera progressivement le contexte."
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
                                p { style: "color: {c.text_primary};", "Jours depuis la derniere visite : {days}" }
                            }
                        }
                        if let Some(agg) = session_summary {
                            if let Some(total) = agg.data.get("total_sessions").and_then(|v| v.as_i64()) {
                                p { style: "color: {c.text_primary}; margin-top: 4px;", "Total des sessions : {total}" }
                            }
                        }
                    }

                    // Carte Activite
                    div {
                        style: "padding: 20px; background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border};",
                        h3 { style: "font-size: 14px; color: {c.text_secondary}; margin-bottom: 12px;", "Activite" }
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
                    "MiyukiniWatch enregistre uniquement quand, ou, qui et combien. Il ne lit pas le contenu de tes messages."
                }
            }
        }
    }
}
