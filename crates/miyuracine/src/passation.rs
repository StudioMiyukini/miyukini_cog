//! @id     miyuracine_passation
//! @do     sceller_passation_racine
//! @role   securite
//! @layer  toolkit
//! @human  miyuracine — module « passation ».

//! Passation de racine (doc gelé §3 : « Rotation = nouvelle racine signée par
//! l'ancienne ») : déclaration « l'ancienne racine se porte garante de la
//! nouvelle », scellée au format Sceau v1.1 par **les deux** clés d'identité
//! (l'ancienne se porte garante ; la nouvelle prouve sa possession).
//!
//! `miyusceau` n'a pas de type dédié : la passation est un `TypeSceau::Certificat`
//! (0x06, « certificat ») dont le premier champ `nature` (tag 0x01) vaut
//! [`NATURE_PASSATION`]. Un type distinct demanderait un amendement du format.
//!
//! Charge TLV (tags croissants, tous obligatoires) :
//!
//! | Tag  | Champ             | Valeur                          |
//! |------|-------------------|---------------------------------|
//! | 0x01 | `nature`          | 1 octet, `0x01` = passation      |
//! | 0x02 | `ancienne_racine` | clé publique Ed25519 (32 o)      |
//! | 0x03 | `nouvelle_racine` | clé publique Ed25519 (32 o)      |
//! | 0x04 | `emise_le`        | secondes Unix (`u64`)            |

use miyusceau::{
    AncresDeConfiance, CleSignature, EcrivainTlv, ErreurSceau, LecteurTlv, NiveauConfiance, Titre,
    TypeSceau,
};

use crate::erreurs::ErreurRacine;

/// Valeur du champ `nature` d'un certificat de passation.
pub const NATURE_PASSATION: u8 = 0x01;

const TAG_NATURE: u8 = 0x01;
const TAG_ANCIENNE: u8 = 0x02;
const TAG_NOUVELLE: u8 = 0x03;
const TAG_EMISE_LE: u8 = 0x04;

/// Déclaration de passation d'une racine COG à la suivante.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Passation {
    /// Clé publique d'identité de l'ancienne racine.
    pub ancienne_racine: [u8; 32],
    /// Clé publique d'identité de la nouvelle racine.
    pub nouvelle_racine: [u8; 32],
    /// Date d'émission (secondes Unix).
    pub emise_le: u64,
}

impl Titre for Passation {
    const TYPE: TypeSceau = TypeSceau::Certificat;

    fn encoder(&self, ecrivain: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
        if self.ancienne_racine == self.nouvelle_racine {
            return Err(ErreurSceau::ValeurInvalide(TAG_NOUVELLE));
        }
        ecrivain
            .octets(TAG_NATURE, &[NATURE_PASSATION])?
            .b32(TAG_ANCIENNE, &self.ancienne_racine)?
            .b32(TAG_NOUVELLE, &self.nouvelle_racine)?
            .u64(TAG_EMISE_LE, self.emise_le)?;
        Ok(())
    }

    fn decoder(lecteur: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
        if lecteur.exiger(TAG_NATURE)? != [NATURE_PASSATION] {
            return Err(ErreurSceau::ValeurInvalide(TAG_NATURE));
        }
        let ancienne_racine = lecteur.exiger_b32(TAG_ANCIENNE)?;
        let nouvelle_racine = lecteur.exiger_b32(TAG_NOUVELLE)?;
        if ancienne_racine == nouvelle_racine {
            return Err(ErreurSceau::ValeurInvalide(TAG_NOUVELLE));
        }
        let emise_le = lecteur.exiger_u64(TAG_EMISE_LE)?;
        Ok(Self {
            ancienne_racine,
            nouvelle_racine,
            emise_le,
        })
    }
}

impl Passation {
    /// Scelle la passation : signatures de l'ancienne **et** de la nouvelle
    /// clé d'identité (les clés doivent correspondre aux champs).
    pub fn sceller(
        &self,
        ancienne: &CleSignature,
        nouvelle: &CleSignature,
    ) -> Result<Vec<u8>, ErreurRacine> {
        if self.ancienne_racine == self.nouvelle_racine {
            return Err(ErreurRacine::PassationRacinesIdentiques);
        }
        if ancienne.cle_publique() != self.ancienne_racine
            || nouvelle.cle_publique() != self.nouvelle_racine
        {
            return Err(ErreurRacine::PassationAncienneRacineInattendue);
        }
        Ok(miyusceau::sceller_multi(self, &[ancienne, nouvelle])?)
    }
}

/// Vérifie un sceau de passation pour une ancienne racine épinglée.
///
/// Exige : sceau strict de type certificat-passation, signature valide de
/// `ancienne_racine` (épinglée), champ `ancienne_racine` égal à la clé
/// épinglée, **et** signature valide de la nouvelle racine annoncée.
pub fn verifier_passation(
    octets: &[u8],
    ancienne_racine: &[u8; 32],
    maintenant_secs: u64,
) -> Result<Passation, ErreurRacine> {
    let mut ancres = AncresDeConfiance::new();
    ancres.ajouter(ancienne_racine, NiveauConfiance::Souverain)?;
    let verifie = miyusceau::verifier::<Passation>(octets, &ancres, maintenant_secs)?;
    let passation = verifie.into_titre();
    if &passation.ancienne_racine != ancienne_racine {
        return Err(ErreurRacine::PassationAncienneRacineInattendue);
    }
    let mut ancres_nouvelle = AncresDeConfiance::new();
    ancres_nouvelle.ajouter(&passation.nouvelle_racine, NiveauConfiance::Souverain)?;
    let contre =
        miyusceau::verifier::<Passation>(octets, &ancres_nouvelle, maintenant_secs)?.into_titre();
    if contre != passation {
        return Err(ErreurRacine::PassationAncienneRacineInattendue);
    }
    Ok(passation)
}
