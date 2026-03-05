//! JayKonta — App standalone Dioxus.
//!
//! Comptabilité COG unifiée Purse + Account.
//! Utilise `miyukini-service-ui` pour le thème et les styles.
//! Utilise `jaykonta` (crate) pour le backend et la persistance.

pub mod views;

use dioxus::prelude::*;
use jaykonta::data::JayKontaDb;
use std::sync::Arc;

/// Contexte DB partagé dans l'arbre Dioxus.
#[derive(Clone)]
pub struct DbContext {
    pub db: Arc<JayKontaDb>,
}

/// Hook pour accéder à la DB JayKonta depuis n'importe quel composant.
pub fn use_db() -> Arc<JayKontaDb> {
    use_context::<Signal<DbContext>>().read().db.clone()
}
