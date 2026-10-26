//! @id     miyuportail_demon
//! @do     executer_entree_et_sortie_en_processus
//! @role   securite
//! @layer  toolkit
//! @human  miyuportail — boucles des processus `portail-entree` et `portail-sortie` : descripteurs des Cores, files à sens unique, état propre persisté (marques anti-rejeu, jetons consommés, clé de publication), contrôle du cloisonnement au démarrage (A-022 §3).

//! # Processus du Portail scindé (A-022 §3)
//!
//! [`DemonEntree`] et [`DemonSortie`] sont les boucles des binaires
//! `portail-entree` et `portail-sortie`. Ils ne partagent **rien** : chacun a
//! sa configuration (descripteurs de voie des Cores, rôle vérifié), son état
//! (répertoire propre, `0700`, son compte) et ses files, à sens unique
//! ([`crate::files`]).
//!
//! | processus | lit | écrit | état propre |
//! |---|---|---|---|
//! | entrée | `voies` (entrée), `depot` | `admises` | marques anti-rejeu (`<service>.rejeu`), journal |
//! | sortie | `voies` (sortie), `reponses` | `publication` | clé de publication (`<service>.cle`), jetons consommés (`<service>.jetons`), journal |
//!
//! - **Fail-closed au démarrage** : chaque processus refuse de démarrer si
//!   un de ses répertoires (état, files, descripteurs) est accessible aux
//!   « autres » ou n'appartient pas au compte attendu
//!   ([`controler_repertoire`]), ou s'il **peut** créer un fichier dans les
//!   files de l'autre sens ([`verifier_cloisonnement`]).
//! - **Usage unique tenu à travers un redémarrage** : la sortie persiste
//!   chaque jeton consommé **avant** d'écrire la publication ; l'entrée
//!   persiste ses marques anti-rejeu **avant** de remettre un lot au moteur.
//! - **Contre-pression** : un élément écrit et non encore relevé par le
//!   lecteur compte dans le quota de la voie ; au-delà, le processus laisse
//!   ses propres entrées en attente.
//! - **Panne** : la panne de l'un n'arrête pas l'autre ; la sortie continue
//!   de répondre aux déclarations déjà admises, l'entrée continue d'admettre.

use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use ed25519_dalek::SigningKey;
use rand_core::OsRng;

use crate::cdr::Desarmement;
use crate::entree::{Marque, PortailEntree, SessionEntree};
use crate::erreurs::ErreurPortail;
use crate::files::{
    Admise, DescripteurVoie, File, PublicationH1, Reponse, Role, SessionDecrite, EXT_ADMISE, EXT_DESCRIPTEUR, EXT_MSCE,
    EXT_PUBLICATION, EXT_REPONSE, TAILLE_MAX_DESCRIPTEUR, TAILLE_MAX_ENVELOPPE,
};
use crate::jeton::{CleJetons, IdSession};
use crate::msce::TAILLE_MAX_TRAME;
use crate::portail::{CoupureVoie, EtatVoie};
use crate::registre::{Empreinte, RegistreJournal};
use crate::sas::{DecodeurIsole, MAX_LOT};
use crate::sortie::{EtatCle, PortailSortie, SessionSortie};
use crate::traite::{Curseur, IdService};

/// Nombre maximal d'éléments lus par file et par passe.
pub const MAX_PAR_PASSE: usize = 1024;

/// Erreur d'un processus du Portail.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ErreurDemon {
    /// Entrée/sortie impossible sur l'état propre.
    #[error("état propre inaccessible : {0}")]
    Etat(String),
    /// Le processus peut écrire dans une file de l'autre sens : refus de démarrer.
    #[error("cloisonnement rompu : écriture possible dans {0}")]
    Cloisonnement(PathBuf),
    /// Répertoire trop ouvert (accessible aux « autres »), lien, ou mauvais propriétaire.
    #[error("répertoire refusé {0} : {1}")]
    Permissions(PathBuf, String),
}

/// Contrôle d'un répertoire au démarrage : répertoire réel (pas un lien),
/// **aucun** droit pour les « autres » (`mode & 0o007 == 0`) et, si
/// `proprietaire` est donné, appartenant à ce compte.
pub fn controler_repertoire(dir: &Path, proprietaire: Option<u32>) -> Result<(), ErreurDemon> {
    use std::os::unix::fs::MetadataExt;
    let refus = |m: String| ErreurDemon::Permissions(dir.to_path_buf(), m);
    let m = std::fs::symlink_metadata(dir).map_err(|e| refus(e.to_string()))?;
    if !m.file_type().is_dir() {
        return Err(refus("pas un répertoire".into()));
    }
    if m.mode() & 0o007 != 0 {
        return Err(refus(format!("accessible aux autres (mode {:o})", m.mode() & 0o7777)));
    }
    if let Some(uid) = proprietaire {
        if m.uid() != uid {
            return Err(refus(format!("propriétaire {} ≠ {uid} attendu", m.uid())));
        }
    }
    Ok(())
}

fn euid() -> u32 {
    rustix::process::geteuid().as_raw()
}

/// Bilan d'une passe.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Bilan {
    /// Éléments acceptés en entrée de file (trames déposées, réponses reçues).
    pub recus: u64,
    /// Éléments remis à la file aval (déclarations admises, trames publiées).
    pub emis: u64,
    /// Éléments refusés (et retirés).
    pub refuses: u64,
    /// Éléments laissés en attente (contre-pression).
    pub retenus: u64,
}

impl Bilan {
    fn vide(&self) -> bool {
        *self == Self::default()
    }
}

fn journal(processus: &str, message: &str) {
    eprintln!("{processus} : {message}");
}

/// Vérifie que ce processus ne peut **créer** aucun fichier dans les
/// répertoires `interdits` (files de l'autre sens) : sonde exclusive
/// immédiatement retirée. Une création réussie ⇒ [`ErreurDemon::Cloisonnement`].
pub fn verifier_cloisonnement(interdits: &[PathBuf]) -> Result<(), ErreurDemon> {
    for dir in interdits {
        let sonde = dir.join(format!(".sonde-cloisonnement-{}", std::process::id()));
        let essai = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW)
            .open(&sonde);
        if essai.is_ok() {
            let _ = std::fs::remove_file(&sonde);
            return Err(ErreurDemon::Cloisonnement(dir.clone()));
        }
    }
    Ok(())
}

/// Écrit un fichier d'état propre (`0600`), de façon atomique.
fn ecrire_etat(chemin: &Path, octets: &[u8]) -> std::io::Result<()> {
    let tmp = chemin.with_extension("tmp");
    let r = (|| {
        let _ = std::fs::remove_file(&tmp);
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .custom_flags(libc::O_NOFOLLOW)
            .open(&tmp)?;
        f.write_all(octets)?;
        f.set_permissions(std::fs::Permissions::from_mode(0o600))?;
        f.sync_all()?;
        std::fs::rename(&tmp, chemin)
    })();
    if r.is_err() {
        let _ = std::fs::remove_file(&tmp);
    }
    r
}

fn lire_etat(chemin: &Path, max: usize) -> Option<Vec<u8>> {
    let dir = chemin.parent()?;
    let nom = chemin.file_name()?.to_str()?;
    File::new(dir).lire(nom, max).ok()
}

fn retirer_etat(chemin: &Path) {
    let _ = std::fs::remove_file(chemin);
}

/// Descripteurs du rôle lus dans `dir` : `<service>.voie`, service du nom =
/// service du contrat ; tout descripteur illisible ou d'un autre rôle est
/// ignoré (voie absente : elle n'existe pas).
fn charger_descripteurs(
    processus: &str,
    dir: &Path,
    role: Role,
    signales: &mut BTreeSet<(String, Vec<u8>)>,
) -> BTreeMap<IdService, (Vec<u8>, DescripteurVoie)> {
    let mut voies = BTreeMap::new();
    let file = File::new(dir);
    let elements = match std::fs::read_dir(dir) {
        Ok(it) => it.filter_map(|e| e.ok()?.file_name().into_string().ok()).collect::<Vec<_>>(),
        Err(e) => {
            if signales.insert((String::new(), e.to_string().into_bytes())) {
                journal(processus, &format!("descripteurs illisibles ({}) : {e}", dir.display()));
            }
            return voies;
        }
    };
    for nom in elements {
        let Some(base) = nom.strip_suffix(&format!(".{EXT_DESCRIPTEUR}")) else {
            continue;
        };
        if nom.starts_with('.') {
            continue;
        }
        let Ok(octets) = file.lire(&nom, TAILLE_MAX_DESCRIPTEUR) else {
            continue;
        };
        let decode = DescripteurVoie::decoder(&octets, role);
        match decode {
            Ok(d) if d.contrat.service.as_str() == base => {
                voies.insert(d.contrat.service.clone(), (octets, d));
            }
            autre => {
                if signales.insert((nom.clone(), octets)) {
                    let motif = autre.map_or_else(|e| e.to_string(), |_| "service différent du nom".to_string());
                    journal(processus, &format!("descripteur {nom} refusé : {motif}"));
                }
            }
        }
    }
    voies
}

/// Transition d'état demandée par un descripteur (coupure inscrite au journal du processus).
fn appliquer_etat(
    etat_actuel: EtatVoie,
    voulu: EtatVoie,
    service: &IdService,
    couper: &dyn CoupureVoie,
    activer: &dyn Fn(&IdService) -> Result<(), ErreurPortail>,
    desactiver: &dyn Fn(&IdService) -> Result<(), ErreurPortail>,
    retablir: &dyn Fn(&IdService) -> Result<(), ErreurPortail>,
) -> Result<(), ErreurPortail> {
    match (etat_actuel, voulu) {
        (a, b) if a == b => Ok(()),
        (_, EtatVoie::Coupee(m)) => couper.couper(service, m),
        (EtatVoie::Coupee(_), EtatVoie::Desactivee) => retablir(service),
        (EtatVoie::Coupee(_), EtatVoie::Active) => retablir(service).and_then(|()| activer(service)),
        (EtatVoie::Desactivee, EtatVoie::Active) => activer(service),
        (EtatVoie::Active, EtatVoie::Desactivee) => desactiver(service),
        _ => Ok(()),
    }
}

/// Éléments écrits par ce processus et pas encore relevés par le lecteur.
fn en_attente(file: &File, emis: &mut Vec<String>) -> usize {
    emis.retain(|n| file.existe(n));
    emis.len()
}

/// Retire les éléments écrits et non relevés (coupure, désactivation, session close).
fn purger(file: &File, emis: &mut Vec<String>) {
    for n in emis.drain(..) {
        let _ = file.retirer(&n);
    }
}

// ---------------------------------------------------------------- entrée

/// Chemins du processus `portail-entree`.
#[derive(Debug, Clone)]
pub struct CheminsEntree {
    /// Descripteurs de voie (rôle entrée), écrits par les Cores.
    pub voies: PathBuf,
    /// État propre (marques anti-rejeu, journal).
    pub etat: PathBuf,
    /// File de dépôt (façade → entrée).
    pub depot: PathBuf,
    /// File des déclarations admises (entrée → moteur).
    pub admises: PathBuf,
    /// Files de l'autre sens : l'entrée ne doit pas pouvoir y écrire.
    pub interdits: Vec<PathBuf>,
}

const MAGIC_MARQUES: &[u8; 4] = b"MPM1";

fn encoder_marques(id: &IdSession, marques: &[Marque]) -> Vec<u8> {
    let mut s = Vec::with_capacity(4 + 32 + 2 + marques.len() * 40);
    s.extend_from_slice(MAGIC_MARQUES);
    s.extend_from_slice(id);
    s.extend_from_slice(&(marques.len() as u16).to_be_bytes());
    for (k, q) in marques {
        s.extend_from_slice(k);
        s.extend_from_slice(&q.to_be_bytes());
    }
    s
}

fn decoder_marques(octets: &[u8]) -> Option<(IdSession, Vec<Marque>)> {
    let mut c = Curseur::new(octets);
    if c.prendre(4).ok()? != MAGIC_MARQUES {
        return None;
    }
    let id = c.b32().ok()?;
    let n = c.prendre(2).ok()?;
    let n = usize::from(u16::from_be_bytes([n[0], n[1]]));
    let mut marques = Vec::with_capacity(n);
    for _ in 0..n {
        marques.push((c.b32().ok()?, c.u64().ok()?));
    }
    c.fin().ok()?;
    Some((id, marques))
}

/// Processus `portail-entree` : relève, ne publie jamais.
pub struct DemonEntree {
    chemins: CheminsEntree,
    portail: PortailEntree,
    connus: BTreeMap<IdService, (Vec<u8>, DescripteurVoie)>,
    signales: BTreeSet<(String, Vec<u8>)>,
    depot: File,
    admises: File,
    emis: BTreeMap<IdService, Vec<String>>,
    marques_ecrites: BTreeMap<IdService, Vec<u8>>,
}

impl std::fmt::Debug for DemonEntree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DemonEntree").field("chemins", &self.chemins).finish_non_exhaustive()
    }
}

const ENTREE: &str = "portail-entree";

impl DemonEntree {
    /// Processus d'entrée : `sas` = [`crate::sas::SasProcessus`] en production.
    pub fn new(
        chemins: CheminsEntree,
        sas: Box<dyn DecodeurIsole>,
        cdr: Box<dyn Desarmement>,
    ) -> Result<Self, ErreurDemon> {
        let registre = RegistreJournal::ouvrir(&chemins.etat.join("registre.journal"))
            .map_err(|e| ErreurDemon::Etat(e.to_string()))?;
        Ok(Self {
            depot: File::new(&chemins.depot),
            admises: File::new(&chemins.admises),
            portail: PortailEntree::new(sas, cdr, Arc::new(registre)),
            chemins,
            connus: BTreeMap::new(),
            signales: BTreeSet::new(),
            emis: BTreeMap::new(),
            marques_ecrites: BTreeMap::new(),
        })
    }

    /// Moitié entrée tenue par ce processus.
    pub fn portail(&self) -> &PortailEntree {
        &self.portail
    }

    /// Contrôle de cloisonnement (fail-closed au démarrage).
    pub fn verifier_cloisonnement(&self) -> Result<(), ErreurDemon> {
        verifier_cloisonnement(&self.chemins.interdits)
    }

    /// Contrôles du démarrage : état et dépôt à ce compte, files et
    /// descripteurs fermés aux autres, puis cloisonnement. Échec = refus de démarrer.
    pub fn verifier_au_demarrage(&self) -> Result<(), ErreurDemon> {
        let moi = euid();
        controler_repertoire(&self.chemins.etat, Some(moi))?;
        controler_repertoire(&self.chemins.depot, Some(moi))?;
        controler_repertoire(&self.chemins.admises, None)?;
        controler_repertoire(&self.chemins.voies, None)?;
        self.verifier_cloisonnement()
    }

    fn chemin_marques(&self, s: &IdService) -> PathBuf {
        self.chemins.etat.join(format!("{s}.rejeu"))
    }

    /// Une passe : descripteurs, dépôt, relève.
    pub fn une_passe(&mut self) -> Bilan {
        let mut b = Bilan::default();
        self.synchroniser();
        self.lire_depot(&mut b);
        self.relever_tout(&mut b);
        if !b.vide() {
            journal(
                ENTREE,
                &format!(
                    "passe : {} déposées, {} admises, {} refusées, {} en attente",
                    b.recus, b.emis, b.refuses, b.retenus
                ),
            );
        }
        b
    }

    fn purger_emis(&mut self, s: &IdService) {
        if let Some(e) = self.emis.get_mut(s) {
            purger(&self.admises, e);
        }
    }

    fn synchroniser(&mut self) {
        let voulus = charger_descripteurs(ENTREE, &self.chemins.voies, Role::Entree, &mut self.signales);
        let partis: Vec<IdService> = self.connus.keys().filter(|s| !voulus.contains_key(*s)).cloned().collect();
        for s in partis {
            let _ = self.portail.supprimer(&s);
            self.purger_emis(&s);
            retirer_etat(&self.chemin_marques(&s));
            self.connus.remove(&s);
            self.marques_ecrites.remove(&s);
            journal(ENTREE, &format!("voie {s} supprimée"));
        }
        for (s, (octets, d)) in voulus {
            if self.connus.get(&s).is_some_and(|(o, _)| *o == octets) {
                continue;
            }
            let ancien = self.connus.get(&s).map(|(_, a)| a.clone());
            match self.appliquer(ancien.as_ref(), &d) {
                Ok(()) => journal(ENTREE, &format!("voie {s} : descripteur appliqué ({:?})", d.etat)),
                Err(e) => journal(ENTREE, &format!("voie {s} : descripteur appliqué en partie : {e}")),
            }
            self.connus.insert(s, (octets, d));
        }
    }

    fn appliquer(&mut self, ancien: Option<&DescripteurVoie>, d: &DescripteurVoie) -> Result<(), ErreurPortail> {
        let s = &d.contrat.service;
        if ancien.map(|a| &a.contrat) != Some(&d.contrat) {
            if ancien.is_some() {
                let _ = self.portail.supprimer(s);
                self.purger_emis(s);
            }
            self.portail.declarer(d.contrat.clone())?;
        }
        let avant = self.portail.etat(s)?;
        let p = &self.portail;
        appliquer_etat(avant, d.etat, s, p, &|x| p.activer(x), &|x| p.desactiver(x), &|x| p.retablir(x))?;
        if d.etat != EtatVoie::Active {
            self.purger_emis(s);
            return Ok(());
        }
        let actuelle = self.portail.session(s)?;
        match &d.session {
            None => {
                if actuelle.is_some() {
                    self.portail.fermer_session(s)?;
                    self.purger_emis(s);
                }
            }
            Some(SessionDecrite::Entree { id, cle_facade, auteurs, graine_jetons }) => {
                if actuelle == Some(*id) {
                    let presents = self.portail.auteurs(s)?;
                    for a in auteurs.iter().filter(|a| !presents.contains(a)) {
                        let _ = self.portail.ajouter_auteur(s, *a);
                    }
                    for a in presents.iter().filter(|a| !auteurs.contains(a)) {
                        self.portail.retirer_auteur(s, a)?;
                    }
                    for a in auteurs.iter().filter(|a| !presents.contains(a)) {
                        match self.portail.ajouter_auteur(s, *a) {
                            Ok(()) | Err(ErreurPortail::AuteurDejaEpingle) => {}
                            Err(e) => return Err(e),
                        }
                    }
                } else {
                    self.purger_emis(s);
                    self.portail.ouvrir_session(
                        s,
                        SessionEntree {
                            id: *id,
                            cle_facade: *cle_facade,
                            auteurs: auteurs.clone(),
                            cle_jetons: CleJetons::depuis_graine(graine_jetons),
                        },
                    )?;
                    // Redémarrage : marques de la même session restaurées (aucun rejeu).
                    if let Some((sid, marques)) =
                        lire_etat(&self.chemin_marques(s), 4 + 32 + 2 + 65_535 * 40).and_then(|o| decoder_marques(&o))
                    {
                        if sid == *id {
                            self.portail.restaurer_marques(s, id, &marques)?;
                        }
                    }
                }
            }
            Some(SessionDecrite::Sortie { .. }) => return Err(ErreurPortail::TransitionRefusee),
        }
        Ok(())
    }

    fn lire_depot(&mut self, b: &mut Bilan) {
        let elements = match self.depot.elements(MAX_PAR_PASSE) {
            Ok(e) => e,
            Err(e) => {
                journal(ENTREE, &format!("dépôt illisible : {e}"));
                return;
            }
        };
        let mut retenus: BTreeSet<IdService> = BTreeSet::new();
        for e in elements {
            let service = match &e.service {
                Some(s) if self.connus.contains_key(s) && e.extension == EXT_MSCE => s.clone(),
                _ => {
                    // Nom non conforme, voie inconnue, ou autre chose qu'une trame
                    // MSCE (réponse, publication…) : l'entrée n'en fait rien.
                    let _ = self.depot.retirer(&e.nom);
                    b.refuses += 1;
                    journal(ENTREE, &format!("dépôt {} refusé (voie inconnue ou élément non admis)", e.nom));
                    continue;
                }
            };
            if retenus.contains(&service) {
                b.retenus += 1;
                continue;
            }
            let trame = match self.depot.lire(&e.nom, TAILLE_MAX_TRAME) {
                Ok(t) => t,
                Err(err) => {
                    let _ = self.depot.retirer(&e.nom);
                    b.refuses += 1;
                    journal(ENTREE, &format!("dépôt {} refusé : {err}", e.nom));
                    continue;
                }
            };
            match self.portail.deposer(&service, trame) {
                Ok(()) => {
                    let _ = self.depot.retirer(&e.nom);
                    b.recus += 1;
                }
                Err(ErreurPortail::FilePleine | ErreurPortail::DebitDepasse) => {
                    retenus.insert(service);
                    b.retenus += 1;
                }
                Err(err) => {
                    let _ = self.depot.retirer(&e.nom);
                    b.refuses += 1;
                    journal(ENTREE, &format!("dépôt {} refusé : {err}", e.nom));
                }
            }
        }
    }

    fn persister_marques(&mut self, s: &IdService) -> Result<(), ErreurDemon> {
        let Some((id, marques)) = self.portail.marques(s).map_err(|e| ErreurDemon::Etat(e.to_string()))? else {
            return Ok(());
        };
        let octets = encoder_marques(&id, &marques);
        if self.marques_ecrites.get(s) == Some(&octets) {
            return Ok(());
        }
        ecrire_etat(&self.chemin_marques(s), &octets).map_err(|e| ErreurDemon::Etat(e.to_string()))?;
        self.marques_ecrites.insert(s.clone(), octets);
        Ok(())
    }

    fn relever_tout(&mut self, b: &mut Bilan) {
        let services: Vec<(IdService, usize)> = self
            .connus
            .iter()
            .filter(|(_, (_, d))| d.etat == EtatVoie::Active && d.session.is_some())
            .map(|(s, (_, d))| (s.clone(), d.contrat.quota_file))
            .collect();
        for (s, quota) in services {
            loop {
                let attente = en_attente(&self.admises, self.emis.entry(s.clone()).or_default());
                if attente >= quota {
                    b.retenus += 1;
                    break;
                }
                let lot = match self.portail.relever(&s, MAX_LOT.min(quota - attente)) {
                    Ok(l) if !l.is_empty() => l,
                    _ => break,
                };
                // Marques persistées AVANT toute remise au moteur : un
                // redémarrage ne réadmet jamais une trame déjà remise.
                if let Err(e) = self.persister_marques(&s) {
                    journal(ENTREE, &format!("voie {s} : marques non persistées, lot refusé : {e}"));
                    b.refuses += lot.len() as u64;
                    break;
                }
                for r in lot {
                    match r {
                        Ok(a) => {
                            let env = Admise { jeton: a.jeton, declaration: a.declaration }.encoder();
                            match self.admises.ecrire(&s, EXT_ADMISE, &env) {
                                Ok(nom) => {
                                    self.emis.entry(s.clone()).or_default().push(nom);
                                    b.emis += 1;
                                }
                                Err(e) => {
                                    b.refuses += 1;
                                    journal(ENTREE, &format!("voie {s} : remise au moteur impossible : {e}"));
                                }
                            }
                        }
                        Err(e) => {
                            b.refuses += 1;
                            journal(ENTREE, &format!("voie {s} : déclaration refusée : {e}"));
                        }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------- sortie

/// Chemins du processus `portail-sortie`.
#[derive(Debug, Clone)]
pub struct CheminsSortie {
    /// Descripteurs de voie (rôle sortie), écrits par les Cores.
    pub voies: PathBuf,
    /// État propre (clés de publication, jetons consommés, journal).
    pub etat: PathBuf,
    /// File des réponses et publications H1 (moteur → sortie).
    pub reponses: PathBuf,
    /// File de publication (sortie → façade).
    pub publication: PathBuf,
    /// Files de l'autre sens : la sortie ne doit pas pouvoir y écrire.
    pub interdits: Vec<PathBuf>,
}

const MAGIC_CLE: &[u8; 4] = b"MPK1";
const MAGIC_JETONS: &[u8; 4] = b"MPJ1";

fn encoder_cle(rotation: u64, (graine, sequence): &EtatCle) -> Vec<u8> {
    let mut s = Vec::with_capacity(4 + 8 + 32 + 8);
    s.extend_from_slice(MAGIC_CLE);
    s.extend_from_slice(&rotation.to_be_bytes());
    s.extend_from_slice(graine);
    s.extend_from_slice(&sequence.to_be_bytes());
    s
}

fn decoder_cle(octets: &[u8]) -> Option<(u64, EtatCle)> {
    let mut c = Curseur::new(octets);
    if c.prendre(4).ok()? != MAGIC_CLE {
        return None;
    }
    let rotation = c.u64().ok()?;
    let graine = c.b32().ok()?;
    let sequence = c.u64().ok()?;
    c.fin().ok()?;
    Some((rotation, (graine, sequence)))
}

fn decoder_jetons(octets: &[u8]) -> Option<(IdSession, Vec<Empreinte>)> {
    let mut c = Curseur::new(octets);
    if c.prendre(4).ok()? != MAGIC_JETONS {
        return None;
    }
    let id = c.b32().ok()?;
    let reste = octets.len().checked_sub(36)?;
    if reste % 32 != 0 {
        return None;
    }
    let mut e = Vec::with_capacity(reste / 32);
    for _ in 0..reste / 32 {
        e.push(c.b32().ok()?);
    }
    Some((id, e))
}

/// Processus `portail-sortie` : publie, ne relève jamais.
pub struct DemonSortie {
    chemins: CheminsSortie,
    portail: PortailSortie,
    connus: BTreeMap<IdService, (Vec<u8>, DescripteurVoie)>,
    signales: BTreeSet<(String, Vec<u8>)>,
    reponses: File,
    publication: File,
    emis: BTreeMap<IdService, Vec<String>>,
    cles_ecrites: BTreeMap<IdService, Vec<u8>>,
    cles_publiees: BTreeMap<IdService, [u8; 32]>,
}

impl std::fmt::Debug for DemonSortie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DemonSortie").field("chemins", &self.chemins).finish_non_exhaustive()
    }
}

const SORTIE: &str = "portail-sortie";

/// Nom, dans la file de publication, de la clé publique de publication d'une voie.
pub fn nom_cle_publication(service: &IdService) -> String {
    format!("{service}.cle")
}

impl DemonSortie {
    /// Processus de sortie.
    pub fn new(chemins: CheminsSortie, cdr: Box<dyn Desarmement>) -> Result<Self, ErreurDemon> {
        let registre = RegistreJournal::ouvrir(&chemins.etat.join("registre.journal"))
            .map_err(|e| ErreurDemon::Etat(e.to_string()))?;
        Ok(Self {
            reponses: File::new(&chemins.reponses),
            publication: File::new(&chemins.publication),
            portail: PortailSortie::new(cdr, Arc::new(registre)),
            chemins,
            connus: BTreeMap::new(),
            signales: BTreeSet::new(),
            emis: BTreeMap::new(),
            cles_ecrites: BTreeMap::new(),
            cles_publiees: BTreeMap::new(),
        })
    }

    /// Moitié sortie tenue par ce processus.
    pub fn portail(&self) -> &PortailSortie {
        &self.portail
    }

    /// Contrôle de cloisonnement (fail-closed au démarrage).
    pub fn verifier_cloisonnement(&self) -> Result<(), ErreurDemon> {
        verifier_cloisonnement(&self.chemins.interdits)
    }

    /// Contrôles du démarrage : état et réponses à ce compte, files et
    /// descripteurs fermés aux autres, puis cloisonnement. Échec = refus de démarrer.
    pub fn verifier_au_demarrage(&self) -> Result<(), ErreurDemon> {
        let moi = euid();
        controler_repertoire(&self.chemins.etat, Some(moi))?;
        controler_repertoire(&self.chemins.reponses, Some(moi))?;
        controler_repertoire(&self.chemins.publication, None)?;
        controler_repertoire(&self.chemins.voies, None)?;
        self.verifier_cloisonnement()
    }

    fn chemin_cle(&self, s: &IdService) -> PathBuf {
        self.chemins.etat.join(format!("{s}.cle"))
    }

    fn chemin_jetons(&self, s: &IdService) -> PathBuf {
        self.chemins.etat.join(format!("{s}.jetons"))
    }

    /// Une passe : descripteurs, réponses et publications H1.
    pub fn une_passe(&mut self) -> Bilan {
        let mut b = Bilan::default();
        self.synchroniser();
        self.traiter_reponses(&mut b);
        if !b.vide() {
            journal(
                SORTIE,
                &format!(
                    "passe : {} reçues, {} publiées, {} refusées, {} en attente",
                    b.recus, b.emis, b.refuses, b.retenus
                ),
            );
        }
        b
    }

    fn purger_emis(&mut self, s: &IdService) {
        if let Some(e) = self.emis.get_mut(s) {
            purger(&self.publication, e);
        }
    }

    fn synchroniser(&mut self) {
        let voulus = charger_descripteurs(SORTIE, &self.chemins.voies, Role::Sortie, &mut self.signales);
        let partis: Vec<IdService> = self.connus.keys().filter(|s| !voulus.contains_key(*s)).cloned().collect();
        for s in partis {
            let _ = self.portail.supprimer(&s);
            self.purger_emis(&s);
            let _ = self.publication.retirer(&nom_cle_publication(&s));
            retirer_etat(&self.chemin_cle(&s));
            retirer_etat(&self.chemin_jetons(&s));
            self.connus.remove(&s);
            self.cles_ecrites.remove(&s);
            self.cles_publiees.remove(&s);
            journal(SORTIE, &format!("voie {s} supprimée"));
        }
        for (s, (octets, d)) in voulus {
            if self.connus.get(&s).is_some_and(|(o, _)| *o == octets) {
                continue;
            }
            let ancien = self.connus.get(&s).map(|(_, a)| a.clone());
            match self.appliquer(ancien.as_ref(), &d) {
                Ok(()) => journal(SORTIE, &format!("voie {s} : descripteur appliqué ({:?})", d.etat)),
                Err(e) => journal(SORTIE, &format!("voie {s} : descripteur appliqué en partie : {e}")),
            }
            self.connus.insert(s, (octets, d));
        }
    }

    /// Persiste la clé de publication (avec la rotation du descripteur).
    fn persister_cle(&mut self, s: &IdService, rotation: u64) -> Result<(), ErreurDemon> {
        let etat = self.portail.etat_cle(s).map_err(|e| ErreurDemon::Etat(e.to_string()))?;
        let octets = encoder_cle(rotation, &etat);
        if self.cles_ecrites.get(s) != Some(&octets) {
            ecrire_etat(&self.chemin_cle(s), &octets).map_err(|e| ErreurDemon::Etat(e.to_string()))?;
            self.cles_ecrites.insert(s.clone(), octets);
        }
        let publique = self.portail.cle_publication(s).map_err(|e| ErreurDemon::Etat(e.to_string()))?;
        if self.cles_publiees.get(s) != Some(&publique) {
            let texte = format!("{}\n", crate::hex(&publique));
            self.publication
                .ecrire_nomme(&nom_cle_publication(s), texte.as_bytes())
                .map_err(|e| ErreurDemon::Etat(e.to_string()))?;
            self.cles_publiees.insert(s.clone(), publique);
        }
        Ok(())
    }

    fn appliquer(&mut self, ancien: Option<&DescripteurVoie>, d: &DescripteurVoie) -> Result<(), ErreurPortail> {
        let s = d.contrat.service.clone();
        let etat_err = |e: ErreurDemon| {
            journal(SORTIE, &format!("voie {s} : {e}"));
            ErreurPortail::EtatIndisponible
        };
        if ancien.map(|a| &a.contrat) != Some(&d.contrat) {
            if ancien.is_some() {
                let _ = self.portail.supprimer(&s);
                self.purger_emis(&s);
                self.cles_ecrites.remove(&s);
            }
            self.portail.declarer(d.contrat.clone())?;
            // Clé persistée reprise seulement pour la même rotation.
            if let Some((rotation, etat)) = lire_etat(&self.chemin_cle(&s), 64).and_then(|o| decoder_cle(&o)) {
                if rotation == d.rotation {
                    self.portail.installer_cle(&s, etat)?;
                }
            }
        } else if ancien.is_some_and(|a| a.rotation != d.rotation) {
            self.portail.installer_cle(&s, (SigningKey::generate(&mut OsRng).to_bytes(), 0))?;
            self.purger_emis(&s);
        }
        let avant = self.portail.etat(&s)?;
        let p = &self.portail;
        appliquer_etat(avant, d.etat, &s, p, &|x| p.activer(x), &|x| p.desactiver(x), &|x| p.retablir(x))?;
        if let EtatVoie::Coupee(_) = d.etat {
            // Coupure : publications en vol retirées, clé détruite (le
            // rétablissement en tire une neuve, même après un redémarrage).
            self.purger_emis(&s);
            retirer_etat(&self.chemin_cle(&s));
            self.cles_ecrites.remove(&s);
            let _ = self.publication.retirer(&nom_cle_publication(&s));
            self.cles_publiees.remove(&s);
            return Ok(());
        }
        self.persister_cle(&s, d.rotation).map_err(etat_err)?;
        if d.etat != EtatVoie::Active {
            self.purger_emis(&s);
            return Ok(());
        }
        let actuelle = self.portail.session(&s)?;
        match &d.session {
            None => {
                if actuelle.is_some() {
                    self.portail.fermer_session(&s)?;
                }
            }
            Some(SessionDecrite::Sortie { id, ancre_jetons }) => {
                if actuelle != Some(*id) {
                    self.portail.ouvrir_session(&s, SessionSortie { id: *id, ancre_jetons: *ancre_jetons })?;
                    let chemin = self.chemin_jetons(&s);
                    match lire_etat(&chemin, 36 + 32 * crate::sortie::MAX_JETONS_CONSOMMES)
                        .and_then(|o| decoder_jetons(&o))
                    {
                        Some((sid, consommes)) if sid == *id => self.portail.restaurer_jetons(&s, id, &consommes)?,
                        _ => {
                            let mut entete = MAGIC_JETONS.to_vec();
                            entete.extend_from_slice(id);
                            ecrire_etat(&chemin, &entete)
                                .map_err(|e| ErreurDemon::Etat(e.to_string()))
                                .map_err(etat_err)?;
                        }
                    }
                }
            }
            Some(SessionDecrite::Entree { .. }) => return Err(ErreurPortail::TransitionRefusee),
        }
        Ok(())
    }

    /// Ajoute un jeton consommé à l'état propre (avant toute publication).
    fn persister_jeton(&self, s: &IdService, empreinte: &Empreinte) -> std::io::Result<()> {
        let mut f =
            std::fs::OpenOptions::new().append(true).custom_flags(libc::O_NOFOLLOW).open(self.chemin_jetons(s))?;
        f.write_all(empreinte)?;
        f.sync_data()
    }

    fn traiter_reponses(&mut self, b: &mut Bilan) {
        let elements = match self.reponses.elements(MAX_PAR_PASSE) {
            Ok(e) => e,
            Err(e) => {
                journal(SORTIE, &format!("file des réponses illisible : {e}"));
                return;
            }
        };
        let mut retenus: BTreeSet<IdService> = BTreeSet::new();
        for e in elements {
            let Some((service, quota)) =
                e.service.as_ref().and_then(|s| self.connus.get(s).map(|(_, d)| (s.clone(), d.contrat.quota_file)))
            else {
                let _ = self.reponses.retirer(&e.nom);
                b.refuses += 1;
                journal(SORTIE, &format!("élément {} refusé (voie inconnue)", e.nom));
                continue;
            };
            if retenus.contains(&service)
                || en_attente(&self.publication, self.emis.entry(service.clone()).or_default()) >= quota
            {
                retenus.insert(service);
                b.retenus += 1;
                continue;
            }
            let resultat =
                self.reponses.lire(&e.nom, TAILLE_MAX_ENVELOPPE).map_err(|x| x.to_string()).and_then(|octets| match e
                    .extension
                    .as_str()
                {
                    EXT_REPONSE => self.repondre(&service, &octets),
                    EXT_PUBLICATION => self.publier_h1(&service, &octets),
                    // Trame déposée, déclaration admise… : la sortie ne relève rien.
                    _ => Err("élément non admis en sortie (aucune relève possible)".to_string()),
                });
            let _ = self.reponses.retirer(&e.nom);
            match resultat {
                Ok(()) => {
                    b.recus += 1;
                    b.emis += self.emettre(&service);
                }
                Err(motif) => {
                    b.refuses += 1;
                    journal(SORTIE, &format!("élément {} refusé : {motif}", e.nom));
                }
            }
        }
    }

    fn repondre(&mut self, s: &IdService, octets: &[u8]) -> Result<(), String> {
        let r = Reponse::decoder(octets).map_err(|e| e.to_string())?;
        if r.jeton.service() != s {
            return Err(ErreurPortail::JetonInvalide.to_string());
        }
        self.portail.repondre(s, &r.jeton, r.publications()).map_err(|e| e.to_string())?;
        // Usage unique tenu au-delà d'un redémarrage : jeton et séquence
        // persistés AVANT d'écrire la moindre publication.
        let persiste = self
            .persister_jeton(s, r.jeton.empreinte())
            .map_err(|e| e.to_string())
            .and_then(|()| self.persister_courant(s));
        if let Err(e) = persiste {
            self.jeter_publications(s);
            return Err(format!("état non persisté, réponse retenue : {e}"));
        }
        Ok(())
    }

    fn publier_h1(&mut self, s: &IdService, octets: &[u8]) -> Result<(), String> {
        let p = PublicationH1::decoder(octets).map_err(|e| e.to_string())?;
        if &p.message.service != s {
            return Err(ErreurPortail::VoieNonDeclaree.to_string());
        }
        self.portail.publier_h1(s, p.publication()).map_err(|e| e.to_string())?;
        if let Err(e) = self.persister_courant(s) {
            self.jeter_publications(s);
            return Err(format!("état non persisté, publication retenue : {e}"));
        }
        Ok(())
    }

    fn persister_courant(&mut self, s: &IdService) -> Result<(), String> {
        let rotation = self.connus.get(s).map_or(0, |(_, d)| d.rotation);
        self.persister_cle(s, rotation).map_err(|e| e.to_string())
    }

    fn jeter_publications(&self, s: &IdService) {
        while let Ok(Some(_)) = self.portail.lire_publication(s) {}
    }

    /// Écrit dans la file de publication les trames signées en attente.
    fn emettre(&mut self, s: &IdService) -> u64 {
        let mut n = 0;
        while let Ok(Some(trame)) = self.portail.lire_publication(s) {
            match self.publication.ecrire(s, EXT_MSCE, &trame) {
                Ok(nom) => {
                    self.emis.entry(s.clone()).or_default().push(nom);
                    n += 1;
                }
                Err(e) => journal(SORTIE, &format!("voie {s} : publication non écrite : {e}")),
            }
        }
        n
    }
}

// ---------------------------------------------------------------- ligne de commande

/// Ligne de commande des binaires `portail-entree` et `portail-sortie`.
///
/// ```text
/// portail-entree --voies D --etat D --depot D --admises D --sas F [--interdit D]…
///                [--intervalle-ms N] [--une-passe | --verifier-cloisonnement]
/// portail-sortie --voies D --etat D --reponses D --publication D [--interdit D]…
///                [--intervalle-ms N] [--une-passe | --verifier-cloisonnement]
/// ```
///
/// Codes de sortie : 0 succès (ou cloisonnement vérifié), 1 cloisonnement
/// rompu (`--verifier-cloisonnement`), 2 arguments refusés, 3 refus de
/// démarrer (permissions, cloisonnement), 4 état propre inaccessible.
pub mod cli {
    use std::collections::BTreeMap;
    use std::path::PathBuf;
    use std::time::Duration;

    use super::{journal, CheminsEntree, CheminsSortie, DemonEntree, DemonSortie, ErreurDemon};
    use crate::cdr::RefusParDefaut;
    use crate::sas::SasProcessus;

    /// Succès.
    pub const SORTIE_OK: i32 = 0;
    /// Cloisonnement rompu (mode `--verifier-cloisonnement`).
    pub const SORTIE_NON_CLOISONNE: i32 = 1;
    /// Arguments refusés.
    pub const SORTIE_ARGUMENTS: i32 = 2;
    /// Refus de démarrer : permissions ou cloisonnement.
    pub const SORTIE_REFUS: i32 = 3;
    /// État propre inaccessible.
    pub const SORTIE_ETAT: i32 = 4;

    struct Args {
        chemins: BTreeMap<String, PathBuf>,
        interdits: Vec<PathBuf>,
        intervalle: Duration,
        une_passe: bool,
        verifier: bool,
    }

    fn analyser(arguments: &[String], cles: &[&str]) -> Option<Args> {
        let mut a = Args {
            chemins: BTreeMap::new(),
            interdits: Vec::new(),
            intervalle: Duration::from_millis(200),
            une_passe: false,
            verifier: false,
        };
        let mut it = arguments.iter();
        while let Some(x) = it.next() {
            match x.as_str() {
                "--une-passe" => a.une_passe = true,
                "--verifier-cloisonnement" => a.verifier = true,
                "--intervalle-ms" => {
                    let ms: u64 = it.next()?.parse().ok()?;
                    if !(10..=60_000).contains(&ms) {
                        return None;
                    }
                    a.intervalle = Duration::from_millis(ms);
                }
                "--interdit" => a.interdits.push(absolu(it.next()?)?),
                option => {
                    let cle = option.strip_prefix("--")?;
                    if !cles.contains(&cle) || a.chemins.contains_key(cle) {
                        return None;
                    }
                    a.chemins.insert(cle.to_string(), absolu(it.next()?)?);
                }
            }
        }
        if cles.iter().any(|c| !a.chemins.contains_key(*c)) || a.interdits.is_empty() || (a.une_passe && a.verifier) {
            return None;
        }
        Some(a)
    }

    fn absolu(s: &str) -> Option<PathBuf> {
        let p = PathBuf::from(s);
        p.is_absolute().then_some(p)
    }

    fn chemin(a: &Args, cle: &str) -> PathBuf {
        a.chemins.get(cle).cloned().unwrap_or_default()
    }

    fn code_erreur(processus: &str, e: &ErreurDemon) -> i32 {
        journal(processus, &format!("refus de démarrer : {e}"));
        match e {
            ErreurDemon::Etat(_) => SORTIE_ETAT,
            _ => SORTIE_REFUS,
        }
    }

    /// Point d'entrée de `portail-entree`.
    pub fn executer_entree(arguments: &[String]) -> i32 {
        const P: &str = "portail-entree";
        let Some(a) = analyser(arguments, &["voies", "etat", "depot", "admises", "sas"]) else {
            journal(P, "arguments refusés");
            return SORTIE_ARGUMENTS;
        };
        if a.verifier {
            return verifier(P, &a.interdits);
        }
        let chemins = CheminsEntree {
            voies: chemin(&a, "voies"),
            etat: chemin(&a, "etat"),
            depot: chemin(&a, "depot"),
            admises: chemin(&a, "admises"),
            interdits: a.interdits.clone(),
        };
        let sas = SasProcessus::new(chemin(&a, "sas"));
        let mut demon = match DemonEntree::new(chemins, Box::new(sas), Box::new(RefusParDefaut)) {
            Ok(d) => d,
            Err(e) => return code_erreur(P, &e),
        };
        if let Err(e) = demon.verifier_au_demarrage() {
            return code_erreur(P, &e);
        }
        journal(P, "démarré : relève seule, aucune capacité de publier");
        loop {
            demon.une_passe();
            if a.une_passe {
                return SORTIE_OK;
            }
            std::thread::sleep(a.intervalle);
        }
    }

    /// Point d'entrée de `portail-sortie`.
    pub fn executer_sortie(arguments: &[String]) -> i32 {
        const P: &str = "portail-sortie";
        let Some(a) = analyser(arguments, &["voies", "etat", "reponses", "publication"]) else {
            journal(P, "arguments refusés");
            return SORTIE_ARGUMENTS;
        };
        if a.verifier {
            return verifier(P, &a.interdits);
        }
        let chemins = CheminsSortie {
            voies: chemin(&a, "voies"),
            etat: chemin(&a, "etat"),
            reponses: chemin(&a, "reponses"),
            publication: chemin(&a, "publication"),
            interdits: a.interdits.clone(),
        };
        let mut demon = match DemonSortie::new(chemins, Box::new(RefusParDefaut)) {
            Ok(d) => d,
            Err(e) => return code_erreur(P, &e),
        };
        if let Err(e) = demon.verifier_au_demarrage() {
            return code_erreur(P, &e);
        }
        journal(P, "démarré : réponses à jeton et publications H1 seules, aucune capacité de relever");
        loop {
            demon.une_passe();
            if a.une_passe {
                return SORTIE_OK;
            }
            std::thread::sleep(a.intervalle);
        }
    }

    fn verifier(processus: &str, interdits: &[PathBuf]) -> i32 {
        match super::verifier_cloisonnement(interdits) {
            Ok(()) => {
                journal(processus, "cloisonnement vérifié : aucune écriture possible dans les files interdites");
                SORTIE_OK
            }
            Err(e) => {
                journal(processus, &e.to_string());
                SORTIE_NON_CLOISONNE
            }
        }
    }
}
