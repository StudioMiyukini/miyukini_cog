//! JayXpose — App standalone Dioxus.
//!
//! Profil exposant, catalogue produits, vitrine, coffre-fort documentaire.
//! Utilise `miyukini-service-ui` pour le thème et les styles.
//! Utilise `jayxpose` (crate) pour le backend et la persistance.

pub mod views;

use std::sync::Arc;
use dioxus::prelude::*;
use jayxpose::data::JayXposeDb;

/// Contexte DB partagé dans l'arbre Dioxus.
#[derive(Clone)]
pub struct DbContext {
    pub db: Arc<JayXposeDb>,
}

/// Hook pour accéder à la DB JayXpose depuis n'importe quel composant.
pub fn use_db() -> Arc<JayXposeDb> {
    use_context::<Signal<DbContext>>().read().db.clone()
}
