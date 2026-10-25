//! @id     miyauth_fuzz_titres
//! @do     fuzzer_passeports_visas
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — décodage et vérification des passeports et visas (titres Sceau), émetteurs local et étranger (EX-10, C-26).

//! Entrée : `mode (1) | reste`.
//!
//! - 0 : passeport scellé brut → `verifier_passeport` ;
//! - 1 : charge de passeport → `decoder_titre::<Passeport>`, puis scellement
//!   par la clé de l'émetteur choisi (octet de tête de la charge : bit 0 =
//!   étranger) et vérification ;
//! - 2 : visa scellé brut avec un passeport valide fixe → `verifier_visa` ;
//! - 3 : charge de visa → `decoder_titre::<Visa>`, rattachée au passeport fixe
//!   (empreinte, émetteur local), scellée par le COG local et vérifiée.
//!
//! Propriétés : aucune panique ; décodages canoniques ; un passeport admis
//! est destiné au COG local avec un niveau cohérent avec son émetteur ; un
//! accès admis respecte la fenêtre, le sujet, l'appareil, l'audience et les
//! portées du passeport.

#![no_main]

#[path = "commun.rs"]
mod commun;

use libfuzzer_sys::fuzz_target;
use miyauth::{empreinte_titre, verifier_passeport, verifier_visa, IdentityRole, Passeport, Visa};
use miyusceau::{decoder_titre, sceller, EcrivainTlv, NiveauConfiance, Titre};

fn canonique<T: Titre>(t: &T) -> Vec<u8> {
    let mut e = EcrivainTlv::new();
    t.encoder(&mut e).expect("titre admis ré-encodable");
    e.finir()
}

fn controler_passeport(p: &Passeport, niveau: NiveauConfiance) {
    assert_eq!(p.destination, commun::cog_local(), "passeport admis pour un autre COG");
    let local = p.emetteur_cog == commun::cog_local();
    assert_eq!(local, niveau == NiveauConfiance::Souverain, "niveau incohérent");
}

fuzz_target!(|octets: &[u8]| {
    let Some((&mode, reste)) = octets.split_first() else { return };
    let annuaire = commun::annuaire();
    let revocations = commun::SansRevocation;
    let politique = commun::politique(&annuaire, &revocations);
    match mode % 4 {
        0 => {
            if let Ok(v) = verifier_passeport(reste, &politique, commun::T0 + 60) {
                controler_passeport(v.passeport(), v.niveau());
                assert_eq!(v.empreinte(), &empreinte_titre(reste).expect("empreinte"));
            }
        }
        1 => {
            let Ok(mut p) = decoder_titre::<Passeport>(reste) else { return };
            assert_eq!(canonique(&p), reste, "passeport non canonique");
            let etranger = reste.first().is_some_and(|o| o & 1 != 0);
            let cle = if etranger { commun::cle_etrangere() } else { commun::cle_locale() };
            p.emetteur_kid = cle.kid();
            let Ok(scelle) = sceller(&p, &cle) else { return };
            if let Ok(v) = verifier_passeport(&scelle, &politique, p.pas_avant) {
                assert_eq!(v.passeport(), &p);
                controler_passeport(&p, v.niveau());
            }
        }
        2 => {
            let etranger = reste.first().is_some_and(|o| o & 1 != 0);
            let passeport = commun::sceller_passeport(&commun::passeport(etranger));
            let _ = verifier_visa(reste, &passeport, commun::AUDIENCE, &politique, commun::T0 + 60);
        }
        _ => {
            let Ok(mut v) = decoder_titre::<Visa>(reste) else { return };
            assert_eq!(canonique(&v), reste, "visa non canonique");
            let etranger = reste.first().is_some_and(|o| o & 1 != 0);
            let p = commun::passeport(etranger);
            let passeport = commun::sceller_passeport(&p);
            v.passeport = empreinte_titre(&passeport).expect("empreinte");
            v.emetteur_cog = commun::cog_local();
            let Ok(visa) = sceller(&v, &commun::cle_locale()) else { return };
            let audience = v.audience.clone();
            if let Ok(acces) = verifier_visa(&visa, &passeport, &audience, &politique, v.pas_avant) {
                assert_eq!(acces.visa(), &v);
                assert_eq!(acces.sujet(), &p.sujet);
                assert!(v.pas_avant >= p.pas_avant && v.pas_apres <= p.pas_apres, "visa hors fenêtre du passeport");
                assert_eq!(v.cle_appareil, p.cle_appareil);
                assert!(v.portees.as_slice().iter().all(|x| p.portees.contient(x)), "portée élargie admise");
                let role = if etranger { IdentityRole::Visitor } else { IdentityRole::Citizen };
                assert_eq!(acces.role(), role);
            }
        }
    }
});
