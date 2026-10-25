//! @id     miyufederation_titres
//! @do     encoder_titres_federaux
//! @role   securite
//! @layer  toolkit
//! @human  miyufederation — module « titres » (candidature 0x06/0x43, confirmation 0x07, accusation 0x06/0x42).

//! Titres fédéraux au format Sceau v1.1.
//!
//! La doc gelée nomme ces titres sans en fixer l'encodage : formats ci-dessous
//! **à consigner par amendement** (natures de certificat nouvelles, champs du
//! type `0x07`).
//!
//! **Candidature** — certificat `0x06`, nature **`0x43`** (« C »), signée par
//! la clé d'identité du candidat (auto-signée) :
//!
//! | Tag | Champ | Valeur |
//! |---|---|---|
//! | 1 | `nature` | 1 octet `0x43` |
//! | 2 | `racine` | clé publique d'identité du candidat (32 o) |
//! | 3 | `journal` | clé publique de journal (têtes du registre fédéral, 32 o) |
//! | 4 | `registre` | identifiant de son registre (32 o) |
//! | 5 | `emise_a` | secondes Unix ; validité 48 h |
//! | 6 | `difficulte` | `u64`, bits nuls exigés de la preuve de calcul (1..=40) |
//! | 7 | `nonce` | `u64`, preuve de calcul ([`crate::calcul`]) |
//!
//! **Confirmation** — reconnaissance `0x07` (lot 2 §10 « chacun signe la
//! racine de l'autre, avec niveau et périmètre »), signée par la clé
//! d'identité du membre qui confirme :
//!
//! | Tag | Champ | Valeur |
//! |---|---|---|
//! | 1 | `reconnue` | clé publique d'identité confirmée (32 o) |
//! | 2 | `confirmatrice` | clé publique d'identité du membre (32 o) |
//! | 3 | `niveau` | `u64` : 1 = confirmation fédérale (seule valeur admise) |
//! | 4 | `perimetre` | texte : `federation` (seule valeur admise) |
//! | 5 | `emise_a` | secondes Unix |
//! | 6 | `expire_a` | secondes Unix, au plus 24 h après `emise_a` |
//!
//! **Accusation** (signalement de blacklist) — certificat `0x06`, nature
//! **`0x42`** (« B »), **un seul sceau cosigné** par les membres qui ont
//! vérifié la preuve (quorum 6, au plus 8 signatures) :
//!
//! | Tag | Champ | Valeur |
//! |---|---|---|
//! | 1 | `nature` | 1 octet `0x42` |
//! | 2 | `accuse` | clé publique d'identité du COG mis en cause (32 o) |
//! | 3 | `type_preuve` | `u64` : 1 = équivoque de têtes |
//! | 4 | `empreinte_preuve` | SHA-256 de la preuve encodée ([`crate::preuve`]) (32 o) |
//! | 5 | `emise_a` | secondes Unix |

use miyusceau::{
    cosigner, sceller, CleSignature, EcrivainTlv, ErreurSceau, LecteurTlv, Titre, TypeSceau, TOLERANCE_HORLOGE_SECS,
};

use crate::calcul;
use crate::erreurs::ErreurFederation;
use crate::preuve::Preuve;
use crate::statut::DUREE_CONFIRMATION_MAX_SECS;

/// Nature de certificat « candidature fédérale » (à amender, lot 2 §4.1).
pub const NATURE_CANDIDATURE: u8 = 0x43;
/// Nature de certificat « accusation fédérale » (à amender, lot 2 §4.1).
pub const NATURE_ACCUSATION: u8 = 0x42;
/// Validité d'une candidature (la preuve de calcul est liée à son jour).
pub const VALIDITE_CANDIDATURE_SECS: u64 = 2 * 86_400;
/// Niveau de reconnaissance « confirmation fédérale ».
pub const NIVEAU_CONFIRMATION: u64 = 1;
/// Périmètre d'une confirmation fédérale.
pub const PERIMETRE_FEDERATION: &str = "federation";
/// Type de preuve « équivoque de têtes signées ».
pub const PREUVE_EQUIVOQUE: u64 = 1;

/// Candidature fédérale (auto-signée par l'identité du candidat).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidature {
    /// Clé publique d'identité (racine) du candidat.
    pub racine: [u8; 32],
    /// Clé publique de journal (têtes du registre fédéral).
    pub journal: [u8; 32],
    /// Identifiant du registre du candidat.
    pub registre: [u8; 32],
    /// Horodatage (secondes Unix).
    pub emise_a: u64,
    /// Difficulté de la preuve de calcul.
    pub difficulte: u8,
    /// Nonce de la preuve de calcul.
    pub nonce: u64,
}

impl Candidature {
    /// Prépare une candidature et calcule sa preuve (au plus 2⁴⁰ essais).
    pub fn preparer(
        racine: [u8; 32],
        journal: [u8; 32],
        registre: [u8; 32],
        emise_a: u64,
        difficulte: u8,
    ) -> Result<Self, ErreurFederation> {
        if difficulte == 0 || difficulte > calcul::DIFFICULTE_MAX || racine == journal {
            return Err(ErreurFederation::ParametreInvalide("candidature"));
        }
        let nonce = calcul::resoudre(&racine, calcul::jour_de(emise_a), difficulte, 1 << 40)
            .ok_or(ErreurFederation::CalculInsuffisant)?;
        Ok(Self { racine, journal, registre, emise_a, difficulte, nonce })
    }

    /// Vrai si la preuve de calcul est valide (liée à la racine et au jour).
    pub fn preuve_valide(&self) -> bool {
        calcul::verifier(&self.racine, calcul::jour_de(self.emise_a), self.nonce, self.difficulte)
    }

    /// Scelle avec la clé d'identité (doit correspondre à `racine`).
    pub fn sceller(&self, identite: &CleSignature) -> Result<Vec<u8>, ErreurFederation> {
        if identite.cle_publique() != self.racine {
            return Err(ErreurFederation::ParametreInvalide("clé d'identité"));
        }
        Ok(sceller(self, identite)?)
    }
}

impl Titre for Candidature {
    const TYPE: TypeSceau = TypeSceau::Certificat;

    fn encoder(&self, e: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
        if self.racine == self.journal {
            return Err(ErreurSceau::ValeurInvalide(3));
        }
        if self.difficulte == 0 || self.difficulte > calcul::DIFFICULTE_MAX {
            return Err(ErreurSceau::ValeurInvalide(6));
        }
        e.octets(1, &[NATURE_CANDIDATURE])?
            .b32(2, &self.racine)?
            .b32(3, &self.journal)?
            .b32(4, &self.registre)?
            .u64(5, self.emise_a)?
            .u64(6, u64::from(self.difficulte))?
            .u64(7, self.nonce)?;
        Ok(())
    }

    fn decoder(l: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
        if l.exiger(1)? != [NATURE_CANDIDATURE] {
            return Err(ErreurSceau::ValeurInvalide(1));
        }
        let racine = l.exiger_b32(2)?;
        let journal = l.exiger_b32(3)?;
        if racine == journal {
            return Err(ErreurSceau::ValeurInvalide(3));
        }
        let registre = l.exiger_b32(4)?;
        let emise_a = l.exiger_u64(5)?;
        let difficulte = l.exiger_u64(6)?;
        if difficulte == 0 || difficulte > u64::from(calcul::DIFFICULTE_MAX) {
            return Err(ErreurSceau::ValeurInvalide(6));
        }
        let nonce = l.exiger_u64(7)?;
        Ok(Self { racine, journal, registre, emise_a, difficulte: difficulte as u8, nonce })
    }

    fn fenetre_validite(&self) -> Option<(u64, u64)> {
        Some((self.emise_a, self.emise_a.saturating_add(VALIDITE_CANDIDATURE_SECS)))
    }
}

/// Confirmation d'un COG par un membre (reconnaissance `0x07`, renouvelée toutes les 24 h).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Confirmation {
    /// Identité confirmée.
    pub reconnue: [u8; 32],
    /// Identité du membre qui confirme.
    pub confirmatrice: [u8; 32],
    /// Horodatage d'émission.
    pub emise_a: u64,
    /// Fin de validité (≤ 24 h après `emise_a`).
    pub expire_a: u64,
}

impl Confirmation {
    /// Confirmation valable 24 h à partir de `emise_a`.
    pub fn nouvelle(reconnue: [u8; 32], confirmatrice: [u8; 32], emise_a: u64) -> Self {
        Self { reconnue, confirmatrice, emise_a, expire_a: emise_a.saturating_add(DUREE_CONFIRMATION_MAX_SECS) }
    }

    /// Scelle avec la clé d'identité du membre (doit correspondre à `confirmatrice`).
    pub fn sceller(&self, identite: &CleSignature) -> Result<Vec<u8>, ErreurFederation> {
        if identite.cle_publique() != self.confirmatrice {
            return Err(ErreurFederation::ParametreInvalide("clé d'identité"));
        }
        Ok(sceller(self, identite)?)
    }

    fn controler(&self) -> Result<(), ErreurSceau> {
        if self.reconnue == self.confirmatrice {
            return Err(ErreurSceau::ValeurInvalide(2));
        }
        if self.expire_a <= self.emise_a || self.expire_a - self.emise_a > DUREE_CONFIRMATION_MAX_SECS {
            return Err(ErreurSceau::ValeurInvalide(6));
        }
        Ok(())
    }
}

impl Titre for Confirmation {
    const TYPE: TypeSceau = TypeSceau::Reconnaissance;

    fn encoder(&self, e: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
        self.controler()?;
        e.b32(1, &self.reconnue)?
            .b32(2, &self.confirmatrice)?
            .u64(3, NIVEAU_CONFIRMATION)?
            .texte(4, PERIMETRE_FEDERATION)?
            .u64(5, self.emise_a)?
            .u64(6, self.expire_a)?;
        Ok(())
    }

    fn decoder(l: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
        let reconnue = l.exiger_b32(1)?;
        let confirmatrice = l.exiger_b32(2)?;
        if l.exiger_u64(3)? != NIVEAU_CONFIRMATION {
            return Err(ErreurSceau::ValeurInvalide(3));
        }
        if l.exiger_texte(4)? != PERIMETRE_FEDERATION {
            return Err(ErreurSceau::ValeurInvalide(4));
        }
        let c = Self { reconnue, confirmatrice, emise_a: l.exiger_u64(5)?, expire_a: l.exiger_u64(6)? };
        c.controler()?;
        Ok(c)
    }

    fn fenetre_validite(&self) -> Option<(u64, u64)> {
        Some((self.emise_a, self.expire_a))
    }
}

/// Accusation (signalement de blacklist) : un sceau unique cosigné par les membres.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Accusation {
    /// Identité mise en cause.
    pub accuse: [u8; 32],
    /// Type de preuve (1 = équivoque).
    pub type_preuve: u64,
    /// SHA-256 de la preuve encodée.
    pub empreinte_preuve: [u8; 32],
    /// Horodatage.
    pub emise_a: u64,
}

impl Accusation {
    /// Accusation portant sur `preuve`.
    pub fn depuis_preuve(accuse: [u8; 32], preuve: &Preuve, emise_a: u64) -> Self {
        Self { accuse, type_preuve: preuve.type_preuve(), empreinte_preuve: preuve.empreinte(), emise_a }
    }

    /// Premier signataire (clé d'identité d'un membre).
    pub fn sceller(&self, identite: &CleSignature) -> Result<Vec<u8>, ErreurFederation> {
        if identite.cle_publique() == self.accuse {
            return Err(ErreurFederation::AutoConfirmation);
        }
        Ok(sceller(self, identite)?)
    }

    /// Ajoute la signature d'un membre (mêmes octets signés, même empreinte de preuve).
    /// La vérification préalable de la preuve est faite par
    /// [`crate::Federation::contresigner_accusation`].
    pub fn cosigner(octets: &[u8], identite: &CleSignature) -> Result<Vec<u8>, ErreurFederation> {
        Ok(cosigner::<Self>(octets, identite)?)
    }
}

impl Titre for Accusation {
    const TYPE: TypeSceau = TypeSceau::Certificat;

    fn encoder(&self, e: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
        if self.type_preuve != PREUVE_EQUIVOQUE {
            return Err(ErreurSceau::ValeurInvalide(3));
        }
        e.octets(1, &[NATURE_ACCUSATION])?
            .b32(2, &self.accuse)?
            .u64(3, self.type_preuve)?
            .b32(4, &self.empreinte_preuve)?
            .u64(5, self.emise_a)?;
        Ok(())
    }

    fn decoder(l: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
        if l.exiger(1)? != [NATURE_ACCUSATION] {
            return Err(ErreurSceau::ValeurInvalide(1));
        }
        let accuse = l.exiger_b32(2)?;
        let type_preuve = l.exiger_u64(3)?;
        if type_preuve != PREUVE_EQUIVOQUE {
            return Err(ErreurSceau::ValeurInvalide(3));
        }
        Ok(Self { accuse, type_preuve, empreinte_preuve: l.exiger_b32(4)?, emise_a: l.exiger_u64(5)? })
    }

    /// Pas d'expiration ; datée dans le futur au-delà de la tolérance = refus.
    fn fenetre_validite(&self) -> Option<(u64, u64)> {
        Some((self.emise_a, u64::MAX - TOLERANCE_HORLOGE_SECS))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyusceau::{decoder_titre, Sceau};

    #[test]
    fn natures_distinctes_de_la_passation() {
        let id = CleSignature::generer();
        let c = Candidature::preparer(id.cle_publique(), [9; 32], [1; 32], 1_800_000_000, 4).unwrap();
        let octets = c.sceller(&id).unwrap();
        let sceau = Sceau::decoder(&octets).unwrap();
        assert_eq!(decoder_titre::<Candidature>(&sceau.charge).unwrap(), c);
        assert!(decoder_titre::<Accusation>(&sceau.charge).is_err());
        assert!(decoder_titre::<miyuracine::Passation>(&sceau.charge).is_err());
    }

    #[test]
    fn confirmation_bornee_a_24h() {
        let mut c = Confirmation::nouvelle([1; 32], [2; 32], 100);
        assert_eq!(c.expire_a, 100 + 86_400);
        c.expire_a += 1;
        let id = CleSignature::depuis_graine(&[5; 32]);
        c.confirmatrice = id.cle_publique();
        assert!(c.sceller(&id).is_err());
        let auto = Confirmation::nouvelle(id.cle_publique(), id.cle_publique(), 100);
        assert!(auto.sceller(&id).is_err());
    }
}
