//! @id     central_fuzz_generer_graines
//! @do     generer_corpus_initial_fuzzing
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — génère le corpus initial déterministe de la cible `etat_mcce` (C-26).

//! `cargo run --example generer_graines` (stable) : écrit `seeds/<cible>/*`.

extern crate self as miyukini_central;

#[path = "../../../../crates/miyukini-central/src/fichiers_secrets.rs"]
#[allow(dead_code)]
pub mod fichiers_secrets;

#[path = "../../src/canal_officiel/persistance.rs"]
#[allow(dead_code)]
mod persistance;

use std::fs;
use std::path::Path;

use persistance::EtatCanal;

fn ecrire(cible: &str, nom: &str, octets: &[u8]) {
    let dossier = Path::new(env!("CARGO_MANIFEST_DIR")).join("seeds").join(cible);
    fs::create_dir_all(&dossier).expect("dossier de graines");
    fs::write(dossier.join(nom), octets).expect("écriture de graine");
}

fn main() {
    let etat = EtatCanal { etat: b"MSRG\x01etat".to_vec(), temps: b"MSMT\x01t".to_vec(), horloge: vec![0, 0, 0, 0, 0x6b, 0x49, 0xd2, 0x00] };
    ecrire("etat_mcce", "etat", &[&[0u8][..], &etat.encoder()].concat());
    // Mode 1 : coupes (2) | données | altération (3).
    ecrire("etat_mcce", "construit", &[&[1u8, 4, 3][..], b"abcdefghij", &[0, 0, 0]].concat());
    ecrire("etat_mcce", "construit_altere", &[&[1u8, 4, 3][..], b"abcdefghij", &[2, 0x40, 0x81]].concat());
}
