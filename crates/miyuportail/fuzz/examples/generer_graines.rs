//! @id     miyuportail_fuzz_generer_graines
//! @do     generer_corpus_initial_fuzzing
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — génère le corpus initial déterministe des cibles du Portail (EX-10).

//! `cargo run --example generer_graines` (stable) : écrit `seeds/<cible>/*`.
//! Clés fixes identiques à celles des cibles (`fuzz_targets/commun.rs`).

#[path = "../fuzz_targets/commun.rs"]
mod commun;

use std::fs;
use std::path::Path;

use miyuportail::msce::{encoder_charge, encoder_interne, sceller, Champ, Declaration, Valeur};
use miyuportail::sas::traiter_requete;
use miyuportail::{RequeteSas, Sens};

fn ecrire(cible: &str, nom: &str, octets: &[u8]) {
    let dossier = Path::new(env!("CARGO_MANIFEST_DIR")).join("seeds").join(cible);
    fs::create_dir_all(&dossier).expect("dossier de graines");
    fs::write(dossier.join(nom), octets).expect("écriture de graine");
}

fn declaration(sens: Sens, type_code: u64, champs: Vec<Champ>) -> Declaration {
    Declaration {
        service: commun::service(),
        sens,
        version_traite: commun::VERSION_TRAITE,
        type_code,
        sequence: 3,
        emise_le: 1_800_000_000,
        champs,
        signataires: Vec::new(),
    }
}

fn champs_complets() -> Vec<Champ> {
    vec![
        Champ { tag: 1, valeur: Valeur::Texte("bonjour\tà tous\n".into()) },
        Champ { tag: 2, valeur: Valeur::Entier(42) },
        Champ { tag: 3, valeur: Valeur::Booleen(true) },
        Champ { tag: 5, valeur: Valeur::Octets(vec![1, 2, 3]) },
        Champ { tag: 9, valeur: Valeur::Document(b"texte".to_vec()) },
        Champ { tag: 200, valeur: Valeur::Entier(7) },
    ]
}

fn champs_minimaux() -> Vec<Champ> {
    vec![Champ { tag: 1, valeur: Valeur::Texte("x".into()) }, Champ { tag: 5, valeur: Valeur::Octets(vec![0]) }]
}

fn avec_mode(mode: u8, octets: &[u8]) -> Vec<u8> {
    let mut v = vec![mode];
    v.extend_from_slice(octets);
    v
}

fn main() {
    let emetteur = commun::cle_emetteur();
    let auteur = commun::cle_auteur();
    let entrante = declaration(Sens::Entrante, commun::ENTRANT, champs_complets());
    let minimale = declaration(Sens::Entrante, commun::ENTRANT, champs_minimaux());
    let sortante = declaration(Sens::Sortante, commun::SORTANT, champs_complets());
    let charge_e = encoder_charge(&entrante).expect("charge");
    let charge_m = encoder_charge(&minimale).expect("charge");
    let charge_s = encoder_charge(&sortante).expect("charge");
    let trame_simple = sceller(&entrante, &[&emetteur]).expect("trame");
    let trame_double = sceller(&entrante, &[&emetteur, &auteur]).expect("trame");

    for cible in ["decoder_trame", "validation2"] {
        ecrire(cible, "charge_simple", &avec_mode(0x01, &charge_e));
        ecrire(cible, "charge_minimale", &avec_mode(0x01, &charge_m));
        ecrire(cible, "charge_cosignee", &avec_mode(0x02, &charge_e));
        ecrire(cible, "charge_sortante", &avec_mode(0x05, &charge_s));
        ecrire(cible, "trame_simple", &avec_mode(0x00, &trame_simple));
        ecrire(cible, "trame_cosignee", &avec_mode(0x08, &trame_double));
    }

    let requete = RequeteSas {
        service: commun::service(),
        ancres: commun::ancres(true),
        traite: commun::traite().restreindre(Sens::Entrante),
        trames: vec![trame_double.clone(), trame_simple.clone()],
    };
    let octets_requete = requete.encoder().expect("requête");
    ecrire("requete_sas", "requete", &avec_mode(0x00, &octets_requete));
    ecrire("requete_sas", "traite", &avec_mode(0x01, &commun::traite().encoder()));
    let mut lot = charge_e.clone();
    lot.extend_from_slice(&[0xFF, 0xFF]);
    lot.extend_from_slice(&charge_m);
    ecrire("requete_sas", "lot_simple", &avec_mode(0x02, &lot));
    ecrire("requete_sas", "lot_cosigne", &avec_mode(0x03, &lot));

    let reponse = traiter_requete(&octets_requete).expect("réponse");
    ecrire("reponse_sas", "reponse", &avec_mode(2, &reponse));
    let mut d = entrante.clone();
    d.signataires = vec![[1; 32], [2; 32]];
    ecrire("reponse_sas", "interne", &avec_mode(0, &encoder_interne(&d)));

    // Déclaration tirée (`commun::declaration_tiree`) : perturbation, type,
    // séquence, présence, texte, entier, booléen, octets, document, entier.
    let tiree = |sequence: u8| -> Vec<u8> {
        let mut v = vec![0x00, 0x00, sequence, 0x0F, 3, b'a', b'b', b'c'];
        v.extend_from_slice(&42u64.to_be_bytes());
        v.push(1);
        v.extend_from_slice(&[2, 0xAA, 0xBB]);
        v.extend_from_slice(&[1, b't', b'e', b'x', b't']);
        v.extend_from_slice(&7u64.to_be_bytes());
        v
    };
    for (nom, mode) in [("valide", 0u8), ("inversee", 1), ("brute", 2), ("doublon", 3)] {
        let mut v = vec![mode];
        v.extend_from_slice(&tiree(5));
        if mode == 1 {
            v.extend_from_slice(&[40, 0x80]);
        }
        ecrire("cosignature", nom, &v);
    }

    // Portail : activer, session (auteurs 0 et 1), dépôts, relève, rejeu,
    // retrait, coupure, rétablissement, publication puis lecture.
    let mut p = vec![0, 1, 0x03];
    for (auteur, sequence) in [(0u8, 5u8), (1, 1), (0, 5), (0, 6)] {
        p.extend_from_slice(&[2, auteur]);
        p.extend_from_slice(&tiree(sequence));
    }
    p.extend_from_slice(&[6, 10, 9, 0, 6, 3]);
    p.extend_from_slice(&[11, 0]);
    let mut sortante = tiree(0);
    sortante[3] = 0x03;
    sortante.truncate(8 + 8 + 1 + 3);
    p.extend_from_slice(&sortante);
    p.extend_from_slice(&[0, 0, 11, 1, 5, 0, 4, b'M', b'S', b'C', b'E', 6, 2]);
    p.extend_from_slice(&[10, 0, 10, 3, 0, 1, 0x0F, 6, 1]);
    ecrire("portail", "scenario", &p);
}
