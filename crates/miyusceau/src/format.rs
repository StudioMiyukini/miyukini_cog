//! @id     miyusceau_format
//! @do     sceller_verifier_sceau_v1
//! @role   securite
//! @layer  toolkit
//! @human  miyusceau — module « format ».

//! Format binaire « Sceau v1 » (big-endian) :
//!
//! ```text
//! magic "MSCE" (4) | version 0x01 (1) | type (1) | kid (32) | len (4) | charge (len) | signature (64)
//! ```
//!
//! Octets signés : `"miyukini/sceau/v1" || 0x00 || version || type || kid || len || charge`.

use ed25519_dalek::Signature;

use crate::ancres::AncresDeConfiance;
use crate::cles::{CleSignature, Kid};
use crate::erreurs::ErreurSceau;
use crate::titre::{Titre, Verifie};
use crate::tlv::{EcrivainTlv, LecteurTlv};

const MAGIC: &[u8; 4] = b"MSCE";
const VERSION: u8 = 0x01;
const DOMAINE: &[u8] = b"miyukini/sceau/v1\0";
const TAILLE_ENTETE: usize = 4 + 1 + 1 + 32 + 4;
const TAILLE_SIGNATURE: usize = 64;

/// Taille maximale de la charge utile (64 Kio).
pub const TAILLE_MAX_CHARGE: usize = 65_536;

/// Tolérance d'horloge appliquée aux fenêtres de validité (secondes).
pub const TOLERANCE_HORLOGE_SECS: u64 = 120;

/// Types de titres (séparation de domaine).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TypeSceau {
    /// Passeport (personne + appareil).
    Passeport = 0x01,
    /// Visa (droit d'entrée ciblé).
    Visa = 0x02,
    /// Manifeste de paquet.
    ManifestePaquet = 0x03,
    /// Entrée du registre.
    EntreeRegistre = 0x04,
    /// Tête signée du registre.
    TeteRegistre = 0x05,
    /// Certificat (clé d'émission, clé éditeur).
    Certificat = 0x06,
    /// Reconnaissance entre COG.
    Reconnaissance = 0x07,
}

impl TryFrom<u8> for TypeSceau {
    type Error = ErreurSceau;

    fn try_from(v: u8) -> Result<Self, ErreurSceau> {
        Ok(match v {
            0x01 => Self::Passeport,
            0x02 => Self::Visa,
            0x03 => Self::ManifestePaquet,
            0x04 => Self::EntreeRegistre,
            0x05 => Self::TeteRegistre,
            0x06 => Self::Certificat,
            0x07 => Self::Reconnaissance,
            autre => return Err(ErreurSceau::TypeInconnu(autre)),
        })
    }
}

/// Sceau décodé (signature non vérifiée : usage interne et outillage).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sceau {
    /// Type du titre.
    pub type_sceau: TypeSceau,
    /// `kid` du signataire.
    pub kid: Kid,
    /// Charge utile TLV.
    pub charge: Vec<u8>,
    /// Signature Ed25519.
    pub signature: [u8; 64],
}

impl Sceau {
    /// Décode strictement des octets (taille exacte, entête, type connu).
    pub fn decoder(octets: &[u8]) -> Result<Self, ErreurSceau> {
        if octets.len() < TAILLE_ENTETE + TAILLE_SIGNATURE {
            return Err(ErreurSceau::TailleInvalide);
        }
        if &octets[0..4] != MAGIC {
            return Err(ErreurSceau::MagicInvalide);
        }
        if octets[4] != VERSION {
            return Err(ErreurSceau::VersionNonSupportee(octets[4]));
        }
        let type_sceau = TypeSceau::try_from(octets[5])?;
        let kid: Kid = octets[6..38].try_into().map_err(|_| ErreurSceau::TailleInvalide)?;
        let longueur = u32::from_be_bytes(octets[38..42].try_into().map_err(|_| ErreurSceau::TailleInvalide)?) as usize;
        if longueur > TAILLE_MAX_CHARGE {
            return Err(ErreurSceau::ChargeTropGrande);
        }
        if octets.len() != TAILLE_ENTETE + longueur + TAILLE_SIGNATURE {
            return Err(ErreurSceau::TailleInvalide);
        }
        let charge = octets[TAILLE_ENTETE..TAILLE_ENTETE + longueur].to_vec();
        let signature: [u8; 64] = octets[TAILLE_ENTETE + longueur..]
            .try_into()
            .map_err(|_| ErreurSceau::TailleInvalide)?;
        Ok(Self { type_sceau, kid, charge, signature })
    }

    /// Encode en octets.
    pub fn encoder(&self) -> Vec<u8> {
        let mut sortie = Vec::with_capacity(TAILLE_ENTETE + self.charge.len() + TAILLE_SIGNATURE);
        sortie.extend_from_slice(MAGIC);
        sortie.push(VERSION);
        sortie.push(self.type_sceau as u8);
        sortie.extend_from_slice(&self.kid);
        sortie.extend_from_slice(&(self.charge.len() as u32).to_be_bytes());
        sortie.extend_from_slice(&self.charge);
        sortie.extend_from_slice(&self.signature);
        sortie
    }

    /// Octets couverts par la signature.
    fn octets_signes(type_sceau: TypeSceau, kid: &Kid, charge: &[u8]) -> Vec<u8> {
        let mut m = Vec::with_capacity(DOMAINE.len() + 2 + 32 + 4 + charge.len());
        m.extend_from_slice(DOMAINE);
        m.push(VERSION);
        m.push(type_sceau as u8);
        m.extend_from_slice(kid);
        m.extend_from_slice(&(charge.len() as u32).to_be_bytes());
        m.extend_from_slice(charge);
        m
    }
}

/// Scelle un titre avec une clé de signature.
pub fn sceller<T: Titre>(titre: &T, cle: &CleSignature) -> Result<Vec<u8>, ErreurSceau> {
    let mut ecrivain = EcrivainTlv::new();
    titre.encoder(&mut ecrivain)?;
    let charge = ecrivain.finir();
    if charge.len() > TAILLE_MAX_CHARGE {
        return Err(ErreurSceau::ChargeTropGrande);
    }
    let kid = cle.kid();
    let signature = cle.signer(&Sceau::octets_signes(T::TYPE, &kid, &charge));
    Ok(Sceau { type_sceau: T::TYPE, kid, charge, signature }.encoder())
}

/// Vérifie un sceau et décode son titre (fail-closed).
///
/// Ordre : format strict → type attendu → signataire épinglé → `verify_strict`
/// → décodage canonique complet → fenêtre de validité (± tolérance).
pub fn verifier<T: Titre>(
    octets: &[u8],
    ancres: &AncresDeConfiance,
    maintenant_secs: u64,
) -> Result<Verifie<T>, ErreurSceau> {
    let sceau = Sceau::decoder(octets)?;
    if sceau.type_sceau != T::TYPE {
        return Err(ErreurSceau::TypeInattendu { attendu: T::TYPE as u8, trouve: sceau.type_sceau as u8 });
    }
    let (cle, niveau) = ancres.trouver(&sceau.kid).ok_or(ErreurSceau::SignataireInconnu)?;
    let message = Sceau::octets_signes(sceau.type_sceau, &sceau.kid, &sceau.charge);
    cle.verify_strict(&message, &Signature::from_bytes(&sceau.signature))
        .map_err(|_| ErreurSceau::SignatureInvalide)?;

    crate::tlv::valider_structure(&sceau.charge)?;
    let mut lecteur = LecteurTlv::new(&sceau.charge);
    let titre = T::decoder(&mut lecteur)?;
    lecteur.terminer()?;

    if let Some((pas_avant, pas_apres)) = titre.fenetre_validite() {
        if maintenant_secs.saturating_add(TOLERANCE_HORLOGE_SECS) < pas_avant {
            return Err(ErreurSceau::PasEncoreValide);
        }
        if maintenant_secs.saturating_sub(TOLERANCE_HORLOGE_SECS) > pas_apres {
            return Err(ErreurSceau::Expire);
        }
    }
    Ok(Verifie::nouveau(titre, sceau.kid, *niveau))
}
