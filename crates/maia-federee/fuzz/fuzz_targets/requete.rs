//! @id     maia_federee_fuzz_requete
//! @do     fuzzer_requete_typee
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — requête typée d'une IA étrangère (`RequeteTypee::decoder`) : TLV strict, forme canonique unique (EX-58, C-26).

//! Entrée : `mode (1) | reste`.
//!
//! - 0 : requête brute → `RequeteTypee::decoder`, puis score sur une Fiche type ;
//! - autre : `max (1) | langue (1) | types (1) | sujet` → `RequeteTypee::nouvelle`,
//!   encodée puis relue.
//!
//! Propriétés : aucune panique ; une seule forme acceptée (ré-encodage
//! identique) ; sujet non vide, une ligne ; `max` dans 1..=20 ; toute
//! requête construite se relit à l'identique (normalisation idempotente).

#![no_main]

#[path = "commun.rs"]
mod commun;

use std::sync::OnceLock;

use libfuzzer_sys::fuzz_target;
use maia_federee::requete::{MAX_RESULTATS, MAX_SUJET_REQUETE};
use maia_federee::{FicheVerifiee, RequeteTypee, TypeFiche};

fn fiche() -> &'static FicheVerifiee {
    static F: OnceLock<FicheVerifiee> = OnceLock::new();
    F.get_or_init(|| commun::fiche_verifiee("Paris est la capitale de la France.").expect("fiche type"))
}

fn controler(r: &RequeteTypee) {
    assert!(!r.sujet().is_empty() && !r.sujet().contains('\n'));
    assert!(r.sujet().chars().count() <= MAX_SUJET_REQUETE);
    assert!((1..=MAX_RESULTATS).contains(&r.max()));
    let _ = r.score(fiche());
}

fuzz_target!(|octets: &[u8]| {
    let Some((&mode, reste)) = octets.split_first() else { return };
    if mode == 0 {
        if let Ok(r) = RequeteTypee::decoder(reste) {
            assert_eq!(r.encoder().expect("requête ré-encodable"), reste, "requête non canonique");
            controler(&r);
        }
        return;
    }
    let [max, langue, types, ref sujet @ ..] = *reste else { return };
    let sujet = String::from_utf8_lossy(sujet);
    let langue = match langue % 4 {
        0 => None,
        1 => Some("fr"),
        2 => Some("eng"),
        _ => Some("FR"),
    };
    let types: Vec<TypeFiche> =
        (0..5u8).filter(|i| types & (1 << i) != 0).filter_map(|i| TypeFiche::try_from(i + 1).ok()).collect();
    let Ok(r) = RequeteTypee::nouvelle(&sujet, &types, langue, max) else { return };
    controler(&r);
    let encode = r.encoder().expect("requête construite encodable");
    let relue = RequeteTypee::decoder(&encode);
    assert_eq!(relue.as_ref().ok(), Some(&r), "requête construite non relue : {relue:?} (sujet {:?})", r.sujet());
});
