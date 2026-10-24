//! @id     miyufrontiere_test_bout_en_bout
//! @do     verifier_critere_sortie_lot3
//! @role   test
//! @layer  operateur
//! @human  Lot 3 bout en bout (niveau bibliothèque) : arrêté signé → Douane + IPS → Vigies → CaringNanny → WorrySentinel → coupure d'une voie au Portail et à la Douane, l'autre voie intacte (EX-12, EX-49) ; capteur périmé ⇒ voie aveugle coupée.

#![allow(clippy::assert_is_empty, clippy::struct_field_names)]

use std::collections::BTreeSet;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};

use borderguard::{sceller_arrete, verifier_arrete, Arrete, Constitution, Flux, Frontiere, Protocole, Voie};
use caringnanny::{CapteurAttendu, HealthStatus, ObservatoireVigies};
use ed25519_dalek::SigningKey;
use miyudouane::nft::auditer;
use miyudouane::{canal_evenements, InstanceDouane, MetaFlux, MoteurIps, NatureEvenement, PlanReseau, Verdict};
use miyufrontiere::{AttributionVoies, CdrPortail, CoupureFrontiere, SourceDouane};
use miyuportail::msce::sceller;
use miyuportail::sas::{decoder_reponse, traiter_requete, ResultatTrame};
use miyuportail::{
    Champ, ChampTraite, ClasseExposition, ContratVoies, Declaration, DecodeurIsole, ErreurPortail, ErreurSas, EtatVoie,
    IdService, NatureChamp, NatureDocument, PortailInterne, RegistreMemoire, RequeteSas, Sens, Traite,
    TypeDeclaration, TypeMime, Valeur,
};
use miyusceau::{AncresDeConfiance, CleSignature, NiveauConfiance};
use miyuvigie::{
    CapteurId, Categorie, ConfigVigie, ControleSortie, ErreurSource, EvenementFlux, JeuRegles, PlanAdressage,
    Severite, SourceEvenements, Vigie, VoieId, Zone,
};
use worrysentinel::{Action, EtatVoie as EtatSentinelle, Motif, NatureAudit, SentinelleVoies};

const T0: u64 = 1_800_000_000;
/// Voie A et voie B (une voie par service).
const A: &str = "jayrdv";
const B: &str = "jaymail";
const TYPE_ENTRANT: u64 = 10;

// ------------------------------------------------------------------ Douane

fn plan() -> PlanReseau {
    PlanReseau::reference(&[
        (borderguard::Zone::Cog, A),
        (borderguard::Zone::Franche, A),
        (borderguard::Zone::Cog, B),
        (borderguard::Zone::Franche, B),
    ])
    .unwrap()
}

fn flux(voie: &str, port: u16) -> Flux {
    Flux {
        voie: voie.into(),
        source: borderguard::Zone::Cog,
        destination: borderguard::Zone::Franche,
        protocole: Protocole::Tcp,
        port,
        pairs: vec![],
        debit_max_kio_s: 512,
        taille_max_paquet: 1400,
    }
}

/// Arrêté du portail interne, scellé par une ancre souveraine puis vérifié.
fn arrete_verifie() -> borderguard::ArreteVerifie {
    let a = Arrete {
        frontiere: Frontiere::PortailInterne,
        numero: 1,
        emis_le: T0,
        pas_avant: T0,
        pas_apres: T0 + 30 * 86_400,
        emetteur: "borderguard".into(),
        motif: "lot 3 bout en bout".into(),
        voies: vec![Voie { id: A.into(), service: A.into() }, Voie { id: B.into(), service: B.into() }],
        flux: vec![flux(A, 7443), flux(B, 7444)],
    };
    let cle = CleSignature::depuis_graine(&[4u8; 32]);
    let octets = sceller_arrete(&a, &cle, &Constitution::reference()).unwrap();
    let mut ancres = AncresDeConfiance::new();
    ancres.ajouter(&cle.cle_publique(), NiveauConfiance::Souverain).unwrap();
    verifier_arrete(&octets, &ancres, &Constitution::reference(), T0 + 1).unwrap()
}

fn ip(s: &str) -> IpAddr {
    s.parse().unwrap()
}

fn meta(t: u64, src: IpAddr, dst: IpAddr, dport: u16) -> MetaFlux {
    MetaFlux {
        horodatage_secs: t,
        source: src,
        destination: dst,
        protocole: Protocole::Tcp,
        port_source: 40_000,
        port_destination: dport,
        taille: 120,
        nouvelle_connexion: true,
    }
}

// ------------------------------------------------------------------ Portail

/// Décodeur « en processus » : même cœur que le sas `portail-sas`, sans
/// confinement (le binaire n'est lançable que depuis les tests de miyuportail).
struct SasEnProcessus;

impl DecodeurIsole for SasEnProcessus {
    fn decoder_lot(&self, requete: &RequeteSas) -> Result<Vec<ResultatTrame>, ErreurSas> {
        let reponse = traiter_requete(&requete.encoder().map_err(|_| ErreurSas::Reponse)?)
            .map_err(|_| ErreurSas::FinAnormale)?;
        decoder_reponse(&reponse, requete.trames.len())
    }
}

fn traite() -> Traite {
    Traite {
        version: 1,
        types: vec![TypeDeclaration {
            code: TYPE_ENTRANT,
            sens: Sens::Entrante,
            champs: vec![
                ChampTraite { tag: 1, nature: NatureChamp::Texte { max: 128 }, obligatoire: true },
                ChampTraite {
                    tag: 2,
                    nature: NatureChamp::Document {
                        max: 1024,
                        nature: NatureDocument::Document,
                        mime: Some(TypeMime::new("text/plain").unwrap()),
                    },
                    obligatoire: false,
                },
            ],
        }],
    }
}

fn service(nom: &str) -> IdService {
    IdService::new(nom).unwrap()
}

struct Portail {
    portail: PortailInterne,
    facade: SigningKey,
    auteur: SigningKey,
}

impl Portail {
    fn deux_voies() -> Self {
        let p = Self {
            portail: PortailInterne::new(
                Box::new(SasEnProcessus),
                Box::new(CdrPortail::default()),
                Arc::new(RegistreMemoire::new()),
            ),
            facade: SigningKey::from_bytes(&[7; 32]),
            auteur: SigningKey::from_bytes(&[9; 32]),
        };
        for nom in [A, B] {
            let s = service(nom);
            p.portail
                .declarer(ContratVoies {
                    service: s.clone(),
                    classe: ClasseExposition::H2,
                    traite: traite(),
                    taille_max_trame: 8192,
                    debit_max_par_minute: 1000,
                    quota_file: 64,
                })
                .unwrap();
            p.portail.activer(&s).unwrap();
            p.portail
                .ouvrir_session(&s, p.facade.verifying_key().to_bytes(), vec![p.auteur.verifying_key().to_bytes()])
                .unwrap();
        }
        p
    }

    fn trame(&self, s: &IdService, sequence: u64) -> Vec<u8> {
        let d = Declaration {
            service: s.clone(),
            sens: Sens::Entrante,
            version_traite: 1,
            type_code: TYPE_ENTRANT,
            sequence,
            emise_le: T0,
            champs: vec![
                Champ { tag: 1, valeur: Valeur::Texte("rendez-vous".into()) },
                Champ { tag: 2, valeur: Valeur::Document("note\u{202e} jointe\r\n".as_bytes().to_vec()) },
            ],
            signataires: Vec::new(),
        };
        sceller(&d, &[&self.facade, &self.auteur]).unwrap()
    }

    /// Dépose puis relève une trame : la voie fonctionne de bout en bout (CDR compris).
    fn depot_admis(&self, nom: &str, sequence: u64) {
        let s = service(nom);
        self.portail.deposer(&s, self.trame(&s, sequence)).unwrap();
        let r = self.portail.relever(&s, 8).unwrap();
        assert_eq!(r.len(), 1);
        let d = &r[0].as_ref().expect("déclaration admise").declaration;
        // Document désarmé par miyucdr (texte reconstruit, contrôle bidi retiré).
        assert_eq!(d.champs[1].valeur, Valeur::Document(b"note jointe\n".to_vec()));
    }
}

// ------------------------------------------------------------------ chaîne

fn voie(s: &str) -> VoieId {
    VoieId::new(s).unwrap()
}

fn vigie(capteur: &str, voies: &[&str]) -> Vigie {
    Vigie::new(ConfigVigie {
        capteur: CapteurId::new(capteur).unwrap(),
        zone: Zone::BorderGuard,
        voies_couvertes: voies.iter().map(|v| voie(v)).collect(),
        plan: PlanAdressage::indicatif(),
        regles: JeuRegles::vide(),
        controle_sortie: ControleSortie::new(),
        capacite_dedup: 1024,
        fenetre_rejeu_ms: 60_000,
    })
}

fn attendu(capteur: &str, voies: &[&str]) -> CapteurAttendu {
    CapteurAttendu {
        capteur: CapteurId::new(capteur).unwrap(),
        voies: voies.iter().map(|v| voie(v)).collect(),
        delai_pouls_max_ms: 5_000,
    }
}

fn executant(p: &Portail, ips: &Arc<Mutex<MoteurIps>>, t: u64) -> CoupureFrontiere<miyuportail::PoigneeCoupure> {
    let mut c = CoupureFrontiere::new(p.portail.poignee_coupure()).avec_horloge(Box::new(move || t));
    for nom in [A, B] {
        c.declarer(voie(nom), service(nom)).unwrap();
    }
    c.ajouter_ips(Arc::clone(ips));
    c
}

fn voies_acceptees(ruleset: &str) -> BTreeSet<String> {
    ruleset
        .lines()
        .filter(|l| l.contains("ct state new") && l.contains(" accept "))
        .filter_map(|l| l.split("comment \"voie=").nth(1)?.split(' ').next().map(str::to_owned))
        .collect()
}

/// Critère de sortie du lot 3 : flux non déclaré sortant de la zone COG sur la
/// voie A ⇒ IPS bloque + événement critique ⇒ signal des Vigies ⇒ WorrySentinel
/// coupe A ⇒ Portail A refuse les dépôts, B fonctionne, règles Douane de A fermées.
#[test]
fn sortie_non_declaree_coupe_la_seule_voie_a() {
    let t = T0 + 100;
    let ms = t * 1000;
    let plan = plan();
    let mut douane = InstanceDouane::nouvelle(Frontiere::PortailInterne, plan.clone(), 0).unwrap();
    douane.charger(arrete_verifie(), T0 + 1).unwrap();
    let (tx, rx) = canal_evenements();
    let ips = Arc::new(Mutex::new(douane.moteur_ips(tx).unwrap()));

    let capteur = "douane-portail-interne";
    let mut source = SourceDouane::new(CapteurId::new(capteur).unwrap(), rx)
        .avec_attribution(AttributionVoies::nouvelle(&plan, douane.regles()));
    let mut vigie = vigie(capteur, &[A, B]);
    let mut nanny = ObservatoireVigies::default();
    nanny.enregistrer_capteur(attendu(capteur, &[A, B]));
    let mut sentinelle = SentinelleVoies::default();
    let portail = Portail::deux_voies();
    let mut coupure = executant(&portail, &ips, t);

    // Situation nominale : les deux voies passent au Portail et à la Douane.
    portail.depot_admis(A, 1);
    portail.depot_admis(B, 1);
    let cog_a = plan.sandbox(borderguard::Zone::Cog, A).unwrap().adresse;
    let cog_b = plan.sandbox(borderguard::Zone::Cog, B).unwrap().adresse;
    let portail_cog = ip("10.77.0.253");
    {
        let mut m = ips.lock().unwrap();
        assert!(matches!(m.evaluer(&meta(t, cog_a, portail_cog, 7443)), Verdict::Autoriser { .. }));
        assert!(matches!(m.evaluer(&meta(t, cog_b, portail_cog, 7444)), Verdict::Autoriser { .. }));
    }
    let lot = vigie.relever(&mut source, ms);
    assert!(lot.signaux.is_empty() && lot.pouls.is_some());
    nanny.ingerer(lot);
    assert!(sentinelle.evaluer(&nanny.transmettre(ms)).is_empty());

    // Attaque : la sandbox COG de A ouvre une connexion non déclarée vers l'extérieur.
    let v = ips.lock().unwrap().evaluer(&meta(t, cog_a, ip("93.184.216.34"), 443));
    assert_eq!(v, Verdict::Bloquer(NatureEvenement::SortieZoneCog), "l'IPS bloque");

    // Vigies : événement critique de la Douane → signal EX-12 attribué à A.
    let lot = vigie.relever(&mut source, ms + 1);
    assert_eq!(lot.signaux.len(), 1);
    let s = &lot.signaux[0];
    assert_eq!((s.severite, s.categorie), (Severite::Critique, Categorie::SortieZoneCog));
    assert_eq!(s.voie, Some(voie(A)));
    assert_eq!((s.zone_src, s.zone_dst), (Zone::Cog, Zone::Exterieur));
    assert_eq!(nanny.ingerer(lot).nouveaux, 1);

    // WorrySentinel : couper A, et seulement A.
    let decisions = sentinelle.evaluer(&nanny.transmettre(ms + 1));
    let coupees: Vec<_> = decisions
        .iter()
        .filter_map(|d| match &d.action {
            Action::CouperVoie(v) => Some((v.clone(), d.motif)),
            _ => None,
        })
        .collect();
    assert_eq!(coupees, vec![(voie(A), Motif::IncidentCritique)]);
    let resultats = sentinelle.appliquer_coupures(&decisions, &mut coupure);
    assert!(resultats.iter().all(|(_, r)| r.is_ok()), "{resultats:?}");
    assert!(sentinelle.coupures_non_confirmees().is_empty());
    assert_eq!(sentinelle.etat_voie(&voie(A)), Some(EtatSentinelle::Coupee));
    assert_eq!(sentinelle.etat_voie(&voie(B)), Some(EtatSentinelle::Ouverte));

    // Portail : A refuse les dépôts, B fonctionne.
    let (sa, sb) = (service(A), service(B));
    assert!(matches!(portail.portail.etat(&sa).unwrap(), EtatVoie::Coupee(_)));
    assert_eq!(portail.portail.deposer(&sa, portail.trame(&sa, 2)), Err(ErreurPortail::VoieCoupee));
    assert_eq!(portail.portail.etat(&sb).unwrap(), EtatVoie::Active);
    portail.depot_admis(B, 2);

    // Douane : flux de A bloqués, B passe ; règles nftables de A fermées.
    let mut m = ips.lock().unwrap();
    assert_eq!(m.evaluer(&meta(t, cog_a, portail_cog, 7443)), Verdict::Bloquer(NatureEvenement::VoieCoupee));
    assert!(matches!(m.evaluer(&meta(t, cog_b, portail_cog, 7444)), Verdict::Autoriser { .. }));
    let ruleset = douane.ruleset(t, m.voies_coupees());
    assert_eq!(voies_acceptees(&ruleset), BTreeSet::from([B.to_string()]));
    assert!(auditer(&ruleset, &BTreeSet::from([A.to_string(), B.to_string()])).is_empty());
    drop(m);

    // L'événement « voie coupée » de l'IPS n'est pas un flux : aucun nouveau signal.
    let lot = vigie.relever(&mut source, ms + 2);
    assert!(lot.signaux.is_empty());
    assert!(source.evenements_etat() >= 1);
}

/// Source d'essai : relève vide (capteur vivant, rien à signaler).
struct SourceMuette(CapteurId);

impl SourceEvenements for SourceMuette {
    fn capteur(&self) -> &CapteurId {
        &self.0
    }

    fn relever(&mut self) -> Result<Vec<EvenementFlux>, ErreurSource> {
        Ok(Vec::new())
    }
}

/// Capteur de la voie A périmé (plus aucun pouls) ⇒ voie A aveugle ⇒ coupée
/// (politique du propriétaire, A-004) ; la voie B, couverte par un capteur
/// vivant, continue.
#[test]
fn capteur_perime_voie_aveugle_coupee() {
    let t = T0 + 100;
    let ms = t * 1000;
    let plan = plan();
    let mut douane = InstanceDouane::nouvelle(Frontiere::PortailInterne, plan.clone(), 0).unwrap();
    douane.charger(arrete_verifie(), T0 + 1).unwrap();
    let (tx, rx) = canal_evenements();
    let ips = Arc::new(Mutex::new(douane.moteur_ips(tx).unwrap()));

    // Voie A : capteur alimenté par la Douane ; voie B : capteur distinct.
    let mut source_a = SourceDouane::new(CapteurId::new("vigie-a").unwrap(), rx);
    let mut source_b = SourceMuette(CapteurId::new("vigie-b").unwrap());
    let (mut vigie_a, mut vigie_b) = (vigie("vigie-a", &[A]), vigie("vigie-b", &[B]));
    let mut nanny = ObservatoireVigies::default();
    nanny.enregistrer_capteur(attendu("vigie-a", &[A]));
    nanny.enregistrer_capteur(attendu("vigie-b", &[B]));
    let mut sentinelle = SentinelleVoies::default();
    let portail = Portail::deux_voies();
    let mut coupure = executant(&portail, &ips, t);

    nanny.ingerer(vigie_a.relever(&mut source_a, ms));
    nanny.ingerer(vigie_b.relever(&mut source_b, ms));
    assert!(sentinelle.evaluer(&nanny.transmettre(ms)).is_empty());

    // 10 s plus tard : seul le capteur B a rendu compte (A muet / périmé).
    let plus_tard = ms + 10_000;
    nanny.ingerer(vigie_b.relever(&mut source_b, plus_tard));
    let rapport = nanny.transmettre(plus_tard);
    assert_eq!(rapport.sante_voies.get(&voie(A)), Some(&HealthStatus::Inconnu));
    assert_eq!(rapport.sante_voies.get(&voie(B)), Some(&HealthStatus::Healthy));
    let decisions = sentinelle.evaluer(&rapport);
    let coupees: Vec<_> = decisions
        .iter()
        .filter_map(|d| match &d.action {
            Action::CouperVoie(v) => Some((v.clone(), d.motif)),
            _ => None,
        })
        .collect();
    assert_eq!(coupees, vec![(voie(A), Motif::VoieAveugle)]);
    assert!(sentinelle.appliquer_coupures(&decisions, &mut coupure).iter().all(|(_, r)| r.is_ok()));

    assert!(matches!(portail.portail.etat(&service(A)).unwrap(), EtatVoie::Coupee(_)));
    portail.depot_admis(B, 1);
    let cog_a = plan.sandbox(borderguard::Zone::Cog, A).unwrap().adresse;
    assert_eq!(
        ips.lock().unwrap().evaluer(&meta(t, cog_a, ip("10.77.0.253"), 7443)),
        Verdict::Bloquer(NatureEvenement::VoieCoupee)
    );
    // Voie aveugle : défaillance de capteur, pas incident — état global inchangé.
    assert_eq!(sentinelle.etat_global(), worrysentinel::DegradationState::Normal);

    // Douane arrêtée (canal fermé) : le capteur A passe en échec, jamais sain.
    drop(ips);
    drop(coupure);
    // Les événements reçus avant la fermeture sont d'abord rendus…
    let lot = vigie_a.relever(&mut source_a, plus_tard + 1);
    nanny.ingerer(lot);
    // … puis la source est en échec : aucun pouls.
    let lot = vigie_a.relever(&mut source_a, plus_tard + 1);
    assert!(lot.pouls.is_none());
    assert_eq!(lot.etat, miyuvigie::EtatCapteur::EnEchec);
    nanny.ingerer(lot);
    assert_eq!(nanny.sante_voie(&voie(A), plus_tard + 1), HealthStatus::Unhealthy);
}

/// Voie absente de la table voie ↔ service : erreur, WorrySentinel trace
/// `CoupureNonConfirmee` et garde l'ordre à rejouer (fail-closed).
#[test]
fn voie_inconnue_coupure_non_confirmee() {
    let portail = Portail::deux_voies();
    let mut coupure = CoupureFrontiere::new(portail.portail.poignee_coupure());
    coupure.declarer(voie(B), service(B)).unwrap();
    assert!(coupure.declarer(voie(B), service(A)).is_err(), "une voie, un service");

    let mut vigie = vigie("v", &[A]);
    let mut nanny = ObservatoireVigies::default();
    nanny.enregistrer_capteur(attendu("v", &[A]));
    let mut sentinelle = SentinelleVoies::default();
    let evt = EvenementFlux {
        id: miyuvigie::EvenementId::new("e1").unwrap(),
        horodatage_ms: 1_000,
        zone_src: Zone::Cog,
        zone_dst: Zone::Exterieur,
        voie: Some(voie(A)),
        verdict: miyuvigie::Verdict::Bloque,
        octets: 60,
        ip_src: None,
        ip_dst: None,
        port_dst: Some(443),
        protocole: None,
    };
    nanny.ingerer(vigie.analyser_flux(vec![evt], 1_000));
    let decisions = sentinelle.evaluer(&nanny.transmettre(1_000));
    let r = sentinelle.appliquer_coupures(&decisions, &mut coupure);
    assert_eq!(r.len(), 1);
    assert!(r[0].1.is_err());
    assert_eq!(sentinelle.coupures_non_confirmees(), vec![voie(A)]);
    assert!(sentinelle
        .journal()
        .iter()
        .any(|e| matches!(&e.nature, NatureAudit::CoupureNonConfirmee { voie: v, .. } if *v == voie(A))));
    // Rien n'a été coupé ailleurs : la voie A du Portail n'est pas déclarée à l'executant.
    assert_eq!(portail.portail.etat(&service(A)).unwrap(), EtatVoie::Active);
    assert_eq!(portail.portail.etat(&service(B)).unwrap(), EtatVoie::Active);
}

/// Attribution : la voie d'un flux non déclaré est celle de la sandbox source.
#[test]
fn attribution_par_sandbox_source() {
    let plan = plan();
    let mut douane = InstanceDouane::nouvelle(Frontiere::PortailInterne, plan.clone(), 0).unwrap();
    douane.charger(arrete_verifie(), T0 + 1).unwrap();
    let attribution = AttributionVoies::nouvelle(&plan, douane.regles());
    let cog_a = plan.sandbox(borderguard::Zone::Cog, A).unwrap().adresse;
    let franche_b = plan.sandbox(borderguard::Zone::Franche, B).unwrap().adresse;
    assert_eq!(attribution.voie(cog_a), Some(&voie(A)));
    assert_eq!(attribution.voie(franche_b), Some(&voie(B)));
    assert_eq!(attribution.voie(ip("10.77.0.253")), None);
}
