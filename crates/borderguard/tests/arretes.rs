//! @id     borderguard_tests_arretes
//! @do     tester_arretes_signes
//! @role   test
//! @layer  core
//! @human  borderguard — arrêtés : signature, ancres, expiration, constitution, décodage strict.

#![allow(clippy::assert_is_empty)]

use borderguard::{
    sceller_arrete, verifier_arrete, Arrete, Constitution, ErreurArrete, Flux, Frontiere, Protocole, Reseau, Voie,
    Zone,
};
use miyusceau::{AncresDeConfiance, CleSignature, ErreurSceau, NiveauConfiance};

const T0: u64 = 1_800_000_000;

fn cle() -> CleSignature {
    CleSignature::depuis_graine(&[7u8; 32])
}

fn ancres(cle: &CleSignature, niveau: NiveauConfiance) -> AncresDeConfiance {
    let mut a = AncresDeConfiance::new();
    a.ajouter(&cle.cle_publique(), niveau).unwrap();
    a
}

fn flux(voie: &str, source: Zone, destination: Zone, port: u16) -> Flux {
    Flux {
        voie: voie.into(),
        source,
        destination,
        protocole: Protocole::Tcp,
        port,
        pairs: vec![],
        debit_max_kio_s: 512,
        taille_max_paquet: 1400,
    }
}

fn arrete_portail() -> Arrete {
    Arrete {
        frontiere: Frontiere::PortailInterne,
        numero: 3,
        emis_le: T0,
        pas_avant: T0,
        pas_apres: T0 + 86_400,
        emetteur: "borderguard".into(),
        motif: "installation jayoffice".into(),
        voies: vec![Voie { id: "jayoffice".into(), service: "jayoffice".into() }],
        flux: vec![flux("jayoffice", Zone::Cog, Zone::Franche, 7443)],
    }
}

fn refus(a: &Arrete) -> ErreurArrete {
    sceller_arrete(a, &cle(), &Constitution::reference()).unwrap_err()
}

#[test]
fn aller_retour_signe() {
    let c = cle();
    let a = arrete_portail();
    let octets = sceller_arrete(&a, &c, &Constitution::reference()).unwrap();
    let v = verifier_arrete(&octets, &ancres(&c, NiveauConfiance::Souverain), &Constitution::reference(), T0 + 10)
        .unwrap();
    assert_eq!(v.arrete(), &a);
    assert_eq!(v.signataire(), &c.kid());
    assert!(v.en_vigueur(T0 + 10));
    assert!(!v.en_vigueur(T0 + 86_400));
    assert_eq!(v.empreinte_hex().len(), 64);
}

#[test]
fn ancre_inconnue_ou_non_souveraine_refusee() {
    let c = cle();
    let octets = sceller_arrete(&arrete_portail(), &c, &Constitution::reference()).unwrap();
    let autre = CleSignature::depuis_graine(&[9u8; 32]);
    assert_eq!(
        verifier_arrete(&octets, &ancres(&autre, NiveauConfiance::Souverain), &Constitution::reference(), T0)
            .unwrap_err(),
        ErreurArrete::Sceau(ErreurSceau::SignataireInconnu)
    );
    assert_eq!(
        verifier_arrete(&octets, &AncresDeConfiance::new(), &Constitution::reference(), T0).unwrap_err(),
        ErreurArrete::Sceau(ErreurSceau::SignataireInconnu)
    );
    // Clé d'un COG reconnu (Fédération, Origin) : jamais d'arrêté étranger.
    assert_eq!(
        verifier_arrete(&octets, &ancres(&c, NiveauConfiance::Reconnu), &Constitution::reference(), T0).unwrap_err(),
        ErreurArrete::SignataireNonSouverain
    );
}

#[test]
fn expiration_et_fenetre() {
    let c = cle();
    let octets = sceller_arrete(&arrete_portail(), &c, &Constitution::reference()).unwrap();
    let a = ancres(&c, NiveauConfiance::Souverain);
    let k = Constitution::reference();
    assert_eq!(
        verifier_arrete(&octets, &a, &k, T0 + 86_400 + 1_000).unwrap_err(),
        ErreurArrete::Sceau(ErreurSceau::Expire)
    );
    assert_eq!(verifier_arrete(&octets, &a, &k, T0 - 1_000).unwrap_err(), ErreurArrete::Sceau(ErreurSceau::PasEncoreValide));
    // Fenêtre au-delà de 90 jours : refusée dès l'encodage.
    let mut long = arrete_portail();
    long.pas_apres = long.pas_avant + 91 * 86_400;
    assert!(matches!(refus(&long), ErreurArrete::Sceau(ErreurSceau::ValeurInvalide(0x07))));
}

#[test]
fn alteration_refusee() {
    let c = cle();
    let mut octets = sceller_arrete(&arrete_portail(), &c, &Constitution::reference()).unwrap();
    let milieu = octets.len() / 2;
    octets[milieu] ^= 0x01;
    assert!(verifier_arrete(&octets, &ancres(&c, NiveauConfiance::Souverain), &Constitution::reference(), T0).is_err());
}

#[test]
fn refus_par_defaut_et_decodage_strict() {
    // Aucun flux : un arrêté vide n'autorise rien et n'est pas émis.
    let mut vide = arrete_portail();
    vide.flux.clear();
    assert!(matches!(refus(&vide), ErreurArrete::Sceau(ErreurSceau::ValeurInvalide(0x0B))));
    // Flux sur une voie non déclarée.
    let mut orphelin = arrete_portail();
    orphelin.flux[0].voie = "fantome".into();
    assert!(matches!(refus(&orphelin), ErreurArrete::Sceau(ErreurSceau::ValeurInvalide(0x0B))));
    // Voie en double.
    let mut double = arrete_portail();
    double.voies.push(Voie { id: "jayoffice".into(), service: "autre".into() });
    assert!(matches!(refus(&double), ErreurArrete::Sceau(ErreurSceau::ValeurInvalide(0x0A))));
    // Identifiant dangereux (injection dans les règles générées).
    let mut injection = arrete_portail();
    injection.voies[0].id = "x\" accept".into();
    injection.flux[0].voie = "x\" accept".into();
    assert!(refus(&injection).to_string().contains("sceau"));
    // Numéro 0, port 0, débit nul, paquet trop petit.
    let mut a = arrete_portail();
    a.numero = 0;
    assert!(matches!(refus(&a), ErreurArrete::Sceau(ErreurSceau::ValeurInvalide(0x04))));
    for (port, debit, taille) in [(0, 1, 1400), (1, 0, 1400), (1, 1, 10)] {
        let mut a = arrete_portail();
        a.flux[0].port = port;
        a.flux[0].debit_max_kio_s = debit;
        a.flux[0].taille_max_paquet = taille;
        assert!(matches!(refus(&a), ErreurArrete::Sceau(ErreurSceau::ValeurInvalide(_))));
    }
    // Flux en double.
    let mut a = arrete_portail();
    a.flux.push(a.flux[0].clone());
    assert!(matches!(refus(&a), ErreurArrete::Sceau(ErreurSceau::ValeurInvalide(0x0B))));
}

#[test]
fn encodage_canonique() {
    let c = cle();
    let mut a = arrete_portail();
    a.voies.push(Voie { id: "jaymessage".into(), service: "jaymessage".into() });
    a.flux.push(flux("jaymessage", Zone::Cog, Zone::Franche, 7444));
    let mut b = a.clone();
    b.voies.reverse();
    b.flux.reverse();
    let k = Constitution::reference();
    assert_eq!(sceller_arrete(&a, &c, &k).unwrap(), sceller_arrete(&b, &c, &k).unwrap());
}

#[test]
fn constitution_la_zone_cog_tire() {
    // Franche → zone COG : interdit (INV-Z2).
    let mut a = arrete_portail();
    a.flux[0] = flux("jayoffice", Zone::Franche, Zone::Cog, 7443);
    assert!(matches!(refus(&a), ErreurArrete::HorsConstitution(_)));
    // Zone COG → International sur la frontière externe (INV-Z1).
    let mut a = arrete_portail();
    a.frontiere = Frontiere::Externe;
    a.flux[0] = flux("jayoffice", Zone::Cog, Zone::International, 443);
    a.flux[0].pairs = vec!["203.0.113.0/24".parse().unwrap()];
    assert!(matches!(refus(&a), ErreurArrete::HorsConstitution(_)));
    // International sans liste blanche (INV-Z6).
    let mut a = arrete_portail();
    a.frontiere = Frontiere::Externe;
    a.flux[0] = flux("jayoffice", Zone::International, Zone::Franche, 443);
    assert!(matches!(refus(&a), ErreurArrete::HorsConstitution(_)));
    a.flux[0].pairs = vec!["203.0.113.0/24".parse::<Reseau>().unwrap()];
    assert!(sceller_arrete(&a, &cle(), &Constitution::reference()).is_ok());
    // Franche locale : jamais d'extérieur.
    let mut a = arrete_portail();
    a.frontiere = Frontiere::Externe;
    a.flux[0] = flux("jayoffice", Zone::Federation, Zone::FrancheLocale, 443);
    assert!(matches!(refus(&a), ErreurArrete::HorsConstitution(_)));
    // Plafond de débit.
    let mut a = arrete_portail();
    a.flux[0].debit_max_kio_s = Constitution::reference().plafond_debit_kio_s + 1;
    assert!(matches!(refus(&a), ErreurArrete::PlafondDepasse(_)));
}

#[test]
fn arrete_hors_constitution_signe_quand_meme_est_refuse() {
    // Un émetteur qui contourne `sceller_arrete` (sceau valide) reste refusé à la vérification.
    let c = cle();
    let mut a = arrete_portail();
    a.flux[0] = flux("jayoffice", Zone::Franche, Zone::Cog, 7443);
    let octets = miyusceau::sceller(&a, &c).unwrap();
    assert!(matches!(
        verifier_arrete(&octets, &ancres(&c, NiveauConfiance::Souverain), &Constitution::reference(), T0),
        Err(ErreurArrete::HorsConstitution(_))
    ));
}

#[test]
fn autre_certificat_refuse() {
    // Un certificat de même type Sceau mais d'une autre nature (passation) n'est pas un arrêté.
    struct Autre;
    impl miyusceau::Titre for Autre {
        const TYPE: miyusceau::TypeSceau = miyusceau::TypeSceau::Certificat;
        fn encoder(&self, e: &mut miyusceau::EcrivainTlv) -> Result<(), ErreurSceau> {
            e.octets(0x01, &[0x01])?;
            Ok(())
        }
        fn decoder(l: &mut miyusceau::LecteurTlv<'_>) -> Result<Self, ErreurSceau> {
            l.exiger(0x01)?;
            Ok(Self)
        }
    }
    let c = cle();
    let octets = miyusceau::sceller(&Autre, &c).unwrap();
    assert_eq!(
        verifier_arrete(&octets, &ancres(&c, NiveauConfiance::Souverain), &Constitution::reference(), T0).unwrap_err(),
        ErreurArrete::Sceau(ErreurSceau::ValeurInvalide(0x01))
    );
}
