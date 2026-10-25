//! @id     maia_memoire_fuzz_generer_graines
//! @do     generer_corpus_initial_fuzzing
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — génère le corpus initial déterministe des cibles de maia-memoire (C-26).

//! `cargo run --example generer_graines` (stable) : écrit `seeds/<cible>/*`.

use std::fs;
use std::path::Path;

use chrono::{TimeZone, Utc};
use maia_memoire::types::{Auteur, FORMAT_EXPORT};
use maia_memoire::{
    Branche, Candidat, EmpreinteSource, ExportMemoire, IdMemoire, Note, PierreTombale, Provenance, TypeUnite,
    ETIQUETTE,
};

fn ecrire(cible: &str, nom: &str, octets: &[u8]) {
    let dossier = Path::new(env!("CARGO_MANIFEST_DIR")).join("seeds").join(cible);
    fs::create_dir_all(&dossier).expect("dossier de graines");
    fs::write(dossier.join(nom), octets).expect("écriture de graine");
}

fn main() {
    for (nom, texte) in [
        ("note", "travail.cog.f12"),
        ("note_profonde", "a.b.c.d.e.s999999"),
        ("branche", "travail.cog"),
        ("branche_ambigue", "travail.f1x"),
        ("numero_zero", "travail.f0"),
        ("majuscule", "Travail.cog"),
    ] {
        ecrire("ident", nom, texte.as_bytes());
    }

    let t = Utc.with_ymd_and_hms(2026, 10, 25, 12, 0, 0).single().expect("date");
    let id = IdMemoire::parse("travail.cog.f12").expect("id");
    let source = EmpreinteSource::parse(&"ab".repeat(32)).expect("empreinte");
    let n = Note {
        id: id.clone(),
        version: 2,
        texte: "Le COG tourne sur le serveur.".into(),
        importance: 5,
        confiance: 7,
        provenance: Provenance::Utilisateur,
        auteur: Auteur::Utilisateur,
        liens: vec![IdMemoire::parse("travail.cog.n1").expect("id")],
        entites: Vec::new(),
        sources: vec![source.clone()],
        cree_le: t,
        enregistre_le: t,
        dernier_usage: Some(t),
    };
    let c = Candidat {
        id: uuid::Uuid::from_u128(0x1234_5678_9abc_def0_1234_5678_9abc_def0),
        cible: Some(id.clone()),
        branche: Branche::parse("travail.cog").expect("branche"),
        type_unite: TypeUnite::Fait,
        texte: "Correction proposée.".into(),
        importance: 3,
        confiance: 4,
        provenance: Provenance::Deduit,
        liens: Vec::new(),
        sources: vec![source],
        sensible: false,
        propose_le: t,
    };
    let p = PierreTombale {
        id: IdMemoire::parse("famille.lea.p3").expect("id"),
        oublie_le: t,
        motif: "demande".into(),
        versions_effacees: 2,
    };
    let e = ExportMemoire {
        format: FORMAT_EXPORT.into(),
        etiquette: ETIQUETTE,
        exporte_le: t,
        notes: vec![n.clone()],
        historique: vec![n.clone()],
        candidats: vec![c.clone()],
        pierres_tombales: vec![p.clone()],
    };
    ecrire("export", "export", &serde_json::to_vec_pretty(&e).expect("json"));
    ecrire("export", "note", &serde_json::to_vec(&n).expect("json"));
    ecrire("export", "candidat", &serde_json::to_vec(&c).expect("json"));
    ecrire("export", "pierre_tombale", &serde_json::to_vec(&p).expect("json"));
}
