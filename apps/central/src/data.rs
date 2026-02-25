//! Couche donnees : connexions KindMother DB pour tous les services + auth Central.
//! Les data layers compilées via miyukini-central sont toujours disponibles.
//! Seuls miyukiniwatch, jay1tribu, jaymanga sont vraiment optionnels.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use dioxus::prelude::*;
use jayxpose::data::JayXposeDb;
use jaykonta::data::JayKontaDb;
use jayfestival::data::JayFestivalDb;
use jaykoa::data::JayKoaDb;
#[cfg(feature = "service-jay1tribu")]
use jay1tribu::Jay1TribuDb;
#[cfg(feature = "service-jaymanga")]
use jaymanga::data::JayMangaDb;
use miyukini_central::auth::{CentralAuthDb, CentralProfile};
#[cfg(feature = "service-miyukiniwatch")]
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
    /// Repertoire des sauvegardes MiyuClicker / base path.
    pub miyuclicker_data_dir: PathBuf,
    /// Base MiyukiniWatch (métriques d'usage, habitudes, agrégats Miou).
    #[cfg(feature = "service-miyukiniwatch")]
    pub miyukiniwatch: Arc<MiyukiniWatchDb>,
    /// Base Jay1Tribu (tribus, salons, amis, messages).
    #[cfg(feature = "service-jay1tribu")]
    pub jay1tribu: Arc<Jay1TribuDb>,
    /// Base JayManga (catalogue manga, lecteur, ventes, agrégation).
    #[cfg(feature = "service-jaymanga")]
    pub jaymanga: Arc<JayMangaDb>,
}

impl ServiceConnections {
    /// Ouvre toutes les bases de donnees activées. `base_path` = racine workspace.
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
            #[cfg(feature = "service-miyukiniwatch")]
            miyukiniwatch: Arc::new(
                MiyukiniWatchDb::open(base_path.join("miyukiniwatch.db"))
                    .map_err(|e| format!("MiyukiniWatch DB: {e}"))?,
            ),
            #[cfg(feature = "service-jay1tribu")]
            jay1tribu: Arc::new(
                Jay1TribuDb::open(base_path.join("jay1tribu.db"))
                    .map_err(|e| format!("Jay1Tribu DB: {e}"))?,
            ),
            #[cfg(feature = "service-jaymanga")]
            jaymanga: Arc::new(
                JayMangaDb::open(base_path.join("jaymanga.db"))
                    .map_err(|e| format!("JayManga DB: {e}"))?,
            ),
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
