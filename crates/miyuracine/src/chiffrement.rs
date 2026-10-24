//! @id     miyuracine_chiffrement
//! @do     chiffrer_secret_maitre
//! @role   securite
//! @layer  toolkit
//! @human  miyuracine — module « chiffrement ».

//! Chiffrement du secret maître SLIP-39 : réseau de Feistel à 4 tours
//! (Luby-Rackoff), fonction de tour PBKDF2-HMAC-SHA256
//! (sections « Encryption / Decryption of the master secret »).
//!
//! F(i, R) = PBKDF2(HMAC-SHA256, mot de passe = i ‖ phrase,
//!                  sel = préfixe ‖ R, itérations = 2500 << e, n/2 octets)
//! préfixe = "" si ext = 1, sinon "shamir" ‖ id (2 octets gros-boutiste).

use sha2::Sha256;
use zeroize::Zeroizing;

use crate::erreurs::ErreurRacine;

/// Itérations de base par tour (10 000 au total pour e = 0, 4 tours).
const ITERATIONS_PAR_TOUR: u32 = 2500;
/// Nombre de tours du réseau de Feistel.
const TOURS: u8 = 4;
/// Exposant d'itération maximal (champ de 4 bits).
pub(crate) const EXPOSANT_MAX: u8 = 15;

/// Contrôle la phrase secrète : ASCII imprimable (32 à 126) uniquement.
pub(crate) fn verifier_phrase(phrase: &[u8]) -> Result<(), ErreurRacine> {
    if phrase.iter().all(|&c| (32..=126).contains(&c)) {
        Ok(())
    } else {
        Err(ErreurRacine::PhraseSecreteInvalide)
    }
}

fn sel(identifiant: u16, extensible: bool) -> Vec<u8> {
    if extensible {
        Vec::new()
    } else {
        let mut s = b"shamir".to_vec();
        s.extend_from_slice(&identifiant.to_be_bytes());
        s
    }
}

fn tour(i: u8, phrase: &[u8], exposant: u8, sel: &[u8], droite: &[u8]) -> Zeroizing<Vec<u8>> {
    let mut mot_de_passe = Zeroizing::new(Vec::with_capacity(1 + phrase.len()));
    mot_de_passe.push(i);
    mot_de_passe.extend_from_slice(phrase);
    let mut sel_complet = Zeroizing::new(Vec::with_capacity(sel.len() + droite.len()));
    sel_complet.extend_from_slice(sel);
    sel_complet.extend_from_slice(droite);
    let mut sortie = Zeroizing::new(vec![0u8; droite.len()]);
    pbkdf2::pbkdf2_hmac::<Sha256>(
        &mot_de_passe,
        &sel_complet,
        ITERATIONS_PAR_TOUR << exposant,
        &mut sortie,
    );
    sortie
}

fn feistel(
    entree: &[u8],
    phrase: &[u8],
    exposant: u8,
    identifiant: u16,
    extensible: bool,
    ordre: &[u8],
) -> Result<Zeroizing<Vec<u8>>, ErreurRacine> {
    if entree.is_empty() || entree.len() & 1 != 0 {
        return Err(ErreurRacine::LongueurSecretInvalide);
    }
    if exposant > EXPOSANT_MAX {
        return Err(ErreurRacine::ParametresInvalides(
            "exposant d'itération > 15",
        ));
    }
    verifier_phrase(phrase)?;
    let moitie = entree.len() / 2;
    let sel = sel(identifiant, extensible);
    let mut gauche = Zeroizing::new(entree[..moitie].to_vec());
    let mut droite = Zeroizing::new(entree[moitie..].to_vec());
    for &i in ordre {
        let f = tour(i, phrase, exposant, &sel, &droite);
        // (L, R) ← (R, L ⊕ F(i, R))
        let mut nouvelle = Zeroizing::new(vec![0u8; moitie]);
        for ((n, l), f) in nouvelle.iter_mut().zip(gauche.iter()).zip(f.iter()) {
            *n = l ^ f;
        }
        gauche = droite;
        droite = nouvelle;
    }
    let mut sortie = Zeroizing::new(Vec::with_capacity(entree.len()));
    sortie.extend_from_slice(&droite);
    sortie.extend_from_slice(&gauche);
    Ok(sortie)
}

/// EMS = Encrypt(MS, P, e, id, ext).
pub(crate) fn chiffrer(
    secret_maitre: &[u8],
    phrase: &[u8],
    exposant: u8,
    identifiant: u16,
    extensible: bool,
) -> Result<Zeroizing<Vec<u8>>, ErreurRacine> {
    let ordre: Vec<u8> = (0..TOURS).collect();
    feistel(
        secret_maitre,
        phrase,
        exposant,
        identifiant,
        extensible,
        &ordre,
    )
}

/// MS = Decrypt(EMS, P, e, id, ext).
pub(crate) fn dechiffrer(
    chiffre: &[u8],
    phrase: &[u8],
    exposant: u8,
    identifiant: u16,
    extensible: bool,
) -> Result<Zeroizing<Vec<u8>>, ErreurRacine> {
    let ordre: Vec<u8> = (0..TOURS).rev().collect();
    feistel(chiffre, phrase, exposant, identifiant, extensible, &ordre)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aller_retour() {
        let ms: Vec<u8> = (0..32u8).collect();
        for ext in [false, true] {
            let c = chiffrer(&ms, b"TREZOR", 0, 0x1234, ext).unwrap();
            assert_ne!(c.as_slice(), ms.as_slice());
            assert_eq!(
                dechiffrer(&c, b"TREZOR", 0, 0x1234, ext)
                    .unwrap()
                    .as_slice(),
                ms.as_slice()
            );
            // Mauvaise phrase : autre secret (aucune vérification possible, par conception).
            assert_ne!(
                dechiffrer(&c, b"autre", 0, 0x1234, ext).unwrap().as_slice(),
                ms.as_slice()
            );
        }
    }

    #[test]
    fn phrase_hors_ascii_refusee() {
        assert_eq!(
            chiffrer(&[0u8; 16], "é".as_bytes(), 0, 0, true).unwrap_err(),
            ErreurRacine::PhraseSecreteInvalide
        );
        assert_eq!(
            chiffrer(&[0u8; 16], b"a\nb", 0, 0, true).unwrap_err(),
            ErreurRacine::PhraseSecreteInvalide
        );
    }
}
