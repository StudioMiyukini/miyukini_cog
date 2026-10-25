//! @id     maia_federee_fuzz_commun
//! @do     outiller_cibles_fuzzing_ia_federee
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — outillage commun des cibles de maia-federee : clé d'émetteur fixe, Fiche type (C-26).

#![allow(dead_code)]

use maia_federee::{sceller_fiche, verifier_fiche, Audience, Fiche, FicheVerifiee, Relation, TypeFiche};
use miyusceau::CleSignature;

pub const MAINTENANT: u64 = 1_800_000_100;

/// Clé d'identité de l'émetteur fixe.
pub fn cle() -> CleSignature {
    CleSignature::depuis_graine(&[0x46; 32])
}

/// Fiche type de l'émetteur fixe.
pub fn fiche(texte: &str) -> Fiche {
    Fiche {
        id: "geo.paris".into(),
        emetteur: cle().cle_publique(),
        sujet: "Capitale de la France".into(),
        type_fiche: TypeFiche::Fait,
        texte: texte.into(),
        relations: vec![Relation {
            sujet: "Paris".into(),
            predicat: "capitale de".into(),
            objet: "France".into(),
        }],
        sources: vec!["https://exemple.org/geographie".into()],
        date: 1_800_000_000,
        version: 1,
        langue: "fr".into(),
        licence: "CC-BY-4.0".into(),
        audience: Audience::Membres,
        reprises: Vec::new(),
    }
}

/// Fiche type scellée puis vérifiée (`None` : texte refusé).
pub fn fiche_verifiee(texte: &str) -> Option<FicheVerifiee> {
    let octets = sceller_fiche(&fiche(texte), &cle()).ok()?;
    verifier_fiche(&octets, MAINTENANT).ok()
}
