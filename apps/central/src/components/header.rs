//! Header principal Steam-like avec navigation par onglets.

use crate::data::profile_display_name;
use crate::state::{use_app_state, MainTab, OpenTab};
use crate::theme::styles;
use dioxus::prelude::*;

/// Header principal de l'application.
#[component]
pub fn Header() -> Element {
    let mut state = use_app_state();
    let theme = state.read().current_theme;
    let c = theme.palette();

    let display_name = state
        .read()
        .current_user
        .as_ref()
        .map_or_else(|| "Profil".to_string(), profile_display_name);
    let initial = display_name
        .chars()
        .next()
        .map_or('?', |c| c.to_uppercase().next().unwrap_or(c));

    rsx! {
        header {
            style: "{styles::header(theme)}",

            div {
                style: "display: flex; align-items: center; gap: 16px;",

                div {
                    style: "display: flex; align-items: center; gap: 8px; padding-right: 16px;",
                    span { style: "font-size: 20px;", "🌸" }
                }

                nav {
                    style: "display: flex; align-items: center; gap: 4px;",
                    for tab in MainTab::all() {
                        button {
                            style: "{styles::nav_tab(theme, state.read().main_tab == *tab)}",
                            onclick: move |_| {
                                let mut s = state.write();
                                s.main_tab = *tab;
                                match *tab {
                                    // "SALON" ramène à l'onglet Salon (Home, index 0)
                                    MainTab::Salon => {
                                        s.active_tab_index = 0;
                                    }
                                    // "SERVICES" ouvre directement le service Services (market)
                                    MainTab::Bibliotheque => {
                                        if let Some(idx) = s.open_tabs.iter().position(|t| t.service_id.as_deref() == Some("market")) {
                                            s.active_tab_index = idx;
                                        } else {
                                            let market_tab = OpenTab {
                                                id: "market".into(),
                                                title: "Services".into(),
                                                service_id: Some("market".into()),
                                                closable: true,
                                            };
                                            s.open_tabs.push(market_tab);
                                            s.active_tab_index = s.open_tabs.len() - 1;
                                        }
                                    }
                                    _ => {}
                                }
                            },
                            onmouseenter: move |_| {},
                            "{tab.label()}"
                        }
                    }
                }
            }

            div {
                style: "display: flex; align-items: center; gap: 16px;",

                div {
                    style: "position: relative;",
                    input {
                        style: "{styles::search_input(theme)}",
                        r#type: "text",
                        placeholder: "Rechercher dans le magasin",
                        value: "{state.read().search_query}",
                        oninput: move |evt| { state.write().search_query = evt.value(); }
                    }
                    span {
                        style: "position: absolute; right: 8px; top: 50%; transform: translateY(-50%); color: {c.text_secondary};",
                        "🔍"
                    }
                }

                button {
                    style: "background: transparent; border: none; color: {c.text_primary}; cursor: pointer; font-size: 16px; padding: 4px;",
                    "🔔"
                }

                button {
                    style: "background: transparent; border: none; {styles::user_profile(theme)}",
                    onclick: move |_| { state.write().show_profile_window = true; },
                    div { style: "{styles::avatar(theme)}", "{initial}" }
                    span { style: "color: {c.text_primary}; font-size: 13px;", "{display_name}" }
                    span { style: "color: {c.text_secondary}; font-size: 10px;", "▼" }
                }
            }
        }
    }
}
