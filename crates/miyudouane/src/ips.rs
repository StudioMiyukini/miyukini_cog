//! @id     miyudouane_ips
//! @do     bloquer_flux_hors_arrete
//! @role   securite
//! @layer  operateur
//! @human  miyudouane — IPS maison (salle blanche) : décide autoriser / bloquer depuis l'arrêté, émet des événements vers les Vigies.

//! # IPS de la Douane
//!
//! Moteur en ligne, écrit en salle blanche (aucun code Suricata / Snort, C-71).
//! Il consomme des **métadonnées de flux** ([`MetaFlux`]) venant d'une source
//! interchangeable ([`SourceFlux`] : NFQUEUE, NFLOG, conntrack, rejeu de test)
//! et applique **les mêmes règles compilées** que le pare-feu nftables.
//!
//! Décisions :
//! - connexion nouvelle **non déclarée** → bloquée, événement **critique**
//!   (`SortieZoneCog` / `EntreeZoneCog` si la zone COG est en jeu, sinon
//!   `FluxNonDeclare`) ;
//! - paquet d'une connexion inconnue → bloqué (`HorsEtat`, critique) ;
//! - taille ou débit au-delà de l'arrêté → bloqué (`TailleDepassee`,
//!   `DebitDepasse`, gravité élevée) ;
//! - arrêté hors de sa fenêtre de validité → tout bloqué (`ArreteHorsVigueur`) ;
//! - bannissement automatique d'une source International après
//!   [`SEUIL_BANNISSEMENT`] blocages en [`FENETRE_BANNISSEMENT_SECS`] ;
//! - décisions de WorrySentinel ([`DecisionSentinelle`]) : bannir, couper ou
//!   rétablir une voie (sans toucher aux autres, EX-49), geler / dégeler.
//!
//! ## Contrat avec les Vigies
//!
//! Chaque blocage et chaque changement d'état produit un [`EvenementDouane`]
//! envoyé sur un `std::sync::mpsc::Sender<EvenementDouane>` (voir
//! [`canal_evenements`]). **Fail-closed** : si le récepteur des Vigies a
//! disparu, le moteur passe « aveugle » et bloque tout (`CanalVigiesFerme`) :
//! une Douane qui ne peut plus rendre compte ne laisse plus rien passer.

use std::collections::{BTreeSet, HashMap};
use std::net::IpAddr;
use std::sync::mpsc::{channel, Receiver, Sender};

use borderguard::{ArreteVerifie, Frontiere, Protocole, Zone};

use crate::compilation::RegleCompilee;
use crate::plan::PlanReseau;

/// Blocages d'une même source International avant bannissement.
pub const SEUIL_BANNISSEMENT: u32 = 5;
/// Fenêtre de comptage des blocages (secondes).
pub const FENETRE_BANNISSEMENT_SECS: u64 = 60;
/// Durée d'un bannissement automatique (secondes).
pub const DUREE_BANNISSEMENT_SECS: u64 = 900;
/// Nombre maximal de connexions suivies (au-delà : refus des nouvelles).
pub const MAX_CONNEXIONS: usize = 65_536;
/// Inactivité après laquelle une connexion suivie est oubliée (secondes).
pub const INACTIVITE_CONNEXION_SECS: u64 = 300;

/// Métadonnées d'un paquet ou d'un début de flux observé à la frontière.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaFlux {
    /// Horodatage (secondes Unix, horloge de confiance).
    pub horodatage_secs: u64,
    /// Adresse source.
    pub source: IpAddr,
    /// Adresse destination.
    pub destination: IpAddr,
    /// Protocole.
    pub protocole: Protocole,
    /// Port source.
    pub port_source: u16,
    /// Port destination.
    pub port_destination: u16,
    /// Taille du paquet IP (octets).
    pub taille: u32,
    /// Vrai pour le premier paquet d'une connexion (SYN, premier datagramme).
    pub nouvelle_connexion: bool,
}

/// Source interchangeable de métadonnées (NFQUEUE, NFLOG, conntrack, rejeu).
pub trait SourceFlux {
    /// Prochaine métadonnée, `None` quand la source est épuisée ou fermée.
    fn prochain(&mut self) -> Option<MetaFlux>;
}

impl<I: Iterator<Item = MetaFlux>> SourceFlux for I {
    fn prochain(&mut self) -> Option<MetaFlux> {
        self.next()
    }
}

/// Gravité d'un événement (alignée sur la priorité de détection du doc §12).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Gravite {
    /// Information (changement d'état demandé).
    Info,
    /// Avertissement (source déjà bannie).
    Avertissement,
    /// Élevée (dépassement de contrat, bannissement).
    Elevee,
    /// Critique (flux non déclaré, sortie / entrée de zone COG, Douane aveugle).
    Critique,
}

/// Nature d'un événement Douane.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NatureEvenement {
    /// Connexion nouvelle hors arrêté.
    FluxNonDeclare,
    /// Tentative de sortie de la zone COG hors arrêté (incident critique, §12).
    SortieZoneCog,
    /// Tentative d'ouverture vers la zone COG (INV-Z2).
    EntreeZoneCog,
    /// Paquet d'une connexion non admise.
    HorsEtat,
    /// Paquet plus grand que `taille_max_paquet`.
    TailleDepassee,
    /// Débit du flux au-delà de `debit_max_kio_s`.
    DebitDepasse,
    /// Source bannie.
    SourceBannie,
    /// Bannissement prononcé (automatique ou WorrySentinel).
    Bannissement {
        /// Fin du bannissement (secondes Unix).
        jusqua_secs: u64,
    },
    /// Voie coupée : ses flux sont bloqués.
    VoieCoupee,
    /// Voie rétablie.
    VoieRetablie,
    /// Frontière gelée : tout bloqué.
    Gel,
    /// Frontière dégelée.
    Degel,
    /// Arrêté expiré ou pas encore en vigueur : tout bloqué.
    ArreteHorsVigueur,
    /// Table des connexions saturée : nouvelles connexions refusées.
    TableSaturee,
    /// Le canal des Vigies est fermé : tout bloqué (non émis, par construction).
    CanalVigiesFerme,
}

/// Verdict de l'IPS sur une métadonnée.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Verdict {
    /// Autorisé par le flux `index_flux` de la voie.
    Autoriser {
        /// Voie de l'arrêté.
        voie: String,
        /// Rang du flux.
        index_flux: usize,
    },
    /// Bloqué.
    Bloquer(NatureEvenement),
}

/// Événement publié vers les Vigies (contrat public, stable).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvenementDouane {
    /// Horodatage (secondes Unix).
    pub horodatage_secs: u64,
    /// Frontière de l'instance.
    pub frontiere: Frontiere,
    /// Numéro de l'arrêté en vigueur.
    pub numero_arrete: u64,
    /// Gravité.
    pub gravite: Gravite,
    /// Nature.
    pub nature: NatureEvenement,
    /// Métadonnée en cause (`None` pour un changement d'état).
    pub flux: Option<MetaFlux>,
    /// Zones déduites de la source et de la destination (`None` pour un changement d'état).
    pub zones: Option<(Zone, Zone)>,
    /// Voie concernée, si connue.
    pub voie: Option<String>,
    /// Vrai si la Douane a bloqué (toujours vrai pour un événement de flux).
    pub bloque: bool,
}

/// Crée le canal Douane → Vigies.
pub fn canal_evenements() -> (Sender<EvenementDouane>, Receiver<EvenementDouane>) {
    channel()
}

/// Décision de WorrySentinel appliquée par la Douane (doc §12).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionSentinelle {
    /// Bannir une adresse jusqu'à une date.
    Bannir {
        /// Adresse.
        adresse: IpAddr,
        /// Fin (secondes Unix).
        jusqua_secs: u64,
    },
    /// Couper une voie (sans impact sur les autres, EX-49).
    CouperVoie(String),
    /// Rétablir une voie.
    RetablirVoie(String),
    /// Geler la frontière (tout bloqué).
    Geler,
    /// Dégeler la frontière.
    Degeler,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CleConnexion {
    a: (IpAddr, u16),
    b: (IpAddr, u16),
    protocole: Protocole,
}

impl CleConnexion {
    /// Clé symétrique (même clé dans les deux sens).
    fn de(meta: &MetaFlux) -> Self {
        let depart = (meta.source, meta.port_source);
        let arrivee = (meta.destination, meta.port_destination);
        let (a, b) = if depart <= arrivee { (depart, arrivee) } else { (arrivee, depart) };
        Self { a, b, protocole: meta.protocole }
    }
}

#[derive(Debug, Clone, Copy)]
struct Debit {
    seconde: u64,
    octets: u64,
}

/// Moteur IPS d'une instance Douane.
#[derive(Debug)]
pub struct MoteurIps {
    frontiere: Frontiere,
    numero_arrete: u64,
    pas_avant: u64,
    pas_apres: u64,
    regles: Vec<RegleCompilee>,
    plan: PlanReseau,
    evenements: Sender<EvenementDouane>,
    connexions: HashMap<CleConnexion, (usize, u64)>,
    debits: HashMap<usize, Debit>,
    blocages: HashMap<IpAddr, (u64, u32)>,
    bannis: HashMap<IpAddr, u64>,
    voies_coupees: BTreeSet<String>,
    gele: bool,
    aveugle: bool,
}

impl MoteurIps {
    /// Moteur pour un arrêté vérifié et ses règles compilées.
    pub fn nouveau(
        arrete: &ArreteVerifie,
        regles: Vec<RegleCompilee>,
        plan: PlanReseau,
        evenements: Sender<EvenementDouane>,
    ) -> Self {
        let a = arrete.arrete();
        Self {
            frontiere: a.frontiere,
            numero_arrete: a.numero,
            pas_avant: a.pas_avant,
            pas_apres: a.pas_apres,
            regles,
            plan,
            evenements,
            connexions: HashMap::new(),
            debits: HashMap::new(),
            blocages: HashMap::new(),
            bannis: HashMap::new(),
            voies_coupees: BTreeSet::new(),
            gele: false,
            aveugle: false,
        }
    }

    /// Vrai si le canal des Vigies est fermé (tout est alors bloqué).
    pub fn est_aveugle(&self) -> bool {
        self.aveugle
    }

    /// Voies actuellement coupées.
    pub fn voies_coupees(&self) -> &BTreeSet<String> {
        &self.voies_coupees
    }

    fn emettre(&mut self, ev: EvenementDouane) {
        if self.evenements.send(ev).is_err() {
            self.aveugle = true;
        }
    }

    fn etat(&mut self, maintenant: u64, gravite: Gravite, nature: NatureEvenement, voie: Option<String>) {
        let ev = EvenementDouane {
            horodatage_secs: maintenant,
            frontiere: self.frontiere,
            numero_arrete: self.numero_arrete,
            gravite,
            nature,
            flux: None,
            zones: None,
            voie,
            bloque: true,
        };
        self.emettre(ev);
    }

    /// Applique une décision de WorrySentinel.
    pub fn appliquer(&mut self, decision: DecisionSentinelle, maintenant_secs: u64) {
        match decision {
            DecisionSentinelle::Bannir { adresse, jusqua_secs } => {
                self.bannis.insert(adresse, jusqua_secs);
                self.connexions.retain(|k, _| k.a.0 != adresse && k.b.0 != adresse);
                self.etat(maintenant_secs, Gravite::Elevee, NatureEvenement::Bannissement { jusqua_secs }, None);
            }
            DecisionSentinelle::CouperVoie(v) => {
                let indices: BTreeSet<usize> =
                    self.regles.iter().filter(|r| r.voie == v).map(|r| r.index_flux).collect();
                self.connexions.retain(|_, (i, _)| !indices.contains(i));
                self.voies_coupees.insert(v.clone());
                self.etat(maintenant_secs, Gravite::Info, NatureEvenement::VoieCoupee, Some(v));
            }
            DecisionSentinelle::RetablirVoie(v) => {
                self.voies_coupees.remove(&v);
                self.etat(maintenant_secs, Gravite::Info, NatureEvenement::VoieRetablie, Some(v));
            }
            DecisionSentinelle::Geler => {
                self.gele = true;
                self.connexions.clear();
                self.etat(maintenant_secs, Gravite::Elevee, NatureEvenement::Gel, None);
            }
            DecisionSentinelle::Degeler => {
                self.gele = false;
                self.etat(maintenant_secs, Gravite::Info, NatureEvenement::Degel, None);
            }
        }
    }

    fn correspond(r: &RegleCompilee, m: &MetaFlux) -> bool {
        r.protocole == m.protocole
            && r.port == m.port_destination
            && (r.sources.is_empty() || r.sources.iter().any(|s| s.contient(m.source)))
            && (r.destinations.is_empty() || r.destinations.iter().any(|d| d.contient(m.destination)))
    }

    fn nature_non_declare(&self, m: &MetaFlux) -> NatureEvenement {
        match (self.plan.zone_de(m.source), self.plan.zone_de(m.destination)) {
            (Zone::Cog, _) => NatureEvenement::SortieZoneCog,
            (_, Zone::Cog) => NatureEvenement::EntreeZoneCog,
            _ => NatureEvenement::FluxNonDeclare,
        }
    }

    fn gravite(nature: &NatureEvenement) -> Gravite {
        match nature {
            NatureEvenement::FluxNonDeclare
            | NatureEvenement::SortieZoneCog
            | NatureEvenement::EntreeZoneCog
            | NatureEvenement::HorsEtat
            | NatureEvenement::ArreteHorsVigueur
            | NatureEvenement::CanalVigiesFerme => Gravite::Critique,
            NatureEvenement::TailleDepassee
            | NatureEvenement::DebitDepasse
            | NatureEvenement::Bannissement { .. }
            | NatureEvenement::Gel
            | NatureEvenement::TableSaturee => Gravite::Elevee,
            NatureEvenement::SourceBannie => Gravite::Avertissement,
            NatureEvenement::VoieCoupee | NatureEvenement::VoieRetablie | NatureEvenement::Degel => Gravite::Info,
        }
    }

    fn decider(&mut self, m: &MetaFlux) -> (Verdict, Option<String>) {
        let t = m.horodatage_secs;
        if self.aveugle {
            return (Verdict::Bloquer(NatureEvenement::CanalVigiesFerme), None);
        }
        if t < self.pas_avant || t >= self.pas_apres {
            return (Verdict::Bloquer(NatureEvenement::ArreteHorsVigueur), None);
        }
        if self.gele {
            return (Verdict::Bloquer(NatureEvenement::Gel), None);
        }
        for ip in [m.source, m.destination] {
            if let Some(&fin) = self.bannis.get(&ip) {
                if t < fin {
                    return (Verdict::Bloquer(NatureEvenement::SourceBannie), None);
                }
                self.bannis.remove(&ip);
            }
        }
        let cle = CleConnexion::de(m);
        let index = if m.nouvelle_connexion {
            let trouve = self
                .regles
                .iter()
                .find(|r| !self.voies_coupees.contains(&r.voie) && Self::correspond(r, m))
                .map(|r| r.index_flux);
            let coupee = self.regles.iter().any(|r| self.voies_coupees.contains(&r.voie) && Self::correspond(r, m));
            match trouve {
                Some(i) => i,
                None if coupee => return (Verdict::Bloquer(NatureEvenement::VoieCoupee), None),
                None => return (Verdict::Bloquer(self.nature_non_declare(m)), None),
            }
        } else {
            match self.connexions.get(&cle) {
                Some(&(i, _)) => i,
                None => return (Verdict::Bloquer(NatureEvenement::HorsEtat), None),
            }
        };
        let Some(regle) = self.regles.iter().find(|r| r.index_flux == index) else {
            return (Verdict::Bloquer(NatureEvenement::HorsEtat), None);
        };
        let voie = regle.voie.clone();
        if m.taille > u32::from(regle.taille_max_paquet) {
            return (Verdict::Bloquer(NatureEvenement::TailleDepassee), Some(voie));
        }
        let plafond = u64::from(regle.debit_max_kio_s) * 1024;
        let d = self.debits.entry(index).or_insert(Debit { seconde: t, octets: 0 });
        if d.seconde != t {
            *d = Debit { seconde: t, octets: 0 };
        }
        if d.octets + u64::from(m.taille) > plafond {
            return (Verdict::Bloquer(NatureEvenement::DebitDepasse), Some(voie));
        }
        d.octets += u64::from(m.taille);
        if m.nouvelle_connexion && !self.connexions.contains_key(&cle) {
            if self.connexions.len() >= MAX_CONNEXIONS {
                self.connexions.retain(|_, (_, vu)| t.saturating_sub(*vu) < INACTIVITE_CONNEXION_SECS);
            }
            if self.connexions.len() >= MAX_CONNEXIONS {
                return (Verdict::Bloquer(NatureEvenement::TableSaturee), Some(voie));
            }
        }
        self.connexions.insert(cle, (index, t));
        (Verdict::Autoriser { voie: voie.clone(), index_flux: index }, Some(voie))
    }

    /// Évalue une métadonnée : verdict, événement éventuel, bannissement automatique.
    pub fn evaluer(&mut self, m: &MetaFlux) -> Verdict {
        let (verdict, voie) = self.decider(m);
        let Verdict::Bloquer(nature) = &verdict else {
            return verdict;
        };
        if *nature == NatureEvenement::CanalVigiesFerme {
            return verdict;
        }
        let zones = (self.plan.zone_de(m.source), self.plan.zone_de(m.destination));
        let ev = EvenementDouane {
            horodatage_secs: m.horodatage_secs,
            frontiere: self.frontiere,
            numero_arrete: self.numero_arrete,
            gravite: Self::gravite(nature),
            nature: nature.clone(),
            flux: Some(m.clone()),
            zones: Some(zones),
            voie,
            bloque: true,
        };
        self.emettre(ev);
        // Bannissement automatique : sources International seulement, hors
        // blocages dus à un état (gel, expiration, bannissement déjà prononcé).
        let automatique = !matches!(
            nature,
            NatureEvenement::SourceBannie | NatureEvenement::Gel | NatureEvenement::ArreteHorsVigueur
        );
        if automatique && zones.0 == Zone::International {
            let t = m.horodatage_secs;
            let e = self.blocages.entry(m.source).or_insert((t, 0));
            if t.saturating_sub(e.0) >= FENETRE_BANNISSEMENT_SECS {
                *e = (t, 0);
            }
            e.1 += 1;
            if e.1 >= SEUIL_BANNISSEMENT {
                self.blocages.remove(&m.source);
                self.appliquer(
                    DecisionSentinelle::Bannir { adresse: m.source, jusqua_secs: t + DUREE_BANNISSEMENT_SECS },
                    t,
                );
            }
        }
        if self.aveugle {
            return Verdict::Bloquer(NatureEvenement::CanalVigiesFerme);
        }
        verdict
    }

    /// Consomme une source jusqu'à épuisement ; renvoie les verdicts dans l'ordre.
    pub fn traiter<S: SourceFlux>(&mut self, source: &mut S) -> Vec<Verdict> {
        let mut verdicts = Vec::new();
        while let Some(m) = source.prochain() {
            verdicts.push(self.evaluer(&m));
        }
        verdicts
    }
}
