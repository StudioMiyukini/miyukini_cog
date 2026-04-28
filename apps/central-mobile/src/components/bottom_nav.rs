//! Barre de navigation inférieure (4 onglets).

use dioxus::prelude::*;

use crate::state::AppState;

/// Navigation par onglets en bas de l'écran.
#[component]
pub fn BottomNav() -> Element {
    let mut state = use_context::<Signal<AppState>>();
    let active = state.read().active_tab.clone();

    let tabs = [
        ("salon", "Salon"),
        ("services", "Services"),
        ("webway", "Webway"),
        ("remote", "Remote"),
    ];

    rsx! {
        nav {
            style: "display: flex; background: #16162a; border-top: 1px solid #2d2d44; padding: 8px 0; flex-shrink: 0;",

            for (id, label) in tabs {
                button {
                    key: "{id}",
                    style: if active == id {
                        "flex: 1; padding: 10px 4px; background: none; border: none; color: #7c3aed; font-size: 13px; font-weight: bold; cursor: pointer; text-align: center;"
                    } else {
                        "flex: 1; padding: 10px 4px; background: none; border: none; color: #666; font-size: 13px; cursor: pointer; text-align: center;"
                    },
                    onclick: move |_| {
                        state.write().active_tab = id.to_string();
                    },
                    "{label}"
                }
            }
        }
    }
}
