//! Écran Historique des actions (audit) MiyukiniWatch.

use dioxus::prelude::*;

const FMT_TIMESTAMP: &str = "%d/%m/%Y %H:%M";
use crate::data::use_service_connections;
use crate::state::use_app_state;
use miyukiniwatch::MiyukiniWatchPresenter;

#[component]
pub fn AuditScreen(profile_id: String) -> Element {
    let conns = use_service_connections();
    let state = use_app_state();
    let c = state.read().current_theme.palette();

    let db = &conns.read().miyukiniwatch;
    let presenter = MiyukiniWatchPresenter::new(std::sync::Arc::clone(db));
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
                "Journal des événements : collecte activée/désactivée, effacements, purges automatiques."
            }

            if events.is_empty() {
                div {
                    style: "padding: 24px; background: {c.bg_card}; border-radius: 8px; color: {c.text_secondary};",
                    p { "Aucun événement enregistré." }
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
