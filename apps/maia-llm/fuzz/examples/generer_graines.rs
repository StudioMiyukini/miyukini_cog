//! @id     maia_llm_fuzz_generer_graines
//! @do     generer_corpus_initial_fuzzing
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — génère le corpus initial déterministe de la cible `artefact_mmia` (C-26).

//! `cargo run --example generer_graines` (stable) : écrit `seeds/<cible>/*`.
//! Clé fixe identique à celle de la cible.

#[path = "../../src/provenance.rs"]
#[allow(dead_code)]
mod provenance;

#[path = "../../src/canal.rs"]
#[allow(dead_code)]
mod canal;

use std::fs;
use std::path::Path;

use canal::encoder_artefact;
use miyusceau::{sceller, CleSignature};
use provenance::ManifesteModele;
use sha2::{Digest, Sha256};

fn ecrire(cible: &str, nom: &str, octets: &[u8]) {
    let dossier = Path::new(env!("CARGO_MANIFEST_DIR")).join("seeds").join(cible);
    fs::create_dir_all(&dossier).expect("dossier de graines");
    fs::write(dossier.join(nom), octets).expect("écriture de graine");
}

fn main() {
    let modele = b"GGUF\x03\x00\x00\x00modele de test";
    let m = ManifesteModele {
        nom_fichier: "qwen-mini.gguf".into(),
        sha256: Sha256::digest(modele).into(),
        taille: modele.len() as u64,
        licence: "MIT".into(),
        date_coupure: "2025-06".into(),
        emis_le: 1_800_000_000,
    };
    let manifeste = sceller(&m, &CleSignature::depuis_graine(&[0x2a; 32])).expect("manifeste");
    ecrire("artefact_mmia", "artefact", &[&[0u8][..], &encoder_artefact(&manifeste, modele)].concat());
    ecrire("artefact_mmia", "artefact_vide", &[0, b'M', b'M', b'I', b'A', 1, 0, 0, 0, 1, 0]);
    let nom = b"qwen-mini.gguf";
    let construit = |parametres: [u8; 4]| [&[1u8, nom.len() as u8][..], nom, &parametres, modele].concat();
    ecrire("artefact_mmia", "construit", &construit([0, 0, 0, 0]));
    ecrire("artefact_mmia", "construit_gpl", &construit([1, 0, 0, 0]));
    ecrire("artefact_mmia", "construit_altere", &construit([0, 7, 0x01, 0]));
    ecrire("artefact_mmia", "construit_tronque", &construit([0, 0, 0, 0x83]));
    let sceau = b"x.gguf.SCEAU";
    ecrire("artefact_mmia", "nom_sceau", &[&[1u8, sceau.len() as u8][..], sceau, &[0, 0, 0, 0], modele].concat());
}
