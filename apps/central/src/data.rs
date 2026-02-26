//! Couche données : connexion auth Central.
//!
//! Les services sont désormais des binaires indépendants qui gèrent leurs propres bases de données.
//! Central ne conserve que la base auth pour la gestion des profils et sessions.

use std::path::Path;
use std::sync::Arc;

use dioxus::prelude::*;
use miyukini_central::auth::{CentralAuthDb, CentralProfile};

/// Connexions DB de Central (standalone — uniquement auth).
pub struct ServiceConnections {
    /// Base auth Central (profils, COG vierge, session).
    pub auth_db: Arc<CentralAuthDb>,
}

impl ServiceConnections {
    /// Ouvre la base auth. `base_path` = répertoire des données.
    pub fn open(base_path: &Path) -> Result<Self, String> {
        let auth_db = CentralAuthDb::open(base_path.join("central.db"))
            .map_err(|e| format!("Central auth DB: {e}"))?;

        Ok(Self {
            auth_db: Arc::new(auth_db),
        })
    }
}

/// Pseudo ou email affiché pour le profil connecté.
pub fn profile_display_name(profile: &CentralProfile) -> String {
    profile
        .pseudonyme
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or(profile.email.as_str())
        .to_string()
}

/// Raccourci Dioxus pour accéder aux connexions DB depuis n'importe quel composant.
pub fn use_service_connections() -> Signal<Arc<ServiceConnections>> {
    use_context::<crate::state::AppContext>().connections
}
