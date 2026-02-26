//! JayManga — App standalone Dioxus.
//!
//! Fournit le module `views` (migré depuis Central) et le contexte DB.

pub mod views;

use std::sync::Arc;
use dioxus::prelude::*;
use jaymanga::data::JayMangaDb;

/// Contexte DB partagé dans l'arbre de composants.
#[derive(Clone)]
pub struct DbContext {
    pub db: Arc<JayMangaDb>,
}

/// Hook pour accéder à la base JayManga depuis n'importe quel composant.
pub fn use_db() -> Arc<JayMangaDb> {
    use_context::<Signal<DbContext>>().read().db.clone()
}
