//! @id     miyuhybride_service
//! @do     installer_exposer_relancer_service_hybride
//! @role   securite
//! @layer  toolkit
//! @human  miyuhybride — cycle de vie d'un service hybride côté Cores : installation (voie désactivée), exposition explicite, session à façade neuve, supervision et reconstruction des façades (EX-43, EX-48, INV-H5, INV-H6).

//! # Cycle de vie (SERVICES-HYBRIDES §4)
//!
//! - **Installation** ([`ServiceHybride::installer`]) : manifeste vérifié,
//!   voie déclarée au Portail, **désactivée** (EX-48) ;
//! - **Exposition** : action explicite de l'utilisateur, service par service
//!   ([`ServiceHybride::activer_exposition`]) ;
//! - **Session** ([`ServiceHybride::ouvrir_session`]) : image de façade
//!   conforme au manifeste (INV-H5), clé de session **éphémère générée en
//!   zone COG** (INV-H6), auteurs épinglés, DB secondaire **vierge** ; le tout
//!   remis à la façade dans un [`LancementFacade`] ;
//! - **Appareils** ([`ServiceHybride::ajouter_auteur`],
//!   [`ServiceHybride::retirer_auteur`]) : un appareil de plus (ou de moins)
//!   est épinglé dans la session **ouverte**, sans reconstruire la façade ;
//!   le Portail tient une séquence anti-rejeu **par auteur** (A-007) ;
//! - **Quotas** ([`ServiceHybride::quotas`]) : disque, sessions et tâches du
//!   contrat, comptés par service et remis à chaque façade (crate::quotas) ;
//! - **Fin de session / coupure** : façade détruite, DB purgée ; la façade
//!   n'est jamais réparée, seulement reconstruite ([`Superviseur`]) ;
//! - **Désinstallation** : voie supprimée, aucun résidu.

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard, RwLock};

use ed25519_dalek::SigningKey;
use miyuportail::{ClasseExposition, ErreurPortail, EtatVoie, IdService, PortailInterne, Traite};
use rand_core::OsRng;

use crate::db_secondaire::DbSecondaire;
use crate::erreurs::ErreurHybride;
use crate::guichets::{AuteursSession, GuichetFranche, GuichetMoteur};
use crate::manifeste::{ContratFacade, ManifesteVerifie};
use crate::quotas::QuotasService;

const MIO: usize = 1024 * 1024;

/// Ce que la zone COG remet à une façade neuve (et rien d'autre).
#[derive(Debug)]
pub struct LancementFacade {
    /// Guichet Franche de la voie (clé de session éphémère incluse).
    pub guichet: GuichetFranche,
    /// DB secondaire vierge, bornée par le contrat.
    pub db: DbSecondaire,
    /// Audience des visas acceptés par la façade.
    pub audience: String,
    /// Durée maximale de la session (secondes).
    pub duree_max_session_secs: u64,
    /// Quotas du contrat (disque, sessions, tâches), partagés par toutes les
    /// façades du service ; la DB secondaire y est déjà adossée.
    pub quotas: QuotasService,
}

/// Service hybride installé (côté Cores, zone COG).
pub struct ServiceHybride {
    portail: Arc<PortailInterne>,
    manifeste: ManifesteVerifie,
    traite: Traite,
    facade: ContratFacade,
    quotas: QuotasService,
    epoque: Arc<AtomicU64>,
    /// Auteurs de la session ouverte, partagés avec le guichet de sa façade.
    session: Mutex<Option<AuteursSession>>,
}

impl std::fmt::Debug for ServiceHybride {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ServiceHybride").field("service", self.manifeste.service()).finish_non_exhaustive()
    }
}

impl ServiceHybride {
    /// Installe un service H1/H2 : déclare sa voie (désactivée). H0 : refus (jamais de voie).
    pub fn installer(portail: Arc<PortailInterne>, manifeste: ManifesteVerifie) -> Result<Self, ErreurHybride> {
        let contrat = manifeste.contrat_voies().ok_or(ErreurHybride::Classe)?;
        let facade = manifeste.contrat().facade.ok_or(ErreurHybride::Classe)?;
        let traite = contrat.traite.clone();
        let quotas = QuotasService::depuis(&manifeste.contrat().ressources);
        portail.declarer(contrat)?;
        Ok(Self {
            portail,
            manifeste,
            traite,
            facade,
            quotas,
            epoque: Arc::new(AtomicU64::new(0)),
            session: Mutex::new(None),
        })
    }

    fn session(&self) -> Result<MutexGuard<'_, Option<AuteursSession>>, ErreurHybride> {
        self.session.lock().map_err(|_| ErreurHybride::EtatIndisponible)
    }

    /// Service.
    pub fn service(&self) -> &IdService {
        self.manifeste.service()
    }

    /// Manifeste vérifié.
    pub fn manifeste(&self) -> &ManifesteVerifie {
        &self.manifeste
    }

    /// Quotas du contrat appliqués au service (compteurs partagés).
    pub fn quotas(&self) -> &QuotasService {
        &self.quotas
    }

    /// État de la voie.
    pub fn etat(&self) -> Result<EtatVoie, ErreurHybride> {
        Ok(self.portail.etat(self.service())?)
    }

    /// Active l'exposition (action explicite de l'utilisateur, EX-48).
    pub fn activer_exposition(&self) -> Result<(), ErreurHybride> {
        Ok(self.portail.activer(self.service())?)
    }

    /// Désactive l'exposition : session close, files purgées.
    pub fn desactiver_exposition(&self) -> Result<(), ErreurHybride> {
        self.epoque.fetch_add(1, Ordering::SeqCst);
        self.session()?.take();
        Ok(self.portail.desactiver(self.service())?)
    }

    /// Rétablit une voie coupée (décision explicite) : elle repasse désactivée.
    pub fn retablir(&self) -> Result<(), ErreurHybride> {
        Ok(self.portail.retablir(self.service())?)
    }

    /// Guichet du moteur (zone COG).
    pub fn guichet_moteur(&self) -> GuichetMoteur {
        GuichetMoteur::new(Arc::clone(&self.portail), self.service().clone(), Arc::clone(&self.epoque))
    }

    /// Ouvre une session à façade neuve : image conforme au manifeste,
    /// clé de session éphémère, auteurs épinglés, DB secondaire vierge.
    /// Refusé si l'exposition n'est pas active.
    pub fn ouvrir_session(&self, image: &[u8; 32], auteurs: Vec<[u8; 32]>) -> Result<LancementFacade, ErreurHybride> {
        if image != &self.facade.image {
            return Err(ErreurHybride::ImageNonConforme);
        }
        match self.etat()? {
            EtatVoie::Active => {}
            EtatVoie::Desactivee => return Err(ErreurPortail::VoieDesactivee.into()),
            EtatVoie::Coupee(_) => return Err(ErreurPortail::VoieCoupee.into()),
        }
        let mut session = self.session()?;
        // Quota disque : la DB secondaire est comptée avant toute écriture.
        let db = DbSecondaire::avec_quota((self.facade.db_secondaire_max_mio as usize).saturating_mul(MIO), &self.quotas)?;
        let cle_session = SigningKey::generate(&mut OsRng);
        if self.manifeste.classe() != ClasseExposition::H1 {
            self.portail.ouvrir_session(self.service(), cle_session.verifying_key().to_bytes(), auteurs.clone())?;
        }
        self.epoque.fetch_add(1, Ordering::SeqCst);
        let partages: AuteursSession = Arc::new(RwLock::new(auteurs));
        *session = Some(Arc::clone(&partages));
        let guichet = GuichetFranche::new(
            Arc::clone(&self.portail),
            self.service().clone(),
            self.traite.clone(),
            cle_session,
            partages,
        )?;
        Ok(LancementFacade {
            guichet,
            db,
            audience: self.manifeste.audience().to_string(),
            duree_max_session_secs: self.facade.duree_max_session_secs,
            quotas: self.quotas.clone(),
        })
    }

    /// Épingle un appareil de plus dans la session ouverte (A-007) : la façade
    /// en service l'accepte aussitôt, sans être reconstruite ; sa séquence
    /// anti-rejeu est propre (neuve, ou reprise au-delà de sa marque haute
    /// s'il avait été retiré de cette session). Le guichet des visas est
    /// inchangé : l'appareil doit toujours présenter visa et PoP.
    pub fn ajouter_auteur(&self, auteur: [u8; 32]) -> Result<(), ErreurHybride> {
        let session = self.session()?;
        let auteurs = session.as_ref().ok_or(ErreurPortail::SessionAbsente)?;
        let mut a = auteurs.write().map_err(|_| ErreurHybride::EtatIndisponible)?;
        if a.contains(&auteur) {
            return Err(ErreurPortail::AuteurDejaEpingle.into());
        }
        if self.manifeste.classe() != ClasseExposition::H1 {
            self.portail.ajouter_auteur(self.service(), auteur)?;
        }
        a.push(auteur);
        Ok(())
    }

    /// Retire un appareil de la session ouverte (A-007) : ses trames sont
    /// refusées par le Portail (même en vol), la façade le refuse ; les autres
    /// appareils continuent. Le dernier appareil ne se retire pas.
    pub fn retirer_auteur(&self, auteur: &[u8; 32]) -> Result<(), ErreurHybride> {
        let session = self.session()?;
        let auteurs = session.as_ref().ok_or(ErreurPortail::SessionAbsente)?;
        let mut a = auteurs.write().map_err(|_| ErreurHybride::EtatIndisponible)?;
        if !a.contains(auteur) {
            return Err(ErreurPortail::AuteurNonEpingle.into());
        }
        if a.len() <= 1 {
            return Err(ErreurPortail::TransitionRefusee.into());
        }
        if self.manifeste.classe() != ClasseExposition::H1 {
            self.portail.retirer_auteur(self.service(), auteur)?;
        }
        a.retain(|x| x != auteur);
        Ok(())
    }

    /// Ferme la session (quarantaine purgée, réponses en attente périmées).
    pub fn fermer_session(&self) -> Result<(), ErreurHybride> {
        self.epoque.fetch_add(1, Ordering::SeqCst);
        self.session()?.take();
        Ok(self.portail.fermer_session(self.service())?)
    }

    /// Désinstalle : voie supprimée, aucun résidu.
    pub fn desinstaller(self) -> Result<(), ErreurHybride> {
        self.epoque.fetch_add(1, Ordering::SeqCst);
        self.session()?.take();
        Ok(self.portail.supprimer(self.service())?)
    }
}

/// Façade jetable : la zone COG la construit depuis un [`LancementFacade`],
/// ne la répare jamais et la détruit dès qu'elle se déclare condamnée.
pub trait FacadeJetable {
    /// Vrai si la façade a constaté une coupure, une désactivation ou une
    /// fin de session : elle refuse tout et attend sa destruction.
    fn condamnee(&self) -> bool;
}

/// Supervision des façades d'un service (Cores) : lance, détruit, reconstruit.
pub struct Superviseur<F: FacadeJetable> {
    service: Arc<ServiceHybride>,
    image: [u8; 32],
    auteurs: Vec<[u8; 32]>,
    fabrique: Box<dyn Fn(LancementFacade) -> F + Send + Sync>,
    facade: Option<F>,
    lancements: u64,
}

impl<F: FacadeJetable> std::fmt::Debug for Superviseur<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Superviseur")
            .field("service", self.service.service())
            .field("lancements", &self.lancements)
            .finish_non_exhaustive()
    }
}

impl<F: FacadeJetable> Superviseur<F> {
    /// Superviseur d'un service ; `image` = empreinte de l'image de façade à lancer.
    pub fn new(
        service: Arc<ServiceHybride>,
        image: [u8; 32],
        auteurs: Vec<[u8; 32]>,
        fabrique: impl Fn(LancementFacade) -> F + Send + Sync + 'static,
    ) -> Self {
        Self { service, image, auteurs, fabrique: Box::new(fabrique), facade: None, lancements: 0 }
    }

    /// Lance une façade neuve (détruit la précédente s'il y en a une).
    pub fn lancer(&mut self) -> Result<&mut F, ErreurHybride> {
        self.detruire();
        let lancement = self.service.ouvrir_session(&self.image, self.auteurs.clone())?;
        self.lancements += 1;
        Ok(self.facade.insert((self.fabrique)(lancement)))
    }

    /// Appareils épinglés à chaque lancement.
    pub fn auteurs(&self) -> &[[u8; 32]] {
        &self.auteurs
    }

    /// Épingle un appareil de plus : dans la session de la façade en service
    /// (sans la reconstruire, A-007), et pour les lancements suivants.
    pub fn ajouter_auteur(&mut self, auteur: [u8; 32]) -> Result<(), ErreurHybride> {
        if self.auteurs.contains(&auteur) {
            return Err(ErreurPortail::AuteurDejaEpingle.into());
        }
        if self.facade.is_some() {
            self.service.ajouter_auteur(auteur)?;
        }
        self.auteurs.push(auteur);
        Ok(())
    }

    /// Retire un appareil : de la session en service et des lancements suivants.
    pub fn retirer_auteur(&mut self, auteur: &[u8; 32]) -> Result<(), ErreurHybride> {
        if !self.auteurs.contains(auteur) {
            return Err(ErreurPortail::AuteurNonEpingle.into());
        }
        if self.auteurs.len() <= 1 {
            return Err(ErreurPortail::TransitionRefusee.into());
        }
        if self.facade.is_some() {
            self.service.retirer_auteur(auteur)?;
        }
        self.auteurs.retain(|a| a != auteur);
        Ok(())
    }

    /// Façade en service, s'il y en a une.
    pub fn facade(&mut self) -> Option<&mut F> {
        self.facade.as_mut()
    }

    /// Nombre de façades lancées.
    pub fn lancements(&self) -> u64 {
        self.lancements
    }

    /// Détruit la façade (DB secondaire purgée avec elle) et clôt la session.
    pub fn detruire(&mut self) {
        if self.facade.take().is_some() {
            // Voie coupée : la session est déjà close, l'erreur est attendue.
            let _ = self.service.fermer_session();
        }
    }

    /// Surveillance : une façade condamnée est **détruite** ; elle n'est
    /// reconstruite que si la voie est de nouveau active (rétablissement et
    /// activation explicites). Rend `true` si une façade neuve a été lancée.
    pub fn surveiller(&mut self) -> Result<bool, ErreurHybride> {
        if self.facade.as_ref().is_some_and(FacadeJetable::condamnee) {
            self.detruire();
        }
        if self.facade.is_some() {
            return Ok(false);
        }
        if self.service.etat()? != EtatVoie::Active {
            return Ok(false);
        }
        self.lancer()?;
        Ok(true)
    }
}
