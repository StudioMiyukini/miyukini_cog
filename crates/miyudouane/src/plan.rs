//! @id     miyudouane_plan
//! @do     declarer_plan_reseau_zones
//! @role   securite
//! @layer  operateur
//! @human  miyudouane — plan réseau déclaratif : zones, frontières, sandboxes par service.

//! # Plan réseau
//!
//! Description déclarative du réseau intégré (modèle zones et couches §11,
//! niveau 1 « namespaces + utilisateurs + nftables ») :
//!
//! - **une zone = un espace de noms** contenant un pont `br0` (sans adresse) ;
//!   le pont est filtré (`table bridge`) : segmentation par service (INV-Z8) ;
//! - **une frontière = un espace de noms** relié aux zones par des paires veth ;
//!   la frontière externe porte la carte physique et route (mode
//!   [`ModeFrontiere::Routage`]) ; les portails ne routent **pas** (mode
//!   [`ModeFrontiere::Rupture`] : rupture protocolaire, toute connexion se
//!   termine dans le portail) ;
//! - **un service = une sandbox** (espace de noms, utilisateur, slice systemd),
//!   attachée au pont de sa zone. Zone COG : aucune route par défaut (INV-Z4).
//!
//! Adressage (doc §11, indicatif) : zone COG `10.77.0.0/24` sans passerelle ·
//! Franche `10.77.1.0/24` · Fédération `10.78.0.0/16`. Franche locale
//! `10.77.2.0/24` (choix de ce crate).

use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;

use borderguard::arrete::identifiant_valide;
use borderguard::{Frontiere, Reseau, Zone};

/// Longueur maximale d'un nom d'interface Linux (IFNAMSIZ − 1).
pub const LONGUEUR_MAX_INTERFACE: usize = 15;
/// Longueur maximale d'un nom d'utilisateur système.
pub const LONGUEUR_MAX_UTILISATEUR: usize = 32;

/// Mode d'une frontière.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModeFrontiere {
    /// La frontière route entre ses liens (frontière externe) : chaîne `forward`.
    Routage,
    /// Rupture protocolaire (portails) : aucun routage, connexions terminées
    /// dans la frontière (`input` côté zone COG, `output` côté Franche).
    Rupture,
}

/// Lien veth entre une frontière et le pont d'une zone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lien {
    /// Zone reliée.
    pub zone: Zone,
    /// Interface côté frontière.
    pub interface: String,
    /// Interface côté zone (port du pont `br0`).
    pub interface_zone: String,
    /// Adresse de la frontière sur ce lien.
    pub adresse: IpAddr,
}

/// Interface extérieure d'une frontière (carte physique, WireGuard fédéral).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceExterieure {
    /// Zones extérieures atteintes par cette interface.
    pub zones: Vec<Zone>,
    /// Nom de l'interface dans l'espace de la frontière.
    pub interface: String,
}

/// Définition d'une zone intérieure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinitionZone {
    /// Zone.
    pub zone: Zone,
    /// Espace de noms (pont de zone).
    pub netns: String,
    /// Réseau de la zone.
    pub reseau: Reseau,
    /// Passerelle par défaut des sandboxes (`None` : aucune route par défaut).
    pub passerelle: Option<IpAddr>,
    /// Groupe NFLOG des journaux de la zone.
    pub groupe_nflog: u16,
}

/// Définition d'une frontière (une instance Douane).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DefinitionFrontiere {
    /// Frontière.
    pub frontiere: Frontiere,
    /// Espace de noms de l'instance.
    pub netns: String,
    /// Routage ou rupture protocolaire.
    pub mode: ModeFrontiere,
    /// Liens vers les zones intérieures.
    pub liens: Vec<Lien>,
    /// Interfaces extérieures (frontière externe seulement).
    pub exterieures: Vec<InterfaceExterieure>,
    /// Groupe NFLOG des journaux de la frontière.
    pub groupe_nflog: u16,
}

impl DefinitionFrontiere {
    /// Lien vers une zone intérieure.
    pub fn lien(&self, zone: Zone) -> Option<&Lien> {
        self.liens.iter().find(|l| l.zone == zone)
    }

    /// Interface extérieure qui atteint `zone`.
    pub fn exterieure(&self, zone: Zone) -> Option<&InterfaceExterieure> {
        self.exterieures.iter().find(|e| e.zones.contains(&zone))
    }
}

/// Plafonds de ressources d'une sandbox (slice systemd, doc §10).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ressources {
    /// Quota CPU en pourcentage d'un cœur.
    pub cpu_pourcent: u32,
    /// Mémoire maximale (Mio).
    pub memoire_mio: u32,
    /// Nombre maximal de tâches.
    pub taches_max: u32,
    /// Poids IO (1..=10000).
    pub poids_io: u32,
}

impl Ressources {
    /// Plafonds de référence, à remplacer par ceux du contrat de service (LogisticsSteward).
    pub const fn reference() -> Self {
        Self { cpu_pourcent: 50, memoire_mio: 512, taches_max: 128, poids_io: 100 }
    }
}

/// Sandbox d'un service dans une zone (EX-21 zone COG, EX-29 Franche).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sandbox {
    /// Service.
    pub service: String,
    /// Zone d'exécution.
    pub zone: Zone,
    /// Espace de noms propre.
    pub netns: String,
    /// Port du pont de zone (côté zone de la paire veth).
    pub interface_zone: String,
    /// Adresse dans le réseau de la zone.
    pub adresse: IpAddr,
    /// Utilisateur système dédié.
    pub utilisateur: String,
    /// Plafonds de ressources.
    pub ressources: Ressources,
    /// Répertoire de la DB secondaire (Franche seulement, une par service).
    pub db_secondaire: Option<PathBuf>,
}

impl Sandbox {
    /// Nom de la slice systemd.
    pub fn slice(&self) -> String {
        format!("mcog-{}-{}.slice", self.zone.nom(), self.service)
    }
}

/// Erreur de plan (fail-closed : un plan invalide ne produit rien).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErreurPlan(pub String);

impl std::fmt::Display for ErreurPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "plan réseau invalide : {}", self.0)
    }
}

impl std::error::Error for ErreurPlan {}

/// Plan réseau complet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanReseau {
    /// Zones intérieures.
    pub zones: Vec<DefinitionZone>,
    /// Frontières.
    pub frontieres: Vec<DefinitionFrontiere>,
    /// Réseaux connus des origines extérieures (Fédération).
    pub reseaux_exterieurs: Vec<(Zone, Reseau)>,
    /// Sandboxes de service.
    pub sandboxes: Vec<Sandbox>,
}

fn v4(a: u8, b: u8, c: u8, d: u8) -> IpAddr {
    IpAddr::V4(Ipv4Addr::new(a, b, c, d))
}

fn reseau(s: &str) -> Reseau {
    s.parse().unwrap_or_else(|()| unreachable!("réseau constant invalide : {s}"))
}

impl PlanReseau {
    /// Plan de référence (doc §11) avec une sandbox par `(zone, service)` donné.
    pub fn reference(services: &[(Zone, &str)]) -> Result<Self, ErreurPlan> {
        let zones = vec![
            DefinitionZone {
                zone: Zone::Cog,
                netns: "mcog-z-cog".into(),
                reseau: reseau("10.77.0.0/24"),
                passerelle: None,
                groupe_nflog: 110,
            },
            DefinitionZone {
                zone: Zone::Franche,
                netns: "mcog-z-franche".into(),
                reseau: reseau("10.77.1.0/24"),
                passerelle: Some(v4(10, 77, 1, 254)),
                groupe_nflog: 111,
            },
            DefinitionZone {
                zone: Zone::FrancheLocale,
                netns: "mcog-z-locale".into(),
                reseau: reseau("10.77.2.0/24"),
                passerelle: None,
                groupe_nflog: 112,
            },
        ];
        let lien = |zone, interface: &str, interface_zone: &str, adresse| Lien {
            zone,
            interface: interface.into(),
            interface_zone: interface_zone.into(),
            adresse,
        };
        let frontieres = vec![
            DefinitionFrontiere {
                frontiere: Frontiere::Externe,
                netns: "mcog-f-externe".into(),
                mode: ModeFrontiere::Routage,
                liens: vec![lien(Zone::Franche, "ext-franche", "z-ext", v4(10, 77, 1, 254))],
                exterieures: vec![
                    InterfaceExterieure { zones: vec![Zone::International, Zone::Origin], interface: "ext0".into() },
                    InterfaceExterieure { zones: vec![Zone::Federation], interface: "wg-fed".into() },
                ],
                groupe_nflog: 101,
            },
            DefinitionFrontiere {
                frontiere: Frontiere::PortailInterne,
                netns: "mcog-f-portail".into(),
                mode: ModeFrontiere::Rupture,
                liens: vec![
                    lien(Zone::Cog, "pi-cog", "z-pi", v4(10, 77, 0, 253)),
                    lien(Zone::Franche, "pi-franche", "z-pi", v4(10, 77, 1, 253)),
                ],
                exterieures: vec![],
                groupe_nflog: 102,
            },
            DefinitionFrontiere {
                frontiere: Frontiere::PortailLocal,
                netns: "mcog-f-local".into(),
                mode: ModeFrontiere::Rupture,
                liens: vec![
                    lien(Zone::Cog, "pl-cog", "z-pl", v4(10, 77, 0, 252)),
                    lien(Zone::FrancheLocale, "pl-locale", "z-pl", v4(10, 77, 2, 252)),
                ],
                exterieures: vec![],
                groupe_nflog: 103,
            },
        ];
        let mut plan = Self {
            zones,
            frontieres,
            reseaux_exterieurs: vec![(Zone::Federation, reseau("10.78.0.0/16"))],
            sandboxes: Vec::new(),
        };
        for (zone, service) in services {
            plan.ajouter_sandbox(*zone, service, Ressources::reference())?;
        }
        plan.valider()?;
        Ok(plan)
    }

    /// Ajoute une sandbox (adresse suivante de la zone à partir de `.10`).
    pub fn ajouter_sandbox(&mut self, zone: Zone, service: &str, ressources: Ressources) -> Result<(), ErreurPlan> {
        if !identifiant_valide(service) {
            return Err(ErreurPlan(format!("service « {service} » : identifiant invalide")));
        }
        let def = self.zone(zone).ok_or_else(|| ErreurPlan(format!("zone {zone} absente du plan")))?;
        if self.sandbox(zone, service).is_some() {
            return Err(ErreurPlan(format!("sandbox {zone}/{service} en double")));
        }
        let IpAddr::V4(base) = def.reseau.adresse() else {
            return Err(ErreurPlan(format!("zone {zone} : seul IPv4 est pris en charge")));
        };
        let rang = self.sandboxes.iter().filter(|s| s.zone == zone).count();
        let hote = 10 + rang;
        if hote > 250 {
            return Err(ErreurPlan(format!("zone {zone} : plus d'adresse de sandbox")));
        }
        let o = base.octets();
        let adresse = v4(o[0], o[1], o[2], hote as u8);
        let lettre = &zone.nom()[..1];
        let db_secondaire = (zone == Zone::Franche)
            .then(|| PathBuf::from(format!("/var/lib/miyukini/franche/{service}/db-secondaire")));
        self.sandboxes.push(Sandbox {
            service: service.into(),
            zone,
            netns: format!("mcog-s-{}-{service}", zone.nom()),
            interface_zone: format!("sv{}", self.sandboxes.len()),
            adresse,
            utilisateur: format!("mc{lettre}-{service}"),
            ressources,
            db_secondaire,
        });
        Ok(())
    }

    /// Définition d'une zone.
    pub fn zone(&self, zone: Zone) -> Option<&DefinitionZone> {
        self.zones.iter().find(|z| z.zone == zone)
    }

    /// Définition d'une frontière.
    pub fn frontiere(&self, frontiere: Frontiere) -> Option<&DefinitionFrontiere> {
        self.frontieres.iter().find(|f| f.frontiere == frontiere)
    }

    /// Sandbox d'un service dans une zone.
    pub fn sandbox(&self, zone: Zone, service: &str) -> Option<&Sandbox> {
        self.sandboxes.iter().find(|s| s.zone == zone && s.service == service)
    }

    /// Réseau connu d'une origine extérieure (sinon : toute adresse, liste blanche obligatoire).
    pub fn reseau_exterieur(&self, zone: Zone) -> Option<Reseau> {
        self.reseaux_exterieurs.iter().find(|(z, _)| *z == zone).map(|(_, r)| *r)
    }

    /// Zone d'une adresse : réseau de zone intérieure, réseau extérieur connu,
    /// sinon International (zero trust).
    pub fn zone_de(&self, ip: IpAddr) -> Zone {
        self.zones
            .iter()
            .find(|z| z.reseau.contient(ip))
            .map(|z| z.zone)
            .or_else(|| self.reseaux_exterieurs.iter().find(|(_, r)| r.contient(ip)).map(|(z, _)| *z))
            .unwrap_or(Zone::International)
    }

    /// Contrôles de cohérence (noms sûrs et bornés, adresses dans les zones,
    /// zone COG sans passerelle — INV-Z4).
    pub fn valider(&self) -> Result<(), ErreurPlan> {
        let nom_sur = |n: &str| {
            !n.is_empty() && n.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        };
        let interface_sure = |n: &str| nom_sur(n) && n.len() <= LONGUEUR_MAX_INTERFACE;
        for z in &self.zones {
            if !nom_sur(&z.netns) {
                return Err(ErreurPlan(format!("espace de noms « {} »", z.netns)));
            }
            if z.zone == Zone::Cog && z.passerelle.is_some() {
                return Err(ErreurPlan("zone COG avec passerelle (INV-Z4)".into()));
            }
            if !z.zone.est_interieure() {
                return Err(ErreurPlan(format!("zone {} non intérieure", z.zone)));
            }
        }
        for f in &self.frontieres {
            if !nom_sur(&f.netns) {
                return Err(ErreurPlan(format!("espace de noms « {} »", f.netns)));
            }
            for l in &f.liens {
                let zone = self.zone(l.zone).ok_or_else(|| ErreurPlan(format!("lien vers {} sans zone", l.zone)))?;
                if !interface_sure(&l.interface) || !interface_sure(&l.interface_zone) {
                    return Err(ErreurPlan(format!("interface du lien {}/{}", f.frontiere, l.zone)));
                }
                if !zone.reseau.contient(l.adresse) {
                    return Err(ErreurPlan(format!("adresse {} hors de la zone {}", l.adresse, l.zone)));
                }
            }
            for e in &f.exterieures {
                if !interface_sure(&e.interface) || e.zones.iter().any(|z| z.est_interieure()) {
                    return Err(ErreurPlan(format!("interface extérieure « {} »", e.interface)));
                }
            }
        }
        for s in &self.sandboxes {
            let zone = self.zone(s.zone).ok_or_else(|| ErreurPlan(format!("sandbox {} sans zone", s.service)))?;
            if !nom_sur(&s.netns) || !interface_sure(&s.interface_zone) || !zone.reseau.contient(s.adresse) {
                return Err(ErreurPlan(format!("sandbox {}/{}", s.zone, s.service)));
            }
            if !nom_sur(&s.utilisateur) || s.utilisateur.len() > LONGUEUR_MAX_UTILISATEUR {
                return Err(ErreurPlan(format!("utilisateur « {} » (≤ 32 caractères)", s.utilisateur)));
            }
            if (s.zone == Zone::Franche) != s.db_secondaire.is_some() {
                return Err(ErreurPlan(format!("DB secondaire de {}/{} : Franche seulement", s.zone, s.service)));
            }
        }
        Ok(())
    }
}
