//! @id     miyufederation_fuzz_generer_graines
//! @do     generer_corpus_initial_fuzzing
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — génère le corpus initial déterministe des cibles de miyufederation (C-26).

//! `cargo run --example generer_graines` (stable) : écrit `seeds/<cible>/*`.
//! COG fixes identiques à ceux des cibles (`fuzz_targets/commun.rs`).

#[path = "../fuzz_targets/commun.rs"]
mod commun;

use std::fs;
use std::path::Path;

use commun::{cogs, T0};
use miyufederation::titres::PREUVE_EQUIVOQUE;
use miyufederation::{Accusation, Confirmation, Preuve, Recours};
use miyusceau::Sceau;

fn ecrire(cible: &str, nom: &str, octets: &[u8]) {
    let dossier = Path::new(env!("CARGO_MANIFEST_DIR")).join("seeds").join(cible);
    fs::create_dir_all(&dossier).expect("dossier de graines");
    fs::write(dossier.join(nom), octets).expect("écriture de graine");
}

fn avec_mode(mode: &[u8], octets: &[u8]) -> Vec<u8> {
    [mode, octets].concat()
}

fn main() {
    let c = cogs();
    let a = c[0].tete(1, 4, [1; 32], T0).expect("tête");
    let b = c[0].tete(1, 4, [2; 32], T0).expect("tête");
    let preuve = Preuve::equivoque(a.clone(), b.clone());
    let accusation = Accusation::depuis_preuve(c[0].identite.cle_publique(), &preuve, T0 + 50)
        .sceller(&c[1].identite)
        .expect("accusation");
    let recours = Recours {
        accuse: c[0].identite.cle_publique(),
        type_preuve: PREUVE_EQUIVOQUE,
        empreinte_preuve: preuve.empreinte(),
        accusation: [3; 32],
        emise_a: T0 + 60,
    }
    .sceller(&c[1].identite)
    .expect("recours");
    let confirmation = Confirmation::nouvelle(c[2].identite.cle_publique(), c[1].identite.cle_publique(), T0)
        .sceller(&c[1].identite)
        .expect("confirmation");

    for (nom, sceau) in [
        ("candidature", &c[0].candidature),
        ("accusation", &accusation),
        ("recours", &recours),
        ("confirmation", &confirmation),
    ] {
        ecrire("titres", &format!("charge_{nom}"), &avec_mode(&[0], &Sceau::decoder(sceau).expect("sceau").charge));
        ecrire("titres", &format!("sceau_{nom}"), &avec_mode(&[1], sceau));
    }
    ecrire("titres", "accusation_quorum", &[2, 5, 1, 1, 1, 1, 1, 2, 5]);
    ecrire("titres", "accusation_sous_quorum", &[2, 4, 1, 1, 1, 1, 1, 2, 5]);
    ecrire("titres", "accusation_recul", &[2, 5, 1, 3, 0, 2, 1, 0, 2]);
    ecrire("titres", "confirmation_construite", &[3, 1, 2, 0, 10, 0, 0x50, 0]);

    ecrire("preuve", "equivoque", &avec_mode(&[0], &preuve.encoder()));
    let recul = Preuve::equivoque(c[0].tete(1, 4, [1; 32], T0).expect("tête"), c[0].tete(2, 3, [1; 32], T0).expect("tête"));
    ecrire("preuve", "recul", &avec_mode(&[0], &recul.encoder()));
    ecrire("preuve", "assemblee", &[&[1u8, a.len().min(255) as u8][..], &a, &b].concat());
}
