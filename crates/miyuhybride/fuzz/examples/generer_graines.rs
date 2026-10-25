//! @id     miyuhybride_fuzz_generer_graines
//! @do     generer_corpus_initial_fuzzing
//! @role   securite
//! @layer  toolkit
//! @human  Fuzzing — génère le corpus initial déterministe de la cible `contrat` (C-26).

//! `cargo run --example generer_graines` (stable) : écrit `seeds/<cible>/*`.
//! Clé d'éditeur fixe identique à celle de la cible.

use std::fs;
use std::path::Path;

use miyuhybride::{
    sceller_manifeste, ClasseExposition, ContratFacade, ContratService, ContratVoie, IdService, ManifesteService,
    OrigineExterne, PointEntree, Ressources, Sens, Traite, Transport,
};
use miyuportail::{ChampTraite, NatureChamp, TypeDeclaration};
use miyusceau::{CleSignature, Sceau};

fn ecrire(cible: &str, nom: &str, octets: &[u8]) {
    let dossier = Path::new(env!("CARGO_MANIFEST_DIR")).join("seeds").join(cible);
    fs::create_dir_all(&dossier).expect("dossier de graines");
    fs::write(dossier.join(nom), octets).expect("écriture de graine");
}

fn avec_mode(mode: u8, octets: &[u8]) -> Vec<u8> {
    [&[mode][..], octets].concat()
}

fn main() {
    let ressources = Ressources { cpu_pourcent: 50, memoire_mio: 256, disque_mio: 64, taches_max: 64, sessions_max: 4 };
    let h0 = ContratService {
        classe: ClasseExposition::H0,
        ressources,
        voie: None,
        traite: None,
        facade: None,
        entrees: Vec::new(),
    };
    let h2 = ContratService {
        classe: ClasseExposition::H2,
        ressources,
        voie: Some(ContratVoie { taille_max_trame: 8192, debit_max_par_minute: 100, quota_file: 64 }),
        traite: Some(Traite {
            version: 1,
            types: vec![
                TypeDeclaration {
                    code: 1,
                    sens: Sens::Entrante,
                    champs: vec![ChampTraite { tag: 1, nature: NatureChamp::Texte { max: 64 }, obligatoire: true }],
                },
                TypeDeclaration {
                    code: 2,
                    sens: Sens::Sortante,
                    champs: vec![ChampTraite { tag: 1, nature: NatureChamp::Entier, obligatoire: true }],
                },
            ],
        }),
        facade: Some(ContratFacade { image: [9; 32], db_secondaire_max_mio: 8, duree_max_session_secs: 3600 }),
        entrees: vec![PointEntree {
            origine: OrigineExterne::Federation,
            transport: Transport::Tcp,
            port: 443,
            debit_max_kio_s: 512,
            taille_max_paquet: 1500,
        }],
    };
    let cle = CleSignature::depuis_graine(&[0x5e; 32]);
    for (nom, c) in [("h0", h0), ("h2", h2)] {
        ecrire("contrat", &format!("contrat_{nom}"), &avec_mode(0, &c.encoder().expect("contrat")));
        let m = ManifesteService {
            service: IdService::new("jayrdv").expect("service"),
            version: "0.1.0".into(),
            sha256_archive: [3; 32],
            nom_binaire: Some("jayrdv".into()),
            contrat: c,
        };
        let sceau = sceller_manifeste(&m, &cle).expect("manifeste");
        ecrire("contrat", &format!("charge_{nom}"), &avec_mode(1, &Sceau::decoder(&sceau).expect("sceau").charge));
        ecrire("contrat", &format!("sceau_{nom}"), &avec_mode(2, &sceau));
    }
}
