//! @id     miyuvigie_sortie
//! @do     detecter_sortie_zone_cog
//! @role   securite
//! @layer  toolkit
//! @human  miyuvigie — module « sortie » : contrôle des connexions sortant de la zone COG (EX-12).

//! Contrôle de sortie de la zone COG (EX-12, `MODELE-ZONES-ET-COUCHES-COG.md` §12
//! « tentative de sortie de la zone COG = incident critique »).
//!
//! La zone COG n'a **aucune route sortante** (INV-Z1) ; elle ne communique
//! qu'avec le Portail interne (INV-Z7). Une connexion depuis la zone COG est
//! donc légitime **uniquement** si sa destination est :
//!
//! - la boucle locale (`127.0.0.0/8`, `::1`), ou
//! - une extrémité déclarée d'une voie du Portail ([`ExutoirePortail`]).
//!
//! Toute autre destination — y compris `0.0.0.0`, multidiffusion, une autre
//! adresse de la zone COG ou de la Franche — est une **sortie** (critique).
//! Une connexion attribuée à la voie A vers l'extrémité de la voie B est un
//! **mouvement latéral** (critique également).
//!
//! Deux entrées :
//! - [`TentativeConnexion`] fournie par un adaptateur (attribution processus → voie) ;
//! - [`analyser_proc_net_tcp`] : relevé strict de `/proc/net/tcp` / `tcp6` de
//!   l'espace réseau de la zone COG (sans attribution de voie).

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use crate::evenement::Protocole;
use crate::nftables::fnv1a;
use crate::signal::{EvenementId, VoieId};
use crate::zone::est_boucle_locale;

/// Extrémité d'une voie du Portail joignable depuis la zone COG.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExutoirePortail {
    /// Adresse de l'extrémité.
    pub ip: IpAddr,
    /// Port de l'extrémité.
    pub port: u16,
    /// Voie servie.
    pub voie: VoieId,
}

/// Tentative de connexion observée dans la zone COG.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TentativeConnexion {
    /// Identifiant unique par capteur.
    pub id: EvenementId,
    /// Horodatage (ms Unix), horloge de la source.
    pub horodatage_ms: u64,
    /// Voie du service à l'origine de la connexion, si attribuée.
    pub voie: Option<VoieId>,
    /// Processus (nom ou pid), pour l'explication.
    pub processus: Option<String>,
    /// Destination.
    pub ip_dst: IpAddr,
    /// Port destination.
    pub port_dst: u16,
    /// Protocole.
    pub protocole: Protocole,
}

/// Résultat du contrôle de sortie.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerdictSortie {
    /// Boucle locale : autorisé.
    BoucleLocale,
    /// Extrémité de la voie indiquée du Portail : autorisé.
    Portail(VoieId),
    /// Destination hors boucle locale et hors Portail : incident critique (EX-12).
    SortieZoneCog,
    /// Connexion de la voie `origine` vers l'extrémité de la voie `cible`.
    MouvementLateral {
        /// Voie d'origine.
        origine: VoieId,
        /// Voie dont l'extrémité est visée.
        cible: VoieId,
    },
}

impl VerdictSortie {
    /// Vrai si la connexion est un incident.
    pub fn est_incident(&self) -> bool {
        matches!(
            self,
            VerdictSortie::SortieZoneCog | VerdictSortie::MouvementLateral { .. }
        )
    }
}

/// Contrôle de sortie : liste fermée des exutoires du Portail.
#[derive(Debug, Clone, Default)]
pub struct ControleSortie {
    exutoires: Vec<ExutoirePortail>,
}

impl ControleSortie {
    /// Contrôle sans exutoire : seule la boucle locale est autorisée.
    pub fn new() -> Self {
        Self::default()
    }

    /// Déclare l'extrémité d'une voie du Portail.
    pub fn declarer_exutoire(&mut self, exutoire: ExutoirePortail) {
        self.exutoires.push(exutoire);
    }

    /// Évalue une destination, éventuellement attribuée à une voie.
    pub fn evaluer(&self, voie: Option<&VoieId>, ip_dst: IpAddr, port_dst: u16) -> VerdictSortie {
        if est_boucle_locale(ip_dst) {
            return VerdictSortie::BoucleLocale;
        }
        let Some(ex) = self
            .exutoires
            .iter()
            .find(|e| e.ip == ip_dst && e.port == port_dst)
        else {
            return VerdictSortie::SortieZoneCog;
        };
        match voie {
            Some(origine) if *origine != ex.voie => VerdictSortie::MouvementLateral {
                origine: origine.clone(),
                cible: ex.voie.clone(),
            },
            _ => VerdictSortie::Portail(ex.voie.clone()),
        }
    }
}

/// Résultat du relevé d'une ligne de `/proc/net/tcp`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LigneProc {
    /// Connexion avec un pair distant.
    Connexion(TentativeConnexion),
    /// Socket sans pair (écoute, pair non spécifié) : rien à contrôler.
    SansPair,
    /// Ligne mal formée (ignorée, comptée, jamais bénigne).
    MalFormee(&'static str),
}

/// Analyse le contenu de `/proc/net/tcp` ou `/proc/net/tcp6` (relevé à
/// `horodatage_ms`). L'en-tête est obligatoire ; un en-tête absent rend
/// toutes les lignes mal formées.
pub fn analyser_proc_net_tcp(contenu: &str, horodatage_ms: u64) -> Vec<LigneProc> {
    let mut lignes = contenu.lines().filter(|l| !l.trim().is_empty()).peekable();
    let entete_ok = lignes.peek().is_some_and(|l| {
        l.split_whitespace()
            .take(4)
            .eq(["sl", "local_address", "rem_address", "st"])
    });
    if entete_ok {
        lignes.next();
        lignes
            .map(|l| analyser_ligne_proc(l, horodatage_ms))
            .collect()
    } else {
        // Sans en-tête, aucune ligne n'est interprétable (y compris la première).
        lignes
            .map(|_| LigneProc::MalFormee("en-tête /proc/net/tcp absent"))
            .collect()
    }
}

fn analyser_ligne_proc(ligne: &str, horodatage_ms: u64) -> LigneProc {
    let champs: Vec<&str> = ligne.split_whitespace().collect();
    if champs.len() < 10 {
        return LigneProc::MalFormee("champs manquants");
    }
    let Some(sl) = champs[0].strip_suffix(':') else {
        return LigneProc::MalFormee("numéro de ligne");
    };
    if sl.is_empty() || !sl.bytes().all(|b| b.is_ascii_digit()) {
        return LigneProc::MalFormee("numéro de ligne");
    }
    let Some((_, _)) = analyser_extremite(champs[1]) else {
        return LigneProc::MalFormee("adresse locale");
    };
    let Some((ip_dst, port_dst)) = analyser_extremite(champs[2]) else {
        return LigneProc::MalFormee("adresse distante");
    };
    let Ok(etat) = u8::from_str_radix(champs[3], 16) else {
        return LigneProc::MalFormee("état");
    };
    if champs[3].len() != 2 || etat == 0 || etat > 0x0C {
        return LigneProc::MalFormee("état");
    }
    // 0x0A = LISTEN ; pair non spécifié = aucune connexion à contrôler.
    if etat == 0x0A || (ip_dst.is_unspecified() && port_dst == 0) {
        return LigneProc::SansPair;
    }
    let id = format!(
        "proc-tcp-{:016x}",
        fnv1a(format!("{} {}", champs[1], champs[2]).as_bytes())
    );
    let Some(id) = EvenementId::new(id) else {
        return LigneProc::MalFormee("identifiant");
    };
    LigneProc::Connexion(TentativeConnexion {
        id,
        horodatage_ms,
        voie: None,
        processus: Some(format!("inode:{}", champs[9])),
        ip_dst,
        port_dst,
        protocole: Protocole::Tcp,
    })
}

/// `HHHHHHHH:PPPP` (IPv4) ou `32 hex:PPPP` (IPv6), mots de 32 bits dans
/// l'ordre de l'hôte (format du noyau, relevé sur la même machine).
fn analyser_extremite(s: &str) -> Option<(IpAddr, u16)> {
    let (adr, port) = s.split_once(':')?;
    if port.len() != 4 || !hex_strict(port) || !hex_strict(adr) {
        return None;
    }
    let port = u16::from_str_radix(port, 16).ok()?;
    let ip = match adr.len() {
        8 => IpAddr::V4(Ipv4Addr::from(
            u32::from_str_radix(adr, 16).ok()?.to_ne_bytes(),
        )),
        32 => {
            let mut octets = [0u8; 16];
            for i in 0..4 {
                let mot = u32::from_str_radix(&adr[i * 8..i * 8 + 8], 16).ok()?;
                octets[i * 4..i * 4 + 4].copy_from_slice(&mot.to_ne_bytes());
            }
            IpAddr::V6(Ipv6Addr::from(octets))
        }
        _ => return None,
    };
    Some((ip, port))
}

fn hex_strict(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn voie(s: &str) -> VoieId {
        VoieId::new(s).unwrap()
    }

    fn controle() -> ControleSortie {
        let mut c = ControleSortie::new();
        c.declarer_exutoire(ExutoirePortail {
            ip: "10.77.0.250".parse().unwrap(),
            port: 7001,
            voie: voie("jaymail"),
        });
        c.declarer_exutoire(ExutoirePortail {
            ip: "10.77.0.250".parse().unwrap(),
            port: 7002,
            voie: voie("jayrdv"),
        });
        c
    }

    #[test]
    fn boucle_et_portail_autorises() {
        let c = controle();
        assert_eq!(
            c.evaluer(None, "127.0.0.1".parse().unwrap(), 5432),
            VerdictSortie::BoucleLocale
        );
        assert_eq!(
            c.evaluer(Some(&voie("jaymail")), "10.77.0.250".parse().unwrap(), 7001),
            VerdictSortie::Portail(voie("jaymail"))
        );
    }

    #[test]
    fn toute_autre_destination_est_une_sortie() {
        let c = controle();
        for (ip, port) in [
            ("93.184.216.34", 443),
            ("10.77.1.5", 80),     // Franche directe (INV-Z7)
            ("10.77.0.9", 80),     // autre hôte de la zone COG
            ("10.77.0.250", 7999), // Portail, port non déclaré
            ("0.0.0.0", 1),
            ("224.0.0.1", 5353),
        ] {
            let v = c.evaluer(None, ip.parse().unwrap(), port);
            assert_eq!(v, VerdictSortie::SortieZoneCog, "{ip}:{port}");
            assert!(v.est_incident());
        }
    }

    #[test]
    fn mouvement_lateral() {
        let v = controle().evaluer(Some(&voie("jaymail")), "10.77.0.250".parse().unwrap(), 7002);
        assert!(matches!(v, VerdictSortie::MouvementLateral { .. }));
        assert!(v.est_incident());
    }

    #[test]
    fn proc_net_tcp() {
        // 127.0.0.1:5432 en écoute ; 10.77.0.5 -> 93.184.216.34:443 SYN_SENT (hôte petit-boutiste).
        let ip_ext = u32::from_ne_bytes([93, 184, 216, 34]);
        let ip_loc = u32::from_ne_bytes([10, 77, 0, 5]);
        let ip_lo = u32::from_ne_bytes([127, 0, 0, 1]);
        let contenu = format!(
            "  sl  local_address rem_address   st tx_queue rx_queue tr tm->when retrnsmt   uid  timeout inode\n\
             \x20  0: {ip_lo:08X}:1538 00000000:0000 0A 00000000:00000000 00:00000000 00000000     0        0 1111 1\n\
             \x20  1: {ip_loc:08X}:9C40 {ip_ext:08X}:01BB 02 00000000:00000000 00:00000000 00000000  1000        0 2222 1\n\
             \x20  2: ZZZ:0000 00000000:0000 01 00000000:00000000 00:00000000 00000000     0        0 3333 1\n"
        );
        let r = analyser_proc_net_tcp(&contenu, 42);
        assert_eq!(r.len(), 3);
        assert_eq!(r[0], LigneProc::SansPair);
        let LigneProc::Connexion(t) = &r[1] else {
            panic!("connexion attendue : {:?}", r[1]);
        };
        assert_eq!(t.ip_dst, "93.184.216.34".parse::<IpAddr>().unwrap());
        assert_eq!(t.port_dst, 443);
        assert!(matches!(r[2], LigneProc::MalFormee(_)));
    }

    #[test]
    fn proc_sans_entete() {
        let r = analyser_proc_net_tcp("   0: 0100007F:1538 00000000:0000 0A 0 0 0 0 0 1\n", 1);
        assert!(matches!(r[0], LigneProc::MalFormee(_)));
    }
}
