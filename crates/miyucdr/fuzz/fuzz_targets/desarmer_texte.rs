//! @id     miyucdr_fuzz_desarmer_texte
//! @do     fuzzer_desarmement_textes
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — désarmement de texte brut, CSV, Markdown, JSON et analyse du type MIME déclaré (EX-10, EX-45).

//! Entrée : `type (1) | octets`. Types 0 à 3 : texte brut, CSV, Markdown,
//! JSON. Type ≥ 0xF0 : MIME déclaré libre (jusqu'au premier octet nul), puis
//! contenu.
//!
//! Propriétés : voir [`commun`] ; de plus, le désarmement d'un texte est
//! **idempotent** (la sortie re-désarmée est identique : une formule
//! neutralisée ne l'est pas deux fois, un texte nettoyé reste propre).

#![no_main]

#[path = "commun.rs"]
mod commun;

use libfuzzer_sys::fuzz_target;
use miyucdr::TypeContenu;

const TYPES: [&str; 4] = ["text/plain", "text/csv", "text/markdown", "application/json"];

fuzz_target!(|entree: &[u8]| {
    let Some((&t, reste)) = entree.split_first() else { return };
    let (mime, octets) = if t >= 0xF0 {
        let fin = reste.iter().position(|&b| b == 0).unwrap_or(reste.len());
        let mime = String::from_utf8_lossy(&reste[..fin]).into_owned();
        let _ = TypeContenu::depuis_mime(&mime);
        (mime, &reste[(fin + 1).min(reste.len())..])
    } else {
        (TYPES[usize::from(t) % TYPES.len()].to_string(), reste)
    };
    if let Some((type_reel, sortie)) = commun::desarmer_et_controler(&mime, octets) {
        if !type_reel.est_image() {
            let seconde = commun::reaccepter(type_reel, &sortie);
            assert_eq!(seconde, sortie, "désarmement non idempotent ({type_reel:?})");
        }
    }
});
