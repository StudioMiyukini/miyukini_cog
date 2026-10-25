//! @id     miyuregistre_entree
//! @do     encoder_decoder_entree_registre
//! @role   securite
//! @layer  toolkit
//! @human  miyuregistre — module « entree » (entrées typées du registre, Sceau type 0x04).

//! Entrées typées du registre (lot 2 §8 : publications, révocations, émissions ;
//! lot 4 : décrets et événements).
//!
//! Une entrée est un titre `Sceau` de type `0x04` (« entrée registre ») ; la
//! **feuille** Merkle est `SHA-256(0x00 || octets du sceau)` (RFC 9162). Le
//! signataire dépend du registre (clé de journal pour un COG ; Origin **et**
//! éditeur pour une publication du canal officiel) : la politique est fixée par
//! le consommateur, pas par l'entrée.
//!
//! Charge TLV (tags strictement croissants, tous obligatoires pour la nature) :
//!
//! | Tag | Champ | Natures | Valeur |
//! |---|---|---|---|
//! | 1 | `nature` | toutes | `u64` : 1 publication, 2 révocation, 3 émission, 4 décret, 5 événement |
//! | 2 | `inscrite_a` | toutes | secondes Unix |
//! | 3 | `categorie` | publication | `u64` : 1 Cores, 2 OS, 3 modèle IA |
//! | 4 | `composant` | publication | texte `[a-z0-9._:-]`, 1..64 o |
//! | 5 | `version` | publication | `u64`, strictement croissante par composant |
//! | 6 | `empreinte` | publication | SHA-256 de l'artefact (32 o) |
//! | 7 | `taille` | publication | taille exacte de l'artefact (`u64`, ≥ 1) |
//! | 3 | `cible_nature` | révocation | `u64` : 1 série, 2 annonce, 3 artefact, 4 clé |
//! | 4 | `cible` | révocation | 32 o |
//! | 5 | `motif` | révocation | texte, 1..256 o |
//! | 3 | `certificat` | émission | identité du certificat éditeur (32 o) |
//! | 3 | `objet` | décret, événement | texte `[a-z0-9._:-]`, 1..64 o |
//! | 4 | `empreinte` | décret, événement | 32 o |
//!
//! Format consigné par l'amendement **A-006** (lot 2 §8.3) : le lot 2 §8
//! nommait les natures sans les encoder.

use miyusceau::{EcrivainTlv, Empreinte, ErreurSceau, LecteurTlv, Sceau, Titre, TypeSceau};
use sha2::{Digest, Sha256};

const TAG_NATURE: u8 = 1;
const TAG_INSCRITE_A: u8 = 2;

/// Longueur maximale d'un identifiant (`composant`, `objet`).
pub const TAILLE_MAX_IDENTIFIANT: usize = 64;
/// Longueur maximale d'un motif de révocation.
pub const TAILLE_MAX_MOTIF: usize = 256;

/// Catégorie d'une publication du canal officiel (EX-01, EX-26).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Categorie {
    /// Mise à jour des Cores.
    Cores,
    /// Correctif de l'OS (miroir re-signé).
    Os,
    /// Modèle IA.
    ModeleIa,
}

impl Categorie {
    /// Code TLV.
    pub const fn code(self) -> u64 {
        match self {
            Self::Cores => 1,
            Self::Os => 2,
            Self::ModeleIa => 3,
        }
    }

    /// Décodage strict d'un code.
    pub const fn depuis_code(code: u64) -> Option<Self> {
        match code {
            1 => Some(Self::Cores),
            2 => Some(Self::Os),
            3 => Some(Self::ModeleIa),
            _ => None,
        }
    }
}

/// Cible d'une révocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cible {
    /// Série d'un titre (passeport, visa, certificat).
    Serie([u8; 32]),
    /// Identité d'une annonce (SHA-256 des octets signés du sceau, voir [`identite_sceau`]).
    Annonce([u8; 32]),
    /// Empreinte d'un artefact (SHA-256, `package_id`).
    Artefact([u8; 32]),
    /// Clé (`kid`) : tout ce qu'elle a signé est révoqué.
    Cle([u8; 32]),
}

impl Cible {
    const fn code(&self) -> u64 {
        match self {
            Self::Serie(_) => 1,
            Self::Annonce(_) => 2,
            Self::Artefact(_) => 3,
            Self::Cle(_) => 4,
        }
    }

    const fn valeur(&self) -> &[u8; 32] {
        match self {
            Self::Serie(v) | Self::Annonce(v) | Self::Artefact(v) | Self::Cle(v) => v,
        }
    }
}

/// Publication d'une version d'un composant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Publication {
    /// Catégorie (Cores, OS, modèle IA).
    pub categorie: Categorie,
    /// Composant publié.
    pub composant: String,
    /// Version (entier croissant).
    pub version: u64,
    /// SHA-256 de l'artefact.
    pub empreinte: Empreinte,
    /// Taille exacte de l'artefact (octets).
    pub taille: u64,
}

/// Contenu typé d'une entrée.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Corps {
    /// Publication (paquet, mise à jour).
    Publication(Publication),
    /// Révocation.
    Revocation {
        /// Cible révoquée.
        cible: Cible,
        /// Motif lisible.
        motif: String,
    },
    /// Émission d'un certificat éditeur.
    Emission {
        /// Identité du certificat.
        certificat: [u8; 32],
    },
    /// Décret (gouvernance, arrêté).
    Decret {
        /// Objet du décret.
        objet: String,
        /// Empreinte du document signé.
        empreinte: [u8; 32],
    },
    /// Événement du registre.
    Evenement {
        /// Code de l'événement.
        objet: String,
        /// Empreinte associée.
        empreinte: [u8; 32],
    },
}

impl Corps {
    const fn nature(&self) -> u64 {
        match self {
            Self::Publication(_) => 1,
            Self::Revocation { .. } => 2,
            Self::Emission { .. } => 3,
            Self::Decret { .. } => 4,
            Self::Evenement { .. } => 5,
        }
    }
}

/// Entrée du registre (titre `Sceau` de type `0x04`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntreeRegistre {
    /// Horodatage d'inscription annoncé par le signataire (secondes Unix).
    pub inscrite_a: u64,
    /// Contenu typé.
    pub corps: Corps,
}

fn identifiant_valide(tag: u8, v: &str) -> Result<(), ErreurSceau> {
    let ok = !v.is_empty()
        && v.len() <= TAILLE_MAX_IDENTIFIANT
        && v.bytes().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, b'.' | b'_' | b':' | b'-'));
    if ok {
        Ok(())
    } else {
        Err(ErreurSceau::ValeurInvalide(tag))
    }
}

fn motif_valide(tag: u8, v: &str) -> Result<(), ErreurSceau> {
    if v.is_empty() || v.len() > TAILLE_MAX_MOTIF || v.chars().any(char::is_control) {
        return Err(ErreurSceau::ValeurInvalide(tag));
    }
    Ok(())
}

impl Titre for EntreeRegistre {
    const TYPE: TypeSceau = TypeSceau::EntreeRegistre;

    fn encoder(&self, e: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
        e.u64(TAG_NATURE, self.corps.nature())?.u64(TAG_INSCRITE_A, self.inscrite_a)?;
        match &self.corps {
            Corps::Publication(p) => {
                identifiant_valide(4, &p.composant)?;
                if p.taille == 0 {
                    return Err(ErreurSceau::ValeurInvalide(7));
                }
                e.u64(3, p.categorie.code())?
                    .texte(4, &p.composant)?
                    .u64(5, p.version)?
                    .b32(6, &p.empreinte)?
                    .u64(7, p.taille)?;
            }
            Corps::Revocation { cible, motif } => {
                motif_valide(5, motif)?;
                e.u64(3, cible.code())?.b32(4, cible.valeur())?.texte(5, motif)?;
            }
            Corps::Emission { certificat } => {
                e.b32(3, certificat)?;
            }
            Corps::Decret { objet, empreinte } | Corps::Evenement { objet, empreinte } => {
                identifiant_valide(3, objet)?;
                e.texte(3, objet)?.b32(4, empreinte)?;
            }
        }
        Ok(())
    }

    fn decoder(l: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
        let nature = l.exiger_u64(TAG_NATURE)?;
        let inscrite_a = l.exiger_u64(TAG_INSCRITE_A)?;
        let corps = match nature {
            1 => {
                let categorie = Categorie::depuis_code(l.exiger_u64(3)?).ok_or(ErreurSceau::ValeurInvalide(3))?;
                let composant = l.exiger_texte(4)?.to_string();
                identifiant_valide(4, &composant)?;
                let version = l.exiger_u64(5)?;
                let empreinte = l.exiger_b32(6)?;
                let taille = l.exiger_u64(7)?;
                if taille == 0 {
                    return Err(ErreurSceau::ValeurInvalide(7));
                }
                Corps::Publication(Publication { categorie, composant, version, empreinte, taille })
            }
            2 => {
                let code = l.exiger_u64(3)?;
                let valeur = l.exiger_b32(4)?;
                let cible = match code {
                    1 => Cible::Serie(valeur),
                    2 => Cible::Annonce(valeur),
                    3 => Cible::Artefact(valeur),
                    4 => Cible::Cle(valeur),
                    _ => return Err(ErreurSceau::ValeurInvalide(3)),
                };
                let motif = l.exiger_texte(5)?.to_string();
                motif_valide(5, &motif)?;
                Corps::Revocation { cible, motif }
            }
            3 => Corps::Emission { certificat: l.exiger_b32(3)? },
            4 | 5 => {
                let objet = l.exiger_texte(3)?.to_string();
                identifiant_valide(3, &objet)?;
                let empreinte = l.exiger_b32(4)?;
                if nature == 4 {
                    Corps::Decret { objet, empreinte }
                } else {
                    Corps::Evenement { objet, empreinte }
                }
            }
            _ => return Err(ErreurSceau::ValeurInvalide(TAG_NATURE)),
        };
        Ok(Self { inscrite_a, corps })
    }
}

/// Identité d'un titre scellé : SHA-256 des octets signés (stable si l'on
/// ajoute un cosignataire). Décodage strict du sceau ; **aucune signature
/// n'est vérifiée** ici.
pub fn identite_sceau(octets: &[u8]) -> Result<[u8; 32], ErreurSceau> {
    let sceau = Sceau::decoder(octets)?;
    Ok(Sha256::digest(sceau.octets_signes()).into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use miyusceau::{decoder_titre, sceller, AncresDeConfiance, CleSignature, NiveauConfiance};

    fn publication() -> EntreeRegistre {
        EntreeRegistre {
            inscrite_a: 1_000,
            corps: Corps::Publication(Publication {
                categorie: Categorie::Cores,
                composant: "strongfather".into(),
                version: 3,
                empreinte: [7; 32],
                taille: 12,
            }),
        }
    }

    #[test]
    fn aller_retour_toutes_natures() {
        let cle = CleSignature::generer();
        let mut ancres = AncresDeConfiance::new();
        ancres.ajouter(&cle.cle_publique(), NiveauConfiance::Reconnu).unwrap();
        let entrees = [
            publication(),
            EntreeRegistre {
                inscrite_a: 2,
                corps: Corps::Revocation { cible: Cible::Annonce([1; 32]), motif: "faille".into() },
            },
            EntreeRegistre { inscrite_a: 3, corps: Corps::Emission { certificat: [2; 32] } },
            EntreeRegistre { inscrite_a: 4, corps: Corps::Decret { objet: "arrete:1".into(), empreinte: [3; 32] } },
            EntreeRegistre { inscrite_a: 5, corps: Corps::Evenement { objet: "rotation".into(), empreinte: [4; 32] } },
        ];
        for e in entrees {
            let sceau = sceller(&e, &cle).unwrap();
            let v = miyusceau::verifier::<EntreeRegistre>(&sceau, &ancres, 10).unwrap();
            assert_eq!(v.titre(), &e);
        }
    }

    #[test]
    fn valeurs_invalides_refusees() {
        let cle = CleSignature::generer();
        let mut e = publication();
        if let Corps::Publication(p) = &mut e.corps {
            p.composant = "Majuscule".into();
        }
        assert!(sceller(&e, &cle).is_err());
        let mut e = publication();
        if let Corps::Publication(p) = &mut e.corps {
            p.taille = 0;
        }
        assert!(sceller(&e, &cle).is_err());
        // Nature inconnue.
        let mut w = EcrivainTlv::new();
        w.u64(1, 9).unwrap().u64(2, 0).unwrap();
        assert_eq!(decoder_titre::<EntreeRegistre>(&w.finir()), Err(ErreurSceau::ValeurInvalide(1)));
        // Champ en trop pour la nature.
        let mut w = EcrivainTlv::new();
        w.u64(1, 3).unwrap().u64(2, 0).unwrap().b32(3, &[0; 32]).unwrap().u64(4, 1).unwrap();
        assert_eq!(decoder_titre::<EntreeRegistre>(&w.finir()), Err(ErreurSceau::ChampInconnu(4)));
    }

    #[test]
    fn identite_stable_par_cosignature() {
        let a = CleSignature::generer();
        let b = CleSignature::generer();
        let s1 = sceller(&publication(), &a).unwrap();
        let s2 = miyusceau::cosigner::<EntreeRegistre>(&s1, &b).unwrap();
        assert_ne!(s1, s2);
        assert_eq!(identite_sceau(&s1).unwrap(), identite_sceau(&s2).unwrap());
    }
}
