//! Header principal Steam-like avec navigation par onglets.

use dioxus::prelude::*;
use crate::data::profile_display_name;
use crate::state::{use_app_state, MainTab};
use crate::theme::styles;

/// Header principal de l'application.
#[component]
pub fn Header() -> Element {
    let mut state = use_app_state();
    let theme = state.read().current_theme;
    let c = theme.palette();

    let display_name = state.read().current_user.as_ref().map_or_else(|| "Profil".to_string(), profile_display_name);
    let initial = display_name.chars().next().map_or('?', |c| c.to_uppercase().next().unwrap_or(c));

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
                            onclick: move |_| { state.write().main_tab = *tab; },
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
