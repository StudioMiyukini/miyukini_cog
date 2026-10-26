//! @id     miyurelais_test_refus
//! @do     verifier_refus_et_pannes_du_relais
//! @role   test
//! @layer  operateur
//! @human  Relais NFLOG : configuration refusée, groupe inattendu, socket en erreur ou tronquée, émetteur non noyau, préfixe invalide, interface non résolue, file IPS bornée, horloge système, messages d'erreur distincts.

#![allow(clippy::unwrap_used)]

use std::collections::BTreeMap;

use borderguard::Protocole;
use miyurelais::netlink::{
    NFULA_IFINDEX_INDEV, NFULA_IFINDEX_OUTDEV, NFULA_MARK, NFULA_PACKET_HDR, NFULA_PAYLOAD, NFULA_SEQ,
};
use miyurelais::relais::{META_FLUX_MAX, TAMPON_RECEPTION};
use miyurelais::{ConfigRelais, ErreurTransport, EtatVeille, Relais, TrameEssai, TransportEssai};
use miyuvigie::{CapteurId, SourceJournal};

const T0_US: u64 = 1_800_000_000_000_000;
const PREFIXE: &str = "MDOU|zone-franche|transit|NON-DECLARE|";
const IPV4: u8 = 2;

fn config(hote: &str, groupes: Vec<u16>) -> ConfigRelais {
    ConfigRelais { capteur: CapteurId::new("relais-essai").unwrap(), hote: hote.into(), groupes, delai_muet_ms: 5_000 }
}

fn ipv4(proto: u8, src: [u8; 4], dst: [u8; 4], transport: &[u8]) -> Vec<u8> {
    let longueur = (20 + transport.len()) as u16;
    let mut p = vec![0x45, 0];
    p.extend_from_slice(&longueur.to_be_bytes());
    p.extend_from_slice(&[0, 1, 0x40, 0, 64, proto, 0, 0]);
    p.extend_from_slice(&src);
    p.extend_from_slice(&dst);
    p.extend_from_slice(transport);
    p
}

fn tcp(sport: u16, dport: u16, drapeaux: u8) -> Vec<u8> {
    let mut t = sport.to_be_bytes().to_vec();
    t.extend_from_slice(&dport.to_be_bytes());
    t.extend_from_slice(&[0, 0, 0, 1, 0, 0, 0, 0, 0x50, drapeaux, 0xff, 0xff, 0, 0, 0, 0]);
    t
}

fn udp(sport: u16, dport: u16) -> Vec<u8> {
    let mut t = sport.to_be_bytes().to_vec();
    t.extend_from_slice(&dport.to_be_bytes());
    t.extend_from_slice(&[0, 8, 0, 0]);
    t
}

/// Trame IPv4 (famille 2) du groupe `groupe`, interfaces d'index 3 / 4.
fn trame(groupe: u16, prefixe: &str, seq: u32, charge: &[u8]) -> Vec<u8> {
    TrameEssai::new()
        .attribut(NFULA_PACKET_HDR, &[0x08, 0x00, 1, 0])
        .be32(NFULA_IFINDEX_INDEV, 3)
        .be32(NFULA_IFINDEX_OUTDEV, 4)
        .prefixe(prefixe)
        .be32(NFULA_SEQ, seq)
        .attribut(NFULA_PAYLOAD, charge)
        .message(IPV4, groupe)
}

fn syn(seq: u32) -> Vec<u8> {
    trame(111, PREFIXE, seq, &ipv4(6, [10, 77, 1, 10], [10, 77, 1, 20], &tcp(40_000, 9999, 0x02)))
}

fn transport(file: Vec<Result<Vec<u8>, ErreurTransport>>) -> TransportEssai {
    TransportEssai {
        file: file.into_iter().collect(),
        noms: BTreeMap::from([(3, "sv0".to_owned()), (4, "sv1".to_owned())]),
    }
}

fn relais(file: Vec<Result<Vec<u8>, ErreurTransport>>) -> Relais<TransportEssai> {
    Relais::new(config("essai", vec![111]), transport(file)).unwrap().avec_horloge(Box::new(|| T0_US))
}

#[test]
fn configuration_refusee() {
    for hote in ["", "a b", "hôte", &"x".repeat(65)] {
        let e = Relais::new(config(hote, vec![111]), TransportEssai::default()).unwrap_err();
        assert_eq!(e.to_string(), "configuration du relais : nom d'hôte invalide", "hôte {hote:?}");
    }
    for groupes in [vec![], vec![0], vec![111, 0]] {
        let e = Relais::new(config("essai", groupes.clone()), TransportEssai::default()).unwrap_err();
        assert!(e.to_string().contains("jamais 0"), "groupes {groupes:?} : {e}");
    }
    assert!(Relais::new(config("h-1.zone_a", vec![1, 65_535]), TransportEssai::default()).is_ok());
}

#[test]
fn erreurs_de_transport_distinctes() {
    let messages = [
        ErreurTransport::Debordement.to_string(),
        ErreurTransport::Tronque(70_000).to_string(),
        ErreurTransport::EmetteurNonNoyau(4242).to_string(),
        ErreurTransport::Io("EBADF".into()).to_string(),
    ];
    assert!(messages[0].contains("ENOBUFS"));
    assert!(messages[1].contains("70000"));
    assert!(messages[2].contains("4242"));
    assert!(messages[3].contains("EBADF"));
    let distincts: std::collections::BTreeSet<_> = messages.iter().collect();
    assert_eq!(distincts.len(), messages.len());
}

/// Un paquet d'un groupe non configuré n'est jamais converti en ligne.
#[test]
fn groupe_inattendu_refuse() {
    let charge = ipv4(6, [10, 0, 0, 1], [10, 0, 0, 2], &tcp(1, 2, 0x02));
    let mut r = relais(vec![Ok(trame(222, PREFIXE, 0, &charge))]);
    let releve = r.relever().unwrap();
    assert!(releve.lignes.is_empty());
    assert_eq!(releve.mal_formees, 1);
    let s = r.statistiques();
    assert_eq!((s.paquets, s.paquets_refuses, s.lignes), (1, 1, 0));
}

/// Erreur d'entrée / sortie : la relève s'arrête (la file n'est plus lue) et échoue.
#[test]
fn erreur_socket_arrete_la_releve() {
    let mut r = relais(vec![Err(ErreurTransport::Io("EBADF".into())), Ok(syn(0))]);
    let e = r.relever().unwrap_err();
    assert!(e.0.contains("EBADF"), "{}", e.0);
    assert_eq!(r.transport().file.len(), 1, "aucune lecture après l'erreur");
    assert_eq!(r.statistiques().debordements, 0, "une erreur d'E/S n'est pas un débordement");
    // Relève suivante : le datagramme restant est lu normalement.
    assert_eq!(r.relever().unwrap().lignes.len(), 1);
}

/// Erreur de socket après des lignes lues : lignes rendues ET échec porté.
#[test]
fn erreur_socket_apres_lignes_portee_par_la_releve() {
    let mut r = relais(vec![Ok(syn(0)), Err(ErreurTransport::Io("EIO".into()))]);
    let releve = r.relever().unwrap();
    assert_eq!(releve.lignes.len(), 1);
    assert!(releve.echec.as_deref().is_some_and(|e| e.contains("EIO")));
}

/// Datagramme plus long que le tampon : tronqué ⇒ perte, échec, comptée débordement.
#[test]
fn datagramme_trop_long_tronque() {
    let mut r = relais(vec![Ok(vec![0; TAMPON_RECEPTION + 1]), Ok(syn(0))]);
    let releve = r.relever().unwrap();
    assert_eq!(releve.lignes.len(), 1, "la lecture continue après une troncature");
    let echec = releve.echec.unwrap();
    assert!(echec.contains(&format!("{}", TAMPON_RECEPTION + 1)), "{echec}");
    assert_eq!(r.statistiques().debordements, 1);
}

/// Datagramme d'un émetteur autre que le noyau : écarté, compté refusé, jamais une ligne.
#[test]
fn emetteur_non_noyau_ecarte() {
    let mut r = relais(vec![Err(ErreurTransport::EmetteurNonNoyau(1234)), Ok(syn(0))]);
    let releve = r.relever().unwrap();
    assert_eq!(releve.lignes.len(), 1);
    assert_eq!(releve.mal_formees, 1);
    assert!(releve.echec.is_none(), "l'injection dégrade le capteur sans le mettre en échec");
    assert_eq!(r.statistiques().datagrammes_refuses, 1);
}

/// Préfixe hors `0x21..=0x7e` (espace) ou absent : paquet refusé.
#[test]
fn prefixe_invalide_ou_absent_refuse() {
    let charge = ipv4(6, [10, 0, 0, 1], [10, 0, 0, 2], &tcp(1, 2, 0x02));
    let sans_prefixe = TrameEssai::new()
        .attribut(NFULA_PACKET_HDR, &[0x08, 0x00, 1, 0])
        .attribut(NFULA_PAYLOAD, &charge)
        .message(IPV4, 111);
    let mut r = relais(vec![Ok(trame(111, "MDOU|a b|", 0, &charge)), Ok(sans_prefixe)]);
    let releve = r.relever().unwrap();
    assert!(releve.lignes.is_empty());
    assert_eq!(releve.mal_formees, 2);
    assert_eq!(r.statistiques().paquets_refuses, 2);
}

/// Index non résolu, ou nom rendu invalide : `if<index>`, jamais le nom brut.
#[test]
fn interface_non_resolue_ou_invalide() {
    let mut t = transport(vec![Ok(syn(0))]);
    t.noms = BTreeMap::from([(3, "nom beaucoup trop long".to_owned())]);
    let mut r = Relais::new(config("essai", vec![111]), t).unwrap().avec_horloge(Box::new(|| T0_US));
    let ligne = r.relever().unwrap().lignes.remove(0);
    assert!(ligne.contains(" IN=if3 OUT=if4 "), "{ligne}");
}

/// File des métadonnées IPS bornée : la plus ancienne est écartée et comptée.
#[test]
fn meta_flux_bornees() {
    let n = META_FLUX_MAX + 3;
    let file = (0..n).map(|i| Ok(syn(i as u32))).collect();
    let mut r = relais(file).avec_meta_flux();
    let releve = r.relever().unwrap();
    assert_eq!(releve.lignes.len(), n);
    assert_eq!(r.statistiques().meta_perdues, 3);
    let meta = r.prendre_meta_flux();
    assert_eq!(meta.len(), META_FLUX_MAX);
    assert!(meta.iter().all(|m| m.protocole == Protocole::Tcp && m.nouvelle_connexion && m.port_destination == 9999));
    assert_eq!(meta[0].interface_entree.as_deref(), Some("sv0"));
    assert!(r.prendre_meta_flux().is_empty(), "file vidée");
}

/// Sans `avec_meta_flux`, aucune métadonnée n'est conservée ; UDP n'est
/// jamais une connexion nouvelle, ICMP ne donne aucune métadonnée.
#[test]
fn meta_flux_udp_icmp_et_desactivees() {
    let udp = trame(111, PREFIXE, 0, &ipv4(17, [10, 0, 0, 1], [10, 0, 0, 2], &udp(5353, 53)));
    let icmp = trame(111, PREFIXE, 1, &ipv4(1, [10, 0, 0, 1], [10, 0, 0, 2], &[8, 0, 0, 0, 0, 0, 0, 0]));
    let mut r = relais(vec![Ok(udp.clone()), Ok(icmp.clone())]).avec_meta_flux();
    let releve = r.relever().unwrap();
    assert_eq!(releve.lignes.len(), 2);
    assert!(releve.lignes[0].contains("PROTO=UDP SPT=5353 DPT=53"), "{}", releve.lignes[0]);
    assert!(releve.lignes[1].contains("PROTO=ICMP TYPE=8 CODE=0"), "{}", releve.lignes[1]);
    let meta = r.prendre_meta_flux();
    assert_eq!(meta.len(), 1);
    assert_eq!(meta[0].protocole, Protocole::Udp);
    assert!(!meta[0].nouvelle_connexion);

    let mut sans = relais(vec![Ok(udp), Ok(icmp)]);
    assert_eq!(sans.relever().unwrap().lignes.len(), 2);
    assert!(sans.prendre_meta_flux().is_empty());
}

/// Marque non nulle écrite en hexadécimal ; horodatage strictement croissant
/// même sous horloge figée.
#[test]
fn marque_et_horodatage_croissant() {
    let charge = ipv4(6, [10, 0, 0, 1], [10, 0, 0, 2], &tcp(1, 2, 0x12));
    let marque = TrameEssai::new()
        .attribut(NFULA_PACKET_HDR, &[0x08, 0x00, 1, 0])
        .be32(NFULA_MARK, 0x2a)
        .prefixe(PREFIXE)
        .attribut(NFULA_PAYLOAD, &charge)
        .message(IPV4, 111);
    let mut r = relais(vec![Ok(marque.clone()), Ok(marque)]);
    let lignes = r.relever().unwrap().lignes;
    assert!(lignes[0].ends_with(" SYN ACK MARK=0x2a"), "{}", lignes[0]);
    let t: Vec<&str> = lignes.iter().map(|l| l.split(' ').next().unwrap()).collect();
    assert_eq!(t, ["1800000000.000000", "1800000000.000001"]);
}

/// Voie désactivée : le silence redevient normal, la relève ne signale rien.
#[test]
fn voie_desactivee_plus_de_capteur_muet() {
    let horloge = std::sync::Arc::new(std::sync::atomic::AtomicU64::new(T0_US));
    let h = std::sync::Arc::clone(&horloge);
    let mut r = Relais::new(config("essai", vec![111]), TransportEssai::default())
        .unwrap()
        .avec_horloge(Box::new(move || h.load(std::sync::atomic::Ordering::SeqCst)));
    r.activer_voie();
    assert_eq!(r.etat_veille(), EtatVeille::Vivante);
    horloge.fetch_add(6_000_000, std::sync::atomic::Ordering::SeqCst);
    assert_eq!(r.etat_veille(), EtatVeille::Muette { silence_ms: 6_000 });
    assert!(r.relever_lignes().unwrap_err().0.contains("capteur muet"));
    r.desactiver_voie();
    assert_eq!(r.etat_veille(), EtatVeille::Inactive);
    assert!(r.relever().unwrap().echec.is_none());
    assert_eq!(r.capteur().as_str(), "relais-essai");
    assert!(format!("{r:?}").starts_with("Relais"));
}

/// Horloge système par défaut : horodatage proche de maintenant.
#[test]
fn horloge_systeme_par_defaut() {
    let mut r = Relais::new(config("essai", vec![111]), transport(vec![Ok(syn(0))])).unwrap();
    let ligne = r.relever().unwrap().lignes.remove(0);
    let secs: u64 = ligne.split('.').next().unwrap().parse().unwrap();
    let maintenant = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    assert!(secs.abs_diff(maintenant) <= 5, "{ligne}");
}
