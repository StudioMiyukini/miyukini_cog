//! @id     miyudouane_compilation
//! @do     compiler_arrete_en_regles
//! @role   securite
//! @layer  operateur
//! @human  miyudouane — compile un arrêté vérifié en règles concrètes (adresses, interfaces, crochets).

//! # Compilation d'un arrêté
//!
//! Un arrêté vérifié + le plan réseau → une liste de [`RegleCompilee`],
//! **source unique** du pare-feu nftables ([`crate::nft`]) et de l'IPS
//! ([`crate::ips`]) : les deux appliquent exactement les mêmes autorisations.
//!
//! - Frontière en **routage** (externe) : une règle `Transit` par flux.
//! - Frontière en **rupture** (portails) : deux règles par flux, `Entree`
//!   (sandbox COG → adresse du portail) et `Sortie` (portail → sandbox de la
//!   zone destinataire). Aucun transit : rien ne traverse sans être terminé.
//! - Toute référence manquante (sandbox, lien, interface) est une erreur :
//!   on ne devine jamais une adresse (fail-closed).

use std::fmt;

use borderguard::{ArreteVerifie, Flux, Frontiere, Protocole, Reseau, Zone};

use crate::plan::{DefinitionFrontiere, ModeFrontiere, PlanReseau};

/// Crochet netfilter d'une règle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Crochet {
    /// `input` : connexion terminée dans la frontière.
    Entree,
    /// `forward` : traversée (frontière externe seulement).
    Transit,
    /// `output` : connexion ouverte par la frontière.
    Sortie,
}

impl Crochet {
    /// Nom de chaîne nftables.
    pub const fn chaine(self) -> &'static str {
        match self {
            Self::Entree => "entree",
            Self::Transit => "transit",
            Self::Sortie => "sortie",
        }
    }
}

/// Règle d'autorisation concrète (une connexion nouvelle admise).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegleCompilee {
    /// Voie déclarée de l'arrêté.
    pub voie: String,
    /// Service propriétaire de la voie.
    pub service: String,
    /// Rang du flux dans l'arrêté (0..).
    pub index_flux: usize,
    /// Marque conntrack (`index_flux + 1`, jamais 0).
    pub marque: u32,
    /// Crochet.
    pub crochet: Crochet,
    /// Interface d'entrée (`None` : sortie locale).
    pub iif: Option<String>,
    /// Interface de sortie (`None` : entrée locale).
    pub oif: Option<String>,
    /// Adresses source admises (vide = toutes).
    pub sources: Vec<Reseau>,
    /// Adresses destination admises (vide = toutes).
    pub destinations: Vec<Reseau>,
    /// Protocole.
    pub protocole: Protocole,
    /// Port de destination.
    pub port: u16,
    /// Débit maximal (Kio/s), appliqué à toutes les connexions du flux.
    pub debit_max_kio_s: u32,
    /// Taille maximale d'un paquet.
    pub taille_max_paquet: u16,
    /// Zone initiatrice (du flux de l'arrêté).
    pub zone_source: Zone,
    /// Zone destinataire (du flux de l'arrêté).
    pub zone_destination: Zone,
}

/// Erreur de compilation (aucune règle produite).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErreurCompilation(pub String);

impl fmt::Display for ErreurCompilation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "compilation refusée : {}", self.0)
    }
}

impl std::error::Error for ErreurCompilation {}

fn hote_sandbox(plan: &PlanReseau, zone: Zone, service: &str, voie: &str) -> Result<Reseau, ErreurCompilation> {
    plan.sandbox(zone, service)
        .map(|s| Reseau::hote(s.adresse))
        .ok_or_else(|| ErreurCompilation(format!("voie {voie} : aucune sandbox {zone}/{service} dans le plan")))
}

/// Adresses de l'extrémité extérieure : liste blanche, sinon réseau connu de
/// la zone (Fédération), sinon refus (jamais « toute adresse » implicite).
fn extremite_exterieure(plan: &PlanReseau, f: &Flux, zone: Zone) -> Result<Vec<Reseau>, ErreurCompilation> {
    if !f.pairs.is_empty() {
        return Ok(f.pairs.clone());
    }
    plan.reseau_exterieur(zone)
        .map(|r| vec![r])
        .ok_or_else(|| ErreurCompilation(format!("voie {} : zone {zone} sans liste blanche ni réseau connu", f.voie)))
}

fn interface_exterieure(def: &DefinitionFrontiere, zone: Zone, voie: &str) -> Result<String, ErreurCompilation> {
    def.exterieure(zone)
        .map(|e| e.interface.clone())
        .ok_or_else(|| ErreurCompilation(format!("voie {voie} : aucune interface vers {zone}")))
}

fn lien(def: &DefinitionFrontiere, zone: Zone, voie: &str) -> Result<(String, std::net::IpAddr), ErreurCompilation> {
    def.lien(zone)
        .map(|l| (l.interface.clone(), l.adresse))
        .ok_or_else(|| ErreurCompilation(format!("voie {voie} : aucun lien {} → {zone}", def.frontiere)))
}

/// Compile un arrêté vérifié pour sa frontière.
pub fn compiler(arrete: &ArreteVerifie, plan: &PlanReseau) -> Result<Vec<RegleCompilee>, ErreurCompilation> {
    let a = arrete.arrete();
    let def = plan
        .frontiere(a.frontiere)
        .ok_or_else(|| ErreurCompilation(format!("frontière {} absente du plan", a.frontiere)))?;
    let mut regles = Vec::new();
    for (index_flux, f) in a.flux.iter().enumerate() {
        let voie = a
            .voie(&f.voie)
            .ok_or_else(|| ErreurCompilation(format!("voie {} non déclarée", f.voie)))?;
        let base = |crochet, iif, oif, sources, destinations| RegleCompilee {
            voie: voie.id.clone(),
            service: voie.service.clone(),
            index_flux,
            marque: u32::try_from(index_flux + 1).unwrap_or(u32::MAX),
            crochet,
            iif,
            oif,
            sources,
            destinations,
            protocole: f.protocole,
            port: f.port,
            debit_max_kio_s: f.debit_max_kio_s,
            taille_max_paquet: f.taille_max_paquet,
            zone_source: f.source,
            zone_destination: f.destination,
        };
        match def.mode {
            ModeFrontiere::Routage => {
                if a.frontiere != Frontiere::Externe {
                    return Err(ErreurCompilation(format!("frontière {} en routage", a.frontiere)));
                }
                let (iif, oif, sources, destinations) = if f.source.est_exterieure() {
                    (
                        interface_exterieure(def, f.source, &f.voie)?,
                        lien(def, f.destination, &f.voie)?.0,
                        extremite_exterieure(plan, f, f.source)?,
                        vec![hote_sandbox(plan, f.destination, &voie.service, &f.voie)?],
                    )
                } else {
                    (
                        lien(def, f.source, &f.voie)?.0,
                        interface_exterieure(def, f.destination, &f.voie)?,
                        vec![hote_sandbox(plan, f.source, &voie.service, &f.voie)?],
                        extremite_exterieure(plan, f, f.destination)?,
                    )
                };
                regles.push(base(Crochet::Transit, Some(iif), Some(oif), sources, destinations));
            }
            ModeFrontiere::Rupture => {
                if f.source.est_exterieure() || f.destination.est_exterieure() {
                    return Err(ErreurCompilation(format!("voie {} : extérieur sur un portail", f.voie)));
                }
                let (iif, adresse_entree) = lien(def, f.source, &f.voie)?;
                let (oif, adresse_sortie) = lien(def, f.destination, &f.voie)?;
                regles.push(base(
                    Crochet::Entree,
                    Some(iif),
                    None,
                    vec![hote_sandbox(plan, f.source, &voie.service, &f.voie)?],
                    vec![Reseau::hote(adresse_entree)],
                ));
                regles.push(base(
                    Crochet::Sortie,
                    None,
                    Some(oif),
                    vec![Reseau::hote(adresse_sortie)],
                    vec![hote_sandbox(plan, f.destination, &voie.service, &f.voie)?],
                ));
            }
        }
    }
    Ok(regles)
}
