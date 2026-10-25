//! @id     miyuportail_portail
//! @do     gerer_voies_sens_unique_par_service
//! @role   securite
//! @layer  toolkit
//! @human  miyuportail — Portail interne : voies à sens unique par service, contrôles d'entrée et de sortie, coupure ciblée (EX-44, EX-47, EX-48, EX-49).

//! # Portail interne
//!
//! SERVICES-HYBRIDES-ET-PORTAIL §3. Chaque service déclaré dans un contrat
//! reçoit **sa** voie : files, quotas, compteurs et clé de publication
//! distincts. Une voie a deux sens à sens unique :
//!
//! - **entrante** (quarantaine → zone COG) : la façade **dépose** des trames
//!   MSCE ; la zone COG **relève** à son rythme (« la zone COG tire »,
//!   INV-Z2) ; chaque lot passe par le sas isolé, la deuxième validation, le
//!   CDR, le contrôle anti-rejeu et l'inscription au registre ;
//! - **sortante** (zone COG → publication) : seules les données `session` ou
//!   `publiable`, par **action explicite de l'utilisateur**, au type déclaré
//!   par le traité, réencodées et signées par la clé de la voie.
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
//! `kid` de sa clé. Les Cores ajoutent ou retirent un auteur dans la session
//! ouverte ([`PortailInterne::ajouter_auteur`], [`PortailInterne::retirer_auteur`]),
//! sans rouvrir la session ni reconstruire la façade. Un auteur retiré n'est
//! plus admis (même pour les trames déjà en quarantaine ou en vol) ; sa
//! marque haute est **conservée** : réépinglé dans la même session, il
//! reprend au-delà de sa dernière séquence admise (aucun rejeu possible).

use std::collections::{BTreeMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use ed25519_dalek::{SigningKey, VerifyingKey};
use rand_core::OsRng;
use sha2::{Digest, Sha256};

use crate::cdr::{Desarmement, ObjetADesarmer};
use crate::erreurs::{ErreurPortail, RefusDecodage};
use crate::msce::{
    conforme_au_traite, decoder_interne, kid, sceller, Ancres, Champ, Contexte, Declaration, Kid, Valeur, MAGIC,
    TAILLE_MAX_TRAME,
};
use crate::registre::{Empreinte, Evenement, Inscription, MotifCoupure};
use crate::sas::{DecodeurIsole, RequeteSas, MAX_AUTEURS, MAX_LOT};
use crate::traite::{ClasseExposition, IdService, NatureChamp, Sens, Traite};
use crate::validation2;

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclarationAdmise {
    /// Structure typée remise au moteur (documents désarmés).
    pub declaration: Declaration,
    /// SHA-256 de la trame reçue (inscrite au registre).
    pub empreinte: Empreinte,
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

/// Suivi anti-rejeu d'un auteur dans une session.
struct SuiviAuteur {
    /// Épinglé (admis) ; `false` = retiré, marque haute conservée.
    epingle: bool,
    /// Plus haute séquence admise pour cet auteur.
    derniere_sequence: Option<u64>,
}

struct Session {
    /// Clé de façade et auteurs **épinglés** (remis au sas et à la deuxième validation).
    ancres: Ancres,
    /// Suivi par `kid` d'auteur : épinglés et retirés (≤ [`MAX_AUTEURS_CONNUS`]).
    auteurs: BTreeMap<Kid, SuiviAuteur>,
}

impl Session {
    fn new(emetteur: [u8; 32], auteurs: Vec<[u8; 32]>) -> Self {
        let suivi = auteurs.iter().map(|a| (kid(a), SuiviAuteur { epingle: true, derniere_sequence: None })).collect();
        Self { ancres: Ancres { emetteur, auteurs }, auteurs: suivi }
    }

    /// `kid` de l'unique auteur signataire d'une déclaration admise par les
    /// validations ; il doit être épinglé **maintenant** (retrait en vol = refus).
    fn auteur_epingle(&self, signataires: &[Kid]) -> Result<Kid, ErreurPortail> {
        let kid_emetteur = kid(&self.ancres.emetteur);
        if !signataires.contains(&kid_emetteur) {
            return Err(ErreurPortail::AuteurNonEpingle);
        }
        let mut autres = signataires.iter().filter(|k| **k != kid_emetteur);
        let (Some(auteur), None) = (autres.next(), autres.next()) else {
            return Err(ErreurPortail::AuteurNonEpingle);
        };
        match self.auteurs.get(auteur) {
            Some(s) if s.epingle => Ok(*auteur),
            _ => Err(ErreurPortail::AuteurNonEpingle),
        }
    }
}

struct Fenetre {
    debut: Instant,
    compte: u32,
}

impl Fenetre {
    fn new() -> Self {
        Self { debut: Instant::now(), compte: 0 }
    }

    fn consommer(&mut self, max: u32) -> Result<(), ErreurPortail> {
        if self.debut.elapsed() >= FENETRE_DEBIT {
            *self = Self::new();
        }
        if self.compte >= max {
            return Err(ErreurPortail::DebitDepasse);
        }
        self.compte += 1;
        Ok(())
    }
}

/// Source des identifiants d'instance de voie : jamais réutilisés dans le
/// processus, même après suppression et redéclaration du même service.
static PROCHAINE_INSTANCE: AtomicU64 = AtomicU64::new(0);

/// Identité d'une voie à un instant : instance (déclaration) et génération
/// (session, coupure, désactivation). Toute opération faite hors verrou
/// (sas, CDR) n'est validée que si l'identité n'a pas changé.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Identite {
    instance: u64,
    generation: u64,
}

struct Voie {
    contrat: ContratVoies,
    etat: EtatVoie,
    /// Instance unique de la déclaration (une voie supprimée puis
    /// redéclarée est une **autre** voie : lots et publications en vol refusés).
    instance: u64,
    generation: u64,
    session: Option<Session>,
    quarantaine: VecDeque<Vec<u8>>,
    publication: VecDeque<Vec<u8>>,
    cle_publication: SigningKey,
    sequence_publication: u64,
    debit_entree: Fenetre,
    debit_sortie: Fenetre,
    compteurs: CompteursVoie,
}

impl Voie {
    fn new(contrat: ContratVoies) -> Self {
        Self {
            contrat,
            etat: EtatVoie::Desactivee,
            instance: PROCHAINE_INSTANCE.fetch_add(1, Ordering::Relaxed),
            generation: 0,
            session: None,
            quarantaine: VecDeque::new(),
            publication: VecDeque::new(),
            cle_publication: SigningKey::generate(&mut OsRng),
            sequence_publication: 0,
            debit_entree: Fenetre::new(),
            debit_sortie: Fenetre::new(),
            compteurs: CompteursVoie::default(),
        }
    }

    fn identite(&self) -> Identite {
        Identite { instance: self.instance, generation: self.generation }
    }

    fn exiger(&self, sens: Sens) -> Result<(), ErreurPortail> {
        if !self.contrat.classe.autorise(sens) {
            return Err(ErreurPortail::SensRefuse);
        }
        match self.etat {
            EtatVoie::Active => Ok(()),
            EtatVoie::Desactivee => Err(ErreurPortail::VoieDesactivee),
            EtatVoie::Coupee(_) => Err(ErreurPortail::VoieCoupee),
        }
    }

    fn couper(&mut self, motif: MotifCoupure) {
        self.etat = EtatVoie::Coupee(motif);
        self.generation = self.generation.wrapping_add(1);
        self.session = None;
        self.quarantaine.clear();
        self.publication.clear();
    }
}

type Voies = BTreeMap<IdService, Voie>;

fn verrouiller(voies: &Mutex<Voies>) -> Result<MutexGuard<'_, Voies>, ErreurPortail> {
    voies.lock().map_err(|_| ErreurPortail::EtatIndisponible)
}

fn voie_mut<'a>(voies: &'a mut Voies, service: &IdService) -> Result<&'a mut Voie, ErreurPortail> {
    voies.get_mut(service).ok_or(ErreurPortail::VoieNonDeclaree)
}

fn empreinte(octets: &[u8]) -> Empreinte {
    Sha256::digest(octets).into()
}

fn cle_valide(cle: &[u8; 32]) -> Result<(), ErreurPortail> {
    let vk = VerifyingKey::from_bytes(cle).map_err(|_| ErreurPortail::CleInvalide)?;
    if vk.is_weak() {
        return Err(ErreurPortail::CleInvalide);
    }
    Ok(())
}

/// Coupure d'une voie (EX-49) : interface offerte à WorrySentinel.
pub trait CoupureVoie: Send + Sync {
    /// Coupe la voie du service : session close, files purgées, incident
    /// inscrit. La coupure est effective même si l'inscription échoue
    /// (l'erreur est alors renvoyée).
    fn couper(&self, service: &IdService, motif: MotifCoupure) -> Result<(), ErreurPortail>;
}

/// Poignée de coupure : ne permet **que** de couper (jamais de rétablir).
#[derive(Clone)]
pub struct PoigneeCoupure {
    voies: Arc<Mutex<Voies>>,
    registre: Arc<dyn Inscription>,
}

impl std::fmt::Debug for PoigneeCoupure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PoigneeCoupure").finish_non_exhaustive()
    }
}

fn couper_voie(
    voies: &Mutex<Voies>,
    registre: &dyn Inscription,
    service: &IdService,
    motif: MotifCoupure,
) -> Result<(), ErreurPortail> {
    {
        let mut v = verrouiller(voies)?;
        voie_mut(&mut v, service)?.couper(motif);
    }
    registre
        .inscrire(&Evenement::Coupure { service: service.clone(), motif })
        .map_err(|_| ErreurPortail::Inscription)
}

impl CoupureVoie for PoigneeCoupure {
    fn couper(&self, service: &IdService, motif: MotifCoupure) -> Result<(), ErreurPortail> {
        couper_voie(&self.voies, self.registre.as_ref(), service, motif)
    }
}

/// Portail interne.
pub struct PortailInterne {
    voies: Arc<Mutex<Voies>>,
    sas: Box<dyn DecodeurIsole>,
    cdr: Box<dyn Desarmement>,
    registre: Arc<dyn Inscription>,
}

impl std::fmt::Debug for PortailInterne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PortailInterne").finish_non_exhaustive()
    }
}

impl CoupureVoie for PortailInterne {
    fn couper(&self, service: &IdService, motif: MotifCoupure) -> Result<(), ErreurPortail> {
        couper_voie(&self.voies, self.registre.as_ref(), service, motif)
    }
}

impl PortailInterne {
    /// Portail sans voie. `sas` : en production, [`crate::sas::SasProcessus`] ;
    /// `cdr` : [`crate::cdr::RefusParDefaut`] tant que le CDR n'est pas branché.
    pub fn new(sas: Box<dyn DecodeurIsole>, cdr: Box<dyn Desarmement>, registre: Arc<dyn Inscription>) -> Self {
        Self { voies: Arc::new(Mutex::new(BTreeMap::new())), sas, cdr, registre }
    }

    /// Poignée de coupure pour WorrySentinel.
    pub fn poignee_coupure(&self) -> PoigneeCoupure {
        PoigneeCoupure { voies: Arc::clone(&self.voies), registre: Arc::clone(&self.registre) }
    }

    /// Déclare les voies d'un manifeste : tout ou rien.
    pub fn declarer_manifeste(&self, contrats: &[ContratVoies]) -> Result<(), ErreurPortail> {
        let mut v = verrouiller(&self.voies)?;
        for (i, c) in contrats.iter().enumerate() {
            c.valider()?;
            if v.contains_key(&c.service) || contrats[..i].iter().any(|d| d.service == c.service) {
                return Err(ErreurPortail::VoieDejaDeclaree);
            }
        }
        for c in contrats {
            v.insert(c.service.clone(), Voie::new(c.clone()));
        }
        Ok(())
    }

    /// Déclare la voie d'un service.
    pub fn declarer(&self, contrat: ContratVoies) -> Result<(), ErreurPortail> {
        self.declarer_manifeste(std::slice::from_ref(&contrat))
    }

    /// Supprime la voie (désactivation du service : aucun résidu).
    pub fn supprimer(&self, service: &IdService) -> Result<(), ErreurPortail> {
        verrouiller(&self.voies)?.remove(service).map(|_| ()).ok_or(ErreurPortail::VoieNonDeclaree)
    }

    /// Active l'exposition (action explicite de l'utilisateur, EX-48).
    pub fn activer(&self, service: &IdService) -> Result<(), ErreurPortail> {
        let mut v = verrouiller(&self.voies)?;
        let voie = voie_mut(&mut v, service)?;
        match voie.etat {
            EtatVoie::Desactivee => {
                voie.etat = EtatVoie::Active;
                Ok(())
            }
            EtatVoie::Active => Ok(()),
            EtatVoie::Coupee(_) => Err(ErreurPortail::TransitionRefusee),
        }
    }

    /// Désactive l'exposition : session close, files purgées.
    pub fn desactiver(&self, service: &IdService) -> Result<(), ErreurPortail> {
        let mut v = verrouiller(&self.voies)?;
        let voie = voie_mut(&mut v, service)?;
        if matches!(voie.etat, EtatVoie::Coupee(_)) {
            return Err(ErreurPortail::TransitionRefusee);
        }
        voie.etat = EtatVoie::Desactivee;
        voie.generation = voie.generation.wrapping_add(1);
        voie.session = None;
        voie.quarantaine.clear();
        voie.publication.clear();
        Ok(())
    }

    /// Rétablit une voie coupée (décision explicite, hors WorrySentinel) :
    /// elle repasse **désactivée**, avec une **nouvelle** clé de publication.
    pub fn retablir(&self, service: &IdService) -> Result<(), ErreurPortail> {
        let mut v = verrouiller(&self.voies)?;
        let voie = voie_mut(&mut v, service)?;
        if !matches!(voie.etat, EtatVoie::Coupee(_)) {
            return Err(ErreurPortail::TransitionRefusee);
        }
        voie.etat = EtatVoie::Desactivee;
        voie.cle_publication = SigningKey::generate(&mut OsRng);
        voie.sequence_publication = 0;
        Ok(())
    }

    /// État d'une voie.
    pub fn etat(&self, service: &IdService) -> Result<EtatVoie, ErreurPortail> {
        Ok(verrouiller(&self.voies)?.get(service).ok_or(ErreurPortail::VoieNonDeclaree)?.etat)
    }

    /// Ouvre une session : épingle la clé de session de la façade (émise par
    /// la zone COG, INV-H6) et les clés des auteurs autorisés.
    pub fn ouvrir_session(
        &self,
        service: &IdService,
        cle_facade: [u8; 32],
        auteurs: Vec<[u8; 32]>,
    ) -> Result<(), ErreurPortail> {
        if auteurs.is_empty() || auteurs.len() > MAX_AUTEURS {
            return Err(ErreurPortail::CleInvalide);
        }
        cle_valide(&cle_facade)?;
        for (i, a) in auteurs.iter().enumerate() {
            cle_valide(a)?;
            if *a == cle_facade || auteurs[..i].contains(a) {
                return Err(ErreurPortail::CleInvalide);
            }
        }
        let mut v = verrouiller(&self.voies)?;
        let voie = voie_mut(&mut v, service)?;
        voie.exiger(Sens::Entrante)?;
        voie.generation = voie.generation.wrapping_add(1);
        voie.quarantaine.clear();
        voie.session = Some(Session::new(cle_facade, auteurs));
        Ok(())
    }

    /// Épingle un auteur (appareil) de plus dans la session ouverte, sans la
    /// rouvrir (A-007). Appel réservé aux Cores, comme [`Self::ouvrir_session`].
    ///
    /// Auteur nouveau pour la session : séquence neuve. Auteur retiré puis
    /// réépinglé : sa marque haute est **conservée** (aucun rejeu possible).
    /// Refus : clé invalide ou égale à la clé de façade, auteur déjà épinglé,
    /// plus de [`MAX_AUTEURS`] épinglés ou de [`MAX_AUTEURS_CONNUS`] connus.
    pub fn ajouter_auteur(&self, service: &IdService, auteur: [u8; 32]) -> Result<(), ErreurPortail> {
        cle_valide(&auteur)?;
        let mut v = verrouiller(&self.voies)?;
        let voie = voie_mut(&mut v, service)?;
        voie.exiger(Sens::Entrante)?;
        let session = voie.session.as_mut().ok_or(ErreurPortail::SessionAbsente)?;
        if auteur == session.ancres.emetteur {
            return Err(ErreurPortail::CleInvalide);
        }
        let k = kid(&auteur);
        match session.auteurs.get(&k) {
            Some(s) if s.epingle => return Err(ErreurPortail::AuteurDejaEpingle),
            None if session.auteurs.len() >= MAX_AUTEURS_CONNUS => return Err(ErreurPortail::TropDAuteurs),
            _ => {}
        }
        if session.ancres.auteurs.len() >= MAX_AUTEURS {
            return Err(ErreurPortail::TropDAuteurs);
        }
        session.ancres.auteurs.push(auteur);
        session
            .auteurs
            .entry(k)
            .and_modify(|s| s.epingle = true)
            .or_insert(SuiviAuteur { epingle: true, derniere_sequence: None });
        Ok(())
    }

    /// Retire un auteur de la session ouverte (A-007) : ses trames sont
    /// refusées dès maintenant, y compris celles déjà en quarantaine ou en
    /// cours de décodage ; les autres auteurs continuent. Sa marque haute est
    /// conservée. Le dernier auteur ne se retire pas (fermer la session).
    pub fn retirer_auteur(&self, service: &IdService, auteur: &[u8; 32]) -> Result<(), ErreurPortail> {
        let mut v = verrouiller(&self.voies)?;
        let voie = voie_mut(&mut v, service)?;
        let session = voie.session.as_mut().ok_or(ErreurPortail::SessionAbsente)?;
        let suivi = session.auteurs.get_mut(&kid(auteur)).filter(|s| s.epingle).ok_or(ErreurPortail::AuteurNonEpingle)?;
        if session.ancres.auteurs.len() <= 1 {
            return Err(ErreurPortail::TransitionRefusee);
        }
        suivi.epingle = false;
        session.ancres.auteurs.retain(|a| a != auteur);
        Ok(())
    }

    /// Clés des auteurs épinglés de la session ouverte (côté Cores).
    pub fn auteurs(&self, service: &IdService) -> Result<Vec<[u8; 32]>, ErreurPortail> {
        let v = verrouiller(&self.voies)?;
        let voie = v.get(service).ok_or(ErreurPortail::VoieNonDeclaree)?;
        Ok(voie.session.as_ref().ok_or(ErreurPortail::SessionAbsente)?.ancres.auteurs.clone())
    }

    /// Ferme la session : quarantaine purgée.
    pub fn fermer_session(&self, service: &IdService) -> Result<(), ErreurPortail> {
        let mut v = verrouiller(&self.voies)?;
        let voie = voie_mut(&mut v, service)?;
        voie.generation = voie.generation.wrapping_add(1);
        voie.session = None;
        voie.quarantaine.clear();
        Ok(())
    }

    /// **Côté Franche** : la façade dépose une trame en quarantaine.
    /// Contrôles : voie déclarée, sens ouvert, voie active, session, taille,
    /// liste blanche `MSCE`, débit, quota.
    pub fn deposer(&self, service: &IdService, trame: Vec<u8>) -> Result<(), ErreurPortail> {
        let mut v = verrouiller(&self.voies)?;
        let voie = voie_mut(&mut v, service)?;
        voie.exiger(Sens::Entrante)?;
        if voie.session.is_none() {
            return Err(ErreurPortail::SessionAbsente);
        }
        if trame.len() > voie.contrat.taille_max_trame {
            return Err(ErreurPortail::TropGrand);
        }
        if !trame.starts_with(MAGIC) {
            return Err(ErreurPortail::ProtocoleRefuse);
        }
        if voie.quarantaine.len() >= voie.contrat.quota_file {
            return Err(ErreurPortail::FilePleine);
        }
        voie.debit_entree.consommer(voie.contrat.debit_max_par_minute)?;
        voie.quarantaine.push_back(trame);
        voie.compteurs.deposees += 1;
        Ok(())
    }

    /// **Côté zone COG** : relève jusqu'à `max` trames (au plus [`MAX_LOT`]).
    ///
    /// Erreur globale si la voie n'est pas relevable ; sinon un résultat par
    /// trame relevée, dans l'ordre de dépôt.
    pub fn relever(
        &self,
        service: &IdService,
        max: usize,
    ) -> Result<Vec<Result<DeclarationAdmise, ErreurPortail>>, ErreurPortail> {
        // 1. Prise du lot (verrou court).
        let (requete, identite) = {
            let mut v = verrouiller(&self.voies)?;
            let voie = voie_mut(&mut v, service)?;
            voie.exiger(Sens::Entrante)?;
            let ancres = voie.session.as_ref().ok_or(ErreurPortail::SessionAbsente)?.ancres.clone();
            let n = max.min(MAX_LOT).min(voie.quarantaine.len());
            if n == 0 {
                return Ok(Vec::new());
            }
            let trames: Vec<Vec<u8>> = voie.quarantaine.drain(..n).collect();
            voie.compteurs.relevees += n as u64;
            let requete = RequeteSas {
                service: service.clone(),
                ancres,
                traite: voie.contrat.traite.restreindre(Sens::Entrante),
                trames,
            };
            (requete, voie.identite())
        };

        // 2. Sas isolé, deuxième validation, CDR (hors verrou : les autres voies continuent).
        let candidats: Vec<Result<DeclarationAdmise, ErreurPortail>> = match self.sas.decoder_lot(&requete) {
            Err(e) => requete.trames.iter().map(|_| Err(ErreurPortail::Sas(e.clone()))).collect(),
            Ok(resultats) if resultats.len() == requete.trames.len() => requete
                .trames
                .iter()
                .zip(resultats)
                .map(|(trame, resultat)| self.controler_entree(&requete, trame, resultat))
                .collect(),
            Ok(_) => requete.trames.iter().map(|_| Err(ErreurPortail::Sas(crate::erreurs::ErreurSas::Reponse))).collect(),
        };

        // 3. Anti-rejeu et inscription (verrou) ; voie coupée entre-temps = tout refusé.
        let mut v = verrouiller(&self.voies)?;
        let voie = voie_mut(&mut v, service)?;
        let toujours_valide = voie.etat == EtatVoie::Active && voie.identite() == identite;
        let mut sortie = Vec::with_capacity(candidats.len());
        for candidat in candidats {
            let resultat = if toujours_valide {
                candidat.and_then(|admise| self.admettre(voie, admise))
            } else {
                Err(ErreurPortail::VoieCoupee)
            };
            if resultat.is_ok() {
                voie.compteurs.admises += 1;
            } else {
                voie.compteurs.refusees += 1;
            }
            sortie.push(resultat);
        }
        Ok(sortie)
    }

    fn controler_entree(
        &self,
        requete: &RequeteSas,
        trame: &[u8],
        resultat: Result<Vec<u8>, RefusDecodage>,
    ) -> Result<DeclarationAdmise, ErreurPortail> {
        let interne = resultat?;
        let selon_sas = decoder_interne(&interne).map_err(|_| ErreurPortail::ValidationDivergente)?;
        let ctx = Contexte {
            service: &requete.service,
            sens: Sens::Entrante,
            traite: &requete.traite,
            ancres: &requete.ancres,
        };
        let selon_zone_cog = validation2::valider(trame, &ctx).map_err(|_| ErreurPortail::ValidationDivergente)?;
        if selon_sas != selon_zone_cog {
            return Err(ErreurPortail::ValidationDivergente);
        }
        let declaration = self.desarmer(selon_zone_cog, &requete.traite)?;
        Ok(DeclarationAdmise { declaration, empreinte: empreinte(trame) })
    }

    /// Désarme chaque champ `Document` (EX-45) ; tout refus refuse le message.
    fn desarmer(&self, mut d: Declaration, traite: &Traite) -> Result<Declaration, ErreurPortail> {
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
                let reconstruit = self.cdr.desarmer(&objet).map_err(|_| ErreurPortail::Desarmement)?;
                if reconstruit.octets.len() > max as usize {
                    return Err(ErreurPortail::Desarmement);
                }
                champ.valeur = Valeur::Document(reconstruit.octets);
            }
        }
        Ok(d)
    }

    fn admettre(&self, voie: &mut Voie, admise: DeclarationAdmise) -> Result<DeclarationAdmise, ErreurPortail> {
        let session = voie.session.as_mut().ok_or(ErreurPortail::SessionAbsente)?;
        let auteur = session.auteur_epingle(&admise.declaration.signataires)?;
        let suivi = session.auteurs.get_mut(&auteur).ok_or(ErreurPortail::AuteurNonEpingle)?;
        if suivi.derniere_sequence.is_some_and(|s| admise.declaration.sequence <= s) {
            return Err(ErreurPortail::Rejeu);
        }
        self.registre
            .inscrire(&Evenement::Entree { service: admise.declaration.service.clone(), empreinte: admise.empreinte })
            .map_err(|_| ErreurPortail::Inscription)?;
        suivi.derniere_sequence = Some(admise.declaration.sequence);
        Ok(admise)
    }

    /// **Côté zone COG** : publie vers la Franche. Renvoie le numéro de publication.
    pub fn publier(&self, service: &IdService, publication: Publication) -> Result<u64, ErreurPortail> {
        if publication.etiquette == Sensibilite::Nationale {
            return Err(ErreurPortail::SortieNationale);
        }
        if publication.declencheur != Declencheur::Utilisateur {
            return Err(ErreurPortail::ActionUtilisateurRequise);
        }
        // 1. Contrôles de voie et instantané du traité.
        let (traite, identite) = {
            let mut v = verrouiller(&self.voies)?;
            let voie = voie_mut(&mut v, service)?;
            voie.exiger(Sens::Sortante)?;
            (voie.contrat.traite.clone(), voie.identite())
        };
        // 2. Conformité au traité et CDR (hors verrou).
        let brouillon = Declaration {
            service: service.clone(),
            sens: Sens::Sortante,
            version_traite: traite.version,
            type_code: publication.type_code,
            sequence: 0,
            emise_le: 0,
            champs: publication.champs,
            signataires: Vec::new(),
        };
        conforme_au_traite(&brouillon, &traite)?;
        let mut declaration = self.desarmer(brouillon, &traite)?;

        // 3. Réencodage signé, quota, débit, inscription (verrou).
        let mut v = verrouiller(&self.voies)?;
        let voie = voie_mut(&mut v, service)?;
        voie.exiger(Sens::Sortante)?;
        if voie.identite() != identite {
            return Err(ErreurPortail::VoieCoupee);
        }
        if voie.publication.len() >= voie.contrat.quota_file {
            return Err(ErreurPortail::FilePleine);
        }
        declaration.sequence = voie.sequence_publication.checked_add(1).ok_or(ErreurPortail::Rejeu)?;
        declaration.emise_le = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs());
        let trame = sceller(&declaration, &[&voie.cle_publication])?;
        if trame.len() > voie.contrat.taille_max_trame {
            return Err(ErreurPortail::TropGrand);
        }
        voie.debit_sortie.consommer(voie.contrat.debit_max_par_minute)?;
        self.registre
            .inscrire(&Evenement::Publication { service: service.clone(), empreinte: empreinte(&trame) })
            .map_err(|_| ErreurPortail::Inscription)?;
        voie.sequence_publication = declaration.sequence;
        voie.publication.push_back(trame);
        voie.compteurs.publiees += 1;
        Ok(declaration.sequence)
    }

    /// **Côté Franche** : la façade lit la prochaine publication (trame MSCE neuve).
    pub fn lire_publication(&self, service: &IdService) -> Result<Option<Vec<u8>>, ErreurPortail> {
        let mut v = verrouiller(&self.voies)?;
        let voie = voie_mut(&mut v, service)?;
        voie.exiger(Sens::Sortante)?;
        let trame = voie.publication.pop_front();
        if trame.is_some() {
            voie.compteurs.lues += 1;
        }
        Ok(trame)
    }

    /// Clé publique de publication de la voie (vérification par la façade).
    pub fn cle_publication(&self, service: &IdService) -> Result<[u8; 32], ErreurPortail> {
        let v = verrouiller(&self.voies)?;
        Ok(v.get(service).ok_or(ErreurPortail::VoieNonDeclaree)?.cle_publication.verifying_key().to_bytes())
    }

    /// Compteurs de la voie (acquittements par compteurs uniquement).
    pub fn compteurs(&self, service: &IdService) -> Result<CompteursVoie, ErreurPortail> {
        let v = verrouiller(&self.voies)?;
        Ok(v.get(service).ok_or(ErreurPortail::VoieNonDeclaree)?.compteurs)
    }
}

#[cfg(test)]
mod tests {
    //! Anti-rejeu par auteur (A-007) : plusieurs appareils dans une session.

    use super::*;
    use crate::cdr::RefusParDefaut;
    use crate::erreurs::ErreurSas;
    use crate::registre::RegistreMemoire;
    use crate::sas::{decoder_reponse, traiter_requete, ResultatTrame};
    use crate::traite::{ChampTraite, TypeDeclaration};

    const ENTRANT: u64 = 10;

    struct SasEnProcessus;

    impl DecodeurIsole for SasEnProcessus {
        fn decoder_lot(&self, requete: &RequeteSas) -> Result<Vec<ResultatTrame>, ErreurSas> {
            let r = traiter_requete(&requete.encoder().map_err(|_| ErreurSas::Reponse)?)
                .map_err(|_| ErreurSas::FinAnormale)?;
            decoder_reponse(&r, requete.trames.len())
        }
    }

    fn cle(graine: u8) -> SigningKey {
        SigningKey::from_bytes(&[graine; 32])
    }

    fn pk(c: &SigningKey) -> [u8; 32] {
        c.verifying_key().to_bytes()
    }

    struct Monde {
        portail: PortailInterne,
        service: IdService,
        facade: SigningKey,
    }

    fn monde(auteurs: &[&SigningKey]) -> Monde {
        let service = IdService::new("docs").unwrap();
        let traite = Traite {
            version: 1,
            types: vec![TypeDeclaration {
                code: ENTRANT,
                sens: Sens::Entrante,
                champs: vec![ChampTraite { tag: 1, nature: NatureChamp::Entier, obligatoire: true }],
            }],
        };
        let contrat = ContratVoies {
            service: service.clone(),
            classe: ClasseExposition::H2,
            traite,
            taille_max_trame: 8192,
            debit_max_par_minute: 1000,
            quota_file: 64,
        };
        let portail =
            PortailInterne::new(Box::new(SasEnProcessus), Box::new(RefusParDefaut), Arc::new(RegistreMemoire::new()));
        portail.declarer(contrat).unwrap();
        portail.activer(&service).unwrap();
        let facade = cle(7);
        portail.ouvrir_session(&service, pk(&facade), auteurs.iter().map(|a| pk(a)).collect()).unwrap();
        Monde { portail, service, facade }
    }

    impl Monde {
        fn trame(&self, auteur: &SigningKey, sequence: u64) -> Vec<u8> {
            let d = Declaration {
                service: self.service.clone(),
                sens: Sens::Entrante,
                version_traite: 1,
                type_code: ENTRANT,
                sequence,
                emise_le: 1_790_000_000,
                champs: vec![Champ { tag: 1, valeur: Valeur::Entier(sequence) }],
                signataires: Vec::new(),
            };
            sceller(&d, &[&self.facade, auteur]).unwrap()
        }

        /// Dépose puis relève : un résultat par trame.
        fn passer(&self, trames: Vec<Vec<u8>>) -> Vec<Result<u64, ErreurPortail>> {
            for t in trames {
                self.portail.deposer(&self.service, t).unwrap();
            }
            self.portail
                .relever(&self.service, MAX_LOT)
                .unwrap()
                .into_iter()
                .map(|r| r.map(|a| a.declaration.sequence))
                .collect()
        }
    }

    #[test]
    fn sequences_independantes_par_auteur() {
        let (a, b) = (cle(9), cle(11));
        let m = monde(&[&a, &b]);
        // Entrelacées : chaque appareil a sa propre séquence.
        assert_eq!(
            m.passer(vec![m.trame(&a, 1), m.trame(&b, 1), m.trame(&b, 2), m.trame(&a, 2), m.trame(&b, 3)]),
            vec![Ok(1), Ok(1), Ok(2), Ok(2), Ok(3)]
        );
        // Rejeu de A refusé même après l'avance de B ; B n'est pas freiné par A.
        let t_a2 = m.trame(&a, 2);
        assert_eq!(
            m.passer(vec![t_a2.clone(), m.trame(&a, 1), m.trame(&b, 4), m.trame(&a, 3), t_a2]),
            vec![Err(ErreurPortail::Rejeu), Err(ErreurPortail::Rejeu), Ok(4), Ok(3), Err(ErreurPortail::Rejeu)]
        );
    }

    #[test]
    fn auteur_non_epingle_refuse() {
        let (a, intrus) = (cle(9), cle(13));
        let m = monde(&[&a]);
        assert_eq!(
            m.passer(vec![m.trame(&intrus, 1)]),
            vec![Err(ErreurPortail::Decodage(RefusDecodage::Signataires))]
        );
        assert_eq!(m.portail.retirer_auteur(&m.service, &pk(&intrus)), Err(ErreurPortail::AuteurNonEpingle));
    }

    #[test]
    fn ajout_et_retrait_dans_la_session_ouverte() {
        let (a, b) = (cle(9), cle(11));
        let m = monde(&[&a]);
        assert_eq!(m.passer(vec![m.trame(&a, 5)]), vec![Ok(5)]);

        // B épinglé sans rouvrir la session : séquence neuve.
        m.portail.ajouter_auteur(&m.service, pk(&b)).unwrap();
        assert_eq!(m.portail.ajouter_auteur(&m.service, pk(&b)), Err(ErreurPortail::AuteurDejaEpingle));
        assert_eq!(m.portail.ajouter_auteur(&m.service, pk(&m.facade)), Err(ErreurPortail::CleInvalide));
        assert_eq!(m.passer(vec![m.trame(&b, 1)]), vec![Ok(1)]);

        // Trame de A en quarantaine, puis A retiré : refusée ; B continue.
        m.portail.deposer(&m.service, m.trame(&a, 6)).unwrap();
        m.portail.retirer_auteur(&m.service, &pk(&a)).unwrap();
        assert_eq!(
            m.passer(vec![m.trame(&a, 7), m.trame(&b, 2)]),
            vec![
                Err(ErreurPortail::Decodage(RefusDecodage::Signataires)),
                Err(ErreurPortail::Decodage(RefusDecodage::Signataires)),
                Ok(2)
            ]
        );
        assert_eq!(m.portail.auteurs(&m.service).unwrap(), vec![pk(&b)]);

        // Le dernier auteur ne se retire pas (la session se ferme).
        assert_eq!(m.portail.retirer_auteur(&m.service, &pk(&b)), Err(ErreurPortail::TransitionRefusee));

        // A réépinglé : sa marque haute (5) est conservée.
        m.portail.ajouter_auteur(&m.service, pk(&a)).unwrap();
        assert_eq!(
            m.passer(vec![m.trame(&a, 5), m.trame(&a, 1), m.trame(&a, 6)]),
            vec![Err(ErreurPortail::Rejeu), Err(ErreurPortail::Rejeu), Ok(6)]
        );
    }

    #[test]
    fn retrait_pendant_le_decodage_refuse_l_auteur_retire() {
        /// Sas qui retire A pendant le décodage du lot (retrait « en vol »).
        type Retrait = Option<(Arc<Mutex<Voies>>, IdService, [u8; 32])>;
        struct SasRetirant(Arc<Mutex<Retrait>>);
        impl DecodeurIsole for SasRetirant {
            fn decoder_lot(&self, requete: &RequeteSas) -> Result<Vec<ResultatTrame>, ErreurSas> {
                if let Some((voies, service, auteur)) = self.0.lock().unwrap().take() {
                    let mut v = voies.lock().unwrap();
                    let session = v.get_mut(&service).unwrap().session.as_mut().unwrap();
                    session.auteurs.get_mut(&kid(&auteur)).unwrap().epingle = false;
                    session.ancres.auteurs.retain(|a| *a != auteur);
                }
                SasEnProcessus.decoder_lot(requete)
            }
        }
        let (a, b) = (cle(9), cle(11));
        let mut m = monde(&[&a, &b]);
        let declencheur = Arc::new(Mutex::new(None));
        m.portail.sas = Box::new(SasRetirant(Arc::clone(&declencheur)));
        *declencheur.lock().unwrap() = Some((Arc::clone(&m.portail.voies), m.service.clone(), pk(&a)));
        assert_eq!(
            m.passer(vec![m.trame(&a, 1), m.trame(&b, 1)]),
            vec![Err(ErreurPortail::AuteurNonEpingle), Ok(1)]
        );
    }

    #[test]
    fn bornes_des_auteurs() {
        let premiers: Vec<SigningKey> = (0..MAX_AUTEURS as u8).map(|i| cle(100 + i)).collect();
        let refs: Vec<&SigningKey> = premiers.iter().collect();
        let m = monde(&refs);
        assert_eq!(m.portail.ajouter_auteur(&m.service, pk(&cle(50))), Err(ErreurPortail::TropDAuteurs));
        // Doublon refusé à l'ouverture.
        let a = cle(9);
        assert_eq!(
            m.portail.ouvrir_session(&m.service, pk(&m.facade), vec![pk(&a), pk(&a)]),
            Err(ErreurPortail::CleInvalide)
        );
        // Connus (épinglés + retirés) bornés : rotation jusqu'à la limite.
        m.portail.ouvrir_session(&m.service, pk(&m.facade), vec![pk(&a)]).unwrap();
        let mut refus = None;
        for i in 0..=MAX_AUTEURS_CONNUS as u16 {
            let mut graine = [0xA5; 32];
            graine[..2].copy_from_slice(&i.to_be_bytes());
            let c = SigningKey::from_bytes(&graine);
            match m.portail.ajouter_auteur(&m.service, pk(&c)) {
                Ok(()) => m.portail.retirer_auteur(&m.service, &pk(&c)).unwrap(),
                Err(e) => {
                    refus = Some(e);
                    break;
                }
            }
        }
        assert_eq!(refus, Some(ErreurPortail::TropDAuteurs));
    }

    #[test]
    fn coupure_ferme_tout_et_session_neuve_repart_de_zero() {
        let (a, b) = (cle(9), cle(11));
        let m = monde(&[&a, &b]);
        assert_eq!(m.passer(vec![m.trame(&a, 3), m.trame(&b, 3)]), vec![Ok(3), Ok(3)]);
        m.portail.couper(&m.service, MotifCoupure::Anomalie).unwrap();
        assert_eq!(m.portail.deposer(&m.service, m.trame(&a, 4)), Err(ErreurPortail::VoieCoupee));
        assert_eq!(m.portail.deposer(&m.service, m.trame(&b, 4)), Err(ErreurPortail::VoieCoupee));
        assert_eq!(m.portail.ajouter_auteur(&m.service, pk(&cle(13))), Err(ErreurPortail::VoieCoupee));
        assert_eq!(m.portail.retirer_auteur(&m.service, &pk(&a)), Err(ErreurPortail::SessionAbsente));
        m.portail.retablir(&m.service).unwrap();
        m.portail.activer(&m.service).unwrap();
        m.portail.ouvrir_session(&m.service, pk(&m.facade), vec![pk(&a)]).unwrap();
        assert_eq!(m.passer(vec![m.trame(&a, 1)]), vec![Ok(1)]);
    }
}
