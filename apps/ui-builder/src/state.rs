//! État minimal pour le UI Builder standalone.
//!
//! Fournit `use_app_state()` compatible avec `ui_builder.rs` de Central,
//! mais sans dépendance vers miyukini-central, auth, ou backends services.

use dioxus::prelude::*;
use crate::theme::Theme;

/// État global minimal (thème uniquement).
#[derive(Debug, Clone)]
pub struct AppState {
    pub current_theme: Theme,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_theme: Theme::Gaming,
        }
    }
}

/// Récupère le signal d'état global (contexte Dioxus).
pub fn use_app_state() -> Signal<AppState> {
    use_context::<Signal<AppState>>()
}
