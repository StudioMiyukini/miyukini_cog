//! @id     miyuvigie_nftables
//! @do     analyser_journaux_nftables
//! @role   securite
//! @layer  toolkit
//! @human  miyuvigie — module « nftables » : lecture stricte des journaux du pare-feu.

//! Lecteur strict des lignes de journal nftables (cible `log`) produites par
//! les règles générées par BorderGuard et appliquées par la Douane.
//!
//! # Convention de préfixe
//!
//! Chaque règle journalisée par les Vigies porte le préfixe :
//!
//! ```text
//! log prefix "MIYU:<verdict>:<zone_src>><zone_dst>:<voie|->: "
//! ```
//!
//! - `<verdict>` : `autorise` | `bloque`
//! - `<zone_src>`, `<zone_dst>` : `cog` | `portail` | `franche` | `federation` | `borderguard` | `ext`
//! - `<voie>` : identifiant de voie (`[A-Za-z0-9._-]`), ou `-` si le flux n'appartient à aucune voie
//! - le préfixe se termine par `": "` (deux-points puis espace) ; il tient dans les
//!   127 octets autorisés par le noyau
//!
//! # Format de ligne attendu
//!
//! Sortie de `journalctl -k -o short-unix` :
//!
//! ```text
//! 1729760000.123456 hote kernel: MIYU:bloque:cog>ext:jaymail: IN=veth-cog OUT=veth-bg SRC=10.77.0.5 DST=93.184.216.34 LEN=60 TTL=64 DF PROTO=TCP SPT=40000 DPT=443 SYN
//! ```
//!
//! Analyse **stricte** : un seul espace entre jetons, clés `[A-Z0-9]+`, clé en
//! double refusée, champs `IN`, `OUT`, `SRC`, `DST`, `LEN`, `PROTO` obligatoires,
//! `DPT` obligatoire pour TCP/UDP. Les zones du préfixe sont recoupées avec le
//! [`PlanAdressage`] : tout désaccord rend la ligne **incohérente**.
//!
//! Une ligne mal formée ou incohérente est ignorée et **comptée** ; elle n'est
//! jamais une preuve bénigne et ne vaut pas preuve de vie du capteur.

use std::collections::{HashMap, HashSet};
use std::net::IpAddr;

use crate::evenement::{EvenementFlux, Protocole, Verdict};
use crate::signal::{EvenementId, VoieId};
use crate::zone::{est_boucle_locale, PlanAdressage, Zone};

/// Marqueur de début du préfixe des Vigies.
pub const MARQUEUR_PREFIXE: &str = "MIYU:";

/// Longueur maximale d'une ligne acceptée (octets).
pub const LONGUEUR_MAX_LIGNE: usize = 4096;

/// Résultat de l'analyse d'une ligne.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResultatLigne {
    /// Ligne valide : événement de flux.
    Evenement(EvenementFlux),
    /// Ligne sans le marqueur `MIYU:` (pas un journal des Vigies).
    HorsConvention,
    /// Ligne portant le marqueur mais mal formée.
    MalFormee(&'static str),
    /// Ligne bien formée mais contradictoire (zones préfixe ≠ adresses).
    Incoherente(&'static str),
}

/// Lecteur de journaux nftables.
#[derive(Debug, Clone)]
pub struct LecteurNftables {
    plan: PlanAdressage,
}

impl LecteurNftables {
    /// Crée un lecteur avec le plan d'adressage servant au recoupement des zones.
    pub fn new(plan: PlanAdressage) -> Self {
        Self { plan }
    }

    /// Analyse une ligne.
    pub fn analyser(&self, ligne: &str) -> ResultatLigne {
        if !ligne.contains(MARQUEUR_PREFIXE) {
            return ResultatLigne::HorsConvention;
        }
        match self.analyser_stricte(ligne) {
            Ok(evt) => ResultatLigne::Evenement(evt),
            Err(Rejet::MalFormee(m)) => ResultatLigne::MalFormee(m),
            Err(Rejet::Incoherente(m)) => ResultatLigne::Incoherente(m),
        }
    }

    fn analyser_stricte(&self, ligne: &str) -> Result<EvenementFlux, Rejet> {
        use Rejet::MalFormee as M;
        if ligne.len() > LONGUEUR_MAX_LIGNE {
            return Err(M("ligne trop longue"));
        }
        let ligne = ligne.strip_suffix('\n').unwrap_or(ligne);
        if !ligne.is_ascii() || ligne.bytes().any(|b| b.is_ascii_control()) {
            return Err(M("caractère non ASCII ou de contrôle"));
        }
        let jetons: Vec<&str> = ligne.split(' ').collect();
        if jetons.iter().any(|j| j.is_empty()) {
            return Err(M("espacement irrégulier"));
        }
        if jetons.len() < 5 {
            return Err(M("ligne tronquée"));
        }
        let horodatage_ms = analyser_horodatage(jetons[0]).ok_or(M("horodatage invalide"))?;
        if !jetons[1]
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'.' | b'_'))
        {
            return Err(M("nom d'hôte invalide"));
        }
        if jetons[2] != "kernel:" {
            return Err(M("émetteur différent de « kernel: »"));
        }
        let prefixe = analyser_prefixe(jetons[3])?;

        let champs = analyser_champs(&jetons[4..])?;
        for requis in ["IN", "OUT", "SRC", "DST", "LEN", "PROTO"] {
            if !champs.contains_key(requis) {
                return Err(M("champ obligatoire absent"));
            }
        }
        let ip_src: IpAddr = champs["SRC"].parse().map_err(|_| M("SRC invalide"))?;
        let ip_dst: IpAddr = champs["DST"].parse().map_err(|_| M("DST invalide"))?;
        let octets: u64 = champs["LEN"].parse().map_err(|_| M("LEN invalide"))?;
        let protocole = match champs["PROTO"] {
            "TCP" => Protocole::Tcp,
            "UDP" => Protocole::Udp,
            "ICMP" | "ICMPv6" => Protocole::Icmp,
            n => Protocole::Autre(n.parse().map_err(|_| M("PROTO invalide"))?),
        };
        let port_dst = match champs.get("DPT") {
            Some(p) => Some(p.parse::<u16>().map_err(|_| M("DPT invalide"))?),
            None if matches!(protocole, Protocole::Tcp | Protocole::Udp) => {
                return Err(M("DPT absent pour TCP/UDP"))
            }
            None => None,
        };

        // Recoupement : les zones du préfixe doivent correspondre aux adresses.
        if self.zone(ip_src) != prefixe.zone_src {
            return Err(Rejet::Incoherente("zone source du préfixe ≠ adresse SRC"));
        }
        if self.zone(ip_dst) != prefixe.zone_dst {
            return Err(Rejet::Incoherente(
                "zone destination du préfixe ≠ adresse DST",
            ));
        }

        let id = EvenementId::new(format!(
            "nft-{}-{:016x}",
            jetons[0],
            fnv1a(ligne.as_bytes())
        ))
        .ok_or(M("identifiant d'événement"))?;
        Ok(EvenementFlux {
            id,
            horodatage_ms,
            zone_src: prefixe.zone_src,
            zone_dst: prefixe.zone_dst,
            voie: prefixe.voie,
            verdict: prefixe.verdict,
            octets,
            ip_src: Some(ip_src),
            ip_dst: Some(ip_dst),
            port_dst,
            protocole: Some(protocole),
        })
    }

    /// Zone d'une adresse ; la boucle locale est rattachée à la zone COG
    /// (journaux relevés dans l'espace réseau de la zone COG).
    fn zone(&self, ip: IpAddr) -> Zone {
        if est_boucle_locale(ip) {
            Zone::Cog
        } else {
            self.plan.zone_de(ip)
        }
    }
}

enum Rejet {
    MalFormee(&'static str),
    Incoherente(&'static str),
}

/// Jetons `CLE=valeur` ou drapeaux `CLE` ; clé en double refusée.
fn analyser_champs<'a>(jetons: &[&'a str]) -> Result<HashMap<&'a str, &'a str>, Rejet> {
    use Rejet::MalFormee as M;
    let mut vus = HashSet::new();
    let mut champs = HashMap::new();
    for j in jetons {
        let (cle, valeur) = match j.split_once('=') {
            Some((c, v)) => (c, Some(v)),
            None => (*j, None),
        };
        if cle.is_empty()
            || !cle
                .bytes()
                .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
        {
            return Err(M("clé invalide"));
        }
        if !vus.insert(cle) {
            return Err(M("clé en double"));
        }
        if let Some(v) = valeur {
            if !v
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b':' | b'-' | b'_'))
            {
                return Err(M("valeur invalide"));
            }
            champs.insert(cle, v);
        }
    }
    Ok(champs)
}

struct Prefixe {
    verdict: Verdict,
    zone_src: Zone,
    zone_dst: Zone,
    voie: Option<VoieId>,
}

fn analyser_prefixe(jeton: &str) -> Result<Prefixe, Rejet> {
    use Rejet::MalFormee as M;
    let reste = jeton
        .strip_prefix(MARQUEUR_PREFIXE)
        .ok_or(M("préfixe absent en 4e position"))?;
    let parties: Vec<&str> = reste.split(':').collect();
    // verdict, zones, voie, "" (le préfixe finit par ':')
    if parties.len() != 4 || !parties[3].is_empty() {
        return Err(M("préfixe : nombre de champs"));
    }
    let verdict = Verdict::depuis_nom(parties[0]).ok_or(M("préfixe : verdict"))?;
    let (src, dst) = parties[1].split_once('>').ok_or(M("préfixe : zones"))?;
    let zone_src = src
        .parse::<Zone>()
        .map_err(|()| M("préfixe : zone source"))?;
    let zone_dst = dst
        .parse::<Zone>()
        .map_err(|()| M("préfixe : zone destination"))?;
    let voie = if parties[2] == "-" {
        None
    } else {
        Some(VoieId::new(parties[2]).ok_or(M("préfixe : voie"))?)
    };
    Ok(Prefixe {
        verdict,
        zone_src,
        zone_dst,
        voie,
    })
}

/// `secondes.microsecondes` (6 chiffres exactement) → millisecondes.
fn analyser_horodatage(s: &str) -> Option<u64> {
    let (sec, micro) = s.split_once('.')?;
    if sec.is_empty() || micro.len() != 6 {
        return None;
    }
    if !sec.bytes().all(|b| b.is_ascii_digit()) || !micro.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let sec: u64 = sec.parse().ok()?;
    let micro: u64 = micro.parse().ok()?;
    sec.checked_mul(1000)?.checked_add(micro / 1000)
}

/// FNV-1a 64 bits (identifiant d'événement, non cryptographique).
pub(crate) fn fnv1a(octets: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &o in octets {
        h ^= u64::from(o);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIGNE: &str = "1729760000.123456 hote kernel: MIYU:bloque:cog>ext:jaymail: IN=veth-cog OUT=veth-bg SRC=10.77.0.5 DST=93.184.216.34 LEN=60 TTL=64 DF PROTO=TCP SPT=40000 DPT=443 SYN";

    fn lecteur() -> LecteurNftables {
        LecteurNftables::new(PlanAdressage::indicatif())
    }

    #[test]
    fn ligne_valide() {
        let ResultatLigne::Evenement(e) = lecteur().analyser(LIGNE) else {
            panic!("ligne valide refusée");
        };
        assert_eq!(e.horodatage_ms, 1_729_760_000_123);
        assert_eq!(e.zone_src, Zone::Cog);
        assert_eq!(e.zone_dst, Zone::Exterieur);
        assert_eq!(e.verdict, Verdict::Bloque);
        assert_eq!(e.voie.as_ref().map(VoieId::as_str), Some("jaymail"));
        assert_eq!(e.octets, 60);
        assert_eq!(e.port_dst, Some(443));
    }

    #[test]
    fn hors_convention() {
        assert_eq!(
            lecteur().analyser("1729760000.123456 hote kernel: usb 1-1: new device"),
            ResultatLigne::HorsConvention
        );
    }

    #[test]
    fn lignes_mal_formees() {
        let l = lecteur();
        let cas = [
            LIGNE.replace("1729760000.123456", "1729760000.12"),
            LIGNE.replace(" SRC=10.77.0.5", ""),
            LIGNE.replace("SRC=10.77.0.5", "SRC=10.77.0.5 SRC=10.77.0.6"),
            LIGNE.replace("MIYU:bloque:", "MIYU:permis:"),
            LIGNE.replace(":jaymail:", ":jay/mail:"),
            LIGNE.replace("cog>ext", "cog-ext"),
            LIGNE.replace("LEN=60", "LEN=-1"),
            LIGNE.replace(" DPT=443", ""),
            LIGNE.replace("kernel:", "sshd:"),
            LIGNE.replace(" IN=", "  IN="),
            LIGNE.replace("DST=93.184.216.34", "DST=93.184.216.999"),
            format!("{LIGNE}\u{7}"),
            "MIYU:".to_string(),
            "x".repeat(LONGUEUR_MAX_LIGNE + 1) + MARQUEUR_PREFIXE,
        ];
        for c in cas {
            assert!(
                matches!(l.analyser(&c), ResultatLigne::MalFormee(_)),
                "acceptée à tort : {c}"
            );
        }
    }

    #[test]
    fn zones_incoherentes() {
        // Le préfixe prétend « cog>cog » mais la destination est extérieure.
        let l = LIGNE.replace("cog>ext", "cog>cog");
        assert!(matches!(
            lecteur().analyser(&l),
            ResultatLigne::Incoherente(_)
        ));
    }

    #[test]
    fn identifiant_stable() {
        let a = lecteur().analyser(LIGNE);
        let b = lecteur().analyser(LIGNE);
        assert_eq!(a, b);
    }
}
