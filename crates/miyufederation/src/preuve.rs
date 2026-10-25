//! @id     miyufederation_preuve
//! @do     encoder_verifier_preuve_faute
//! @role   securite
//! @layer  toolkit
//! @human  miyufederation — module « preuve » (preuves cryptographiques de faute : équivoque de têtes signées, EX-32).

//! Preuves de faute (EX-32, zones §16 « déclarations signées par le COG mis en
//! cause »).
//!
//! Seul type défini ici : **équivoque** = deux têtes de registre signées par
//! la clé de journal du COG mis en cause, incompatibles (même séquence et
//! contenu différent, taille en recul, ou même taille et racine différente :
//! mêmes règles que `miyusceau::EtatRegistre`). Les signatures de témoins
//! éventuelles sont ignorées : seule compte celle du mis en cause.
//!
//! Encodage (à amender) : `"MFPV" | 0x01 | type (1) | long. A (4, BE) | A | long. B (4, BE) | B`,
//! les deux sceaux rangés par ordre d'octets croissant (encodage unique) ;
//! empreinte = SHA-256 de l'encodage.

use miyusceau::{
    decoder_titre, verifier_seuil, AncresDeConfiance, NiveauConfiance, PreuveEquivoque, Sceau, TeteRegistre,
    TAILLE_MAX_SCEAU, TOLERANCE_HORLOGE_SECS,
};
use sha2::{Digest, Sha256};

use crate::erreurs::ErreurFederation;
use crate::titres::PREUVE_EQUIVOQUE;

const MAGIC: &[u8; 4] = b"MFPV";
const VERSION: u8 = 0x01;

/// Preuve de faute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Preuve {
    /// Deux têtes signées contradictoires (rangées : `premiere < seconde`).
    Equivoque {
        /// Premier sceau de tête.
        premiere: Vec<u8>,
        /// Second sceau de tête.
        seconde: Vec<u8>,
    },
}

fn ordonner(a: Vec<u8>, b: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
    if a <= b {
        (a, b)
    } else {
        (b, a)
    }
}

impl Preuve {
    /// Preuve d'équivoque à partir de deux sceaux de tête (ordre indifférent).
    pub fn equivoque(a: Vec<u8>, b: Vec<u8>) -> Self {
        let (premiere, seconde) = ordonner(a, b);
        Self::Equivoque { premiere, seconde }
    }

    /// Reprend la preuve produite par `miyusceau::EtatRegistre`.
    pub fn depuis_equivoque(p: &PreuveEquivoque) -> Self {
        Self::equivoque(p.acceptee.clone(), p.contradictoire.clone())
    }

    /// Code du type de preuve.
    pub const fn type_preuve(&self) -> u64 {
        match self {
            Self::Equivoque { .. } => PREUVE_EQUIVOQUE,
        }
    }

    /// Encodage canonique.
    pub fn encoder(&self) -> Vec<u8> {
        let Self::Equivoque { premiere, seconde } = self;
        let mut s = Vec::with_capacity(14 + premiere.len() + seconde.len());
        s.extend_from_slice(MAGIC);
        s.push(VERSION);
        s.push(PREUVE_EQUIVOQUE as u8);
        for part in [premiere, seconde] {
            s.extend_from_slice(&(part.len() as u32).to_be_bytes());
            s.extend_from_slice(part);
        }
        s
    }

    /// Décodage strict (taille exacte, ordre canonique, sceaux bien formés).
    pub fn decoder(octets: &[u8]) -> Result<Self, ErreurFederation> {
        let invalide = || ErreurFederation::PreuveInvalide;
        if octets.len() < 14 || &octets[0..4] != MAGIC || octets[4] != VERSION || u64::from(octets[5]) != PREUVE_EQUIVOQUE {
            return Err(invalide());
        }
        let mut pos = 6usize;
        let mut parts = Vec::with_capacity(2);
        for _ in 0..2 {
            let l = octets.get(pos..pos + 4).ok_or_else(invalide)?;
            let l = u32::from_be_bytes([l[0], l[1], l[2], l[3]]) as usize;
            if l > TAILLE_MAX_SCEAU {
                return Err(invalide());
            }
            pos += 4;
            let part = octets.get(pos..pos + l).ok_or_else(invalide)?;
            Sceau::decoder(part)?;
            parts.push(part.to_vec());
            pos += l;
        }
        if pos != octets.len() {
            return Err(invalide());
        }
        let seconde = parts.pop().ok_or_else(invalide)?;
        let premiere = parts.pop().ok_or_else(invalide)?;
        if premiere > seconde {
            return Err(invalide());
        }
        Ok(Self::Equivoque { premiere, seconde })
    }

    /// Empreinte (SHA-256 de l'encodage canonique), signée dans l'accusation.
    pub fn empreinte(&self) -> [u8; 32] {
        Sha256::digest(self.encoder()).into()
    }

    /// Vérifie la preuve contre la clé de journal et le registre du mis en cause.
    pub fn verifier(&self, journal: &[u8; 32], registre: &[u8; 32], maintenant: u64) -> Result<(), ErreurFederation> {
        let Self::Equivoque { premiere, seconde } = self;
        let mut ancres = AncresDeConfiance::new();
        ancres.ajouter(journal, NiveauConfiance::Reconnu)?;
        let a = tete_signee(premiere, &ancres, registre, maintenant)?;
        let b = tete_signee(seconde, &ancres, registre, maintenant)?;
        let (x, y) = if a.sequence <= b.sequence { (&a, &b) } else { (&b, &a) };
        let contradictoire = (x.sequence == y.sequence && x != y)
            || y.taille < x.taille
            || (y.taille == x.taille && y.racine != x.racine);
        if contradictoire {
            Ok(())
        } else {
            Err(ErreurFederation::PreuveInvalide)
        }
    }
}

/// Tête signée par la clé épinglée, du registre attendu, non datée dans le
/// futur. La fraîcheur n'est pas exigée (une faute passée reste une faute).
fn tete_signee(
    octets: &[u8],
    ancres: &AncresDeConfiance,
    registre: &[u8; 32],
    maintenant: u64,
) -> Result<TeteRegistre, ErreurFederation> {
    let tete = decoder_titre::<TeteRegistre>(&Sceau::decoder(octets)?.charge)?;
    if tete.emise_a > maintenant.saturating_add(TOLERANCE_HORLOGE_SECS) {
        return Err(ErreurFederation::PreuveInvalide);
    }
    // Vérifiée à son instant d'émission : toujours dans sa propre fenêtre.
    let verifie = verifier_seuil::<TeteRegistre>(octets, ancres, 1, tete.emise_a)?;
    if &verifie.titre().registre != registre {
        return Err(ErreurFederation::PreuveInvalide);
    }
    Ok(verifie.into_titre())
}
