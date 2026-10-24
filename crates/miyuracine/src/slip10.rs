//! @id     miyuracine_slip10
//! @do     deriver_cles_slip10
//! @role   securite
//! @layer  toolkit
//! @human  miyuracine — module « slip10 ».

//! Dérivation SLIP-10 pour Ed25519 : dérivation **durcie uniquement**
//! (la seule définie pour Ed25519), validée par les vecteurs officiels 1 et 2
//! (`tests/vecteurs_slip10.rs`).
//!
//! - Maître : I = HMAC-SHA512(clé = "ed25519 seed", données = graine) ;
//!   clé = I[0..32], code de chaîne = I[32..64].
//! - Enfant : I = HMAC-SHA512(clé = code de chaîne, données = 0x00 ‖ clé ‖ ser32(i + 2³¹)).
//!
//! La clé obtenue est une graine Ed25519 (RFC 8032), convertie en
//! [`miyusceau::CleSignature`] par [`NoeudSlip10::cle_signature`].

use hmac::{Hmac, Mac};
use sha2::Sha512;
use zeroize::{Zeroize, Zeroizing};

use crate::erreurs::ErreurRacine;

/// Décalage de durcissement (2³¹).
pub const DURCI: u32 = 0x8000_0000;
/// Clé HMAC du nœud maître pour Ed25519.
const CLE_COURBE: &[u8] = b"ed25519 seed";

/// Nœud étendu (clé privée + code de chaîne), effacé à la destruction.
pub struct NoeudSlip10 {
    cle: Zeroizing<[u8; 32]>,
    chaine: Zeroizing<[u8; 32]>,
}

impl std::fmt::Debug for NoeudSlip10 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NoeudSlip10 { <masqué> }")
    }
}

fn hmac_sha512(cle: &[u8], morceaux: &[&[u8]]) -> NoeudSlip10 {
    // HMAC accepte toute longueur de clé : l'erreur est impossible.
    let mut mac = <Hmac<Sha512> as Mac>::new_from_slice(cle).unwrap_or_else(|_| unreachable!());
    for m in morceaux {
        mac.update(m);
    }
    let mut i = mac.finalize().into_bytes();
    let mut noeud = NoeudSlip10 {
        cle: Zeroizing::new([0u8; 32]),
        chaine: Zeroizing::new([0u8; 32]),
    };
    noeud.cle.copy_from_slice(&i[..32]);
    noeud.chaine.copy_from_slice(&i[32..]);
    i.as_mut_slice().zeroize();
    noeud
}

impl NoeudSlip10 {
    /// Nœud maître depuis une graine de 128 à 512 bits.
    pub fn maitre(graine: &[u8]) -> Result<Self, ErreurRacine> {
        if !(16..=64).contains(&graine.len()) {
            return Err(ErreurRacine::LongueurGraineInvalide);
        }
        Ok(hmac_sha512(CLE_COURBE, &[graine]))
    }

    /// Enfant durci d'indice `indice` (< 2³¹ ; le durcissement est ajouté ici).
    pub fn enfant_durci(&self, indice: u32) -> Result<Self, ErreurRacine> {
        if indice >= DURCI {
            return Err(ErreurRacine::IndiceHorsPlage);
        }
        let ser = (indice | DURCI).to_be_bytes();
        Ok(hmac_sha512(
            self.chaine.as_slice(),
            &[&[0u8], self.cle.as_slice(), &ser],
        ))
    }

    /// Dérive le chemin `m/i₁'/i₂'/…` (tous durcis) depuis ce nœud.
    pub fn deriver(&self, chemin: &[u32]) -> Result<Self, ErreurRacine> {
        let mut courant = Self {
            cle: self.cle.clone(),
            chaine: self.chaine.clone(),
        };
        for &i in chemin {
            courant = courant.enfant_durci(i)?;
        }
        Ok(courant)
    }

    /// Clé privée (graine Ed25519). À effacer après usage.
    pub fn cle_privee(&self) -> Zeroizing<[u8; 32]> {
        self.cle.clone()
    }

    /// Code de chaîne.
    pub fn code_chaine(&self) -> Zeroizing<[u8; 32]> {
        self.chaine.clone()
    }

    /// Clé de signature Ed25519 de ce nœud.
    pub fn cle_signature(&self) -> miyusceau::CleSignature {
        miyusceau::CleSignature::depuis_graine(&self.cle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn indice_non_durci_refuse() {
        let m = NoeudSlip10::maitre(&[0u8; 16]).unwrap();
        assert_eq!(
            m.enfant_durci(DURCI).unwrap_err(),
            ErreurRacine::IndiceHorsPlage
        );
        assert_eq!(
            m.deriver(&[0, DURCI + 1]).unwrap_err(),
            ErreurRacine::IndiceHorsPlage
        );
    }

    #[test]
    fn graine_hors_plage_refusee() {
        assert!(NoeudSlip10::maitre(&[0u8; 15]).is_err());
        assert!(NoeudSlip10::maitre(&[0u8; 65]).is_err());
        assert!(NoeudSlip10::maitre(&[0u8; 64]).is_ok());
    }

    #[test]
    fn debug_masque() {
        let m = NoeudSlip10::maitre(&[0u8; 16]).unwrap();
        assert_eq!(format!("{m:?}"), "NoeudSlip10 { <masqué> }");
    }
}
