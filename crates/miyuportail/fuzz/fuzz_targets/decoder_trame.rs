//! @id     miyuportail_fuzz_decoder_trame
//! @do     fuzzer_decodeur_msce_numero_un
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — décodeur n° 1 des trames MSCE (type 0x08) exécuté dans le sas (EX-10, C-26).

//! Entrée : voir [`commun::cas`] (trame brute, ou charge quelconque signée par
//! des clés fixes pour atteindre le décodage de la charge).
//!
//! Propriétés : aucune panique ; toute déclaration admise est canonique
//! (charge ré-encodée identique, trame re-scellée identique — Ed25519 est
//! déterministe), conforme au traité, et traverse le format interne sans perte.

#![no_main]

#[path = "commun.rs"]
mod commun;

use ed25519_dalek::SigningKey;
use libfuzzer_sys::fuzz_target;
use miyuportail::msce::{self, conforme_au_traite, decoder_interne, decoder_trame, encoder_charge, encoder_interne};

fuzz_target!(|entree: &[u8]| {
    let Some(cas) = commun::cas(entree) else { return };
    let service = commun::service();
    let traite = commun::traite();
    let ancres = commun::ancres(cas.avec_auteur);
    let ctx = commun::contexte(&service, cas.sens, &traite, &ancres);

    let Ok(d) = decoder_trame(&cas.trame, &ctx) else { return };

    // Charge canonique : octets [11 .. 11 + len] de la trame.
    let len = u32::from_be_bytes([cas.trame[7], cas.trame[8], cas.trame[9], cas.trame[10]]) as usize;
    let charge = &cas.trame[msce::TAILLE_ENTETE..msce::TAILLE_ENTETE + len];
    assert_eq!(encoder_charge(&d).expect("déclaration admise ré-encodable"), charge, "charge non canonique");

    // Trame canonique : re-scellement par les mêmes clés.
    if !cas.signataires.is_empty() {
        let cles: Vec<&SigningKey> = cas.signataires.iter().collect();
        assert_eq!(msce::sceller(&d, &cles).expect("re-scellement"), cas.trame, "trame non canonique");
    }

    // Signataires : kid triés, attendus par la politique de la voie.
    assert!(d.signataires.windows(2).all(|p| p[0] < p[1]), "signataires non triés");
    assert_eq!(d.signataires.len(), if ancres.auteurs.is_empty() { 1 } else { 2 });
    assert_eq!(d.service, service);
    assert_eq!(d.sens, cas.sens);

    conforme_au_traite(&d, &traite).expect("déclaration admise non conforme au traité");

    // Format interne (sortie du sas) : aller-retour exact.
    let interne = encoder_interne(&d);
    assert_eq!(decoder_interne(&interne).expect("format interne relu"), d, "format interne avec perte");
});
