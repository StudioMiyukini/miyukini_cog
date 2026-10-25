//! @id     miyuhybride_tests_cycle
//! @do     tester_moteur_facade_traite
//! @role   test
//! @layer  toolkit
//! @human  miyuhybride — patron moteur / façade / traité : exposition désactivée, clé de session éphémère, auteur obligatoire, réponse à jeton unique, reconstruction après coupure (EX-41..43, EX-48, INV-H5, INV-H6).

use std::sync::Arc;

use miyuhybride::*;
use miyuportail::sas::{decoder_reponse, traiter_requete, ResultatTrame};
use miyuportail::{
    CoupureVoie, DecodeurIsole, ErreurSas, MotifCoupure, PortailInterne, RefusParDefaut, RegistreMemoire, RequeteSas,
};
use miyusceau::{AncresDeConfiance, CleSignature, NiveauConfiance};

const T0: u64 = 1_800_000_000;
const IMAGE: [u8; 32] = [9; 32];

/// Sas en processus (même cœur que `portail-sas`, sans confinement).
struct SasEnProcessus;

impl DecodeurIsole for SasEnProcessus {
    fn decoder_lot(&self, requete: &RequeteSas) -> Result<Vec<ResultatTrame>, ErreurSas> {
        let r =
            traiter_requete(&requete.encoder().map_err(|_| ErreurSas::Reponse)?).map_err(|_| ErreurSas::FinAnormale)?;
        decoder_reponse(&r, requete.trames.len())
    }
}

message_traite! {
    /// Question entrante.
    pub struct Question (Entrante, 1) {
        1 => pub texte: String, max 64;
    }
}

message_traite! {
    /// Réponse sortante.
    pub struct Reponse (Sortante, 2) {
        1 => pub demande: Vec<u8>, max 32;
        2 => pub longueur: u64, max 0;
    }
}

fn manifeste() -> ManifesteVerifie {
    let cle = CleSignature::generer();
    let mut ancres = AncresDeConfiance::new();
    ancres.ajouter(&cle.cle_publique(), NiveauConfiance::Souverain).unwrap();
    let m = ManifesteService {
        service: IdService::new("essai").unwrap(),
        version: "1".into(),
        sha256_archive: [1; 32],
        nom_binaire: None,
        contrat: ContratService {
            classe: ClasseExposition::H2,
            ressources: Ressources { cpu_pourcent: 10, memoire_mio: 64, disque_mio: 8, taches_max: 8, sessions_max: 1 },
            voie: Some(ContratVoie { taille_max_trame: 4096, debit_max_par_minute: 100, quota_file: 16 }),
            traite: Some(traite(1, vec![Question::type_declaration(), Reponse::type_declaration()])),
            facade: Some(ContratFacade { image: IMAGE, db_secondaire_max_mio: 1, duree_max_session_secs: 60 }),
            entrees: vec![PointEntree {
                origine: OrigineExterne::Federation,
                transport: Transport::Tcp,
                port: 443,
                debit_max_kio_s: 64,
                taille_max_paquet: 1500,
            }],
        },
    };
    verifier_manifeste(&sceller_manifeste(&m, &cle).unwrap(), &ancres, T0).unwrap()
}

struct Monde {
    service: Arc<ServiceHybride>,
    poignee: miyuportail::PoigneeCoupure,
    auteur: CleSignature,
}

fn monde() -> Monde {
    let portail = Arc::new(PortailInterne::new(
        Box::new(SasEnProcessus),
        Box::new(RefusParDefaut),
        Arc::new(RegistreMemoire::new()),
    ));
    let poignee = portail.poignee_coupure();
    let service = Arc::new(ServiceHybride::installer(portail, manifeste()).unwrap());
    Monde { service, poignee, auteur: CleSignature::generer() }
}

fn question(m: &Monde, g: &GuichetFranche, seq: u64, texte: &str) -> Result<EmpreinteDemande, ErreurHybride> {
    let d = declaration_entrante(g.service(), g.version_traite(), &Question { texte: texte.into() }, seq, T0).unwrap();
    let sig = signer_auteur(&m.auteur, &d).unwrap();
    g.deposer(&d, &sig)
}

#[test]
fn exposition_desactivee_par_defaut_puis_activee() {
    let m = monde();
    assert_eq!(m.service.etat().unwrap(), EtatVoie::Desactivee);
    let refus = m.service.ouvrir_session(&IMAGE, vec![m.auteur.cle_publique()]).unwrap_err();
    assert_eq!(refus, ErreurHybride::Portail(ErreurPortail::VoieDesactivee));
    m.service.activer_exposition().unwrap();
    m.service.ouvrir_session(&IMAGE, vec![m.auteur.cle_publique()]).unwrap();
}

#[test]
fn image_de_facade_non_conforme_refusee() {
    let m = monde();
    m.service.activer_exposition().unwrap();
    assert_eq!(
        m.service.ouvrir_session(&[0; 32], vec![m.auteur.cle_publique()]).unwrap_err(),
        ErreurHybride::ImageNonConforme
    );
}

#[test]
fn aller_retour_moteur_facade_par_le_traite() {
    let m = monde();
    m.service.activer_exposition().unwrap();
    let l = m.service.ouvrir_session(&IMAGE, vec![m.auteur.cle_publique()]).unwrap();
    let g = l.guichet;
    let demande = question(&m, &g, 1, "bonjour").unwrap();

    let moteur = m.service.guichet_moteur();
    let mut lot = moteur.relever(8).unwrap();
    assert_eq!(lot.len(), 1);
    let admise = lot.pop().unwrap().unwrap();
    assert_eq!(admise.message::<Question>().unwrap().texte, "bonjour");
    assert_eq!(admise.jeton.empreinte(), &demande);
    // Signataires : clé de session éphémère + auteur.
    assert_eq!(admise.declaration.signataires.len(), 2);
    let r = Reponse { demande: admise.jeton.empreinte().to_vec(), longueur: 7 };
    moteur.repondre(admise.jeton, &[MessageSortant::de(&r).unwrap()]).unwrap();

    let publie = g.lire_publication().unwrap().unwrap();
    assert_eq!(Reponse::depuis_declaration(&publie).unwrap(), r);
    assert_eq!(g.lire_publication().unwrap(), None);
    // Message sortant dans le mauvais sens : refusé à la construction.
    assert_eq!(MessageSortant::de(&Question { texte: "x".into() }), Err(ErreurTraite::Sens));
}

#[test]
fn auteur_obligatoire_et_non_falsifiable() {
    let m = monde();
    m.service.activer_exposition().unwrap();
    let g = m.service.ouvrir_session(&IMAGE, vec![m.auteur.cle_publique()]).unwrap().guichet;
    let d = declaration_entrante(g.service(), 1, &Question { texte: "a".into() }, 1, T0).unwrap();
    // Auteur non épinglé.
    let intrus = CleSignature::generer();
    assert!(g.deposer(&d, &signer_auteur(&intrus, &d).unwrap()).is_err());
    // Signature de l'auteur sur une autre déclaration : refusée avant tout dépôt.
    let autre = declaration_entrante(g.service(), 1, &Question { texte: "b".into() }, 1, T0).unwrap();
    let (cle, sig) = signer_auteur(&m.auteur, &autre).unwrap();
    assert!(matches!(g.deposer(&d, &(cle, sig)), Err(ErreurHybride::Decodage(_))));
    assert_eq!(g.compteurs().unwrap().deposees, 0);
}

#[test]
fn jeton_d_une_session_close_refuse() {
    let m = monde();
    m.service.activer_exposition().unwrap();
    let g = m.service.ouvrir_session(&IMAGE, vec![m.auteur.cle_publique()]).unwrap().guichet;
    question(&m, &g, 1, "a").unwrap();
    let moteur = m.service.guichet_moteur();
    let admise = moteur.relever(1).unwrap().pop().unwrap().unwrap();
    // Nouvelle session (façade neuve) avant la réponse.
    let _g2 = m.service.ouvrir_session(&IMAGE, vec![m.auteur.cle_publique()]).unwrap();
    let r = MessageSortant::de(&Reponse { demande: vec![0; 32], longueur: 0 }).unwrap();
    assert_eq!(moteur.repondre(admise.jeton, &[r]), Err(ErreurHybride::SessionPerimee));
}

#[test]
fn ancienne_facade_sans_effet_apres_nouvelle_session() {
    let m = monde();
    m.service.activer_exposition().unwrap();
    let g1 = m.service.ouvrir_session(&IMAGE, vec![m.auteur.cle_publique()]).unwrap().guichet;
    let g2 = m.service.ouvrir_session(&IMAGE, vec![m.auteur.cle_publique()]).unwrap().guichet;
    assert_ne!(g1.cle_session(), g2.cle_session(), "clé de session éphémère (INV-H6)");
    // L'ancienne façade dépose encore, mais sa clé n'est plus épinglée : refus en zone COG.
    question(&m, &g1, 1, "vieux").unwrap();
    let lot = m.service.guichet_moteur().relever(8).unwrap();
    assert!(lot[0].is_err());
}

struct FacadeEssai {
    guichet: GuichetFranche,
    condamnee: bool,
}

impl FacadeJetable for FacadeEssai {
    fn condamnee(&self) -> bool {
        self.condamnee
    }
}

impl FacadeEssai {
    fn sonder(&mut self) {
        if self.guichet.etat() != Ok(EtatVoie::Active) {
            self.condamnee = true;
        }
    }
}

#[test]
fn coupure_condamne_puis_reconstruction_explicite() {
    let m = monde();
    m.service.activer_exposition().unwrap();
    let mut sup =
        Superviseur::new(Arc::clone(&m.service), IMAGE, vec![m.auteur.cle_publique()], |l: LancementFacade| {
            assert!(l.db.est_vide(), "DB secondaire vierge");
            FacadeEssai { guichet: l.guichet, condamnee: false }
        });
    assert!(sup.surveiller().unwrap());
    let cle1 = sup.facade().unwrap().guichet.cle_session();

    m.poignee.couper(m.service.service(), MotifCoupure::Anomalie).unwrap();
    sup.facade().unwrap().sonder();
    // Voie coupée : façade détruite, aucune reconstruction.
    assert!(!sup.surveiller().unwrap());
    assert!(sup.facade().is_none());
    // Rétablissement explicite : la voie repasse désactivée, toujours rien.
    m.service.retablir().unwrap();
    assert!(!sup.surveiller().unwrap());
    // Activation explicite : façade neuve, nouvelle clé.
    m.service.activer_exposition().unwrap();
    assert!(sup.surveiller().unwrap());
    assert_eq!(sup.lancements(), 2);
    assert_ne!(sup.facade().unwrap().guichet.cle_session(), cle1);
}
