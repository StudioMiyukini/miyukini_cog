//! @id     miyuracine_tests_vecteurs_slip39
//! @do     verifier_vecteurs_officiels_slip39
//! @role   test
//! @layer  toolkit
//! @human  miyuracine — vecteurs officiels SLIP-39.

//! Vecteurs officiels SLIP-39 : `vectors.json` du dépôt de référence
//! <https://github.com/trezor/python-shamir-mnemonic> (lien donné par la
//! section « Test vectors » de SLIP-0039), copié à l'identique dans
//! `tests/vecteurs/slip39-vectors.json` (SHA-256 contrôlé ci-dessous).
//!
//! Chaque vecteur : (description, parts, secret maître hexadécimal, xprv BIP-32).
//! Secret vide = la combinaison doit échouer. Phrase secrète : "TREZOR".
//! Le xprv (BIP-32 secp256k1) est hors champ : seul le secret maître est comparé.

use miyuracine::slip39::combiner_parts;
use sha2::{Digest, Sha256};
use std::fmt::Write as _;

const VECTEURS: &str = include_str!("vecteurs/slip39-vectors.json");

fn hex(octets: &[u8]) -> String {
    octets.iter().fold(String::new(), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    })
}

#[test]
fn fichier_de_vecteurs_officiel_inchange() {
    assert_eq!(
        hex(&Sha256::digest(VECTEURS.as_bytes())),
        "13ebecebdd869dd2bc2cdf69e7ce3a158cf106cac76c39d17682b1c6cdabbdc4"
    );
}

#[test]
fn tous_les_vecteurs_officiels() {
    let vecteurs: serde_json::Value = serde_json::from_str(VECTEURS).unwrap();
    let liste = vecteurs.as_array().unwrap();
    assert_eq!(liste.len(), 45, "45 vecteurs officiels");
    let (mut valides, mut invalides) = (0, 0);
    for v in liste {
        let description = v[0].as_str().unwrap();
        let parts: Vec<&str> = v[1]
            .as_array()
            .unwrap()
            .iter()
            .map(|m| m.as_str().unwrap())
            .collect();
        let attendu = v[2].as_str().unwrap();
        let resultat = combiner_parts(&parts, b"TREZOR");
        if attendu.is_empty() {
            assert!(
                resultat.is_err(),
                "{description} : la combinaison aurait dû échouer"
            );
            invalides += 1;
        } else {
            let secret = resultat.unwrap_or_else(|e| panic!("{description} : {e}"));
            assert_eq!(hex(&secret), attendu, "{description}");
            valides += 1;
        }
    }
    assert_eq!((valides, invalides), (15, 30));
}

#[test]
fn motifs_de_refus_attendus() {
    use miyuracine::ErreurRacine as E;
    let vecteurs: serde_json::Value = serde_json::from_str(VECTEURS).unwrap();
    let refus = |n: usize| {
        let v = &vecteurs[n - 1];
        let parts: Vec<&str> = v[1]
            .as_array()
            .unwrap()
            .iter()
            .map(|m| m.as_str().unwrap())
            .collect();
        combiner_parts(&parts, b"TREZOR").unwrap_err()
    };
    assert_eq!(refus(2), E::SommeControleInvalide);
    assert_eq!(refus(3), E::RemplissageInvalide);
    assert_eq!(refus(5), E::NombrePartsInvalide);
    assert_eq!(refus(6), E::PartsIncompatibles);
    assert_eq!(refus(7), E::PartsIncompatibles);
    assert_eq!(refus(11), E::IndicesDupliques);
    assert_eq!(refus(12), E::SeuilsMembresDifferents);
    assert_eq!(refus(13), E::EmpreinteInvalide);
}
