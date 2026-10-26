//! Scanner d'effacement physique partagé par les tests d'intégration.
//!
//! @id     kindmother_service_tests_commun
//! @do     dechiffrer_pages_sqlcipher_pour_chercher_un_temoin
//! @role   test
//! @layer  core
//! @human  kindmother-service — déchiffre les pages SQLCipher 4 (AES-256-CBC, IV en fin de
//!         page, réserve de 80 octets) d'un fichier de base et de son `-wal` pour chercher
//!         une valeur témoin dans le clair.

#![allow(dead_code)]

use std::path::{Path, PathBuf};

use aes::cipher::{block_padding::NoPadding, BlockDecryptMut, KeyIvInit};
use kindmother_service::DatabaseKeyStore;

/// Taille de page SQLCipher 4 par défaut.
pub const PAGE: usize = 4096;
/// Réserve SQLCipher 4 : IV (16) + HMAC-SHA512 (64).
pub const RESERVE: usize = 80;
/// En-tête du fichier WAL, puis en-tête de chaque trame.
pub const WAL_HEADER: usize = 32;
pub const WAL_FRAME_HEADER: usize = 24;

pub type Dechiffreur = cbc::Decryptor<aes::Aes256>;

pub fn wal_path(db: &Path) -> PathBuf {
    PathBuf::from(format!("{}-wal", db.display()))
}

/// Clé AES brute d'une base (`x'<64 hex>'` → 32 octets).
pub fn cle_brute(dir: &Path, base: &str) -> [u8; 32] {
    let store = DatabaseKeyStore::open(dir).expect("store");
    let (pragma, _) = store.pragma_key(base, true).expect("key");
    cle_depuis_pragma(pragma.as_str())
}

pub fn cle_depuis_pragma(pragma: &str) -> [u8; 32] {
    let hex = pragma
        .strip_prefix("x'")
        .and_then(|s| s.strip_suffix('\''))
        .expect("raw key");
    assert_eq!(hex.len(), 64, "clé brute SQLCipher attendue");
    let mut out = [0u8; 32];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).expect("hex");
    }
    out
}

/// Déchiffre une page SQLCipher (numéro `pgno`, base 1).
pub fn dechiffrer_page(cle: &[u8; 32], pgno: u32, page: &[u8]) -> Vec<u8> {
    assert_eq!(page.len(), PAGE);
    let debut = if pgno == 1 { 16 } else { 0 };
    let fin = PAGE - RESERVE;
    let iv = &page[fin..fin + 16];
    let mut clair = page[debut..fin].to_vec();
    Dechiffreur::new(cle.into(), iv.into())
        .decrypt_padded_mut::<NoPadding>(&mut clair)
        .expect("decrypt");
    clair
}

/// Toutes les pages déchiffrées du fichier principal (contrôle de la page 1 inclus).
pub fn pages_principales(cle: &[u8; 32], db: &Path) -> Vec<Vec<u8>> {
    let octets = std::fs::read(db).expect("read db");
    assert_eq!(octets.len() % PAGE, 0, "fichier de base non aligné sur les pages");
    let pages: Vec<Vec<u8>> = octets
        .chunks(PAGE)
        .enumerate()
        .map(|(i, p)| dechiffrer_page(cle, u32::try_from(i + 1).expect("pgno"), p))
        .collect();
    // Contrôle du déchiffrement : en-tête SQLite (octets 16.. du fichier) en clair.
    let p1 = &pages[0];
    assert_eq!(u16::from_be_bytes([p1[0], p1[1]]) as usize, PAGE, "taille de page");
    assert_eq!(p1[4] as usize, RESERVE, "réserve SQLCipher");
    assert_eq!(&p1[5..8], &[64, 32, 32], "fractions de charge utile");
    pages
}

/// Toutes les pages déchiffrées des trames du `-wal` (trames périmées comprises).
pub fn pages_wal(cle: &[u8; 32], db: &Path) -> Vec<Vec<u8>> {
    let Ok(octets) = std::fs::read(wal_path(db)) else {
        return Vec::new();
    };
    if octets.len() < WAL_HEADER {
        return Vec::new();
    }
    octets[WAL_HEADER..]
        .as_chunks::<{ WAL_FRAME_HEADER + PAGE }>()
        .0
        .iter()
        .map(|trame| {
            let pgno = u32::from_be_bytes([trame[0], trame[1], trame[2], trame[3]]);
            dechiffrer_page(cle, pgno, &trame[WAL_FRAME_HEADER..])
        })
        .collect()
}

pub fn contient(pages: &[Vec<u8>], temoin: &[u8]) -> bool {
    pages
        .iter()
        .any(|p| p.windows(temoin.len()).any(|w| w == temoin))
}

/// Le témoin figure-t-il dans le clair du fichier principal ou du `-wal` ?
pub fn temoin_present(cle: &[u8; 32], db: &Path, temoin: &[u8]) -> (bool, bool) {
    (
        contient(&pages_principales(cle, db), temoin),
        contient(&pages_wal(cle, db), temoin),
    )
}

