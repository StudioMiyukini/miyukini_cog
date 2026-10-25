//! @id     miyusceau_fuzz_preuves_merkle
//! @do     fuzzer_preuves_merkle
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — décodage et vérification des preuves Merkle RFC 9162 (EX-10).

//! Entrée : `sélecteur (1) | feuille ou racine1 (32) | racine (32) | preuve`.
//! Propriétés : aucune panique ; décodage canonique (ré-encodage identique) ;
//! une preuve générée sur un arbre construit depuis l'entrée vérifie, et ne
//! vérifie pas une autre feuille.

#![no_main]

use libfuzzer_sys::fuzz_target;
use miyusceau::{hacher_feuille, racine, Empreinte, PreuveCoherence, PreuveInclusion};

fuzz_target!(|donnees: &[u8]| {
    if donnees.len() < 65 {
        return;
    }
    let mut a: Empreinte = [0; 32];
    let mut b: Empreinte = [0; 32];
    a.copy_from_slice(&donnees[1..33]);
    b.copy_from_slice(&donnees[33..65]);
    let preuve = &donnees[65..];
    match donnees[0] % 3 {
        0 => {
            if let Ok(p) = PreuveInclusion::decoder(preuve) {
                assert_eq!(p.encoder().expect("preuve décodée ré-encodable"), preuve);
                let _ = p.verifier(&a, &b);
            }
        }
        1 => {
            if let Ok(p) = PreuveCoherence::decoder(preuve) {
                assert_eq!(p.encoder().expect("preuve décodée ré-encodable"), preuve);
                let _ = p.verifier(&a, &b);
            }
        }
        _ => {
            // Arbre construit depuis l'entrée (≤ 64 feuilles) : aller-retour.
            let feuilles: Vec<Empreinte> = preuve.iter().take(64).map(|o| hacher_feuille(&[*o])).collect();
            if feuilles.is_empty() {
                return;
            }
            let n = feuilles.len();
            let r = racine(&feuilles);
            let i = usize::from(donnees[1]) % n;
            let m = 1 + usize::from(donnees[2]) % n;
            let pi = PreuveInclusion::generer(&feuilles, i).expect("index valide");
            assert!(pi.verifier(&feuilles[i], &r).is_ok());
            assert!(pi.verifier(&hacher_feuille(b"\xff\xfe"), &r).is_err());
            let pc = PreuveCoherence::generer(&feuilles, m).expect("taille valide");
            assert!(pc.verifier(&racine(&feuilles[..m]), &r).is_ok());
        }
    }
});
