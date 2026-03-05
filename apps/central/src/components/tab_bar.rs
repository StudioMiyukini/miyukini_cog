//! Barre d'onglets des services ouverts.

use crate::state::use_app_state;
use crate::theme::styles;
use dioxus::prelude::*;

#[component]
pub fn TabBar() -> Element {
    let mut state = use_app_state();
    let theme = state.read().current_theme;
    let c = theme.palette();
    let open_tabs = state.read().open_tabs.clone();
    let active_index = state.read().active_tab_index;

    rsx! {
        div {
            style: "{styles::tab_bar(theme)}",

            for (index, tab) in open_tabs.iter().enumerate() {
                div {
                    key: "{tab.id}",
                    style: "{styles::service_tab(theme, index == active_index)}",
                    onclick: move |_| { state.write().active_tab_index = index; },

                    if tab.service_id.is_some() {
                        span { style: "font-size: 14px;",
                            { state.read().services.iter().find(|s| Some(&s.id) == tab.service_id.as_ref()).map_or_else(|| "📦".to_string(), |s| s.icon.clone()) }
                        }
                    } else {
                        span { style: "font-size: 14px;", "🏠" }
                    }

                    span { style: "overflow: hidden; text-overflow: ellipsis; white-space: nowrap;", "{tab.title}" }

                    if tab.closable {
                        button {
                            style: "{styles::tab_close_btn(theme)}",
                            onclick: move |evt| { evt.stop_propagation(); state.write().close_tab(index); },
                            "✕"
                        }
                    }
                }
            }

            button {
                style: "padding: 8px 12px; background: transparent; border: none; color: {c.text_secondary}; cursor: pointer; font-size: 16px;",
                onclick: move |_| { state.write().main_tab = crate::state::MainTab::Bibliotheque; },
                "+"
            }
        }
    }
}
