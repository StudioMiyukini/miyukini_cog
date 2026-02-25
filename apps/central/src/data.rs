//! Couche donnees : connexions KindMother DB pour tous les services + auth Central.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use dioxus::prelude::*;
use jayxpose::data::JayXposeDb;
use jaykonta::data::JayKontaDb;
use jayfestival::data::JayFestivalDb;
use jaykoa::data::JayKoaDb;
use jay1tribu::Jay1TribuDb;
use jaymanga::data::JayMangaDb;
use miyukini_central::auth::{CentralAuthDb, CentralProfile};
use miyukiniwatch::MiyukiniWatchDb;

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
    /// Base MiyukiniWatch (métriques d'usage, habitudes, agrégats Miou).
    pub miyukiniwatch: Arc<MiyukiniWatchDb>,
    /// Base Jay1Tribu (tribus, salons, amis, messages — chat/tribu pleins uniquement si Webway connecté).
    pub jay1tribu: Arc<Jay1TribuDb>,
    /// Base JayManga (catalogue manga, lecteur, ventes, agrégation).
    pub jaymanga: Arc<JayMangaDb>,
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
        let miyukiniwatch = MiyukiniWatchDb::open(base_path.join("miyukiniwatch.db"))
            .map_err(|e| format!("MiyukiniWatch DB: {e}"))?;
        let jay1tribu = Jay1TribuDb::open(base_path.join("jay1tribu.db"))
            .map_err(|e| format!("Jay1Tribu DB: {e}"))?;
        let jaymanga = JayMangaDb::open(base_path.join("jaymanga.db"))
            .map_err(|e| format!("JayManga DB: {e}"))?;

        Ok(Self {
            auth_db: Arc::new(auth_db),
            jayxpose: Arc::new(jayxpose),
            jaykonta: Arc::new(jaykonta),
            jayfestival: Arc::new(jayfestival),
            jaykoa: Arc::new(jaykoa),
            miyukiniwatch: Arc::new(miyukiniwatch),
            jay1tribu: Arc::new(jay1tribu),
            jaymanga: Arc::new(jaymanga),
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
