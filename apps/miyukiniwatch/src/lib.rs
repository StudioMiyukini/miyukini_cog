//! MiyukiniWatch — App standalone Dioxus.
//!
//! Mesure des habitudes au service de Miou.
//! Utilise `miyukini-service-ui` pour le theme et les styles.
//! Utilise `miyukiniwatch` (crate) pour le backend et la persistance.

pub mod views;

use dioxus::prelude::*;
use miyukiniwatch::MiyukiniWatchDb;
use std::sync::Arc;

/// Contexte DB partage dans l'arbre Dioxus.
#[derive(Clone)]
pub struct DbContext {
    pub db: Arc<MiyukiniWatchDb>,
}

/// Hook pour acceder a la DB MiyukiniWatch depuis n'importe quel composant.
pub fn use_db() -> Arc<MiyukiniWatchDb> {
    use_context::<Signal<DbContext>>().read().db.clone()
}

/// Profil ID contexte (depuis env var ou "default").
#[derive(Clone)]
pub struct ProfileContext {
    pub profile_id: String,
}

/// Hook pour acceder au profile_id depuis n'importe quel composant.
pub fn use_profile_id() -> String {
    use_context::<Signal<ProfileContext>>()
        .read()
        .profile_id
        .clone()
}
