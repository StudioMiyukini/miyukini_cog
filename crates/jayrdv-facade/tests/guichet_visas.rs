//! @id     jayrdv_facade_tests_guichet_visas
//! @do     tester_guichet_visas_bout_en_bout
//! @role   test
//! @layer  service
//! @human  jayrdv-facade — guichet des visas de bout en bout (A-022 §1, A-024, A-027) : demande par la façade, émission en zone COG par la clé `visas` (indice 5) avec sa délégation, accès accepté ; refus (révocation, délégation absente ou expirée, possession, durée, portée, rejeu, redémarrage).

#![allow(clippy::doc_markdown, clippy::many_single_char_names)]

use std::path::PathBuf;
use std::sync::Arc;

use jayrdv_facade::{
    requete_demande_visa, requete_lire_visa, ClientFedere, ConfigurationFacade, DefiRecu, FacadeJayRdv,
    ParametresDemandeVisa, ReponseHttp, VisaRecu, PORTEE_RESERVER,
};
use jayrdv_traite::SERVICE;
use miyauth::http::RegistreNonces;
use miyauth::{
    emettre_passeport, AnnuaireEmetteurs, CogId, DemandePasseport, ListeRevocation, Portees, RevocationsFederees,
    SecretTitulaire,
};
use miyuhybride::{
    sceller_manifeste, verifier_manifeste, BilanGuichet, ConfigGuichetVisas, GuichetVisas, ManifesteService,
    RevocationsFacade, ServiceHybride, VoieVisas,
};
use miyuportail::sas::{decoder_reponse, traiter_requete, ResultatTrame};
use miyuportail::{DecodeurIsole, ErreurSas, IdService, PortailInterne, RefusParDefaut, RegistreMemoire, RequeteSas};
use miyuracine::{Racine, Usage};
use miyuregistre::{Cible, Corps, EntreeRegistre, Journal, StockageMemoire, SuiviRevocationsRegistre};
use miyusceau::{empreinte_cle, sceller, AncresDeConfiance, CleSignature, NiveauConfiance};

const T0: u64 = 1_800_000_000;
const JOUR: u64 = 86_400;

struct SasEnProcessus;

impl DecodeurIsole for SasEnProcessus {
    fn decoder_lot(&self, requete: &RequeteSas) -> Result<Vec<ResultatTrame>, ErreurSas> {
        let r =
            traiter_requete(&requete.encoder().map_err(|_| ErreurSas::Reponse)?).map_err(|_| ErreurSas::FinAnormale)?;
        decoder_reponse(&r, requete.trames.len())
    }
}

/// Registre de révocation du COG hôte, suivi par la zone COG.
struct Registre {
    cle: CleSignature,
    journal: Journal<StockageMemoire>,
    suivi: SuiviRevocationsRegistre,
}

impl Registre {
    fn new(cog: CogId) -> Self {
        let cle = CleSignature::generer();
        Self {
            journal: Journal::ouvrir([3; 32], StockageMemoire::new(), StockageMemoire::new()).unwrap(),
            suivi: SuiviRevocationsRegistre::new(cog, [3; 32], &cle.cle_publique()).unwrap(),
            cle,
        }
    }

    fn publier(&mut self, cibles: &[Cible], t: u64, rev: &RevocationsFacade) {
        let m = self.suivi.taille().unwrap();
        for c in cibles {
            let e = EntreeRegistre { inscrite_a: t, corps: Corps::Revocation { cible: *c, motif: "perte".into() } };
            self.journal.ajouter(&sceller(&e, &self.cle).unwrap()).unwrap();
        }
        let tete = self.journal.emettre_tete(&[&self.cle], t, 86_400).unwrap();
        self.suivi.appliquer(&tete, m, &self.journal.entrees(m, u64::MAX).unwrap(), t).unwrap();
        rev.rafraichir(&self.suivi).unwrap();
    }
}

fn dossier_nonces(nom: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("jayrdv-guichet-visas-{nom}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    d
}

struct Monde {
    racine: Racine,
    hote: CogId,
    annuaire: AnnuaireEmetteurs,
    deleg_emission: Vec<u8>,
    deleg_visas: Vec<u8>,
    revocations: RevocationsFacade,
    registre: Registre,
    service: Arc<ServiceHybride>,
    voie: VoieVisas,
    nonces: PathBuf,
}

impl Monde {
    fn new(nom: &str) -> Self {
        Self::avec_fin_visas(nom, T0 + 60 * JOUR)
    }

    /// COG hôte à racine épinglée ; délégation `visas` jusqu'à `fin_visas`.
    fn avec_fin_visas(nom: &str, fin_visas: u64) -> Self {
        let racine = Racine::depuis_secret(&[0x42; 32]);
        let hote = CogId::new(&racine.cog_id()).unwrap();
        let mut annuaire = AnnuaireEmetteurs::new();
        annuaire.epingler_racine(hote.clone(), &racine.cle_publique_identite(), NiveauConfiance::Souverain).unwrap();
        let deleg_emission = racine.deleguer(Usage::Emission, 0, T0 - JOUR, T0 + 60 * JOUR).unwrap();
        let deleg_visas = racine.deleguer(Usage::Visas, 0, T0 - JOUR, fin_visas).unwrap();

        let revocations = RevocationsFacade::new();
        let mut registre = Registre::new(hote.clone());
        registre.publier(&[], T0, &revocations);

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
        let portail = Arc::new(PortailInterne::new(
            Box::new(SasEnProcessus),
            Box::new(RefusParDefaut),
            Arc::new(RegistreMemoire::new()),
        ));
        let service = Arc::new(ServiceHybride::installer(Arc::clone(&portail), verifie).unwrap());
        service.activer_exposition().unwrap();
        let voie = VoieVisas::installer(portail).unwrap();
        voie.activer().unwrap();
        Self {
            racine,
            hote,
            annuaire,
            deleg_emission,
            deleg_visas,
            revocations,
            registre,
            service,
            voie,
            nonces: dossier_nonces(nom),
        }
    }

    /// Guichet des visas en zone COG (registre de nonces persistant).
    fn guichet(&self) -> GuichetVisas {
        GuichetVisas::new(
            ConfigGuichetVisas {
                cog_local: self.hote.clone(),
                annuaire: &self.annuaire,
                cle_visas: self.racine.cle(Usage::Visas, 0).unwrap(),
                delegation_visas: self.deleg_visas.clone(),
                audiences: &[SERVICE],
            },
            self.revocations.clone(),
            RegistreNonces::persistant(&self.nonces, 1000, 300).unwrap(),
            T0,
        )
        .unwrap()
    }

    /// Passeport citoyen, signé par la clé `emission` (indice 1, génération `g`).
    fn passeport(&self, cle: &CleSignature, g: u32) -> Vec<u8> {
        emettre_passeport(
            &self.hote,
            &self.racine.cle(Usage::Emission, g).unwrap(),
            &DemandePasseport {
                secret: &SecretTitulaire::generer(),
                destination: self.hote.clone(),
                cle_appareil: cle.cle_publique(),
                portees: Portees::new([PORTEE_RESERVER]).unwrap(),
                pas_avant: T0 - 3_600,
                pas_apres: T0 + JOUR,
            },
        )
        .unwrap()
    }

    /// Façade JayRDV (mode strict : racine épinglée) reliée à la voie des visas.
    fn facade(&self, auteurs: Vec<[u8; 32]>) -> FacadeJayRdv {
        let lancement = self.service.ouvrir_session(&jayrdv_facade::empreinte_image(), auteurs).unwrap();
        let mut instantane = RevocationsFederees::new();
        instantane.inserer(ListeRevocation::new(self.hote.clone(), T0, 900));
        let config = ConfigurationFacade { cog_local: self.hote.clone(), annuaire: self.annuaire.clone(), revocations: instantane };
        FacadeJayRdv::avec_revocations(lancement, config, self.revocations.clone())
            .avec_guichet_visas(self.voie.ouvrir_session().unwrap())
    }
}

fn defi(f: &mut FacadeJayRdv, t: u64) -> DefiRecu {
    let r = f.traiter(&ClientFedere::requete_defi(), t);
    assert_eq!(r.statut, 200, "{}", String::from_utf8_lossy(&r.corps));
    DefiRecu::depuis(&r.corps).unwrap()
}

fn motif(r: &ReponseHttp) -> String {
    let v: serde_json::Value = serde_json::from_slice(&r.corps).unwrap_or_default();
    v["refus"].as_str().unwrap_or_default().to_string()
}

fn params<'a>(passeport: &'a [u8], deleg: Option<&'a [u8]>, portees: &'a [&'a str], duree: u64) -> ParametresDemandeVisa<'a> {
    ParametresDemandeVisa { passeport, delegation_passeport: deleg, portees, duree_secs: duree, emise_le: T0 }
}

/// Dépose une demande, fait tourner le guichet, rend la réponse de `GET /v1/visa/<demande>`.
fn demander(
    f: &mut FacadeJayRdv,
    g: &GuichetVisas,
    m: &Monde,
    req: &jayrdv_facade::RequeteHttp,
) -> (ReponseHttp, BilanGuichet) {
    let r = f.traiter(req, T0);
    assert_eq!(r.statut, 202, "{}", String::from_utf8_lossy(&r.corps));
    let demande: serde_json::Value = serde_json::from_slice(&r.corps).unwrap();
    let demande = demande["demande"].as_str().unwrap().to_string();
    assert_eq!(f.traiter(&requete_lire_visa(&demande), T0).statut, 202, "en attente avant la relève");
    let bilan = g.traiter(&m.voie.guichet_moteur(), 16, T0).unwrap();
    (f.traiter(&requete_lire_visa(&demande), T0), bilan)
}

fn essai(f: &mut FacadeJayRdv, g: &GuichetVisas, m: &Monde, req: &jayrdv_facade::RequeteHttp) -> String {
    let (r, b) = demander(f, g, m, req);
    assert_eq!((r.statut, b.emis), (403, 0), "{}", String::from_utf8_lossy(&r.corps));
    motif(&r)
}

fn cache(f: &mut FacadeJayRdv, c: &ClientFedere, t: u64) -> ReponseHttp {
    let d = defi(f, t);
    f.traiter(&c.lire_cache(&d), t)
}

#[test]
fn bout_en_bout_demande_guichet_visa_acces() {
    let m = Monde::new("nominal");
    let cle = CleSignature::generer();
    let passeport = m.passeport(&cle, 0);
    let mut f = m.facade(vec![cle.cle_publique()]);
    assert!(f.mode_strict(), "racine du COG hôte connue : mode strict");
    let g = m.guichet();

    let d = defi(&mut f, T0);
    let req = requete_demande_visa(&cle, &d, &params(&passeport, Some(&m.deleg_emission), &[PORTEE_RESERVER], 900));
    let (r, bilan) = demander(&mut f, &g, &m, &req);
    assert_eq!((r.statut, bilan.emis), (200, 1), "{}", String::from_utf8_lossy(&r.corps));
    let v = VisaRecu::depuis(&r.corps).unwrap();
    assert_eq!(v.pas_apres, T0 + 900, "visa de 15 min au plus");
    assert_eq!(v.delegation, m.deleg_visas, "délégation de la clé `visas` jointe au visa");

    // Accès accepté par la façade, délégations présentées dans X-Miyu-Delegations.
    let copie = CleSignature::depuis_graine(&cle.graine());
    let client = ClientFedere::new(copie, v.visa.clone(), passeport.clone())
        .avec_delegations(vec![v.delegation.clone(), m.deleg_emission.clone()]);
    let r = cache(&mut f, &client, T0 + 5);
    assert_eq!(r.statut, 200, "{}", String::from_utf8_lossy(&r.corps));

    // Sans la délégation de la clé `visas` : refus (mode strict, aucune ancre directe).
    let sans_visas = ClientFedere::new(CleSignature::depuis_graine(&cle.graine()), v.visa.clone(), passeport.clone())
        .avec_delegations(vec![m.deleg_emission.clone()]);
    assert_eq!(motif(&cache(&mut f, &sans_visas, T0 + 6)), "identite");
    let sans_rien = ClientFedere::new(CleSignature::depuis_graine(&cle.graine()), v.visa, passeport);
    assert_eq!(motif(&cache(&mut f, &sans_rien, T0 + 6)), "identite");
}

#[test]
fn visa_jamais_au_dela_de_la_delegation() {
    let m = Monde::avec_fin_visas("fin-delegation", T0 + 300);
    let cle = CleSignature::generer();
    let passeport = m.passeport(&cle, 0);
    let mut f = m.facade(vec![cle.cle_publique()]);
    let g = m.guichet();
    assert_eq!(g.fin_delegation(), T0 + 300);
    let d = defi(&mut f, T0);
    let req = requete_demande_visa(&cle, &d, &params(&passeport, Some(&m.deleg_emission), &[PORTEE_RESERVER], 900));
    let (r, _) = demander(&mut f, &g, &m, &req);
    assert_eq!(VisaRecu::depuis(&r.corps).unwrap().pas_apres, T0 + 300);
}

#[test]
fn refus_du_guichet() {
    let mut m = Monde::new("refus");
    let cle = CleSignature::generer();
    let passeport = m.passeport(&cle, 0);
    let mut f = m.facade(vec![cle.cle_publique()]);
    let g = m.guichet();

    // Visa de plus de 15 min.
    let d = defi(&mut f, T0);
    let req = requete_demande_visa(&cle, &d, &params(&passeport, Some(&m.deleg_emission), &[PORTEE_RESERVER], 901));
    assert_eq!(essai(&mut f, &g, &m, &req), "visa_duree");

    // Portée non incluse dans le passeport.
    let d = defi(&mut f, T0);
    let req = requete_demande_visa(&cle, &d, &params(&passeport, Some(&m.deleg_emission), &["jayrdv:admin"], 600));
    assert_eq!(essai(&mut f, &g, &m, &req), "visa_portee");

    // Délégation de la clé `emission` absente : la chaîne ne tient pas.
    let d = defi(&mut f, T0);
    let req = requete_demande_visa(&cle, &d, &params(&passeport, None, &[PORTEE_RESERVER], 600));
    assert_eq!(essai(&mut f, &g, &m, &req), "visa_delegation");

    // Preuve de possession invalide : signée par une autre clé.
    let autre = CleSignature::generer();
    let d = defi(&mut f, T0);
    let mut req = requete_demande_visa(&autre, &d, &params(&passeport, Some(&m.deleg_emission), &[PORTEE_RESERVER], 600));
    let mut corps: serde_json::Value = serde_json::from_slice(&req.corps).unwrap();
    corps["cle_appareil"] = miyauth::http::b64_encoder(&cle.cle_publique()).into();
    req.corps = serde_json::to_vec(&corps).unwrap();
    assert_eq!(essai(&mut f, &g, &m, &req), "visa_possession");

    // Rejeu : même demande (même défi, même preuve) : refusée par la zone COG.
    let d = defi(&mut f, T0);
    let req = requete_demande_visa(&cle, &d, &params(&passeport, Some(&m.deleg_emission), &[PORTEE_RESERVER], 600));
    let (r, _) = demander(&mut f, &g, &m, &req);
    assert_eq!(r.statut, 200);
    assert_eq!(essai(&mut f, &g, &m, &req), "visa_rejeu");

    // Passeport révoqué (clé d'appareil) : refus.
    m.registre.publier(&[Cible::Cle(empreinte_cle(&cle.cle_publique()))], T0, &m.revocations);
    let d = defi(&mut f, T0);
    let req = requete_demande_visa(&cle, &d, &params(&passeport, Some(&m.deleg_emission), &[PORTEE_RESERVER], 600));
    assert_eq!(essai(&mut f, &g, &m, &req), "visa_revoque");
}

#[test]
fn delegation_expiree_refusee() {
    let m = Monde::new("expiree");
    let cle = CleSignature::generer();
    // Passeport signé par la génération 1 de `emission`, dont la délégation a expiré hier.
    let passeport = m.passeport(&cle, 1);
    let expiree = m.racine.deleguer(Usage::Emission, 1, T0 - 10 * JOUR, T0 - JOUR).unwrap();
    let mut f = m.facade(vec![cle.cle_publique()]);
    let g = m.guichet();
    let d = defi(&mut f, T0);
    let req = requete_demande_visa(&cle, &d, &params(&passeport, Some(&expiree), &[PORTEE_RESERVER], 600));
    let (r, _) = demander(&mut f, &g, &m, &req);
    assert_eq!((r.statut, motif(&r).as_str()), (403, "visa_delegation"));

    // Guichet dont la délégation `visas` est expirée : refuse de démarrer.
    let vieille = m.racine.deleguer(Usage::Visas, 1, T0 - 10 * JOUR, T0 - JOUR).unwrap();
    assert!(GuichetVisas::new(
        ConfigGuichetVisas {
            cog_local: m.hote.clone(),
            annuaire: &m.annuaire,
            cle_visas: m.racine.cle(Usage::Visas, 1).unwrap(),
            delegation_visas: vieille,
            audiences: &[SERVICE],
        },
        m.revocations.clone(),
        RegistreNonces::default(),
        T0,
    )
    .is_err());
    // Clé `emission` (indice 1) présentée comme clé de visas : refusée.
    assert!(GuichetVisas::new(
        ConfigGuichetVisas {
            cog_local: m.hote.clone(),
            annuaire: &m.annuaire,
            cle_visas: m.racine.cle(Usage::Emission, 0).unwrap(),
            delegation_visas: m.deleg_emission.clone(),
            audiences: &[SERVICE],
        },
        m.revocations.clone(),
        RegistreNonces::default(),
        T0,
    )
    .is_err());
}

#[test]
fn revocations_perimees_refusees() {
    let m = Monde::new("perimees");
    let cle = CleSignature::generer();
    let passeport = m.passeport(&cle, 0);
    let mut f = m.facade(vec![cle.cle_publique()]);
    let g = m.guichet();
    // 16 min après la dernière tête vérifiée : le guichet refuse (fraîcheur ≤ 15 min).
    let t = T0 + 16 * 60;
    let d = defi(&mut f, t);
    let mut p = params(&passeport, Some(&m.deleg_emission), &[PORTEE_RESERVER], 600);
    p.emise_le = t;
    let req = requete_demande_visa(&cle, &d, &p);
    let r = f.traiter(&req, t);
    let demande = serde_json::from_slice::<serde_json::Value>(&r.corps).unwrap()["demande"].as_str().unwrap().to_string();
    g.traiter(&m.voie.guichet_moteur(), 16, t).unwrap();
    let r = f.traiter(&requete_lire_visa(&demande), t);
    assert_eq!((r.statut, motif(&r).as_str()), (403, "visa_revocations"));
}

#[test]
fn defi_rejoue_apres_redemarrage_du_guichet() {
    let m = Monde::new("redemarrage");
    let cle = CleSignature::generer();
    let passeport = m.passeport(&cle, 0);
    let mut f = m.facade(vec![cle.cle_publique()]);
    let d = defi(&mut f, T0);
    let req = requete_demande_visa(&cle, &d, &params(&passeport, Some(&m.deleg_emission), &[PORTEE_RESERVER], 600));
    {
        let g = m.guichet();
        assert_eq!(demander(&mut f, &g, &m, &req).0.statut, 200);
    }
    // Nouveau processus du guichet (même dossier de nonces) : la même preuve est refusée.
    let g = m.guichet();
    let (r, _) = demander(&mut f, &g, &m, &req);
    assert_eq!((r.statut, motif(&r).as_str()), (403, "visa_rejeu"));
    let _ = std::fs::remove_dir_all(&m.nonces);
}

#[test]
fn visa_signe_par_l_indice_1_refuse_par_la_facade() {
    let m = Monde::new("indice1");
    let cle = CleSignature::generer();
    let passeport = m.passeport(&cle, 0);
    let mut f = m.facade(vec![cle.cle_publique()]);
    // Un visa forgé avec la clé `emission` (indice 1) au lieu de `visas` (indice 5).
    let mut annuaire = m.annuaire.clone();
    annuaire.ajouter_delegation(&m.deleg_emission, T0).unwrap();
    let rev = {
        let mut r = RevocationsFederees::new();
        r.inserer(ListeRevocation::new(m.hote.clone(), T0, 900));
        r
    };
    let politique = miyauth::PolitiqueIdentite { cog_local: m.hote.clone(), annuaire: &annuaire, revocations: &rev };
    let pv = miyauth::verifier_passeport(&passeport, &politique, T0).unwrap();
    let liaison = [7u8; 32];
    let defi_local = [9u8; 32];
    let sig = miyauth::http::signer_pop(&cle, &defi_local, SERVICE, &liaison);
    let possession = miyauth::prouver_possession(pv, &defi_local, SERVICE, &liaison, &sig).unwrap();
    let visa = miyauth::emettre_visa(
        &m.hote,
        &m.racine.cle(Usage::Emission, 0).unwrap(),
        &possession,
        &miyauth::DemandeVisa {
            audience: SERVICE.into(),
            portees: Portees::new([PORTEE_RESERVER]).unwrap(),
            pas_avant: T0,
            pas_apres: T0 + 900,
        },
    )
    .unwrap();
    let client = ClientFedere::new(CleSignature::depuis_graine(&cle.graine()), visa, passeport)
        .avec_delegations(vec![m.deleg_emission.clone()]);
    assert_eq!(motif(&cache(&mut f, &client, T0 + 1)), "identite");
}
