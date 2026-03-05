//! Ecran Detail des metriques MiyukiniWatch.

use crate::{use_db, use_profile_id};
use dioxus::prelude::*;
use miyukini_service_ui::use_palette;
use miyukiniwatch::MiyukiniWatchPresenter;

#[component]
pub fn DetailScreen() -> Element {
    let c = use_palette();
    let db = use_db();
    let profile_id = use_profile_id();

    let presenter = MiyukiniWatchPresenter::new(db);
    let aggregates = presenter.get_aggregates(&profile_id).unwrap_or_default();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 24px; max-width: 800px;",

            if aggregates.is_empty() {
                div {
                    style: "padding: 24px; background: {c.bg_card}; border-radius: 8px; color: {c.text_secondary};",
                    p { "Aucune donnee a afficher pour le moment." }
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
                    "MiyukiniWatch enregistre uniquement quand, ou, qui et combien. "
                    "Il ne lit pas le contenu de tes messages, tes saisies ou tes fichiers."
                }
            }
        }
    }
}
