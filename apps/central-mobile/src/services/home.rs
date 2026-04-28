//! Vue Salon — écran d'accueil simplifié.

use dioxus::prelude::*;

use crate::state::AppState;

/// Vue Salon (accueil).
#[component]
pub fn HomeView() -> Element {
    let state = use_context::<Signal<AppState>>();
    let user = state
        .read()
        .user_display_name
        .clone()
        .unwrap_or_else(|| "Utilisateur".to_string());

    rsx! {
        div {
            style: "padding: 20px; display: flex; flex-direction: column; gap: 16px;",

            h2 {
                style: "font-size: 22px; font-weight: bold;",
                "Bienvenue, {user}"
            }

            p {
                style: "color: #888; font-size: 14px;",
                "Central Mobile — votre COG dans la poche."
            }

            // TODO Phase 2 : afficher les services récents / favoris
            // TODO Phase 4 : intégrer Miou Chat ici
        }
    }
}
