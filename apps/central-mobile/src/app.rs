//! Composant racine de Central Mobile.
//!
//! Gère le flux : Appairage → Connexion → Dashboard.

use dioxus::prelude::*;

use crate::state::{AppScreen, AppState};

/// Composant racine.
#[component]
pub fn App() -> Element {
    // État global de l'application : charge la connexion sauvegardée si elle existe
    let state = use_signal(|| {
        let mut s = AppState::default();
        if let Some(saved) = crate::platform::load_connection() {
            tracing::info!("Connexion sauvegardée trouvée: {}@{}:{}", saved.mode, saved.host, saved.port);
            s.saved_connection = Some(saved);
            // Skip l'appairage, aller directement à la connexion
            s.screen = AppScreen::Connexion;
        }
        s
    });

    // Fournir l'état via le contexte Dioxus
    use_context_provider(|| state);

    // Router selon l'écran actif
    let current_screen = state.read().screen.clone();

    rsx! {
        div {
            style: "width: 100%; height: 100%; display: flex; flex-direction: column; background: #1a1a2e; color: #e0e0e0; font-family: -apple-system, sans-serif;",

            match current_screen {
                AppScreen::Pairing => rsx! {
                    crate::screens::pairing::PairingScreen {}
                },
                AppScreen::Connexion => rsx! {
                    crate::screens::connexion::ConnexionScreen {}
                },
                AppScreen::Main => rsx! {
                    // Zone de contenu principal (flex: 1)
                    div {
                        style: "flex: 1; min-height: 0; overflow-y: auto;",
                        MainContent {}
                    }
                    // Barre de navigation en bas
                    crate::components::bottom_nav::BottomNav {}
                },
            }
        }
    }
}

/// Contenu principal selon l'onglet actif.
#[component]
fn MainContent() -> Element {
    let state = use_context::<Signal<AppState>>();
    let tab = state.read().active_tab.clone();

    rsx! {
        // Indicateur de connexion bridge
        crate::components::bridge_status::BridgeStatusBar {}

        match tab.as_str() {
            "salon" => rsx! { crate::services::home::HomeView {} },
            "services" => rsx! { crate::services::market::MarketView {} },
            "webway" => rsx! { crate::services::mws_status::MwsStatusView {} },
            "remote" => rsx! { crate::services::remote_control::RemoteControlView {} },
            _ => rsx! { crate::services::home::HomeView {} },
        }
    }
}
