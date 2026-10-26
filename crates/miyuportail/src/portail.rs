//! @id     miyuportail_portail
//! @do     gerer_voies_sens_unique_par_service
//! @role   securite
//! @layer  toolkit
//! @human  miyuportail — Portail interne : types communs des voies, table de voies générique, coupure ciblée et composition « processus unique » des deux moitiés entrée / sortie (EX-44, EX-47, EX-48, EX-49, A-022 §3).

//! # Portail interne
//!
//! SERVICES-HYBRIDES-ET-PORTAIL §3. Chaque service déclaré dans un contrat
//! reçoit **sa** voie : files, quotas, compteurs et clé de publication
//! distincts. Une voie a deux sens à sens unique, tenus par **deux moitiés
//! sans aucun état commun** (A-022 §3) :
//!
//! - **entrée** ([`crate::entree::PortailEntree`], quarantaine → zone COG) :
//!   la façade **dépose** des trames MSCE ; la zone COG **relève** à son
//!   rythme (« la zone COG tire », INV-Z2) ; chaque lot passe par le sas
//!   isolé, la deuxième validation, le CDR, le contrôle anti-rejeu et
//!   l'inscription au registre ; chaque déclaration admise porte un
//!   [`JetonReponse`] signé. L'entrée n'a **aucune** méthode de publication ;
//! - **sortie** ([`crate::sortie::PortailSortie`], zone COG → publication) :
//!   seules les réponses portant un jeton valide **non encore consommé**
//!   (H2) et les publications H1, données `session` ou `publiable`, par
//!   **action explicite de l'utilisateur**, au type déclaré par le traité,
//!   réencodées et signées par la clé de la voie. La sortie n'a **aucune**
//!   méthode de relève.
//!
//! En production, chaque moitié est un **processus** distinct
//! (`portail-entree`, `portail-sortie` : comptes, profils AppArmor et
//! sandboxes distincts), relié au reste par des files à sens unique
//! ([`crate::files`], [`crate::demon`]). [`PortailInterne`] compose les deux
//! moitiés dans **un** processus (essais, intégration des services) : il
//! garde chacune avec sa propre table de voies et ne partage entre elles que
//! la configuration que les Cores remettent à chacune (contrat, état,
//! session, ancre des jetons) — exactement ce que reçoivent les deux
//! processus.
//!
//! Aucun chemin retour : la façade ne voit que des **compteurs**.
//! Ce qui n'est pas déclaré n'existe pas ; une voie déclarée est
//! **désactivée** tant que l'utilisateur ne l'active pas (EX-48) ; une voie
//! **coupée** (WorrySentinel, EX-49) refuse tout sans toucher aux autres.
//!
//! ## Anti-rejeu par auteur (A-007)
//!
//! Une session épingle **plusieurs** auteurs (appareils, ≤ [`MAX_AUTEURS`]) ;
//! chacun a sa **propre** séquence strictement croissante, indexée par le
//! `kid` de sa clé ([`crate::entree`]).

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use ed25519_dalek::VerifyingKey;
use sha2::{Digest, Sha256};

use crate::cdr::{Desarmement, ObjetADesarmer};
use crate::entree::{PortailEntree, SessionEntree, VoieEntree};
use crate::erreurs::{ErreurPortail, RefusDecodage};
use crate::jeton::{nouvel_id_session, CleJetons, JetonReponse};
use crate::msce::{Champ, Declaration, Valeur, TAILLE_MAX_TRAME};
use crate::registre::{Empreinte, Evenement, Inscription, MotifCoupure};
use crate::sas::{DecodeurIsole, MAX_AUTEURS};
use crate::sortie::{PortailSortie, SessionSortie, VoieSortie};
use crate::traite::{ClasseExposition, IdService, NatureChamp, Traite};

/// Quota maximal d'une file.
pub const QUOTA_FILE_MAX: usize = 4096;
/// Fenêtre de débit.
pub const FENETRE_DEBIT: Duration = Duration::from_secs(60);
/// Nombre maximal d'auteurs **connus** d'une session (épinglés ou retirés,
/// dont la marque haute est conservée) : borne la mémoire de l'anti-rejeu.
pub const MAX_AUTEURS_CONNUS: usize = 8 * MAX_AUTEURS;

/// Contrat de voies d'un service (extrait du manifeste signé, déjà validé par StrongFather).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContratVoies {
    /// Service.
    pub service: IdService,
    /// Classe d'exposition.
    pub classe: ClasseExposition,
    /// Traité versionné.
    pub traite: Traite,
    /// Taille maximale d'une trame (≤ [`TAILLE_MAX_TRAME`]).
    pub taille_max_trame: usize,
    /// Débit maximal par sens (trames par minute).
    pub debit_max_par_minute: u32,
    /// Profondeur maximale de chaque file (≤ [`QUOTA_FILE_MAX`]).
    pub quota_file: usize,
}

impl ContratVoies {
    /// Contrôle du contrat (bornes, traité, classe).
    pub fn valider(&self) -> Result<(), ErreurPortail> {
        if self.classe == ClasseExposition::H0 {
            return Err(ErreurPortail::ClasseSansFacade);
        }
        self.traite.valider()?;
        if self.taille_max_trame == 0 || self.taille_max_trame > TAILLE_MAX_TRAME {
            return Err(ErreurPortail::ContratInvalide("taille de trame hors bornes"));
        }
        if self.debit_max_par_minute == 0 {
            return Err(ErreurPortail::ContratInvalide("débit nul"));
        }
        if self.quota_file == 0 || self.quota_file > QUOTA_FILE_MAX {
            return Err(ErreurPortail::ContratInvalide("quota de file hors bornes"));
        }
        if self.traite.types.iter().any(|t| !self.classe.autorise(t.sens)) {
            return Err(ErreurPortail::ContratInvalide("type déclaré dans un sens fermé à la classe"));
        }
        Ok(())
    }
}

/// Étiquette de sensibilité posée par KindMother (EX-47).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sensibilite {
    /// Ne sort jamais.
    Nationale,
    /// Visible des participants d'une session.
    Session,
    /// Diffusable.
    Publiable,
}

/// Origine de la demande de publication.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Declencheur {
    /// Action explicite de l'utilisateur (seule admise).
    Utilisateur,
    /// MAIA : ne peut rien publier.
    Maia,
    /// Service, sans action utilisateur.
    Service,
}

/// Demande de publication (zone COG → Franche).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Publication {
    /// Type de publication déclaré par le traité.
    pub type_code: u64,
    /// Champs.
    pub champs: Vec<Champ>,
    /// Étiquette KindMother.
    pub etiquette: Sensibilite,
    /// Origine.
    pub declencheur: Declencheur,
}

/// Déclaration admise en zone COG.
#[derive(Debug, PartialEq, Eq)]
pub struct DeclarationAdmise {
    /// Structure typée remise au moteur (documents désarmés).
    pub declaration: Declaration,
    /// SHA-256 de la trame reçue (inscrite au registre).
    pub empreinte: Empreinte,
    /// Jeton de réponse à usage unique, signé par l'entrée (A-022 §3).
    pub jeton: JetonReponse,
}

/// Compteurs d'une voie : seul retour visible de la façade.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CompteursVoie {
    /// Trames déposées par la façade.
    pub deposees: u64,
    /// Trames relevées par la zone COG (admises ou refusées).
    pub relevees: u64,
    /// Trames admises.
    pub admises: u64,
    /// Trames refusées.
    pub refusees: u64,
    /// Publications émises.
    pub publiees: u64,
    /// Publications lues par la façade.
    pub lues: u64,
}

/// État d'une voie.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EtatVoie {
    /// Déclarée, exposition désactivée (défaut, EX-48).
    Desactivee,
    /// Activée par l'utilisateur.
    Active,
    /// Coupée (EX-49) : tout est refusé jusqu'au rétablissement.
    Coupee(MotifCoupure),
}

// ---------------------------------------------------------------- commun aux deux moitiés

/// Fenêtre de débit d'un sens.
pub(crate) struct Fenetre {
    debut: Instant,
    compte: u32,
}

impl Fenetre {
    pub(crate) fn new() -> Self {
        Self { debut: Instant::now(), compte: 0 }
    }

    /// Consomme `n` unités (tout ou rien).
    pub(crate) fn consommer_n(&mut self, max: u32, n: u32) -> Result<(), ErreurPortail> {
        if self.debut.elapsed() >= FENETRE_DEBIT {
            *self = Self::new();
        }
        if self.compte.saturating_add(n) > max {
            return Err(ErreurPortail::DebitDepasse);
        }
        self.compte += n;
        Ok(())
    }

    pub(crate) fn consommer(&mut self, max: u32) -> Result<(), ErreurPortail> {
        self.consommer_n(max, 1)
    }
}

/// Source des identifiants d'instance de voie : jamais réutilisés dans le
/// processus, même après suppression et redéclaration du même service.
static PROCHAINE_INSTANCE: AtomicU64 = AtomicU64::new(0);

pub(crate) fn prochaine_instance() -> u64 {
    PROCHAINE_INSTANCE.fetch_add(1, Ordering::Relaxed)
}

/// Identité d'une voie à un instant : instance (déclaration) et génération
/// (session, coupure, désactivation). Toute opération faite hors verrou
/// (sas, CDR) n'est validée que si l'identité n'a pas changé.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Identite {
    pub(crate) instance: u64,
    pub(crate) generation: u64,
}

/// Une voie d'une moitié (entrée ou sortie) : cycle de vie commun.
pub(crate) trait VoieGeree: Send + 'static {
    fn nouvelle(contrat: ContratVoies) -> Self;
    fn contrat(&self) -> &ContratVoies;
    fn etat(&self) -> EtatVoie;
    fn poser_etat(&mut self, etat: EtatVoie);
    /// Incrémente la génération : lots et publications en vol refusés.
    fn nouvelle_generation(&mut self);
    /// Session close, files purgées (désactivation, coupure).
    fn purger(&mut self);
    /// Effet propre au rétablissement (sortie : nouvelle clé de publication).
    fn au_retablissement(&mut self) {}

    fn exiger(&self, sens: crate::traite::Sens) -> Result<(), ErreurPortail> {
        if !self.contrat().classe.autorise(sens) {
            return Err(ErreurPortail::SensRefuse);
        }
        match self.etat() {
            EtatVoie::Active => Ok(()),
            EtatVoie::Desactivee => Err(ErreurPortail::VoieDesactivee),
            EtatVoie::Coupee(_) => Err(ErreurPortail::VoieCoupee),
        }
    }
}

/// Table des voies d'une moitié. Le verrou **global** (la table) n'est tenu
/// que le temps d'une recherche, d'une déclaration ou d'une suppression ;
/// chaque voie a son **propre** verrou, sous lequel se font ses contrôles et
/// son inscription au registre : une inscription lente ne bloque que sa voie
/// (B8). Ordre des verrous : table puis voie, jamais l'inverse.
pub(crate) struct Voies<V>(Arc<Mutex<Table<V>>>);

/// Voies d'une moitié, chacune sous son propre verrou.
type Table<V> = BTreeMap<IdService, Arc<Mutex<V>>>;

impl<V> Clone for Voies<V> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

pub(crate) fn verrouiller_voie<V>(voie: &Mutex<V>) -> Result<MutexGuard<'_, V>, ErreurPortail> {
    voie.lock().map_err(|_| ErreurPortail::EtatIndisponible)
}

impl<V: VoieGeree> Voies<V> {
    pub(crate) fn new() -> Self {
        Self(Arc::new(Mutex::new(BTreeMap::new())))
    }

    fn table(&self) -> Result<MutexGuard<'_, Table<V>>, ErreurPortail> {
        self.0.lock().map_err(|_| ErreurPortail::EtatIndisponible)
    }

    /// Voie d'un service ; le verrou global est relâché au retour.
    pub(crate) fn voie(&self, service: &IdService) -> Result<Arc<Mutex<V>>, ErreurPortail> {
        self.table()?.get(service).cloned().ok_or(ErreurPortail::VoieNonDeclaree)
    }

    /// Services déclarés.
    pub(crate) fn services(&self) -> Result<Vec<IdService>, ErreurPortail> {
        Ok(self.table()?.keys().cloned().collect())
    }

    /// Déclare les voies d'un manifeste : tout ou rien.
    pub(crate) fn declarer_manifeste(&self, contrats: &[ContratVoies]) -> Result<(), ErreurPortail> {
        let mut v = self.table()?;
        for (i, c) in contrats.iter().enumerate() {
            c.valider()?;
            if v.contains_key(&c.service) || contrats[..i].iter().any(|d| d.service == c.service) {
                return Err(ErreurPortail::VoieDejaDeclaree);
            }
        }
        for c in contrats {
            v.insert(c.service.clone(), Arc::new(Mutex::new(V::nouvelle(c.clone()))));
        }
        Ok(())
    }

    pub(crate) fn supprimer(&self, service: &IdService) -> Result<(), ErreurPortail> {
        self.table()?.remove(service).map(|_| ()).ok_or(ErreurPortail::VoieNonDeclaree)
    }

    pub(crate) fn activer(&self, service: &IdService) -> Result<(), ErreurPortail> {
        let partagee = self.voie(service)?;
        let mut voie = verrouiller_voie(&partagee)?;
        match voie.etat() {
            EtatVoie::Desactivee => {
                voie.poser_etat(EtatVoie::Active);
                Ok(())
            }
            EtatVoie::Active => Ok(()),
            EtatVoie::Coupee(_) => Err(ErreurPortail::TransitionRefusee),
        }
    }

    pub(crate) fn desactiver(&self, service: &IdService) -> Result<(), ErreurPortail> {
        let partagee = self.voie(service)?;
        let mut voie = verrouiller_voie(&partagee)?;
        if matches!(voie.etat(), EtatVoie::Coupee(_)) {
            return Err(ErreurPortail::TransitionRefusee);
        }
        voie.poser_etat(EtatVoie::Desactivee);
        voie.nouvelle_generation();
        voie.purger();
        Ok(())
    }

    pub(crate) fn retablir(&self, service: &IdService) -> Result<(), ErreurPortail> {
        let partagee = self.voie(service)?;
        let mut voie = verrouiller_voie(&partagee)?;
        if !matches!(voie.etat(), EtatVoie::Coupee(_)) {
            return Err(ErreurPortail::TransitionRefusee);
        }
        voie.poser_etat(EtatVoie::Desactivee);
        voie.au_retablissement();
        Ok(())
    }

    /// Coupe la voie (sans inscription : l'appelant inscrit une fois).
    pub(crate) fn couper(&self, service: &IdService, motif: MotifCoupure) -> Result<(), ErreurPortail> {
        let partagee = self.voie(service)?;
        let mut voie = verrouiller_voie(&partagee)?;
        voie.poser_etat(EtatVoie::Coupee(motif));
        voie.nouvelle_generation();
        voie.purger();
        Ok(())
    }

    pub(crate) fn etat(&self, service: &IdService) -> Result<EtatVoie, ErreurPortail> {
        let partagee = self.voie(service)?;
        let etat = verrouiller_voie(&partagee)?.etat();
        Ok(etat)
    }
}

pub(crate) fn empreinte(octets: &[u8]) -> Empreinte {
    Sha256::digest(octets).into()
}

pub(crate) fn cle_valide(cle: &[u8; 32]) -> Result<(), ErreurPortail> {
    let vk = VerifyingKey::from_bytes(cle).map_err(|_| ErreurPortail::CleInvalide)?;
    if vk.is_weak() {
        return Err(ErreurPortail::CleInvalide);
    }
    Ok(())
}

/// Désarme chaque champ `Document` (EX-45) ; tout refus refuse le message.
pub(crate) fn desarmer(
    cdr: &dyn Desarmement,
    mut d: Declaration,
    traite: &Traite,
) -> Result<Declaration, ErreurPortail> {
    let ty = traite.type_pour(d.sens, d.type_code).ok_or(RefusDecodage::TypeDeclaration)?;
    for champ in &mut d.champs {
        if let Valeur::Document(octets) = &champ.valeur {
            let Some(NatureChamp::Document { max, nature, mime }) = ty.champ(champ.tag).map(|c| c.nature) else {
                return Err(ErreurPortail::Desarmement);
            };
            let objet = ObjetADesarmer {
                service: &d.service,
                sens: d.sens,
                type_code: d.type_code,
                tag: champ.tag,
                nature,
                mime,
                taille_max: max as usize,
                octets,
            };
            let reconstruit = cdr.desarmer(&objet).map_err(|_| ErreurPortail::Desarmement)?;
            if reconstruit.octets.len() > max as usize {
                return Err(ErreurPortail::Desarmement);
            }
            champ.valeur = Valeur::Document(reconstruit.octets);
        }
    }
    Ok(d)
}

/// Secondes Unix courantes (0 si l'horloge précède l'époque).
pub(crate) fn maintenant() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

// ---------------------------------------------------------------- coupure (EX-49)

/// Coupure d'une voie (EX-49) : interface offerte à WorrySentinel.
pub trait CoupureVoie: Send + Sync {
    /// Coupe la voie du service : session close, files purgées, incident
    /// inscrit. La coupure est effective même si l'inscription échoue
    /// (l'erreur est alors renvoyée).
    fn couper(&self, service: &IdService, motif: MotifCoupure) -> Result<(), ErreurPortail>;
}

/// Poignée de coupure : ne permet **que** de couper (jamais de rétablir).
/// Celle d'une moitié ne coupe que cette moitié ; celle de
/// [`PortailInterne`] coupe les deux sens.
#[derive(Clone)]
pub struct PoigneeCoupure {
    entree: Option<Voies<VoieEntree>>,
    sortie: Option<Voies<VoieSortie>>,
    registre: Arc<dyn Inscription>,
}

impl std::fmt::Debug for PoigneeCoupure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PoigneeCoupure").finish_non_exhaustive()
    }
}

impl PoigneeCoupure {
    pub(crate) fn new(
        entree: Option<Voies<VoieEntree>>,
        sortie: Option<Voies<VoieSortie>>,
        registre: Arc<dyn Inscription>,
    ) -> Self {
        Self { entree, sortie, registre }
    }
}

impl CoupureVoie for PoigneeCoupure {
    fn couper(&self, service: &IdService, motif: MotifCoupure) -> Result<(), ErreurPortail> {
        // Les deux sens sont coupés avant toute erreur : une moitié coupée
        // n'attend pas l'autre (voie entière, deux sens).
        let e = self.entree.as_ref().map(|v| v.couper(service, motif));
        let s = self.sortie.as_ref().map(|v| v.couper(service, motif));
        if let Some(r) = e {
            r?;
        }
        if let Some(r) = s {
            r?;
        }
        self.registre
            .inscrire(&Evenement::Coupure { service: service.clone(), motif })
            .map_err(|_| ErreurPortail::Inscription)
    }
}

// ---------------------------------------------------------------- processus unique

/// Portail interne en **un** processus : composition des deux moitiés
/// ([`PortailEntree`], [`PortailSortie`]), chacune avec sa table de voies.
///
/// En production, les deux moitiés tournent dans deux processus
/// (`portail-entree`, `portail-sortie`, A-022 §3) ; ce type sert aux essais
/// et à l'intégration des services. Les Cores (ici : ce type) remettent à
/// chaque moitié sa configuration : contrat, état, session et, pour une
/// session, la graine des jetons à l'entrée et leur ancre à la sortie.
pub struct PortailInterne {
    entree: PortailEntree,
    sortie: PortailSortie,
    registre: Arc<dyn Inscription>,
}

impl std::fmt::Debug for PortailInterne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PortailInterne").finish_non_exhaustive()
    }
}

impl CoupureVoie for PortailInterne {
    fn couper(&self, service: &IdService, motif: MotifCoupure) -> Result<(), ErreurPortail> {
        self.poignee_coupure().couper(service, motif)
    }
}

/// Applique `op` aux deux moitiés ; la première erreur l'emporte (les deux
/// moitiés ont toujours la même table de voies et le même état).
fn les_deux(e: Result<(), ErreurPortail>, s: Result<(), ErreurPortail>) -> Result<(), ErreurPortail> {
    e.and(s)
}

impl PortailInterne {
    /// Portail sans voie. `sas` : en production, [`crate::sas::SasProcessus`] ;
    /// `cdr` : [`crate::cdr::RefusParDefaut`] tant que le CDR n'est pas branché.
    /// Le CDR sert aux deux sens : la sortie en reçoit une instance qui
    /// délègue à la même implémentation (sans état de voie).
    pub fn new(sas: Box<dyn DecodeurIsole>, cdr: Box<dyn Desarmement>, registre: Arc<dyn Inscription>) -> Self {
        let cdr: Arc<dyn Desarmement> = Arc::from(cdr);
        Self {
            entree: PortailEntree::new(sas, Box::new(CdrPartage(Arc::clone(&cdr))), Arc::clone(&registre)),
            sortie: PortailSortie::new(Box::new(CdrPartage(cdr)), Arc::clone(&registre)),
            registre,
        }
    }

    /// Moitié entrée (relève).
    pub fn entree(&self) -> &PortailEntree {
        &self.entree
    }

    /// Moitié sortie (publication).
    pub fn sortie(&self) -> &PortailSortie {
        &self.sortie
    }

    /// Poignée de coupure pour WorrySentinel (deux sens).
    pub fn poignee_coupure(&self) -> PoigneeCoupure {
        PoigneeCoupure::new(Some(self.entree.table()), Some(self.sortie.table()), Arc::clone(&self.registre))
    }

    /// Déclare les voies d'un manifeste : tout ou rien (les deux moitiés).
    pub fn declarer_manifeste(&self, contrats: &[ContratVoies]) -> Result<(), ErreurPortail> {
        self.entree.declarer_manifeste(contrats)?;
        if let Err(e) = self.sortie.declarer_manifeste(contrats) {
            for c in contrats {
                let _ = self.entree.supprimer(&c.service);
            }
            return Err(e);
        }
        Ok(())
    }

    /// Déclare la voie d'un service.
    pub fn declarer(&self, contrat: ContratVoies) -> Result<(), ErreurPortail> {
        self.declarer_manifeste(std::slice::from_ref(&contrat))
    }

    /// Supprime la voie (désactivation du service : aucun résidu).
    pub fn supprimer(&self, service: &IdService) -> Result<(), ErreurPortail> {
        les_deux(self.entree.supprimer(service), self.sortie.supprimer(service))
    }

    /// Active l'exposition (action explicite de l'utilisateur, EX-48).
    pub fn activer(&self, service: &IdService) -> Result<(), ErreurPortail> {
        les_deux(self.entree.activer(service), self.sortie.activer(service))
    }

    /// Désactive l'exposition : session close, files purgées.
    pub fn desactiver(&self, service: &IdService) -> Result<(), ErreurPortail> {
        les_deux(self.entree.desactiver(service), self.sortie.desactiver(service))
    }

    /// Rétablit une voie coupée (décision explicite, hors WorrySentinel) :
    /// elle repasse **désactivée**, avec une **nouvelle** clé de publication.
    pub fn retablir(&self, service: &IdService) -> Result<(), ErreurPortail> {
        les_deux(self.entree.retablir(service), self.sortie.retablir(service))
    }

    /// État d'une voie.
    pub fn etat(&self, service: &IdService) -> Result<EtatVoie, ErreurPortail> {
        self.entree.etat(service)
    }

    /// Ouvre une session : épingle la clé de session de la façade (émise par
    /// la zone COG, INV-H6) et les clés des auteurs autorisés ; tire
    /// l'identifiant de session et la paire de jetons, dont la graine va à
    /// l'entrée et l'ancre à la sortie.
    pub fn ouvrir_session(
        &self,
        service: &IdService,
        cle_facade: [u8; 32],
        auteurs: Vec<[u8; 32]>,
    ) -> Result<(), ErreurPortail> {
        let id = nouvel_id_session();
        let jetons = CleJetons::generer();
        let ancre = jetons.ancre();
        self.entree.ouvrir_session(service, SessionEntree { id, cle_facade, auteurs, cle_jetons: jetons })?;
        self.sortie.ouvrir_session(service, SessionSortie { id, ancre_jetons: ancre })
    }

    /// Épingle un auteur (appareil) de plus dans la session ouverte (A-007).
    pub fn ajouter_auteur(&self, service: &IdService, auteur: [u8; 32]) -> Result<(), ErreurPortail> {
        self.entree.ajouter_auteur(service, auteur)
    }

    /// Retire un auteur de la session ouverte (A-007).
    pub fn retirer_auteur(&self, service: &IdService, auteur: &[u8; 32]) -> Result<(), ErreurPortail> {
        self.entree.retirer_auteur(service, auteur)
    }

    /// Clés des auteurs épinglés de la session ouverte (côté Cores).
    pub fn auteurs(&self, service: &IdService) -> Result<Vec<[u8; 32]>, ErreurPortail> {
        self.entree.auteurs(service)
    }

    /// Ferme la session : quarantaine purgée, jetons périmés.
    pub fn fermer_session(&self, service: &IdService) -> Result<(), ErreurPortail> {
        les_deux(self.entree.fermer_session(service), self.sortie.fermer_session(service))
    }

    /// **Côté Franche** : la façade dépose une trame en quarantaine.
    pub fn deposer(&self, service: &IdService, trame: Vec<u8>) -> Result<(), ErreurPortail> {
        self.entree.deposer(service, trame)
    }

    /// **Côté zone COG** : relève jusqu'à `max` trames (au plus [`crate::sas::MAX_LOT`]).
    pub fn relever(
        &self,
        service: &IdService,
        max: usize,
    ) -> Result<Vec<Result<DeclarationAdmise, ErreurPortail>>, ErreurPortail> {
        self.entree.relever(service, max)
    }

    /// **Côté zone COG** : publie vers la Franche. Renvoie le numéro de publication.
    ///
    /// Mode processus unique (essais, intégration) : aucun jeton n'est exigé
    /// ici. Le processus `portail-sortie` n'expose pas ce chemin : il exige un
    /// jeton pour toute publication d'une voie H2 ([`PortailSortie::repondre`]).
    pub fn publier(&self, service: &IdService, publication: Publication) -> Result<u64, ErreurPortail> {
        self.sortie.publier_sans_jeton(service, publication)
    }

    /// **Côté zone COG** : répond à une déclaration admise (jeton consommé).
    pub fn repondre(
        &self,
        service: &IdService,
        jeton: &JetonReponse,
        publications: Vec<Publication>,
    ) -> Result<Vec<u64>, ErreurPortail> {
        self.sortie.repondre(service, jeton, publications)
    }

    /// **Côté Franche** : la façade lit la prochaine publication (trame MSCE neuve).
    pub fn lire_publication(&self, service: &IdService) -> Result<Option<Vec<u8>>, ErreurPortail> {
        self.sortie.lire_publication(service)
    }

    /// Clé publique de publication de la voie (vérification par la façade).
    pub fn cle_publication(&self, service: &IdService) -> Result<[u8; 32], ErreurPortail> {
        self.sortie.cle_publication(service)
    }

    /// Compteurs de la voie (acquittements par compteurs uniquement).
    pub fn compteurs(&self, service: &IdService) -> Result<CompteursVoie, ErreurPortail> {
        let e = self.entree.compteurs(service)?;
        let s = self.sortie.compteurs(service)?;
        Ok(CompteursVoie { publiees: s.publiees, lues: s.lues, ..e })
    }
}

/// CDR partagé par les deux moitiés **du processus unique** (sans état de voie).
struct CdrPartage(Arc<dyn Desarmement>);

impl Desarmement for CdrPartage {
    fn desarmer(&self, objet: &ObjetADesarmer<'_>) -> Result<crate::cdr::ObjetDesarme, crate::cdr::RefusDesarmement> {
        self.0.desarmer(objet)
    }
}
