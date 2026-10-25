//! @id     miyurelais_exemple_essai_vm
//! @do     essayer_relais_nflog_reel
//! @role   test
//! @layer  operateur
//! @human  Essai réel (VM jetable, root) : socket NFLOG → relais → Vigie → CaringNanny → WorrySentinel ; affiche lignes, signaux et coupures.

//! `essai_vm --groupe <G> --voie <V> --interface <svN> [--delai-muet-s <N>]
//! [--voie-active] --duree-s <D>`
//!
//! Sortie (une ligne par fait, pour les assertions du script d'essai) :
//! `LIGNE <ligne canonique>`, `LOT etat=<…> acceptees=<n> mal_formees=<n>`,
//! `SIGNAL regle=<id> voie=<v|-> severite=<…> categorie=<…>`,
//! `COUPURE voie=<v> motif=<…>`, `STATS …`.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::time::{Duration, Instant};

use caringnanny::{CapteurAttendu, ObservatoireVigies};
use miyurelais::{ConfigRelais, ParametresInstance, Relais, SocketNflog};
use miyuvigie::{
    CapteurId, ConfigVigie, ControleSortie, ErreurSource, JeuRegles, PlanAdressage, ReleveJournal, SourceJournal, Vigie,
    VoieId, Zone,
};
use worrysentinel::{Action, SentinelleVoies};

/// Affiche les lignes relevées avant de les rendre à la Vigie.
struct Affichee<'a>(&'a mut Relais<SocketNflog>);

impl SourceJournal for Affichee<'_> {
    fn capteur(&self) -> &CapteurId {
        self.0.capteur()
    }

    fn relever_lignes(&mut self) -> Result<ReleveJournal, ErreurSource> {
        let r = self.0.relever_lignes();
        match &r {
            Ok(r) => r.lignes.iter().for_each(|l| println!("LIGNE {l}")),
            Err(e) => println!("ECHEC {}", e.0),
        }
        r
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let val = |cle: &str| args.iter().position(|a| a == cle).and_then(|i| args.get(i + 1)).cloned();
    let groupe: u16 = val("--groupe").expect("--groupe").parse().unwrap();
    let voie = VoieId::new(val("--voie").expect("--voie")).unwrap();
    let interface = val("--interface").expect("--interface");
    let delai_muet_s: u64 = val("--delai-muet-s").map_or(5, |v| v.parse().unwrap());
    let duree_s: u64 = val("--duree-s").expect("--duree-s").parse().unwrap();
    let active = args.iter().any(|a| a == "--voie-active");

    let capteur = CapteurId::new("vigie-relais").unwrap();
    let socket = match SocketNflog::ouvrir(&[groupe], ParametresInstance { seuil_lot: 1, ..Default::default() }) {
        Ok(s) => s,
        Err(e) => {
            println!("OUVERTURE-ECHEC {e}");
            std::process::exit(3);
        }
    };
    println!("PRET groupe={groupe}");
    let config = ConfigRelais {
        capteur: capteur.clone(),
        hote: "vm-essai".into(),
        groupes: vec![groupe],
        delai_muet_ms: delai_muet_s * 1000,
    };
    let mut relais = Relais::new(config, socket).unwrap().avec_meta_flux();
    if active {
        relais.activer_voie();
    }
    let mut vigie = Vigie::new(ConfigVigie {
        capteur: capteur.clone(),
        zone: Zone::Franche,
        voies_couvertes: vec![voie.clone()],
        plan: PlanAdressage::indicatif(),
        regles: JeuRegles::vide(),
        controle_sortie: ControleSortie::new(),
        capacite_dedup: 4096,
        fenetre_rejeu_ms: 60_000,
    });
    vigie.declarer_interface_voie(interface, voie.clone());
    let mut nanny = ObservatoireVigies::default();
    nanny.enregistrer_capteur(CapteurAttendu { capteur, voies: vec![voie], delai_pouls_max_ms: 5_000 });
    let mut sentinelle = SentinelleVoies::default();

    let debut = Instant::now();
    while debut.elapsed() < Duration::from_secs(duree_s) {
        std::thread::sleep(Duration::from_millis(500));
        let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as u64;
        let lot = vigie.relever_journal(&mut Affichee(&mut relais), t);
        println!(
            "LOT etat={:?} acceptees={} mal_formees={} incoherentes={}",
            lot.etat, lot.compteurs.acceptees, lot.compteurs.mal_formees, lot.compteurs.incoherentes
        );
        for s in &lot.signaux {
            println!(
                "SIGNAL regle={} voie={} severite={:?} categorie={:?}",
                s.regle.as_str(),
                s.voie.as_ref().map_or("-", VoieId::as_str),
                s.severite,
                s.categorie
            );
        }
        for m in relais.prendre_meta_flux() {
            println!("META src={} dst={} dport={} nouvelle={} entree={:?}", m.source, m.destination, m.port_destination, m.nouvelle_connexion, m.interface_entree);
        }
        nanny.ingerer(lot);
        for d in sentinelle.evaluer(&nanny.transmettre(t)) {
            if let Action::CouperVoie(v) = &d.action {
                println!("COUPURE voie={} motif={:?}", v.as_str(), d.motif);
            }
        }
    }
    let s = relais.statistiques();
    println!(
        "STATS datagrammes={} paquets={} lignes={} temoins={} refuses={} pertes={} debordements={}",
        s.datagrammes, s.paquets, s.lignes, s.temoins, s.paquets_refuses, s.pertes, s.debordements
    );
}
