//! @id     miyuhybride_fuzz_contrat
//! @do     fuzzer_contrat_de_service
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — contrat de service (tag 5 du manifeste signé) et manifeste de service : décodage canonique, règles H0–H3 (EX-16, EX-40, EX-48, C-26).

//! Entrée : `mode (1) | reste`.
//!
//! - 0 : contrat brut → `ContratService::decoder`, puis `valider`, puis
//!   manifeste scellé par l'éditeur fixe et revérifié ;
//! - 1 : charge TLV brute → titre `ManifesteService` ;
//! - autre : sceau brut → `verifier_manifeste` (éditeur fixe épinglé).
//!
//! Propriétés : aucune panique ; décodages canoniques (ré-encodage
//! identique) ; un contrat validé se scelle et se relit à l'identique ; un
//! manifeste vérifié a un contrat valide, jamais H3, exposition par défaut
//! désactivée, et H0 sans voie ni façade.

#![no_main]

use libfuzzer_sys::fuzz_target;
use miyuhybride::{sceller_manifeste, verifier_manifeste, ClasseExposition, ContratService, IdService, ManifesteService};
use miyusceau::{decoder_titre, AncresDeConfiance, CleSignature, EcrivainTlv, NiveauConfiance, Titre};

const MAINTENANT: u64 = 1_800_000_000;

fn editeur() -> (CleSignature, AncresDeConfiance) {
    let cle = CleSignature::depuis_graine(&[0x5e; 32]);
    let mut ancres = AncresDeConfiance::new();
    ancres.ajouter(&cle.cle_publique(), NiveauConfiance::Souverain).expect("ancre");
    (cle, ancres)
}

fn controler_verifie(m: &ManifesteService) {
    let c = &m.contrat;
    c.valider(&m.service).expect("contrat d'un manifeste vérifié valide");
    assert_ne!(c.classe, ClasseExposition::H3, "H3 admis");
    if c.classe == ClasseExposition::H0 {
        assert!(c.voie.is_none() && c.traite.is_none() && c.facade.is_none() && c.entrees.is_empty());
        assert!(c.contrat_voies(&m.service).is_none());
    } else {
        c.contrat_voies(&m.service).expect("contrat de voies").valider().expect("contrat de voies valide");
    }
}

fn contrat_brut(octets: &[u8]) {
    let Ok(c) = ContratService::decoder(octets) else { return };
    assert_eq!(c.encoder().expect("contrat décodé ré-encodable"), octets, "contrat non canonique");
    let service = IdService::new("fuzz.service").expect("service");
    if c.valider(&service).is_err() {
        return;
    }
    let m = ManifesteService {
        service,
        version: "1.0.0".into(),
        sha256_archive: [3; 32],
        nom_binaire: None,
        contrat: c,
    };
    let (cle, ancres) = editeur();
    let sceau = sceller_manifeste(&m, &cle).expect("contrat validé scellable");
    let v = verifier_manifeste(&sceau, &ancres, MAINTENANT).expect("manifeste scellé vérifiable");
    assert_eq!(v.manifeste(), &m, "aller-retour du manifeste différent");
    controler_verifie(v.manifeste());
}

fn charge_brute(charge: &[u8]) {
    let Ok(m) = decoder_titre::<ManifesteService>(charge) else { return };
    let mut e = EcrivainTlv::new();
    m.encoder(&mut e).expect("manifeste décodé ré-encodable");
    assert_eq!(e.finir(), charge, "manifeste non canonique");
    controler_verifie(&m);
}

fuzz_target!(|octets: &[u8]| {
    let Some((&mode, reste)) = octets.split_first() else { return };
    match mode {
        0 => contrat_brut(reste),
        1 => charge_brute(reste),
        _ => {
            let (_, ancres) = editeur();
            if let Ok(v) = verifier_manifeste(reste, &ancres, MAINTENANT) {
                controler_verifie(v.manifeste());
            }
            assert!(verifier_manifeste(reste, &AncresDeConfiance::new(), MAINTENANT).is_err(), "sans ancre admis");
        }
    }
});
