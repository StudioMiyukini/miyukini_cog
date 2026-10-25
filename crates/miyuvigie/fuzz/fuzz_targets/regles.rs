//! @id     miyuvigie_fuzz_regles
//! @do     fuzzer_jeu_regles_vigies
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — analyse stricte des fichiers de règles des Vigies (`vigies-regles 1`) (EX-10, C-26).

//! Propriétés : aucune panique ; jeu admis ⇒ identifiants uniques, aucun
//! identifiant réservé, au moins une condition par règle, et le jeu
//! s'applique (Vigie, journal type) sans panique ; un jeu signé dont la
//! signature est refusée n'est jamais chargé.

#![no_main]

#[path = "commun.rs"]
mod commun;

use std::collections::HashSet;

use libfuzzer_sys::fuzz_target;
use miyuvigie::{JeuRegles, VerificateurJeuRegles, REGLES_INTEGREES};

struct Refuse;

impl VerificateurJeuRegles for Refuse {
    fn verifier(&self, _contenu: &[u8], _signature: &[u8]) -> bool {
        false
    }
}

fuzz_target!(|octets: &[u8]| {
    let texte = commun::texte(octets);
    assert!(JeuRegles::charger_signe(&texte, &[], &Refuse).is_err(), "jeu chargé malgré une signature refusée");
    let Ok(jeu) = JeuRegles::charger(&texte) else { return };
    let mut ids = HashSet::new();
    for r in jeu.regles() {
        assert!(ids.insert(r.id.as_str().to_string()), "identifiant en double admis");
        assert!(!REGLES_INTEGREES.contains(&r.id.as_str()), "identifiant réservé admis");
        assert!(!r.conditions.is_empty(), "règle sans condition admise");
    }
    let mut vigie = commun::vigie(jeu);
    let _ = vigie.analyser_journal([commun::LIGNE_TYPE], 1_729_760_001_000);
});
