//! @id     maia_memoire_fuzz_export
//! @do     fuzzer_export_memoire_v1
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — désérialisation de l'export intégral `maia-memoire/v1` (EX-56, C-26).

//! Entrée : JSON quelconque → `ExportMemoire` (et ses éléments seuls :
//! `Note`, `Candidat`, `PierreTombale`).
//!
//! Propriétés : aucune panique ; un export relu se resérialise et se relit à
//! l'identique (aller-retour) ; tout identifiant, branche et empreinte relus
//! respectent leur grammaire (`parse` les accepte à l'identique).

#![no_main]

use libfuzzer_sys::fuzz_target;
use maia_memoire::{Branche, Candidat, EmpreinteSource, ExportMemoire, IdMemoire, Note, PierreTombale};
use serde::de::DeserializeOwned;
use serde::Serialize;

fn aller_retour<T: Serialize + DeserializeOwned + PartialEq + std::fmt::Debug>(v: &T) {
    let json = serde_json::to_vec(v).expect("resérialisation");
    let relu: T = serde_json::from_slice(&json).expect("relecture de l'export resérialisé");
    assert_eq!(&relu, v, "aller-retour différent");
}

fn id(i: &IdMemoire) {
    assert_eq!(IdMemoire::parse(i.as_str()).as_ref(), Ok(i));
}

fn sources(s: &[EmpreinteSource]) {
    for e in s {
        assert_eq!(EmpreinteSource::parse(e.as_str()).as_ref(), Ok(e));
    }
}

fn note(n: &Note) {
    id(&n.id);
    n.liens.iter().chain(&n.entites).for_each(id);
    sources(&n.sources);
}

fn candidat(c: &Candidat) {
    c.cible.iter().chain(&c.liens).for_each(id);
    assert_eq!(Branche::parse(c.branche.as_str()).as_ref(), Ok(&c.branche));
    sources(&c.sources);
}

fuzz_target!(|octets: &[u8]| {
    if let Ok(e) = serde_json::from_slice::<ExportMemoire>(octets) {
        e.notes.iter().chain(&e.historique).for_each(note);
        e.candidats.iter().for_each(candidat);
        e.pierres_tombales.iter().for_each(|p| id(&p.id));
        aller_retour(&e);
    }
    if let Ok(n) = serde_json::from_slice::<Note>(octets) {
        note(&n);
        aller_retour(&n);
    }
    if let Ok(c) = serde_json::from_slice::<Candidat>(octets) {
        candidat(&c);
        aller_retour(&c);
    }
    if let Ok(p) = serde_json::from_slice::<PierreTombale>(octets) {
        id(&p.id);
        aller_retour(&p);
    }
});
