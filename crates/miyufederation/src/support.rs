//! @id     miyufederation_support
//! @do     abstraire_support_etat_federal
//! @role   securite
//! @layer  toolkit
//! @human  miyufederation — module « support » (trait de stockage de l'état fédéral scellé MFED : fichier, mémoire de test, KindMother ; découpage en morceaux vérifié).

//! Supports de l'état fédéral scellé (`MFED`, [`crate::federation`]).
//!
//! Un support ne stocke que des **octets opaques** : l'état scellé complet
//! (titres signés d'origine, HMAC du secret local, ancre du registre local).
//! Il ne décide rien ; toute la revérification a lieu au chargement
//! ([`crate::FederationPersistante::charger`]). Exigences d'un support :
//!
//! - [`SupportEtat::ecrire`] est **atomique** : après un succès, une lecture
//!   rend exactement ces octets ; après un échec (ou une interruption), une
//!   lecture rend soit l'état précédent, soit le nouveau, jamais un mélange ;
//! - [`SupportEtat::lire`] rend `None` seulement si aucun état n'a jamais été
//!   écrit (absence) ; toute anomalie (illisible, incomplet) est une erreur.
//!
//! Implémentations : [`crate::StockageFederal`] (fichier `0600` d'un dossier
//! `0700` verrouillé), [`SupportMemoire`] (tests : redémarrage simulé,
//! altération, pannes d'écriture), et `SupportKindMother` (fonctionnalité
//! `kindmother` : KindMother, seul écrivain du COG, base dédiée).
//!
//! Le découpage en morceaux ([`decouper`], [`assembler_morceaux`]) sert aux
//! supports dont une écriture est bornée (trame KindMother de 4 Mio) : un état
//! jusqu'à 64 Mio est écrit en morceaux d'une **génération** nouvelle, puis
//! rendu visible d'un seul coup par un pointeur (taille, nombre de morceaux,
//! SHA-256) ; la relecture contrôle tout avant de rendre un octet.

use std::future::Future;
use std::sync::{Arc, Mutex};

use sha2::{Digest, Sha256};

use crate::persistance::{ErreurEtat, StockageFederal, TAILLE_MAX_FICHIER};

/// Stockage atomique des octets de l'état fédéral scellé (voir module).
pub trait SupportEtat: Send + Sync {
    /// Octets du dernier état écrit ; `None` : aucun état (premier usage).
    fn lire(&self) -> impl Future<Output = Result<Option<Vec<u8>>, ErreurEtat>> + Send;
    /// Remplace atomiquement l'état par `octets`.
    fn ecrire(&self, octets: &[u8]) -> impl Future<Output = Result<(), ErreurEtat>> + Send;
}

impl SupportEtat for StockageFederal {
    fn lire(&self) -> impl Future<Output = Result<Option<Vec<u8>>, ErreurEtat>> + Send {
        std::future::ready(StockageFederal::lire(self))
    }

    fn ecrire(&self, octets: &[u8]) -> impl Future<Output = Result<(), ErreurEtat>> + Send {
        std::future::ready(StockageFederal::ecrire(self, octets))
    }
}

#[derive(Debug, Default)]
struct Memoire {
    octets: Option<Vec<u8>>,
    ecritures: u64,
    pannes: u32,
}

/// Support en mémoire **pour les tests** : les clones partagent le même
/// contenu (un clone survit au « redémarrage » d'une
/// [`crate::FederationPersistante`]) ; altération et pannes injectables.
#[derive(Debug, Clone, Default)]
pub struct SupportMemoire {
    interne: Arc<Mutex<Memoire>>,
}

impl SupportMemoire {
    /// Support vide.
    pub fn new() -> Self {
        Self::default()
    }

    fn avec<T>(&self, f: impl FnOnce(&mut Memoire) -> T) -> T {
        // Un test qui panique en tenant le verrou ne doit pas masquer l'état.
        let mut garde = self.interne.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        f(&mut garde)
    }

    /// Copie des octets stockés.
    pub fn octets(&self) -> Option<Vec<u8>> {
        self.avec(|m| m.octets.clone())
    }

    /// Remplace les octets stockés (altération, rejeu d'un état ancien, suppression).
    pub fn remplacer(&self, octets: Option<Vec<u8>>) {
        self.avec(|m| m.octets = octets);
    }

    /// Les `n` prochaines écritures échouent sans rien changer.
    pub fn echouer_ecritures(&self, n: u32) {
        self.avec(|m| m.pannes = n);
    }

    /// Nombre d'écritures réussies.
    pub fn ecritures(&self) -> u64 {
        self.avec(|m| m.ecritures)
    }
}

impl SupportEtat for SupportMemoire {
    fn lire(&self) -> impl Future<Output = Result<Option<Vec<u8>>, ErreurEtat>> + Send {
        std::future::ready(Ok(self.octets()))
    }

    fn ecrire(&self, octets: &[u8]) -> impl Future<Output = Result<(), ErreurEtat>> + Send {
        let issue = self.avec(|m| {
            if m.pannes > 0 {
                m.pannes -= 1;
                return Err(ErreurEtat::Support("panne d'écriture simulée".to_string()));
            }
            m.octets = Some(octets.to_vec());
            m.ecritures += 1;
            Ok(())
        });
        std::future::ready(issue)
    }
}

// ── Découpage en morceaux (supports à écriture bornée) ─────────────────────

/// Taille d'un morceau (avant encodage hexadécimal).
pub const TAILLE_MORCEAU: usize = 256 * 1024;
/// Nombre maximal de morceaux d'un état (64 Mio et entête compris).
pub const MAX_MORCEAUX: usize = (TAILLE_MAX_FICHIER as usize).div_ceil(TAILLE_MORCEAU);

/// Pointeur de l'état courant : génération et contrôle de l'assemblage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pointeur {
    /// Génération des morceaux de l'état courant (≥ 1).
    pub generation: u64,
    /// Nombre de morceaux (`1..=`[`MAX_MORCEAUX`]).
    pub morceaux: u32,
    /// Taille totale de l'état (≤ taille maximale de l'état `MFED`).
    pub taille: u64,
    /// SHA-256 de l'état assemblé.
    pub empreinte: [u8; 32],
}

/// Hexadécimal minuscule.
pub fn hex(octets: &[u8]) -> String {
    const CHIFFRES: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(octets.len() * 2);
    for o in octets {
        s.push(char::from(CHIFFRES[usize::from(o >> 4)]));
        s.push(char::from(CHIFFRES[usize::from(o & 0x0f)]));
    }
    s
}

/// Décodage strict de l'hexadécimal minuscule (`None` : longueur impaire,
/// caractère hors `0-9a-f`, ou plus de `max` octets — refusé avant allocation).
pub fn de_hex(s: &str, max: usize) -> Option<Vec<u8>> {
    let b = s.as_bytes();
    if !b.len().is_multiple_of(2) || b.len() / 2 > max {
        return None;
    }
    let chiffre = |c: u8| match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        _ => None,
    };
    b.as_chunks::<2>().0.iter().map(|[h, l]| Some((chiffre(*h)? << 4) | chiffre(*l)?)).collect()
}

/// Découpe un état en morceaux hexadécimaux et calcule son pointeur
/// (génération à fixer par le support). État vide ou trop grand ⇒ refus.
pub fn decouper(octets: &[u8], generation: u64) -> Result<(Pointeur, Vec<String>), ErreurEtat> {
    if octets.is_empty() {
        return Err(ErreurEtat::Illisible("état vide"));
    }
    if octets.len() as u64 > TAILLE_MAX_FICHIER {
        return Err(ErreurEtat::TropGrand);
    }
    let morceaux: Vec<String> = octets.chunks(TAILLE_MORCEAU).map(hex).collect();
    let pointeur = Pointeur {
        generation,
        morceaux: u32::try_from(morceaux.len()).map_err(|_| ErreurEtat::TropGrand)?,
        taille: octets.len() as u64,
        empreinte: Sha256::digest(octets).into(),
    };
    Ok((pointeur, morceaux))
}

/// Assemble les morceaux `(rang, hexadécimal)` lus pour `pointeur` et
/// contrôle tout avant de rendre un octet : nombre de morceaux, rangs
/// `0..n` exactement (sans doublon), morceaux pleins sauf le dernier (non
/// vide), taille totale, SHA-256. Bornes vérifiées **avant** allocation.
/// Le contenu reste à revérifier (`MFED` : HMAC, titres signés).
pub fn assembler_morceaux(pointeur: &Pointeur, morceaux: &[(u64, &str)]) -> Result<Vec<u8>, ErreurEtat> {
    let n = pointeur.morceaux as usize;
    if n == 0 || n > MAX_MORCEAUX || pointeur.taille == 0 || pointeur.taille > TAILLE_MAX_FICHIER {
        return Err(ErreurEtat::Illisible("pointeur hors bornes"));
    }
    if (pointeur.taille as usize).div_ceil(TAILLE_MORCEAU) != n {
        return Err(ErreurEtat::Illisible("pointeur incohérent"));
    }
    if morceaux.len() != n {
        return Err(ErreurEtat::Illisible("morceaux manquants ou en trop"));
    }
    let mut ordonnes: Vec<&(u64, &str)> = morceaux.iter().collect();
    ordonnes.sort_unstable_by_key(|(rang, _)| *rang);
    let mut etat = Vec::with_capacity(pointeur.taille as usize);
    for (attendu, (rang, texte)) in ordonnes.into_iter().enumerate() {
        if *rang != attendu as u64 {
            return Err(ErreurEtat::Illisible("rang de morceau"));
        }
        let octets = de_hex(texte, TAILLE_MORCEAU).ok_or(ErreurEtat::Illisible("morceau"))?;
        let dernier = attendu + 1 == n;
        if octets.is_empty() || (!dernier && octets.len() != TAILLE_MORCEAU) {
            return Err(ErreurEtat::Illisible("taille de morceau"));
        }
        etat.extend_from_slice(&octets);
    }
    if etat.len() as u64 != pointeur.taille {
        return Err(ErreurEtat::Illisible("taille de l'état"));
    }
    let empreinte: [u8; 32] = Sha256::digest(&etat).into();
    if empreinte != pointeur.empreinte {
        return Err(ErreurEtat::Illisible("empreinte de l'état"));
    }
    Ok(etat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_aller_retour_et_refus() {
        let o: Vec<u8> = (0..=255).collect();
        assert_eq!(de_hex(&hex(&o), 256).unwrap(), o);
        assert!(de_hex("abc", 8).is_none());
        assert!(de_hex("AB", 8).is_none());
        assert!(de_hex("zz", 8).is_none());
        assert!(de_hex("0000", 1).is_none());
        assert_eq!(de_hex("", 0).unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn decouper_puis_assembler() {
        for taille in [1, 41, TAILLE_MORCEAU - 1, TAILLE_MORCEAU, TAILLE_MORCEAU + 1, 3 * TAILLE_MORCEAU + 7] {
            let o: Vec<u8> = (0..taille).map(|i| (i * 7 % 251) as u8).collect();
            let (p, m) = decouper(&o, 3).unwrap();
            assert_eq!(p.morceaux as usize, taille.div_ceil(TAILLE_MORCEAU));
            let mut lus: Vec<(u64, &str)> = m.iter().enumerate().map(|(i, s)| (i as u64, s.as_str())).collect();
            lus.reverse();
            assert_eq!(assembler_morceaux(&p, &lus).unwrap(), o);
        }
    }

    #[test]
    fn assemblage_incoherent_refuse() {
        let o: Vec<u8> = vec![7; 2 * TAILLE_MORCEAU + 3];
        let (p, m) = decouper(&o, 1).unwrap();
        let lus = |m: &[String]| m.iter().enumerate().map(|(i, s)| (i as u64, s.clone())).collect::<Vec<_>>();
        let essai = |p: &Pointeur, l: &[(u64, String)]| {
            let l: Vec<(u64, &str)> = l.iter().map(|(r, s)| (*r, s.as_str())).collect();
            assembler_morceaux(p, &l)
        };
        // Morceau manquant, doublon, rang décalé.
        let mut l = lus(&m);
        l.pop();
        assert!(essai(&p, &l).is_err());
        let mut l = lus(&m);
        l[2].0 = 1;
        assert!(essai(&p, &l).is_err());
        let mut l = lus(&m);
        l[2].0 = 3;
        assert!(essai(&p, &l).is_err());
        // Un octet altéré : empreinte fausse.
        let mut l = lus(&m);
        l[1].1.replace_range(0..2, "08");
        assert!(matches!(essai(&p, &l), Err(ErreurEtat::Illisible("empreinte de l'état"))));
        // Pointeur menteur (taille, nombre).
        assert!(essai(&Pointeur { taille: p.taille + 1, ..p }, &lus(&m)).is_err());
        assert!(essai(&Pointeur { morceaux: 4, ..p }, &lus(&m)).is_err());
        assert!(essai(&Pointeur { taille: TAILLE_MAX_FICHIER + 1, ..p }, &lus(&m)).is_err());
        // Morceau intermédiaire court.
        let mut l = lus(&m);
        l[0].1.truncate(10);
        assert!(essai(&p, &l).is_err());
        assert!(decouper(&[], 1).is_err());
    }

    /// Exécute un futur prêt immédiatement (supports synchrones).
    fn rt<F: Future>(f: F) -> F::Output {
        let mut f = std::pin::pin!(f);
        let mut cx = std::task::Context::from_waker(std::task::Waker::noop());
        loop {
            if let std::task::Poll::Ready(v) = f.as_mut().poll(&mut cx) {
                return v;
            }
        }
    }

    #[test]
    fn memoire_pannes_et_partage() {
        let m = SupportMemoire::new();
        let clone = m.clone();
        m.echouer_ecritures(1);
        assert!(rt(SupportEtat::ecrire(&m, b"a")).is_err());
        assert_eq!(clone.octets(), None);
        rt(SupportEtat::ecrire(&m, b"b")).unwrap();
        assert_eq!(clone.octets(), Some(b"b".to_vec()));
        assert_eq!(clone.ecritures(), 1);
    }
}
