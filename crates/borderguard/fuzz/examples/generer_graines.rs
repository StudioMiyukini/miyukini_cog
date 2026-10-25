//! @id     borderguard_fuzz_generer_graines
//! @do     generer_corpus_initial_fuzzing
//! @role   securite
//! @layer  core
//! @human  Fuzzing — génère le corpus initial déterministe de la cible des arrêtés (EX-10).

//! `cargo run --example generer_graines` (stable) : écrit `seeds/<cible>/*`.

use std::fs;
use std::path::Path;

use borderguard::{sceller_arrete, Arrete, Constitution, Flux, Frontiere, Protocole, Reseau, Voie, Zone};
use miyusceau::{CleSignature, EcrivainTlv, Titre};

const T0: u64 = 1_800_000_000;

fn ecrire(cible: &str, nom: &str, octets: &[u8]) {
    let dossier = Path::new(env!("CARGO_MANIFEST_DIR")).join("seeds").join(cible);
    fs::create_dir_all(&dossier).expect("dossier de graines");
    fs::write(dossier.join(nom), octets).expect("écriture de graine");
}

fn flux(voie: &str, source: Zone, destination: Zone, port: u16, pairs: Vec<Reseau>) -> Flux {
    Flux {
        voie: voie.into(),
        source,
        destination,
        protocole: Protocole::Tcp,
        port,
        pairs,
        debit_max_kio_s: 512,
        taille_max_paquet: 1400,
    }
}

fn main() {
    let cle = CleSignature::depuis_graine(&[7; 32]);
    let portail = Arrete {
        frontiere: Frontiere::PortailInterne,
        numero: 3,
        emis_le: T0,
        pas_avant: T0,
        pas_apres: T0 + 86_400,
        emetteur: "borderguard".into(),
        motif: "installation jayoffice".into(),
        voies: vec![Voie { id: "jayoffice".into(), service: "jayoffice".into() }],
        flux: vec![flux("jayoffice", Zone::Cog, Zone::Franche, 7443, vec![])],
    };
    let reseau = Reseau::nouveau("93.184.216.0".parse().expect("ip"), 24).expect("réseau");
    let reseau6 = Reseau::nouveau("2001:db8::".parse().expect("ip"), 32).expect("réseau");
    let mut externe = portail.clone();
    externe.frontiere = Frontiere::Externe;
    externe.voies = vec![
        Voie { id: "jaymail".into(), service: "jaymail".into() },
        Voie { id: "site".into(), service: "site".into() },
    ];
    externe.flux = vec![
        flux("jaymail", Zone::Franche, Zone::International, 443, vec![reseau, reseau6]),
        flux("site", Zone::International, Zone::Franche, 443, vec![reseau]),
    ];

    for (nom, a) in [("portail", &portail), ("externe", &externe)] {
        let mut e = EcrivainTlv::new();
        a.encoder(&mut e).expect("charge");
        ecrire("arrete", &format!("charge_{nom}"), &[&[1][..], &e.finir()].concat());
        let s = sceller_arrete(a, &cle, &Constitution::reference()).expect("arrêté de graine conforme");
        let mut entree = vec![0, 0x01];
        entree.extend_from_slice(&(T0 + 10).to_be_bytes());
        entree.extend_from_slice(&s);
        ecrire("arrete", &format!("sceau_{nom}"), &entree);
    }
    ecrire("arrete", "reseau_v4", &[&[2][..], &reseau.encoder()].concat());
    ecrire("arrete", "reseau_v6", &[&[2][..], &reseau6.encoder()].concat());
}
