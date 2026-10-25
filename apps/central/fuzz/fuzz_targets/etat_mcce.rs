//! @id     central_fuzz_etat_mcce
//! @do     fuzzer_etat_persistant_canal_officiel
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — lecture de l'état persistant `etat.mcce` du canal officiel de Central (fail-closed, A-007) (C-26).

//! Entrée : `mode (1) | reste`.
//!
//! - 0 : état brut → `EtatCanal::decoder` ;
//! - autre : trois parties quelconques (`reste` découpé selon ses 2 premiers
//!   octets) encodées par `EtatCanal::encoder`, puis altérées par les 3
//!   derniers octets (position, masque, troncature).
//!
//! Propriétés : aucune panique ; décodage canonique (ré-encodage identique) ;
//! taille exacte exigée (préfixe strict ou octet ajouté refusés) ; parties
//! non vides et bornées (1 Mio) ; aller-retour encoder → décoder identique.

#![no_main]

// `persistance.rs` appelle `miyukini_central::fichiers_secrets::ecrire_secret` :
// le chemin est résolu vers ce crate, qui inclut le module réel.
extern crate self as miyukini_central;

#[path = "../../../../crates/miyukini-central/src/fichiers_secrets.rs"]
#[allow(dead_code)]
pub mod fichiers_secrets;

#[path = "../../src/canal_officiel/persistance.rs"]
#[allow(dead_code)]
mod persistance;

use libfuzzer_sys::fuzz_target;
use persistance::EtatCanal;

const TAILLE_MAX_PARTIE: usize = 1 << 20;

fn controler(octets: &[u8]) {
    let Some(etat) = EtatCanal::decoder(octets) else { return };
    assert_eq!(etat.encoder(), octets, "état non canonique");
    for partie in [&etat.etat, &etat.temps, &etat.horloge] {
        assert!(!partie.is_empty() && partie.len() <= TAILLE_MAX_PARTIE, "partie hors bornes admise");
    }
    // Taille exacte : ni préfixe strict, ni octet ajouté.
    assert!(EtatCanal::decoder(&octets[..octets.len() - 1]).is_none(), "état tronqué admis");
    let mut long = octets.to_vec();
    long.push(0);
    assert!(EtatCanal::decoder(&long).is_none(), "octet en trop admis");
}

fuzz_target!(|octets: &[u8]| {
    let Some((&mode, reste)) = octets.split_first() else { return };
    if mode == 0 {
        controler(reste);
        return;
    }
    if reste.len() < 5 {
        return;
    }
    let (corps, alteration) = reste.split_at(reste.len() - 3);
    let (coupes, donnees) = corps.split_at(2);
    let a = usize::from(coupes[0]).min(donnees.len());
    let b = (a + usize::from(coupes[1])).min(donnees.len());
    let etat = EtatCanal { etat: donnees[..a].to_vec(), temps: donnees[a..b].to_vec(), horloge: donnees[b..].to_vec() };
    let mut encode = etat.encoder();
    let vides = etat.etat.is_empty() || etat.temps.is_empty() || etat.horloge.is_empty();
    match EtatCanal::decoder(&encode) {
        Some(relu) => {
            assert!(!vides, "partie vide admise");
            assert_eq!(relu, etat, "aller-retour différent");
        }
        None => assert!(vides, "état bien formé refusé"),
    }

    // Altération : bits retournés et/ou troncature.
    let (pos, masque, coupe) = (alteration[0], alteration[1], alteration[2]);
    if masque != 0 {
        let p = usize::from(pos) * 13 % encode.len();
        encode[p] ^= masque;
    }
    if coupe & 0x80 != 0 {
        let garde = encode.len().saturating_sub(usize::from(coupe & 0x7F));
        encode.truncate(garde);
    }
    controler(&encode);
});
