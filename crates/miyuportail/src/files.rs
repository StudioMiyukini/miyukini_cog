//! @id     miyuportail_files
//! @do     encoder_files_sens_unique_portail
//! @role   securite
//! @layer  toolkit
//! @human  miyuportail — files à sens unique entre la façade, l'entrée, le moteur et la sortie du Portail scindé : enveloppes (formats existants MSCE et MPI1 à l'intérieur), descripteurs de voie remis par les Cores, écriture atomique et lecture bornée (A-022 §3).

//! # Files à sens unique du Portail scindé (A-022 §3)
//!
//! Quatre files, chacune un répertoire **possédé par son lecteur**, de mode
//! `3730` (collant + setgid, groupe `-wx`, **rien** pour les autres) et dont
//! le groupe est **propre à la file** : il ne réunit que son écrivain et son
//! lecteur (`miyukini-q-depot`, `-q-admises`, `-q-reponses`,
//! `-q-publication`). L'écrivain peut **créer** sans lister ni effacer ce
//! qu'il n'a pas écrit ; un élément est créé en `0640` et hérite du groupe
//! de la file (setgid) : seul ce groupe peut le lire. L'entrée et la sortie
//! n'ont aucun groupe de file en commun :
//!
//! | file | écrivain → lecteur | contenu |
//! |---|---|---|
//! | `depot` | façade → **entrée** | trame MSCE `0x08` (format existant) |
//! | `admises` | **entrée** → moteur | [`Admise`] : jeton + déclaration au format interne `MPI1` |
//! | `reponses` | moteur → **sortie** | [`Reponse`] (jeton + 1..32 messages `MPI1`) ou [`PublicationH1`] |
//! | `publication` | **sortie** → façade | trame MSCE signée par la clé de la voie (format existant) |
//!
//! L'entrée n'a aucun droit sur `reponses` ni `publication` ; la sortie
//! aucun sur `depot` ni `admises`. Les deux moitiés n'ont **aucun** fichier
//! commun : chacune a son état (`/var/lib/miyukini/portail-entree`,
//! `…/portail-sortie`) et sa configuration (descripteurs [`DescripteurVoie`]
//! écrits par les Cores dans `/etc/miyukini/portail-entree/voies`,
//! `…/portail-sortie/voies`).
//!
//! Nom d'un élément de file : `<service>+<horodatage ns, 20 chiffres>-<pid>-<n>.<ext>`
//! (ordre de lecture = ordre lexicographique, donc d'écriture par service).
//! Écriture : fichier caché `.<nom>.tmp` créé exclusivement, mode `0640`
//! (jamais lisible des « autres »), `fsync`, puis `rename` (aucun lecteur ne
//! voit un élément partiel). Un descripteur portant un secret (graine des
//! jetons de l'entrée) est écrit en `0600` au nom du compte destinataire
//! ([`ecrire_descripteur`]).
//! Lecture : `O_NOFOLLOW`, fichier régulier seulement, taille bornée.
//!
//! Enveloppes (big-endian) — **formats nouveaux, à consigner par amendement** :
//!
//! ```text
//! Admise         "MPA1" | lj u16 | jeton (MJR1) | li u32 | MPI1
//! Réponse        "MPR1" | lj u16 | jeton (MJR1) | n u8 (1..=32) | n × [ li u32 | MPI1 ]
//! Publication H1 "MPP1" | étiquette u8 (1 nationale, 2 session, 3 publiable)
//!                       | déclencheur u8 (1 utilisateur, 2 MAIA, 3 service) | li u32 | MPI1
//! Descripteur    "MPV1" | rôle u8 (1 entrée, 2 sortie) | ls u8 | service | classe u8 (1 H1, 2 H2, 3 H3)
//!                       | taille u32 | débit u32 | quota u32 | état u8 (0 désactivée, 1 active, 2 coupée)
//!                       | motif u8 (0 aucun, 1 anomalie, 2 hors contrat, 3 décision) | rotation u64
//!                       | lt u32 | traité (encodage canonique) | session u8 (0 | 1)
//!                       | si session : id (32) | entrée : clé façade (32) | na u8 | na × auteur (32) | graine jetons (32)
//!                                             | sortie : ancre jetons (32)
//! ```
//!
//! Un message `MPI1` d'une réponse ou d'une publication est une déclaration
//! **sortante** du service, à la version du traité, séquence et date à 0,
//! sans signataire : la sortie lui donne séquence, date et signature.

use std::io::{Read, Write};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use crate::erreurs::RefusDecodage;
use crate::jeton::{IdSession, JetonReponse, TAILLE_MAX_JETON};
use crate::msce::{decoder_interne, encoder_interne, Declaration, TAILLE_MAX_TRAME};
use crate::portail::{ContratVoies, Declencheur, EtatVoie, Publication, Sensibilite};
use crate::registre::MotifCoupure;
use crate::sas::MAX_AUTEURS;
use crate::sortie::MAX_MESSAGES_REPONSE;
use crate::traite::{ClasseExposition, Curseur, IdService, Sens, Traite};

/// Magic d'une déclaration admise (entrée → moteur).
pub const MAGIC_ADMISE: &[u8; 4] = b"MPA1";
/// Magic d'une réponse (moteur → sortie).
pub const MAGIC_REPONSE: &[u8; 4] = b"MPR1";
/// Magic d'une publication H1 (moteur → sortie).
pub const MAGIC_PUBLICATION: &[u8; 4] = b"MPP1";
/// Magic d'un descripteur de voie (Cores → entrée ou sortie).
pub const MAGIC_DESCRIPTEUR: &[u8; 4] = b"MPV1";
/// Taille maximale d'une enveloppe (réponse de 32 messages).
pub const TAILLE_MAX_ENVELOPPE: usize = 4 + 2 + TAILLE_MAX_JETON + 1 + MAX_MESSAGES_REPONSE * (4 + TAILLE_MAX_TRAME);
/// Taille maximale d'un descripteur de voie.
pub const TAILLE_MAX_DESCRIPTEUR: usize = 256 * 1024;
/// Extension d'une trame MSCE (dépôt, publication).
pub const EXT_MSCE: &str = "msce";
/// Extension d'une déclaration admise.
pub const EXT_ADMISE: &str = "mpa";
/// Extension d'une réponse.
pub const EXT_REPONSE: &str = "mpr";
/// Extension d'une publication H1.
pub const EXT_PUBLICATION: &str = "mpp";
/// Extension d'un descripteur de voie.
pub const EXT_DESCRIPTEUR: &str = "voie";

// ---------------------------------------------------------------- enveloppes

fn jeton_lp(s: &mut Vec<u8>, jeton: &JetonReponse) {
    s.extend_from_slice(&(jeton.octets().len() as u16).to_be_bytes());
    s.extend_from_slice(jeton.octets());
}

fn interne_lp(s: &mut Vec<u8>, d: &Declaration) {
    let i = encoder_interne(d);
    s.extend_from_slice(&(i.len() as u32).to_be_bytes());
    s.extend_from_slice(&i);
}

fn lire_jeton(c: &mut Curseur<'_>) -> Result<JetonReponse, RefusDecodage> {
    let lj = c.prendre(2)?;
    let lj = usize::from(u16::from_be_bytes([lj[0], lj[1]]));
    JetonReponse::decoder(c.prendre(lj)?)
}

fn lire_interne(c: &mut Curseur<'_>) -> Result<Declaration, RefusDecodage> {
    let li = c.u32()? as usize;
    if li > TAILLE_MAX_TRAME {
        return Err(RefusDecodage::Taille);
    }
    decoder_interne(c.prendre(li)?)
}

/// Message sortant d'une réponse ou d'une publication : déclaration
/// sortante du service, séquence et date à 0, sans signataire.
fn message_conforme(service: &IdService, d: &Declaration) -> Result<(), RefusDecodage> {
    if d.sens != Sens::Sortante {
        return Err(RefusDecodage::Sens);
    }
    if &d.service != service {
        return Err(RefusDecodage::Service);
    }
    if d.sequence != 0 || d.emise_le != 0 || !d.signataires.is_empty() {
        return Err(RefusDecodage::Valeur);
    }
    Ok(())
}

/// Déclaration admise remise au moteur (entrée → moteur).
#[derive(Debug, PartialEq, Eq)]
pub struct Admise {
    /// Jeton de réponse (à recopier dans la réponse).
    pub jeton: JetonReponse,
    /// Structure typée (documents désarmés).
    pub declaration: Declaration,
}

impl Admise {
    /// Encode l'enveloppe.
    pub fn encoder(&self) -> Vec<u8> {
        let mut s = Vec::new();
        s.extend_from_slice(MAGIC_ADMISE);
        jeton_lp(&mut s, &self.jeton);
        interne_lp(&mut s, &self.declaration);
        s
    }

    /// Décodage strict (côté moteur).
    pub fn decoder(octets: &[u8]) -> Result<Self, RefusDecodage> {
        if octets.len() > TAILLE_MAX_ENVELOPPE {
            return Err(RefusDecodage::Taille);
        }
        let mut c = Curseur::new(octets);
        if c.prendre(4)? != MAGIC_ADMISE {
            return Err(RefusDecodage::Magic);
        }
        let jeton = lire_jeton(&mut c)?;
        let declaration = lire_interne(&mut c)?;
        c.fin()?;
        if declaration.sens != Sens::Entrante || &declaration.service != jeton.service() {
            return Err(RefusDecodage::Service);
        }
        Ok(Self { jeton, declaration })
    }
}

/// Réponse du moteur à une déclaration admise (moteur → sortie).
#[derive(Debug, PartialEq, Eq)]
pub struct Reponse {
    /// Jeton reçu avec la déclaration admise.
    pub jeton: JetonReponse,
    /// Messages sortants (1 à [`MAX_MESSAGES_REPONSE`]).
    pub messages: Vec<Declaration>,
}

impl Reponse {
    /// Encode l'enveloppe ; refus si hors bornes ou message non conforme.
    pub fn encoder(&self) -> Result<Vec<u8>, RefusDecodage> {
        if self.messages.is_empty() || self.messages.len() > MAX_MESSAGES_REPONSE {
            return Err(RefusDecodage::Taille);
        }
        let mut s = Vec::new();
        s.extend_from_slice(MAGIC_REPONSE);
        jeton_lp(&mut s, &self.jeton);
        s.push(self.messages.len() as u8);
        for m in &self.messages {
            message_conforme(self.jeton.service(), m)?;
            interne_lp(&mut s, m);
        }
        Ok(s)
    }

    /// Décodage strict (côté sortie).
    pub fn decoder(octets: &[u8]) -> Result<Self, RefusDecodage> {
        if octets.len() > TAILLE_MAX_ENVELOPPE {
            return Err(RefusDecodage::Taille);
        }
        let mut c = Curseur::new(octets);
        if c.prendre(4)? != MAGIC_REPONSE {
            return Err(RefusDecodage::Magic);
        }
        let jeton = lire_jeton(&mut c)?;
        let n = usize::from(c.u8()?);
        if n == 0 || n > MAX_MESSAGES_REPONSE {
            return Err(RefusDecodage::Taille);
        }
        let mut messages = Vec::with_capacity(n);
        for _ in 0..n {
            let m = lire_interne(&mut c)?;
            message_conforme(jeton.service(), &m)?;
            messages.push(m);
        }
        c.fin()?;
        Ok(Self { jeton, messages })
    }

    /// Publications (étiquette `session`, action de l'auteur de la demande).
    pub fn publications(&self) -> Vec<Publication> {
        self.messages
            .iter()
            .map(|m| Publication {
                type_code: m.type_code,
                champs: m.champs.clone(),
                etiquette: Sensibilite::Session,
                declencheur: Declencheur::Utilisateur,
            })
            .collect()
    }
}

/// Publication d'une voie H1 (moteur → sortie).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationH1 {
    /// Étiquette KindMother.
    pub etiquette: Sensibilite,
    /// Origine.
    pub declencheur: Declencheur,
    /// Message sortant.
    pub message: Declaration,
}

fn code_etiquette(e: Sensibilite) -> u8 {
    match e {
        Sensibilite::Nationale => 1,
        Sensibilite::Session => 2,
        Sensibilite::Publiable => 3,
    }
}

fn code_declencheur(d: Declencheur) -> u8 {
    match d {
        Declencheur::Utilisateur => 1,
        Declencheur::Maia => 2,
        Declencheur::Service => 3,
    }
}

impl PublicationH1 {
    /// Encode l'enveloppe.
    pub fn encoder(&self) -> Result<Vec<u8>, RefusDecodage> {
        message_conforme(&self.message.service, &self.message)?;
        let mut s = Vec::new();
        s.extend_from_slice(MAGIC_PUBLICATION);
        s.push(code_etiquette(self.etiquette));
        s.push(code_declencheur(self.declencheur));
        interne_lp(&mut s, &self.message);
        Ok(s)
    }

    /// Décodage strict (côté sortie).
    pub fn decoder(octets: &[u8]) -> Result<Self, RefusDecodage> {
        if octets.len() > TAILLE_MAX_ENVELOPPE {
            return Err(RefusDecodage::Taille);
        }
        let mut c = Curseur::new(octets);
        if c.prendre(4)? != MAGIC_PUBLICATION {
            return Err(RefusDecodage::Magic);
        }
        let etiquette = match c.u8()? {
            1 => Sensibilite::Nationale,
            2 => Sensibilite::Session,
            3 => Sensibilite::Publiable,
            _ => return Err(RefusDecodage::Valeur),
        };
        let declencheur = match c.u8()? {
            1 => Declencheur::Utilisateur,
            2 => Declencheur::Maia,
            3 => Declencheur::Service,
            _ => return Err(RefusDecodage::Valeur),
        };
        let message = lire_interne(&mut c)?;
        c.fin()?;
        message_conforme(&message.service, &message)?;
        Ok(Self { etiquette, declencheur, message })
    }

    /// Publication à remettre à la sortie.
    pub fn publication(&self) -> Publication {
        Publication {
            type_code: self.message.type_code,
            champs: self.message.champs.clone(),
            etiquette: self.etiquette,
            declencheur: self.declencheur,
        }
    }
}

// ---------------------------------------------------------------- descripteurs de voie

/// Moitié à laquelle un descripteur est destiné.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// Processus `portail-entree`.
    Entree = 1,
    /// Processus `portail-sortie`.
    Sortie = 2,
}

/// Session décrite par les Cores, selon la moitié.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionDecrite {
    /// Pour l'entrée : clé de façade, auteurs épinglés, graine des jetons.
    Entree {
        /// Identifiant de session.
        id: IdSession,
        /// Clé de session de la façade.
        cle_facade: [u8; 32],
        /// Auteurs épinglés.
        auteurs: Vec<[u8; 32]>,
        /// Graine de la clé de jetons.
        graine_jetons: [u8; 32],
    },
    /// Pour la sortie : ancre des jetons seulement.
    Sortie {
        /// Identifiant de session.
        id: IdSession,
        /// Clé publique des jetons.
        ancre_jetons: [u8; 32],
    },
}

impl SessionDecrite {
    /// Identifiant de la session.
    pub fn id(&self) -> &IdSession {
        match self {
            Self::Entree { id, .. } | Self::Sortie { id, .. } => id,
        }
    }
}

/// Descripteur d'une voie : configuration remise par les Cores à **une** moitié.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DescripteurVoie {
    /// Moitié destinataire.
    pub role: Role,
    /// Contrat de la voie.
    pub contrat: ContratVoies,
    /// État voulu.
    pub etat: EtatVoie,
    /// Compteur de rétablissements : sa hausse impose une nouvelle clé de publication.
    pub rotation: u64,
    /// Session ouverte (voie active seulement).
    pub session: Option<SessionDecrite>,
}

fn code_classe(c: ClasseExposition) -> u8 {
    match c {
        ClasseExposition::H0 => 0,
        ClasseExposition::H1 => 1,
        ClasseExposition::H2 => 2,
        ClasseExposition::H3 => 3,
    }
}

fn code_motif(m: MotifCoupure) -> u8 {
    match m {
        MotifCoupure::Anomalie => 1,
        MotifCoupure::HorsContrat => 2,
        MotifCoupure::Decision => 3,
    }
}

impl DescripteurVoie {
    /// Encode le descripteur ; refus si la session ne correspond pas au rôle.
    pub fn encoder(&self) -> Result<Vec<u8>, RefusDecodage> {
        let mut s = Vec::new();
        s.extend_from_slice(MAGIC_DESCRIPTEUR);
        s.push(self.role as u8);
        let so = self.contrat.service.as_str().as_bytes();
        s.push(so.len() as u8);
        s.extend_from_slice(so);
        s.push(code_classe(self.contrat.classe));
        for v in [self.contrat.taille_max_trame, self.contrat.debit_max_par_minute as usize, self.contrat.quota_file] {
            s.extend_from_slice(&u32::try_from(v).map_err(|_| RefusDecodage::Valeur)?.to_be_bytes());
        }
        let (etat, motif) = match self.etat {
            EtatVoie::Desactivee => (0, 0),
            EtatVoie::Active => (1, 0),
            EtatVoie::Coupee(m) => (2, code_motif(m)),
        };
        s.push(etat);
        s.push(motif);
        s.extend_from_slice(&self.rotation.to_be_bytes());
        let t = self.contrat.traite.encoder();
        s.extend_from_slice(&(t.len() as u32).to_be_bytes());
        s.extend_from_slice(&t);
        match (&self.session, self.role) {
            (None, _) => s.push(0),
            (Some(SessionDecrite::Entree { id, cle_facade, auteurs, graine_jetons }), Role::Entree) => {
                if auteurs.is_empty() || auteurs.len() > MAX_AUTEURS {
                    return Err(RefusDecodage::Valeur);
                }
                s.push(1);
                s.extend_from_slice(id);
                s.extend_from_slice(cle_facade);
                s.push(auteurs.len() as u8);
                for a in auteurs {
                    s.extend_from_slice(a);
                }
                s.extend_from_slice(graine_jetons);
            }
            (Some(SessionDecrite::Sortie { id, ancre_jetons }), Role::Sortie) => {
                s.push(1);
                s.extend_from_slice(id);
                s.extend_from_slice(ancre_jetons);
            }
            _ => return Err(RefusDecodage::Valeur),
        }
        if s.len() > TAILLE_MAX_DESCRIPTEUR {
            return Err(RefusDecodage::Taille);
        }
        Ok(s)
    }

    /// Décodage strict ; le rôle doit être celui attendu par le processus.
    pub fn decoder(octets: &[u8], attendu: Role) -> Result<Self, RefusDecodage> {
        if octets.len() > TAILLE_MAX_DESCRIPTEUR {
            return Err(RefusDecodage::Taille);
        }
        let mut c = Curseur::new(octets);
        if c.prendre(4)? != MAGIC_DESCRIPTEUR {
            return Err(RefusDecodage::Magic);
        }
        let role = match c.u8()? {
            1 => Role::Entree,
            2 => Role::Sortie,
            _ => return Err(RefusDecodage::Valeur),
        };
        if role != attendu {
            return Err(RefusDecodage::Valeur);
        }
        let ls = usize::from(c.u8()?);
        let service = std::str::from_utf8(c.prendre(ls)?)
            .ok()
            .and_then(|s| IdService::new(s).ok())
            .ok_or(RefusDecodage::Service)?;
        let classe = match c.u8()? {
            1 => ClasseExposition::H1,
            2 => ClasseExposition::H2,
            3 => ClasseExposition::H3,
            _ => return Err(RefusDecodage::Valeur),
        };
        let taille_max_trame = c.u32()? as usize;
        let debit_max_par_minute = c.u32()?;
        let quota_file = c.u32()? as usize;
        let etat = match (c.u8()?, c.u8()?) {
            (0, 0) => EtatVoie::Desactivee,
            (1, 0) => EtatVoie::Active,
            (2, 1) => EtatVoie::Coupee(MotifCoupure::Anomalie),
            (2, 2) => EtatVoie::Coupee(MotifCoupure::HorsContrat),
            (2, 3) => EtatVoie::Coupee(MotifCoupure::Decision),
            _ => return Err(RefusDecodage::Valeur),
        };
        let rotation = c.u64()?;
        let lt = c.u32()? as usize;
        let traite = Traite::decoder(c.prendre(lt)?)?;
        let session = match c.u8()? {
            0 => None,
            1 => {
                let id = c.b32()?;
                Some(match role {
                    Role::Entree => {
                        let cle_facade = c.b32()?;
                        let na = usize::from(c.u8()?);
                        if na == 0 || na > MAX_AUTEURS {
                            return Err(RefusDecodage::Valeur);
                        }
                        let mut auteurs = Vec::with_capacity(na);
                        for _ in 0..na {
                            auteurs.push(c.b32()?);
                        }
                        SessionDecrite::Entree { id, cle_facade, auteurs, graine_jetons: c.b32()? }
                    }
                    Role::Sortie => SessionDecrite::Sortie { id, ancre_jetons: c.b32()? },
                })
            }
            _ => return Err(RefusDecodage::Valeur),
        };
        c.fin()?;
        let contrat = ContratVoies { service, classe, traite, taille_max_trame, debit_max_par_minute, quota_file };
        contrat.valider().map_err(|_| RefusDecodage::Valeur)?;
        Ok(Self { role, contrat, etat, rotation, session })
    }
}

// ---------------------------------------------------------------- répertoires de file

/// Élément présent dans une file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    /// Nom de fichier (sans chemin).
    pub nom: String,
    /// Service lu dans le nom (`None` : nom non conforme, à retirer).
    pub service: Option<IdService>,
    /// Extension.
    pub extension: String,
}

/// Mode d'un élément de file : propriétaire (écrivain) et groupe de la file (lecteur).
pub const MODE_ELEMENT: u32 = 0o640;
/// Mode d'un descripteur portant un secret : compte destinataire seul.
pub const MODE_SECRET: u32 = 0o600;

/// Écrit le descripteur `<service>.voie` dans `dir` (côté Cores). Avec une
/// session d'entrée (graine des jetons : secret), le fichier est en `0600`
/// et remis au compte destinataire `proprietaire` (`(uid, gid)`, root
/// requis) ; sans secret, `0640`, groupe hérité du répertoire.
pub fn ecrire_descripteur(dir: &Path, d: &DescripteurVoie, proprietaire: Option<(u32, u32)>) -> std::io::Result<()> {
    let octets = d.encoder().map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string()))?;
    let secret = matches!(d.session, Some(SessionDecrite::Entree { .. }));
    let nom = format!("{}.{EXT_DESCRIPTEUR}", d.contrat.service);
    let (mode, prop) = if secret { (MODE_SECRET, proprietaire) } else { (MODE_ELEMENT, None) };
    File::new(dir).ecrire_avec(&nom, &octets, mode, prop)
}

/// Répertoire d'une file à sens unique.
#[derive(Debug, Clone)]
pub struct File {
    dir: PathBuf,
}

static COMPTEUR: AtomicU64 = AtomicU64::new(0);

fn nom_admis(nom: &str) -> bool {
    !nom.is_empty() && !nom.starts_with('.') && !nom.contains('/') && nom.len() <= 255
}

/// Analyse `<service>+<reste>.<ext>`.
pub fn analyser_nom(nom: &str) -> Element {
    let extension = nom.rsplit_once('.').map(|(_, e)| e.to_string()).unwrap_or_default();
    let service = nom.split_once('+').and_then(|(s, _)| IdService::new(s).ok());
    Element { nom: nom.to_string(), service, extension }
}

impl File {
    /// File dans `dir`.
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    /// Répertoire.
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    fn chemin(&self, nom: &str) -> std::io::Result<PathBuf> {
        if !nom_admis(nom) {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "nom d'élément refusé"));
        }
        Ok(self.dir.join(nom))
    }

    /// Écrit un élément (atomique) ; rend son nom.
    pub fn ecrire(&self, service: &IdService, extension: &str, octets: &[u8]) -> std::io::Result<String> {
        let ns = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_nanos());
        let n = COMPTEUR.fetch_add(1, Ordering::Relaxed);
        let nom = format!("{service}+{:020}-{}-{n:06}.{extension}", ns as u64, std::process::id());
        self.ecrire_nomme(&nom, octets)?;
        Ok(nom)
    }

    /// Écrit (ou remplace) l'élément `nom` de façon atomique (mode `0640`).
    pub fn ecrire_nomme(&self, nom: &str, octets: &[u8]) -> std::io::Result<()> {
        self.ecrire_avec(nom, octets, MODE_ELEMENT, None)
    }

    fn ecrire_avec(
        &self,
        nom: &str,
        octets: &[u8],
        mode: u32,
        proprietaire: Option<(u32, u32)>,
    ) -> std::io::Result<()> {
        let final_ = self.chemin(nom)?;
        let tmp = self.dir.join(format!(".{nom}.tmp"));
        let r = (|| {
            let mut f = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .mode(mode)
                .custom_flags(libc::O_NOFOLLOW)
                .open(&tmp)?;
            f.write_all(octets)?;
            // Mode explicite (propriétaire + groupe de la file au plus, jamais
            // les « autres ») : un masque plus strict que 0027 rendrait
            // l'élément illisible à son lecteur, membre du groupe de la file.
            f.set_permissions(std::fs::Permissions::from_mode(mode))?;
            if let Some((uid, gid)) = proprietaire {
                std::os::unix::fs::fchown(&f, Some(uid), Some(gid))?;
            }
            f.sync_all()?;
            std::fs::rename(&tmp, &final_)
        })();
        if r.is_err() {
            let _ = std::fs::remove_file(&tmp);
        }
        r
    }

    /// Éléments présents (au plus `max`), triés par nom ; fichiers cachés ignorés.
    pub fn elements(&self, max: usize) -> std::io::Result<Vec<Element>> {
        let mut noms = Vec::new();
        for e in std::fs::read_dir(&self.dir)? {
            let e = e?;
            if let Some(nom) = e.file_name().to_str() {
                if nom_admis(nom) {
                    noms.push(nom.to_string());
                }
            }
        }
        noms.sort();
        noms.truncate(max);
        Ok(noms.iter().map(|n| analyser_nom(n)).collect())
    }

    /// Lit un élément : lien refusé, fichier régulier seulement, au plus `max` octets.
    pub fn lire(&self, nom: &str, max: usize) -> std::io::Result<Vec<u8>> {
        let f = std::fs::OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
            .open(self.chemin(nom)?)?;
        let m = f.metadata()?;
        if !m.file_type().is_file() || m.len() > max as u64 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "élément refusé (type ou taille)"));
        }
        let mut octets = Vec::with_capacity(m.len() as usize);
        f.take(max as u64 + 1).read_to_end(&mut octets)?;
        if octets.len() > max {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "élément trop grand"));
        }
        Ok(octets)
    }

    /// Retire un élément (absent : sans erreur).
    pub fn retirer(&self, nom: &str) -> std::io::Result<()> {
        match std::fs::remove_file(self.chemin(nom)?) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            r => r,
        }
    }

    /// Vrai si l'élément existe encore (pas encore relevé par son lecteur).
    pub fn existe(&self, nom: &str) -> bool {
        self.chemin(nom).is_ok_and(|c| c.symlink_metadata().is_ok())
    }
}

#[cfg(test)]
#[allow(clippy::many_single_char_names)]
mod tests {
    use super::*;
    use crate::jeton::CleJetons;
    use crate::msce::{Champ, Valeur};
    use crate::traite::{ChampTraite, NatureChamp, TypeDeclaration};

    fn service() -> IdService {
        IdService::new("jayrdv").unwrap()
    }

    fn sortant(type_code: u64) -> Declaration {
        Declaration {
            service: service(),
            sens: Sens::Sortante,
            version_traite: 1,
            type_code,
            sequence: 0,
            emise_le: 0,
            champs: vec![Champ { tag: 1, valeur: Valeur::Entier(3) }],
            signataires: Vec::new(),
        }
    }

    fn contrat() -> ContratVoies {
        ContratVoies {
            service: service(),
            classe: ClasseExposition::H2,
            traite: Traite {
                version: 1,
                types: vec![TypeDeclaration {
                    code: 1,
                    sens: Sens::Entrante,
                    champs: vec![ChampTraite { tag: 1, nature: NatureChamp::Entier, obligatoire: true }],
                }],
            },
            taille_max_trame: 4096,
            debit_max_par_minute: 60,
            quota_file: 16,
        }
    }

    #[test]
    fn enveloppes_aller_retour() {
        let j = CleJetons::generer().emettre(&service(), &[1; 32], &[2; 32], 9);
        let r =
            Reponse { jeton: JetonReponse::decoder(j.octets()).unwrap(), messages: vec![sortant(101), sortant(102)] };
        let o = r.encoder().unwrap();
        assert_eq!(Reponse::decoder(&o).unwrap(), r);
        assert_eq!(Admise::decoder(&o), Err(RefusDecodage::Magic));
        assert_eq!(PublicationH1::decoder(&o), Err(RefusDecodage::Magic));

        let p = PublicationH1 {
            etiquette: Sensibilite::Publiable,
            declencheur: Declencheur::Utilisateur,
            message: sortant(5),
        };
        assert_eq!(PublicationH1::decoder(&p.encoder().unwrap()).unwrap(), p);

        // Message signé, séquencé ou entrant : refusé.
        let mut m = sortant(101);
        m.sequence = 4;
        let r = Reponse { jeton: JetonReponse::decoder(j.octets()).unwrap(), messages: vec![m] };
        assert!(r.encoder().is_err());
    }

    #[test]
    fn descripteurs_par_role() {
        let d = DescripteurVoie {
            role: Role::Sortie,
            contrat: contrat(),
            etat: EtatVoie::Coupee(MotifCoupure::Decision),
            rotation: 3,
            session: Some(SessionDecrite::Sortie { id: [4; 32], ancre_jetons: [5; 32] }),
        };
        let o = d.encoder().unwrap();
        assert_eq!(DescripteurVoie::decoder(&o, Role::Sortie).unwrap(), d);
        // Un descripteur de sortie n'est jamais lu par l'entrée.
        assert!(DescripteurVoie::decoder(&o, Role::Entree).is_err());
        // Session d'entrée dans un descripteur de sortie : refus à l'encodage.
        let faux = DescripteurVoie {
            session: Some(SessionDecrite::Entree {
                id: [4; 32],
                cle_facade: [1; 32],
                auteurs: vec![[2; 32]],
                graine_jetons: [3; 32],
            }),
            ..d
        };
        assert!(faux.encoder().is_err());
    }

    #[test]
    fn descripteur_secret_en_0600() {
        use std::os::unix::fs::MetadataExt;
        let dir = std::env::temp_dir().join(format!(
            "miyuportail-desc-{}-{}",
            std::process::id(),
            COMPTEUR.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let mut d = DescripteurVoie {
            role: Role::Entree,
            contrat: contrat(),
            etat: EtatVoie::Active,
            rotation: 0,
            session: None,
        };
        ecrire_descripteur(&dir, &d, None).unwrap();
        assert_eq!(std::fs::metadata(dir.join("jayrdv.voie")).unwrap().mode() & 0o777, 0o640);
        d.session = Some(SessionDecrite::Entree {
            id: [4; 32],
            cle_facade: [1; 32],
            auteurs: vec![[2; 32]],
            graine_jetons: [3; 32],
        });
        ecrire_descripteur(&dir, &d, None).unwrap();
        assert_eq!(std::fs::metadata(dir.join("jayrdv.voie")).unwrap().mode() & 0o777, 0o600);
        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn file_atomique_et_bornee() {
        let dir = std::env::temp_dir().join(format!(
            "miyuportail-file-{}-{}",
            std::process::id(),
            COMPTEUR.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let f = File::new(&dir);
        let a = f.ecrire(&service(), EXT_MSCE, b"un").unwrap();
        let b = f.ecrire(&service(), EXT_MSCE, b"deux").unwrap();
        let e = f.elements(10).unwrap();
        assert_eq!(e.iter().map(|x| x.nom.clone()).collect::<Vec<_>>(), vec![a.clone(), b.clone()]);
        assert_eq!(e[0].service.as_ref(), Some(&service()));
        assert_eq!(f.lire(&a, 16).unwrap(), b"un");
        assert!(f.lire(&b, 3).is_err());
        assert!(f.lire("../x", 3).is_err());
        std::os::unix::fs::symlink("/etc/hostname", dir.join("jayrdv+lien.msce")).unwrap();
        assert!(f.lire("jayrdv+lien.msce", 1024).is_err());
        // Permissions effectives : propriétaire + groupe, jamais les autres.
        use std::os::unix::fs::MetadataExt;
        assert_eq!(std::fs::metadata(dir.join(&b)).unwrap().mode() & 0o777, 0o640);
        f.retirer(&a).unwrap();
        assert!(!f.existe(&a));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
