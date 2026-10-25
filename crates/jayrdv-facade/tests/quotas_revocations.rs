//! @id     jayrdv_facade_tests_quotas_revocations
//! @do     tester_quotas_et_revocations_facade
//! @role   test
//! @layer  service
//! @human  jayrdv-facade — quotas du contrat (sessions, tâches, disque) appliqués par la façade, sessions libérées (fin, inactivité, destruction) ; révocations rafraîchies depuis le registre vérifié : titre révoqué refusé, liste périmée refusée.

#![allow(clippy::doc_markdown, clippy::many_single_char_names)]

use std::sync::Arc;

use jayrdv_facade::{ClientFedere, ConfigurationFacade, DefiRecu, FacadeJayRdv, ReponseHttp, RequeteHttp, PORTEE_RESERVER};
use jayrdv_traite::SERVICE;
use miyauth::http::{
    formater_autorisation_passeport, formater_pop, liaison_requete, signer_pop, verifier_requete_passeport,
    MagasinDefis, MagasinDefisMemoire,
};
use miyauth::{
    emettre_passeport, emettre_visa, AnnuaireEmetteurs, CogId, DemandePasseport, DemandeVisa, ListeRevocation,
    PolitiqueIdentite, Portees, RevocationsFederees, SecretTitulaire,
};
use miyuhybride::{
    sceller_manifeste, verifier_manifeste, LancementFacade, ManifesteService, RefusRemise, RevocationsFacade,
    ServiceHybride,
};
use miyuportail::sas::{decoder_reponse, traiter_requete, ResultatTrame};
use miyuportail::{DecodeurIsole, ErreurSas, IdService, PortailInterne, RefusParDefaut, RegistreMemoire, RequeteSas};
use miyuhybride::FacadeJetable;
use miyuregistre::{Cible, Corps, EntreeRegistre, Journal, StockageMemoire, SuiviRevocationsRegistre};
use miyusceau::{empreinte_cle, sceller, AncresDeConfiance, CleSignature, NiveauConfiance};

const T0: u64 = 1_800_000_000;

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

/// Registre d'un COG (journal) et son suivi vérifié tenu par la zone COG.
struct Registre {
    cle: CleSignature,
    journal: Journal<StockageMemoire>,
    suivi: SuiviRevocationsRegistre,
}

impl Registre {
    fn new(cog: CogId, id: [u8; 32]) -> Self {
        let cle = CleSignature::generer();
        Self {
            journal: Journal::ouvrir(id, StockageMemoire::new(), StockageMemoire::new()).unwrap(),
            suivi: SuiviRevocationsRegistre::new(cog, id, &cle.cle_publique()).unwrap(),
            cle,
        }
    }

    /// Inscrit des révocations, émet une tête à `t`, l'applique au suivi.
    fn publier_tete(&mut self, cibles: &[Cible], t: u64) {
        let m = self.suivi.taille().unwrap();
        for c in cibles {
            let e = EntreeRegistre { inscrite_a: t, corps: Corps::Revocation { cible: *c, motif: "perte".into() } };
            self.journal.ajouter(&sceller(&e, &self.cle).unwrap()).unwrap();
        }
        let tete = self.journal.emettre_tete(&[&self.cle], t, 86_400).unwrap();
        let nouvelles = self.journal.entrees(m, u64::MAX).unwrap();
        self.suivi.appliquer(&tete, m, &nouvelles, t).unwrap();
    }

    /// Zone COG : remet l'instantané du suivi à la façade.
    fn remettre(&mut self, revocations: &RevocationsFacade, _t: u64) {
        revocations.rafraichir(&self.suivi).unwrap();
    }
}

struct Monde {
    hote: CogId,
    reconnu: CogId,
    cle_hote: CleSignature,
    cle_reconnu: CleSignature,
    annuaire: AnnuaireEmetteurs,
    service: Arc<ServiceHybride>,
}

fn monde(ajuster: impl FnOnce(&mut miyuhybride::ContratService)) -> Monde {
    let (hote, reconnu) = (cog('h'), cog('r'));
    let (cle_hote, cle_reconnu) = (CleSignature::generer(), CleSignature::generer());
    let mut annuaire = AnnuaireEmetteurs::new();
    annuaire.ajouter(hote.clone(), &cle_hote.cle_publique(), NiveauConfiance::Souverain).unwrap();
    annuaire.ajouter(reconnu.clone(), &cle_reconnu.cle_publique(), NiveauConfiance::Reconnu).unwrap();

    let editeur = CleSignature::generer();
    let mut editeurs = AncresDeConfiance::new();
    editeurs.ajouter(&editeur.cle_publique(), NiveauConfiance::Souverain).unwrap();
    let mut contrat = jayrdv_traite::contrat_reference(jayrdv_facade::empreinte_image());
    ajuster(&mut contrat);
    let manifeste = ManifesteService {
        service: IdService::new(SERVICE).unwrap(),
        version: "0.1.0".into(),
        sha256_archive: [5; 32],
        nom_binaire: Some("jayrdv".into()),
        contrat,
    };
    let verifie = verifier_manifeste(&sceller_manifeste(&manifeste, &editeur).unwrap(), &editeurs, T0).unwrap();
    let portail = Arc::new(PortailInterne::new(
        Box::new(SasEnProcessus),
        Box::new(RefusParDefaut),
        Arc::new(RegistreMemoire::new()),
    ));
    let service = Arc::new(ServiceHybride::installer(portail, verifie).unwrap());
    service.activer_exposition().unwrap();
    Monde { hote, reconnu, cle_hote, cle_reconnu, annuaire, service }
}

impl Monde {
    /// Appareil fédéré : passeport du COG reconnu, visa du COG hôte valable sur `[debut, debut + 900]`.
    fn appareil(&self, debut: u64) -> (CleSignature, ClientFedere) {
        let cle = CleSignature::generer();
        let passeport = emettre_passeport(
            &self.reconnu,
            &self.cle_reconnu,
            &DemandePasseport {
                secret: &SecretTitulaire::generer(),
                destination: self.hote.clone(),
                cle_appareil: cle.cle_publique(),
                portees: Portees::new([PORTEE_RESERVER]).unwrap(),
                pas_avant: T0 - 3_600,
                pas_apres: T0 + 86_400,
            },
        )
        .unwrap();
        let mut revocations = RevocationsFederees::new();
        revocations.inserer(ListeRevocation::new(self.hote.clone(), debut, 900));
        revocations.inserer(ListeRevocation::new(self.reconnu.clone(), debut, 900));
        let defis = MagasinDefisMemoire::default();
        let d = defis.emettre(SERVICE, debut).unwrap();
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
        let politique =
            PolitiqueIdentite { cog_local: self.hote.clone(), annuaire: &self.annuaire, revocations: &revocations };
        let possession = verifier_requete_passeport(&req, SERVICE, &politique, &defis, debut).unwrap();
        let visa = emettre_visa(
            &self.hote,
            &self.cle_hote,
            &possession,
            &DemandeVisa {
                audience: SERVICE.into(),
                portees: Portees::new([PORTEE_RESERVER]).unwrap(),
                pas_avant: debut,
                pas_apres: debut + 900,
            },
        )
        .unwrap();
        let copie = CleSignature::depuis_graine(&cle.graine());
        (copie, ClientFedere::new(cle, visa, passeport))
    }

    fn configuration(&self, snapshot_au: u64) -> ConfigurationFacade {
        let mut revocations = RevocationsFederees::new();
        revocations.inserer(ListeRevocation::new(self.hote.clone(), snapshot_au, 3_600));
        revocations.inserer(ListeRevocation::new(self.reconnu.clone(), snapshot_au, 3_600));
        ConfigurationFacade { cog_local: self.hote.clone(), annuaire: self.annuaire.clone(), revocations }
    }

    fn lancement(&self, auteurs: Vec<[u8; 32]>) -> LancementFacade {
        self.service.ouvrir_session(&jayrdv_facade::empreinte_image(), auteurs).unwrap()
    }
}

fn defi(f: &mut FacadeJayRdv, t: u64) -> DefiRecu {
    let r = f.traiter(&ClientFedere::requete_defi(), t);
    assert_eq!(r.statut, 200, "{}", String::from_utf8_lossy(&r.corps));
    DefiRecu::depuis(&r.corps).unwrap()
}

fn cache(f: &mut FacadeJayRdv, c: &ClientFedere, t: u64) -> ReponseHttp {
    let d = defi(f, t);
    f.traiter(&c.lire_cache(&d), t)
}

fn motif(r: &ReponseHttp) -> String {
    let v: serde_json::Value = serde_json::from_slice(&r.corps).unwrap_or_default();
    v["refus"].as_str().unwrap_or_default().to_string()
}

/// Façade dont les deux COG émetteurs ont une liste fraîche à `t`.
fn facade_rafraichie(
    m: &Monde,
    auteurs: Vec<[u8; 32]>,
    t: u64,
) -> (FacadeJayRdv, RevocationsFacade, Registre, Registre) {
    let revocations = RevocationsFacade::new();
    let mut reg_hote = Registre::new(m.hote.clone(), [3; 32]);
    let mut reg_reconnu = Registre::new(m.reconnu.clone(), [4; 32]);
    for r in [&mut reg_hote, &mut reg_reconnu] {
        r.publier_tete(&[], t);
        r.remettre(&revocations, t);
    }
    let f = FacadeJayRdv::avec_revocations(m.lancement(auteurs), m.configuration(t), revocations.clone());
    (f, revocations, reg_hote, reg_reconnu)
}

#[test]
fn quota_de_sessions_refuse_au_dela_et_libere_fin_et_inactivite() {
    let m = monde(|c| c.ressources.sessions_max = 2);
    let (_, a) = m.appareil(T0);
    let (_, b) = m.appareil(T0);
    let (_, c) = m.appareil(T0);
    let auteurs: Vec<[u8; 32]> = [&a, &b, &c].iter().map(|x| x.cle_appareil()).collect();
    let (mut f, revocations, mut rh, mut rr) = facade_rafraichie(&m, auteurs, T0);

    assert_eq!(cache(&mut f, &a, T0 + 10).statut, 200);
    assert_eq!(cache(&mut f, &b, T0 + 10).statut, 200);
    // Troisième appareil : quota de sessions (2) atteint ⇒ refus, rien d'ouvert.
    let r = cache(&mut f, &c, T0 + 10);
    assert_eq!((r.statut, motif(&r).as_str()), (429, "quota_sessions"));
    assert_eq!(f.sessions_ouvertes(), 2);
    assert_eq!(m.service.quotas().sessions().utilise(), 2);
    // Une session prolongée ne consomme pas de quota.
    assert_eq!(cache(&mut f, &a, T0 + 20).statut, 200);

    // Fin explicite de A : quota rendu, C entre.
    let d = defi(&mut f, T0 + 30);
    assert_eq!(f.traiter(&a.fermer_session(&d), T0 + 30).statut, 204);
    assert_eq!(m.service.quotas().sessions().utilise(), 1);
    assert_eq!(cache(&mut f, &c, T0 + 30).statut, 200);

    // Inactivité (15 min) : B et C libérées ; un nouvel appareil (visa plus tardif) entre.
    let tard = T0 + 30 + miyuhybride::DELAI_INACTIVITE_DEFAUT_SECS + 1;
    let (_, e) = m.appareil(tard - 100);
    m.service.ajouter_auteur(e.cle_appareil()).unwrap();
    for r in [&mut rh, &mut rr] {
        r.publier_tete(&[], tard);
        r.remettre(&revocations, tard);
    }
    assert_eq!(cache(&mut f, &e, tard).statut, 200);
    assert_eq!(f.sessions_ouvertes(), 1, "sessions inactives libérées");
    assert_eq!(m.service.quotas().sessions().utilise(), 1);

    // Destruction de la façade : tout est rendu.
    drop(f);
    assert_eq!(m.service.quotas().sessions().utilise(), 0);
    assert_eq!(m.service.quotas().disque().utilise(), 0);
}

#[test]
fn session_liberee_au_retrait_de_l_appareil() {
    let m = monde(|c| c.ressources.sessions_max = 2);
    let (_, a) = m.appareil(T0);
    let (_, b) = m.appareil(T0);
    let (mut f, _rev, _rh, _rr) = facade_rafraichie(&m, vec![a.cle_appareil(), b.cle_appareil()], T0);
    assert_eq!(cache(&mut f, &a, T0 + 10).statut, 200);
    assert_eq!(cache(&mut f, &b, T0 + 10).statut, 200);
    assert_eq!(m.service.quotas().sessions().utilise(), 2);
    // Les Cores retirent B (superviseur) : sa session est libérée aussitôt.
    m.service.retirer_auteur(&b.cle_appareil()).unwrap();
    f.auteur_retire(&b.cle_appareil());
    assert_eq!((f.sessions_ouvertes(), m.service.quotas().sessions().utilise()), (1, 1));

    // Retrait direct au service (sans notification) : libéré à la requête suivante.
    let (_, c) = m.appareil(T0);
    m.service.ajouter_auteur(c.cle_appareil()).unwrap();
    assert_eq!(cache(&mut f, &c, T0 + 20).statut, 200);
    assert_eq!(m.service.quotas().sessions().utilise(), 2);
    m.service.retirer_auteur(&c.cle_appareil()).unwrap();
    assert_eq!(cache(&mut f, &a, T0 + 30).statut, 200);
    assert_eq!((f.sessions_ouvertes(), m.service.quotas().sessions().utilise()), (1, 1));
}

#[test]
fn quota_de_taches_et_disque_comptes() {
    let m = monde(|c| c.ressources.taches_max = 1);
    let (_, a) = m.appareil(T0);
    let (mut f, _rev, _rh, _rr) = facade_rafraichie(&m, vec![a.cle_appareil()], T0);
    // Une tâche déjà en cours ailleurs dans le service : refus.
    let tache = m.service.quotas().lancer_tache().unwrap();
    let r = f.traiter(&ClientFedere::requete_defi(), T0 + 1);
    assert_eq!((r.statut, motif(&r).as_str()), (429, "quota_taches"));
    drop(tache);
    assert_eq!(m.service.quotas().taches().utilise(), 0);

    // Dépôt : la DB secondaire réserve son disque avant d'écrire.
    let mut a = a;
    let d = defi(&mut f, T0 + 2);
    let req = a.demander_creneaux(&d, "2026-11-02T00:00:00+00:00", "2026-11-03T00:00:00+00:00", T0).unwrap();
    assert_eq!(f.traiter(&req, T0 + 2).statut, 202);
    assert!(m.service.quotas().disque().utilise() > 0);
    assert_eq!(m.service.quotas().disque().utilise() as usize, f.db().utilise());
    assert_eq!(m.service.quotas().taches().utilise(), 0, "tâche rendue après la requête");
    drop(f);
    assert_eq!(m.service.quotas().disque().utilise(), 0, "façade détruite : disque rendu");
}

#[test]
fn titre_revoque_refuse_apres_rafraichissement() {
    let m = monde(|_| {});
    let (cle_a, a) = m.appareil(T0);
    let (_, b) = m.appareil(T0);
    let (mut f, revocations, _rh, mut rr) = facade_rafraichie(&m, vec![a.cle_appareil(), b.cle_appareil()], T0);
    assert_eq!(cache(&mut f, &a, T0 + 10).statut, 200);

    // Le COG reconnu révoque la clé d'appareil de A ; la zone COG rafraîchit.
    rr.publier_tete(&[Cible::Cle(empreinte_cle(&cle_a.cle_publique()))], T0 + 60);
    rr.remettre(&revocations, T0 + 60);
    let r = cache(&mut f, &a, T0 + 61);
    assert_ne!(r.statut, 200);
    assert_eq!(motif(&r), "identite");
    // B n'est pas touché.
    assert_eq!(cache(&mut f, &b, T0 + 61).statut, 200);

    // Recul refusé : une liste plus ancienne (sans la révocation) n'est pas reprise.
    let mut vieux = Registre::new(m.reconnu.clone(), [4; 32]);
    vieux.publier_tete(&[], T0 + 30);
    assert_eq!(revocations.rafraichir(&vieux.suivi), Err(RefusRemise::Recul));
    // Plus récente mais sans la révocation de A (autre journal) : refusée (inclusion exigée).
    let mut sans = Registre::new(m.reconnu.clone(), [4; 32]);
    sans.publier_tete(&[Cible::Cle([7; 32])], T0 + 90);
    assert_eq!(revocations.rafraichir(&sans.suivi), Err(RefusRemise::Recul));
    assert_ne!(cache(&mut f, &a, T0 + 91).statut, 200);
    // Liste jamais rafraîchie : refusée.
    let neuve = Registre::new(m.reconnu.clone(), [4; 32]);
    assert_eq!(revocations.rafraichir(&neuve.suivi), Err(RefusRemise::JamaisRafraichie));
}

#[test]
fn liste_absente_ou_perimee_refuse() {
    let m = monde(|_| {});
    let (_, a) = m.appareil(T0);

    // Aucune liste remise : refus (pas d'acceptation par défaut).
    let vide = RevocationsFacade::new();
    let mut f = FacadeJayRdv::avec_revocations(m.lancement(vec![a.cle_appareil()]), m.configuration(T0), vide);
    let r = cache(&mut f, &a, T0 + 10);
    assert_eq!((r.statut == 200, motif(&r).as_str()), (false, "identite"));
    drop(f);

    // Têtes à T0 − 600 : fraîches jusqu'à T0 + 300, périmées ensuite (visa encore valable).
    let (mut f, revocations, mut rh, mut rr) = facade_rafraichie(&m, vec![a.cle_appareil()], T0 - 600);
    assert_eq!(cache(&mut f, &a, T0 + 10).statut, 200);
    let r = cache(&mut f, &a, T0 + 301);
    assert_eq!((r.statut == 200, motif(&r).as_str()), (false, "identite"));
    // Une seule des deux listes rafraîchie : toujours refusé.
    rh.publier_tete(&[], T0 + 400);
    rh.remettre(&revocations, T0 + 400);
    assert_ne!(cache(&mut f, &a, T0 + 400).statut, 200);
    // Les deux : accepté de nouveau.
    rr.publier_tete(&[], T0 + 400);
    rr.remettre(&revocations, T0 + 400);
    assert_eq!(cache(&mut f, &a, T0 + 401).statut, 200);
}

#[test]
fn instantane_du_lancement_borne_a_15_minutes() {
    let m = monde(|_| {});
    // Façade ouverte à T0 − 800 ; instantané de fraîcheur propre 1 h.
    let (_, a) = m.appareil(T0 - 100);
    let mut f = FacadeJayRdv::new(m.lancement(vec![a.cle_appareil()]), m.configuration(T0 - 800));
    defi(&mut f, T0 - 800);
    assert_eq!(cache(&mut f, &a, T0 - 50).statut, 200);
    // Au-delà de 15 min après l'ouverture, sans rafraîchissement : refus.
    let r = cache(&mut f, &a, T0 + 101);
    assert_eq!((r.statut == 200, motif(&r).as_str()), (false, "identite"));
}

#[test]
fn instantane_ancien_refuse_meme_a_la_premiere_requete() {
    let m = monde(|_| {});
    // Instantané daté de T0 − 2 000 : la façade ne s'ouvre qu'à T0 ; la borne
    // suit la tête de la liste, pas l'heure d'ouverture.
    let (_, a) = m.appareil(T0 - 100);
    let mut f = FacadeJayRdv::new(m.lancement(vec![a.cle_appareil()]), m.configuration(T0 - 2_000));
    defi(&mut f, T0);
    let r = cache(&mut f, &a, T0 + 1);
    assert_eq!((r.statut == 200, motif(&r).as_str()), (false, "identite"));
}
