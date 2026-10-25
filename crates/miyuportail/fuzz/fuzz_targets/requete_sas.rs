//! @id     miyuportail_fuzz_requete_sas
//! @do     fuzzer_protocole_requete_sas
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — décodage des requêtes parent → sas (`MPSR`) et du traité transmis, cœur du sas (EX-10, EX-46).

//! Entrée : `mode (1) | reste`.
//!
//! - mode 0 : requête brute → `RequeteSas::decoder`, puis `traiter_requete`
//!   (le cœur exécuté sous seccomp) ;
//! - mode 1 : traité brut → `Traite::decoder` ;
//! - autre : lot fabriqué (traité fixe, ancres fixes) dont les trames sont des
//!   charges quelconques signées ; la réponse du sas doit refléter exactement
//!   le décodeur n° 1 appelé directement.
//!
//! Propriétés : aucune panique ; décodages canoniques (ré-encodage identique) ;
//! une requête refusée n'obtient aucune réponse ; toute réponse du sas est
//! relue par `decoder_reponse` et son format interne par `decoder_interne`.

#![no_main]

#[path = "commun.rs"]
mod commun;

use libfuzzer_sys::fuzz_target;
use miyuportail::msce::{decoder_interne, decoder_trame, encoder_interne};
use miyuportail::sas::{decoder_reponse, traiter_requete, MAX_LOT};
use miyuportail::{RequeteSas, Sens, Traite};

fn controler_reponse(reponse: &[u8], requete: &RequeteSas) {
    let resultats = decoder_reponse(reponse, requete.trames.len()).expect("réponse du sas relisible");
    let ctx = commun::contexte(&requete.service, Sens::Entrante, &requete.traite, &requete.ancres);
    for (trame, r) in requete.trames.iter().zip(&resultats) {
        let direct = decoder_trame(trame, &ctx).map(|d| encoder_interne(&d));
        assert_eq!(&direct, r, "réponse du sas différente du décodeur n° 1");
        if let Ok(interne) = r {
            decoder_interne(interne).expect("format interne relisible");
        }
    }
}

fuzz_target!(|entree: &[u8]| {
    let Some((&mode, reste)) = entree.split_first() else { return };
    match mode {
        0 => {
            let decodee = RequeteSas::decoder(reste);
            let traitee = traiter_requete(reste);
            match decodee {
                Ok(r) => {
                    assert_eq!(r.encoder().expect("requête admise ré-encodable"), reste, "requête non canonique");
                    controler_reponse(&traitee.expect("requête admise traitée"), &r);
                }
                Err(_) => assert!(traitee.is_err(), "requête refusée mais traitée"),
            }
        }
        1 => {
            if let Ok(t) = Traite::decoder(reste) {
                assert_eq!(t.encoder(), reste, "traité non canonique");
                t.valider().expect("traité admis valide");
            }
        }
        _ => {
            // Lot : trames séparées par 0xFF 0xFF (au plus MAX_LOT), chacune une
            // charge signée par l'émetteur (et l'auteur si le bit 0 du mode est levé).
            let avec_auteur = mode & 0x01 != 0;
            let emetteur = commun::cle_emetteur();
            let auteur = commun::cle_auteur();
            let cles = if avec_auteur { vec![&emetteur, &auteur] } else { vec![&emetteur] };
            let mut trames = Vec::new();
            let (mut debut, mut i) = (0, 0);
            while i <= reste.len() && trames.len() < MAX_LOT {
                let separateur = i + 1 < reste.len() && reste[i] == 0xFF && reste[i + 1] == 0xFF;
                if i == reste.len() || separateur {
                    trames.push(commun::encadrer(&reste[debut..i], &cles));
                    i += 2;
                    debut = i.min(reste.len());
                } else {
                    i += 1;
                }
            }
            let requete = RequeteSas {
                service: commun::service(),
                ancres: commun::ancres(avec_auteur),
                traite: commun::traite().restreindre(Sens::Entrante),
                trames,
            };
            let Ok(octets) = requete.encoder() else { return };
            assert_eq!(RequeteSas::decoder(&octets).expect("requête encodée relue"), requete);
            controler_reponse(&traiter_requete(&octets).expect("requête valide traitée"), &requete);
        }
    }
});
