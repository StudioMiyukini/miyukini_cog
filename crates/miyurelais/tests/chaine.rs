//! @id     miyurelais_test_chaine
//! @do     verifier_relais_nflog_vers_sentinelle
//! @role   test
//! @layer  operateur
//! @human  Relais NFLOG → Vigie → CaringNanny → WorrySentinel sur trames netlink construites : NON-DECLARE coupe la voie émettrice ; capteur muet, perte ou débordement ⇒ voie aveugle coupée ; trame refusée ⇒ capteur dégradé.

#![allow(clippy::unwrap_used, clippy::assert_is_empty)]

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use caringnanny::{CapteurAttendu, HealthStatus, ObservatoireVigies};
use miyurelais::netlink::{
    NFULA_IFINDEX_INDEV, NFULA_IFINDEX_OUTDEV, NFULA_IFINDEX_PHYSINDEV, NFULA_IFINDEX_PHYSOUTDEV, NFULA_PACKET_HDR,
    NFULA_PAYLOAD, NFULA_SEQ,
};
use miyurelais::{ConfigRelais, ErreurTransport, Relais, TrameEssai, TransportEssai};
use miyuvigie::{
    CapteurId, Categorie, ConfigVigie, ControleSortie, EtatCapteur, JeuRegles, PlanAdressage, Severite, Vigie,
    VoieId, Zone,
};
use worrysentinel::{Action, Motif, SentinelleVoies};

const VOIE: &str = "jayoffice";
const T0_US: u64 = 1_800_000_000_000_000;

fn voie() -> VoieId {
    VoieId::new(VOIE).unwrap()
}

fn capteur() -> CapteurId {
    CapteurId::new("vigie-pont-franche").unwrap()
}

fn ipv4_tcp_syn(src: [u8; 4], dst: [u8; 4], dport: u16) -> Vec<u8> {
    let mut p = vec![0x45, 0, 0, 60, 0, 1, 0x40, 0, 64, 6, 0, 0];
    p.extend_from_slice(&src);
    p.extend_from_slice(&dst);
    p.extend_from_slice(&40_000u16.to_be_bytes());
    p.extend_from_slice(&dport.to_be_bytes());
    p.extend_from_slice(&[0, 0, 0, 1, 0, 0, 0, 0, 0xa0, 0x02, 0xff, 0xff]);
    p
}

/// Trame NFLOG du pont de la zone Franche (famille pont, groupe 111), telle
/// que le noyau l'émet : `_INDEV` / `_OUTDEV` = le pont `br0` (2), ports
/// `sv0` (3) / `sv1` (4) en `_PHYSINDEV` / `_PHYSOUTDEV`.
fn trame(prefixe: &str, seq: u32) -> Vec<u8> {
    TrameEssai::new()
        .attribut(NFULA_PACKET_HDR, &[0x08, 0x00, 2, 0])
        .be32(NFULA_IFINDEX_INDEV, 2)
        .be32(NFULA_IFINDEX_PHYSINDEV, 3)
        .be32(NFULA_IFINDEX_OUTDEV, 2)
        .be32(NFULA_IFINDEX_PHYSOUTDEV, 4)
        .prefixe(prefixe)
        .be32(NFULA_SEQ, seq)
        .attribut(NFULA_PAYLOAD, &ipv4_tcp_syn([10, 77, 1, 10], [10, 77, 1, 20], 9999))
        .message(7, 111)
}

const NON_DECLARE: &str = "MDOU|zone-franche|transit|NON-DECLARE|";

struct Banc {
    relais: Relais<TransportEssai>,
    horloge: Arc<AtomicU64>,
    vigie: Vigie,
    nanny: ObservatoireVigies,
    sentinelle: SentinelleVoies,
}

impl Banc {
    fn new(file: Vec<Result<Vec<u8>, ErreurTransport>>) -> Self {
        let transport = TransportEssai {
            file: file.into_iter().collect(),
            noms: BTreeMap::from([(2, "br0".to_owned()), (3, "sv0".to_owned()), (4, "sv1".to_owned())]),
        };
        let horloge = Arc::new(AtomicU64::new(T0_US));
        let h = Arc::clone(&horloge);
        let relais = Relais::new(
            ConfigRelais { capteur: capteur(), hote: "essai".into(), groupes: vec![111], delai_muet_ms: 5_000 },
            transport,
        )
        .unwrap()
        .avec_horloge(Box::new(move || h.load(Ordering::SeqCst)));
        let mut vigie = Vigie::new(ConfigVigie {
            capteur: capteur(),
            zone: Zone::Franche,
            voies_couvertes: vec![voie()],
            plan: PlanAdressage::indicatif(),
            regles: JeuRegles::vide(),
            controle_sortie: ControleSortie::new(),
            capacite_dedup: 1024,
            fenetre_rejeu_ms: 60_000,
        });
        vigie.declarer_interface_voie("sv0", voie());
        let mut nanny = ObservatoireVigies::default();
        nanny.enregistrer_capteur(CapteurAttendu { capteur: capteur(), voies: vec![voie()], delai_pouls_max_ms: 5_000 });
        Self { relais, horloge, vigie, nanny, sentinelle: SentinelleVoies::default() }
    }

    fn relais_alimenter(&mut self, file: Vec<Result<Vec<u8>, ErreurTransport>>) {
        self.relais.transport_mut().file.extend(file);
    }

    fn maintenant_ms(&self) -> u64 {
        self.horloge.load(Ordering::SeqCst) / 1000
    }

    fn avancer_ms(&self, ms: u64) {
        self.horloge.fetch_add(ms * 1000, Ordering::SeqCst);
    }

    /// Relève, ingère, évalue : coupures décidées.
    fn tour(&mut self) -> (miyuvigie::LotVigie, Vec<(VoieId, Motif)>) {
        let t = self.maintenant_ms();
        let lot = self.vigie.relever_journal(&mut self.relais, t);
        self.nanny.ingerer(lot.clone());
        let rapport = self.nanny.transmettre(t);
        let coupures = self
            .sentinelle
            .evaluer(&rapport)
            .iter()
            .filter_map(|d| match &d.action {
                Action::CouperVoie(v) => Some((v.clone(), d.motif)),
                _ => None,
            })
            .collect();
        (lot, coupures)
    }
}

#[test]
fn non_declare_coupe_la_voie_emettrice() {
    let mut b = Banc::new(vec![Ok(trame(NON_DECLARE, 0))]);
    let (lot, coupures) = b.tour();
    assert_eq!(lot.etat, EtatCapteur::Nominal);
    assert_eq!(lot.compteurs.acceptees, 1);
    let s = &lot.signaux[0];
    assert_eq!(s.regle.as_str(), miyuvigie::REGLE_A005_NON_DECLARE);
    assert_eq!(s.severite, Severite::Critique);
    assert_eq!(s.categorie, Categorie::FluxHorsContrat);
    assert_eq!(s.voie, Some(voie()));
    assert_eq!(coupures, vec![(voie(), Motif::IncidentCritique)]);
}

#[test]
fn temoin_de_vie_sans_ligne_et_voie_saine() {
    let mut b = Banc::new(vec![Ok(trame("MDOU-TEMOIN|zone-franche|", 0))]);
    b.relais.activer_voie();
    let (lot, coupures) = b.tour();
    assert_eq!(lot.etat, EtatCapteur::Nominal);
    assert!(lot.signaux.is_empty());
    assert!(coupures.is_empty());
    assert_eq!(b.relais.statistiques().temoins, 1);
    assert!(lot.pouls.is_some());
}

#[test]
fn capteur_muet_voie_aveugle_coupee() {
    let mut b = Banc::new(vec![]);
    b.relais.activer_voie();
    let (lot, coupures) = b.tour();
    assert_eq!(lot.etat, EtatCapteur::Nominal, "silence encore court");
    assert!(coupures.is_empty());
    b.avancer_ms(5_000);
    let (lot, coupures) = b.tour();
    assert_eq!(lot.etat, EtatCapteur::EnEchec);
    assert!(lot.pouls.is_none());
    assert!(lot.signaux[0].detail.contains("capteur muet"), "{}", lot.signaux[0].detail);
    // Capteur en échec : santé de la voie au pire `Unhealthy` (jamais saine).
    assert_eq!(b.nanny.transmettre(b.maintenant_ms()).sante_voies.get(&voie()), Some(&HealthStatus::Unhealthy));
    assert_eq!(coupures, vec![(voie(), Motif::VoieAveugle)]);
}

#[test]
fn voie_inactive_silence_normal() {
    let mut b = Banc::new(vec![]);
    b.avancer_ms(60_000);
    let (lot, coupures) = b.tour();
    assert_eq!(lot.etat, EtatCapteur::Nominal);
    assert!(coupures.is_empty());
}

#[test]
fn trou_de_sequence_lignes_et_echec_dans_la_meme_releve() {
    // Séquences 0 puis 3 : deux paquets perdus.
    let mut b = Banc::new(vec![Ok(trame(NON_DECLARE, 0)), Ok(trame(NON_DECLARE, 3))]);
    let (lot, coupures) = b.tour();
    assert_eq!(lot.compteurs.acceptees, 2, "les lignes lues sont analysées");
    assert_eq!(b.relais.statistiques().pertes, 2);
    assert_eq!(lot.etat, EtatCapteur::EnEchec, "perte signalée dans la même relève (H1)");
    assert!(lot.pouls.is_none());
    assert!(lot.signaux.iter().any(|s| s.severite == Severite::Critique), "signal de la ligne gardé");
    assert!(lot.signaux.iter().any(|s| s.categorie == Categorie::CapteurDegrade && s.detail.contains("perdu")));
    assert!(coupures.contains(&(voie(), Motif::IncidentCritique)) || coupures.contains(&(voie(), Motif::VoieAveugle)));
}

/// H1 (non-régression) : sous flux continu, chaque relève rend des lignes ;
/// une panne ne doit jamais être reportée indéfiniment.
#[test]
fn panne_sous_flux_continu_jamais_reportee() {
    // Relève 1 : séquences 0, 1. Puis débordement + séquence 5 (trou) ; ensuite flux normal.
    let file = vec![
        Ok(trame(NON_DECLARE, 0)),
        Ok(trame(NON_DECLARE, 1)),
        Err(ErreurTransport::Debordement),
        Ok(trame(NON_DECLARE, 5)),
    ];
    let mut b = Banc::new(file);
    let (lot, _) = b.tour();
    assert_eq!(lot.etat, EtatCapteur::EnEchec);
    assert_eq!(lot.compteurs.acceptees, 3);
    assert!(lot.signaux.iter().any(|s| s.detail.contains("ENOBUFS")));
    // Relève suivante, flux normal : pas d'échec résiduel.
    b.relais_alimenter(vec![Ok(trame(NON_DECLARE, 6))]);
    let (lot, _) = b.tour();
    assert_eq!(lot.etat, EtatCapteur::Nominal, "aucune panne différée");
}

/// B2 : un datagramme d'un émetteur autre que le noyau est écarté et compté.
#[test]
fn emetteur_non_noyau_ecarte_et_compte() {
    let mut b = Banc::new(vec![Err(ErreurTransport::EmetteurNonNoyau(4242)), Ok(trame(NON_DECLARE, 0))]);
    let (lot, _) = b.tour();
    assert_eq!(lot.etat, EtatCapteur::Degrade);
    assert_eq!(lot.compteurs.acceptees, 1);
    assert_eq!(b.relais.statistiques().datagrammes_refuses, 1);
}

#[test]
fn debordement_socket_echec_immediat() {
    let mut b = Banc::new(vec![Err(ErreurTransport::Debordement)]);
    let (lot, coupures) = b.tour();
    assert_eq!(lot.etat, EtatCapteur::EnEchec);
    assert!(lot.signaux[0].detail.contains("ENOBUFS"));
    assert_eq!(coupures, vec![(voie(), Motif::VoieAveugle)]);
}

#[test]
fn trame_refusee_capteur_degrade() {
    let mut mauvaise = trame(NON_DECLARE, 0);
    mauvaise[17] = 9; // version nfgenmsg
    let mut b = Banc::new(vec![Ok(mauvaise)]);
    let (lot, _) = b.tour();
    assert_eq!(lot.etat, EtatCapteur::Degrade);
    assert_eq!(lot.compteurs.mal_formees, 1);
    assert_eq!(b.relais.statistiques().datagrammes_refuses, 1);
}

#[test]
fn horodatages_strictement_croissants_identifiants_distincts() {
    // Deux paquets identiques dans le même datagramme, même instant : deux événements.
    let mut d = trame(NON_DECLARE, 0);
    d.extend(trame(NON_DECLARE, 1));
    let mut b = Banc::new(vec![Ok(d)]);
    let (lot, _) = b.tour();
    assert_eq!(lot.compteurs.acceptees, 2);
    assert_eq!(lot.compteurs.doublons, 0);
}
