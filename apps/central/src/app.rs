//! Application principale Miyukini Central.

use std::sync::Arc;

use dioxus::prelude::*;
use crate::data::ServiceConnections;
use crate::state::{AppContext, AppState};
use crate::components::{Header, TabBar};
use crate::services::ActiveServiceView;
use crate::theme::styles;
use crate::screens::{RiteEntree, Connexion, ProfileWindow};

/// Point d'entrée de l'application.
#[component]
pub fn App() -> Element {
    // Un seul provider : connexions + état dérivé (évite hook-in-hook).
    use_context_provider(|| {
        let base_path = std::env::current_dir().unwrap_or_default();
        let connections = ServiceConnections::open(&base_path)
            .expect("Impossible d'ouvrir les bases de donnees service");
        let connections = Arc::new(connections);
        let auth_db = &*connections.auth_db;
        let is_cog_virgin = auth_db.is_cog_virgin().unwrap_or(true);
        // Charge le dernier profil connu uniquement pour pré-remplir l'écran de connexion (pas d'auto-login).
        let last_profile = auth_db
            .get_current_profile_id()
            .ok()
            .flatten()
            .and_then(|id| auth_db.get_profile(&id).ok().flatten());
        let mut state = AppState::default();
        state.is_cog_virgin = is_cog_virgin;
        if let Some(ref p) = last_profile {
            state.last_login_email = p.email.clone();
            state.last_login_pseudo = p.pseudonyme.clone().unwrap_or_default();
        }
        // current_user reste None au démarrage : l'écran de connexion s'affiche toujours.
        AppContext {
            connections: Signal::new(connections),
            state: Signal::new(state),
        }
    });

    let ctx = use_context::<AppContext>();
    let state = ctx.state;
    let is_cog_virgin = state.read().is_cog_virgin;
    let has_user = state.read().current_user.is_some();
    let theme = state.read().current_theme;
    let c = theme.palette();

    rsx! {
        div {
            style: "{styles::main_container(theme)}",

            // CSS global pour les scrollbars et fonts
            style { {GLOBAL_CSS} }

            if is_cog_virgin {
                RiteEntree {}
            } else if !has_user {
                Connexion {}
            } else {
                Header {}
                main {
                    style: "{styles::content_area(theme)}",
                    role: "main",

                    TabBar {}
                    div {
                        style: "{styles::content_panel(theme)}",
                        ActiveServiceView {}
                    }
                }
                footer {
                    style: "display: flex; align-items: center; justify-content: space-between; height: 24px; background: {c.bg_header}; padding: 0 16px; font-size: 11px; color: {c.text_muted}; border-top: 1px solid {c.border};",
                    span { "Miyukini Central v0.1.0" }
                    span { "COG: Actif • KindMother: 4 DB connectees" }
                }
                if state.read().show_profile_window {
                    ProfileWindow {}
                }
            }
        }
    }
}

/// CSS global injecté dans la page.
const GLOBAL_CSS: &str = r#"
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: 'Segoe UI', -apple-system, BlinkMacSystemFont, 'Roboto', sans-serif;
    -webkit-font-smoothing: antialiased;
    overflow: hidden;
}

/* Scrollbars style Steam */
::-webkit-scrollbar {
    width: 8px;
    height: 8px;
}

::-webkit-scrollbar-track {
    background: #1b2838;
}

::-webkit-scrollbar-thumb {
    background: #3d4f5f;
    border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
    background: #4d6070;
}

/* Focus outline */
*:focus {
    outline: none;
}

*:focus-visible {
    outline: 2px solid #1a9fff;
    outline-offset: 2px;
}

/* Button reset */
button {
    font-family: inherit;
}

/* Smooth transitions */
button, a, div {
    transition: background-color 0.15s ease, color 0.15s ease, border-color 0.15s ease, transform 0.15s ease;
}

/* Hover effects */
button:hover:not(:disabled) {
    filter: brightness(1.1);
}

/* Active state */
button:active:not(:disabled) {
    transform: scale(0.98);
}

/* Selection color */
::selection {
    background: #1a9fff;
    color: white;
}
"#;
