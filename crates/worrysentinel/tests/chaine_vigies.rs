//! @id     worrysentinel_test_chaine_vigies
//! @do     verifier_chaine_vigies_caringnanny_worrysentinel
//! @role   test
//! @layer  core
//! @human  Chaîne Vigies → CaringNanny → WorrySentinel → CoupureVoie (EX-12, EX-49).

use caringnanny::{CapteurAttendu, HealthStatus, ObservatoireVigies};
use miyuvigie::{
    CapteurId, Categorie, ConfigVigie, ControleSortie, EtatCapteur, EvenementFlux, EvenementId,
    ExutoirePortail, JeuRegles, PlanAdressage, Prefixe, Protocole, SourceEvenements,
    TentativeConnexion, Verdict, Vigie, VoieId, Zone,
};
use worrysentinel::{
    Action, CoupureVoie, DegradationState, ErreurCoupure, EtatVoie, Motif, NatureAudit,
    OrdreCoupure, PolitiqueVoies, ReponseVoieAveugle, SentinelleVoies,
};

const VOIES: [&str; 3] = ["jaymail", "jayrdv", "jaykoa"];

fn voie(s: &str) -> VoieId {
    VoieId::new(s).unwrap()
}
fn capteur(s: &str) -> CapteurId {
    CapteurId::new(s).unwrap()
}

fn plan() -> PlanAdressage {
    let mut p = PlanAdressage::indicatif();
    p.declarer(Prefixe::v4(10, 77, 0, 250, 32).unwrap(), Zone::Portail);
    p
}

fn controle() -> ControleSortie {
    let mut c = ControleSortie::new();
    for (i, v) in VOIES.iter().enumerate() {
        c.declarer_exutoire(ExutoirePortail {
            ip: "10.77.0.250".parse().unwrap(),
            port: 7001 + i as u16,
            voie: voie(v),
        });
    }
    c
}

fn vigie(id: &str, voies: &[&str]) -> Vigie {
    Vigie::new(ConfigVigie {
        capteur: capteur(id),
        zone: Zone::BorderGuard,
        voies_couvertes: voies.iter().map(|v| voie(v)).collect(),
        plan: plan(),
        regles: JeuRegles::vide(),
        controle_sortie: controle(),
        capacite_dedup: 1024,
        fenetre_rejeu_ms: 60_000,
    })
}

fn observatoire(capteurs: &[(&str, &[&str])]) -> ObservatoireVigies {
    let mut o = ObservatoireVigies::default();
    for (c, voies) in capteurs {
        o.enregistrer_capteur(CapteurAttendu {
            capteur: capteur(c),
            voies: voies.iter().map(|v| voie(v)).collect(),
            delai_pouls_max_ms: 5_000,
        });
    }
    o
}

fn flux(
    id: &str,
    t: u64,
    src: Zone,
    dst: Zone,
    v: Option<&str>,
    verdict: Verdict,
) -> EvenementFlux {
    EvenementFlux {
        id: EvenementId::new(id).unwrap(),
        horodatage_ms: t,
        zone_src: src,
        zone_dst: dst,
        voie: v.map(voie),
        verdict,
        octets: 60,
        ip_src: None,
        ip_dst: None,
        port_dst: Some(443),
        protocole: Some(Protocole::Tcp),
    }
}

/// Source Douane simulée.
struct SourceDouane {
    capteur: CapteurId,
    a_rendre: Vec<EvenementFlux>,
    en_panne: bool,
}

impl SourceEvenements for SourceDouane {
    fn capteur(&self) -> &CapteurId {
        &self.capteur
    }
    fn relever(&mut self) -> Result<Vec<EvenementFlux>, miyuvigie::ErreurSource> {
        if self.en_panne {
            return Err(miyuvigie::ErreurSource("douane muette".into()));
        }
        Ok(std::mem::take(&mut self.a_rendre))
    }
}

/// Portail simulé : enregistre les voies coupées.
#[derive(Default)]
struct PortailSimule {
    coupees: Vec<VoieId>,
    echouer: bool,
}

impl CoupureVoie for PortailSimule {
    fn couper(&mut self, ordre: &OrdreCoupure) -> Result<(), ErreurCoupure> {
        if self.echouer {
            return Err(ErreurCoupure("façade injoignable".into()));
        }
        self.coupees.push(ordre.voie.clone());
        Ok(())
    }
}

/// EX-12 + EX-49 : une sortie de zone COG sur la voie jaymail coupe jaymail
/// et seulement jaymail ; l'état global monte d'un cran (T1).
#[test]
fn sortie_zone_cog_coupe_la_seule_voie_concernee() {
    let mut douane = vigie("douane-bg", &VOIES);
    let mut nanny = observatoire(&[("douane-bg", &VOIES)]);
    let mut sentinelle = SentinelleVoies::default();
    let mut portail = PortailSimule::default();

    // Trafic sain sur toutes les voies : aucune décision.
    let mut src = SourceDouane {
        capteur: capteur("douane-bg"),
        a_rendre: vec![
            flux(
                "e1",
                1_000,
                Zone::Cog,
                Zone::Portail,
                Some("jaymail"),
                Verdict::Autorise,
            ),
            flux(
                "e2",
                1_000,
                Zone::Cog,
                Zone::Portail,
                Some("jayrdv"),
                Verdict::Autorise,
            ),
        ],
        en_panne: false,
    };
    nanny.ingerer(douane.relever(&mut src, 1_000));
    let d = sentinelle.evaluer(&nanny.transmettre(1_000));
    assert_eq!(d, Vec::new(), "aucune décision attendue");
    for v in VOIES {
        assert_eq!(sentinelle.etat_voie(&voie(v)), Some(EtatVoie::Ouverte));
    }

    // Tentative de sortie (bloquée par la Douane, mais la tentative compte).
    src.a_rendre = vec![flux(
        "e3",
        2_000,
        Zone::Cog,
        Zone::Exterieur,
        Some("jaymail"),
        Verdict::Bloque,
    )];
    let lot = douane.relever(&mut src, 2_000);
    assert_eq!(lot.signaux.len(), 1);
    assert_eq!(lot.signaux[0].categorie, Categorie::SortieZoneCog);
    nanny.ingerer(lot);
    let decisions = sentinelle.evaluer(&nanny.transmettre(2_000));

    let coupures: Vec<_> = decisions
        .iter()
        .filter(|d| matches!(d.action, Action::CouperVoie(_)))
        .collect();
    assert_eq!(coupures.len(), 1);
    assert_eq!(coupures[0].action, Action::CouperVoie(voie("jaymail")));
    assert_eq!(coupures[0].motif, Motif::IncidentCritique);
    // Explicable : preuves = signal + événement source.
    assert!(coupures[0].preuves.iter().any(|p| p == "e3"));
    assert!(coupures[0].explication.contains("R-EX12-SORTIE"));

    // EX-49 : les autres voies ne changent pas.
    assert_eq!(
        sentinelle.etat_voie(&voie("jaymail")),
        Some(EtatVoie::Coupee)
    );
    assert_eq!(
        sentinelle.etat_voie(&voie("jayrdv")),
        Some(EtatVoie::Ouverte)
    );
    assert_eq!(
        sentinelle.etat_voie(&voie("jaykoa")),
        Some(EtatVoie::Ouverte)
    );

    // Escalade globale par étape : T0 → T1 seulement.
    assert_eq!(sentinelle.etat_global(), DegradationState::Unstable);
    assert!(decisions.iter().any(|d| d.action
        == Action::ChangerEtatGlobal {
            de: DegradationState::Normal,
            vers: DegradationState::Unstable
        }));

    // Le consommateur ne reçoit qu'un ordre, pour jaymail.
    let r = sentinelle.appliquer_coupures(&decisions, &mut portail);
    assert_eq!(r.len(), 1);
    assert_eq!(portail.coupees, vec![voie("jaymail")]);
    assert_eq!(sentinelle.coupures_non_confirmees().len(), 0);

    // Le trafic sain des autres voies continue sans décision.
    src.a_rendre = vec![flux(
        "e4",
        3_000,
        Zone::Cog,
        Zone::Portail,
        Some("jayrdv"),
        Verdict::Autorise,
    )];
    nanny.ingerer(douane.relever(&mut src, 3_000));
    assert_eq!(sentinelle.evaluer(&nanny.transmettre(3_000)).len(), 0);
    assert_eq!(
        sentinelle.etat_voie(&voie("jayrdv")),
        Some(EtatVoie::Ouverte)
    );

    // Audit : séquence continue, décision de coupure tracée.
    let j = sentinelle.journal();
    assert!(j.windows(2).all(|w| w[1].sequence == w[0].sequence + 1));
    assert!(j.iter().any(|e| matches!(&e.nature, NatureAudit::CoupureConfirmee { voie: v, .. } if *v == voie("jaymail"))));
}

/// EX-12 par le contrôle de sortie : une Vigie de sandbox (une seule voie)
/// attribue la connexion à sa voie, qui est coupée seule.
#[test]
fn connexion_sortante_depuis_une_sandbox() {
    let mut sandbox = vigie("sandbox-jayrdv", &["jayrdv"]);
    let mut autre = vigie("sandbox-jaykoa", &["jaykoa"]);
    let mut nanny = observatoire(&[
        ("sandbox-jayrdv", &["jayrdv"]),
        ("sandbox-jaykoa", &["jaykoa"]),
    ]);
    let mut sentinelle = SentinelleVoies::default();

    let t = |id: &str, ip: &str, port: u16| TentativeConnexion {
        id: EvenementId::new(id).unwrap(),
        horodatage_ms: 1_000,
        voie: None,
        processus: Some("moteur-jayrdv".into()),
        ip_dst: ip.parse().unwrap(),
        port_dst: port,
        protocole: Protocole::Tcp,
    };
    // Portail de sa propre voie et boucle locale : légitimes.
    nanny.ingerer(autre.analyser_connexions(vec![t("k1", "127.0.0.1", 5432)], 1_000));
    let lot = sandbox.analyser_connexions(
        vec![
            t("c1", "10.77.0.250", 7002),
            t("c2", "127.0.0.1", 8080),
            t("c3", "1.1.1.1", 53),
        ],
        1_000,
    );
    assert_eq!(lot.signaux.len(), 1);
    assert_eq!(lot.signaux[0].voie, Some(voie("jayrdv")));
    nanny.ingerer(lot);
    let d = sentinelle.evaluer(&nanny.transmettre(1_000));
    assert!(d
        .iter()
        .any(|d| d.action == Action::CouperVoie(voie("jayrdv"))));
    assert_eq!(
        sentinelle.etat_voie(&voie("jaykoa")),
        Some(EtatVoie::Ouverte)
    );
}

/// Un capteur muet ou périmé rend la voie `Inconnu` (jamais `Healthy`) ;
/// politique par défaut : voie aveugle coupée (fail-closed), sans escalade globale.
#[test]
fn capteur_perime_voie_inconnue() {
    let mut douane = vigie("douane-bg", &["jaymail"]);
    let mut sandbox = vigie("sandbox-jayrdv", &["jayrdv"]);
    let mut nanny = observatoire(&[("douane-bg", &["jaymail"]), ("sandbox-jayrdv", &["jayrdv"])]);

    nanny.ingerer(douane.analyser_flux(vec![], 1_000));
    nanny.ingerer(sandbox.analyser_flux(vec![], 1_000));
    assert_eq!(
        nanny.sante_voie(&voie("jaymail"), 2_000),
        HealthStatus::Healthy
    );

    // Seule la sandbox continue d'émettre ; la Douane se tait.
    nanny.ingerer(sandbox.analyser_flux(vec![], 9_000));
    let rapport = nanny.transmettre(9_000);
    assert_eq!(rapport.sante_voies[&voie("jaymail")], HealthStatus::Inconnu);
    assert_eq!(rapport.sante_voies[&voie("jayrdv")], HealthStatus::Healthy);

    let mut defaut = SentinelleVoies::default();
    let d = defaut.evaluer(&rapport);
    assert_eq!(d.len(), 1);
    assert_eq!(d[0].action, Action::CouperVoie(voie("jaymail")));
    assert_eq!(d[0].motif, Motif::VoieAveugle);
    assert_eq!(defaut.etat_voie(&voie("jayrdv")), Some(EtatVoie::Ouverte));
    assert_eq!(defaut.etat_global(), DegradationState::Normal);

    let mut souple = SentinelleVoies::new(PolitiqueVoies {
        voie_aveugle: ReponseVoieAveugle::Surveiller,
        ..PolitiqueVoies::default()
    });
    souple.evaluer(&rapport);
    assert_eq!(
        souple.etat_voie(&voie("jaymail")),
        Some(EtatVoie::Surveillee)
    );

    // Source en panne : aucun pouls, capteur Unhealthy.
    let mut src = SourceDouane {
        capteur: capteur("douane-bg"),
        a_rendre: vec![],
        en_panne: true,
    };
    let lot = douane.relever(&mut src, 10_000);
    assert_eq!(lot.etat, EtatCapteur::EnEchec);
    assert!(lot.pouls.is_none());
    nanny.ingerer(lot);
    assert_eq!(
        nanny.sante_capteur(&capteur("douane-bg"), 10_000),
        HealthStatus::Unhealthy
    );
}

/// Lignes de journal mal formées : ignorées, comptées, jamais une preuve
/// bénigne ni un pouls ; la voie passe sous surveillance.
#[test]
fn lignes_mal_formees_ignorees_et_comptees() {
    let mut v = vigie("journal-bg", &["jaymail"]);
    let mut nanny = observatoire(&[("journal-bg", &["jaymail"])]);
    let mut sentinelle = SentinelleVoies::new(PolitiqueVoies {
        voie_aveugle: ReponseVoieAveugle::Surveiller,
        ..PolitiqueVoies::default()
    });

    // Rejet ordinaire de la frontière externe (convention MDOU, A-004) : aucun signal.
    let saine = "1729760000.000000 hote nflog-101: MDOU|externe|transit|NON-DECLARE| IN=ext0 OUT=ext-franche SRC=192.0.2.66 DST=10.77.1.10 LEN=60 PROTO=TCP SPT=40000 DPT=22";
    let lot = v.analyser_journal([saine], 1_000);
    assert_eq!(lot.signaux.len(), 0);
    assert!(lot.pouls.is_some());
    nanny.ingerer(lot);
    assert_eq!(
        nanny.sante_voie(&voie("jaymail"), 1_000),
        HealthStatus::Healthy
    );

    let lignes = [
        "1729760001.000000 hote nflog-101: MDOU|externe|transit|NON-DECLARE| IN=ext0 SRC=192.0.2.66",
        "1729760001.000000 hote nflog-101: MDOU|externe|transit|NON-DECLARE| IN=ext0 OUT=x SRC=192.0.2.66 DST=10.77.1.10 LEN=60 PROTO=TCP DPT=22 DPT=22",
        "MDOU|garbage",
        "1729760001.000000 hote kernel: carte réseau prête",
    ];
    let lot = v.analyser_journal(lignes, 2_000);
    assert_eq!(lot.compteurs.mal_formees, 3);
    assert_eq!(lot.compteurs.hors_convention, 1);
    assert_eq!(lot.compteurs.acceptees, 0);
    assert_eq!(lot.etat, EtatCapteur::Degrade);
    assert!(
        lot.pouls.is_none(),
        "un lot rejeté n'est pas une preuve de vie"
    );
    assert_eq!(lot.signaux.len(), 1);
    assert_eq!(lot.signaux[0].categorie, Categorie::CapteurDegrade);
    nanny.ingerer(lot);
    assert_eq!(
        nanny.sante_voie(&voie("jaymail"), 2_000),
        HealthStatus::Degraded
    );
    assert_eq!(
        nanny.compteurs(&capteur("journal-bg")).unwrap().mal_formees,
        3
    );

    let d = sentinelle.evaluer(&nanny.transmettre(2_000));
    assert_eq!(d.len(), 1);
    assert_eq!(d[0].action, Action::SurveillerVoie(voie("jaymail")));
    assert_eq!(sentinelle.etat_global(), DegradationState::Normal);

    // Sans nouvelle ligne valide, le pouls vieillit : Inconnu, jamais Healthy.
    assert_eq!(
        nanny.sante_voie(&voie("jaymail"), 7_000),
        HealthStatus::Inconnu
    );
}

/// Événements rejoués / dupliqués : un seul signal, une seule décision.
#[test]
fn rejeu_et_doublons_dedoublonnes() {
    let mut douane = vigie("douane-bg", &VOIES);
    let mut nanny = observatoire(&[("douane-bg", &VOIES)]);
    let mut sentinelle = SentinelleVoies::default();
    let mut portail = PortailSimule::default();

    let e = flux(
        "x1",
        10_000,
        Zone::Cog,
        Zone::Exterieur,
        Some("jaykoa"),
        Verdict::Bloque,
    );
    let lot = douane.analyser_flux(vec![e.clone(), e.clone()], 10_000);
    assert_eq!(lot.signaux.len(), 1);
    assert_eq!(lot.compteurs.doublons, 1);
    let copie = lot.clone();
    nanny.ingerer(lot);
    // Même lot réinjecté dans CaringNanny : écarté.
    let recu = nanny.ingerer(copie.clone());
    assert_eq!((recu.nouveaux, recu.doublons), (0, 1));
    let rapport = nanny.transmettre(10_000);
    let d1 = sentinelle.evaluer(&rapport);
    // Même rapport rejoué vers WorrySentinel : aucune nouvelle décision.
    let d2 = sentinelle.evaluer(&rapport);
    assert_eq!(
        d1.iter()
            .filter(|d| matches!(d.action, Action::CouperVoie(_)))
            .count(),
        1
    );
    assert_eq!(d2.len(), 0);
    sentinelle.appliquer_coupures(&d1, &mut portail);
    sentinelle.appliquer_coupures(&d2, &mut portail);
    assert_eq!(portail.coupures_len(), 1);

    // Rejeu d'un événement ancien (hors fenêtre) : écarté et compté.
    let lot = douane.analyser_flux(
        vec![
            flux(
                "x2",
                200_000,
                Zone::Cog,
                Zone::Portail,
                Some("jaymail"),
                Verdict::Autorise,
            ),
            flux(
                "x0",
                1_000,
                Zone::Cog,
                Zone::Exterieur,
                Some("jaymail"),
                Verdict::Bloque,
            ),
        ],
        200_000,
    );
    assert_eq!(lot.compteurs.rejeus, 1);
    assert!(lot
        .signaux
        .iter()
        .all(|s| s.categorie == Categorie::CapteurDegrade));
}

impl PortailSimule {
    fn coupures_len(&self) -> usize {
        self.coupees.len()
    }
}

/// Coupure non confirmée : la décision tient et reste à rejouer.
#[test]
fn coupure_non_confirmee_reste_a_rejouer() {
    let mut douane = vigie("douane-bg", &VOIES);
    let mut nanny = observatoire(&[("douane-bg", &VOIES)]);
    let mut sentinelle = SentinelleVoies::default();
    let mut portail = PortailSimule {
        echouer: true,
        ..PortailSimule::default()
    };
    nanny.ingerer(douane.analyser_flux(
        vec![flux(
            "y1",
            1_000,
            Zone::Franche,
            Zone::Cog,
            Some("jayrdv"),
            Verdict::Autorise,
        )],
        1_000,
    ));
    let d = sentinelle.evaluer(&nanny.transmettre(1_000));
    let r = sentinelle.appliquer_coupures(&d, &mut portail);
    assert!(r[0].1.is_err());
    assert_eq!(
        sentinelle.etat_voie(&voie("jayrdv")),
        Some(EtatVoie::Coupee)
    );
    assert_eq!(sentinelle.coupures_non_confirmees(), vec![voie("jayrdv")]);
}

/// Escalade globale : un cran par évaluation, plafonnée à T3, jamais T4.
#[test]
fn escalade_globale_par_etapes_sans_t4() {
    let mut douane = vigie("douane-bg", &VOIES);
    let mut nanny = observatoire(&[("douane-bg", &VOIES)]);
    let mut sentinelle = SentinelleVoies::default();

    let evts = VOIES
        .iter()
        .enumerate()
        .map(|(i, v)| {
            flux(
                &format!("z{i}"),
                1_000,
                Zone::Cog,
                Zone::Exterieur,
                Some(*v),
                Verdict::Bloque,
            )
        })
        .chain(std::iter::once(flux(
            "zh",
            1_000,
            Zone::Cog,
            Zone::Exterieur,
            None,
            Verdict::Bloque,
        )))
        .collect();
    nanny.ingerer(douane.analyser_flux(evts, 1_000));
    let d = sentinelle.evaluer(&nanny.transmettre(1_000));
    assert_eq!(
        d.iter()
            .filter(|d| matches!(d.action, Action::CouperVoie(_)))
            .count(),
        3
    );
    assert!(d.iter().any(|d| d.action == Action::IncidentHorsVoie));
    assert_eq!(sentinelle.etat_global(), DegradationState::Unstable);

    let mut etats = vec![];
    for t in [2_000, 3_000, 4_000, 5_000] {
        nanny.ingerer(douane.analyser_flux(vec![], t));
        sentinelle.evaluer(&nanny.transmettre(t));
        etats.push(sentinelle.etat_global());
    }
    assert_eq!(
        etats,
        vec![
            DegradationState::Degraded,
            DegradationState::Restricted,
            DegradationState::Restricted,
            DegradationState::Restricted
        ]
    );

    // Réarmement : autorisation obligatoire, tracé.
    assert!(sentinelle.rearmer(&voie("jaymail"), "  ", 6_000).is_err());
    let d = sentinelle
        .rearmer(&voie("jaymail"), "TAMR approbation #42", 6_000)
        .unwrap();
    assert_eq!(d.action, Action::RearmerVoie(voie("jaymail")));
    assert_eq!(
        sentinelle.etat_voie(&voie("jaymail")),
        Some(EtatVoie::Ouverte)
    );
    assert_eq!(
        sentinelle.etat_voie(&voie("jayrdv")),
        Some(EtatVoie::Coupee)
    );
}
