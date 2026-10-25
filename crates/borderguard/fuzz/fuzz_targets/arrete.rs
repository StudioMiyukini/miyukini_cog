//! @id     borderguard_fuzz_arrete
//! @do     fuzzer_decodage_verification_arretes
//! @role   securite
//! @layer  core
//! @human  Fuzzing — décodage et vérification des arrêtés de frontière signés, ancres quelconques (EX-10, C-26).

//! Entrée : `mode (1) | reste`.
//!
//! - 0 : `ancres (1) | maintenant (8) | sceau` → `verifier_arrete` ; bits 0-1
//!   des ancres : clé des arrêtés absente / souveraine / reconnue, bit 2 :
//!   clé étrangère souveraine en plus ;
//! - 1 : charge TLV → `decoder_titre::<Arrete>` ; tout arrêté décodé est
//!   scellé par la clé fixe (hors constitution) puis vérifié ;
//! - 2 : réseau encodé → `Reseau::decoder`.
//!
//! Propriétés : aucune panique ; décodage canonique (ré-encodage identique) ;
//! un arrêté n'est admis que signé par une ancre **souveraine** et conforme à
//! la constitution ; `sceller_arrete` et la vérification sont d'accord.

#![no_main]

use borderguard::{sceller_arrete, verifier_arrete, Arrete, Constitution, Reseau};
use libfuzzer_sys::fuzz_target;
use miyusceau::{decoder_titre, sceller, AncresDeConfiance, CleSignature, EcrivainTlv, NiveauConfiance, Titre};

fn cle_arretes() -> CleSignature {
    CleSignature::depuis_graine(&[7; 32])
}

fn charge_canonique(a: &Arrete) -> Vec<u8> {
    let mut e = EcrivainTlv::new();
    a.encoder(&mut e).expect("arrêté admis ré-encodable");
    e.finir()
}

fuzz_target!(|octets: &[u8]| {
    let Some((&mode, reste)) = octets.split_first() else { return };
    let constitution = Constitution::reference();
    let cle = cle_arretes();
    match mode % 3 {
        0 => {
            if reste.len() < 9 {
                return;
            }
            let (choix, maintenant, sceau) = (reste[0], &reste[1..9], &reste[9..]);
            let maintenant = u64::from_be_bytes(maintenant.try_into().expect("8 octets"));
            let mut ancres = AncresDeConfiance::new();
            let niveau = match choix & 0x03 {
                1 => Some(NiveauConfiance::Souverain),
                2 => Some(NiveauConfiance::Reconnu),
                _ => None,
            };
            if let Some(n) = niveau {
                ancres.ajouter(&cle.cle_publique(), n).expect("ancre");
            }
            if choix & 0x04 != 0 {
                let etrangere = CleSignature::depuis_graine(&[choix; 32]);
                let _ = ancres.ajouter(&etrangere.cle_publique(), NiveauConfiance::Souverain);
            }
            if let Ok(v) = verifier_arrete(sceau, &ancres, &constitution, maintenant) {
                assert!(
                    *v.signataire() == cle.kid() && niveau == Some(NiveauConfiance::Souverain)
                        || choix & 0x04 != 0 && *v.signataire() != cle.kid(),
                    "arrêté admis sans ancre souveraine"
                );
                constitution.controler(v.arrete()).expect("arrêté admis hors constitution");
                assert_eq!(v.empreinte_hex().len(), 64);
            }
        }
        1 => {
            let Ok(a) = decoder_titre::<Arrete>(reste) else { return };
            assert_eq!(charge_canonique(&a), reste, "arrêté non canonique");
            let conforme = constitution.controler(&a).is_ok();
            let brut = sceller(&a, &cle).expect("arrêté décodé scellable");
            match sceller_arrete(&a, &cle, &constitution) {
                Ok(o) => {
                    assert!(conforme);
                    assert_eq!(o, brut, "scellement non déterministe");
                }
                Err(_) => assert!(!conforme, "arrêté conforme refusé au scellement"),
            }
            let mut ancres = AncresDeConfiance::new();
            ancres.ajouter(&cle.cle_publique(), NiveauConfiance::Souverain).expect("ancre");
            match verifier_arrete(&brut, &ancres, &constitution, a.pas_avant) {
                Ok(v) => {
                    assert!(conforme, "arrêté hors constitution admis");
                    assert_eq!(v.arrete(), &a);
                    assert!(v.en_vigueur(a.pas_avant));
                }
                Err(_) => assert!(!conforme, "arrêté conforme, signé et en vigueur refusé"),
            }
            let mut reconnues = AncresDeConfiance::new();
            reconnues.ajouter(&cle.cle_publique(), NiveauConfiance::Reconnu).expect("ancre");
            assert!(verifier_arrete(&brut, &reconnues, &constitution, a.pas_avant).is_err(), "ancre reconnue admise");
        }
        _ => {
            if let Some((r, suite)) = Reseau::decoder(reste) {
                let enc = r.encoder();
                assert_eq!(&reste[..reste.len() - suite.len()], &enc[..], "réseau non canonique");
                assert!(r.contient(r.adresse()));
            }
        }
    }
});
