//! @id     borderguard_zone
//! @do     definir_zones_frontieres
//! @role   securite
//! @layer  core
//! @human  borderguard — zones, frontières, réseaux et protocoles des arrêtés.

//! Vocabulaire des arrêtés (modèle zones et couches v1.3, §2 à §6).
//!
//! - [`Zone`] : zones intérieures (COG, Franche, Franche locale) et origines
//!   extérieures (Fédération, Origin, International).
//! - [`Frontiere`] : une frontière = une instance Douane (INV-Z9, EX-19).
//! - [`Reseau`] : préfixe IP canonique (bits d'hôte nuls), pour les listes
//!   blanches de pairs.
//! - [`Protocole`] : TCP ou UDP (numéros IANA).
//!
//! Tous les codes binaires sont figés : ils font partie du format signé.

use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

/// Zone (intérieure) ou origine (extérieure) du trafic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Zone {
    /// Zone COG (nationale) = LAN : Kernel, Cores, Services, DB principale.
    Cog = 1,
    /// Zone Franche = DMZ : façades, relais, DB secondaires.
    Franche = 2,
    /// Franche locale = seconde DMZ, LAN domestique uniquement (domotique).
    FrancheLocale = 3,
    /// Fédération : COG reconnus et appareils de l'utilisateur (WAN privé).
    Federation = 4,
    /// Origin : autorité globale, source du canal officiel.
    Origin = 5,
    /// International : Internet hors fédération, zero trust.
    International = 6,
}

impl Zone {
    /// Toutes les zones, dans l'ordre des codes.
    pub const TOUTES: [Self; 6] = [
        Self::Cog,
        Self::Franche,
        Self::FrancheLocale,
        Self::Federation,
        Self::Origin,
        Self::International,
    ];

    /// Code binaire signé.
    pub const fn code(self) -> u8 {
        self as u8
    }

    /// Zone depuis son code (refus de tout code inconnu).
    pub fn depuis_code(code: u8) -> Option<Self> {
        Self::TOUTES.into_iter().find(|z| z.code() == code)
    }

    /// Nom stable (journaux, préfixes nftables).
    pub const fn nom(self) -> &'static str {
        match self {
            Self::Cog => "cog",
            Self::Franche => "franche",
            Self::FrancheLocale => "locale",
            Self::Federation => "federation",
            Self::Origin => "origin",
            Self::International => "international",
        }
    }

    /// Vrai pour les zones intérieures au COG.
    pub const fn est_interieure(self) -> bool {
        matches!(self, Self::Cog | Self::Franche | Self::FrancheLocale)
    }

    /// Vrai pour les origines extérieures (Fédération, Origin, International).
    pub const fn est_exterieure(self) -> bool {
        !self.est_interieure()
    }
}

impl fmt::Display for Zone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.nom())
    }
}

/// Frontière gardée par une instance Douane dédiée (INV-Z9).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Frontiere {
    /// BorderGuard externe : origines extérieures ⇄ Franche (routage, IPS).
    Externe = 1,
    /// Portail interne : zone COG → Franche (rupture protocolaire, la zone COG tire).
    PortailInterne = 2,
    /// Portail local : zone COG → Franche locale (rupture protocolaire, la zone COG tire).
    PortailLocal = 3,
}

impl Frontiere {
    /// Toutes les frontières.
    pub const TOUTES: [Self; 3] = [Self::Externe, Self::PortailInterne, Self::PortailLocal];

    /// Code binaire signé.
    pub const fn code(self) -> u8 {
        self as u8
    }

    /// Frontière depuis son code.
    pub fn depuis_code(code: u8) -> Option<Self> {
        Self::TOUTES.into_iter().find(|f| f.code() == code)
    }

    /// Nom stable.
    pub const fn nom(self) -> &'static str {
        match self {
            Self::Externe => "externe",
            Self::PortailInterne => "portail-interne",
            Self::PortailLocal => "portail-local",
        }
    }

    /// Couples `(initiateur, destinataire)` admis par la constitution réseau
    /// sur cette frontière. Tout autre couple est hors constitution.
    ///
    /// - Externe : extérieur ⇄ Franche, dans les deux sens (jamais la zone COG
    ///   ni la Franche locale : INV-Z1, §3).
    /// - Portails : **seule la zone COG ouvre** (INV-Z2, INV-Z7).
    pub fn couple_admis(self, source: Zone, destination: Zone) -> bool {
        match self {
            Self::Externe => {
                (source.est_exterieure() && destination == Zone::Franche)
                    || (source == Zone::Franche && destination.est_exterieure())
            }
            Self::PortailInterne => source == Zone::Cog && destination == Zone::Franche,
            Self::PortailLocal => source == Zone::Cog && destination == Zone::FrancheLocale,
        }
    }
}

impl fmt::Display for Frontiere {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.nom())
    }
}

impl FromStr for Frontiere {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        Self::TOUTES.into_iter().find(|f| f.nom() == s).ok_or(())
    }
}

/// Protocole de transport d'un flux.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum Protocole {
    /// TCP (IANA 6).
    Tcp = 6,
    /// UDP (IANA 17).
    Udp = 17,
}

impl Protocole {
    /// Code binaire signé (numéro IANA).
    pub const fn code(self) -> u8 {
        self as u8
    }

    /// Protocole depuis son code.
    pub fn depuis_code(code: u8) -> Option<Self> {
        match code {
            6 => Some(Self::Tcp),
            17 => Some(Self::Udp),
            _ => None,
        }
    }

    /// Mot-clé nftables.
    pub const fn nom(self) -> &'static str {
        match self {
            Self::Tcp => "tcp",
            Self::Udp => "udp",
        }
    }
}

impl fmt::Display for Protocole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.nom())
    }
}

/// Préfixe IP canonique (bits d'hôte nuls).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Reseau {
    adresse: IpAddr,
    prefixe: u8,
}

impl Reseau {
    /// Préfixe canonique ; refus si le préfixe dépasse la famille ou si des
    /// bits d'hôte sont posés (une seule écriture par réseau).
    pub fn nouveau(adresse: IpAddr, prefixe: u8) -> Option<Self> {
        let max = match adresse {
            IpAddr::V4(_) => 32,
            IpAddr::V6(_) => 128,
        };
        if prefixe > max {
            return None;
        }
        let r = Self { adresse, prefixe };
        (r.masquer(adresse) == adresse).then_some(r)
    }

    /// Hôte unique (/32 ou /128).
    pub fn hote(adresse: IpAddr) -> Self {
        let prefixe = match adresse {
            IpAddr::V4(_) => 32,
            IpAddr::V6(_) => 128,
        };
        Self { adresse, prefixe }
    }

    /// Adresse de réseau.
    pub const fn adresse(&self) -> IpAddr {
        self.adresse
    }

    /// Longueur du préfixe.
    pub const fn prefixe(&self) -> u8 {
        self.prefixe
    }

    /// Vrai pour IPv4.
    pub const fn est_v4(&self) -> bool {
        matches!(self.adresse, IpAddr::V4(_))
    }

    fn masquer(&self, ip: IpAddr) -> IpAddr {
        match ip {
            IpAddr::V4(a) => {
                let bits = u32::from(a);
                let masque = if self.prefixe == 0 { 0 } else { u32::MAX << (32 - u32::from(self.prefixe)) };
                IpAddr::V4(Ipv4Addr::from(bits & masque))
            }
            IpAddr::V6(a) => {
                let bits = u128::from(a);
                let masque = if self.prefixe == 0 { 0 } else { u128::MAX << (128 - u32::from(self.prefixe)) };
                IpAddr::V6(Ipv6Addr::from(bits & masque))
            }
        }
    }

    /// Vrai si `ip` appartient au réseau (même famille obligatoire).
    pub fn contient(&self, ip: IpAddr) -> bool {
        match (self.adresse, ip) {
            (IpAddr::V4(_), IpAddr::V4(_)) | (IpAddr::V6(_), IpAddr::V6(_)) => self.masquer(ip) == self.adresse,
            _ => false,
        }
    }

    /// Encodage binaire canonique : famille (4 ou 6), adresse, préfixe.
    pub fn encoder(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(18);
        match self.adresse {
            IpAddr::V4(a) => {
                v.push(4);
                v.extend_from_slice(&a.octets());
            }
            IpAddr::V6(a) => {
                v.push(6);
                v.extend_from_slice(&a.octets());
            }
        }
        v.push(self.prefixe);
        v
    }

    /// Décode un réseau en tête de `octets` ; renvoie le réseau et le reste.
    pub fn decoder(octets: &[u8]) -> Option<(Self, &[u8])> {
        let (&famille, reste) = octets.split_first()?;
        let (adresse, reste) = match famille {
            4 => {
                let a: [u8; 4] = reste.get(..4)?.try_into().ok()?;
                (IpAddr::V4(Ipv4Addr::from(a)), &reste[4..])
            }
            6 => {
                let a: [u8; 16] = reste.get(..16)?.try_into().ok()?;
                (IpAddr::V6(Ipv6Addr::from(a)), &reste[16..])
            }
            _ => return None,
        };
        let (&prefixe, reste) = reste.split_first()?;
        Some((Self::nouveau(adresse, prefixe)?, reste))
    }
}

impl fmt::Display for Reseau {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.adresse, self.prefixe)
    }
}

impl FromStr for Reseau {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let (a, p) = s.split_once('/').ok_or(())?;
        let adresse: IpAddr = a.parse().map_err(|_| ())?;
        let prefixe: u8 = p.parse().map_err(|_| ())?;
        Self::nouveau(adresse, prefixe).ok_or(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reseau_canonique() {
        assert!("10.77.0.0/24".parse::<Reseau>().is_ok());
        assert!("10.77.0.1/24".parse::<Reseau>().is_err());
        assert!("10.77.0.0/33".parse::<Reseau>().is_err());
        let r: Reseau = "10.77.0.0/24".parse().unwrap();
        assert!(r.contient("10.77.0.200".parse().unwrap()));
        assert!(!r.contient("10.77.1.1".parse().unwrap()));
        assert!(!r.contient("::1".parse().unwrap()));
        let enc = r.encoder();
        let (d, reste) = Reseau::decoder(&enc).unwrap();
        assert_eq!(d, r);
        assert_eq!(reste, &[] as &[u8]);
        let v6: Reseau = "fd00::/8".parse().unwrap();
        let enc6 = v6.encoder();
        assert_eq!(Reseau::decoder(&enc6).unwrap().0, v6);
    }

    #[test]
    fn la_zone_cog_tire() {
        for f in Frontiere::TOUTES {
            for z in Zone::TOUTES {
                // Personne n'ouvre vers la zone COG (INV-Z2).
                assert!(!f.couple_admis(z, Zone::Cog));
            }
        }
        // Aucun couple zone COG ⇄ extérieur (INV-Z1), ni extérieur ⇄ extérieur (INV-Z3).
        for a in Zone::TOUTES {
            for b in Zone::TOUTES {
                if (a == Zone::Cog && b.est_exterieure()) || (a.est_exterieure() && b.est_exterieure()) {
                    assert!(Frontiere::TOUTES.iter().all(|f| !f.couple_admis(a, b)));
                }
            }
        }
        assert!(Frontiere::PortailInterne.couple_admis(Zone::Cog, Zone::Franche));
        assert!(!Frontiere::Externe.couple_admis(Zone::International, Zone::FrancheLocale));
    }
}
