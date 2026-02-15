//! Écran Détail des métriques MiyukiniWatch.

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;
use miyukiniwatch::MiyukiniWatchPresenter;

#[component]
pub fn DetailScreen(profile_id: String) -> Element {
    let conns = use_service_connections();
    let state = use_app_state();
    let c = state.read().current_theme.palette();

    let db = &conns.read().miyukiniwatch;
    let presenter = MiyukiniWatchPresenter::new(std::sync::Arc::clone(db));
    let aggregates = presenter.get_aggregates(&profile_id).unwrap_or_default();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px; max-width: 800px;",

            if aggregates.is_empty() {
                div {
                    style: "padding: 24px; background: {c.bg_card}; border-radius: 8px; color: {c.text_secondary};",
                    p { "Aucune donnée à afficher pour le moment." }
                }
            } else {
                for agg in aggregates {
                    div {
                        key: "{agg.id}",
                        style: "padding: 16px; background: {c.bg_card}; border-radius: 8px; border: 1px solid {c.border};",
                        h3 { style: "font-size: 14px; color: {c.text_secondary}; margin-bottom: 8px;", "{agg.id}" }
                        pre {
                            style: "font-size: 12px; color: {c.text_primary}; overflow-x: auto;",
                            "{agg.data}"
                        }
                    }
                }
            }

            div {
                style: "margin-top: 24px; padding: 12px; background: {c.bg_secondary}; border-radius: 6px;",
                p {
                    style: "font-size: 12px; color: {c.text_muted};",
                    "MiyukiniWatch enregistre uniquement quand, où, qui et combien. "
                    "Il ne lit pas le contenu de tes messages, tes saisies ou tes fichiers."
                }
            }
        }
    }
}
