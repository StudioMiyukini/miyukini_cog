//! @id     miyuportail_fuzz_validation2
//! @do     fuzzer_deuxieme_validation_differentielle
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — deuxième validation en zone COG, différentielle contre le décodeur n° 1 (EX-10, EX-46).

//! Entrée : voir [`commun::cas`].
//!
//! Propriétés : aucune panique ; les deux décodeurs indépendants **admettent
//! exactement les mêmes trames** et produisent la même déclaration typée
//! (la zone COG refuse toute divergence : une divergence est soit un refus
//! injustifié, soit une trame que l'un des deux laisse passer à tort).

#![no_main]

#[path = "commun.rs"]
mod commun;

use libfuzzer_sys::fuzz_target;
use miyuportail::msce::decoder_trame;
use miyuportail::validation2::valider;

fuzz_target!(|entree: &[u8]| {
    let Some(cas) = commun::cas(entree) else { return };
    let service = commun::service();
    let traite = commun::traite();
    let ancres = commun::ancres(cas.avec_auteur);
    let ctx = commun::contexte(&service, cas.sens, &traite, &ancres);

    let v2 = valider(&cas.trame, &ctx);
    let v1 = decoder_trame(&cas.trame, &ctx);
    match (&v1, &v2) {
        (Ok(a), Ok(b)) => assert_eq!(a, b, "déclarations divergentes"),
        (Err(_), Err(_)) => {}
        _ => panic!("décodeurs divergents : n° 1 = {v1:?}, n° 2 = {v2:?}"),
    }
});
