//! @id     miyuregistre_stockage
//! @do     stocker_journal_append_only
//! @role   securite
//! @layer  toolkit
//! @human  miyuregistre — module « stockage » (journal append-only, fsync, récupération de fin tronquée).

//! Stockage append-only des enregistrements d'un registre (entrées, têtes).
//!
//! Interface indépendante du support ([`Stockage`]) ; deux implémentations :
//! [`StockageMemoire`] (tests, miroirs éphémères) et [`StockageFichier`].
//!
//! # Format fichier (big-endian)
//!
//! ```text
//! entête         : "MRGJ" | version 0x01
//! enregistrement : longueur (4) | octets (longueur) | contrôle (8)
//! contrôle       : SHA-256(longueur || octets)[0..8]
//! ```
//!
//! # Sûreté en cas de coupure
//!
//! - Ajout = une seule écriture en fin de fichier puis `fsync` (`sync_data`) ;
//!   l'ajout n'est confirmé qu'après le `fsync`.
//! - Création : entête écrite puis `fsync` du fichier **et** du répertoire (Unix).
//! - Ouverture : le dernier enregistrement incomplet ou dont le contrôle est
//!   faux et qui s'étend jusqu'à la fin du fichier est une **fin tronquée**
//!   (écriture interrompue) : il est retiré (`set_len` + `fsync`). Un
//!   enregistrement altéré suivi d'autres octets est une **corruption** : refus,
//!   aucune réparation automatique.
//! - Échec d'écriture : la fin est tronquée à la dernière longueur confirmée ;
//!   si cela échoue aussi, le stockage est **empoisonné** (refus jusqu'à réouverture).
//!
//! Un seul écrivain par fichier (l'appelant garantit l'exclusivité, `&mut`).

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::erreurs::ErreurStockage;

/// Taille maximale d'un enregistrement (1 Mio : un sceau fait au plus 92 812 o).
pub const TAILLE_MAX_ENREGISTREMENT: usize = 1 << 20;

const MAGIC: &[u8; 4] = b"MRGJ";
const VERSION: u8 = 0x01;
const TAILLE_ENTETE: u64 = 5;
const TAILLE_CONTROLE: usize = 8;

/// Journal append-only d'enregistrements opaques, indexés à partir de 0.
pub trait Stockage {
    /// Ajoute un enregistrement de façon durable ; renvoie son index.
    fn ajouter(&mut self, octets: &[u8]) -> Result<u64, ErreurStockage>;
    /// Lit l'enregistrement `index`.
    fn lire(&self, index: u64) -> Result<Vec<u8>, ErreurStockage>;
    /// Nombre d'enregistrements.
    fn taille(&self) -> u64;
    /// Retire les enregistrements à partir de `taille` (récupération d'un
    /// miroir : histoire non confirmée par une tête). Sans effet si `taille`
    /// ≥ taille actuelle.
    fn tronquer(&mut self, taille: u64) -> Result<(), ErreurStockage>;
}

fn valider_taille(octets: &[u8]) -> Result<(), ErreurStockage> {
    if octets.is_empty() || octets.len() > TAILLE_MAX_ENREGISTREMENT {
        return Err(ErreurStockage::TailleInvalide);
    }
    Ok(())
}

/// Stockage en mémoire (non durable).
#[derive(Debug, Default, Clone)]
pub struct StockageMemoire {
    enregistrements: Vec<Vec<u8>>,
}

impl StockageMemoire {
    /// Stockage vide.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Stockage for StockageMemoire {
    fn ajouter(&mut self, octets: &[u8]) -> Result<u64, ErreurStockage> {
        valider_taille(octets)?;
        self.enregistrements.push(octets.to_vec());
        Ok(self.enregistrements.len() as u64 - 1)
    }

    fn lire(&self, index: u64) -> Result<Vec<u8>, ErreurStockage> {
        usize::try_from(index)
            .ok()
            .and_then(|i| self.enregistrements.get(i))
            .cloned()
            .ok_or(ErreurStockage::HorsBornes(index))
    }

    fn taille(&self) -> u64 {
        self.enregistrements.len() as u64
    }

    fn tronquer(&mut self, taille: u64) -> Result<(), ErreurStockage> {
        let t = usize::try_from(taille).unwrap_or(usize::MAX);
        self.enregistrements.truncate(t);
        Ok(())
    }
}

fn controle(longueur: [u8; 4], octets: &[u8]) -> [u8; TAILLE_CONTROLE] {
    let mut h = Sha256::new();
    h.update(longueur);
    h.update(octets);
    let d = h.finalize();
    let mut c = [0u8; TAILLE_CONTROLE];
    c.copy_from_slice(&d[..TAILLE_CONTROLE]);
    c
}

/// Entête du format fichier (`"MRGJ"` ‖ version).
pub(crate) fn entete_fichier() -> [u8; TAILLE_ENTETE as usize] {
    [MAGIC[0], MAGIC[1], MAGIC[2], MAGIC[3], VERSION]
}

/// Enregistrement au format fichier : longueur (4) ‖ octets ‖ contrôle (8).
pub(crate) fn encoder_enregistrement(octets: &[u8]) -> Result<Vec<u8>, ErreurStockage> {
    valider_taille(octets)?;
    let longueur = u32::try_from(octets.len()).map_err(|_| ErreurStockage::TailleInvalide)?.to_be_bytes();
    let mut enregistrement = Vec::with_capacity(4 + octets.len() + TAILLE_CONTROLE);
    enregistrement.extend_from_slice(&longueur);
    enregistrement.extend_from_slice(octets);
    enregistrement.extend_from_slice(&controle(longueur, octets));
    Ok(enregistrement)
}

/// Stockage fichier append-only, durable (`fsync` à chaque ajout).
#[derive(Debug)]
pub struct StockageFichier {
    chemin: PathBuf,
    fichier: File,
    /// Position (octets) du début de chaque enregistrement.
    positions: Vec<u64>,
    /// Longueur confirmée du fichier.
    fin: u64,
    empoisonne: bool,
    /// Octets retirés à l'ouverture (fin tronquée récupérée).
    recupere: u64,
}

#[cfg(unix)]
fn synchroniser_repertoire(chemin: &Path) -> Result<(), ErreurStockage> {
    if let Some(parent) = chemin.parent() {
        let parent = if parent.as_os_str().is_empty() { Path::new(".") } else { parent };
        File::open(parent)?.sync_all()?;
    }
    Ok(())
}

#[cfg(not(unix))]
fn synchroniser_repertoire(_chemin: &Path) -> Result<(), ErreurStockage> {
    Ok(())
}

impl StockageFichier {
    /// Ouvre (ou crée) le journal `chemin` et récupère une fin tronquée.
    pub fn ouvrir(chemin: impl AsRef<Path>) -> Result<Self, ErreurStockage> {
        let chemin = chemin.as_ref().to_path_buf();
        let mut fichier = OpenOptions::new().read(true).write(true).create(true).truncate(false).open(&chemin)?;
        let mut contenu = Vec::new();
        fichier.read_to_end(&mut contenu)?;

        // Entête : absente ou partielle (création interrompue) ⇒ réécrite.
        if contenu.len() < TAILLE_ENTETE as usize {
            let mut entete = MAGIC.to_vec();
            entete.push(VERSION);
            if !entete.starts_with(&contenu) {
                return Err(ErreurStockage::EnteteInvalide);
            }
            fichier.set_len(0)?;
            fichier.seek(SeekFrom::Start(0))?;
            fichier.write_all(&entete)?;
            fichier.sync_all()?;
            synchroniser_repertoire(&chemin)?;
            return Ok(Self {
                chemin,
                fichier,
                positions: Vec::new(),
                fin: TAILLE_ENTETE,
                empoisonne: false,
                recupere: contenu.len() as u64,
            });
        }
        if &contenu[0..4] != MAGIC || contenu[4] != VERSION {
            return Err(ErreurStockage::EnteteInvalide);
        }

        let mut positions = Vec::new();
        let mut pos = TAILLE_ENTETE as usize;
        let total = contenu.len();
        while pos < total {
            let index = positions.len() as u64;
            let Some(entete) = contenu.get(pos..pos + 4) else {
                break; // longueur partielle : fin tronquée
            };
            let longueur_brute: [u8; 4] = entete.try_into().map_err(|_| ErreurStockage::Corrompu { index })?;
            let longueur = u32::from_be_bytes(longueur_brute) as usize;
            if longueur == 0 || longueur > TAILLE_MAX_ENREGISTREMENT {
                return Err(ErreurStockage::Corrompu { index });
            }
            let fin_enr = pos + 4 + longueur + TAILLE_CONTROLE;
            if fin_enr > total {
                break; // enregistrement incomplet : fin tronquée
            }
            let octets = &contenu[pos + 4..pos + 4 + longueur];
            if contenu[pos + 4 + longueur..fin_enr] != controle(longueur_brute, octets) {
                if fin_enr == total {
                    break; // dernière écriture déchirée : fin tronquée
                }
                return Err(ErreurStockage::Corrompu { index });
            }
            positions.push(pos as u64);
            pos = fin_enr;
        }
        let fin = positions.last().map_or(TAILLE_ENTETE, |&p| {
            let p = p as usize;
            let longueur = u32::from_be_bytes([contenu[p], contenu[p + 1], contenu[p + 2], contenu[p + 3]]) as usize;
            (p + 4 + longueur + TAILLE_CONTROLE) as u64
        });
        let recupere = total as u64 - fin;
        if recupere > 0 {
            fichier.set_len(fin)?;
            fichier.sync_all()?;
        }
        Ok(Self { chemin, fichier, positions, fin, empoisonne: false, recupere })
    }

    /// Chemin du fichier.
    pub fn chemin(&self) -> &Path {
        &self.chemin
    }

    /// Octets retirés à l'ouverture (fin tronquée récupérée), 0 si aucun.
    pub fn octets_recuperes(&self) -> u64 {
        self.recupere
    }

    fn exiger_sain(&self) -> Result<(), ErreurStockage> {
        if self.empoisonne {
            return Err(ErreurStockage::Empoisonne);
        }
        Ok(())
    }
}

impl Stockage for StockageFichier {
    fn ajouter(&mut self, octets: &[u8]) -> Result<u64, ErreurStockage> {
        self.exiger_sain()?;
        let enregistrement = encoder_enregistrement(octets)?;

        let ecrire = |f: &mut File, fin: u64| -> std::io::Result<()> {
            f.seek(SeekFrom::Start(fin))?;
            f.write_all(&enregistrement)?;
            f.sync_data()
        };
        if let Err(e) = ecrire(&mut self.fichier, self.fin) {
            // Annule la fin partielle ; à défaut, refus jusqu'à réouverture.
            if self.fichier.set_len(self.fin).and_then(|()| self.fichier.sync_all()).is_err() {
                self.empoisonne = true;
            }
            return Err(e.into());
        }
        self.positions.push(self.fin);
        self.fin += enregistrement.len() as u64;
        Ok(self.positions.len() as u64 - 1)
    }

    fn lire(&self, index: u64) -> Result<Vec<u8>, ErreurStockage> {
        self.exiger_sain()?;
        let i = usize::try_from(index).map_err(|_| ErreurStockage::HorsBornes(index))?;
        let &pos = self.positions.get(i).ok_or(ErreurStockage::HorsBornes(index))?;
        let mut f = &self.fichier;
        f.seek(SeekFrom::Start(pos))?;
        let mut longueur_brute = [0u8; 4];
        f.read_exact(&mut longueur_brute)?;
        let longueur = u32::from_be_bytes(longueur_brute) as usize;
        if longueur == 0 || longueur > TAILLE_MAX_ENREGISTREMENT {
            return Err(ErreurStockage::Corrompu { index });
        }
        let mut octets = vec![0u8; longueur];
        f.read_exact(&mut octets)?;
        let mut c = [0u8; TAILLE_CONTROLE];
        f.read_exact(&mut c)?;
        // Défense en profondeur : altération sur disque après l'ouverture.
        if c != controle(longueur_brute, &octets) {
            return Err(ErreurStockage::Corrompu { index });
        }
        Ok(octets)
    }

    fn taille(&self) -> u64 {
        self.positions.len() as u64
    }

    fn tronquer(&mut self, taille: u64) -> Result<(), ErreurStockage> {
        self.exiger_sain()?;
        let t = usize::try_from(taille).unwrap_or(usize::MAX);
        if t >= self.positions.len() {
            return Ok(());
        }
        let nouvelle_fin = self.positions[t];
        if self.fichier.set_len(nouvelle_fin).and_then(|()| self.fichier.sync_all()).is_err() {
            self.empoisonne = true;
            return Err(ErreurStockage::Empoisonne);
        }
        self.positions.truncate(t);
        self.fin = nouvelle_fin;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    fn chemin_temporaire(nom: &str) -> PathBuf {
        static N: AtomicU64 = AtomicU64::new(0);
        let n = N.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("miyuregistre-{}-{nom}-{n}.mrgj", std::process::id()))
    }

    #[test]
    fn memoire_ajout_lecture_troncature() {
        let mut s = StockageMemoire::new();
        assert_eq!(s.ajouter(b"a").unwrap(), 0);
        assert_eq!(s.ajouter(b"bb").unwrap(), 1);
        assert_eq!(s.lire(1).unwrap(), b"bb");
        assert!(matches!(s.lire(2), Err(ErreurStockage::HorsBornes(2))));
        assert!(matches!(s.ajouter(b""), Err(ErreurStockage::TailleInvalide)));
        s.tronquer(1).unwrap();
        assert_eq!(s.taille(), 1);
    }

    #[test]
    fn fichier_persistance_et_reouverture() {
        let p = chemin_temporaire("persistance");
        {
            let mut s = StockageFichier::ouvrir(&p).unwrap();
            s.ajouter(b"un").unwrap();
            s.ajouter(b"deux").unwrap();
        }
        let s = StockageFichier::ouvrir(&p).unwrap();
        assert_eq!(s.taille(), 2);
        assert_eq!(s.lire(0).unwrap(), b"un");
        assert_eq!(s.lire(1).unwrap(), b"deux");
        assert_eq!(s.octets_recuperes(), 0);
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn fichier_fin_tronquee_recuperee() {
        let p = chemin_temporaire("tronquee");
        {
            let mut s = StockageFichier::ouvrir(&p).unwrap();
            s.ajouter(b"intact").unwrap();
            s.ajouter(b"coupe-en-plein-vol").unwrap();
        }
        // Coupure pendant la seconde écriture : il en manque une partie.
        let longueur = std::fs::metadata(&p).unwrap().len();
        OpenOptions::new().write(true).open(&p).unwrap().set_len(longueur - 5).unwrap();
        let mut s = StockageFichier::ouvrir(&p).unwrap();
        assert_eq!(s.taille(), 1);
        assert!(s.octets_recuperes() > 0);
        assert_eq!(s.lire(0).unwrap(), b"intact");
        // Le journal reste utilisable après récupération.
        assert_eq!(s.ajouter(b"suite").unwrap(), 1);
        drop(s);
        let s = StockageFichier::ouvrir(&p).unwrap();
        assert_eq!(s.lire(1).unwrap(), b"suite");
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn fichier_derniere_ecriture_dechiree() {
        let p = chemin_temporaire("dechiree");
        {
            let mut s = StockageFichier::ouvrir(&p).unwrap();
            s.ajouter(b"intact").unwrap();
            s.ajouter(b"dechire").unwrap();
        }
        // Longueur complète mais contenu déchiré (dernier octet du contrôle faux).
        let mut octets = std::fs::read(&p).unwrap();
        let n = octets.len();
        octets[n - 1] ^= 0xFF;
        std::fs::write(&p, &octets).unwrap();
        let s = StockageFichier::ouvrir(&p).unwrap();
        assert_eq!(s.taille(), 1);
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn fichier_corruption_au_milieu_refusee() {
        let p = chemin_temporaire("corrompu");
        {
            let mut s = StockageFichier::ouvrir(&p).unwrap();
            s.ajouter(b"premier").unwrap();
            s.ajouter(b"second").unwrap();
        }
        let mut octets = std::fs::read(&p).unwrap();
        octets[5 + 4] ^= 0x01; // premier octet du premier enregistrement
        std::fs::write(&p, &octets).unwrap();
        assert!(matches!(StockageFichier::ouvrir(&p), Err(ErreurStockage::Corrompu { index: 0 })));
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn fichier_entete_etrangere_refusee() {
        let p = chemin_temporaire("entete");
        std::fs::write(&p, b"PASUNJOURNAL").unwrap();
        assert!(matches!(StockageFichier::ouvrir(&p), Err(ErreurStockage::EnteteInvalide)));
        std::fs::write(&p, b"MR").unwrap(); // création interrompue
        let s = StockageFichier::ouvrir(&p).unwrap();
        assert_eq!(s.taille(), 0);
        let _ = std::fs::remove_file(&p);
    }

    #[test]
    fn fichier_troncature_explicite() {
        let p = chemin_temporaire("troncature");
        let mut s = StockageFichier::ouvrir(&p).unwrap();
        for i in 0..4u8 {
            s.ajouter(&[i + 1]).unwrap();
        }
        s.tronquer(2).unwrap();
        assert_eq!(s.taille(), 2);
        drop(s);
        let s = StockageFichier::ouvrir(&p).unwrap();
        assert_eq!(s.taille(), 2);
        assert_eq!(s.lire(1).unwrap(), vec![2]);
        let _ = std::fs::remove_file(&p);
    }
}
