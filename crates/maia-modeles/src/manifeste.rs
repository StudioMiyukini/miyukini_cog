//! @id     maia_modeles_manifeste
//! @do     controler_manifeste_modele
//! @role   securite
//! @layer  toolkit
//! @human  maia-modeles — manifeste signé d'un modèle GGUF et contrôles purs (signature, signataire épinglé, nom, taille, licence, empreinte).

//! Manifeste signé d'un modèle (EX-26 « modèles IA uniquement via le canal
//! officiel, signés ») et contrôles **purs** (aucune entrée/sortie).
//!
//! # Règle
//!
//! Un fichier `X.gguf` n'est admis que si :
//!
//! 1. un **manifeste signé** `X.gguf.sceau` l'accompagne ;
//! 2. ce manifeste est un Sceau v1.1 (`miyusceau`) de type `ManifestePaquet`
//!    (`0x03`), de genre [`GENRE_MODELE`], signé par une **ancre de confiance
//!    épinglée** (`model_trust_anchors`) ;
//! 3. le nom de fichier, la taille et le **SHA-256 recalculé** du fichier
//!    correspondent au manifeste ;
//! 4. la licence déclarée est autorisée (C-72 : `Apache-2.0` ou `MIT`).
//!
//! Sinon refus (fail-closed). Aucune ancre = aucun modèle admis.
//!
//! # Format du manifeste (charge TLV, tags croissants)
//!
//! | Tag | Champ            | Type            | Contrainte                         |
//! |-----|------------------|-----------------|------------------------------------|
//! | 1   | genre            | texte           | exactement `maia/modele-gguf/v1`   |
//! | 2   | nom de fichier   | texte           | 1..=255 octets, sans `/` ni `\`    |
//! | 3   | SHA-256          | 32 octets       | empreinte du fichier complet       |
//! | 4   | taille           | u64 (BE)        | taille exacte en octets            |
//! | 5   | licence          | texte (SPDX)    | `Apache-2.0` ou `MIT`              |
//! | 6   | date de coupure  | texte           | connaissances du modèle, ex. `2025-06` (1..=32) |
//! | 7   | émis le          | u64             | secondes Unix de la signature      |
//!
//! Enveloppe : `MSCE` · version `0x01` · type `0x03` · kid · longueur · charge ·
//! signature Ed25519 (voir `docs/tools/MiyuSceau/_index.md`).

use std::path::PathBuf;

use miyusceau::{
    verifier, AncresDeConfiance, EcrivainTlv, ErreurSceau, LecteurTlv, NiveauConfiance, Titre,
    TypeSceau,
};

/// Genre obligatoire du manifeste (séparation des manifestes de paquets).
pub const GENRE_MODELE: &str = "maia/modele-gguf/v1";
/// Extension du manifeste signé, ajoutée au nom du fichier modèle.
pub const EXTENSION_MANIFESTE: &str = "sceau";
/// Licences de modèles autorisées (C-72).
pub const LICENCES_AUTORISEES: &[&str] = &["Apache-2.0", "MIT"];

const NOM_MAX: usize = 255;
const LICENCE_MAX: usize = 64;
const COUPURE_MAX: usize = 32;

/// Manifeste signé d'un modèle GGUF.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifesteModele {
    /// Nom du fichier modèle (sans chemin).
    pub nom_fichier: String,
    /// SHA-256 du fichier complet.
    pub sha256: [u8; 32],
    /// Taille exacte en octets.
    pub taille: u64,
    /// Licence SPDX.
    pub licence: String,
    /// Date de coupure des connaissances (ex. `2025-06`).
    pub date_coupure: String,
    /// Date d'émission (secondes Unix).
    pub emis_le: u64,
}

fn texte_borne(valeur: &str, tag: u8, max: usize) -> Result<(), ErreurSceau> {
    if valeur.is_empty() || valeur.len() > max {
        return Err(ErreurSceau::ValeurInvalide(tag));
    }
    Ok(())
}

/// Nom de fichier simple : 1..=255 octets, sans séparateur ni NUL, ni `.`/`..`.
pub(crate) fn nom_fichier_valide(nom: &str) -> bool {
    !nom.is_empty()
        && nom.len() <= NOM_MAX
        && !nom.contains(['/', '\\', '\0'])
        && nom != "."
        && nom != ".."
}

impl Titre for ManifesteModele {
    const TYPE: TypeSceau = TypeSceau::ManifestePaquet;

    fn encoder(&self, e: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
        if !nom_fichier_valide(&self.nom_fichier) {
            return Err(ErreurSceau::ValeurInvalide(2));
        }
        texte_borne(&self.licence, 5, LICENCE_MAX)?;
        texte_borne(&self.date_coupure, 6, COUPURE_MAX)?;
        e.texte(1, GENRE_MODELE)?
            .texte(2, &self.nom_fichier)?
            .b32(3, &self.sha256)?
            .u64(4, self.taille)?
            .texte(5, &self.licence)?
            .texte(6, &self.date_coupure)?
            .u64(7, self.emis_le)?;
        Ok(())
    }

    fn decoder(l: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
        if l.exiger_texte(1)? != GENRE_MODELE {
            return Err(ErreurSceau::ValeurInvalide(1));
        }
        let nom_fichier = l.exiger_texte(2)?.to_string();
        if !nom_fichier_valide(&nom_fichier) {
            return Err(ErreurSceau::ValeurInvalide(2));
        }
        let sha256 = l.exiger_b32(3)?;
        let taille = l.exiger_u64(4)?;
        let licence = l.exiger_texte(5)?.to_string();
        texte_borne(&licence, 5, LICENCE_MAX)?;
        let date_coupure = l.exiger_texte(6)?.to_string();
        texte_borne(&date_coupure, 6, COUPURE_MAX)?;
        let emis_le = l.exiger_u64(7)?;
        Ok(Self {
            nom_fichier,
            sha256,
            taille,
            licence,
            date_coupure,
            emis_le,
        })
    }
}

/// Motif de refus d'un modèle (une variante par cause, C-11).
#[derive(Debug)]
pub enum ErreurProvenance {
    /// Aucun manifeste signé à côté du modèle.
    ManifesteAbsent(PathBuf),
    /// Manifeste illisible ou trop gros.
    ManifesteIllisible(String),
    /// Sceau refusé (format, type, signataire inconnu, signature…).
    Sceau(ErreurSceau),
    /// Le manifeste décrit un autre fichier.
    NomDifferent {
        /// Nom attendu par le manifeste.
        attendu: String,
        /// Nom du fichier présenté.
        trouve: String,
    },
    /// Taille du fichier différente de celle signée.
    TailleDifferente {
        /// Taille signée.
        attendue: u64,
        /// Taille constatée.
        trouvee: u64,
    },
    /// Empreinte SHA-256 du fichier différente de celle signée.
    EmpreinteDifferente,
    /// Licence non autorisée (C-72).
    LicenceRefusee(String),
    /// Lecture du fichier modèle impossible.
    LectureModele(String),
    /// Ancre de confiance mal formée dans la configuration.
    AncreInvalide(String),
    /// Le fichier a changé entre le contrôle et le chargement (TOCTOU).
    ModeleModifie,
}

impl std::fmt::Display for ErreurProvenance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ManifesteAbsent(p) => {
                write!(
                    f,
                    "manifeste signé absent ({}) : modèle refusé",
                    p.display()
                )
            }
            Self::ManifesteIllisible(e) => write!(f, "manifeste illisible : {e}"),
            Self::Sceau(e) => write!(f, "manifeste refusé : {e}"),
            Self::NomDifferent { attendu, trouve } => {
                write!(f, "manifeste pour « {attendu} », fichier « {trouve} »")
            }
            Self::TailleDifferente { attendue, trouvee } => {
                write!(f, "taille {trouvee} o, {attendue} o signés")
            }
            Self::EmpreinteDifferente => {
                write!(f, "empreinte SHA-256 différente de la valeur signée")
            }
            Self::LicenceRefusee(l) => {
                write!(f, "licence « {l} » non autorisée (Apache-2.0 ou MIT)")
            }
            Self::LectureModele(e) => write!(f, "lecture du modèle impossible : {e}"),
            Self::AncreInvalide(e) => write!(f, "ancre de confiance invalide : {e}"),
            Self::ModeleModifie => {
                write!(
                    f,
                    "fichier modèle modifié entre le contrôle et le chargement"
                )
            }
        }
    }
}

impl std::error::Error for ErreurProvenance {}

/// Décode une liste de clés publiques Ed25519 hexadécimales (64 caractères)
/// en ancres de niveau `Souverain`. Une seule entrée invalide = erreur.
pub fn ancres_depuis_hex(cles: &[String]) -> Result<AncresDeConfiance, ErreurProvenance> {
    let mut ancres = AncresDeConfiance::new();
    for c in cles {
        let octets = hex::decode(c.trim())
            .map_err(|e| ErreurProvenance::AncreInvalide(format!("{c} : {e}")))?;
        let cle: [u8; 32] = octets
            .try_into()
            .map_err(|_| ErreurProvenance::AncreInvalide(format!("{c} : 32 octets attendus")))?;
        ancres
            .ajouter(&cle, NiveauConfiance::Souverain)
            .map_err(|e| ErreurProvenance::AncreInvalide(format!("{c} : {e}")))?;
    }
    Ok(ancres)
}

/// Contrôle **pur** d'un manifeste face à un fichier présenté : signature et
/// signataire épinglé, genre, nom, taille, licence. L'empreinte est contrôlée
/// séparément ([`controler_empreinte`]) car elle exige de lire le fichier.
pub fn controler_manifeste(
    octets: &[u8],
    ancres: &AncresDeConfiance,
    nom_fichier: &str,
    taille: u64,
) -> Result<ManifesteModele, ErreurProvenance> {
    let verifie = verifier::<ManifesteModele>(octets, ancres, crate::maintenant_secs())
        .map_err(ErreurProvenance::Sceau)?;
    let m = verifie.into_titre();
    if m.nom_fichier != nom_fichier {
        return Err(ErreurProvenance::NomDifferent {
            attendu: m.nom_fichier,
            trouve: nom_fichier.to_string(),
        });
    }
    if m.taille != taille {
        return Err(ErreurProvenance::TailleDifferente {
            attendue: m.taille,
            trouvee: taille,
        });
    }
    if !LICENCES_AUTORISEES.contains(&m.licence.as_str()) {
        return Err(ErreurProvenance::LicenceRefusee(m.licence));
    }
    Ok(m)
}

/// Compare l'empreinte calculée à l'empreinte signée.
pub fn controler_empreinte(
    m: &ManifesteModele,
    calculee: &[u8; 32],
) -> Result<(), ErreurProvenance> {
    if &m.sha256 == calculee {
        Ok(())
    } else {
        Err(ErreurProvenance::EmpreinteDifferente)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::essai::{ancres, cle};
    use miyusceau::sceller;

    #[test]
    fn empreinte_signee_falsifiee_refusee() {
        let c = cle();
        let m = ManifesteModele {
            nom_fichier: "a.gguf".into(),
            sha256: [1u8; 32],
            taille: 3,
            licence: "MIT".into(),
            date_coupure: "2025-06".into(),
            emis_le: 1,
        };
        let mut octets = sceller(&m, &c).unwrap();
        // Altération d'un octet de l'empreinte dans la charge signée.
        let pos = octets.windows(32).position(|w| w == [1u8; 32]).unwrap();
        octets[pos] ^= 1;
        assert!(matches!(
            controler_manifeste(&octets, &ancres(&c), "a.gguf", 3),
            Err(ErreurProvenance::Sceau(ErreurSceau::SignatureInvalide))
        ));
        // Empreinte calculée différente de l'empreinte signée.
        let ok = sceller(&m, &c).unwrap();
        let mv = controler_manifeste(&ok, &ancres(&c), "a.gguf", 3).unwrap();
        assert!(controler_empreinte(&mv, &[2u8; 32]).is_err());
        assert!(controler_empreinte(&mv, &[1u8; 32]).is_ok());
        // Nom ou taille présentés différents.
        assert!(matches!(
            controler_manifeste(&ok, &ancres(&c), "b.gguf", 3),
            Err(ErreurProvenance::NomDifferent { .. })
        ));
        assert!(matches!(
            controler_manifeste(&ok, &ancres(&c), "a.gguf", 4),
            Err(ErreurProvenance::TailleDifferente { .. })
        ));
    }

    #[test]
    fn licence_non_autorisee_refusee() {
        let c = cle();
        let m = ManifesteModele {
            nom_fichier: "a.gguf".into(),
            sha256: [0u8; 32],
            taille: 1,
            licence: "Llama-3-Community".into(),
            date_coupure: "2024-12".into(),
            emis_le: 1,
        };
        let octets = sceller(&m, &c).unwrap();
        assert!(matches!(
            controler_manifeste(&octets, &ancres(&c), "a.gguf", 1),
            Err(ErreurProvenance::LicenceRefusee(_))
        ));
    }

    #[test]
    fn genre_et_type_imposes() {
        // Un sceau d'un autre type (ex. passeport) est refusé par le type.
        struct Autre;
        impl Titre for Autre {
            const TYPE: TypeSceau = TypeSceau::Passeport;
            fn encoder(&self, e: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
                e.texte(1, GENRE_MODELE)?;
                Ok(())
            }
            fn decoder(_: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
                Ok(Self)
            }
        }
        let c = cle();
        let octets = sceller(&Autre, &c).unwrap();
        assert!(matches!(
            controler_manifeste(&octets, &ancres(&c), "a.gguf", 1),
            Err(ErreurProvenance::Sceau(ErreurSceau::TypeInattendu { .. }))
        ));
        // Un manifeste de paquet d'un autre genre est refusé par le genre.
        struct Paquet;
        impl Titre for Paquet {
            const TYPE: TypeSceau = TypeSceau::ManifestePaquet;
            fn encoder(&self, e: &mut EcrivainTlv) -> Result<(), ErreurSceau> {
                e.texte(1, "service/paquet")?;
                Ok(())
            }
            fn decoder(_: &mut LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
                Ok(Self)
            }
        }
        let octets = sceller(&Paquet, &c).unwrap();
        assert!(matches!(
            controler_manifeste(&octets, &ancres(&c), "a.gguf", 1),
            Err(ErreurProvenance::Sceau(ErreurSceau::ValeurInvalide(1)))
        ));
    }

    #[test]
    fn noms_de_fichier_invalides_refuses_au_scellement() {
        for nom in ["", ".", "..", "a/b.gguf", "a\\b.gguf", "a\0.gguf"] {
            let m = ManifesteModele {
                nom_fichier: nom.into(),
                sha256: [0u8; 32],
                taille: 1,
                licence: "MIT".into(),
                date_coupure: "2025-06".into(),
                emis_le: 1,
            };
            assert!(sceller(&m, &cle()).is_err(), "{nom:?}");
        }
    }

    #[test]
    fn ancres_invalides_refusees() {
        assert!(ancres_depuis_hex(&["zz".into()]).is_err());
        assert!(ancres_depuis_hex(&["00".repeat(31)]).is_err());
        assert!(ancres_depuis_hex(&[]).unwrap().is_empty());
    }
}
