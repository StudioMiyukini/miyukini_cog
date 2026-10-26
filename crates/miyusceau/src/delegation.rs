//! @id     miyusceau_delegation
//! @do     sceller_verifier_delegation_cle_usage
//! @role   securite
//! @layer  toolkit
//! @human  miyusceau — délégation de clé d'usage (certificat 0x06, nature 0x45, A-022 §2).

//! # Délégation de clé d'usage — certificat `0x06`, nature `0x45` (« E »)
//!
//! Amendement A-022 §2 : la **clé d'identité** d'un COG (indice SLIP-10 0,
//! racine hors ligne) certifie, pendant la cérémonie, chaque clé d'usage
//! d'indice ≥ 1 (passeports, visas, journal, éditeur, arrêtés, Fiches).
//! Tout vérificateur remonte ainsi à la racine épinglée ; une clé d'usage
//! sans délégation valide est refusée.
//!
//! Comme les natures `0x41`–`0x44`, la délégation est un
//! `TypeSceau::Certificat` (0x06) dont le premier champ `nature` vaut
//! [`NATURE_DELEGATION`].
//!
//! ## Charge TLV (tags croissants, tous obligatoires, aucun autre admis)
//!
//! | Tag  | Champ        | Valeur                                                        |
//! |------|--------------|---------------------------------------------------------------|
//! | 0x01 | `nature`     | 1 octet, `0x45`                                               |
//! | 0x02 | `cog_id`     | texte, 26 caractères base32 minuscules (= `cog_id` de la clé signataire) |
//! | 0x03 | `indice`     | `u64`, `1 ≤ indice < 2³¹` (l'identité, 0, ne se délègue pas) |
//! | 0x04 | `generation` | `u64`, `< 2³¹` (indice SLIP-10 durci)                         |
//! | 0x05 | `cle`        | clé publique Ed25519 (32 o), point valide et non faible       |
//! | 0x06 | `pas_avant`  | secondes Unix                                                 |
//! | 0x07 | `pas_apres`  | secondes Unix, `pas_avant < pas_apres ≤ pas_avant + 90 j`     |
//!
//! Signée par **une** clé : l'identité du COG `cog_id`
//! ([`verifier_delegation`] l'exige épinglée, compte exactement ce signataire
//! et contrôle `cog_id = cog_id(clé d'identité)`).
//!
//! Règle d'encodage (même règle qu'A-020) : les contrôles de
//! [`DelegationCle::nouvelle`], de l'encodeur et du décodeur sont **les
//! mêmes** ; tout ce que l'encodeur accepte, le décodeur l'accepte, et
//! l'aller-retour est exact (contrôlé à chaque scellement par `miyusceau`).

use ed25519_dalek::VerifyingKey;
use sha2::{Digest, Sha256};

use crate::ancres::{AncresDeConfiance, NiveauConfiance};
use crate::cles::{empreinte_cle, CleSignature};
use crate::erreurs::ErreurSceau;
use crate::format::{sceller, verifier, TypeSceau, TOLERANCE_HORLOGE_SECS};
use crate::titre::{Titre, Verifie};
use crate::tlv::{EcrivainTlv, LecteurTlv};

/// Valeur du champ `nature` d'un certificat de délégation de clé d'usage (« E »).
pub const NATURE_DELEGATION: u8 = 0x45;
/// Durée maximale d'une délégation : 90 jours (rythme de rotation des clés d'usage).
pub const DUREE_MAX_DELEGATION_SECS: u64 = 90 * 86_400;
/// Borne exclusive des indices et générations (indices SLIP-10 durcis).
pub const BORNE_INDICE: u64 = 1 << 31;
/// Longueur d'un `cog_id` (26 caractères base32 = 130 bits).
pub const LONGUEUR_COG_ID: usize = 26;

const ALPHABET_BASE32: &[u8; 32] = b"abcdefghijklmnopqrstuvwxyz234567";

const TAG_NATURE: u8 = 0x01;
const TAG_COG_ID: u8 = 0x02;
const TAG_INDICE: u8 = 0x03;
const TAG_GENERATION: u8 = 0x04;
const TAG_CLE: u8 = 0x05;
const TAG_PAS_AVANT: u8 = 0x06;
const TAG_PAS_APRES: u8 = 0x07;

/// `cog_id` d'une clé publique d'identité : `base32(SHA-256(clé))[0..26]`
/// (RFC 4648, minuscules, sans remplissage).
pub fn cog_id_depuis_cle(cle_publique: &[u8; 32]) -> String {
    let empreinte: [u8; 32] = Sha256::digest(cle_publique).into();
    let mut sortie = String::with_capacity(LONGUEUR_COG_ID);
    let mut tampon: u32 = 0;
    let mut bits: u32 = 0;
    for octet in empreinte {
        tampon = (tampon << 8) | u32::from(octet);
        bits += 8;
        while bits >= 5 && sortie.len() < LONGUEUR_COG_ID {
            bits -= 5;
            sortie.push(char::from(
                ALPHABET_BASE32[((tampon >> bits) & 0x1f) as usize],
            ));
        }
        tampon &= (1 << bits) - 1;
        if sortie.len() >= LONGUEUR_COG_ID {
            break;
        }
    }
    sortie
}

fn cog_id_valide(c: &str) -> bool {
    c.len() == LONGUEUR_COG_ID && c.bytes().all(|b| ALPHABET_BASE32.contains(&b))
}

fn cle_valide(cle: &[u8; 32]) -> bool {
    VerifyingKey::from_bytes(cle).is_ok_and(|k| !k.is_weak())
}

/// Délégation d'une clé d'usage par la clé d'identité d'un COG.
///
/// Champs privés : seul [`DelegationCle::nouvelle`] (ou le décodeur, qui
/// applique les mêmes contrôles) en construit une.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelegationCle {
    cog_id: String,
    indice: u32,
    generation: u32,
    cle_publique: [u8; 32],
    pas_avant: u64,
    pas_apres: u64,
}

impl DelegationCle {
    /// Construit une délégation après contrôle de **tous** les invariants du
    /// format (ceux du décodeur) : `cog_id` bien formé, `1 ≤ indice < 2³¹`,
    /// `generation < 2³¹`, clé Ed25519 valide et non faible,
    /// `pas_avant < pas_apres ≤ pas_avant + 90 j`.
    pub fn nouvelle(
        cog_id: &str,
        indice: u32,
        generation: u32,
        cle_publique: [u8; 32],
        pas_avant: u64,
        pas_apres: u64,
    ) -> Result<Self, ErreurSceau> {
        let d = Self {
            cog_id: cog_id.to_owned(),
            indice,
            generation,
            cle_publique,
            pas_avant,
            pas_apres,
        };
        d.controler()?;
        Ok(d)
    }

    /// Invariants communs à la construction, à l'encodage et au décodage.
    fn controler(&self) -> Result<(), ErreurSceau> {
        if !cog_id_valide(&self.cog_id) {
            return Err(ErreurSceau::ValeurInvalide(TAG_COG_ID));
        }
        if self.indice == 0 || u64::from(self.indice) >= BORNE_INDICE {
            return Err(ErreurSceau::ValeurInvalide(TAG_INDICE));
        }
        if u64::from(self.generation) >= BORNE_INDICE {
            return Err(ErreurSceau::ValeurInvalide(TAG_GENERATION));
        }
        if !cle_valide(&self.cle_publique) {
            return Err(ErreurSceau::ValeurInvalide(TAG_CLE));
        }
        if self.pas_apres <= self.pas_avant
            || self.pas_apres - self.pas_avant > DUREE_MAX_DELEGATION_SECS
        {
            return Err(ErreurSceau::ValeurInvalide(TAG_PAS_APRES));
        }
        Ok(())
    }

    /// `cog_id` du COG délégant.
    pub fn cog_id(&self) -> &str {
        &self.cog_id
    }

    /// Indice d'usage SLIP-10 (≥ 1).
    pub fn indice(&self) -> u32 {
        self.indice
    }

    /// Génération de la clé d'usage.
    pub fn generation(&self) -> u32 {
        self.generation
    }

    /// Clé publique déléguée.
    pub fn cle_publique(&self) -> &[u8; 32] {
        &self.cle_publique
    }

    /// `kid` de la clé déléguée.
    pub fn kid(&self) -> [u8; 32] {
        empreinte_cle(&self.cle_publique)
    }

    /// Début de validité (secondes Unix).
    pub fn pas_avant(&self) -> u64 {
        self.pas_avant
    }

    /// Fin de validité (secondes Unix).
    pub fn pas_apres(&self) -> u64 {
        self.pas_apres
    }

    /// Vrai si la fenêtre couvre `instant`, avec la tolérance d'horloge du
    /// format ([`TOLERANCE_HORLOGE_SECS`], la même que pour tout titre).
    pub fn couvre(&self, instant: u64) -> bool {
        instant.saturating_add(TOLERANCE_HORLOGE_SECS) >= self.pas_avant
            && instant.saturating_sub(TOLERANCE_HORLOGE_SECS) <= self.pas_apres
    }

    /// Scelle la délégation avec la clé d'identité du COG (indice 0), qui
    /// doit correspondre au `cog_id` ; la clé déléguée doit en différer.
    pub fn sceller(&self, identite: &CleSignature) -> Result<Vec<u8>, ErreurSceau> {
        let cle_identite = identite.cle_publique();
        if cog_id_depuis_cle(&cle_identite) != self.cog_id {
            return Err(ErreurSceau::ValeurInvalide(TAG_COG_ID));
        }
        if cle_identite == self.cle_publique {
            return Err(ErreurSceau::ValeurInvalide(TAG_CLE));
        }
        sceller(self, identite)
    }
}

fn un_octet(valeur: &[u8], tag: u8) -> Result<u8, ErreurSceau> {
    match valeur {
        [o] => Ok(*o),
        _ => Err(ErreurSceau::ValeurInvalide(tag)),
    }
}

fn entier_u32(valeur: u64, tag: u8) -> Result<u32, ErreurSceau> {
    u32::try_from(valeur).map_err(|_| ErreurSceau::ValeurInvalide(tag))
}

impl Titre for DelegationCle {
    const TYPE: TypeSceau = TypeSceau::Certificat;

    fn encoder(&self, ecrivain: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
        self.controler()?;
        ecrivain
            .octets(TAG_NATURE, &[NATURE_DELEGATION])?
            .texte(TAG_COG_ID, &self.cog_id)?
            .u64(TAG_INDICE, u64::from(self.indice))?
            .u64(TAG_GENERATION, u64::from(self.generation))?
            .b32(TAG_CLE, &self.cle_publique)?
            .u64(TAG_PAS_AVANT, self.pas_avant)?
            .u64(TAG_PAS_APRES, self.pas_apres)?;
        Ok(())
    }

    fn decoder(lecteur: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
        if un_octet(lecteur.exiger(TAG_NATURE)?, TAG_NATURE)? != NATURE_DELEGATION {
            return Err(ErreurSceau::ValeurInvalide(TAG_NATURE));
        }
        let cog_id = lecteur.exiger_texte(TAG_COG_ID)?.to_owned();
        let indice = entier_u32(lecteur.exiger_u64(TAG_INDICE)?, TAG_INDICE)?;
        let generation = entier_u32(lecteur.exiger_u64(TAG_GENERATION)?, TAG_GENERATION)?;
        let cle_publique = lecteur.exiger_b32(TAG_CLE)?;
        let pas_avant = lecteur.exiger_u64(TAG_PAS_AVANT)?;
        let pas_apres = lecteur.exiger_u64(TAG_PAS_APRES)?;
        let d = Self {
            cog_id,
            indice,
            generation,
            cle_publique,
            pas_avant,
            pas_apres,
        };
        d.controler()?;
        Ok(d)
    }

    fn fenetre_validite(&self) -> Option<(u64, u64)> {
        Some((self.pas_avant, self.pas_apres))
    }
}

/// Vérifie une délégation contre la clé d'identité **épinglée** d'un COG.
///
/// Exige : sceau strict `0x06`/`0x45`, signature valide de `cle_identite`
/// (seul signataire compté ; une signature non épinglée en plus n'est pas
/// comptée), `cog_id = cog_id(cle_identite)`, clé déléguée ≠ clé
/// d'identité, fenêtre couvrant `maintenant` (tolérance du format).
/// Le niveau du [`Verifie`] rendu est `niveau`.
pub fn verifier_delegation(
    octets: &[u8],
    cle_identite: &[u8; 32],
    niveau: NiveauConfiance,
    maintenant: u64,
) -> Result<Verifie<DelegationCle>, ErreurSceau> {
    let mut ancres = AncresDeConfiance::new();
    let kid = ancres.ajouter(cle_identite, niveau)?;
    let verifie = verifier::<DelegationCle>(octets, &ancres, maintenant)?;
    if verifie.signataires() != [kid] {
        return Err(ErreurSceau::SignataireInconnu);
    }
    let d = verifie.titre();
    if d.cog_id != cog_id_depuis_cle(cle_identite) {
        return Err(ErreurSceau::ValeurInvalide(TAG_COG_ID));
    }
    if &d.cle_publique == cle_identite {
        return Err(ErreurSceau::ValeurInvalide(TAG_CLE));
    }
    Ok(verifie)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cog_id_longueur_et_alphabet() {
        let c = cog_id_depuis_cle(&[7u8; 32]);
        assert!(cog_id_valide(&c));
    }

    #[test]
    fn controles_du_constructeur() {
        let cle = CleSignature::depuis_graine(&[1u8; 32]).cle_publique();
        let cog = cog_id_depuis_cle(&[2u8; 32]);
        assert!(DelegationCle::nouvelle(&cog, 5, 0, cle, 0, DUREE_MAX_DELEGATION_SECS).is_ok());
        assert!(DelegationCle::nouvelle(&cog, 0, 0, cle, 0, 10).is_err());
        assert!(DelegationCle::nouvelle(&cog, 1 << 31, 0, cle, 0, 10).is_err());
        assert!(DelegationCle::nouvelle(&cog, 1, 1 << 31, cle, 0, 10).is_err());
        assert!(DelegationCle::nouvelle(&cog, 1, 0, cle, 10, 10).is_err());
        assert!(
            DelegationCle::nouvelle(&cog, 1, 0, cle, 0, DUREE_MAX_DELEGATION_SECS + 1).is_err()
        );
        assert!(DelegationCle::nouvelle("COG", 1, 0, cle, 0, 10).is_err());
        assert!(DelegationCle::nouvelle(&cog, 1, 0, [0u8; 32], 0, 10).is_err());
    }
}
