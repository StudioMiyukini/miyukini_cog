//! @id     miyudouane_tests_commun
//! @do     fournir_arretes_de_test
//! @role   test
//! @layer  operateur
//! @human  miyudouane — arrêtés et plan de test partagés.

#![allow(dead_code)]

use borderguard::{sceller_arrete, verifier_arrete, Arrete, ArreteVerifie, Constitution, Flux, Frontiere, Protocole, Voie, Zone};
use miyudouane::PlanReseau;
use miyusceau::{AncresDeConfiance, CleSignature, NiveauConfiance};

pub const T0: u64 = 1_800_000_000;

pub fn cle() -> CleSignature {
    CleSignature::depuis_graine(&[4u8; 32])
}

pub fn plan() -> PlanReseau {
    PlanReseau::reference(&[
        (Zone::Cog, "jayoffice"),
        (Zone::Franche, "jayoffice"),
        (Zone::Franche, "site"),
        (Zone::Cog, "alicia"),
        (Zone::FrancheLocale, "alicia"),
        (Zone::Cog, "site"),
    ])
    .unwrap()
}

pub fn flux(voie: &str, source: Zone, destination: Zone, protocole: Protocole, port: u16, pairs: &[&str]) -> Flux {
    Flux {
        voie: voie.into(),
        source,
        destination,
        protocole,
        port,
        pairs: pairs.iter().map(|p| p.parse().unwrap()).collect(),
        debit_max_kio_s: 512,
        taille_max_paquet: 1400,
    }
}

pub fn voie(id: &str) -> Voie {
    Voie { id: id.into(), service: id.into() }
}

pub fn arrete(frontiere: Frontiere, numero: u64, voies: Vec<Voie>, flux: Vec<Flux>) -> Arrete {
    Arrete {
        frontiere,
        numero,
        emis_le: T0,
        pas_avant: T0,
        pas_apres: T0 + 30 * 86_400,
        emetteur: "borderguard".into(),
        motif: "essai".into(),
        voies,
        flux,
    }
}

pub fn arrete_portail(numero: u64) -> Arrete {
    arrete(
        Frontiere::PortailInterne,
        numero,
        vec![voie("jayoffice"), voie("site")],
        vec![
            flux("jayoffice", Zone::Cog, Zone::Franche, Protocole::Tcp, 7443, &[]),
            flux("site", Zone::Cog, Zone::Franche, Protocole::Tcp, 7080, &[]),
        ],
    )
}

pub fn arrete_externe(numero: u64) -> Arrete {
    arrete(
        Frontiere::Externe,
        numero,
        vec![voie("jayoffice"), voie("site")],
        vec![
            flux("site", Zone::International, Zone::Franche, Protocole::Tcp, 443, &["203.0.113.0/24", "198.51.100.7/32"]),
            flux("jayoffice", Zone::Federation, Zone::Franche, Protocole::Udp, 51820, &[]),
        ],
    )
}

pub fn arrete_local(numero: u64) -> Arrete {
    arrete(
        Frontiere::PortailLocal,
        numero,
        vec![voie("alicia")],
        vec![flux("alicia", Zone::Cog, Zone::FrancheLocale, Protocole::Tcp, 1883, &[])],
    )
}

pub fn verifie(a: &Arrete) -> ArreteVerifie {
    let c = cle();
    let octets = sceller_arrete(a, &c, &Constitution::reference()).unwrap();
    let mut ancres = AncresDeConfiance::new();
    ancres.ajouter(&c.cle_publique(), NiveauConfiance::Souverain).unwrap();
    verifier_arrete(&octets, &ancres, &Constitution::reference(), T0 + 1).unwrap()
}
