//! @id     miyuvigie_evenement
//! @do     definir_entree_capteur_flux
//! @role   securite
//! @layer  toolkit
//! @human  miyuvigie — module « evenement » : événements de flux en entrée des Vigies.

//! Entrée générique des Vigies : un **événement de flux** tel qu'observé par
//! la Douane (pare-feu + IPS) ou par un autre capteur.
//!
//! La Douane (crate séparé, lot 3) définit son propre type d'événement ; un
//! adaptateur (`miyufrontiere::SourceDouane`) implémente [`SourceEvenements`] en
//! convertissant ses événements en [`EvenementFlux`]. Champs requis :
//!
//! | champ | rôle |
//! |---|---|
//! | `id` | identifiant unique **par capteur** (déduplication, preuve) |
//! | `horodatage_ms` | instant de l'événement, ms Unix (horloge de la source) |
//! | `zone_src`, `zone_dst` | zones déterminées par la Douane (ou [`Zone::Inconnue`]) |
//! | `voie` | voie du Portail concernée, si attribuable |
//! | `verdict` | décision de la Douane : autorisé / bloqué / inconnu |
//! | `octets` | volume du flux (0 si inconnu) |
//! | `ip_src`, `ip_dst`, `port_dst`, `protocole` | facultatifs, pour l'explication |
//! | `motif` | motif de rejet de la Douane (`MDOU|…|<MOTIF>|`, nature de l'IPS), lu par les règles intégrées A-005 |
//!
//! `voie` n'est renseignée que depuis le **côté d'entrée** (la sandbox qui a
//! émis le trafic) : jamais depuis l'interface de sortie ni depuis une adresse
//! que rien ne recoupe (anti-usurpation, A-005).

use std::fmt;
use std::net::IpAddr;

use crate::nftables::MotifJournal;
use crate::signal::{CapteurId, EvenementId, VoieId};
use crate::zone::Zone;

/// Décision de la Douane sur un flux.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Verdict {
    /// Flux autorisé (conforme à un arrêté).
    Autorise,
    /// Flux bloqué par l'IPS / le pare-feu.
    Bloque,
    /// Verdict non communiqué — jamais traité comme « autorisé ».
    Inconnu,
}

impl Verdict {
    /// Nom utilisé dans les règles et les journaux.
    pub fn nom(self) -> &'static str {
        match self {
            Verdict::Autorise => "autorise",
            Verdict::Bloque => "bloque",
            Verdict::Inconnu => "inconnu",
        }
    }

    /// Analyse stricte (`inconnu` refusé en entrée de règle comme de journal).
    pub fn depuis_nom(s: &str) -> Option<Self> {
        match s {
            "autorise" => Some(Verdict::Autorise),
            "bloque" => Some(Verdict::Bloque),
            _ => None,
        }
    }
}

/// Protocole de transport (facultatif, pour l'explication).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Protocole {
    /// TCP.
    Tcp,
    /// UDP.
    Udp,
    /// ICMP / ICMPv6.
    Icmp,
    /// Autre numéro de protocole.
    Autre(u8),
}

/// Événement de flux générique (entrée des Vigies).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvenementFlux {
    /// Identifiant unique par capteur.
    pub id: EvenementId,
    /// Horodatage (ms Unix), horloge de la source.
    pub horodatage_ms: u64,
    /// Zone d'origine.
    pub zone_src: Zone,
    /// Zone de destination.
    pub zone_dst: Zone,
    /// Voie du Portail, si attribuable.
    pub voie: Option<VoieId>,
    /// Verdict de la Douane.
    pub verdict: Verdict,
    /// Volume en octets.
    pub octets: u64,
    /// Adresse source (facultative).
    pub ip_src: Option<IpAddr>,
    /// Adresse destination (facultative).
    pub ip_dst: Option<IpAddr>,
    /// Port destination (facultatif).
    pub port_dst: Option<u16>,
    /// Protocole (facultatif).
    pub protocole: Option<Protocole>,
    /// Motif de rejet de la Douane, si connu (A-005).
    pub motif: Option<MotifJournal>,
}

/// Échec de relève d'une source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErreurSource(pub String);

impl fmt::Display for ErreurSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "source indisponible : {}", self.0)
    }
}

impl std::error::Error for ErreurSource {}

/// Source d'événements de flux relevée par une Vigie (adaptateur Douane, etc.).
///
/// `relever` rend les événements disponibles depuis la dernière relève.
/// `Ok(vec![])` signifie « source vivante, aucun événement » ; `Err` signifie
/// « source muette » : aucun pouls n'est alors émis (le capteur vieillira vers
/// `Inconnu` côté CaringNanny).
pub trait SourceEvenements {
    /// Capteur que représente cette source.
    fn capteur(&self) -> &CapteurId;

    /// Relève les événements disponibles.
    fn relever(&mut self) -> Result<Vec<EvenementFlux>, ErreurSource>;
}

/// Relevé d'une source de **lignes** de journal (relais NFLOG, convention
/// `MDOU|`, A-004) : les lignes rendues, plus les entrées que la source n'a pas
/// pu mettre en ligne (trame netlink ou paquet indécodable).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReleveJournal {
    /// Lignes au format canonique lu par [`crate::LecteurNftables`].
    pub lignes: Vec<String>,
    /// Entrées reçues mais non converties : comptées comme mal formées par la
    /// Vigie (jamais bénignes, le capteur passe dégradé).
    pub mal_formees: u64,
}

/// Source de lignes de journal relevée par une Vigie (relais NFLOG).
///
/// Même contrat que [`SourceEvenements`] : `Ok` = source vivante (même sans
/// ligne), `Err` = source muette, défaillante ou ayant perdu des entrées —
/// aucun pouls, capteur en échec (voie aveugle côté WorrySentinel, A-004).
pub trait SourceJournal {
    /// Capteur que représente cette source.
    fn capteur(&self) -> &CapteurId;

    /// Relève les lignes disponibles depuis la dernière relève.
    fn relever_lignes(&mut self) -> Result<ReleveJournal, ErreurSource>;
}
