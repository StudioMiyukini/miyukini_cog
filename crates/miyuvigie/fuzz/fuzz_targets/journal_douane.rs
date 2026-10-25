//! @id     miyuvigie_fuzz_journal_douane
//! @do     fuzzer_lecteur_journaux_douane
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — lecture stricte des journaux NFLOG de la Douane (préfixe `MDOU|`, A-004) par une Vigie (EX-10, C-26).

//! Entrée : lignes de journal quelconques (non fiables : un processus
//! compromis peut écrire dans le journal noyau).
//!
//! Propriétés : aucune panique ; chaque ligne est comptée exactement une
//! fois (acceptée, hors convention, mal formée, incohérente, doublon ou
//! rejeu) ; toute ligne sans le marqueur est hors convention.

#![no_main]

#[path = "commun.rs"]
mod commun;

use libfuzzer_sys::fuzz_target;
use miyuvigie::{JeuRegles, LecteurNftables, ResultatLigne, VoieId, MARQUEUR_PREFIXE};

fuzz_target!(|octets: &[u8]| {
    let texte = commun::texte(octets);
    let mut lecteur = LecteurNftables::new(commun::plan());
    lecteur.declarer_interface_voie("sv0", VoieId::new("jayoffice").expect("voie"));
    let regles = JeuRegles::vide();
    let lignes: Vec<&str> = texte.split('\n').collect();
    for l in &lignes {
        match lecteur.analyser(l) {
            ResultatLigne::HorsConvention => assert!(!l.contains(MARQUEUR_PREFIXE)),
            ResultatLigne::Evenement(e) => {
                assert!(l.contains(MARQUEUR_PREFIXE));
                let _ = regles.evaluer(&e);
            }
            ResultatLigne::MalFormee(_) | ResultatLigne::Incoherente(_) => {}
        }
    }
    let mut vigie = commun::vigie(JeuRegles::vide());
    let lot = vigie.analyser_journal(lignes.iter().copied(), 1_729_760_001_000);
    let c = lot.compteurs;
    assert_eq!(
        c.acceptees + c.hors_convention + c.mal_formees + c.incoherentes + c.doublons + c.rejeus,
        lignes.len() as u64,
        "ligne non comptée ou comptée deux fois"
    );
});
