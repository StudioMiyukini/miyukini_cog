//! @id     miyudouane_tests_ips
//! @do     tester_ips_douane
//! @role   test
//! @layer  operateur
//! @human  miyudouane — IPS : blocage des flux non déclarés, événements critiques, débit, taille, bannissement, coupure de voie, fail-closed.

#![allow(clippy::assert_is_empty)]

mod commun;

use std::net::IpAddr;

use borderguard::{Frontiere, Protocole, Zone};
use commun::*;
use miyudouane::{
    canal_evenements, DecisionSentinelle, EvenementDouane, Gravite, InstanceDouane, MetaFlux, MoteurIps,
    NatureEvenement, Verdict,
};
use std::sync::mpsc::Receiver;

fn ip(s: &str) -> IpAddr {
    s.parse().unwrap()
}

fn meta(t: u64, src: &str, sport: u16, dst: &str, dport: u16, nouvelle: bool) -> MetaFlux {
    MetaFlux {
        horodatage_secs: t,
        source: ip(src),
        destination: ip(dst),
        protocole: Protocole::Tcp,
        port_source: sport,
        port_destination: dport,
        taille: 100,
        nouvelle_connexion: nouvelle,
    }
}

fn moteur(a: &borderguard::Arrete) -> (MoteurIps, Receiver<EvenementDouane>) {
    let mut i = InstanceDouane::nouvelle(a.frontiere, plan(), 0).unwrap();
    i.charger(verifie(a), T0 + 1).unwrap();
    let (tx, rx) = canal_evenements();
    (i.moteur_ips(tx).unwrap(), rx)
}

#[test]
fn flux_declare_autorise_et_suivi() {
    let (mut m, rx) = moteur(&arrete_portail(1));
    let t = T0 + 10;
    // jayoffice zone COG (10.77.0.10) → portail (10.77.0.253:7443).
    assert!(matches!(m.evaluer(&meta(t, "10.77.0.10", 40000, "10.77.0.253", 7443, true)), Verdict::Autoriser { .. }));
    // Réponse de la même connexion (sens inverse) : suivie.
    assert!(matches!(m.evaluer(&meta(t, "10.77.0.253", 7443, "10.77.0.10", 40000, false)), Verdict::Autoriser { .. }));
    // Second tronçon : portail → relais Franche.
    assert!(matches!(m.evaluer(&meta(t, "10.77.1.253", 50000, "10.77.1.10", 7443, true)), Verdict::Autoriser { .. }));
    assert!(rx.try_recv().is_err(), "aucun événement pour un flux déclaré");
}

#[test]
fn flux_non_declare_bloque_evenement_critique() {
    let (mut m, rx) = moteur(&arrete_portail(1));
    let t = T0 + 10;
    // Franche → zone COG : la zone COG tire, personne n'y entre.
    let v = m.evaluer(&meta(t, "10.77.1.10", 40000, "10.77.0.253", 7443, true));
    assert_eq!(v, Verdict::Bloquer(NatureEvenement::EntreeZoneCog));
    let ev = rx.try_recv().unwrap();
    assert_eq!(ev.gravite, Gravite::Critique);
    assert_eq!(ev.nature, NatureEvenement::EntreeZoneCog);
    assert_eq!(ev.frontiere, Frontiere::PortailInterne);
    assert_eq!(ev.numero_arrete, 1);
    assert_eq!(ev.zones, Some((Zone::Franche, Zone::Cog)));
    assert!(ev.bloque);
    // Sortie de la zone COG vers un port non déclaré : incident critique.
    let v = m.evaluer(&meta(t, "10.77.0.10", 40001, "10.77.0.253", 22, true));
    assert_eq!(v, Verdict::Bloquer(NatureEvenement::SortieZoneCog));
    assert_eq!(rx.try_recv().unwrap().gravite, Gravite::Critique);
    // Service d'une autre sandbox sur la voie de jayoffice : non déclaré.
    let v = m.evaluer(&meta(t, "10.77.0.11", 40002, "10.77.0.253", 7443, true));
    assert_eq!(v, Verdict::Bloquer(NatureEvenement::SortieZoneCog));
    // Paquet hors connexion connue.
    let v = m.evaluer(&meta(t, "10.77.1.10", 7443, "10.77.1.253", 50001, false));
    assert_eq!(v, Verdict::Bloquer(NatureEvenement::HorsEtat));
    assert_eq!(rx.try_iter().count(), 2);
}

#[test]
fn externe_liste_blanche_et_bannissement() {
    let (mut m, rx) = moteur(&arrete_externe(1));
    let t = T0 + 10;
    assert!(matches!(m.evaluer(&meta(t, "203.0.113.9", 40000, "10.77.1.11", 443, true)), Verdict::Autoriser { .. }));
    // Hors liste blanche.
    for k in 0..5 {
        let v = m.evaluer(&meta(t, "192.0.2.66", 40000 + k, "10.77.1.11", 443, true));
        assert_eq!(v, Verdict::Bloquer(NatureEvenement::FluxNonDeclare));
    }
    let evs: Vec<_> = rx.try_iter().collect();
    assert_eq!(evs.iter().filter(|e| e.nature == NatureEvenement::FluxNonDeclare).count(), 5);
    assert!(evs.iter().any(|e| matches!(e.nature, NatureEvenement::Bannissement { .. })));
    // Banni : même un flux de la liste blanche d'une autre source reste permis, la source bannie non.
    assert_eq!(
        m.evaluer(&meta(t + 1, "192.0.2.66", 41000, "10.77.1.11", 443, true)),
        Verdict::Bloquer(NatureEvenement::SourceBannie)
    );
    assert!(matches!(m.evaluer(&meta(t + 1, "203.0.113.10", 40000, "10.77.1.11", 443, true)), Verdict::Autoriser { .. }));
    // Fin du bannissement.
    assert_eq!(
        m.evaluer(&meta(t + 2_000, "192.0.2.66", 41001, "10.77.1.11", 443, true)),
        Verdict::Bloquer(NatureEvenement::FluxNonDeclare)
    );
}

#[test]
fn taille_et_debit() {
    let (mut m, rx) = moteur(&arrete_portail(1));
    let t = T0 + 10;
    let mut gros = meta(t, "10.77.0.10", 40000, "10.77.0.253", 7443, true);
    gros.taille = 1401;
    assert_eq!(m.evaluer(&gros), Verdict::Bloquer(NatureEvenement::TailleDepassee));
    assert_eq!(rx.try_recv().unwrap().gravite, Gravite::Elevee);
    assert!(matches!(m.evaluer(&meta(t, "10.77.0.10", 40000, "10.77.0.253", 7443, true)), Verdict::Autoriser { .. }));
    // 512 Kio/s : ~5 000 paquets de 100 o dépassent le plafond dans la même seconde.
    let mut bloque = false;
    for _ in 0..6_000 {
        if m.evaluer(&meta(t, "10.77.0.10", 40000, "10.77.0.253", 7443, false))
            == Verdict::Bloquer(NatureEvenement::DebitDepasse)
        {
            bloque = true;
            break;
        }
    }
    assert!(bloque);
    // Seconde suivante : à nouveau permis.
    assert!(matches!(m.evaluer(&meta(t + 1, "10.77.0.10", 40000, "10.77.0.253", 7443, false)), Verdict::Autoriser { .. }));
}

#[test]
fn coupure_de_voie_sans_impact_sur_les_autres() {
    let (mut m, rx) = moteur(&arrete_portail(1));
    let t = T0 + 10;
    assert!(matches!(m.evaluer(&meta(t, "10.77.0.12", 40000, "10.77.0.253", 7080, true)), Verdict::Autoriser { .. }));
    m.appliquer(DecisionSentinelle::CouperVoie("jayoffice".into()), t);
    assert_eq!(rx.try_recv().unwrap().nature, NatureEvenement::VoieCoupee);
    assert_eq!(
        m.evaluer(&meta(t, "10.77.0.10", 40000, "10.77.0.253", 7443, true)),
        Verdict::Bloquer(NatureEvenement::VoieCoupee)
    );
    // La voie « site » continue, connexion en cours comprise.
    assert!(matches!(m.evaluer(&meta(t, "10.77.0.12", 40000, "10.77.0.253", 7080, false)), Verdict::Autoriser { .. }));
    m.appliquer(DecisionSentinelle::RetablirVoie("jayoffice".into()), t);
    assert!(matches!(m.evaluer(&meta(t, "10.77.0.10", 40000, "10.77.0.253", 7443, true)), Verdict::Autoriser { .. }));
    // Gel : tout bloqué.
    m.appliquer(DecisionSentinelle::Geler, t);
    assert_eq!(m.evaluer(&meta(t, "10.77.0.12", 40000, "10.77.0.253", 7080, true)), Verdict::Bloquer(NatureEvenement::Gel));
    m.appliquer(DecisionSentinelle::Degeler, t);
    assert!(matches!(m.evaluer(&meta(t, "10.77.0.12", 40000, "10.77.0.253", 7080, true)), Verdict::Autoriser { .. }));
}

#[test]
fn arrete_expire_tout_bloque() {
    let (mut m, rx) = moteur(&arrete_portail(1));
    let v = m.evaluer(&meta(T0 + 31 * 86_400, "10.77.0.10", 40000, "10.77.0.253", 7443, true));
    assert_eq!(v, Verdict::Bloquer(NatureEvenement::ArreteHorsVigueur));
    assert_eq!(rx.try_recv().unwrap().gravite, Gravite::Critique);
}

#[test]
fn vigies_absentes_douane_aveugle_tout_bloque() {
    let (mut m, rx) = moteur(&arrete_portail(1));
    drop(rx);
    let t = T0 + 10;
    // Premier blocage : l'émission échoue, la Douane passe aveugle.
    assert!(matches!(m.evaluer(&meta(t, "10.77.1.10", 1, "10.77.0.253", 7443, true)), Verdict::Bloquer(_)));
    assert!(m.est_aveugle());
    // Même un flux déclaré est désormais bloqué.
    assert_eq!(
        m.evaluer(&meta(t, "10.77.0.10", 40000, "10.77.0.253", 7443, true)),
        Verdict::Bloquer(NatureEvenement::CanalVigiesFerme)
    );
}

#[test]
fn source_interchangeable() {
    let (mut m, _rx) = moteur(&arrete_portail(1));
    let t = T0 + 10;
    let mut source = vec![
        meta(t, "10.77.0.10", 40000, "10.77.0.253", 7443, true),
        meta(t, "10.77.0.10", 40001, "10.77.0.253", 9999, true),
    ]
    .into_iter();
    let verdicts = m.traiter(&mut source);
    assert!(matches!(verdicts[0], Verdict::Autoriser { .. }));
    assert_eq!(verdicts[1], Verdict::Bloquer(NatureEvenement::SortieZoneCog));
}

/// Usurpation : des paquets hors connexion à source usurpée ne comptent pas
/// pour le bannissement automatique (seules les tentatives de connexion nouvelle).
#[test]
fn bannissement_ignore_les_paquets_hors_connexion() {
    let (mut m, rx) = moteur(&arrete_externe(1));
    let t = T0 + 10;
    for k in 0..20 {
        let v = m.evaluer(&meta(t, "192.0.2.66", 40000 + k, "10.77.1.11", 443, false));
        assert_eq!(v, Verdict::Bloquer(NatureEvenement::HorsEtat));
    }
    assert!(!rx.try_iter().any(|e| matches!(e.nature, NatureEvenement::Bannissement { .. })));
    // La source n'est pas bannie : ses tentatives restent jugées sur l'arrêté.
    assert_eq!(
        m.evaluer(&meta(t, "192.0.2.66", 41000, "10.77.1.11", 443, true)),
        Verdict::Bloquer(NatureEvenement::FluxNonDeclare)
    );
}

/// Un pair de la liste blanche de l'arrêté n'est jamais banni automatiquement,
/// même après de nombreux blocages (adresse usurpée vers un port non déclaré).
#[test]
fn pair_en_liste_blanche_jamais_banni_automatiquement() {
    let (mut m, rx) = moteur(&arrete_externe(1));
    let t = T0 + 10;
    assert!(m.est_pair_autorise(ip("203.0.113.9")));
    assert!(!m.est_pair_autorise(ip("192.0.2.66")));
    for k in 0..20 {
        let v = m.evaluer(&meta(t, "203.0.113.9", 40000 + k, "10.77.1.11", 22, true));
        assert_eq!(v, Verdict::Bloquer(NatureEvenement::FluxNonDeclare));
    }
    assert!(!rx.try_iter().any(|e| matches!(e.nature, NatureEvenement::Bannissement { .. })));
    // Sa voie déclarée reste ouverte.
    assert!(matches!(m.evaluer(&meta(t, "203.0.113.9", 42000, "10.77.1.11", 443, true)), Verdict::Autoriser { .. }));
}
