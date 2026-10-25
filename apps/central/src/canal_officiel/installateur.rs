//! @id     central_canal_officiel_installateur
//! @do     installer_maj_canal_officiel
//! @role   securite
//! @layer  app
//! @human  central — installateur du canal officiel (Cores par le chemin StrongFather existant, OS refusé, modèles IA délégués).

//! [`Installateur`] du canal officiel côté Central (lot 4, A-006).
//!
//! | Catégorie | Traitement |
//! |---|---|
//! | `Cores` | paquet `.msp` par le chemin gouverné existant : [`installer_depuis_canal`] (signature éditeur officiel, opération StrongFather explicite `mettre_a_jour_canal`, arrêt du service, réinstallation) |
//! | `Os` | **refus** : Central n'a pas d'installateur d'OS (l'annonce reste en attente, refus signalé) |
//! | `ModeleIa` | délégué à l'installateur de maia-llm (artefact `MMIA`, [`super::modeles`]), branché par [`InstallateurCentral::avec_modeles`] si `modeles` est configuré ; aucun ⇒ refus |
//!
//! Version installée (`Cores`) : manifeste du registre des services, convertie
//! par [`version_canal`] ; version illisible ⇒ `u64::MAX` (aucune
//! installation, fail-closed).

use std::path::PathBuf;
use std::sync::{Arc, Mutex, PoisonError};

use miyucanal::{Installateur, MiseAJourVerifiee};
use miyuregistre::Categorie;
use miyusceau::AncresDeConfiance;

use crate::service_manager::gouvernance::GouvernanceMarket;
use crate::service_manager::installer::installer_depuis_canal;
use crate::service_manager::launcher::ServiceProcesses;
use crate::service_manager::registry::{self, ServiceRegistryData};
use crate::service_manager::{confiance, version_canal, ServiceManager};

/// Motif de refus des mises à jour d'OS.
pub const REFUS_OS: &str = "aucun installateur d'OS dans Central : correctif d'OS refusé";
/// Motif de refus des modèles IA sans installateur branché.
pub const REFUS_MODELES: &str = "aucun installateur de modèles IA branché (maia-llm) : modèle refusé";

/// Installateur du canal officiel adossé au cycle de vie des services de Central.
pub struct InstallateurCentral {
    registre: Arc<Mutex<ServiceRegistryData>>,
    processus: Arc<ServiceProcesses>,
    gouvernance: GouvernanceMarket,
    racine: PathBuf,
    /// Fichier du registre à réécrire après installation (`None` : mémoire seule).
    fichier_registre: Option<PathBuf>,
    ancres: AncresDeConfiance,
    modeles: Option<Box<dyn Installateur + Send>>,
}

impl std::fmt::Debug for InstallateurCentral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InstallateurCentral")
            .field("racine", &self.racine)
            .field("modeles", &self.modeles.is_some())
            .finish_non_exhaustive()
    }
}

impl InstallateurCentral {
    /// Installateur explicite (racine des services, registre, porte, éditeurs `.msp`).
    pub fn new(
        registre: Arc<Mutex<ServiceRegistryData>>,
        processus: Arc<ServiceProcesses>,
        gouvernance: GouvernanceMarket,
        racine: PathBuf,
        fichier_registre: Option<PathBuf>,
        ancres: AncresDeConfiance,
    ) -> Self {
        Self { registre, processus, gouvernance, racine, fichier_registre, ancres, modeles: None }
    }

    /// Installateur du Service Manager de Central (dossiers et éditeurs de production).
    pub fn depuis_service_manager(sm: &ServiceManager) -> Self {
        Self::new(
            sm.registry.clone(),
            sm.processes.clone(),
            sm.gouvernance.clone(),
            registry::services_root_dir(),
            Some(registry::registry_path()),
            confiance::ancres_market(),
        )
    }

    /// Branche l'installateur des modèles IA.
    #[must_use]
    pub fn avec_modeles(mut self, modeles: Box<dyn Installateur + Send>) -> Self {
        self.modeles = Some(modeles);
        self
    }

    fn installer_cores(&self, maj: &MiseAJourVerifiee) -> Result<(), String> {
        let service_id = maj.composant();
        let mut reg = self.registre.lock().unwrap_or_else(PoisonError::into_inner);
        let processus = &self.processus;
        installer_depuis_canal(
            maj.artefact(),
            service_id,
            maj.version(),
            &self.ancres,
            confiance::maintenant_secs(),
            &self.gouvernance,
            &mut reg,
            &self.racine,
            || {
                if processus.is_running(service_id) {
                    processus.stop(service_id)?;
                }
                Ok(())
            },
        )?;
        if let Some(f) = &self.fichier_registre {
            registry::save_registry_to(f, &reg)?;
        }
        Ok(())
    }
}

impl Installateur for InstallateurCentral {
    fn version_installee(&self, categorie: Categorie, composant: &str) -> Option<u64> {
        match categorie {
            Categorie::Cores => {
                let reg = self.registre.lock().unwrap_or_else(PoisonError::into_inner);
                let service = reg.services.get(composant)?;
                Some(version_canal(&service.manifest.version).unwrap_or_else(|| {
                    tracing::warn!(
                        "canal officiel : version installée « {} » de {composant} illisible — aucune mise à jour",
                        service.manifest.version
                    );
                    u64::MAX
                }))
            }
            Categorie::ModeleIa => self.modeles.as_ref().and_then(|m| m.version_installee(categorie, composant)),
            Categorie::Os => None,
        }
    }

    fn installer(&mut self, maj: &MiseAJourVerifiee) -> Result<(), String> {
        match maj.categorie() {
            Categorie::Cores => self.installer_cores(maj),
            Categorie::Os => Err(REFUS_OS.to_string()),
            Categorie::ModeleIa => match self.modeles.as_mut() {
                Some(m) => m.installer(maj),
                None => Err(REFUS_MODELES.to_string()),
            },
        }
    }
}
