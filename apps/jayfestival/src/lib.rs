//! JayFestival — Application standalone (Dioxus desktop).
//!
//! Fournit :
//! - `DbContext` / `use_db()` — acces a la base JayFestivalDb via contexte Dioxus
//! - `views` — module UI complet (facade publique UNC, espaces ORG/EXP/VIS)

pub mod views;

use dioxus::prelude::*;
use jayfestival::data::JayFestivalDb;
use std::sync::Arc;

/// Contexte de base de donnees partage dans l'arbre de composants.
#[derive(Clone)]
pub struct DbContext {
    pub db: Arc<JayFestivalDb>,
}

/// Hook pour acceder a la base JayFestivalDb depuis n'importe quel composant.
/// Le provider doit etre installe a la racine de l'app via `use_context_provider`.
pub fn use_db() -> Arc<JayFestivalDb> {
    use_context::<Signal<DbContext>>().read().db.clone()
}
