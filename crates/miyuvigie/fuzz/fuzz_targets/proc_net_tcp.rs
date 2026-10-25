//! @id     miyuvigie_fuzz_proc_net_tcp
//! @do     fuzzer_releve_proc_net_tcp
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — relevé strict de `/proc/net/tcp` et contrôle de sortie de la zone COG (EX-10, EX-12, C-26).

//! Propriétés : aucune panique ; une sortie par ligne non vide (en-tête
//! exclu) ; sans en-tête, tout est mal formé ; la Vigie analyse le relevé
//! sans panique et compte chaque connexion.

#![no_main]

#[path = "commun.rs"]
mod commun;

use libfuzzer_sys::fuzz_target;
use miyuvigie::{analyser_proc_net_tcp, JeuRegles, LigneProc};

fuzz_target!(|octets: &[u8]| {
    let texte = commun::texte(octets);
    let non_vides: Vec<&str> = texte.lines().filter(|l| !l.trim().is_empty()).collect();
    let resultats = analyser_proc_net_tcp(&texte, 1_000);
    let entete = non_vides
        .first()
        .is_some_and(|l| l.split_whitespace().take(4).eq(["sl", "local_address", "rem_address", "st"]));
    assert_eq!(resultats.len(), non_vides.len() - usize::from(entete), "ligne perdue");
    if !entete {
        assert!(resultats.iter().all(|r| matches!(r, LigneProc::MalFormee(_))));
    }
    let connexions = resultats.iter().filter(|r| matches!(r, LigneProc::Connexion(_))).count() as u64;
    let mut vigie = commun::vigie(JeuRegles::vide());
    let lot = vigie.analyser_proc_net_tcp(&texte, 1_000);
    let c = lot.compteurs;
    assert_eq!(c.acceptees + c.doublons + c.rejeus, connexions, "connexion non comptée");
});
