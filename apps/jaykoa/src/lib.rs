//! JayKoa — App standalone Dioxus (Calendrier universel).
//!
//! Utilise `miyukini-service-ui` pour le theme et les styles.
//! Utilise `jaykoa` (crate) pour le backend et la persistance.

pub mod views;

use std::sync::Arc;
use dioxus::prelude::*;
use jaykoa::data::JayKoaDb;

/// Contexte DB partage dans l'arbre Dioxus.
#[derive(Clone)]
pub struct DbContext {
    pub db: Arc<JayKoaDb>,
}

/// Hook pour acceder a la DB JayKoa depuis n'importe quel composant.
pub fn use_db() -> Arc<JayKoaDb> {
    use_context::<Signal<DbContext>>().read().db.clone()
}
