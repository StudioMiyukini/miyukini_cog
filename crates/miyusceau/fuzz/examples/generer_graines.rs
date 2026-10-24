//! @id     miyusceau_fuzz_generer_graines
//! @do     generer_corpus_initial_fuzzing
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — génère le corpus initial déterministe des cibles (EX-10).

//! `cargo run --example generer_graines` (stable) : écrit `seeds/<cible>/*`.
//! Clés fixes (graines [31..=34; 32]) identiques à celles de `verifier_seuil`.

use std::fs;
use std::path::Path;

use miyusceau::{
    hacher_feuille, racine, sceller, sceller_multi, CleSignature, EcrivainTlv, Empreinte, PreuveCoherence,
    PreuveInclusion, TeteRegistre, Titre, AGE_MAX_TETE_DEFAUT_SECS,
};

const T0: u64 = 1_800_000_000;

fn ecrire(cible: &str, nom: &str, octets: &[u8]) {
    let dossier = Path::new(env!("CARGO_MANIFEST_DIR")).join("seeds").join(cible);
    fs::create_dir_all(&dossier).expect("dossier de graines");
    fs::write(dossier.join(nom), octets).expect("écriture de graine");
}

fn main() {
    let cles: Vec<CleSignature> = (31u8..=34).map(|g| CleSignature::depuis_graine(&[g; 32])).collect();
    let feuilles: Vec<Empreinte> = (0u8..7).map(|i| hacher_feuille(&[i])).collect();
    let registre = [2u8; 32];
    let tete = TeteRegistre {
        registre,
        taille: 7,
        racine: racine(&feuilles),
        sequence: 1,
        emise_a: T0,
        validite_max_secs: AGE_MAX_TETE_DEFAUT_SECS,
    };
    let simple = sceller(&tete, &cles[0]).expect("sceau simple");
    let double = sceller_multi(&tete, &[&cles[0], &cles[1]]).expect("sceau 2 sur 2");
    let quadruple = sceller_multi(&tete, &[&cles[0], &cles[1], &cles[2], &cles[3]]).expect("sceau 4 sur 4");

    ecrire("decoder_sceau", "simple", &simple);
    ecrire("decoder_sceau", "double", &double);
    ecrire("decoder_sceau", "quadruple", &quadruple);

    // ancres (masque) | seuil | maintenant | sceau ; seuil 2 → registre [2; 32].
    for (nom, masque, seuil, sceau) in
        [("simple", 0x01u8, 2u8, &simple), ("double", 0x03, 2, &double), ("quadruple", 0x2f, 12, &quadruple)]
    {
        let mut entree = vec![masque, seuil];
        entree.extend_from_slice(&T0.to_be_bytes());
        entree.extend_from_slice(sceau);
        ecrire("verifier_seuil", nom, &entree);
    }

    let mut e = EcrivainTlv::new();
    tete.encoder(&mut e).expect("charge");
    ecrire("decoder_titres", "tete", &e.finir());

    let inclusion = PreuveInclusion::generer(&feuilles, 5).expect("inclusion").encoder();
    let coherence = PreuveCoherence::generer(&feuilles, 3).expect("cohérence").encoder();
    let r = racine(&feuilles);
    let mut entree = vec![0u8];
    entree.extend_from_slice(&feuilles[5]);
    entree.extend_from_slice(&r);
    entree.extend_from_slice(&inclusion);
    ecrire("preuves_merkle", "inclusion", &entree);
    let mut entree = vec![1u8];
    entree.extend_from_slice(&racine(&feuilles[..3]));
    entree.extend_from_slice(&r);
    entree.extend_from_slice(&coherence);
    ecrire("preuves_merkle", "coherence", &entree);
    let mut entree = vec![2u8];
    entree.extend_from_slice(&[0u8; 64]);
    entree.extend_from_slice(&(0u8..20).collect::<Vec<_>>());
    ecrire("preuves_merkle", "arbre", &entree);
}
