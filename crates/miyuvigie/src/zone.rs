//! @id     miyuvigie_zone
//! @do     classer_adresses_par_zone
//! @role   securite
//! @layer  toolkit
//! @human  miyuvigie — module « zone » : zones du COG et plan d'adressage.

//! Zones du modèle (`docs/architecture/MODELE-ZONES-ET-COUCHES-COG.md` §11) et
//! classement d'une adresse IP dans une zone.
//!
//! Adressage indicatif du document : zone COG `10.77.0.0/24` (sans passerelle),
//! Franche `10.77.1.0/24`, Fédération `10.78.0.0/16` ; Franche locale
//! `10.77.2.0/24` (plan de la Douane, A-004). Le plan est une donnée :
//! il est fourni par l'appelant ([`PlanAdressage::new`]) ; [`PlanAdressage::indicatif`]
//! reprend les valeurs du document.
//!
//! Une adresse qui ne tombe dans aucun préfixe déclaré est classée
//! [`Zone::Exterieur`] : hors des réseaux connus = hors de la zone COG (fail-closed).

use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

/// Zone réseau du COG.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Zone {
    /// Zone COG (TRUSTED) : aucune route sortante (INV-Z1).
    Cog,
    /// Portail interne : seul passage entre la Franche et la zone COG (INV-Z7).
    Portail,
    /// Zone Franche : façades, relais, quarantaine.
    Franche,
    /// Franche locale : seconde DMZ, LAN domestique (domotique), A-004.
    FrancheLocale,
    /// Réseau de la Fédération.
    Federation,
    /// Frontière BorderGuard / Douane.
    BorderGuard,
    /// Tout le reste (International, Internet) — UNKNOWN.
    Exterieur,
    /// Zone non déterminable (donnée absente ou incohérente) — jamais bénigne.
    Inconnue,
}

impl Zone {
    /// Nom court utilisé dans les préfixes de journal et les règles.
    pub fn nom(self) -> &'static str {
        match self {
            Zone::Cog => "cog",
            Zone::Portail => "portail",
            Zone::Franche => "franche",
            Zone::FrancheLocale => "locale",
            Zone::Federation => "federation",
            Zone::BorderGuard => "borderguard",
            Zone::Exterieur => "ext",
            Zone::Inconnue => "inconnue",
        }
    }
}

impl fmt::Display for Zone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.nom())
    }
}

impl FromStr for Zone {
    type Err = ();

    /// Analyse stricte (minuscules exactes) ; `inconnue` n'est pas acceptée en
    /// entrée : une source ne peut pas déclarer explicitement une zone inconnue.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cog" => Ok(Zone::Cog),
            "portail" => Ok(Zone::Portail),
            "franche" => Ok(Zone::Franche),
            "locale" => Ok(Zone::FrancheLocale),
            "federation" => Ok(Zone::Federation),
            "borderguard" => Ok(Zone::BorderGuard),
            "ext" => Ok(Zone::Exterieur),
            _ => Err(()),
        }
    }
}

/// Préfixe réseau (CIDR) IPv4 ou IPv6.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Prefixe {
    reseau: IpAddr,
    longueur: u8,
}

impl Prefixe {
    /// Construit un préfixe ; `None` si la longueur dépasse la taille de l'adresse
    /// ou si des bits d'hôte sont positionnés (préfixe ambigu refusé).
    pub fn new(reseau: IpAddr, longueur: u8) -> Option<Self> {
        let max = match reseau {
            IpAddr::V4(_) => 32,
            IpAddr::V6(_) => 128,
        };
        if longueur > max {
            return None;
        }
        let p = Self { reseau, longueur };
        if p.contient(reseau) && masquer(reseau, longueur) == reseau {
            Some(p)
        } else {
            None
        }
    }

    /// Préfixe IPv4 depuis ses octets (raccourci).
    pub fn v4(a: u8, b: u8, c: u8, d: u8, longueur: u8) -> Option<Self> {
        Self::new(IpAddr::V4(Ipv4Addr::new(a, b, c, d)), longueur)
    }

    /// Vrai si `ip` appartient au préfixe (familles différentes : faux).
    pub fn contient(&self, ip: IpAddr) -> bool {
        match (self.reseau, ip) {
            (IpAddr::V4(_), IpAddr::V4(_)) | (IpAddr::V6(_), IpAddr::V6(_)) => {
                masquer(ip, self.longueur) == self.reseau
            }
            _ => false,
        }
    }
}

fn masquer(ip: IpAddr, longueur: u8) -> IpAddr {
    match ip {
        IpAddr::V4(v4) => {
            let bits = u32::from(v4);
            let masque = if longueur == 0 {
                0
            } else {
                u32::MAX << (32 - u32::from(longueur))
            };
            IpAddr::V4(Ipv4Addr::from(bits & masque))
        }
        IpAddr::V6(v6) => {
            let bits = u128::from(v6);
            let masque = if longueur == 0 {
                0
            } else {
                u128::MAX << (128 - u32::from(longueur))
            };
            IpAddr::V6(Ipv6Addr::from(bits & masque))
        }
    }
}

/// Plan d'adressage : préfixes déclarés par zone (le plus spécifique gagne).
#[derive(Debug, Clone, Default)]
pub struct PlanAdressage {
    entrees: Vec<(Prefixe, Zone)>,
}

impl PlanAdressage {
    /// Plan vide : toute adresse non-boucle est [`Zone::Exterieur`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Plan indicatif du modèle (§11) : COG `10.77.0.0/24`, Franche
    /// `10.77.1.0/24`, Franche locale `10.77.2.0/24`, Fédération `10.78.0.0/16`.
    ///
    /// Le Portail et BorderGuard n'ont pas d'adressage dans le document ; ils
    /// doivent être déclarés par l'appelant ([`PlanAdressage::declarer`]).
    pub fn indicatif() -> Self {
        let mut p = Self::new();
        // Les constantes ci-dessous sont valides par construction.
        if let Some(cog) = Prefixe::v4(10, 77, 0, 0, 24) {
            p.declarer(cog, Zone::Cog);
        }
        if let Some(franche) = Prefixe::v4(10, 77, 1, 0, 24) {
            p.declarer(franche, Zone::Franche);
        }
        if let Some(locale) = Prefixe::v4(10, 77, 2, 0, 24) {
            p.declarer(locale, Zone::FrancheLocale);
        }
        if let Some(fede) = Prefixe::v4(10, 78, 0, 0, 16) {
            p.declarer(fede, Zone::Federation);
        }
        p
    }

    /// Déclare un préfixe pour une zone.
    pub fn declarer(&mut self, prefixe: Prefixe, zone: Zone) {
        self.entrees.push((prefixe, zone));
    }

    /// Classe une adresse : préfixe déclaré le plus long, sinon [`Zone::Exterieur`].
    ///
    /// Les adresses de boucle locale ne sont **pas** classées ici : la boucle
    /// est locale à l'espace réseau de l'émetteur (voir [`est_boucle_locale`]).
    pub fn zone_de(&self, ip: IpAddr) -> Zone {
        self.entrees
            .iter()
            .filter(|(p, _)| p.contient(ip))
            .max_by_key(|(p, _)| p.longueur)
            .map_or(Zone::Exterieur, |(_, z)| *z)
    }
}

/// Vrai pour `127.0.0.0/8`, `::1` et `::ffff:127.0.0.0/104`.
pub fn est_boucle_locale(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => v4.is_loopback(),
        IpAddr::V6(v6) => {
            v6.is_loopback() || v6.to_ipv4_mapped().is_some_and(|v4| v4.is_loopback())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plan_indicatif_classe_les_zones_du_document() {
        let p = PlanAdressage::indicatif();
        assert_eq!(p.zone_de("10.77.0.5".parse().unwrap()), Zone::Cog);
        assert_eq!(p.zone_de("10.77.1.9".parse().unwrap()), Zone::Franche);
        assert_eq!(p.zone_de("10.78.3.4".parse().unwrap()), Zone::Federation);
        assert_eq!(p.zone_de("10.77.2.9".parse().unwrap()), Zone::FrancheLocale);
        assert_eq!(p.zone_de("8.8.8.8".parse().unwrap()), Zone::Exterieur);
        assert_eq!(p.zone_de("2001:db8::1".parse().unwrap()), Zone::Exterieur);
    }

    #[test]
    fn prefixe_le_plus_specifique_gagne() {
        let mut p = PlanAdressage::indicatif();
        p.declarer(Prefixe::v4(10, 77, 0, 250, 32).unwrap(), Zone::Portail);
        assert_eq!(p.zone_de("10.77.0.250".parse().unwrap()), Zone::Portail);
        assert_eq!(p.zone_de("10.77.0.251".parse().unwrap()), Zone::Cog);
    }

    #[test]
    fn prefixe_avec_bits_hote_refuse() {
        assert!(Prefixe::v4(10, 77, 0, 1, 24).is_none());
        assert!(Prefixe::v4(10, 0, 0, 0, 33).is_none());
    }

    #[test]
    fn boucle_locale() {
        assert!(est_boucle_locale("127.0.0.1".parse().unwrap()));
        assert!(est_boucle_locale("::1".parse().unwrap()));
        assert!(est_boucle_locale("::ffff:127.0.0.1".parse().unwrap()));
        assert!(!est_boucle_locale("0.0.0.0".parse().unwrap()));
        assert!(!est_boucle_locale("10.77.0.1".parse().unwrap()));
    }

    #[test]
    fn zone_analyse_stricte() {
        assert_eq!("cog".parse::<Zone>(), Ok(Zone::Cog));
        assert!("COG".parse::<Zone>().is_err());
        assert!("inconnue".parse::<Zone>().is_err());
    }
}
