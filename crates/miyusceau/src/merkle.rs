//! @id     miyusceau_merkle
//! @do     hacher_prouver_arbre_merkle
//! @role   securite
//! @layer  toolkit
//! @human  miyusceau — module « merkle » (arbre de Merkle RFC 9162).

//! Arbre de Merkle du registre (modèle Certificate Transparency, RFC 9162 §2.1),
//! décision 5 du lot 2 :
//!
//! - feuille : `SHA-256(0x00 || entrée)` ; nœud : `SHA-256(0x01 || gauche || droite)` ;
//! - arbre vide : `SHA-256("")` ;
//! - découpage : `k` = plus grande puissance de 2 strictement inférieure à `n`.
//!
//! Preuves d'inclusion (§2.1.3) et de cohérence (§2.1.4) : génération
//! (outillage du registre, tests) et vérification (Central, hors ligne).
//!
//! Encodage binaire des preuves (big-endian, format propre à MiyuSceau,
//! décodage strict, taille exacte) :
//!
//! ```text
//! inclusion : index (8) | taille (8)  | nombre (1, ≤ 64) | nombre × empreinte (32)
//! cohérence : taille1 (8) | taille2 (8) | nombre (1, ≤ 64) | nombre × empreinte (32)
//! ```

use sha2::{Digest, Sha256};

use crate::erreurs::ErreurRegistre;

/// Empreinte SHA-256 (feuille, nœud, racine).
pub type Empreinte = [u8; 32];

/// Longueur maximale d'un chemin de preuve (arbre d'au plus 2^64 feuilles).
pub const MAX_CHEMIN: usize = 64;

/// Racine de l'arbre vide : `SHA-256("")`.
pub fn racine_vide() -> Empreinte {
    Sha256::digest([]).into()
}

/// Empreinte d'une feuille : `SHA-256(0x00 || entrée)`.
pub fn hacher_feuille(entree: &[u8]) -> Empreinte {
    let mut h = Sha256::new();
    h.update([0x00]);
    h.update(entree);
    h.finalize().into()
}

/// Empreinte d'un nœud interne : `SHA-256(0x01 || gauche || droite)`.
pub fn hacher_noeud(gauche: &Empreinte, droite: &Empreinte) -> Empreinte {
    let mut h = Sha256::new();
    h.update([0x01]);
    h.update(gauche);
    h.update(droite);
    h.finalize().into()
}

/// Plus grande puissance de 2 strictement inférieure à `n` (`n ≥ 2`).
fn decoupe(n: usize) -> usize {
    debug_assert!(n >= 2);
    let mut k = 1usize;
    while k << 1 < n {
        k <<= 1;
    }
    k
}

/// Racine (MTH) d'une liste d'empreintes de feuilles.
pub fn racine(feuilles: &[Empreinte]) -> Empreinte {
    match feuilles.len() {
        0 => racine_vide(),
        1 => feuilles[0],
        n => {
            let k = decoupe(n);
            hacher_noeud(&racine(&feuilles[..k]), &racine(&feuilles[k..]))
        }
    }
}

fn chemin_inclusion(m: usize, feuilles: &[Empreinte], sortie: &mut Vec<Empreinte>) {
    let n = feuilles.len();
    if n <= 1 {
        return;
    }
    let k = decoupe(n);
    if m < k {
        chemin_inclusion(m, &feuilles[..k], sortie);
        sortie.push(racine(&feuilles[k..]));
    } else {
        chemin_inclusion(m - k, &feuilles[k..], sortie);
        sortie.push(racine(&feuilles[..k]));
    }
}

fn sous_preuve(m: usize, feuilles: &[Empreinte], complet: bool, sortie: &mut Vec<Empreinte>) {
    let n = feuilles.len();
    if m == n {
        if !complet {
            sortie.push(racine(feuilles));
        }
        return;
    }
    let k = decoupe(n);
    if m <= k {
        sous_preuve(m, &feuilles[..k], complet, sortie);
        sortie.push(racine(&feuilles[k..]));
    } else {
        sous_preuve(m - k, &feuilles[k..], false, sortie);
        sortie.push(racine(&feuilles[..k]));
    }
}

/// Lecture stricte de l'encodage commun des preuves.
fn decoder_preuve(octets: &[u8]) -> Result<(u64, u64, Vec<Empreinte>), ErreurRegistre> {
    if octets.len() < 17 {
        return Err(ErreurRegistre::PreuveInvalide);
    }
    let a = u64::from_be_bytes(octets[0..8].try_into().map_err(|_| ErreurRegistre::PreuveInvalide)?);
    let b = u64::from_be_bytes(octets[8..16].try_into().map_err(|_| ErreurRegistre::PreuveInvalide)?);
    let nombre = usize::from(octets[16]);
    if nombre > MAX_CHEMIN || octets.len() != 17 + nombre * 32 {
        return Err(ErreurRegistre::PreuveInvalide);
    }
    let mut chemin = Vec::with_capacity(nombre);
    for i in 0..nombre {
        let mut e = [0u8; 32];
        e.copy_from_slice(&octets[17 + 32 * i..17 + 32 * (i + 1)]);
        chemin.push(e);
    }
    Ok((a, b, chemin))
}

/// Encodage commun, symétrique de [`decoder_preuve`] : un chemin de plus de
/// [`MAX_CHEMIN`] empreintes (refusé au décodage, et tronqué par l'octet de
/// longueur au-delà de 255) est refusé.
fn encoder_preuve(a: u64, b: u64, chemin: &[Empreinte]) -> Result<Vec<u8>, ErreurRegistre> {
    if chemin.len() > MAX_CHEMIN {
        return Err(ErreurRegistre::PreuveInvalide);
    }
    let mut sortie = Vec::with_capacity(17 + chemin.len() * 32);
    sortie.extend_from_slice(&a.to_be_bytes());
    sortie.extend_from_slice(&b.to_be_bytes());
    sortie.push(chemin.len() as u8);
    for e in chemin {
        sortie.extend_from_slice(e);
    }
    Ok(sortie)
}

/// Preuve d'inclusion d'une feuille dans un arbre de taille donnée.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreuveInclusion {
    /// Index de la feuille (0 ≤ index < taille).
    pub index: u64,
    /// Taille de l'arbre (nombre de feuilles).
    pub taille: u64,
    /// Chemin d'audit, de la feuille vers la racine.
    pub chemin: Vec<Empreinte>,
}

impl PreuveInclusion {
    /// Construit la preuve d'inclusion de la feuille `index` (`None` si hors bornes).
    pub fn generer(feuilles: &[Empreinte], index: usize) -> Option<Self> {
        if index >= feuilles.len() {
            return None;
        }
        let mut chemin = Vec::new();
        chemin_inclusion(index, feuilles, &mut chemin);
        Some(Self { index: index as u64, taille: feuilles.len() as u64, chemin })
    }

    /// Décodage strict (taille exacte, chemin ≤ [`MAX_CHEMIN`]).
    pub fn decoder(octets: &[u8]) -> Result<Self, ErreurRegistre> {
        let (index, taille, chemin) = decoder_preuve(octets)?;
        Ok(Self { index, taille, chemin })
    }

    /// Encodage binaire (chemin d'au plus [`MAX_CHEMIN`] empreintes, sinon
    /// [`ErreurRegistre::PreuveInvalide`]) : `decoder(&p.encoder()?) == Ok(p)`.
    pub fn encoder(&self) -> Result<Vec<u8>, ErreurRegistre> {
        encoder_preuve(self.index, self.taille, &self.chemin)
    }

    /// Vérifie que `feuille` (empreinte, voir [`hacher_feuille`]) est à
    /// `index` dans l'arbre de racine `racine` (RFC 9162 §2.1.3.2).
    pub fn verifier(&self, feuille: &Empreinte, racine: &Empreinte) -> Result<(), ErreurRegistre> {
        if self.index >= self.taille || self.chemin.len() > MAX_CHEMIN {
            return Err(ErreurRegistre::PreuveInvalide);
        }
        let mut f_n = self.index;
        let mut s_n = self.taille - 1;
        let mut r = *feuille;
        for p in &self.chemin {
            if s_n == 0 {
                return Err(ErreurRegistre::PreuveInvalide);
            }
            if f_n & 1 == 1 || f_n == s_n {
                r = hacher_noeud(p, &r);
                while f_n & 1 == 0 && f_n != 0 {
                    f_n >>= 1;
                    s_n >>= 1;
                }
            } else {
                r = hacher_noeud(&r, p);
            }
            f_n >>= 1;
            s_n >>= 1;
        }
        if s_n == 0 && &r == racine {
            Ok(())
        } else {
            Err(ErreurRegistre::PreuveInvalide)
        }
    }
}

/// Preuve de cohérence entre deux tailles d'un même registre (append-only).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreuveCoherence {
    /// Ancienne taille.
    pub taille1: u64,
    /// Nouvelle taille.
    pub taille2: u64,
    /// Chemin de cohérence.
    pub chemin: Vec<Empreinte>,
}

impl PreuveCoherence {
    /// Construit la preuve de cohérence entre les `ancienne_taille` premières
    /// feuilles et l'arbre complet (`None` si `ancienne_taille` hors de `1..=n`).
    pub fn generer(feuilles: &[Empreinte], ancienne_taille: usize) -> Option<Self> {
        if ancienne_taille == 0 || ancienne_taille > feuilles.len() {
            return None;
        }
        let mut chemin = Vec::new();
        sous_preuve(ancienne_taille, feuilles, true, &mut chemin);
        Some(Self { taille1: ancienne_taille as u64, taille2: feuilles.len() as u64, chemin })
    }

    /// Décodage strict (taille exacte, chemin ≤ [`MAX_CHEMIN`]).
    pub fn decoder(octets: &[u8]) -> Result<Self, ErreurRegistre> {
        let (taille1, taille2, chemin) = decoder_preuve(octets)?;
        Ok(Self { taille1, taille2, chemin })
    }

    /// Encodage binaire (chemin d'au plus [`MAX_CHEMIN`] empreintes, sinon
    /// [`ErreurRegistre::PreuveInvalide`]) : `decoder(&p.encoder()?) == Ok(p)`.
    pub fn encoder(&self) -> Result<Vec<u8>, ErreurRegistre> {
        encoder_preuve(self.taille1, self.taille2, &self.chemin)
    }

    /// Vérifie que l'arbre de racine `racine2` (taille `taille2`) prolonge
    /// celui de racine `racine1` (taille `taille1`) sans réécrire l'histoire
    /// (RFC 9162 §2.1.4.2).
    ///
    /// Tailles égales : chemin vide et racines égales exigés. `taille1 = 0` :
    /// refus (cas trivial, à traiter par l'appelant).
    pub fn verifier(&self, racine1: &Empreinte, racine2: &Empreinte) -> Result<(), ErreurRegistre> {
        let refus = Err(ErreurRegistre::PreuveInvalide);
        if self.taille1 == 0 || self.taille1 > self.taille2 || self.chemin.len() > MAX_CHEMIN {
            return refus;
        }
        if self.taille1 == self.taille2 {
            return if self.chemin.is_empty() && racine1 == racine2 { Ok(()) } else { refus };
        }
        if self.chemin.is_empty() {
            return refus;
        }
        // 1. Ancienne taille puissance de 2 : la racine ancienne ouvre le chemin.
        let mut chemin: Vec<Empreinte> = Vec::with_capacity(self.chemin.len() + 1);
        if self.taille1.is_power_of_two() {
            chemin.push(*racine1);
        }
        chemin.extend_from_slice(&self.chemin);
        // 2-3.
        let mut f_n = self.taille1 - 1;
        let mut s_n = self.taille2 - 1;
        while f_n & 1 == 1 {
            f_n >>= 1;
            s_n >>= 1;
        }
        // 4-5.
        let mut fr = chemin[0];
        let mut sr = chemin[0];
        for c in &chemin[1..] {
            if s_n == 0 {
                return refus;
            }
            if f_n & 1 == 1 || f_n == s_n {
                fr = hacher_noeud(c, &fr);
                sr = hacher_noeud(c, &sr);
                while f_n & 1 == 0 && f_n != 0 {
                    f_n >>= 1;
                    s_n >>= 1;
                }
            } else {
                sr = hacher_noeud(&sr, c);
            }
            f_n >>= 1;
            s_n >>= 1;
        }
        // 6.
        if &fr == racine1 && &sr == racine2 && s_n == 0 {
            Ok(())
        } else {
            refus
        }
    }
}
