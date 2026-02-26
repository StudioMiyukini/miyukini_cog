//! Fondations UI partagées entre tous les services standalone Miyukini.
//!
//! Fournit :
//! - `Theme` / `ThemePalette` — système de thèmes (actuellement Gaming/Steam)
//! - `spacing` — constantes d'espacement
//! - `styles` — fonctions de style dérivées du thème
//! - `use_theme()` — hook Dioxus pour accéder au thème courant

pub mod theme;
pub mod styles;

pub use theme::{Theme, ThemePalette, spacing};
pub use styles as ui_styles;

use dioxus::prelude::*;

/// Contexte de thème partagé dans l'arbre de composants.
#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub theme: Theme,
}

/// Hook pour accéder au thème courant depuis n'importe quel composant.
/// Le provider doit être installé à la racine de l'app via `use_context_provider`.
pub fn use_theme() -> Theme {
    use_context::<Signal<ThemeContext>>().read().theme
}

/// Hook pour accéder directement à la palette de couleurs.
pub fn use_palette() -> ThemePalette {
    use_theme().palette()
}
