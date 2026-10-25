//! @id     miyurelais_ip
//! @do     resumer_entetes_ip
//! @role   securite
//! @layer  toolkit
//! @human  miyurelais — résumé strict des en-têtes IPv4 / IPv6 et TCP / UDP / ICMP d'une charge utile NFLOG tronquée.

//! Résumé des en-têtes d'un paquet journalisé (charge utile NFLOG, tronquée
//! au `copy_range`) : adresses, longueur annoncée, TTL, protocole, ports,
//! drapeaux TCP. Lecture **bornée** : seules les tranches présentes sont lues ;
//! un en-tête de transport absent (fragment non initial, troncature) laisse
//! [`Transport::Absent`] — le relais refuse alors de produire une ligne TCP /
//! UDP sans port (le lecteur des Vigies l'exige).

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// En-têtes d'extension IPv6 parcourus au plus.
pub const EXTENSIONS_IPV6_MAX: usize = 8;

/// En-tête de transport résumé.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Transport {
    /// TCP : ports, drapeaux (si l'octet 13 est présent).
    Tcp {
        /// Port source.
        port_source: u16,
        /// Port destination.
        port_destination: u16,
        /// Drapeaux (FIN 0x01, SYN 0x02, RST 0x04, PSH 0x08, ACK 0x10, URG 0x20).
        drapeaux: Option<u8>,
    },
    /// UDP : ports.
    Udp {
        /// Port source.
        port_source: u16,
        /// Port destination.
        port_destination: u16,
    },
    /// ICMP / ICMPv6 : type et code.
    Icmp {
        /// Type.
        type_: u8,
        /// Code.
        code: u8,
    },
    /// En-tête absent (fragment non initial, charge tronquée) ou autre protocole.
    Absent,
}

/// Résumé d'un paquet IP.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResumeIp {
    /// Adresse source.
    pub source: IpAddr,
    /// Adresse destination.
    pub destination: IpAddr,
    /// Longueur annoncée par l'en-tête (octets, en-tête IP compris).
    pub longueur: u32,
    /// TTL / limite de sauts.
    pub ttl: u8,
    /// Numéro de protocole de transport (IANA).
    pub protocole: u8,
    /// Transport résumé.
    pub transport: Transport,
}

/// TCP.
pub const IPPROTO_TCP: u8 = 6;
/// UDP.
pub const IPPROTO_UDP: u8 = 17;
/// ICMP.
pub const IPPROTO_ICMP: u8 = 1;
/// ICMPv6.
pub const IPPROTO_ICMPV6: u8 = 58;

fn u16_be(o: &[u8], i: usize) -> Option<u16> {
    Some(u16::from_be_bytes([*o.get(i)?, *o.get(i + 1)?]))
}

fn transport(protocole: u8, t: &[u8]) -> Transport {
    let ports = || Some((u16_be(t, 0)?, u16_be(t, 2)?));
    match protocole {
        IPPROTO_TCP => match ports() {
            Some((s, d)) => Transport::Tcp { port_source: s, port_destination: d, drapeaux: t.get(13).copied() },
            None => Transport::Absent,
        },
        IPPROTO_UDP => match ports() {
            Some((s, d)) => Transport::Udp { port_source: s, port_destination: d },
            None => Transport::Absent,
        },
        IPPROTO_ICMP | IPPROTO_ICMPV6 => match (t.first(), t.get(1)) {
            (Some(&type_), Some(&code)) => Transport::Icmp { type_, code },
            _ => Transport::Absent,
        },
        _ => Transport::Absent,
    }
}

/// Résume un paquet IPv4 ; `None` si l'en-tête est invalide ou tronqué.
pub fn resumer_ipv4(o: &[u8]) -> Option<ResumeIp> {
    let premier = *o.first()?;
    if premier >> 4 != 4 {
        return None;
    }
    let ihl = usize::from(premier & 0x0f) * 4;
    if ihl < 20 || o.len() < ihl {
        return None;
    }
    let longueur = u16_be(o, 2)?;
    if usize::from(longueur) < ihl {
        return None;
    }
    let fragment = u16_be(o, 6)? & 0x1fff;
    let protocole = o[9];
    let source = Ipv4Addr::new(o[12], o[13], o[14], o[15]);
    let destination = Ipv4Addr::new(o[16], o[17], o[18], o[19]);
    let transport = if fragment == 0 { transport(protocole, &o[ihl..]) } else { Transport::Absent };
    Some(ResumeIp {
        source: IpAddr::V4(source),
        destination: IpAddr::V4(destination),
        longueur: u32::from(longueur),
        ttl: o[8],
        protocole,
        transport,
    })
}

/// Résume un paquet IPv6 (en-têtes d'extension usuels parcourus, bornés).
pub fn resumer_ipv6(o: &[u8]) -> Option<ResumeIp> {
    if o.len() < 40 || o[0] >> 4 != 6 {
        return None;
    }
    let charge = u32::from(u16_be(o, 4)?);
    let mut suivant = o[6];
    let ttl = o[7];
    let adresse = |i: usize| -> Option<Ipv6Addr> {
        let b: [u8; 16] = o.get(i..i + 16)?.try_into().ok()?;
        Some(Ipv6Addr::from(b))
    };
    let (source, destination) = (adresse(8)?, adresse(24)?);
    let mut i = 40usize;
    let mut initial = true;
    for _ in 0..EXTENSIONS_IPV6_MAX {
        match suivant {
            // Hop-by-hop, routage, options de destination : longueur en unités de 8 octets.
            0 | 43 | 60 => {
                let (Some(&n), Some(&l)) = (o.get(i), o.get(i + 1)) else {
                    initial = false;
                    break;
                };
                suivant = n;
                i = i.checked_add((usize::from(l) + 1) * 8)?;
            }
            // Fragment : 8 octets ; décalage ≠ 0 ⇒ pas d'en-tête de transport.
            44 => {
                let (Some(&n), Some(decalage)) = (o.get(i), u16_be(o, i + 2)) else {
                    initial = false;
                    break;
                };
                suivant = n;
                if decalage & 0xfff8 != 0 {
                    initial = false;
                    break;
                }
                i = i.checked_add(8)?;
            }
            _ => break,
        }
    }
    let transport = if initial && matches!(suivant, IPPROTO_TCP | IPPROTO_UDP | IPPROTO_ICMPV6) {
        o.get(i..).map_or(Transport::Absent, |t| transport(suivant, t))
    } else {
        Transport::Absent
    };
    Some(ResumeIp {
        source: IpAddr::V6(source),
        destination: IpAddr::V6(destination),
        longueur: charge + 40,
        ttl,
        protocole: suivant,
        transport,
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// IPv4 + TCP SYN de `src:40000` vers `dst:dport`.
    pub(crate) fn ipv4_tcp(src: [u8; 4], dst: [u8; 4], dport: u16, drapeaux: u8) -> Vec<u8> {
        let mut p = vec![0x45, 0, 0, 60, 0, 1, 0x40, 0, 64, IPPROTO_TCP, 0, 0];
        p.extend_from_slice(&src);
        p.extend_from_slice(&dst);
        p.extend_from_slice(&40_000u16.to_be_bytes());
        p.extend_from_slice(&dport.to_be_bytes());
        p.extend_from_slice(&[0, 0, 0, 1, 0, 0, 0, 0, 0xa0, drapeaux, 0xff, 0xff]);
        p
    }

    #[test]
    fn ipv4_tcp_syn() {
        let r = resumer_ipv4(&ipv4_tcp([10, 77, 1, 10], [10, 77, 1, 20], 9999, 0x02)).unwrap();
        assert_eq!(r.source, IpAddr::from([10, 77, 1, 10]));
        assert_eq!(r.longueur, 60);
        assert_eq!(r.ttl, 64);
        assert_eq!(
            r.transport,
            Transport::Tcp { port_source: 40_000, port_destination: 9999, drapeaux: Some(0x02) }
        );
    }

    #[test]
    fn ipv4_tronque_ou_fragment() {
        let paquet = ipv4_tcp([10, 0, 0, 1], [10, 0, 0, 2], 80, 2);
        assert_eq!(resumer_ipv4(&paquet[..22]).unwrap().transport, Transport::Absent);
        let resume = resumer_ipv4(&paquet[..26]).unwrap();
        assert_eq!(resume.transport, Transport::Tcp { port_source: 40_000, port_destination: 80, drapeaux: None });
        let mut fragment = paquet.clone();
        fragment[7] = 0x10; // décalage de fragment ≠ 0
        assert_eq!(resumer_ipv4(&fragment).unwrap().transport, Transport::Absent);
        assert!(resumer_ipv4(&paquet[..19]).is_none());
        let mut ihl = paquet.clone();
        ihl[0] = 0x44; // IHL 16 < 20
        assert!(resumer_ipv4(&ihl).is_none());
        let mut longueur = paquet;
        longueur[2..4].copy_from_slice(&10u16.to_be_bytes()); // longueur < IHL
        assert!(resumer_ipv4(&longueur).is_none());
    }

    #[test]
    fn ipv6_udp_avec_extension() {
        let mut p = vec![0x60, 0, 0, 0, 0, 16, 0, 64];
        p.extend_from_slice(&[0xfd; 16]);
        p.extend_from_slice(&[0xfe; 16]);
        p[6] = 60; // options de destination
        p.extend_from_slice(&[IPPROTO_UDP, 0, 0, 0, 0, 0, 0, 0]);
        p.extend_from_slice(&[0x13, 0x88, 0x00, 0x35, 0, 8, 0, 0]);
        let r = resumer_ipv6(&p).unwrap();
        assert_eq!(r.longueur, 56);
        assert_eq!(r.protocole, IPPROTO_UDP);
        assert_eq!(r.transport, Transport::Udp { port_source: 5000, port_destination: 53 });
        // Extension annoncée mais tronquée.
        assert_eq!(resumer_ipv6(&p[..41]).unwrap().transport, Transport::Absent);
    }
}
