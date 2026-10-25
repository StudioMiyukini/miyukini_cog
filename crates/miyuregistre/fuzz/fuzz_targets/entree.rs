//! @id     miyuregistre_fuzz_entree
//! @do     fuzzer_decodage_entree_registre
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — décodage des entrées du registre (titre Sceau 0x04) et des états persistants du temps (EX-10, C-26).

//! Entrée : `mode (1) | reste`.
//!
//! - 0 : sceau d'entrée complet → `controler_structure_entree`, `identite_sceau` ;
//! - 1 : charge TLV → `decoder_titre::<EntreeRegistre>`, puis scellement par
//!   une clé fixe et vérification complète ;
//! - 2 : état `TempsMedian` ; 3 : état `HorlogeConfiance`.
//!
//! Propriétés : aucune panique ; tout titre ou état admis est canonique
//! (ré-encodage identique) ; un titre admis scellé est relu à l'identique.

#![no_main]

use libfuzzer_sys::fuzz_target;
use miyuregistre::{controler_structure_entree, identite_sceau, EntreeRegistre, HorlogeConfiance, TempsMedian};
use miyusceau::{
    decoder_titre, sceller, verifier, AncresDeConfiance, CleSignature, EcrivainTlv, NiveauConfiance, Sceau, Titre,
};

fn charge_canonique(entree: &EntreeRegistre) -> Vec<u8> {
    let mut e = EcrivainTlv::new();
    entree.encoder(&mut e).expect("entrée admise ré-encodable");
    e.finir()
}

fuzz_target!(|octets: &[u8]| {
    let Some((&mode, reste)) = octets.split_first() else { return };
    match mode % 4 {
        0 => {
            let sceau = Sceau::decoder(reste);
            assert_eq!(identite_sceau(reste).is_ok(), sceau.is_ok());
            if let Ok(entree) = controler_structure_entree(reste) {
                let sceau = sceau.expect("entrée admise : sceau décodé");
                assert_eq!(charge_canonique(&entree), sceau.charge, "entrée non canonique");
            }
        }
        1 => {
            let Ok(entree) = decoder_titre::<EntreeRegistre>(reste) else { return };
            assert_eq!(charge_canonique(&entree), reste, "entrée non canonique");
            let cle = CleSignature::depuis_graine(&[41; 32]);
            let scelle = sceller(&entree, &cle).expect("entrée admise scellable");
            assert_eq!(controler_structure_entree(&scelle).expect("entrée scellée relue"), entree);
            let mut ancres = AncresDeConfiance::new();
            ancres.ajouter(&cle.cle_publique(), NiveauConfiance::Souverain).expect("ancre");
            if let Ok(v) = verifier::<EntreeRegistre>(&scelle, &ancres, entree.inscrite_a) {
                assert_eq!(v.titre(), &entree);
            }
        }
        2 => {
            if let Ok(t) = TempsMedian::decoder(reste) {
                assert_eq!(t.encoder(), reste, "fenêtre MTP non canonique");
                if let Some(m) = t.mtp() {
                    assert!(t.fenetre().any(|h| h == m), "MTP hors fenêtre");
                    assert!(t.controler(m).is_ok(), "MTP refusé par son propre contrôle");
                }
                let _ = t.controler(u64::MAX);
                let _ = t.controler(0);
            }
        }
        _ => {
            if let Ok(mut h) = HorlogeConfiance::decoder(reste) {
                assert_eq!(h.encoder(), reste, "horloge non canonique");
                let avant = h.clone();
                if let Ok(m) = h.maintenant(u64::MAX) {
                    assert_eq!(m, u64::MAX);
                }
                let _ = avant.mtp();
            }
        }
    }
});
