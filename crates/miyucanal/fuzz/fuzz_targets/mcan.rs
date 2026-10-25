//! @id     miyucanal_fuzz_mcan
//! @do     fuzzer_protocole_canal_officiel
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — demandes `MCAN`, réponses « entrées » du dépôt non fiable et annonces 2 sur 2 (EX-10, C-26).

//! Entrée : `mode (1) | reste`.
//!
//! - 0 : demande brute → `Demande::decoder` (côté dépôt) ;
//! - 1 : réponse « entrées » brute → `decoder_entrees` (nombre attendu = les
//!   4 premiers octets), puis chaque entrée vérifiée comme annonce ;
//! - 2 : charge d'entrée quelconque scellée par Origin et/ou l'éditeur
//!   (bits 0-1 du deuxième octet) → `verifier_annonce`.
//!
//! Propriétés : aucune panique ; décodages canoniques ; une annonce n'est
//! admise que publication **et** cosignée par Origin **et** l'éditeur.

#![no_main]

use libfuzzer_sys::fuzz_target;
use miyucanal::annonce::verifier_entree_canal;
use miyucanal::{decoder_entrees, encoder_entrees, verifier_annonce, AncresCanal, Demande, MAX_ENTREES_PAR_DEMANDE};
use miyuregistre::{Corps, EntreeRegistre};
use miyusceau::{decoder_titre, sceller_multi, CleSignature, TOLERANCE_HORLOGE_SECS};

const MAINTENANT: u64 = 1_800_000_000;

fn controler_annonce(sceau: &[u8], ancres: &AncresCanal) {
    if let Ok(a) = verifier_annonce(sceau, ancres, MAINTENANT) {
        assert_eq!(a.signataires().len(), 2);
        assert!(a.signataires().contains(ancres.origin()) && a.signataires().contains(ancres.editeur()));
        assert_eq!(a.sceau(), sceau);
        assert!(verifier_entree_canal(sceau, ancres, MAINTENANT).is_ok());
    }
}

fuzz_target!(|octets: &[u8]| {
    let Some((&mode, reste)) = octets.split_first() else { return };
    let origin = CleSignature::depuis_graine(&[51; 32]);
    let editeur = CleSignature::depuis_graine(&[52; 32]);
    let ancres = AncresCanal::nouvelles(&origin.cle_publique(), &editeur.cle_publique()).expect("ancres");
    match mode % 3 {
        0 => {
            if let Ok(d) = Demande::decoder(reste) {
                assert_eq!(d.encoder(), reste, "demande non canonique");
            }
        }
        1 => {
            // Contrat de l'appelant (releveur) : au plus MAX_ENTREES_PAR_DEMANDE
            // entrées demandées ; `decoder_entrees` préalloue ce nombre.
            let attendues = reste.get(0..4).map_or(0, |n| u32::from_be_bytes([n[0], n[1], n[2], n[3]]));
            if attendues > MAX_ENTREES_PAR_DEMANDE {
                return;
            }
            if let Ok(entrees) = decoder_entrees(reste, attendues) {
                assert_eq!(entrees.len() as u64, u64::from(attendues));
                assert_eq!(encoder_entrees(&entrees), reste, "réponse « entrées » non canonique");
                for e in &entrees {
                    controler_annonce(e, &ancres);
                }
            }
        }
        _ => {
            let Some((&signataires, charge)) = reste.split_first() else { return };
            let Ok(entree) = decoder_titre::<EntreeRegistre>(charge) else { return };
            let cles: Vec<&CleSignature> = match signataires % 4 {
                0 => vec![&origin, &editeur],
                1 => vec![&origin],
                2 => vec![&editeur],
                _ => vec![&editeur, &origin],
            };
            let sceau = sceller_multi(&entree, &cles).expect("entrée admise scellable");
            let r = verifier_annonce(&sceau, &ancres, MAINTENANT);
            if r.is_ok() {
                assert_eq!(cles.len(), 2, "annonce admise avec une seule signature");
                assert!(matches!(entree.corps, Corps::Publication(_)), "annonce admise hors publication");
                assert!(entree.inscrite_a <= MAINTENANT + TOLERANCE_HORLOGE_SECS, "annonce admise venue du futur");
            }
            controler_annonce(&sceau, &ancres);
        }
    }
});
