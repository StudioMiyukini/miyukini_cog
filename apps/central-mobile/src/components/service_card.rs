//! Carte de service (version mobile, compacte).

use dioxus::prelude::*;

/// Carte d'un service pour la vue mobile.
#[component]
pub fn ServiceCard(name: String, description: String, icon: String, is_installed: bool) -> Element {
    let border_color = if is_installed { "#22c55e" } else { "#3d3d55" };

    rsx! {
        div {
            style: "padding: 14px; background: #2d2d44; border: 1px solid {border_color}; border-radius: 10px; display: flex; gap: 12px; align-items: center;",

            // Icône
            div {
                style: "width: 44px; height: 44px; background: #3d3d55; border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 20px; flex-shrink: 0;",
                "{icon}"
            }

            // Infos
            div {
                style: "flex: 1; min-width: 0;",
                div {
                    style: "font-weight: bold; font-size: 15px; color: #e0e0e0;",
                    "{name}"
                }
                div {
                    style: "font-size: 13px; color: #888; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;",
                    "{description}"
                }
            }
        }
    }
}
