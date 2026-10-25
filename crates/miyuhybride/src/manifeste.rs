//! @id     miyuhybride_manifeste
//! @do     sceller_verifier_manifeste_service
//! @role   securite
//! @layer  toolkit
//! @human  miyuhybride — manifeste de service signé : classe H0–H3, contrat de ressources et de flux, traité, façade (EX-16, EX-40, EX-48).

//! # Manifeste de service signé
//!
//! MODELE-ZONES §10 : « chaque service déclare ses besoins maximaux dans son
//! manifeste signé (Sceau v1) » ; SERVICES-HYBRIDES §1 : « la classe est
//! déclarée dans le contrat (manifeste signé) ». La doc gelée liste le
//! contenu du contrat mais n'en fixe pas l'encodage : ce module le fixe
//! (point à inscrire par amendement).
//!
//! ## Enveloppe
//!
//! Titre **Sceau v1.1** de type `ManifestePaquet` (`0x03`), signé par
//! l'éditeur (clé épinglée). La charge **étend** celle du Market
//! (`miyumarket::signature`) : mêmes tags 1 à 4, plus le tag 5 « contrat »,
//! que le Market transporte sans l'interpréter. Un même sceau est donc
//! vérifiable par l'installeur du Market et décodé ici.
//!
//! | Tag | Champ | Forme |
//! |---|---|---|
//! | 1 | service | identifiant du Portail `[a-z0-9._:-]`, 1..64 |
//! | 2 | version | texte ASCII imprimable 1..64 |
//! | 3 | SHA-256 de l'archive | 32 o |
//! | 4 | nom du binaire | facultatif, texte 1..255 |
//! | 5 | contrat | TLV imbriqué ci-dessous, **obligatoire** ici |
//!
//! ## Contrat (TLV imbriqué, tags strictement croissants)
//!
//! | Tag | Champ | Forme |
//! |---|---|---|
//! | 1 | genre | texte = [`GENRE_CONTRAT`] (séparation de domaine) |
//! | 2 | classe | 1 o : 0 H0, 1 H1, 2 H2, 3 H3 |
//! | 3 | exposition par défaut | 1 o, **doit valoir 0** (désactivée, EX-48) |
//! | 4 | ressources | TLV : 1 CPU % (u64), 2 mémoire Mio, 3 disque Mio, 4 tâches, 5 sessions |
//! | 5 | voie | TLV : 1 taille max de trame, 2 débit max / minute / sens, 3 quota de file — absent en H0 |
//! | 6 | traité | encodage canonique `miyuportail::Traite::encoder` — absent en H0 |
//! | 7 | façade | TLV : 1 SHA-256 de l'image signée, 2 DB secondaire max (Mio), 3 durée max de session (s) — absent en H0 |
//! | 8 | entrées externes | `n (1 o) × [origine 1 o | protocole 1 o | port u16 | débit Kio/s u32 | taille max paquet u16]`, strictement croissantes — absent en H0 |
//!
//! ## Règles (fail-closed)
//!
//! - H0 : ni voie, ni traité, ni façade, ni entrée (jamais de façade, EX-40) ;
//! - H1 : traité **sortant** seulement ; H2 : au moins un type entrant ;
//! - **H3 refusé** : la liste blanche sortante et l'intention StrongFather
//!   n'ont pas d'encodage dans le contrat (non spécifié) ;
//! - entrées : origine **Fédération** seule ; Origin et International
//!   exigent une liste blanche de pairs (INV-Z6) que le format n'exprime pas
//!   encore : refusées ;
//! - exposition par défaut ≠ 0 : refus (EX-48) ; ce qui n'est pas déclaré
//!   n'existe pas.

use miyuportail::{ClasseExposition, ContratVoies, IdService, Sens, Traite};
use miyusceau::{
    AncresDeConfiance, CleSignature, EcrivainTlv, ErreurSceau, Kid, LecteurTlv, Titre, TypeSceau, Verifie,
};
use sha2::{Digest, Sha256};

use crate::erreurs::ErreurManifeste;

/// Genre du contrat (séparation de domaine avec les autres titres `0x03`).
pub const GENRE_CONTRAT: &str = "miyukini/contrat-service/v1";
/// Longueur maximale de la version.
pub const MAX_VERSION: usize = 64;
/// Longueur maximale du nom de binaire (celle du Market).
pub const MAX_NOM_BINAIRE: usize = 255;
/// Nombre maximal d'entrées externes.
pub const MAX_ENTREES: usize = 16;
/// Plafond CPU déclarable (% d'un cœur).
pub const PLAFOND_CPU_POURCENT: u32 = 1600;
/// Plafond mémoire déclarable (Mio).
pub const PLAFOND_MEMOIRE_MIO: u32 = 65_536;
/// Plafond disque déclarable (Mio).
pub const PLAFOND_DISQUE_MIO: u32 = 1_048_576;
/// Plafond de tâches déclarables.
pub const PLAFOND_TACHES: u32 = 4096;
/// Plafond de sessions simultanées déclarables.
pub const PLAFOND_SESSIONS: u32 = 1024;
/// Durée maximale d'une session de façade (INV-H5 : « à chaque session, ou chaque jour »).
pub const DUREE_MAX_SESSION_SECS: u64 = 86_400;
/// Taille minimale d'un paquet IP (celle des arrêtés BorderGuard).
pub const TAILLE_MIN_PAQUET: u16 = 80;

const TAG_SERVICE: u8 = 1;
const TAG_VERSION: u8 = 2;
const TAG_ARCHIVE: u8 = 3;
const TAG_BINAIRE: u8 = 4;
const TAG_CONTRAT: u8 = 5;

const C_GENRE: u8 = 1;
const C_CLASSE: u8 = 2;
const C_EXPOSITION: u8 = 3;
const C_RESSOURCES: u8 = 4;
const C_VOIE: u8 = 5;
const C_TRAITE: u8 = 6;
const C_FACADE: u8 = 7;
const C_ENTREES: u8 = 8;

const TAILLE_ENTREE: usize = 1 + 1 + 2 + 4 + 2;

/// Plafonds de ressources déclarés (MODELE-ZONES §10 « Ressources max en Franche »).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ressources {
    /// CPU, en pourcentage d'un cœur.
    pub cpu_pourcent: u32,
    /// Mémoire maximale (Mio).
    pub memoire_mio: u32,
    /// Disque maximal (Mio), DB secondaire comprise.
    pub disque_mio: u32,
    /// Nombre maximal de tâches.
    pub taches_max: u32,
    /// Nombre maximal de sessions simultanées.
    pub sessions_max: u32,
}

/// Contrat de la voie du Portail (MODELE-ZONES §10 « Flux » côté Portail).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContratVoie {
    /// Taille maximale d'une trame MSCE.
    pub taille_max_trame: u32,
    /// Débit maximal par sens (trames par minute).
    pub debit_max_par_minute: u32,
    /// Profondeur maximale de chaque file.
    pub quota_file: u32,
}

/// Contrat de la façade (MODELE-ZONES §10 « Relais »).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContratFacade {
    /// SHA-256 de l'image signée de la façade (INV-H5, EX-43).
    pub image: [u8; 32],
    /// Taille maximale de la DB secondaire (Mio, ≥ 1).
    pub db_secondaire_max_mio: u32,
    /// Durée maximale d'une session de façade (secondes).
    pub duree_max_session_secs: u64,
}

/// Origine extérieure d'une entrée (zone qui ouvre la connexion vers la Franche).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum OrigineExterne {
    /// Fédération (COG reconnus, appareils fédérés) — code de zone BorderGuard 4.
    Federation = 4,
}

/// Transport d'une entrée.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Transport {
    /// TCP (IANA 6).
    Tcp = 6,
    /// UDP (IANA 17).
    Udp = 17,
}

/// Point d'entrée externe de la façade (MODELE-ZONES §10 « Flux : sens, protocole, port, débit, taille »).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PointEntree {
    /// Zone extérieure autorisée.
    pub origine: OrigineExterne,
    /// Transport.
    pub transport: Transport,
    /// Port de destination (≠ 0).
    pub port: u16,
    /// Débit maximal (Kio/s, ≥ 1).
    pub debit_max_kio_s: u32,
    /// Taille maximale d'un paquet IP (≥ [`TAILLE_MIN_PAQUET`]).
    pub taille_max_paquet: u16,
}

impl PointEntree {
    fn encoder(&self) -> [u8; TAILLE_ENTREE] {
        let mut o = [0u8; TAILLE_ENTREE];
        o[0] = self.origine as u8;
        o[1] = self.transport as u8;
        o[2..4].copy_from_slice(&self.port.to_be_bytes());
        o[4..8].copy_from_slice(&self.debit_max_kio_s.to_be_bytes());
        o[8..10].copy_from_slice(&self.taille_max_paquet.to_be_bytes());
        o
    }

    fn decoder(o: &[u8; TAILLE_ENTREE]) -> Result<Self, ErreurManifeste> {
        let origine = match o[0] {
            4 => OrigineExterne::Federation,
            5 | 6 => {
                return Err(ErreurManifeste::NonSpecifie("entrée Origin / International : liste blanche de pairs"))
            }
            _ => return Err(ErreurManifeste::Contrat("origine d'entrée inconnue")),
        };
        let transport = match o[1] {
            6 => Transport::Tcp,
            17 => Transport::Udp,
            _ => return Err(ErreurManifeste::Contrat("transport inconnu")),
        };
        Ok(Self {
            origine,
            transport,
            port: u16::from_be_bytes([o[2], o[3]]),
            debit_max_kio_s: u32::from_be_bytes([o[4], o[5], o[6], o[7]]),
            taille_max_paquet: u16::from_be_bytes([o[8], o[9]]),
        })
    }
}

/// Contrat de service (tag 5 du manifeste).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContratService {
    /// Classe d'exposition (EX-40).
    pub classe: ClasseExposition,
    /// Plafonds de ressources.
    pub ressources: Ressources,
    /// Contrat de voie (absent en H0).
    pub voie: Option<ContratVoie>,
    /// Traité versionné (absent en H0).
    pub traite: Option<Traite>,
    /// Façade (absente en H0).
    pub facade: Option<ContratFacade>,
    /// Entrées externes de la façade (vide en H0).
    pub entrees: Vec<PointEntree>,
}

/// Manifeste de service (titre Sceau v1.1 `0x03`, charge étendue du Market).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifesteService {
    /// Service (identifiant de voie).
    pub service: IdService,
    /// Version.
    pub version: String,
    /// SHA-256 de l'archive du paquet.
    pub sha256_archive: [u8; 32],
    /// Nom du binaire (facultatif, comme au Market).
    pub nom_binaire: Option<String>,
    /// Contrat.
    pub contrat: ContratService,
}

fn code_classe(c: ClasseExposition) -> u8 {
    match c {
        ClasseExposition::H0 => 0,
        ClasseExposition::H1 => 1,
        ClasseExposition::H2 => 2,
        ClasseExposition::H3 => 3,
    }
}

fn borne(v: u32, plafond: u32, quoi: &'static str) -> Result<(), ErreurManifeste> {
    if v == 0 || v > plafond {
        return Err(ErreurManifeste::Contrat(quoi));
    }
    Ok(())
}

fn u32_de(l: &mut LecteurTlv<'_>, tag: u8) -> Result<u32, ErreurManifeste> {
    u32::try_from(l.exiger_u64(tag)?).map_err(|_| ErreurManifeste::Sceau(ErreurSceau::ValeurInvalide(tag)))
}

fn version_valide(v: &str) -> bool {
    !v.is_empty() && v.len() <= MAX_VERSION && v.bytes().all(|b| b.is_ascii_graphic())
}

fn binaire_valide(v: &str) -> bool {
    !v.is_empty() && v.len() <= MAX_NOM_BINAIRE && !v.contains('\0')
}

impl ContratService {
    /// Contrôle des règles de classe, des bornes et du traité (voir l'en-tête du module).
    pub fn valider(&self, service: &IdService) -> Result<(), ErreurManifeste> {
        let r = &self.ressources;
        borne(r.cpu_pourcent, PLAFOND_CPU_POURCENT, "CPU hors bornes")?;
        borne(r.memoire_mio, PLAFOND_MEMOIRE_MIO, "mémoire hors bornes")?;
        borne(r.disque_mio, PLAFOND_DISQUE_MIO, "disque hors bornes")?;
        borne(r.taches_max, PLAFOND_TACHES, "tâches hors bornes")?;
        borne(r.sessions_max, PLAFOND_SESSIONS, "sessions hors bornes")?;
        match self.classe {
            ClasseExposition::H0 => {
                if self.voie.is_some() || self.traite.is_some() || self.facade.is_some() || !self.entrees.is_empty() {
                    return Err(ErreurManifeste::Classe("H0 : ni voie, ni traité, ni façade, ni entrée"));
                }
                return Ok(());
            }
            ClasseExposition::H3 => {
                return Err(ErreurManifeste::NonSpecifie(
                    "H3 : liste blanche sortante et intention StrongFather sans encodage dans le contrat",
                ));
            }
            ClasseExposition::H1 | ClasseExposition::H2 => {}
        }
        let contrat =
            self.contrat_voies(service).ok_or(ErreurManifeste::Classe("voie, traité et façade obligatoires"))?;
        contrat.valider().map_err(ErreurManifeste::Traite)?;
        let traite = &contrat.traite;
        if !traite.types.iter().any(|t| t.sens == Sens::Sortante) && self.classe == ClasseExposition::H1 {
            return Err(ErreurManifeste::Classe("H1 : au moins un type de publication"));
        }
        if self.classe == ClasseExposition::H2 && !traite.types.iter().any(|t| t.sens == Sens::Entrante) {
            return Err(ErreurManifeste::Classe("H2 : au moins un type entrant"));
        }
        let facade = self.facade.ok_or(ErreurManifeste::Classe("façade obligatoire"))?;
        if facade.db_secondaire_max_mio == 0 || facade.db_secondaire_max_mio > r.disque_mio {
            return Err(ErreurManifeste::Contrat("DB secondaire hors du disque déclaré"));
        }
        if facade.duree_max_session_secs == 0 || facade.duree_max_session_secs > DUREE_MAX_SESSION_SECS {
            return Err(ErreurManifeste::Contrat("durée de session hors bornes"));
        }
        if self.entrees.is_empty() || self.entrees.len() > MAX_ENTREES {
            return Err(ErreurManifeste::Contrat("entrées externes : 1 à 16"));
        }
        if self.entrees.windows(2).any(|p| p[0].encoder() >= p[1].encoder()) {
            return Err(ErreurManifeste::Contrat("entrées non canoniques"));
        }
        for e in &self.entrees {
            if e.port == 0 || e.debit_max_kio_s == 0 || e.taille_max_paquet < TAILLE_MIN_PAQUET {
                return Err(ErreurManifeste::Contrat("entrée hors bornes"));
            }
        }
        Ok(())
    }

    /// Contrat de voies pour le Portail (`None` en H0 ou si incomplet).
    pub fn contrat_voies(&self, service: &IdService) -> Option<ContratVoies> {
        let (voie, traite) = (self.voie?, self.traite.clone()?);
        Some(ContratVoies {
            service: service.clone(),
            classe: self.classe,
            traite,
            taille_max_trame: voie.taille_max_trame as usize,
            debit_max_par_minute: voie.debit_max_par_minute,
            quota_file: voie.quota_file as usize,
        })
    }

    /// Encodage TLV canonique du contrat (tag 5 du manifeste).
    pub fn encoder(&self) -> Result<Vec<u8>, ErreurSceau> {
        let mut e = EcrivainTlv::new();
        e.texte(C_GENRE, GENRE_CONTRAT)?.octets(C_CLASSE, &[code_classe(self.classe)])?.octets(C_EXPOSITION, &[0])?;
        let r = &self.ressources;
        let mut er = EcrivainTlv::new();
        er.u64(1, r.cpu_pourcent.into())?
            .u64(2, r.memoire_mio.into())?
            .u64(3, r.disque_mio.into())?
            .u64(4, r.taches_max.into())?
            .u64(5, r.sessions_max.into())?;
        e.octets(C_RESSOURCES, &er.finir())?;
        if let Some(v) = self.voie {
            let mut ev = EcrivainTlv::new();
            ev.u64(1, v.taille_max_trame.into())?.u64(2, v.debit_max_par_minute.into())?.u64(3, v.quota_file.into())?;
            e.octets(C_VOIE, &ev.finir())?;
        }
        if let Some(t) = &self.traite {
            e.octets(C_TRAITE, &t.encoder())?;
        }
        if let Some(f) = self.facade {
            let mut ef = EcrivainTlv::new();
            ef.b32(1, &f.image)?.u64(2, f.db_secondaire_max_mio.into())?.u64(3, f.duree_max_session_secs)?;
            e.octets(C_FACADE, &ef.finir())?;
        }
        if !self.entrees.is_empty() {
            let n = u8::try_from(self.entrees.len()).map_err(|_| ErreurSceau::ValeurInvalide(C_ENTREES))?;
            let mut liste = vec![n];
            for p in &self.entrees {
                liste.extend_from_slice(&p.encoder());
            }
            e.octets(C_ENTREES, &liste)?;
        }
        Ok(e.finir())
    }

    /// Décodage strict (structure seulement ; [`ContratService::valider`] ensuite).
    pub fn decoder(octets: &[u8]) -> Result<Self, ErreurManifeste> {
        let mut l = LecteurTlv::new(octets);
        if l.exiger_texte(C_GENRE)? != GENRE_CONTRAT {
            return Err(ErreurManifeste::Contrat("genre de contrat inconnu"));
        }
        let classe = match l.exiger(C_CLASSE)? {
            [0] => ClasseExposition::H0,
            [1] => ClasseExposition::H1,
            [2] => ClasseExposition::H2,
            [3] => ClasseExposition::H3,
            _ => return Err(ErreurManifeste::Contrat("classe inconnue")),
        };
        match l.exiger(C_EXPOSITION)? {
            [0] => {}
            [_] => return Err(ErreurManifeste::ExpositionParDefaut),
            _ => return Err(ErreurManifeste::Contrat("exposition mal formée")),
        }
        let mut lr = LecteurTlv::new(l.exiger(C_RESSOURCES)?);
        let ressources = Ressources {
            cpu_pourcent: u32_de(&mut lr, 1)?,
            memoire_mio: u32_de(&mut lr, 2)?,
            disque_mio: u32_de(&mut lr, 3)?,
            taches_max: u32_de(&mut lr, 4)?,
            sessions_max: u32_de(&mut lr, 5)?,
        };
        lr.terminer()?;
        let voie = match l.optionnel(C_VOIE)? {
            Some(o) => {
                let mut lv = LecteurTlv::new(o);
                let v = ContratVoie {
                    taille_max_trame: u32_de(&mut lv, 1)?,
                    debit_max_par_minute: u32_de(&mut lv, 2)?,
                    quota_file: u32_de(&mut lv, 3)?,
                };
                lv.terminer()?;
                Some(v)
            }
            None => None,
        };
        let traite = match l.optionnel(C_TRAITE)? {
            Some(o) => Some(Traite::decoder(o).map_err(|_| ErreurManifeste::Contrat("traité mal formé"))?),
            None => None,
        };
        let facade = match l.optionnel(C_FACADE)? {
            Some(o) => {
                let mut lf = LecteurTlv::new(o);
                let f = ContratFacade {
                    image: lf.exiger_b32(1)?,
                    db_secondaire_max_mio: u32_de(&mut lf, 2)?,
                    duree_max_session_secs: lf.exiger_u64(3)?,
                };
                lf.terminer()?;
                Some(f)
            }
            None => None,
        };
        let entrees = match l.optionnel(C_ENTREES)? {
            Some(o) => {
                let (&n, reste) = o.split_first().ok_or(ErreurManifeste::Contrat("entrées vides"))?;
                if n == 0 || reste.len() != usize::from(n) * TAILLE_ENTREE {
                    return Err(ErreurManifeste::Contrat("entrées mal formées"));
                }
                reste.as_chunks::<TAILLE_ENTREE>().0.iter().map(PointEntree::decoder).collect::<Result<Vec<_>, _>>()?
            }
            None => Vec::new(),
        };
        l.terminer()?;
        Ok(Self { classe, ressources, voie, traite, facade, entrees })
    }
}

// Le titre est décodé par `miyusceau::verifier`, qui exige un `ErreurSceau` :
// les refus de contrat y sont rapportés comme valeur invalide du tag 5, puis
// requalifiés par `verifier_manifeste` (second décodage, détaillé).
impl Titre for ManifesteService {
    const TYPE: TypeSceau = TypeSceau::ManifestePaquet;

    fn encoder(&self, e: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
        if !version_valide(&self.version) {
            return Err(ErreurSceau::ValeurInvalide(TAG_VERSION));
        }
        e.texte(TAG_SERVICE, self.service.as_str())?
            .texte(TAG_VERSION, &self.version)?
            .b32(TAG_ARCHIVE, &self.sha256_archive)?;
        if let Some(b) = &self.nom_binaire {
            if !binaire_valide(b) {
                return Err(ErreurSceau::ValeurInvalide(TAG_BINAIRE));
            }
            e.texte(TAG_BINAIRE, b)?;
        }
        e.octets(TAG_CONTRAT, &self.contrat.encoder()?)?;
        Ok(())
    }

    fn decoder(l: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
        let service =
            IdService::new(l.exiger_texte(TAG_SERVICE)?).map_err(|_| ErreurSceau::ValeurInvalide(TAG_SERVICE))?;
        let version = l.exiger_texte(TAG_VERSION)?.to_string();
        if !version_valide(&version) {
            return Err(ErreurSceau::ValeurInvalide(TAG_VERSION));
        }
        let sha256_archive = l.exiger_b32(TAG_ARCHIVE)?;
        let nom_binaire = match l.optionnel(TAG_BINAIRE)? {
            Some(o) => {
                let b = std::str::from_utf8(o).map_err(|_| ErreurSceau::ValeurInvalide(TAG_BINAIRE))?;
                if !binaire_valide(b) {
                    return Err(ErreurSceau::ValeurInvalide(TAG_BINAIRE));
                }
                Some(b.to_string())
            }
            None => None,
        };
        let contrat = ContratService::decoder(l.exiger(TAG_CONTRAT)?).map_err(|e| match e {
            ErreurManifeste::Sceau(s) => s,
            _ => ErreurSceau::ValeurInvalide(TAG_CONTRAT),
        })?;
        contrat.valider(&service).map_err(|_| ErreurSceau::ValeurInvalide(TAG_CONTRAT))?;
        Ok(Self { service, version, sha256_archive, nom_binaire, contrat })
    }
}

/// Manifeste vérifié : signature d'un éditeur épinglé, contrat valide.
/// Seul [`verifier_manifeste`] en construit (le détenir prouve le contrôle).
#[derive(Debug, Clone)]
pub struct ManifesteVerifie {
    verifie: Verifie<ManifesteService>,
    empreinte: [u8; 32],
}

impl ManifesteVerifie {
    /// Manifeste.
    pub fn manifeste(&self) -> &ManifesteService {
        self.verifie.titre()
    }

    /// Service.
    pub fn service(&self) -> &IdService {
        &self.manifeste().service
    }

    /// Classe d'exposition.
    pub fn classe(&self) -> ClasseExposition {
        self.manifeste().contrat.classe
    }

    /// Contrat de service.
    pub fn contrat(&self) -> &ContratService {
        &self.manifeste().contrat
    }

    /// Contrat de voies à déclarer au Portail (`None` en H0 : jamais de voie).
    pub fn contrat_voies(&self) -> Option<ContratVoies> {
        self.contrat().contrat_voies(self.service())
    }

    /// Audience des visas de la façade (= identifiant du service).
    pub fn audience(&self) -> &str {
        self.service().as_str()
    }

    /// `kid` de l'éditeur signataire.
    pub fn editeur(&self) -> &Kid {
        self.verifie.signataire()
    }

    /// SHA-256 des octets signés (identité du manifeste, à inscrire au registre).
    pub fn empreinte(&self) -> &[u8; 32] {
        &self.empreinte
    }
}

/// Scelle un manifeste avec la clé de l'éditeur, après validation du contrat.
pub fn sceller_manifeste(m: &ManifesteService, cle_editeur: &CleSignature) -> Result<Vec<u8>, ErreurManifeste> {
    m.contrat.valider(&m.service)?;
    Ok(miyusceau::sceller(m, cle_editeur)?)
}

/// Vérifie un manifeste signé (fail-closed) : éditeur épinglé, `verify_strict`,
/// TLV canonique, contrat conforme aux règles de classe.
pub fn verifier_manifeste(
    octets: &[u8],
    editeurs: &AncresDeConfiance,
    maintenant_secs: u64,
) -> Result<ManifesteVerifie, ErreurManifeste> {
    if editeurs.is_empty() {
        return Err(ErreurManifeste::AucuneAncre);
    }
    let verifie = match miyusceau::verifier::<ManifesteService>(octets, editeurs, maintenant_secs) {
        Ok(v) => v,
        Err(ErreurSceau::ValeurInvalide(TAG_CONTRAT)) => {
            // Requalifie le refus du contrat (motif détaillé).
            let sceau = miyusceau::Sceau::decoder(octets)?;
            let mut l = LecteurTlv::new(&sceau.charge);
            let service = IdService::new(l.exiger_texte(TAG_SERVICE)?)
                .map_err(|_| ErreurManifeste::Contrat("service invalide"))?;
            let _ = (l.exiger(TAG_VERSION)?, l.exiger(TAG_ARCHIVE)?, l.optionnel(TAG_BINAIRE)?);
            let contrat = ContratService::decoder(l.exiger(TAG_CONTRAT)?)?;
            contrat.valider(&service)?;
            return Err(ErreurManifeste::Contrat("contrat refusé"));
        }
        Err(e) => return Err(e.into()),
    };
    let empreinte = Sha256::digest(miyusceau::Sceau::decoder(octets)?.octets_signes()).into();
    Ok(ManifesteVerifie { verifie, empreinte })
}
