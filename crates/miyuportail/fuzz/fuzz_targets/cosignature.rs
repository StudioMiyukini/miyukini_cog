//! @id     miyuportail_fuzz_cosignature
//! @do     fuzzer_cosignature_auteur_facade
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — cosignature de l'auteur par la façade (`sceller_cosigne`, A-007) : signature de l'appareil non fiable, trame 0x08 relue par les deux décodeurs (EX-10, C-26).

//! L'appareil de l'auteur remet à la façade `(clé, signature)` : entrée non
//! fiable. Entrée : `mode (1) | déclaration tirée | [clé (32) | signature (64)]`.
//!
//! - mode & 3 = 0 : cosignature valide de l'auteur fixe ;
//! - 1 : cosignature valide, puis un octet de la signature ou de la clé inversé ;
//! - 2 : clé et signature brutes de l'entrée ;
//! - 3 : l'auteur est la façade elle-même (doublon).
//!
//! Propriétés : aucune panique ; une cosignature fausse ou en doublon ne
//! produit **aucune** trame ; toute trame produite est admise par le décodeur
//! n° 1 **et** la deuxième validation exactement quand la déclaration est
//! conforme au traité, avec la même déclaration et les deux signataires.

#![no_main]

#[path = "commun.rs"]
mod commun;

use ed25519_dalek::Signer;
use libfuzzer_sys::fuzz_target;
use miyuportail::msce::{self, conforme_au_traite, decoder_trame, encoder_charge, sceller_cosigne};
use miyuportail::validation2::valider;
use miyuportail::{Ancres, RefusDecodage, Sens};

fuzz_target!(|entree: &[u8]| {
    let mut f = commun::Flux::new(entree);
    let mode = f.u8();
    let d = commun::declaration_tiree(&mut f, Sens::Entrante);
    let emetteur = commun::cle_emetteur();
    let auteur = commun::cle_auteur();
    let Ok(charge) = encoder_charge(&d) else { return };
    let message = msce::octets_signes(&charge);
    let valide = (auteur.verifying_key().to_bytes(), auteur.sign(&message).to_bytes());

    let (cosignature, attendue_valide) = match mode & 3 {
        0 => (valide, true),
        1 => {
            let (mut cle, mut sig) = valide;
            let i = usize::from(f.u8()) % 96;
            let masque = f.u8() | 1;
            if i < 32 { cle[i] ^= masque } else { sig[i - 32] ^= masque }
            ((cle, sig), false)
        }
        2 => {
            let mut cle = [0u8; 32];
            let mut sig = [0u8; 64];
            cle.copy_from_slice(&[f.prendre(32), &[0u8; 32][..]].concat()[..32]);
            sig.copy_from_slice(&[f.prendre(64), &[0u8; 64][..]].concat()[..64]);
            let attendue = (cle, sig) == valide;
            ((cle, sig), attendue)
        }
        _ => ((emetteur.verifying_key().to_bytes(), emetteur.sign(&message).to_bytes()), false),
    };

    let resultat = sceller_cosigne(&d, &[&emetteur], &[cosignature]);
    if !attendue_valide {
        // Une signature forgée par hasard (mode 2) est impossible en pratique ;
        // une clé inversée peut rester valide seulement si la signature l'est aussi.
        assert!(resultat.is_err(), "cosignature fausse ou en doublon acceptée");
        if mode & 3 == 3 {
            assert_eq!(resultat, Err(RefusDecodage::Signataires));
        }
        return;
    }
    let trame = resultat.expect("cosignature valide refusée");

    let service = commun::service();
    let traite = commun::traite();
    let ancres = Ancres { emetteur: emetteur.verifying_key().to_bytes(), auteurs: vec![cosignature.0] };
    let ctx = commun::contexte(&service, Sens::Entrante, &traite, &ancres);
    let v1 = decoder_trame(&trame, &ctx);
    let v2 = valider(&trame, &ctx);
    assert_eq!(v1, v2, "décodeurs divergents sur une trame cosignée");

    // Re-scellement local par les deux clés : trame identique (Ed25519 déterministe).
    let locale = msce::sceller(&d, &[&emetteur, &auteur]).expect("re-scellement");
    assert_eq!(locale, trame, "trame cosignée non canonique");

    match (conforme_au_traite(&d, &traite), v1) {
        (Ok(()), Ok(lue)) => {
            let mut attendue = d.clone();
            let mut kids = vec![msce::kid(&ancres.emetteur), msce::kid(&cosignature.0)];
            kids.sort_unstable();
            attendue.signataires = kids;
            assert_eq!(lue, attendue, "déclaration cosignée altérée");
        }
        (Ok(()), Err(e)) => panic!("déclaration conforme refusée après cosignature : {e:?}"),
        (Err(_), Ok(lue)) => {
            // Encodage TLV non typé : seule la charge canonique doit être conservée.
            assert_eq!(encoder_charge(&lue).expect("relue ré-encodable"), charge, "charge non canonique");
        }
        (Err(_), Err(_)) => {}
    }
});
