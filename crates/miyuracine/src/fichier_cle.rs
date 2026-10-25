//! @id     miyuracine_fichier_cle
//! @do     chiffrer_racine_support_amovible
//! @role   securite
//! @layer  toolkit
//! @human  miyuracine — fichier de clé racine chiffré Argon2id + XChaCha20-Poly1305.

//! Fichier de clé racine chiffré, destiné au support amovible
//! (`docs/securite/ARCHITECTURE-lot2-identite-signee.md` §2, ligne « Racine COG » :
//! « fichier chiffré Argon2id exporté sur support amovible »).
//!
//! Le fichier contient le secret maître de 256 bits de la [`Racine`], chiffré
//! par XChaCha20-Poly1305 sous une clé dérivée du mot de passe par Argon2id
//! (RFC 9106, version 0x13). Il complète la sauvegarde SLIP-39 2 sur 3 sans la
//! remplacer.
//!
//! # Format `MYRC` version 1 (122 octets exactement, entiers gros-boutistes)
//!
//! | Décalage | Taille | Champ        | Valeur / contrainte                                              |
//! |---------:|-------:|--------------|------------------------------------------------------------------|
//! | 0        | 4      | `magie`      | `4D 59 52 43` (ASCII « MYRC »)                                   |
//! | 4        | 2      | `version`    | `00 01` ; toute autre valeur ⇒ refus                             |
//! | 6        | 4      | `m_kio`      | mémoire Argon2id en Kio, 262 144 ≤ m ≤ 4 194 304 (256 Mio–4 Gio)  |
//! | 10       | 4      | `t`          | itérations Argon2id, 3 ≤ t ≤ 64                                  |
//! | 14       | 4      | `p`          | parallélisme Argon2id, 1 ≤ p ≤ 16                                |
//! | 18       | 32     | `sel`        | sel Argon2id aléatoire (`OsRng`), propre à chaque écriture       |
//! | 50       | 24     | `nonce`      | nonce XChaCha20-Poly1305 aléatoire (`OsRng`)                      |
//! | 74       | 32     | `chiffre`    | secret maître chiffré (XChaCha20)                                |
//! | 106      | 16     | `etiquette`  | étiquette Poly1305                                               |
//!
//! - Clé AEAD : `Argon2id(mot_de_passe, sel, m_kio, t, p, version 0x13)`,
//!   sortie 32 octets, sans secret ni données associées Argon2.
//! - Données associées AEAD : les 74 octets d'en-tête (`magie` à `nonce`
//!   inclus) ; toute altération de l'en-tête invalide l'étiquette.
//! - Clair : les 32 octets du secret maître (graine SLIP-10 de la racine).
//!
//! # Lecture fail-closed
//!
//! Taille différente de 122 octets, magie ou version inconnue, paramètres
//! hors bornes, échec d'Argon2id ou de l'étiquette (mauvais mot de passe,
//! fichier altéré) : **une seule** erreur, [`ErreurRacine::FichierCleInvalide`],
//! sans indice distinctif. Les bornes hautes empêchent un fichier forgé
//! d'imposer un calcul ou une allocation démesurés. Seule exception, d'ordre
//! local et non cryptographique : un `m_kio` dépassant le budget de mémoire
//! verrouillée par [`crate::durcissement`] ⇒ [`ErreurRacine::VerrouillageMemoire`].
//!
//! Contrôles du système de fichiers (Linux), avant toute lecture du contenu :
//! dernier composant ouvert en `O_NOFOLLOW` (lien symbolique ⇒ refus),
//! fichier ordinaire, aucun droit pour le groupe ni les autres
//! (`mode & 0o077 == 0`), propriétaire = uid effectif. Ces refus
//! ([`ErreurRacine::FichierCleRefuse`]) ne portent que sur des métadonnées.
//! Un support FAT/exFAT doit donc être monté avec `uid=<opérateur>,fmask=0177,dmask=0077`.
//!
//! # Écriture atomique
//!
//! Fichier temporaire `.<nom>.<16 hex>.tmp` créé dans le même répertoire
//! (`O_CREAT | O_EXCL | O_NOFOLLOW`, droits `0600`, `fchmod` explicite,
//! droits relus), écrit puis `fsync`, renommé par `renameat2(RENAME_NOREPLACE)`
//! (jamais d'écrasement : une destination existante, lien symbolique compris,
//! est refusée), puis `fsync` du répertoire. Tout échec supprime le temporaire.

use chacha20poly1305::aead::AeadInPlace;
use chacha20poly1305::{Key, KeyInit, Tag, XChaCha20Poly1305, XNonce};
use rand_core::{OsRng, RngCore};
use zeroize::Zeroizing;

use crate::durcissement::verifier_budget;
use crate::erreurs::ErreurRacine;
use crate::racine::{Racine, TAILLE_SECRET};

/// Magie du format (« MYRC », MiYuRaCine).
pub const MAGIE: [u8; 4] = *b"MYRC";
/// Version du format.
pub const VERSION: u16 = 1;
/// Taille du sel Argon2id (octets).
pub const TAILLE_SEL: usize = 32;
/// Taille du nonce XChaCha20-Poly1305 (octets).
pub const TAILLE_NONCE: usize = 24;
/// Taille de l'étiquette Poly1305 (octets).
pub const TAILLE_ETIQUETTE: usize = 16;
/// Taille de l'en-tête (données associées AEAD).
pub const TAILLE_EN_TETE: usize = 4 + 2 + 4 + 4 + 4 + TAILLE_SEL + TAILLE_NONCE;
/// Taille exacte du fichier.
pub const TAILLE_FICHIER: usize = TAILLE_EN_TETE + TAILLE_SECRET + TAILLE_ETIQUETTE;
/// Longueur minimale du mot de passe à l'écriture (octets).
pub const MOT_DE_PASSE_MIN: usize = 12;

const DEC_VERSION: usize = 4;
const DEC_M: usize = 6;
const DEC_T: usize = 10;
const DEC_P: usize = 14;
const DEC_SEL: usize = 18;
const DEC_NONCE: usize = DEC_SEL + TAILLE_SEL;
const DEC_CHIFFRE: usize = TAILLE_EN_TETE;
const DEC_ETIQUETTE: usize = DEC_CHIFFRE + TAILLE_SECRET;

/// Bornes admises des paramètres Argon2id (inclusives).
#[derive(Debug, Clone, Copy)]
struct Bornes {
    m: (u32, u32),
    t: (u32, u32),
    p: (u32, u32),
}

impl Bornes {
    fn admet(&self, m: u32, t: u32, p: u32) -> bool {
        (self.m.0..=self.m.1).contains(&m)
            && (self.t.0..=self.t.1).contains(&t)
            && (self.p.0..=self.p.1).contains(&p)
    }
}

/// Bornes de production : m ∈ [256 Mio, 4 Gio], t ∈ [3, 64], p ∈ [1, 16].
const BORNES: Bornes = Bornes {
    m: (262_144, 4_194_304),
    t: (3, 64),
    p: (1, 16),
};

/// Bornes faibles, **tests unitaires uniquement** (jamais compilées ailleurs).
#[cfg(test)]
const BORNES_TEST: Bornes = Bornes {
    m: (8, 64),
    t: (1, 4),
    p: (1, 2),
};

/// Paramètres Argon2id validés contre les bornes de production.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParametresArgon2 {
    m_kio: u32,
    t: u32,
    p: u32,
}

impl ParametresArgon2 {
    /// Défaut : 1 Gio, 4 itérations, 4 voies.
    pub const DEFAUT: Self = Self {
        m_kio: 1_048_576,
        t: 4,
        p: 4,
    };

    /// Paramètres explicites ; hors bornes ⇒ [`ErreurRacine::ParametresArgon2HorsBornes`].
    pub fn nouveaux(m_kio: u32, t: u32, p: u32) -> Result<Self, ErreurRacine> {
        if BORNES.admet(m_kio, t, p) {
            Ok(Self { m_kio, t, p })
        } else {
            Err(ErreurRacine::ParametresArgon2HorsBornes)
        }
    }

    /// Mémoire Argon2id (Kio).
    pub fn memoire_kio(&self) -> u32 {
        self.m_kio
    }

    /// Itérations.
    pub fn iterations(&self) -> u32 {
        self.t
    }

    /// Parallélisme.
    pub fn parallelisme(&self) -> u32 {
        self.p
    }

    /// Mémoire Argon2id en octets (budget de verrouillage).
    pub fn memoire_octets(&self) -> u64 {
        u64::from(self.m_kio) * 1024
    }
}

/// Mémoire Argon2id maximale admise (octets) : budget de verrouillage pour lire
/// n'importe quel fichier conforme.
pub const MEMOIRE_ARGON2_MAX: u64 = BORNES.m.1 as u64 * 1024;

fn deriver_cle(
    mot_de_passe: &[u8],
    sel: &[u8],
    m: u32,
    t: u32,
    p: u32,
) -> Result<Zeroizing<[u8; 32]>, ErreurRacine> {
    verifier_budget(u64::from(m) * 1024)?;
    let params =
        argon2::Params::new(m, t, p, Some(32)).map_err(|_| ErreurRacine::FichierCleInvalide)?;
    let argon = argon2::Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    let mut cle = Zeroizing::new([0u8; 32]);
    argon
        .hash_password_into(mot_de_passe, sel, cle.as_mut_slice())
        .map_err(|_| ErreurRacine::FichierCleInvalide)?;
    Ok(cle)
}

fn chiffrer_avec(
    secret: &[u8; TAILLE_SECRET],
    mot_de_passe: &[u8],
    (m, t, p): (u32, u32, u32),
    bornes: &Bornes,
) -> Result<Vec<u8>, ErreurRacine> {
    if mot_de_passe.len() < MOT_DE_PASSE_MIN {
        return Err(ErreurRacine::MotDePasseTropCourt);
    }
    if !bornes.admet(m, t, p) {
        return Err(ErreurRacine::ParametresArgon2HorsBornes);
    }
    let mut octets = vec![0u8; TAILLE_FICHIER];
    octets[..DEC_VERSION].copy_from_slice(&MAGIE);
    octets[DEC_VERSION..DEC_M].copy_from_slice(&VERSION.to_be_bytes());
    octets[DEC_M..DEC_T].copy_from_slice(&m.to_be_bytes());
    octets[DEC_T..DEC_P].copy_from_slice(&t.to_be_bytes());
    octets[DEC_P..DEC_SEL].copy_from_slice(&p.to_be_bytes());
    OsRng
        .try_fill_bytes(&mut octets[DEC_SEL..DEC_CHIFFRE])
        .map_err(|_| ErreurRacine::AleaIndisponible)?;

    let cle = deriver_cle(mot_de_passe, &octets[DEC_SEL..DEC_NONCE], m, t, p)?;
    let aead = XChaCha20Poly1305::new(Key::from_slice(cle.as_slice()));
    let (en_tete, reste) = octets.split_at_mut(TAILLE_EN_TETE);
    let (chiffre, etiquette) = reste.split_at_mut(TAILLE_SECRET);
    chiffre.copy_from_slice(secret);
    let tag = aead
        .encrypt_in_place_detached(XNonce::from_slice(&en_tete[DEC_NONCE..]), en_tete, chiffre)
        .map_err(|_| ErreurRacine::FichierCleInvalide)?;
    etiquette.copy_from_slice(&tag);
    Ok(octets)
}

/// Taille, magie, version et bornes ; `None` au moindre écart.
fn en_tete_valide(octets: &[u8], bornes: &Bornes) -> Option<ParametresArgon2> {
    if octets.len() != TAILLE_FICHIER || octets[..DEC_VERSION] != MAGIE {
        return None;
    }
    let lire_u32 =
        |d: usize| u32::from_be_bytes([octets[d], octets[d + 1], octets[d + 2], octets[d + 3]]);
    let version = u16::from_be_bytes([octets[DEC_VERSION], octets[DEC_VERSION + 1]]);
    let (m_kio, t, p) = (lire_u32(DEC_M), lire_u32(DEC_T), lire_u32(DEC_P));
    (version == VERSION && bornes.admet(m_kio, t, p)).then_some(ParametresArgon2 { m_kio, t, p })
}

fn dechiffrer_avec(
    octets: &[u8],
    mot_de_passe: &[u8],
    bornes: &Bornes,
) -> Result<Racine, ErreurRacine> {
    let ParametresArgon2 { m_kio: m, t, p } =
        en_tete_valide(octets, bornes).ok_or(ErreurRacine::FichierCleInvalide)?;
    let cle = deriver_cle(mot_de_passe, &octets[DEC_SEL..DEC_NONCE], m, t, p)?;
    let aead = XChaCha20Poly1305::new(Key::from_slice(cle.as_slice()));
    let mut clair = Zeroizing::new([0u8; TAILLE_SECRET]);
    clair.copy_from_slice(&octets[DEC_CHIFFRE..DEC_ETIQUETTE]);
    aead.decrypt_in_place_detached(
        XNonce::from_slice(&octets[DEC_NONCE..DEC_CHIFFRE]),
        &octets[..TAILLE_EN_TETE],
        clair.as_mut_slice(),
        Tag::from_slice(&octets[DEC_ETIQUETTE..]),
    )
    .map_err(|_| ErreurRacine::FichierCleInvalide)?;
    Ok(Racine::depuis_secret(&clair))
}

/// Chiffre la racine au format `MYRC` v1 (sel et nonce neufs à chaque appel).
pub fn chiffrer_racine(
    racine: &Racine,
    mot_de_passe: &[u8],
    params: ParametresArgon2,
) -> Result<Vec<u8>, ErreurRacine> {
    chiffrer_avec(
        racine.secret(),
        mot_de_passe,
        (params.m_kio, params.t, params.p),
        &BORNES,
    )
}

/// Déchiffre un contenu `MYRC` v1 ; tout défaut ⇒ [`ErreurRacine::FichierCleInvalide`].
pub fn dechiffrer_racine(octets: &[u8], mot_de_passe: &[u8]) -> Result<Racine, ErreurRacine> {
    dechiffrer_avec(octets, mot_de_passe, &BORNES)
}

/// Écrit la racine chiffrée dans un fichier **neuf** (écriture atomique, `0600`).
pub fn ecrire_fichier_cle(
    chemin: &std::path::Path,
    racine: &Racine,
    mot_de_passe: &[u8],
    params: ParametresArgon2,
) -> Result<(), ErreurRacine> {
    systeme::refuser_existant(chemin)?;
    let octets = chiffrer_racine(racine, mot_de_passe, params)?;
    systeme::ecrire_atomique(chemin, &octets)
}

/// Lit les paramètres Argon2id d'un fichier de clé, sans mot de passe ni
/// calcul (pour dimensionner le verrouillage mémoire **avant** de lire un
/// secret). Mêmes contrôles et mêmes refus que [`lire_fichier_cle`] pour la
/// structure ; l'authenticité n'est établie que par le déchiffrement.
pub fn lire_parametres(chemin: &std::path::Path) -> Result<ParametresArgon2, ErreurRacine> {
    let octets = systeme::lire(chemin)?;
    en_tete_valide(&octets, &BORNES).ok_or(ErreurRacine::FichierCleInvalide)
}

/// Lit un fichier **secret** (part SLIP-39, phrase, mot de passe) avec les
/// mêmes contrôles que le fichier de clé (Linux) : dernier composant ouvert
/// en `O_NOFOLLOW` (lien symbolique ⇒ refus), fichier ordinaire, aucun droit
/// pour le groupe ni les autres (`0600` ou plus strict), propriétaire = uid
/// effectif, au plus `taille_max` octets (lecture bornée, tampon effacé à la
/// libération). Refus : [`ErreurRacine::FichierSecretRefuse`].
pub fn lire_fichier_secret(chemin: &std::path::Path, taille_max: u64) -> Result<Zeroizing<Vec<u8>>, ErreurRacine> {
    systeme::lire_borne(chemin, taille_max, ErreurRacine::FichierSecretRefuse, || {
        ErreurRacine::FichierSecretRefuse("fichier trop grand")
    })
}

/// Lit et déchiffre un fichier de clé racine (contrôles de la section « Lecture »).
pub fn lire_fichier_cle(
    chemin: &std::path::Path,
    mot_de_passe: &[u8],
) -> Result<Racine, ErreurRacine> {
    let octets = systeme::lire(chemin)?;
    dechiffrer_racine(&octets, mot_de_passe)
}

#[cfg(target_os = "linux")]
mod systeme {
    use std::fs::File;
    use std::io::{Read, Write};
    use std::os::unix::fs::MetadataExt;
    use std::path::{Path, PathBuf};

    use rand_core::{OsRng, RngCore};
    use rustix::fs::{Mode, OFlags, RenameFlags, CWD};
    use rustix::io::Errno;
    use zeroize::Zeroizing;

    use super::TAILLE_FICHIER;
    use crate::erreurs::ErreurRacine;

    fn es(e: impl Into<std::io::Error>) -> ErreurRacine {
        ErreurRacine::EntreeSortie(e.into().kind())
    }

    fn controler(
        meta: &std::fs::Metadata,
        refus: fn(&'static str) -> ErreurRacine,
    ) -> Result<(), ErreurRacine> {
        if !meta.file_type().is_file() {
            return Err(refus("pas un fichier ordinaire"));
        }
        if meta.mode() & 0o077 != 0 {
            return Err(refus(
                "droits trop larges (attendu 0600, aucun accès groupe/autres)",
            ));
        }
        if meta.uid() != rustix::process::geteuid().as_raw() {
            return Err(refus("propriétaire différent de l'utilisateur effectif"));
        }
        Ok(())
    }

    /// Lecture stricte commune (fichier de clé, fichiers secrets) : `O_NOFOLLOW`,
    /// contrôles de métadonnées sur le descripteur ouvert, au plus `taille_max`
    /// octets lus dans un tampon préalloué (aucune réallocation, effacé).
    pub(super) fn lire_borne(
        chemin: &Path,
        taille_max: u64,
        refus: fn(&'static str) -> ErreurRacine,
        trop_grand: impl Fn() -> ErreurRacine,
    ) -> Result<Zeroizing<Vec<u8>>, ErreurRacine> {
        let fd = rustix::fs::open(
            chemin,
            OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::CLOEXEC | OFlags::NOCTTY | OFlags::NONBLOCK,
            Mode::empty(),
        )
        .map_err(|e| {
            if e == Errno::LOOP {
                refus("lien symbolique")
            } else {
                es(e)
            }
        })?;
        let fichier = File::from(fd);
        let meta = fichier.metadata().map_err(es)?;
        controler(&meta, refus)?;
        if meta.len() > taille_max {
            return Err(trop_grand());
        }
        let capacite = usize::try_from(taille_max.saturating_add(1)).map_err(|_| trop_grand())?;
        let mut octets = Zeroizing::new(Vec::with_capacity(capacite));
        fichier
            .take(taille_max.saturating_add(1))
            .read_to_end(&mut octets)
            .map_err(es)?;
        if octets.len() as u64 > taille_max {
            return Err(trop_grand());
        }
        Ok(octets)
    }

    pub(super) fn refuser_existant(chemin: &Path) -> Result<(), ErreurRacine> {
        match std::fs::symlink_metadata(chemin) {
            Ok(m) if m.file_type().is_symlink() => {
                Err(ErreurRacine::FichierCleRefuse("lien symbolique"))
            }
            Ok(_) => Err(ErreurRacine::FichierCleRefuse("la destination existe déjà")),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(es(e)),
        }
    }

    pub(super) fn lire(chemin: &Path) -> Result<Zeroizing<Vec<u8>>, ErreurRacine> {
        let octets = lire_borne(chemin, TAILLE_FICHIER as u64, ErreurRacine::FichierCleRefuse, || {
            ErreurRacine::FichierCleInvalide
        })?;
        if octets.len() != TAILLE_FICHIER {
            return Err(ErreurRacine::FichierCleInvalide);
        }
        Ok(octets)
    }

    fn temporaire(chemin: &Path) -> Result<(PathBuf, PathBuf), ErreurRacine> {
        let nom = chemin
            .file_name()
            .ok_or(ErreurRacine::FichierCleRefuse("chemin sans nom de fichier"))?;
        let parent = match chemin.parent() {
            Some(p) if !p.as_os_str().is_empty() => p.to_path_buf(),
            _ => PathBuf::from("."),
        };
        let mut alea = [0u8; 8];
        OsRng
            .try_fill_bytes(&mut alea)
            .map_err(|_| ErreurRacine::AleaIndisponible)?;
        let suffixe = u64::from_be_bytes(alea);
        let mut nom_tmp = std::ffi::OsString::from(".");
        nom_tmp.push(nom);
        nom_tmp.push(format!(".{suffixe:016x}.tmp"));
        Ok((parent.join(nom_tmp), parent))
    }

    pub(super) fn ecrire_atomique(chemin: &Path, octets: &[u8]) -> Result<(), ErreurRacine> {
        let (tmp, parent) = temporaire(chemin)?;
        let fd = rustix::fs::open(
            &tmp,
            OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::from_raw_mode(0o600),
        )
        .map_err(es)?;
        let resultat = (|| {
            rustix::fs::fchmod(&fd, Mode::from_raw_mode(0o600)).map_err(es)?;
            let mut fichier = File::from(fd);
            controler(&fichier.metadata().map_err(es)?, ErreurRacine::FichierCleRefuse)?;
            fichier.write_all(octets).map_err(es)?;
            fichier.sync_all().map_err(es)?;
            drop(fichier);
            rustix::fs::renameat_with(CWD, &tmp, CWD, chemin, RenameFlags::NOREPLACE).map_err(|e| {
                if e == Errno::EXIST {
                    ErreurRacine::FichierCleRefuse("la destination existe déjà")
                } else {
                    es(e)
                }
            })
        })();
        if let Err(e) = resultat {
            let _ = std::fs::remove_file(&tmp);
            return Err(e);
        }
        let dossier = rustix::fs::open(
            &parent,
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::CLOEXEC,
            Mode::empty(),
        )
        .map_err(es)?;
        rustix::fs::fsync(&dossier).map_err(es)
    }
}

#[cfg(not(target_os = "linux"))]
mod systeme {
    use std::path::Path;

    use crate::erreurs::ErreurRacine;

    const REFUS: ErreurRacine =
        ErreurRacine::FichierCleRefuse("plateforme non prise en charge (Linux requis)");

    pub(super) fn refuser_existant(_: &Path) -> Result<(), ErreurRacine> {
        Err(REFUS)
    }
    pub(super) fn lire(_: &Path) -> Result<zeroize::Zeroizing<Vec<u8>>, ErreurRacine> {
        Err(REFUS)
    }
    pub(super) fn lire_borne(
        _: &Path,
        _: u64,
        refus: fn(&'static str) -> ErreurRacine,
        _: impl Fn() -> ErreurRacine,
    ) -> Result<zeroize::Zeroizing<Vec<u8>>, ErreurRacine> {
        Err(refus("plateforme non prise en charge (Linux requis)"))
    }
    pub(super) fn ecrire_atomique(_: &Path, _: &[u8]) -> Result<(), ErreurRacine> {
        Err(REFUS)
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU32, Ordering};

    use super::*;

    const MDP: &[u8] = b"mot de passe de test 2026";
    const FAIBLES: (u32, u32, u32) = (8, 1, 1);

    fn racine() -> Racine {
        let mut s = [0u8; TAILLE_SECRET];
        for (i, o) in s.iter_mut().enumerate() {
            *o = i as u8 ^ 0xa5;
        }
        Racine::depuis_secret(&s)
    }

    fn chiffrer_faible(r: &Racine) -> Vec<u8> {
        chiffrer_avec(r.secret(), MDP, FAIBLES, &BORNES_TEST).unwrap()
    }

    fn dossier() -> PathBuf {
        static N: AtomicU32 = AtomicU32::new(0);
        let d = std::env::temp_dir().join(format!(
            "miyuracine-fichier-cle-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::SeqCst)
        ));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn format_et_taille() {
        assert_eq!(TAILLE_EN_TETE, 74);
        assert_eq!(TAILLE_FICHIER, 122);
        let c = chiffrer_faible(&racine());
        assert_eq!(c.len(), TAILLE_FICHIER);
        assert_eq!(&c[..4], b"MYRC");
        assert_eq!(&c[4..6], &[0, 1]);
        assert_eq!(&c[6..10], &8u32.to_be_bytes());
        // Sel et nonce neufs : deux chiffrements diffèrent.
        assert_ne!(c, chiffrer_faible(&racine()));
    }

    #[test]
    fn aller_retour() {
        let r = racine();
        let c = chiffrer_faible(&r);
        let r2 = dechiffrer_avec(&c, MDP, &BORNES_TEST).unwrap();
        assert_eq!(r2.secret(), r.secret());
    }

    #[test]
    fn mauvais_mot_de_passe() {
        let c = chiffrer_faible(&racine());
        assert_eq!(
            dechiffrer_avec(&c, b"mot de passe de test 2027", &BORNES_TEST).unwrap_err(),
            ErreurRacine::FichierCleInvalide
        );
        assert_eq!(
            dechiffrer_avec(&c, b"", &BORNES_TEST).unwrap_err(),
            ErreurRacine::FichierCleInvalide
        );
    }

    #[test]
    fn mot_de_passe_trop_court_refuse() {
        assert_eq!(
            chiffrer_avec(racine().secret(), b"court", FAIBLES, &BORNES_TEST).unwrap_err(),
            ErreurRacine::MotDePasseTropCourt
        );
    }

    #[test]
    fn parametres_faibles_refuses() {
        // Construction publique : bornes de production.
        for (m, t, p) in [
            (262_143, 3, 1),
            (262_144, 2, 1),
            (262_144, 3, 0),
            (262_144, 3, 17),
            (4_194_305, 3, 1),
            (65_536, 3, 4),
        ] {
            assert_eq!(
                ParametresArgon2::nouveaux(m, t, p).unwrap_err(),
                ErreurRacine::ParametresArgon2HorsBornes
            );
        }
        assert!(ParametresArgon2::nouveaux(262_144, 3, 1).is_ok());
        assert!(BORNES.admet(
            ParametresArgon2::DEFAUT.memoire_kio(),
            ParametresArgon2::DEFAUT.iterations(),
            ParametresArgon2::DEFAUT.parallelisme()
        ));
        // Un fichier aux paramètres faibles (pourtant authentique) est refusé
        // par la lecture de production, sans calcul Argon2id.
        let c = chiffrer_faible(&racine());
        assert_eq!(
            dechiffrer_racine(&c, MDP).unwrap_err(),
            ErreurRacine::FichierCleInvalide
        );
        // Écriture de production aux paramètres faibles : refus.
        assert_eq!(
            chiffrer_avec(racine().secret(), MDP, FAIBLES, &BORNES).unwrap_err(),
            ErreurRacine::ParametresArgon2HorsBornes
        );
    }

    #[test]
    fn tronque_ou_allonge_refuse() {
        let c = chiffrer_faible(&racine());
        for n in 0..TAILLE_FICHIER {
            assert_eq!(
                dechiffrer_avec(&c[..n], MDP, &BORNES_TEST).unwrap_err(),
                ErreurRacine::FichierCleInvalide,
                "longueur {n}"
            );
        }
        let mut long = c.clone();
        long.push(0);
        assert_eq!(
            dechiffrer_avec(&long, MDP, &BORNES_TEST).unwrap_err(),
            ErreurRacine::FichierCleInvalide
        );
    }

    #[test]
    fn altere_octet_par_octet_refuse() {
        let c = chiffrer_faible(&racine());
        for i in 0..TAILLE_FICHIER {
            for masque in [0x01u8, 0x80, 0xff] {
                let mut a = c.clone();
                a[i] ^= masque;
                assert_eq!(
                    dechiffrer_avec(&a, MDP, &BORNES_TEST).unwrap_err(),
                    ErreurRacine::FichierCleInvalide,
                    "octet {i}, masque {masque:#04x}"
                );
            }
        }
    }

    #[test]
    fn version_inconnue_refusee() {
        let mut c = chiffrer_faible(&racine());
        c[DEC_VERSION + 1] = 2;
        assert_eq!(
            dechiffrer_avec(&c, MDP, &BORNES_TEST).unwrap_err(),
            ErreurRacine::FichierCleInvalide
        );
    }

    #[test]
    fn fichier_aller_retour_et_droits() {
        let d = dossier();
        let chemin = d.join("racine.myrc");
        let r = racine();
        systeme::refuser_existant(&chemin).unwrap();
        systeme::ecrire_atomique(&chemin, &chiffrer_faible(&r)).unwrap();
        let meta = std::fs::metadata(&chemin).unwrap();
        assert_eq!(meta.permissions().mode() & 0o777, 0o600);
        let lu = systeme::lire(&chemin).unwrap();
        assert_eq!(
            dechiffrer_avec(&lu, MDP, &BORNES_TEST).unwrap().secret(),
            r.secret()
        );
        // Aucun temporaire résiduel.
        assert_eq!(std::fs::read_dir(&d).unwrap().count(), 1);

        // Jamais d'écrasement.
        assert_eq!(
            systeme::refuser_existant(&chemin).unwrap_err(),
            ErreurRacine::FichierCleRefuse("la destination existe déjà")
        );
        assert!(matches!(
            systeme::ecrire_atomique(&chemin, &chiffrer_faible(&r)).unwrap_err(),
            ErreurRacine::FichierCleRefuse(_)
        ));
        assert_eq!(std::fs::read_dir(&d).unwrap().count(), 1);

        // Droits trop larges : refus à la lecture.
        for mode in [0o640, 0o604, 0o620, 0o644] {
            std::fs::set_permissions(&chemin, std::fs::Permissions::from_mode(mode)).unwrap();
            assert!(
                matches!(
                    systeme::lire(&chemin).unwrap_err(),
                    ErreurRacine::FichierCleRefuse(_)
                ),
                "mode {mode:o}"
            );
        }
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn lien_symbolique_refuse() {
        let d = dossier();
        let cible = d.join("cible.myrc");
        systeme::ecrire_atomique(&cible, &chiffrer_faible(&racine())).unwrap();
        let lien = d.join("lien.myrc");
        std::os::unix::fs::symlink(&cible, &lien).unwrap();
        assert_eq!(
            systeme::lire(&lien).unwrap_err(),
            ErreurRacine::FichierCleRefuse("lien symbolique")
        );
        assert_eq!(
            systeme::refuser_existant(&lien).unwrap_err(),
            ErreurRacine::FichierCleRefuse("lien symbolique")
        );
        // Lien pendant : l'écriture ne le suit pas.
        let pendant = d.join("pendant.myrc");
        std::os::unix::fs::symlink(d.join("absent"), &pendant).unwrap();
        assert!(systeme::refuser_existant(&pendant).is_err());
        assert!(systeme::ecrire_atomique(&pendant, &chiffrer_faible(&racine())).is_err());
        assert!(!d.join("absent").exists());
        std::fs::remove_dir_all(&d).unwrap();
    }

    /// B5 : fichiers secrets (parts, phrase, mot de passe) — mêmes contrôles
    /// stricts que le fichier de clé.
    #[test]
    fn fichier_secret_strict() {
        let d = dossier();
        let ok = d.join("part.txt");
        std::fs::write(&ok, b"mots de la part\n").unwrap();
        std::fs::set_permissions(&ok, std::fs::Permissions::from_mode(0o600)).unwrap();
        assert_eq!(lire_fichier_secret(&ok, 4096).unwrap().as_slice(), b"mots de la part\n");
        std::fs::set_permissions(&ok, std::fs::Permissions::from_mode(0o400)).unwrap();
        assert!(lire_fichier_secret(&ok, 4096).is_ok());
        // Droits ouverts au groupe ou aux autres : refus.
        for mode in [0o640, 0o604, 0o644, 0o660] {
            std::fs::set_permissions(&ok, std::fs::Permissions::from_mode(mode)).unwrap();
            assert!(
                matches!(lire_fichier_secret(&ok, 4096), Err(ErreurRacine::FichierSecretRefuse(_))),
                "mode {mode:o}"
            );
        }
        std::fs::set_permissions(&ok, std::fs::Permissions::from_mode(0o600)).unwrap();
        // Lien symbolique (même vers un fichier valide) : refus.
        let lien = d.join("lien.txt");
        std::os::unix::fs::symlink(&ok, &lien).unwrap();
        assert_eq!(
            lire_fichier_secret(&lien, 4096).unwrap_err(),
            ErreurRacine::FichierSecretRefuse("lien symbolique")
        );
        // Trop grand, non ordinaire : refus.
        assert_eq!(
            lire_fichier_secret(&ok, 4).unwrap_err(),
            ErreurRacine::FichierSecretRefuse("fichier trop grand")
        );
        assert_eq!(
            lire_fichier_secret(&d, 4096).unwrap_err(),
            ErreurRacine::FichierSecretRefuse("pas un fichier ordinaire")
        );
        std::fs::remove_dir_all(&d).unwrap();
    }

    #[test]
    fn fichier_tronque_sur_disque_refuse() {
        let d = dossier();
        let chemin = d.join("court.myrc");
        let c = chiffrer_faible(&racine());
        systeme::ecrire_atomique(&chemin, &c[..TAILLE_FICHIER - 1]).unwrap();
        assert_eq!(
            systeme::lire(&chemin).unwrap_err(),
            ErreurRacine::FichierCleInvalide
        );
        let repertoire = d.join("rep");
        std::fs::create_dir(&repertoire).unwrap();
        assert!(systeme::lire(&repertoire).is_err());
        std::fs::remove_dir_all(&d).unwrap();
    }
}
