//! @id     miyufederation_calcul
//! @do     calculer_verifier_preuve_hashcash
//! @role   securite
//! @layer  toolkit
//! @human  miyufederation — module « calcul » (preuve de calcul légère de candidature, hashcash SHA-256, EX-38).

//! Preuve de calcul légère (EX-38, zones §16 « hashcash contre la création en
//! masse »).
//!
//! La doc gelée ne fixe ni la fonction ni la difficulté : choix consignés pour
//! un amendement.
//!
//! - Condensé : `SHA-256("miyukini/federation/candidature/v1" || 0x00 || racine (32) || jour (8, BE) || nonce (8, BE))`
//! - Preuve valide : au moins `difficulte` bits nuls en tête du condensé
//! - `jour` = `emise_a / 86 400` : la preuve est **liée** à la clé d'identité
//!   et au jour de la candidature (pas de réemploi pour une autre identité, ni
//!   au-delà de la validité de la candidature)
//! - Peu de mémoire (SHA-256 seul) : faisable sur un petit serveur
//! - Difficulté par défaut [`DIFFICULTE_DEFAUT`] = 22 bits (≈ 4 millions de
//!   condensés en moyenne, quelques secondes en compilation optimisée)

use sha2::{Digest, Sha256};

/// Difficulté par défaut (bits nuls en tête).
pub const DIFFICULTE_DEFAUT: u8 = 22;
/// Difficulté minimale admise par une politique (en dessous : refus de la politique).
pub const DIFFICULTE_PLANCHER: u8 = 8;
/// Difficulté maximale (au-delà : candidature mal formée).
pub const DIFFICULTE_MAX: u8 = 40;

const DOMAINE: &[u8] = b"miyukini/federation/candidature/v1\0";

/// Jour de calcul d'un horodatage (secondes Unix).
pub const fn jour_de(emise_a: u64) -> u64 {
    emise_a / 86_400
}

/// Condensé hashcash d'un essai.
pub fn condense(racine: &[u8; 32], jour: u64, nonce: u64) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(DOMAINE);
    h.update(racine);
    h.update(jour.to_be_bytes());
    h.update(nonce.to_be_bytes());
    h.finalize().into()
}

/// Nombre de bits nuls en tête.
pub fn bits_nuls_en_tete(octets: &[u8; 32]) -> u32 {
    let mut n = 0;
    for o in octets {
        if *o == 0 {
            n += 8;
        } else {
            n += o.leading_zeros();
            break;
        }
    }
    n
}

/// Vrai si `nonce` est une preuve valide de `difficulte` bits.
pub fn verifier(racine: &[u8; 32], jour: u64, nonce: u64, difficulte: u8) -> bool {
    difficulte <= DIFFICULTE_MAX && bits_nuls_en_tete(&condense(racine, jour, nonce)) >= u32::from(difficulte)
}

/// Cherche le premier nonce valide (au plus `essais_max` essais).
pub fn resoudre(racine: &[u8; 32], jour: u64, difficulte: u8, essais_max: u64) -> Option<u64> {
    if difficulte > DIFFICULTE_MAX {
        return None;
    }
    (0..essais_max).find(|n| verifier(racine, jour, *n, difficulte))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preuve_liee_a_la_racine_et_au_jour() {
        let racine = [3u8; 32];
        let nonce = resoudre(&racine, 20_000, 10, 1 << 20).unwrap();
        assert!(verifier(&racine, 20_000, nonce, 10));
        // Autre jour ou autre identité : la même preuve ne vaut (presque) jamais.
        let ailleurs = (0..64u8).filter(|i| verifier(&[*i; 32], 20_000, nonce, 10)).count();
        assert!(ailleurs <= 2);
        assert!(!verifier(&racine, 20_000, nonce, DIFFICULTE_MAX + 1));
    }

    #[test]
    fn bits_en_tete() {
        let mut o = [0xffu8; 32];
        assert_eq!(bits_nuls_en_tete(&o), 0);
        o[0] = 0;
        o[1] = 0x10;
        assert_eq!(bits_nuls_en_tete(&o), 11);
        assert_eq!(bits_nuls_en_tete(&[0; 32]), 256);
    }
}
