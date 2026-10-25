//! @id     miyuhybride_tests_manifeste
//! @do     tester_manifeste_service_signe
//! @role   test
//! @layer  toolkit
//! @human  miyuhybride — manifeste signé : aller-retour, éditeur épinglé, règles de classe H0–H3, exposition par défaut, compatibilité Market (EX-16, EX-40, EX-48).

use miyuhybride::*;
use miyuportail::{ChampTraite, NatureChamp, TypeDeclaration};
use miyusceau::{AncresDeConfiance, CleSignature, EcrivainTlv, NiveauConfiance};

const T0: u64 = 1_800_000_000;

fn traite_h2() -> Traite {
    Traite {
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
    }
}

fn contrat(classe: ClasseExposition) -> ContratService {
    ContratService {
        classe,
        ressources: Ressources { cpu_pourcent: 50, memoire_mio: 256, disque_mio: 64, taches_max: 64, sessions_max: 4 },
        voie: Some(ContratVoie { taille_max_trame: 8192, debit_max_par_minute: 100, quota_file: 64 }),
        traite: Some(traite_h2()),
        facade: Some(ContratFacade { image: [9; 32], db_secondaire_max_mio: 8, duree_max_session_secs: 3600 }),
        entrees: vec![PointEntree {
            origine: OrigineExterne::Federation,
            transport: Transport::Tcp,
            port: 443,
            debit_max_kio_s: 512,
            taille_max_paquet: 1500,
        }],
    }
}

fn manifeste(c: ContratService) -> ManifesteService {
    ManifesteService {
        service: IdService::new("jayrdv").unwrap(),
        version: "0.1.0".into(),
        sha256_archive: [3; 32],
        nom_binaire: None,
        contrat: c,
    }
}

fn editeur() -> (CleSignature, AncresDeConfiance) {
    let cle = CleSignature::generer();
    let mut ancres = AncresDeConfiance::new();
    ancres.ajouter(&cle.cle_publique(), NiveauConfiance::Souverain).unwrap();
    (cle, ancres)
}

#[test]
fn aller_retour_signe_et_contrat_de_voies() {
    let (cle, ancres) = editeur();
    let m = manifeste(contrat(ClasseExposition::H2));
    let octets = sceller_manifeste(&m, &cle).unwrap();
    let v = verifier_manifeste(&octets, &ancres, T0).unwrap();
    assert_eq!(v.manifeste(), &m);
    assert_eq!(v.classe(), ClasseExposition::H2);
    assert_eq!(v.audience(), "jayrdv");
    assert_eq!(v.editeur(), &cle.kid());
    let cv = v.contrat_voies().unwrap();
    cv.valider().unwrap();
    assert_eq!((cv.taille_max_trame, cv.debit_max_par_minute, cv.quota_file), (8192, 100, 64));
    assert_eq!(cv.traite, traite_h2());
}

#[test]
fn editeur_non_epingle_ou_absent_refuse() {
    let (cle, _) = editeur();
    let (_, autres) = editeur();
    let octets = sceller_manifeste(&manifeste(contrat(ClasseExposition::H2)), &cle).unwrap();
    assert!(matches!(verifier_manifeste(&octets, &autres, T0), Err(ErreurManifeste::Sceau(_))));
    assert_eq!(verifier_manifeste(&octets, &AncresDeConfiance::new(), T0).unwrap_err(), ErreurManifeste::AucuneAncre);
}

#[test]
fn manifeste_altere_refuse() {
    let (cle, ancres) = editeur();
    let mut octets = sceller_manifeste(&manifeste(contrat(ClasseExposition::H2)), &cle).unwrap();
    // Un octet de la charge (classe, ressources…) modifié : signature invalide.
    let i = octets.len() / 2;
    octets[i] ^= 1;
    assert!(verifier_manifeste(&octets, &ancres, T0).is_err());
}

#[test]
fn h0_sans_facade_ni_voie() {
    let (cle, ancres) = editeur();
    let mut c = contrat(ClasseExposition::H0);
    assert!(matches!(sceller_manifeste(&manifeste(c.clone()), &cle), Err(ErreurManifeste::Classe(_))));
    c.voie = None;
    c.traite = None;
    c.facade = None;
    c.entrees.clear();
    let v = verifier_manifeste(&sceller_manifeste(&manifeste(c), &cle).unwrap(), &ancres, T0).unwrap();
    assert_eq!(v.classe(), ClasseExposition::H0);
    assert!(v.contrat_voies().is_none(), "H0 : jamais de voie (EX-40)");
}

#[test]
fn h1_publication_seule_et_h3_non_specifie() {
    let (cle, _) = editeur();
    // H1 avec un type entrant : refusé (sens fermé à la classe).
    assert!(sceller_manifeste(&manifeste(contrat(ClasseExposition::H1)), &cle).is_err());
    let mut h1 = contrat(ClasseExposition::H1);
    h1.traite.as_mut().unwrap().types.retain(|t| t.sens == Sens::Sortante);
    sceller_manifeste(&manifeste(h1), &cle).unwrap();
    // H2 sans type entrant : refusé.
    let mut h2 = contrat(ClasseExposition::H2);
    h2.traite.as_mut().unwrap().types.retain(|t| t.sens == Sens::Sortante);
    assert!(matches!(sceller_manifeste(&manifeste(h2), &cle), Err(ErreurManifeste::Classe(_))));
    // H3 : liste blanche sortante et intention StrongFather non encodables → refus.
    assert!(matches!(
        sceller_manifeste(&manifeste(contrat(ClasseExposition::H3)), &cle),
        Err(ErreurManifeste::NonSpecifie(_))
    ));
}

#[test]
fn bornes_du_contrat() {
    let (cle, _) = editeur();
    let mut c = contrat(ClasseExposition::H2);
    c.ressources.memoire_mio = 0;
    assert!(sceller_manifeste(&manifeste(c), &cle).is_err());
    let mut c = contrat(ClasseExposition::H2);
    c.facade.as_mut().unwrap().db_secondaire_max_mio = 65; // > disque
    assert!(sceller_manifeste(&manifeste(c), &cle).is_err());
    let mut c = contrat(ClasseExposition::H2);
    c.entrees.clear();
    assert!(sceller_manifeste(&manifeste(c), &cle).is_err());
    let mut c = contrat(ClasseExposition::H2);
    c.entrees[0].port = 0;
    assert!(sceller_manifeste(&manifeste(c), &cle).is_err());
}

/// Contrat encodé à la main (ce qu'un éditeur fautif pourrait signer).
fn contrat_brut(exposition: u8, origine: u8) -> Vec<u8> {
    let c = contrat(ClasseExposition::H2);
    let mut octets = c.encoder().unwrap();
    // Exposition : 3e champ, valeur d'un octet après « genre » et « classe ».
    let genre = 5 + GENRE_CONTRAT.len();
    let pos_exposition = genre + 6 + 5;
    assert_eq!(octets[pos_exposition], 0);
    octets[pos_exposition] = exposition;
    // Origine de la première entrée : 1er octet après le compteur, en fin de charge.
    let pos_origine = octets.len() - 10;
    octets[pos_origine] = origine;
    octets
}

fn sceller_brut(cle: &CleSignature, contrat: &[u8]) -> Vec<u8> {
    // Titre construit hors de `ManifesteService` (même disposition).
    struct Brut(Vec<u8>);
    impl miyusceau::Titre for Brut {
        const TYPE: miyusceau::TypeSceau = miyusceau::TypeSceau::ManifestePaquet;
        fn encoder(&self, e: &mut EcrivainTlv) -> Result<(), miyusceau::ErreurSceau> {
            e.texte(1, "jayrdv")?.texte(2, "0.1.0")?.b32(3, &[3; 32])?.octets(5, &self.0)?;
            Ok(())
        }
        fn decoder(_: &mut miyusceau::LecteurTlv<'_>) -> Result<Self, miyusceau::ErreurSceau> {
            Err(miyusceau::ErreurSceau::ValeurInvalide(0))
        }
    }
    miyusceau::sceller(&Brut(contrat.to_vec()), cle).unwrap()
}

#[test]
fn exposition_par_defaut_refusee_ex48() {
    let (cle, ancres) = editeur();
    let ok = sceller_brut(&cle, &contrat_brut(0, 4));
    verifier_manifeste(&ok, &ancres, T0).unwrap();
    let expose = sceller_brut(&cle, &contrat_brut(1, 4));
    assert_eq!(verifier_manifeste(&expose, &ancres, T0).unwrap_err(), ErreurManifeste::ExpositionParDefaut);
}

#[test]
fn entree_origin_ou_international_non_specifiee() {
    let (cle, ancres) = editeur();
    for origine in [5u8, 6] {
        let m = sceller_brut(&cle, &contrat_brut(0, origine));
        assert!(matches!(verifier_manifeste(&m, &ancres, T0), Err(ErreurManifeste::NonSpecifie(_))), "{origine}");
    }
    let m = sceller_brut(&cle, &contrat_brut(0, 9));
    assert!(matches!(verifier_manifeste(&m, &ancres, T0), Err(ErreurManifeste::Contrat(_))));
}

#[test]
fn le_market_transporte_le_contrat_sans_l_interpreter() {
    let (cle, ancres) = editeur();
    let m = manifeste(contrat(ClasseExposition::H2));
    let octets = sceller_manifeste(&m, &cle).unwrap();
    let marche = miyusceau::verifier::<miyumarket::signature::ManifestePaquet>(&octets, &ancres, T0).unwrap();
    assert_eq!(marche.titre().service_id, "jayrdv");
    assert_eq!(marche.titre().sha256_archive, [3; 32]);
    assert_eq!(marche.titre().contrat.as_deref(), Some(&m.contrat.encoder().unwrap()[..]));
    // Paquet Market sans contrat : refusé comme manifeste de service.
    let sans = miyumarket::signature::ManifestePaquet {
        service_id: "jayrdv".into(),
        version: "0.1.0".into(),
        sha256_archive: [3; 32],
        nom_binaire: None,
        contrat: None,
    };
    let octets = miyusceau::sceller(&sans, &cle).unwrap();
    assert!(verifier_manifeste(&octets, &ancres, T0).is_err());
}

#[cfg(feature = "douane")]
#[test]
fn entrees_douane_generees_sans_application() {
    use miyuhybride::douane::{ajouter_sandbox_facade, entrees_douane};
    let (cle, ancres) = editeur();
    let v =
        verifier_manifeste(&sceller_manifeste(&manifeste(contrat(ClasseExposition::H2)), &cle).unwrap(), &ancres, T0)
            .unwrap();
    let e = entrees_douane(&v).unwrap();
    assert_eq!(e.voie.service, "jayrdv");
    assert_eq!(e.flux.len(), 1);
    let f = &e.flux[0];
    assert_eq!((f.source, f.destination, f.port), (borderguard::Zone::Federation, borderguard::Zone::Franche, 443));
    assert_eq!(f.pairs, Vec::<borderguard::Reseau>::new());
    assert_eq!((e.ressources.cpu_pourcent, e.ressources.memoire_mio), (50, 256));
    // Le flux généré entre dans la constitution de référence.
    let arrete = borderguard::Arrete {
        frontiere: borderguard::Frontiere::Externe,
        numero: 1,
        emis_le: T0,
        pas_avant: T0,
        pas_apres: T0 + 3600,
        emetteur: "logisticssteward".into(),
        motif: "installation jayrdv".into(),
        voies: vec![e.voie.clone()],
        flux: e.flux.clone(),
    };
    borderguard::Constitution::reference().controler(&arrete).unwrap();
    // Sandbox Franche : DB secondaire au chemin réservé, plafonds du contrat.
    let mut plan = miyudouane::PlanReseau::reference(&[]).unwrap();
    ajouter_sandbox_facade(&mut plan, &e).unwrap();
    let sb = plan.sandbox(borderguard::Zone::Franche, "jayrdv").unwrap();
    assert!(sb.db_secondaire.is_some());
    assert_eq!(sb.ressources, e.ressources);
    let unites = miyudouane::zones::unites_systemd(&plan);
    assert!(unites.iter().any(|(_, u)| u.contains("MemoryMax=256M")));
}
