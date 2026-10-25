//! Test de sortie du lot 5 « Services hybrides » (PLAN-IMPLEMENTATION §7).
//!
//! @id     jayrdv_tests_sortie_lot5
//! @do     tester_service_h2_depuis_appareil_federe
//! @role   test
//! @layer  service
//! @human  Sortie du lot 5 : un appareil fédéré réserve un créneau JayRDV par la façade (Franche) — plusieurs appareils partagent une façade, anti-rejeu par auteur (A-007) —, le moteur (zone COG) ne reçoit que des messages du traité via le Portail ; exposition désactivée et coupure de voie refusent, la façade est reconstruite.
//!
//! Niveau bibliothèque, en processus, sans réseau : KindMother éphémère réel,
//! Portail interne réel (sas en processus : même cœur que `portail-sas`),
//! manifeste signé réel, titres d'identité réels (passeport d'un COG reconnu,
//! visa du COG hôte, preuve de possession).

#![cfg(all(feature = "kindmother-only", not(feature = "memory")))]
#![allow(clippy::doc_markdown)] // En-têtes MSCM.

use std::sync::Arc;

use jayrdv::{resource_create, slot_create, JayRdvStore, MoteurJayRdv, SlotStatus};
use jayrdv_facade::{
    ClientFedere, ConfigurationFacade, DefiRecu, FacadeJayRdv, ReponseHttp, ReponseJson, RequeteHttp, PORTEE_RESERVER,
};
use jayrdv_traite::{Entrant, SERVICE};
use miyauth::http::{
    formater_autorisation_passeport, formater_pop, liaison_requete, signer_pop, verifier_requete_passeport,
    MagasinDefis, MagasinDefisMemoire,
};
use miyauth::{
    emettre_passeport, emettre_visa, AnnuaireEmetteurs, CogId, DemandePasseport, DemandeVisa, ListeRevocation,
    PolitiqueIdentite, Portees, RevocationsFederees, SecretTitulaire,
};
use miyuhybride::{sceller_manifeste, verifier_manifeste, EtatVoie, ManifesteService, ServiceHybride, Superviseur};
use miyuportail::sas::{decoder_reponse, traiter_requete, ResultatTrame};
use miyuportail::{
    CoupureVoie, DecodeurIsole, ErreurSas, Evenement, IdService, MotifCoupure, PoigneeCoupure, PortailInterne,
    RefusParDefaut, RegistreMemoire, RequeteSas,
};
use miyusceau::{AncresDeConfiance, CleSignature, NiveauConfiance};

const T0: u64 = 1_800_000_000;

/// Sas en processus (même cœur que `portail-sas`, sans confinement).
struct SasEnProcessus;

impl DecodeurIsole for SasEnProcessus {
    fn decoder_lot(&self, requete: &RequeteSas) -> Result<Vec<ResultatTrame>, ErreurSas> {
        let r =
            traiter_requete(&requete.encoder().map_err(|_| ErreurSas::Reponse)?).map_err(|_| ErreurSas::FinAnormale)?;
        decoder_reponse(&r, requete.trames.len())
    }
}

fn cog(c: char) -> CogId {
    CogId::new(&c.to_string().repeat(26)).unwrap()
}

fn store_kindmother() -> JayRdvStore {
    let dir = tempfile::tempdir().unwrap().keep();
    let (addr, creds) = kindmother_service::spawn_ephemeral_blocking(&dir).unwrap();
    let cred = kindmother_client::OperatorCredential::load_from_dir("jayrdv", &creds).unwrap();
    JayRdvStore::connect_with(&addr.to_string(), &cred).unwrap()
}

/// COG hôte (H, celui de JayRDV) et COG reconnu (R, celui de l'appareil).
struct Identites {
    hote: CogId,
    reconnu: CogId,
    cle_hote: CleSignature,
    cle_reconnu: CleSignature,
    annuaire: AnnuaireEmetteurs,
    revocations: RevocationsFederees,
}

impl Identites {
    fn new() -> Self {
        let (hote, reconnu) = (cog('h'), cog('r'));
        let (cle_hote, cle_reconnu) = (CleSignature::generer(), CleSignature::generer());
        let mut annuaire = AnnuaireEmetteurs::new();
        annuaire.ajouter(hote.clone(), &cle_hote.cle_publique(), NiveauConfiance::Souverain).unwrap();
        annuaire.ajouter(reconnu.clone(), &cle_reconnu.cle_publique(), NiveauConfiance::Reconnu).unwrap();
        let mut revocations = RevocationsFederees::new();
        revocations.inserer(ListeRevocation::new(hote.clone(), T0, 3600));
        revocations.inserer(ListeRevocation::new(reconnu.clone(), T0, 3600));
        Self { hote, reconnu, cle_hote, cle_reconnu, annuaire, revocations }
    }

    fn configuration(&self) -> ConfigurationFacade {
        ConfigurationFacade {
            cog_local: self.hote.clone(),
            annuaire: self.annuaire.clone(),
            revocations: self.revocations.clone(),
        }
    }

    /// Appareil fédéré : passeport du COG reconnu, puis visa délivré par la
    /// frontière (BorderGuard) du COG hôte après preuve de possession.
    fn appareil(&self) -> (CleSignature, Vec<u8>, Vec<u8>) {
        let cle = CleSignature::generer();
        let passeport = emettre_passeport(
            &self.reconnu,
            &self.cle_reconnu,
            &DemandePasseport {
                secret: &SecretTitulaire::generer(),
                destination: self.hote.clone(),
                cle_appareil: cle.cle_publique(),
                portees: Portees::new([PORTEE_RESERVER]).unwrap(),
                pas_avant: T0 - 60,
                pas_apres: T0 + 86_400,
            },
        )
        .unwrap();
        // Frontière : demande de visa (passeport + PoP), audience = service.
        let defis = MagasinDefisMemoire::default();
        let d = defis.emettre(SERVICE, T0).unwrap();
        let pop = formater_pop(&d.id, &signer_pop(&cle, &d.defi, SERVICE, &liaison_requete("POST", "/visa", b"")));
        let auth = formater_autorisation_passeport(&passeport);
        let req = miyauth::http::RequeteHttp {
            methode: "POST",
            chemin: "/visa",
            corps: b"",
            autorisation: Some(&auth),
            pop: Some(&pop),
            liaison: None,
        };
        let politique = PolitiqueIdentite {
            cog_local: self.hote.clone(),
            annuaire: &self.annuaire,
            revocations: &self.revocations,
        };
        let possession = verifier_requete_passeport(&req, SERVICE, &politique, &defis, T0).unwrap();
        let visa = emettre_visa(
            &self.hote,
            &self.cle_hote,
            &possession,
            &DemandeVisa {
                audience: SERVICE.into(),
                portees: Portees::new([PORTEE_RESERVER]).unwrap(),
                pas_avant: T0,
                pas_apres: T0 + 900,
            },
        )
        .unwrap();
        (cle, visa, passeport)
    }
}

struct Monde {
    store: JayRdvStore,
    registre: Arc<RegistreMemoire>,
    poignee: PoigneeCoupure,
    service: Arc<ServiceHybride>,
    superviseur: Superviseur<FacadeJayRdv>,
    client: ClientFedere,
    /// Copie de la clé d'appareil (pour forger des requêtes d'un client fautif).
    cle_appareil: CleSignature,
    ids: Identites,
}

fn monde() -> Monde {
    // Zone COG : KindMother et données du moteur.
    let store = store_kindmother();
    let r = resource_create(&store, "Cabinet".into(), Some("salle".into())).unwrap();
    for h in [9, 10, 11] {
        slot_create(
            &store,
            r.id.clone(),
            None,
            format!("2026-11-02T{h:02}:00:00+00:00"),
            format!("2026-11-02T{h:02}:30:00+00:00"),
        )
        .unwrap();
    }

    // Installation : manifeste signé par l'éditeur, vérifié, voie déclarée (désactivée).
    let editeur = CleSignature::generer();
    let mut editeurs = AncresDeConfiance::new();
    editeurs.ajouter(&editeur.cle_publique(), NiveauConfiance::Souverain).unwrap();
    let manifeste = ManifesteService {
        service: IdService::new(SERVICE).unwrap(),
        version: "0.1.0".into(),
        sha256_archive: [5; 32],
        nom_binaire: Some("jayrdv".into()),
        contrat: jayrdv_traite::contrat_reference(jayrdv_facade::empreinte_image()),
    };
    let verifie = verifier_manifeste(&sceller_manifeste(&manifeste, &editeur).unwrap(), &editeurs, T0).unwrap();
    let registre = Arc::new(RegistreMemoire::new());
    let portail = Arc::new(PortailInterne::new(Box::new(SasEnProcessus), Box::new(RefusParDefaut), registre.clone()));
    let poignee = portail.poignee_coupure();
    let service = Arc::new(ServiceHybride::installer(portail, verifie).unwrap());

    // Identités : l'appareil fédéré obtient son visa à la frontière ; les
    // Cores épinglent sa clé d'appareil comme auteur de la session.
    let ids = Identites::new();
    let (cle, visa, passeport) = ids.appareil();
    let auteur = cle.cle_publique();
    let cle_appareil = CleSignature::depuis_graine(&cle.graine());
    let config = ids.configuration();
    let superviseur =
        Superviseur::new(Arc::clone(&service), jayrdv_facade::empreinte_image(), vec![auteur], move |l| {
            FacadeJayRdv::new(l, config.clone())
        });
    Monde {
        store,
        registre,
        poignee,
        service,
        superviseur,
        client: ClientFedere::new(cle, visa, passeport),
        cle_appareil,
        ids,
    }
}

impl Monde {
    fn facade(&mut self) -> &mut FacadeJayRdv {
        self.superviseur.facade().expect("façade lancée")
    }

    fn envoyer(&mut self, r: &RequeteHttp) -> ReponseHttp {
        self.facade().traiter(r, T0 + 10)
    }

    fn defi(&mut self) -> DefiRecu {
        let r = self.envoyer(&ClientFedere::requete_defi());
        assert_eq!(r.statut, 200);
        DefiRecu::depuis(&r.corps).unwrap()
    }

    /// Recalcule la preuve de possession d'une requête modifiée (client fautif
    /// qui détient sa clé d'appareil).
    fn reprouver(&mut self, req: &mut RequeteHttp) {
        let cle = CleSignature::depuis_graine(&self.cle_appareil.graine());
        self.reprouver_par(&cle, req);
    }

    /// Idem, pour la clé d'appareil donnée.
    fn reprouver_par(&mut self, cle: &CleSignature, req: &mut RequeteHttp) {
        let r = self.envoyer(&ClientFedere::requete_defi());
        let v: serde_json::Value = serde_json::from_slice(&r.corps).unwrap();
        let id: [u8; 16] = miyauth::http::b64_decoder(v["id"].as_str().unwrap()).unwrap().try_into().unwrap();
        let defi: [u8; 32] = miyauth::http::b64_decoder(v["defi"].as_str().unwrap()).unwrap().try_into().unwrap();
        let liaison = liaison_requete(&req.methode, &req.chemin, &req.corps);
        let pop = formater_pop(&id, &signer_pop(cle, &defi, SERVICE, &liaison));
        req.entetes.retain(|(n, _)| n != "x-miyu-pop");
        req.entetes.push(("x-miyu-pop".into(), pop));
    }

    fn moteur(&self) -> MoteurJayRdv<'_> {
        MoteurJayRdv::new(&self.store, self.service.guichet_moteur())
    }

    fn demande(r: &ReponseHttp) -> String {
        assert_eq!(r.statut, 202, "{}", String::from_utf8_lossy(&r.corps));
        let v: serde_json::Value = serde_json::from_slice(&r.corps).unwrap();
        v["demande"].as_str().unwrap().to_string()
    }

    fn lire(&mut self, demande: &str) -> ReponseJson {
        let d = self.defi();
        let req = self.client.lire_reponse(&d, demande);
        self.lire_requete(&req)
    }

    /// Lecture d'une réponse par un autre appareil de la même façade.
    fn lire_par(&mut self, client: &ClientFedere, demande: &str) -> ReponseJson {
        let d = self.defi();
        self.lire_requete(&client.lire_reponse(&d, demande))
    }

    fn lire_requete(&mut self, req: &RequeteHttp) -> ReponseJson {
        let r = self.envoyer(req);
        assert_eq!(r.statut, 200, "{}", String::from_utf8_lossy(&r.corps));
        serde_json::from_slice(&r.corps).unwrap()
    }

    fn creneaux(&mut self) -> Vec<String> {
        let d = self.defi();
        let req =
            self.client.demander_creneaux(&d, "2026-11-02T00:00:00+00:00", "2026-11-03T00:00:00+00:00", T0).unwrap();
        let demande = Self::demande(&self.envoyer(&req));
        let rapport = self.moteur().traiter_lot(64).unwrap();
        assert!(matches!(rapport.recus.as_slice(), [Entrant::DemandeCreneaux(_)]));
        match self.lire(&demande) {
            ReponseJson::Creneaux { creneaux } => creneaux.into_iter().map(|c| c.creneau).collect(),
            autre => panic!("{autre:?}"),
        }
    }

    fn reserver(&mut self, creneau: &str) -> ReponseJson {
        let d = self.defi();
        let req = self.client.reserver(&d, creneau, "Ada Lovelace", Some("ada@exemple.org"), T0).unwrap();
        let demande = Self::demande(&self.envoyer(&req));
        let rapport = self.moteur().traiter_lot(64).unwrap();
        assert_eq!(rapport.refusees_portail + rapport.hors_traite, 0);
        assert!(matches!(rapport.recus.as_slice(), [Entrant::Reservation(r)] if r.creneau == creneau));
        self.lire(&demande)
    }
}

#[test]
fn sortie_reservation_depuis_un_appareil_federe() {
    let mut m = monde();

    // Exposition désactivée par défaut (EX-48) : aucune façade possible.
    assert_eq!(m.service.etat().unwrap(), EtatVoie::Desactivee);
    assert!(!m.superviseur.surveiller().unwrap());
    assert!(m.superviseur.facade().is_none());

    // Action explicite de l'utilisateur : exposition activée, façade neuve.
    m.service.activer_exposition().unwrap();
    assert!(m.superviseur.surveiller().unwrap());

    // 1. Créneaux libres.
    let creneaux = m.creneaux();
    assert_eq!(creneaux.len(), 3);

    // 2. Réservation : décidée en zone COG, inscrite dans KindMother.
    let ReponseJson::Confirmee { rendez_vous, creneau } = m.reserver(&creneaux[0]) else { panic!("refusée") };
    assert_eq!(creneau.creneau, creneaux[0]);
    let slot = m.store.slot_by_id(&creneaux[0]).unwrap().unwrap();
    assert_eq!(slot.status, SlotStatus::Booked);
    let rdv = m.store.appointment_by_id(&rendez_vous).unwrap().unwrap();
    assert_eq!(rdv.client_name.as_deref(), Some("Ada Lovelace"));
    assert_eq!(rdv.slot_id.as_deref(), Some(creneaux[0].as_str()));

    // 3. Le cache de la façade (DB secondaire) ne retient que le publié, sans le créneau pris.
    let d = m.defi();
    let req = m.client.lire_cache(&d);
    let ReponseJson::Creneaux { creneaux: cache } = serde_json::from_slice(&m.envoyer(&req).corps).unwrap() else {
        panic!("cache")
    };
    assert_eq!(cache.len(), 2);
    assert!(cache.iter().all(|c| c.creneau != creneaux[0]));

    // 4. Le cache n'est pas autoritaire : seconde réservation du même créneau refusée en zone COG.
    assert_eq!(m.reserver(&creneaux[0]), ReponseJson::Refusee { motif: "indisponible".into() });

    // 5. Tout est passé par le Portail : 3 entrées admises, inscrites au registre.
    let evenements = m.registre.evenements();
    let entrees = evenements.iter().filter(|e| matches!(e, Evenement::Entree { .. })).count();
    let publications = evenements.iter().filter(|e| matches!(e, Evenement::Publication { .. })).count();
    assert_eq!(entrees, 3);
    assert_eq!(publications, 4 + 1 + 1, "3 créneaux + fin, confirmation, refus");
}

#[test]
fn appareil_non_reconnu_auteur_hors_session_ou_signature_falsifiee_refuses() {
    let mut m = monde();
    m.service.activer_exposition().unwrap();
    m.superviseur.surveiller().unwrap();

    // Corps modifié en route : la preuve de possession (liée au corps) tombe.
    let d = m.defi();
    let mut req = m.client.reserver(&d, "x", "Ada", None, T0).unwrap();
    req.corps = String::from_utf8(req.corps).unwrap().replace("\"nom\":\"Ada\"", "\"nom\":\"Eve\"").into_bytes();
    assert_eq!(m.envoyer(&req).statut, 401);

    // Client fautif qui reprouve sa possession sur un corps différent de ce
    // qu'il a signé comme auteur : signature d'auteur refusée, rien de déposé.
    let d = m.defi();
    let mut req = m.client.reserver(&d, "x", "Ada", None, T0).unwrap();
    req.corps = String::from_utf8(req.corps).unwrap().replace("\"nom\":\"Ada\"", "\"nom\":\"Eve\"").into_bytes();
    m.reprouver(&mut req);
    let r = m.envoyer(&req);
    assert_eq!((r.statut, String::from_utf8_lossy(&r.corps).contains("signature")), (403, true));

    // Champ hors API (aucun octet hors traité ne traverse) : refusé.
    let d = m.defi();
    let mut req = m.client.reserver(&d, "x", "Ada", None, T0).unwrap();
    let mut v: serde_json::Value = serde_json::from_slice(&req.corps).unwrap();
    v["script"] = serde_json::Value::String("<x>".into());
    req.corps = serde_json::to_vec(&v).unwrap();
    m.reprouver(&mut req);
    assert_eq!(m.envoyer(&req).statut, 400);

    // Appareil fédéré valide, mais non épinglé comme auteur de la session.
    let (cle, visa, passeport) = m.ids.appareil();
    let mut autre = ClientFedere::new(cle, visa, passeport);
    let d = m.defi();
    let req = autre.reserver(&d, "x", "Bob", None, T0).unwrap();
    assert_eq!(m.envoyer(&req).statut, 403);

    // Passeport d'un COG non reconnu : visa impossible à vérifier.
    let inconnu = CleSignature::generer();
    let mut faux = ClientFedere::new(inconnu, b"visa".to_vec(), b"passeport".to_vec());
    let d = m.defi();
    let req = faux.reserver(&d, "x", "Mallory", None, T0).unwrap();
    assert_eq!(m.envoyer(&req).statut, 401);

    // Aucune trame n'a atteint la quarantaine.
    let rapport = m.moteur().traiter_lot(64).unwrap();
    assert!(rapport.recus.is_empty() && rapport.refusees_portail == 0);
    assert_eq!(m.registre.evenements().len(), 0);
}

#[test]
fn exposition_desactivee_la_facade_refuse_et_n_est_pas_relancee() {
    let mut m = monde();
    m.service.activer_exposition().unwrap();
    m.superviseur.surveiller().unwrap();
    m.service.desactiver_exposition().unwrap();

    let req = ClientFedere::requete_defi();
    assert_eq!(m.envoyer(&req).statut, 503);
    assert!(m.facade().db().est_vide(), "DB secondaire purgée");
    assert!(!m.superviseur.surveiller().unwrap(), "pas de relance tant que l'exposition est désactivée");
    assert!(m.superviseur.facade().is_none());
}

#[test]
fn coupure_de_voie_la_facade_refuse_puis_est_reconstruite() {
    let mut m = monde();
    m.service.activer_exposition().unwrap();
    m.superviseur.surveiller().unwrap();
    let cle_avant = m.facade().cle_session();
    let creneaux = m.creneaux();

    // Une demande en vol, puis WorrySentinel coupe la voie (EX-49).
    let d = m.defi();
    let req = m.client.reserver(&d, &creneaux[1], "Ada", None, T0).unwrap();
    let _en_vol = Monde::demande(&m.envoyer(&req));
    m.poignee.couper(&IdService::new(SERVICE).unwrap(), MotifCoupure::Anomalie).unwrap();

    // La façade refuse tout, purge sa DB, se déclare condamnée.
    assert_eq!(m.envoyer(&ClientFedere::requete_defi()).statut, 503);
    assert!(m.facade().db().est_vide());
    // Le moteur ne relève rien sur une voie coupée.
    assert!(m.moteur().traiter_lot(64).is_err());
    // Détruite, non reconstruite tant que la voie est coupée.
    assert!(!m.superviseur.surveiller().unwrap());
    assert!(m.superviseur.facade().is_none());

    // Rétablissement et activation explicites : façade neuve, nouvelle clé, DB vierge.
    m.service.retablir().unwrap();
    assert!(!m.superviseur.surveiller().unwrap());
    m.service.activer_exposition().unwrap();
    assert!(m.superviseur.surveiller().unwrap());
    assert_ne!(m.facade().cle_session(), cle_avant);
    assert!(m.facade().db().est_vide());

    // La demande en vol a été purgée ; l'appareil réserve de nouveau, de bout en bout.
    assert_eq!(m.store.slot_by_id(&creneaux[1]).unwrap().unwrap().status, SlotStatus::Available);
    assert!(matches!(m.reserver(&creneaux[1]), ReponseJson::Confirmee { .. }));
}

/// Deux appareils fédérés, une seule instance de façade (A-007) : séquences
/// anti-rejeu indépendantes, épinglage et retrait sans reconstruction, coupure
/// qui condamne la façade pour tous.
#[test]
fn deux_appareils_federes_une_meme_facade() {
    let mut m = monde();
    m.service.activer_exposition().unwrap();
    assert!(m.superviseur.surveiller().unwrap());
    let cle_session = m.facade().cle_session();

    // Appareil B (même COG reconnu, visa du COG hôte) : non épinglé, refusé.
    let (cle_b, visa_b, passeport_b) = m.ids.appareil();
    let pk_b = cle_b.cle_publique();
    let copie_b = CleSignature::depuis_graine(&cle_b.graine());
    let mut b = ClientFedere::new(cle_b, visa_b, passeport_b);
    let d = m.defi();
    let req = b.demander_creneaux(&d, "2026-11-02T00:00:00+00:00", "2026-11-03T00:00:00+00:00", T0).unwrap();
    assert_eq!(m.envoyer(&req).statut, 403);

    // Les Cores épinglent B dans la session ouverte : même façade, même clé.
    m.superviseur.ajouter_auteur(pk_b).unwrap();
    assert_eq!(m.superviseur.lancements(), 1);
    assert_eq!(m.facade().cle_session(), cle_session);

    // A avance seul (séquences 1 à 3).
    let creneaux = m.creneaux();
    assert_eq!(creneaux.len(), 3);
    assert_eq!(m.creneaux(), creneaux);
    let d = m.defi();
    let rejouable_ada = m.client.demander_creneaux(&d, "2026-11-02T00:00:00+00:00", "2026-11-03T00:00:00+00:00", T0).unwrap();
    let creneaux_ada = Monde::demande(&m.envoyer(&rejouable_ada));

    // B dépose ses séquences 1 et 2, entrelacées avec A : toutes admises.
    let d = m.defi();
    let rejouable_bob = b.demander_creneaux(&d, "2026-11-02T00:00:00+00:00", "2026-11-03T00:00:00+00:00", T0).unwrap();
    let creneaux_bob = Monde::demande(&m.envoyer(&rejouable_bob));
    let d = m.defi();
    let req = m.client.reserver(&d, &creneaux[0], "Ada", None, T0).unwrap();
    let resa_ada = Monde::demande(&m.envoyer(&req));
    let d = m.defi();
    let req = b.reserver(&d, &creneaux[1], "Bob", None, T0).unwrap();
    let resa_bob = Monde::demande(&m.envoyer(&req));
    let rapport = m.moteur().traiter_lot(64).unwrap();
    assert_eq!((rapport.recus.len(), rapport.refusees_portail), (4, 0));
    assert!(matches!(m.lire(&creneaux_ada), ReponseJson::Creneaux { .. }));
    assert!(matches!(m.lire_par(&b, &creneaux_bob), ReponseJson::Creneaux { .. }));
    assert!(matches!(m.lire(&resa_ada), ReponseJson::Confirmee { .. }));
    assert!(matches!(m.lire_par(&b, &resa_bob), ReponseJson::Confirmee { .. }));
    // Chacun ne lit que ses réponses.
    let d = m.defi();
    assert_eq!(m.envoyer(&b.lire_reponse(&d, &creneaux_ada)).statut, 404);

    // Rejeu de la trame A n° 3 (nouvelle preuve de possession), après l'avance de B : refusé.
    let mut rejeu = rejouable_ada.clone();
    m.reprouver(&mut rejeu);
    assert_eq!(m.envoyer(&rejeu).statut, 409);
    // Rejeu de B n° 1 : refusé aussi, chacun sa séquence.
    let mut rejeu_bob = rejouable_bob.clone();
    m.reprouver_par(&copie_b, &mut rejeu_bob);
    assert_eq!(m.envoyer(&rejeu_bob).statut, 409);

    // A dépose une réservation, puis les Cores le retirent avant le relevé :
    // sa trame en quarantaine est refusée par le Portail ; B continue.
    let d = m.defi();
    let req = m.client.reserver(&d, &creneaux[2], "Ada", None, T0).unwrap();
    let _en_vol = Monde::demande(&m.envoyer(&req));
    let pk_a = m.cle_appareil.cle_publique();
    m.superviseur.retirer_auteur(&pk_a).unwrap();
    let d = m.defi();
    let req = b.reserver(&d, &creneaux[2], "Bob", None, T0).unwrap();
    let derniere_resa_bob = Monde::demande(&m.envoyer(&req));
    let rapport = m.moteur().traiter_lot(64).unwrap();
    assert_eq!(rapport.refusees_portail, 1, "trame de A refusée après son retrait");
    assert!(matches!(rapport.recus.as_slice(), [Entrant::Reservation(r)] if r.creneau == creneaux[2]));
    assert!(matches!(m.lire_par(&b, &derniere_resa_bob), ReponseJson::Confirmee { .. }));
    // A retiré : toute nouvelle requête est refusée à la façade.
    let d = m.defi();
    let req = m.client.demander_creneaux(&d, "2026-11-02T00:00:00+00:00", "2026-11-03T00:00:00+00:00", T0).unwrap();
    let r = m.envoyer(&req);
    assert_eq!((r.statut, String::from_utf8_lossy(&r.corps).contains("auteur_hors_session")), (403, true));
    assert_eq!(m.superviseur.lancements(), 1, "aucune reconstruction pour épingler ou retirer");

    // A réépinglé : il reprend au-delà de sa marque haute ; son ancien rejeu reste refusé.
    m.superviseur.ajouter_auteur(pk_a).unwrap();
    let mut rejeu = rejouable_ada;
    m.reprouver(&mut rejeu);
    assert_eq!(m.envoyer(&rejeu).statut, 409);
    let d = m.defi();
    let req = m.client.demander_creneaux(&d, "2026-11-02T00:00:00+00:00", "2026-11-03T00:00:00+00:00", T0).unwrap();
    let demande_a = Monde::demande(&m.envoyer(&req));
    m.moteur().traiter_lot(64).unwrap();
    assert!(matches!(m.lire(&demande_a), ReponseJson::Creneaux { .. }));

    // Coupure de la voie : la façade est condamnée, les deux appareils refusés.
    let (d_a, d_b) = (m.defi(), m.defi());
    m.poignee.couper(&IdService::new(SERVICE).unwrap(), MotifCoupure::Anomalie).unwrap();
    let req = m.client.reserver(&d_a, &creneaux[0], "Ada", None, T0).unwrap();
    assert_eq!(m.envoyer(&req).statut, 503);
    let req = b.reserver(&d_b, &creneaux[0], "Bob", None, T0).unwrap();
    assert_eq!(m.envoyer(&req).statut, 503);
    assert!(m.facade().db().est_vide());
    assert!(!m.superviseur.surveiller().unwrap());
    assert!(m.superviseur.facade().is_none());
}
