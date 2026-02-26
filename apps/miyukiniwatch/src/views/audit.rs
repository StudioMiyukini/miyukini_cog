//! Ecran Historique des actions (audit) MiyukiniWatch.

use dioxus::prelude::*;
use miyukini_service_ui::use_palette;
use miyukiniwatch::MiyukiniWatchPresenter;
use crate::{use_db, use_profile_id};

const FMT_TIMESTAMP: &str = "%d/%m/%Y %H:%M";

#[component]
pub fn AuditScreen() -> Element {
    let c = use_palette();
    let db = use_db();
    let profile_id = use_profile_id();

    let presenter = MiyukiniWatchPresenter::new(db);
    let events = presenter.list_audit(&profile_id, 50).unwrap_or_default();

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 16px; max-width: 700px;",

            h2 {
                style: "font-size: 18px; color: {c.text_white}; margin-bottom: 8px;",
                "Historique des actions"
            }
            p {
                style: "font-size: 13px; color: {c.text_secondary}; margin-bottom: 16px;",
                "Journal des evenements : collecte activee/desactivee, effacements, purges automatiques."
            }

            if events.is_empty() {
                div {
                    style: "padding: 24px; background: {c.bg_card}; border-radius: 8px; color: {c.text_secondary};",
                    p { "Aucun evenement enregistre." }
                }
            } else {
                for evt in events {
                    div {
                        key: "{evt.id}",
                        style: "padding: 12px 16px; background: {c.bg_card}; border-radius: 6px; border: 1px solid {c.border};",
                        div {
                            style: "display: flex; justify-content: space-between; align-items: center;",
                            span { style: "font-weight: 500; color: {c.text_primary};", "{evt.event_type}" }
                            span {
                                style: "font-size: 12px; color: {c.text_muted};",
                                "{evt.timestamp.format(FMT_TIMESTAMP).to_string()}"
                            }
                        }
                        if let Some(ref d) = evt.details {
                            p { style: "font-size: 13px; color: {c.text_secondary}; margin-top: 4px;", "{d}" }
                        }
                        if let Some(n) = evt.records_affected {
                            if n > 0 {
                                span { style: "font-size: 12px; color: {c.text_muted};", " ({n} enregistrements)" }
                            }
                        }
                    }
                }
            }
        }
    }
}
