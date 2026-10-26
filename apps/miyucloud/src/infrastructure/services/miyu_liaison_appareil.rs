//! @id     miyucloud_infrastructure_services_miyu_liaison_appareil
//! @do     verifier_liaison_cle_appareil
//! @role   securite
//! @layer  app
//! @human  miyucloud — demande de liaison d'une clé d'appareil (format v2, preuve de possession, rotation signée par l'ancienne clé).

//! Demande de liaison d'une clé d'appareil à un compte MiyuCloud (I6).
//!
//! Le message signé est celui de la **liaison d'appareil v2** de
//! [`miyauth::appareil`] (A-013) :
//!
//! ```text
//! "miyukini/miyauth/liaison-appareil/v2" || 0x00 || cog (26) || cog (26)
//!     || clé nouvelle (32) || pas_apres (u64 gros-boutiste, 8)
//! ```
//!
//! avec `origine = destination = cog_id` du COG d'origine épinglé par MiyuCloud
//! (MiyuCloud est un service de ce COG ; la clé est celle que l'appareil dérive
//! pour cette destination, `miyauth::cle_appareil_pour`).
//!
//! Transport (base64 URL sans remplissage), format `MLA2` :
//!
//! | champ | octets |
//! |---|---|
//! | `"MLA2"` | 4 |
//! | clé nouvelle | 32 |
//! | `pas_apres` (BE) | 8 |
//! | signature du message par la clé nouvelle (**possession**) | 64 |
//! | *rotation seulement* : clé ancienne | 32 |
//! | *rotation seulement* : signature du message par la clé ancienne (**autorisation**) | 64 |
//!
//! Soit 108 octets (liaison depuis une session de confiance) ou 204 octets
//! (rotation : la clé ancienne autorise sa remplaçante). Refus : échéance
//! passée ou à plus de 5 min (+ tolérance d'horloge), clé faible, signature
//! invalide (`verify_strict`), clé ancienne égale à la nouvelle, autre longueur.

use ed25519_dalek::{Signature, VerifyingKey};
use miyauth::CogId;

/// En-tête du format de transport.
pub const MAGIC_LIAISON: [u8; 4] = *b"MLA2";
/// Longueur d'une demande de liaison simple.
pub const TAILLE_LIAISON: usize = 4 + 32 + 8 + 64;
/// Longueur d'une demande de rotation (liaison + clé ancienne et sa signature).
pub const TAILLE_ROTATION: usize = TAILLE_LIAISON + 32 + 64;
/// Longueur maximale du texte base64 URL accepté.
pub const TAILLE_MAX_TEXTE: usize = TAILLE_ROTATION.div_ceil(3) * 4;

/// Refus d'une demande de liaison (toute variante = refus).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefusLiaison {
    /// Texte non décodable ou longueur / en-tête inattendus.
    Format,
    /// Échéance passée ou trop lointaine.
    Echeance,
    /// Clé faible ou invalide, ou clé ancienne égale à la nouvelle.
    Cle,
    /// Signature de possession ou d'autorisation invalide.
    Signature,
}

impl std::fmt::Display for RefusLiaison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Format => "demande de liaison mal formée (MLA2 attendu)",
            Self::Echeance => "demande de liaison expirée ou d'échéance trop lointaine",
            Self::Cle => "clé d'appareil invalide",
            Self::Signature => "preuve de liaison invalide",
        })
    }
}

/// Demande de liaison décodée (non vérifiée).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DemandeLiaisonAppareil {
    /// Clé publique d'appareil à lier.
    pub cle: [u8; 32],
    /// Échéance (secondes Unix).
    pub pas_apres: u64,
    /// Signature du message par `cle` (possession).
    pub preuve: [u8; 64],
    /// Rotation : clé ancienne et sa signature du même message (autorisation).
    pub autorisation: Option<([u8; 32], [u8; 64])>,
}

/// Octets signés (message de liaison d'appareil v2, origine = destination = `cog`).
pub fn message(cog: &CogId, cle: &[u8; 32], pas_apres: u64) -> Vec<u8> {
    miyauth::message_liaison(cog, cog, cle, pas_apres)
}

fn cle_valide(cle: &[u8; 32]) -> Option<VerifyingKey> {
    VerifyingKey::from_bytes(cle).ok().filter(|k| !k.is_weak())
}

fn verifier_signature(
    cle: &VerifyingKey,
    message: &[u8],
    sig: &[u8; 64],
) -> Result<(), RefusLiaison> {
    cle.verify_strict(message, &Signature::from_bytes(sig))
        .map_err(|_| RefusLiaison::Signature)
}

impl DemandeLiaisonAppareil {
    /// Décode le texte base64 URL (`MLA2`, 108 ou 204 octets).
    pub fn decoder(texte: &str) -> Result<Self, RefusLiaison> {
        let t = texte.trim();
        if t.is_empty() || t.len() > TAILLE_MAX_TEXTE {
            return Err(RefusLiaison::Format);
        }
        let o = miyauth::http::b64_decoder(t).ok_or(RefusLiaison::Format)?;
        if (o.len() != TAILLE_LIAISON && o.len() != TAILLE_ROTATION) || o[..4] != MAGIC_LIAISON {
            return Err(RefusLiaison::Format);
        }
        let fixe = |debut: usize, fin: usize| o[debut..fin].to_vec();
        let vers32 = |v: Vec<u8>| <[u8; 32]>::try_from(v).map_err(|_| RefusLiaison::Format);
        let vers64 = |v: Vec<u8>| <[u8; 64]>::try_from(v).map_err(|_| RefusLiaison::Format);
        let cle = vers32(fixe(4, 36))?;
        let mut e = [0u8; 8];
        e.copy_from_slice(&o[36..44]);
        let preuve = vers64(fixe(44, 108))?;
        let autorisation = if o.len() == TAILLE_ROTATION {
            Some((vers32(fixe(108, 140))?, vers64(fixe(140, 204))?))
        } else {
            None
        };
        Ok(Self {
            cle,
            pas_apres: u64::from_be_bytes(e),
            preuve,
            autorisation,
        })
    }

    /// Encode au format de transport.
    pub fn encoder(&self) -> String {
        let mut o = Vec::with_capacity(TAILLE_ROTATION);
        o.extend_from_slice(&MAGIC_LIAISON);
        o.extend_from_slice(&self.cle);
        o.extend_from_slice(&self.pas_apres.to_be_bytes());
        o.extend_from_slice(&self.preuve);
        if let Some((ancienne, sig)) = &self.autorisation {
            o.extend_from_slice(ancienne);
            o.extend_from_slice(sig);
        }
        miyauth::http::b64_encoder(&o)
    }

    /// Vérifie échéance, clés et signatures pour le COG `cog` à `maintenant`.
    /// La clé ancienne (rotation) doit en plus être une clé active du compte :
    /// contrôle à la charge de l'appelant.
    pub fn verifier(&self, cog: &CogId, maintenant: u64) -> Result<(), RefusLiaison> {
        let limite = maintenant
            .saturating_add(miyauth::DUREE_MAX_LIAISON_SECS)
            .saturating_add(miyusceau::TOLERANCE_HORLOGE_SECS);
        if self.pas_apres < maintenant || self.pas_apres > limite {
            return Err(RefusLiaison::Echeance);
        }
        let nouvelle = cle_valide(&self.cle).ok_or(RefusLiaison::Cle)?;
        let m = message(cog, &self.cle, self.pas_apres);
        verifier_signature(&nouvelle, &m, &self.preuve)?;
        if let Some((ancienne, sig)) = &self.autorisation {
            if ancienne == &self.cle {
                return Err(RefusLiaison::Cle);
            }
            let ancienne = cle_valide(ancienne).ok_or(RefusLiaison::Cle)?;
            verifier_signature(&ancienne, &m, sig)?;
        }
        Ok(())
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};

    pub(crate) fn signer(graine: &[u8; 32], m: &[u8]) -> [u8; 64] {
        SigningKey::from_bytes(graine).sign(m).to_bytes()
    }

    pub(crate) fn publique(graine: &[u8; 32]) -> [u8; 32] {
        SigningKey::from_bytes(graine).verifying_key().to_bytes()
    }

    /// Demande signée par `nouvelle` (et `ancienne` si fournie).
    pub(crate) fn demande(
        cog: &CogId,
        nouvelle: &[u8; 32],
        ancienne: Option<&[u8; 32]>,
        pas_apres: u64,
    ) -> DemandeLiaisonAppareil {
        let cle = publique(nouvelle);
        let m = message(cog, &cle, pas_apres);
        DemandeLiaisonAppareil {
            cle,
            pas_apres,
            preuve: signer(nouvelle, &m),
            autorisation: ancienne.map(|a| (publique(a), signer(a, &m))),
        }
    }

    fn cog() -> CogId {
        CogId::new(&miyusceau::cog_id_depuis_cle(&publique(&[3; 32]))).unwrap()
    }

    /// Vecteur de référence partagé avec `apps/central-mobile` (même texte attendu).
    pub(crate) const VECTEUR_ROTATION: &str = include_str!("miyu_liaison_appareil.vecteur");

    #[test]
    fn shared_test_vector_matches_mobile_encoding() {
        let d = demande(&cog(), &[1; 32], Some(&[2; 32]), 1_800_000_000);
        assert_eq!(d.encoder(), VECTEUR_ROTATION.trim());
        assert_eq!(
            DemandeLiaisonAppareil::decoder(VECTEUR_ROTATION).unwrap(),
            d
        );
        assert!(d.verifier(&cog(), 1_800_000_000 - 60).is_ok());
    }

    #[test]
    fn link_and_rotation_roundtrip_and_verify() {
        let t = 1_000_000;
        for ancienne in [None, Some(&[2u8; 32])] {
            let d = demande(&cog(), &[1; 32], ancienne, t + 60);
            let texte = d.encoder();
            let lu = DemandeLiaisonAppareil::decoder(&texte).unwrap();
            assert_eq!(lu, d);
            assert!(lu.verifier(&cog(), t).is_ok());
        }
    }

    #[test]
    fn deadline_is_enforced() {
        let t = 1_000_000;
        let passee = demande(&cog(), &[1; 32], None, t - 1);
        assert_eq!(passee.verifier(&cog(), t), Err(RefusLiaison::Echeance));
        let loin = demande(
            &cog(),
            &[1; 32],
            None,
            t + miyauth::DUREE_MAX_LIAISON_SECS + miyusceau::TOLERANCE_HORLOGE_SECS + 1,
        );
        assert_eq!(loin.verifier(&cog(), t), Err(RefusLiaison::Echeance));
    }

    #[test]
    fn signatures_bind_cog_key_and_deadline() {
        let t = 1_000_000;
        let d = demande(&cog(), &[1; 32], Some(&[2; 32]), t + 60);
        // Autre COG.
        let autre = CogId::new(&miyusceau::cog_id_depuis_cle(&publique(&[9; 32]))).unwrap();
        assert_eq!(d.verifier(&autre, t), Err(RefusLiaison::Signature));
        // Échéance modifiée.
        let mut e = d.clone();
        e.pas_apres += 1;
        assert_eq!(e.verifier(&cog(), t), Err(RefusLiaison::Signature));
        // Clé substituée (possession d'autrui).
        let mut e = d.clone();
        e.cle = publique(&[7; 32]);
        assert_eq!(e.verifier(&cog(), t), Err(RefusLiaison::Signature));
        // Autorisation par une autre clé que celle annoncée.
        let mut e = d.clone();
        e.autorisation = Some((publique(&[8; 32]), d.autorisation.unwrap().1));
        assert_eq!(e.verifier(&cog(), t), Err(RefusLiaison::Signature));
        // Ancienne = nouvelle.
        let m = message(&cog(), &publique(&[1; 32]), t + 60);
        let mut e = d.clone();
        e.autorisation = Some((publique(&[1; 32]), signer(&[1; 32], &m)));
        assert_eq!(e.verifier(&cog(), t), Err(RefusLiaison::Cle));
    }

    #[test]
    fn malformed_inputs_are_refused() {
        let d = demande(&cog(), &[1; 32], None, 5);
        let mut o = miyauth::http::b64_decoder(&d.encoder()).unwrap();
        for texte in [
            String::new(),
            "pas du base64 !".to_string(),
            miyauth::http::b64_encoder(&o[..100]),
            "A".repeat(TAILLE_MAX_TEXTE + 1),
        ] {
            assert_eq!(
                DemandeLiaisonAppareil::decoder(&texte),
                Err(RefusLiaison::Format)
            );
        }
        o[0] = b'X';
        assert_eq!(
            DemandeLiaisonAppareil::decoder(&miyauth::http::b64_encoder(&o)),
            Err(RefusLiaison::Format)
        );
        // Clé faible (point d'ordre faible) : refusée.
        let mut faible = d.clone();
        faible.cle = [0; 32];
        faible.cle[0] = 1;
        assert_eq!(faible.verifier(&cog(), 0), Err(RefusLiaison::Cle));
    }
}
