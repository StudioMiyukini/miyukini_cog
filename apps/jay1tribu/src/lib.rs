//! Jay1Tribu — App standalone Dioxus.
//!
//! Messagerie P2P, tribus, salons et amis.
//! Utilise `miyukini-service-ui` pour le theme et les styles.
//! Utilise `jay1tribu` (crate) pour le backend et la persistance.

pub mod views;

use std::sync::Arc;
use dioxus::prelude::*;
use jay1tribu::Jay1TribuDb;

/// Contexte DB partage dans l'arbre Dioxus.
#[derive(Clone)]
pub struct DbContext {
    pub db: Arc<Jay1TribuDb>,
}

/// Hook pour acceder a la DB Jay1Tribu depuis n'importe quel composant.
pub fn use_db() -> Arc<Jay1TribuDb> {
    use_context::<Signal<DbContext>>().read().db.clone()
}
