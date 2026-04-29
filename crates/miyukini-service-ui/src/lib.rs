//! Fondations UI partagees entre tous les services standalone Miyukini.
//!
//! **Ce crate est une facade de retrocompatibilite.** Le code canonique vit
//! dans [`miyuki_ui_dioxus`] (composants) et [`miyuki_ui_tokens`] (tokens).
//!
//! Fournit :
//! - `Theme` / `ThemePalette` -- systeme de themes (types legacy avec `&'static str`)
//! - `spacing` -- constantes d'espacement
//! - `styles` -- fonctions de style derivees du theme
//! - `use_theme()` / `use_palette()` -- hooks Dioxus pour acceder au theme courant
//! - `ThemeContext` -- contexte signal pour l'arbre de composants
//!
//! # Migration
//!
//! Les nouvelles apps devraient utiliser `miyuki_ui_dioxus` directement.
//! Ce crate reste disponible pour les apps existantes (jaymanga, jaykoa, etc.).

pub mod styles;
pub mod theme;

pub use styles as ui_styles;
pub use theme::{spacing, Theme, ThemePalette};

use dioxus::prelude::*;

/// Contexte de theme partage dans l'arbre de composants.
#[derive(Clone, Copy)]
pub struct ThemeContext {
    pub theme: Theme,
}

/// Hook pour acceder au theme courant depuis n'importe quel composant.
/// Le provider doit etre installe a la racine de l'app via `use_context_provider`.
pub fn use_theme() -> Theme {
    use_context::<Signal<ThemeContext>>().read().theme
}

/// Hook pour acceder directement a la palette de couleurs.
pub fn use_palette() -> ThemePalette {
    use_theme().palette()
}

// --- Re-exports depuis miyuki-ui-dioxus pour acces direct ---

/// Re-export de la librairie de composants Dioxus atomiques.
pub use miyuki_ui_dioxus;

/// Re-export des design tokens (Rgba, Palette, UiTheme, etc.).
pub use miyuki_ui_tokens;
