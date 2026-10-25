//! @id     miyucanal_fuzz_generer_graines
//! @do     generer_corpus_initial_fuzzing
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — génère le corpus initial déterministe des cibles du canal officiel (EX-10).

//! `cargo run --example generer_graines` (stable) : écrit `seeds/<cible>/*`.
//! Clés fixes identiques à celles des cibles.

use std::fs;
use std::path::Path;

use miyucanal::{encoder_entrees, sceller_annonce, sceller_quarantaine, CleQuarantaine, Demande, VuePersistee};
use miyuregistre::{Categorie, Corps, EntreeRegistre, Publication};
use miyusceau::{CleSignature, EcrivainTlv, Titre};

const T0: u64 = 1_800_000_000;

fn ecrire(cible: &str, nom: &str, octets: &[u8]) {
    let dossier = Path::new(env!("CARGO_MANIFEST_DIR")).join("seeds").join(cible);
    fs::create_dir_all(&dossier).expect("dossier de graines");
    fs::write(dossier.join(nom), octets).expect("écriture de graine");
}

fn avec_mode(mode: &[u8], octets: &[u8]) -> Vec<u8> {
    [mode, octets].concat()
}

fn main() {
    let origin = CleSignature::depuis_graine(&[51; 32]);
    let editeur = CleSignature::depuis_graine(&[52; 32]);
    for (nom, d) in [
        ("tete", Demande::Tete),
        ("entrees", Demande::Entrees { depuis: 3, nombre: 16 }),
        ("artefact", Demande::Artefact { empreinte: [4; 32], taille: 1_000 }),
    ] {
        ecrire("mcan", &format!("demande_{nom}"), &avec_mode(&[0], &d.encoder()));
    }
    let publication = Publication {
        categorie: Categorie::Cores,
        composant: "strongfather".into(),
        version: 2,
        empreinte: [9; 32],
        taille: 4_096,
    };
    let annonce = sceller_annonce(publication.clone(), T0 - 10, &origin, &editeur).expect("annonce");
    ecrire("mcan", "entrees", &avec_mode(&[1], &encoder_entrees(&[annonce.clone(), b"x".to_vec()])));
    let entree = EntreeRegistre { inscrite_a: T0 - 10, corps: Corps::Publication(publication) };
    let mut e = EcrivainTlv::new();
    entree.encoder(&mut e).expect("charge");
    let charge = e.finir();
    ecrire("mcan", "annonce_2sur2", &avec_mode(&[2, 0], &charge));
    ecrire("mcan", "annonce_origin_seul", &avec_mode(&[2, 1], &charge));

    let cle = CleQuarantaine::deriver(&[9; 32]);
    let vues = [
        VuePersistee { identite: [1; 32], index: 0, vue_le: 1_000 },
        VuePersistee { identite: [2; 32], index: 3, vue_le: 2_000 },
    ];
    ecrire("quarantaine", "etat", &avec_mode(&[0], &sceller_quarantaine(&[7; 32], &vues, &cle)));
    ecrire("quarantaine", "vues", &[1, 1, 0, 3, 232, 2, 3, 7, 208, 1, 5, 3, 0, 0, 0, 0]);
    ecrire("quarantaine", "vues_alterees", &[1, 1, 0, 3, 232, 2, 3, 7, 208, 9, 0x10, 0x81]);
}
