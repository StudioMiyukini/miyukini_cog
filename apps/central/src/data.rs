//! Couche donnees : connexions KindMother DB pour tous les services + auth Central.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use dioxus::prelude::*;
use jayxpose::data::JayXposeDb;
use jaykonta::data::JayKontaDb;
use jayfestival::data::JayFestivalDb;
use jaykoa::data::JayKoaDb;
use miyukini_central::auth::{CentralAuthDb, CentralProfile};

/// Connexions DB partagees pour tous les services du hub.
pub struct ServiceConnections {
    /// Base auth Central (profils, COG vierge, session).
    pub auth_db: Arc<CentralAuthDb>,
    /// Base JayXpose (profil exposant, catalogue, vitrine, documents).
    pub jayxpose: Arc<JayXposeDb>,
    /// Base JayKonta (comptabilite unifiee Purse + Account).
    pub jaykonta: Arc<JayKontaDb>,
    /// Base JayFestival (editions, organisateurs, exposants).
    pub jayfestival: Arc<JayFestivalDb>,
    /// Base JayKoa (calendrier universel, agendas, entries temporelles).
    pub jaykoa: Arc<JayKoaDb>,
    /// Repertoire des sauvegardes MiyuClicker.
    pub miyuclicker_data_dir: PathBuf,
}

impl ServiceConnections {
    /// Ouvre toutes les bases de donnees. `base_path` = racine workspace.
    pub fn open(base_path: &Path) -> Result<Self, String> {
        let auth_db = CentralAuthDb::open(base_path.join("central.db"))
            .map_err(|e| format!("Central auth DB: {e}"))?;
        let jayxpose = JayXposeDb::open(base_path.join("jayxpose.db"))
            .map_err(|e| format!("JayXpose DB: {e}"))?;
        let jaykonta = JayKontaDb::open(base_path.join("jaykonta.db"))
            .map_err(|e| format!("JayKonta DB: {e}"))?;
        let jayfestival = JayFestivalDb::open(base_path.join("jayfestival.db"))
            .map_err(|e| format!("JayFestival DB: {e}"))?;
        let jaykoa = JayKoaDb::open(base_path.join("jaykoa.db"))
            .map_err(|e| format!("JayKoa DB: {e}"))?;

        Ok(Self {
            auth_db: Arc::new(auth_db),
            jayxpose: Arc::new(jayxpose),
            jaykonta: Arc::new(jaykonta),
            jayfestival: Arc::new(jayfestival),
            jaykoa: Arc::new(jaykoa),
            miyuclicker_data_dir: base_path.to_path_buf(),
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

/// Raccourci Dioxus pour acceder aux connexions DB depuis n'importe quel composant.
pub fn use_service_connections() -> Signal<Arc<ServiceConnections>> {
    use_context::<crate::state::AppContext>().connections
}
