//! Miyukini UI Builder — Exécutable standalone
//!
//! Lance uniquement la vitrine des assets UI (panneaux, boutons, barres, curseurs)
//! sans charger l'application Central complète. Compilation rapide et lancement
//! instantané pour itérer sur le design.

mod state;
mod theme;
mod services;

use dioxus::prelude::*;
use crate::state::AppState;
use crate::services::ui_builder::UiBuilderView;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("ui_builder=debug,dioxus=info")
        .init();

    tracing::info!("Démarrage de Miyukini UI Builder (standalone)…");

    let config = dioxus::desktop::Config::new()
        .with_window(
            dioxus::desktop::WindowBuilder::new()
                .with_title("Miyukini UI Builder")
                .with_inner_size(dioxus::desktop::LogicalSize::new(1400.0, 900.0))
                .with_min_inner_size(dioxus::desktop::LogicalSize::new(800.0, 600.0))
                .with_decorations(true)
                .with_resizable(true),
        )
        .with_disable_context_menu(true);

    dioxus::LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(App);
}

/// CSS global — reset, scrollbars thème Gaming, transitions.
const GLOBAL_CSS: &str = r#"
/* ── Reset ────────────────────────────────────────────────────── */
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

/* ── Scrollbars — style Steam / Gaming sombre ─────────────────── */
* {
    scrollbar-width: thin;
    scrollbar-color: #3a5a7c #1b2838;
}

::-webkit-scrollbar {
    width: 8px;
    height: 8px;
}

::-webkit-scrollbar-track {
    background: #1b2838;
    border-radius: 4px;
}

::-webkit-scrollbar-thumb {
    background: linear-gradient(180deg, #3a5a7c 0%, #2e4a68 100%);
    border-radius: 4px;
    border: 1px solid #1b2838;
}

::-webkit-scrollbar-thumb:hover {
    background: linear-gradient(180deg, #4a7aac 0%, #3a6a8c 100%);
}

::-webkit-scrollbar-thumb:active {
    background: linear-gradient(180deg, #1a9fff 0%, #3a7aac 100%);
}

::-webkit-scrollbar-corner {
    background: #1b2838;
}

/* ── Focus ────────────────────────────────────────────────────── */
*:focus {
    outline: none;
}

*:focus-visible {
    outline: 2px solid #1a9fff;
    outline-offset: 2px;
}

/* ── Boutons ──────────────────────────────────────────────────── */
button {
    font-family: inherit;
}

button, a, div {
    transition: background-color 0.15s ease, color 0.15s ease,
                border-color 0.15s ease, transform 0.15s ease;
}

button:hover:not(:disabled) {
    filter: brightness(1.1);
}

button:active:not(:disabled) {
    transform: scale(0.98);
}

/* ── Sélection ────────────────────────────────────────────────── */
::selection {
    background: #1a9fff;
    color: white;
}
"#;

/// Application racine — fournit le contexte d'état minimal puis affiche le UI Builder.
#[component]
fn App() -> Element {
    let app_state = use_signal(AppState::default);
    use_context_provider(|| app_state);

    let c = app_state.read().current_theme.palette();

    rsx! {
        // Injection du CSS global (reset + scrollbars + transitions)
        style { {GLOBAL_CSS} }

        div {
            style: "width: 100%; height: 100vh; background: {c.bg_main}; color: {c.text_primary}; font-family: 'Segoe UI', Arial, sans-serif; overflow: hidden;",
            UiBuilderView {}
        }
    }
}
