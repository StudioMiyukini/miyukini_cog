//! Themes visuels Miyukini Central.
//!
//! Ce module est une facade qui re-exporte les types et styles depuis
//! `miyukini-service-ui` (lui-meme adosse a `miyuki-ui-tokens`).
//! Les fonctions de style specifiques a Central sont definies ici.

// --- Re-exports depuis miyukini-service-ui (types legacy compatibles) ---

// Types principaux
#[allow(unused_imports)]
pub use miyukini_service_ui::theme::{Theme, ThemePalette, spacing};

// --- Module styles : re-export complet + ajouts Central-specifiques ---

pub mod styles {
    //! Styles derives du theme.
    //!
    //! Re-exporte tous les styles de `miyukini_service_ui::styles` et ajoute
    //! les fonctions specifiques a Central (ex: `type_badge` qui depend de `ServiceType`).

    // Re-export de TOUTES les fonctions de style depuis miyukini-service-ui
    pub use miyukini_service_ui::styles::*;

    use super::Theme;

    /// Badge de type de service (specifique a Central car depend de `ServiceType`).
    pub fn type_badge(theme: Theme, service_type: crate::state::ServiceType) -> String {
        let _ = theme;
        let color = service_type.color();
        miyukini_service_ui::styles::type_badge_color(color)
    }
}

/// Retrocompatibilite : acces aux couleurs du theme par defaut.
pub mod colors {
    pub use super::ThemePalette;
    use super::Theme;

    /// Couleurs du theme Gaming (par defaut).
    /// Pour un theme dynamique, utiliser `state.read().current_theme.palette()`.
    #[allow(dead_code)]
    pub fn gaming() -> ThemePalette {
        Theme::Gaming.palette()
    }
}
