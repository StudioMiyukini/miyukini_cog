//! @id     miyusceau_fuzz_verifier_seuil
//! @do     fuzzer_verification_seuil
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — vérification à seuil avec ancres et seuil aléatoires (EX-10).

//! Entrée : `ancres (1) | seuil (1) | maintenant (8, BE) | sceau`.
//! Ancres : sous-ensemble (masque) de 4 clés fixes (graines [31..=34; 32]),
//! pour que les graines du corpus scellées avec ces clés atteignent la
//! vérification des signatures. Propriétés : aucune panique ; un titre
//! accepté a au moins `seuil` signataires, tous épinglés et distincts.

#![no_main]

use libfuzzer_sys::fuzz_target;
use miyusceau::{
    verifier_seuil, verifier_tete, AncresDeConfiance, CleSignature, NiveauConfiance, PolitiqueTete, TeteRegistre,
};

fuzz_target!(|donnees: &[u8]| {
    if donnees.len() < 10 {
        return;
    }
    let masque = donnees[0];
    let seuil = usize::from(donnees[1] % 10);
    let mut instant = [0u8; 8];
    instant.copy_from_slice(&donnees[2..10]);
    let maintenant = u64::from_be_bytes(instant);
    let sceau = &donnees[10..];

    let mut ancres = AncresDeConfiance::new();
    let mut epingles = Vec::new();
    for (i, graine) in (31u8..=34).enumerate() {
        if masque & (1 << i) != 0 {
            let cle = CleSignature::depuis_graine(&[graine; 32]);
            let niveau = if masque & (1 << (i + 4)) != 0 { NiveauConfiance::Reconnu } else { NiveauConfiance::Souverain };
            ancres.ajouter(&cle.cle_publique(), niveau).expect("clé fixe valide");
            epingles.push(cle.kid());
        }
    }

    if let Ok(v) = verifier_seuil::<TeteRegistre>(sceau, &ancres, seuil, maintenant) {
        assert!(seuil >= 1 && v.signataires().len() >= seuil);
        assert!(v.signataires().windows(2).all(|p| p[0] < p[1]));
        assert!(v.signataires().iter().all(|k| epingles.contains(k)));
    }
    if let Ok(politique) = PolitiqueTete::nouvelle(seuil, 1 + (maintenant % 604_800)) {
        let registre = [donnees[1]; 32];
        if let Ok(t) = verifier_tete(sceau, &ancres, &politique, &registre, maintenant) {
            assert_eq!(t.tete().registre, registre);
            assert!(t.verifie().signataires().len() >= seuil);
        }
    }
});
