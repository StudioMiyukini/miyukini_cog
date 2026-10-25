//! @id     miyuportail_fuzz_reponse_sas
//! @do     fuzzer_protocole_reponse_sas
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — décodage côté parent des réponses du sas (`MPSA`) et du format interne `MPI1` (EX-10, EX-46).

//! Le sas peut être compromis : sa réponse est une entrée non fiable pour le
//! parent. Entrée : `attendus (1) | réponse`.
//!
//! Propriétés : aucune panique ; réponse admise canonique (ré-encodage
//! identique) ; tout format interne admis est canonique.

#![no_main]

use libfuzzer_sys::fuzz_target;
use miyuportail::msce::{decoder_interne, encoder_interne};
use miyuportail::sas::{decoder_reponse, encoder_reponse};

fuzz_target!(|entree: &[u8]| {
    let Some((&attendus, reponse)) = entree.split_first() else { return };
    if let Ok(resultats) = decoder_reponse(reponse, usize::from(attendus)) {
        assert_eq!(resultats.len(), usize::from(attendus));
        assert_eq!(encoder_reponse(&resultats), reponse, "réponse non canonique");
        for interne in resultats.iter().flatten() {
            if let Ok(d) = decoder_interne(interne) {
                assert_eq!(&encoder_interne(&d), interne, "format interne non canonique");
            }
        }
    }
    // Le format interne seul (tête de l'entrée), hors enveloppe.
    if let Ok(d) = decoder_interne(reponse) {
        assert_eq!(encoder_interne(&d), reponse, "format interne non canonique");
    }
});
