//! MiyukiniWatch — Service silencieux de mesure des habitudes.
//!
//! Interface : tableau de bord, détail, paramètres, audit.

use dioxus::prelude::*;
use crate::data::use_service_connections;
use crate::state::use_app_state;

mod dashboard;
mod detail;
mod params;
mod audit;

use dashboard::DashboardScreen;
use detail::DetailScreen;
use params::ParamsScreen;
use audit::AuditScreen;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum MiyukiniWatchTab {
    #[default]
    Dashboard,
    Detail,
    Params,
    Audit,
}

#[component]
pub fn MiyukiniWatchView() -> Element {
    let state = use_app_state();
    let conns = use_service_connections();
    let mut active_tab = use_signal(|| MiyukiniWatchTab::Dashboard);

    let profile_id = state.read().current_user.as_ref().map(|u| u.id.clone()).unwrap_or_default();
    let theme = state.read().current_theme;
    let c = theme.palette();

    let db = &conns.read().miyukiniwatch;
    let presenter = miyukiniwatch::MiyukiniWatchPresenter::new(std::sync::Arc::clone(db));
    let prefs = presenter.get_prefs(&profile_id).unwrap_or_default();
    let has_data = presenter.has_data(&profile_id).unwrap_or(false);
    let is_collecting = prefs.collect_enabled;

    rsx! {
        div {
            style: "display: flex; flex-direction: column; height: 100%; overflow: hidden;",

            // En-tête
            div {
                style: "padding: 24px 24px 16px; border-bottom: 1px solid {c.border};",
                h1 {
                    style: "font-size: 24px; font-weight: 600; color: {c.text_white}; margin-bottom: 8px;",
                    "MiyukiniWatch"
                }
                p {
                    style: "font-size: 14px; color: {c.text_secondary}; max-width: 600px;",
                    "Ce service mesure tes habitudes pour aider Miou à te faire de meilleures suggestions. "
                    "Il ne lit jamais le contenu de tes messages."
                }
                if !is_collecting {
                    div {
                        style: "margin-top: 12px; padding: 12px; background: {c.bg_secondary}; border-radius: 6px; color: {c.text_secondary};",
                        "La collecte est désactivée. Les données existantes restent consultables."
                    }
                }
            }

            // Onglets
            div {
                style: "display: flex; gap: 4px; padding: 0 24px; background: {c.bg_header}; border-bottom: 1px solid {c.border};",
                TabButton {
                    label: "Tableau de bord",
                    is_active: *active_tab.read() == MiyukiniWatchTab::Dashboard,
                    onclick: move |_| active_tab.set(MiyukiniWatchTab::Dashboard),
                }
                TabButton {
                    label: "Détail",
                    is_active: *active_tab.read() == MiyukiniWatchTab::Detail,
                    onclick: move |_| active_tab.set(MiyukiniWatchTab::Detail),
                }
                TabButton {
                    label: "Paramètres",
                    is_active: *active_tab.read() == MiyukiniWatchTab::Params,
                    onclick: move |_| active_tab.set(MiyukiniWatchTab::Params),
                }
                TabButton {
                    label: "Historique",
                    is_active: *active_tab.read() == MiyukiniWatchTab::Audit,
                    onclick: move |_| active_tab.set(MiyukiniWatchTab::Audit),
                }
            }

            // Contenu
            div {
                style: "flex: 1; overflow-y: auto; padding: 24px;",
                match *active_tab.read() {
                    MiyukiniWatchTab::Dashboard => rsx! {
                        DashboardScreen {
                            profile_id: profile_id.clone(),
                            has_data,
                            is_collecting,
                        }
                    },
                    MiyukiniWatchTab::Detail => rsx! {
                        DetailScreen { profile_id: profile_id.clone() }
                    },
                    MiyukiniWatchTab::Params => rsx! {
                        ParamsScreen {
                            profile_id: profile_id.clone(),
                            prefs: prefs.clone(),
                        }
                    },
                    MiyukiniWatchTab::Audit => rsx! {
                        AuditScreen { profile_id: profile_id.clone() }
                    },
                }
            }
        }
    }
}

#[component]
fn TabButton(
    label: &'static str,
    is_active: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let state = use_app_state();
    let c = state.read().current_theme.palette();
    let bg = if is_active { c.accent_blue } else { "transparent" };
    let color = if is_active { c.text_white } else { c.text_secondary };
    let border_color = if is_active { c.accent_blue } else { "transparent" };

    rsx! {
        button {
            style: "padding: 12px 20px; background: {bg}; color: {color}; border: none; cursor: pointer; font-size: 14px; border-bottom: 2px solid {border_color};",
            onclick: move |evt| onclick.call(evt),
            "{label}"
        }
    }
}
