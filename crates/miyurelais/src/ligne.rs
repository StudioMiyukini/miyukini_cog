//! @id     miyurelais_ligne
//! @do     convertir_paquet_nflog_en_ligne_mdou
//! @role   securite
//! @layer  toolkit
//! @human  miyurelais — conversion d'un paquet NFLOG décodé en ligne canonique `MDOU|` (Vigies) et en métadonnée de flux (IPS).

//! # Format de ligne du relais (à consigner par amendement)
//!
//! Une ligne par paquet journalisé, ASCII imprimable, jetons séparés par **un**
//! espace, sans espace final :
//!
//! ```text
//! <sec>.<µs6> <hôte> nflog-<G>: <préfixe> IN=<if> OUT=<if> SRC=<ip> DST=<ip> LEN=<n> TTL=<n> PROTO=<p> [SPT=<n> DPT=<n>] [TYPE=<n> CODE=<n>] [<drapeaux TCP>] [FRAG|TRONQUE] [MARK=0x<hex>]
//! ```
//!
//! | jeton | origine (attribut NFLOG) | règle |
//! |---|---|---|
//! | `<sec>.<µs6>` | `NFULA_TIMESTAMP` si présent et à moins de [`ECART_HORODATAGE_MAX_US`] de l'horloge du relais, sinon horloge du relais à la réception | strictement croissant par relais (+1 µs si besoin) |
//! | `<hôte>` | configuration | `[A-Za-z0-9._-]{1,64}` |
//! | `nflog-<G>:` | `nfgenmsg.res_id` | décimal sans zéro initial |
//! | `<préfixe>` | `NFULA_PREFIX` | octets `0x21..=0x7e` seulement (aucun espace) ; sinon paquet refusé |
//! | `IN=` / `OUT=` | famille pont : `NFULA_IFINDEX_PHYSINDEV` / `_PHYSOUTDEV` (le **port** `sv<n>`, ce que `iifname` / `oifname` filtrent ; le noyau met le pont lui-même dans `_INDEV` / `_OUTDEV`), à défaut `_INDEV` / `_OUTDEV` ; autres familles : `NFULA_IFINDEX_INDEV` / `_OUTDEV` — nom résolu dans l'espace réseau du relais | vide si absent ; nom `[A-Za-z0-9._:-]{1,15}`, sinon `if<index>` |
//! | `SRC` `DST` `LEN` `TTL` | en-tête IPv4 / IPv6 de `NFULA_PAYLOAD` | `LEN` = longueur annoncée par l'en-tête, pas la longueur copiée |
//! | `PROTO` | idem | `TCP`, `UDP`, `ICMP`, `ICMPv6` ou numéro décimal ; TCP / UDP **sans en-tête de transport lisible** : numéro décimal (`6`, `17`) |
//! | `SPT` `DPT` | en-tête TCP / UDP | présents dès que `PROTO=TCP` / `UDP` |
//! | `FRAG` / `TRONQUE` | — | TCP / UDP sans ports : `FRAG` fragment non initial, `TRONQUE` en-tête de transport hors de la copie (troncature, chaîne d'extensions IPv6 longue) ; la ligne est **émise** (amendement à consigner : avant, paquet refusé ⇒ capteur dégradé, provocable à volonté) |
//! | `TYPE` `CODE` | en-tête ICMP | si présents |
//! | drapeaux | octet 13 TCP | `SYN` `ACK` `FIN` `RST` `PSH` `URG`, dans cet ordre |
//! | `MARK` | `NFULA_MARK` | seulement si ≠ 0 |
//!
//! C'est exactement le format lu par `miyuvigie::LecteurNftables` (préfixe en
//! 4e position, clés `[A-Z0-9]+`, valeurs `[A-Za-z0-9.:_-]`).
//!
//! # Témoin de vie
//!
//! Un paquet dont le préfixe commence par [`PREFIXE_TEMOIN`]
//! (`MDOU-TEMOIN|<lieu>|`) est une **preuve de vie** de la voie journalisée
//! (règle `log` limitée en débit, côté acceptation) : il nourrit la veille du
//! capteur muet et ne produit **aucune** ligne.
//!
//! Familles : IPv4 (2), IPv6 (10), pont (7, protocole déduit de
//! `hw_protocol` puis vérifié sur la version de l'en-tête). Toute autre
//! famille est refusée (ARP, netdev sans IP…).

use std::fmt::Write as _;
use std::net::IpAddr;

use crate::ip::{resumer_ipv4, resumer_ipv6, ResumeIp, Transport, IPPROTO_ICMP, IPPROTO_ICMPV6, IPPROTO_TCP, IPPROTO_UDP};
use crate::netlink::PaquetNflog;

/// Préfixe d'un paquet témoin (preuve de vie, aucune ligne produite).
pub const PREFIXE_TEMOIN: &str = "MDOU-TEMOIN|";

/// Écart maximal admis entre l'horodatage du noyau et l'horloge du relais (µs).
pub const ECART_HORODATAGE_MAX_US: u64 = 5_000_000;

/// `NFPROTO_IPV4`.
pub const NFPROTO_IPV4: u8 = 2;
/// `NFPROTO_BRIDGE`.
pub const NFPROTO_BRIDGE: u8 = 7;
/// `NFPROTO_IPV6`.
pub const NFPROTO_IPV6: u8 = 10;

/// Refus de conversion d'un paquet (compté « mal formé » par la Vigie).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefusPaquet {
    /// Paquet sans préfixe.
    SansPrefixe,
    /// Préfixe hors `0x21..=0x7e`.
    PrefixeInvalide,
    /// Famille non IP.
    Famille(u8),
    /// En-tête IP invalide ou tronqué.
    EnTeteIp,
}

/// Paquet converti.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Conversion {
    /// Ligne canonique pour les Vigies.
    Ligne(String),
    /// Témoin de vie : aucune ligne.
    Temoin,
}

/// Vrai si `s` est un nom d'hôte admis dans la ligne.
pub fn hote_valide(s: &str) -> bool {
    !s.is_empty() && s.len() <= 64 && s.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-' | b'_'))
}

/// Vrai si `s` est un nom d'interface admis dans la ligne.
pub fn nom_interface_valide(s: &str) -> bool {
    !s.is_empty() && s.len() <= 15 && s.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b':' | b'-' | b'_'))
}

/// Index des interfaces (entrée, sortie) écrits dans la ligne : en famille
/// pont, les ports physiques (`sv<n>`) plutôt que le pont.
pub fn interfaces(p: &PaquetNflog) -> (Option<u32>, Option<u32>) {
    if p.famille == NFPROTO_BRIDGE {
        (
            p.interface_entree_physique.or(p.interface_entree),
            p.interface_sortie_physique.or(p.interface_sortie),
        )
    } else {
        (p.interface_entree, p.interface_sortie)
    }
}

/// Résume l'en-tête IP de la charge utile selon la famille.
pub fn resumer(p: &PaquetNflog) -> Result<ResumeIp, RefusPaquet> {
    let version = p.charge.first().map(|o| o >> 4);
    let r = match (p.famille, version) {
        (NFPROTO_IPV4, Some(4)) => resumer_ipv4(&p.charge),
        (NFPROTO_IPV6, Some(6)) => resumer_ipv6(&p.charge),
        (NFPROTO_BRIDGE, Some(4)) if p.protocole_l2 == Some(0x0800) => resumer_ipv4(&p.charge),
        (NFPROTO_BRIDGE, Some(6)) if p.protocole_l2 == Some(0x86dd) => resumer_ipv6(&p.charge),
        (NFPROTO_IPV4 | NFPROTO_IPV6 | NFPROTO_BRIDGE, _) => None,
        (f, _) => return Err(RefusPaquet::Famille(f)),
    };
    r.ok_or(RefusPaquet::EnTeteIp)
}

/// Contexte de conversion fourni par le relais.
pub struct Contexte<'a> {
    /// Nom d'hôte (validé par [`hote_valide`]).
    pub hote: &'a str,
    /// Horodatage retenu (µs depuis l'époque Unix).
    pub horodatage_us: u64,
    /// Nom de l'interface d'entrée (vide si absente).
    pub entree: &'a str,
    /// Nom de l'interface de sortie (vide si absente).
    pub sortie: &'a str,
}

fn nom_protocole(p: u8) -> String {
    match p {
        IPPROTO_TCP => "TCP".into(),
        IPPROTO_UDP => "UDP".into(),
        IPPROTO_ICMP => "ICMP".into(),
        IPPROTO_ICMPV6 => "ICMPv6".into(),
        n => n.to_string(),
    }
}

/// Convertit un paquet décodé en ligne canonique (ou témoin).
pub fn convertir(p: &PaquetNflog, ctx: &Contexte<'_>) -> Result<Conversion, RefusPaquet> {
    let prefixe = p.prefixe.as_deref().ok_or(RefusPaquet::SansPrefixe)?;
    if prefixe.is_empty() || !prefixe.iter().all(|b| (0x21..=0x7e).contains(b)) {
        return Err(RefusPaquet::PrefixeInvalide);
    }
    if prefixe.starts_with(PREFIXE_TEMOIN.as_bytes()) {
        return Ok(Conversion::Temoin);
    }
    let prefixe = std::str::from_utf8(prefixe).map_err(|_| RefusPaquet::PrefixeInvalide)?;
    let r = resumer(p)?;
    // TCP / UDP sans ports : protocole en numéro (le lecteur des Vigies n'exige
    // alors pas DPT), marqué FRAG / TRONQUE.
    let sans_ports = matches!(r.transport, Transport::Absent) && matches!(r.protocole, IPPROTO_TCP | IPPROTO_UDP);
    let protocole = if sans_ports { r.protocole.to_string() } else { nom_protocole(r.protocole) };
    let mut l = String::with_capacity(160);
    // Écritures dans une String : infaillibles.
    let _ = write!(
        l,
        "{}.{:06} {} nflog-{}: {} IN={} OUT={} SRC={} DST={} LEN={} TTL={} PROTO={}",
        ctx.horodatage_us / 1_000_000,
        ctx.horodatage_us % 1_000_000,
        ctx.hote,
        p.groupe,
        prefixe,
        ctx.entree,
        ctx.sortie,
        r.source,
        r.destination,
        r.longueur,
        r.ttl,
        protocole,
    );
    match r.transport {
        Transport::Tcp { port_source, port_destination, drapeaux } => {
            let _ = write!(l, " SPT={port_source} DPT={port_destination}");
            if let Some(d) = drapeaux {
                for (bit, nom) in [(0x02, "SYN"), (0x10, "ACK"), (0x01, "FIN"), (0x04, "RST"), (0x08, "PSH"), (0x20, "URG")] {
                    if d & bit != 0 {
                        l.push(' ');
                        l.push_str(nom);
                    }
                }
            }
        }
        Transport::Udp { port_source, port_destination } => {
            let _ = write!(l, " SPT={port_source} DPT={port_destination}");
        }
        Transport::Icmp { type_, code } => {
            let _ = write!(l, " TYPE={type_} CODE={code}");
        }
        Transport::Absent if sans_ports => {
            l.push_str(if r.fragment_non_initial { " FRAG" } else { " TRONQUE" });
        }
        Transport::Absent => {}
    }
    if let Some(m) = p.marque.filter(|m| *m != 0) {
        let _ = write!(l, " MARK=0x{m:x}");
    }
    Ok(Conversion::Ligne(l))
}

/// Métadonnée de flux pour l'IPS de la Douane (`miyudouane::SourceFlux`).
///
/// TCP / UDP seulement (les arrêtés ne portent que sur eux). Connexion
/// nouvelle = TCP `SYN` sans `ACK` ; UDP jamais (NFLOG ne porte pas l'état
/// conntrack demandé : l'IPS traitera le datagramme hors état, sans
/// bannissement ni attribution de voie — sens prudent).
pub fn meta_flux(p: &PaquetNflog, horodatage_secs: u64, entree: Option<&str>) -> Option<miyudouane::MetaFlux> {
    let r = resumer(p).ok()?;
    let (protocole, port_source, port_destination, nouvelle) = match r.transport {
        Transport::Tcp { port_source, port_destination, drapeaux } => (
            borderguard::Protocole::Tcp,
            port_source,
            port_destination,
            drapeaux.is_some_and(|d| d & 0x12 == 0x02),
        ),
        Transport::Udp { port_source, port_destination } => {
            (borderguard::Protocole::Udp, port_source, port_destination, false)
        }
        Transport::Icmp { .. } | Transport::Absent => return None,
    };
    Some(miyudouane::MetaFlux {
        horodatage_secs,
        source: r.source,
        destination: r.destination,
        protocole,
        port_source,
        port_destination,
        taille: r.longueur,
        nouvelle_connexion: nouvelle,
        interface_entree: entree.filter(|n| nom_interface_valide(n)).map(str::to_owned),
    })
}

/// Adresses du paquet (diagnostic).
pub fn adresses(p: &PaquetNflog) -> Option<(IpAddr, IpAddr)> {
    resumer(p).ok().map(|r| (r.source, r.destination))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ip::tests::ipv4_tcp;
    use miyuvigie::{LecteurNftables, PlanAdressage, ResultatLigne, VoieId};

    fn paquet(prefixe: &str, charge: Vec<u8>) -> PaquetNflog {
        PaquetNflog {
            famille: NFPROTO_BRIDGE,
            groupe: 111,
            protocole_l2: Some(0x0800),
            crochet: Some(2),
            marque: None,
            horodatage_us: None,
            interface_entree: Some(3),
            interface_sortie: Some(4),
            interface_entree_physique: None,
            interface_sortie_physique: None,
            prefixe: Some(prefixe.as_bytes().to_vec()),
            sequence: Some(0),
            charge,
        }
    }

    fn ctx<'a>() -> Contexte<'a> {
        Contexte { hote: "essai", horodatage_us: 1_729_760_000_000_042, entree: "sv0", sortie: "sv1" }
    }

    #[test]
    fn ligne_lue_par_les_vigies() {
        let p = paquet("MDOU|zone-franche|transit|NON-DECLARE|", ipv4_tcp([10, 77, 1, 10], [10, 77, 1, 20], 9999, 0x02));
        let Ok(Conversion::Ligne(l)) = convertir(&p, &ctx()) else { panic!() };
        assert_eq!(
            l,
            "1729760000.000042 essai nflog-111: MDOU|zone-franche|transit|NON-DECLARE| IN=sv0 OUT=sv1 \
             SRC=10.77.1.10 DST=10.77.1.20 LEN=60 TTL=64 PROTO=TCP SPT=40000 DPT=9999 SYN"
        );
        let mut lecteur = LecteurNftables::new(PlanAdressage::indicatif());
        lecteur.declarer_interface_voie("sv0", VoieId::new("jayoffice").unwrap());
        let ResultatLigne::Evenement(e) = lecteur.analyser(&l) else { panic!("{:?}", lecteur.analyser(&l)) };
        assert_eq!(e.voie.as_ref().map(VoieId::as_str), Some("jayoffice"));
        assert_eq!(e.port_dst, Some(9999));
        assert_eq!(e.motif, Some(miyuvigie::MotifJournal::NonDeclare));
    }

    /// B1 : TCP sans ports (fragment non initial, troncature, chaîne
    /// d'extensions IPv6 longue) ⇒ ligne émise, lue par les Vigies (non mal
    /// formée : le capteur ne se dégrade pas).
    #[test]
    fn sans_ports_ligne_emise_et_lue() {
        let mut lecteur = LecteurNftables::new(PlanAdressage::indicatif());
        lecteur.declarer_interface_voie("sv0", VoieId::new("jayoffice").unwrap());
        let prefixe = "MDOU|zone-franche|transit|NON-DECLARE|";
        let mut fragment = ipv4_tcp([10, 77, 1, 10], [10, 77, 1, 20], 9999, 0x02);
        fragment[7] = 0x10;
        let tronque = ipv4_tcp([10, 77, 1, 10], [10, 77, 1, 20], 9999, 0x02)[..22].to_vec();
        for (charge, marque) in [(fragment, " FRAG"), (tronque, " TRONQUE")] {
            let Ok(Conversion::Ligne(l)) = convertir(&paquet(prefixe, charge), &ctx()) else { panic!() };
            assert!(l.ends_with(&format!("PROTO=6{marque}")), "{l}");
            assert!(!l.contains("SPT="), "{l}");
            let ResultatLigne::Evenement(e) = lecteur.analyser(&l) else { panic!("{:?}", lecteur.analyser(&l)) };
            assert_eq!(e.port_dst, None);
            assert_eq!(e.voie.as_ref().map(VoieId::as_str), Some("jayoffice"));
        }
        let mut v6 = paquet(prefixe, crate::ip::tests::ipv6_extensions(crate::ip::EXTENSIONS_IPV6_MAX + 2));
        v6.famille = NFPROTO_IPV6;
        let Ok(Conversion::Ligne(l)) = convertir(&v6, &ctx()) else { panic!() };
        assert!(l.contains(" PROTO=60"), "{l}");
    }

    #[test]
    fn ports_physiques_du_pont() {
        let mut p = paquet("x", Vec::new());
        assert_eq!(interfaces(&p), (Some(3), Some(4)));
        p.interface_entree_physique = Some(13);
        p.interface_sortie_physique = Some(14);
        assert_eq!(interfaces(&p), (Some(13), Some(14)));
        p.famille = NFPROTO_IPV4;
        assert_eq!(interfaces(&p), (Some(3), Some(4)), "hors pont : IN / OUT du crochet");
    }

    #[test]
    fn temoin_et_refus() {
        let charge = ipv4_tcp([10, 77, 1, 10], [10, 77, 1, 20], 80, 0x12);
        assert_eq!(convertir(&paquet("MDOU-TEMOIN|zone-franche|", charge.clone()), &ctx()), Ok(Conversion::Temoin));
        assert_eq!(convertir(&paquet("MDOU|a b|", charge.clone()), &ctx()), Err(RefusPaquet::PrefixeInvalide));
        assert_eq!(convertir(&paquet("MDOU|é|", charge.clone()), &ctx()), Err(RefusPaquet::PrefixeInvalide));
        let mut sans = paquet("x", charge.clone());
        sans.prefixe = None;
        assert_eq!(convertir(&sans, &ctx()), Err(RefusPaquet::SansPrefixe));
        assert!(matches!(convertir(&paquet("x", charge[..22].to_vec()), &ctx()), Ok(Conversion::Ligne(_))));
        let mut arp = paquet("x", charge.clone());
        arp.famille = 3;
        assert_eq!(convertir(&arp, &ctx()), Err(RefusPaquet::Famille(3)));
        let mut l2 = paquet("x", charge);
        l2.protocole_l2 = Some(0x86dd);
        assert_eq!(convertir(&l2, &ctx()), Err(RefusPaquet::EnTeteIp));
    }

    #[test]
    fn drapeaux_marque_et_meta_flux() {
        let mut p = paquet("MDOU|externe|entree|NON-DECLARE|", ipv4_tcp([1, 2, 3, 4], [10, 77, 1, 5], 22, 0x12));
        p.marque = Some(0x2a);
        let Ok(Conversion::Ligne(l)) = convertir(&p, &Contexte { entree: "ext0", sortie: "", ..ctx() }) else { panic!() };
        assert!(l.ends_with(" OUT= SRC=1.2.3.4 DST=10.77.1.5 LEN=60 TTL=64 PROTO=TCP SPT=40000 DPT=22 SYN ACK MARK=0x2a"), "{l}");
        let m = meta_flux(&p, 1_729_760_000, Some("ext0")).unwrap();
        assert!(!m.nouvelle_connexion, "SYN+ACK n'ouvre pas de connexion");
        assert_eq!(m.port_destination, 22);
        assert_eq!(m.interface_entree.as_deref(), Some("ext0"));
        p.charge = ipv4_tcp([1, 2, 3, 4], [10, 77, 1, 5], 22, 0x02);
        assert!(meta_flux(&p, 0, None).unwrap().nouvelle_connexion);
    }
}
