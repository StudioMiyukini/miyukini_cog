//! @id     miyurelais_socket
//! @do     lier_groupe_nflog_netlink
//! @role   securite
//! @layer  operateur
//! @human  miyurelais — socket netlink NETLINK_NETFILTER (rustix, sans unsafe) liée aux groupes NFLOG, réception non bloquante, noms d'interfaces.

//! Socket `NETLINK_NETFILTER` liée à un ou plusieurs groupes NFLOG.
//!
//! Ouverture : `socket(AF_NETLINK, SOCK_RAW | SOCK_CLOEXEC, NETLINK_NETFILTER)`,
//! `bind(pid 0)`, puis par groupe `NFULNL_CFG_CMD_BIND` et paramètres
//! (copie tronquée, séquence locale, lot) — chaque requête **acquittée**
//! (`NLM_F_ACK`) : un groupe déjà pris (`EBUSY`), un droit manquant (`EPERM`,
//! il faut `CAP_NET_ADMIN` dans l'espace réseau) ou un noyau sans
//! `nfnetlink_log` font échouer l'ouverture.
//!
//! Tampon de réception : `SO_RCVBUFFORCE` (exige `CAP_NET_ADMIN`, ignore
//! `net.core.rmem_max`), repli `SO_RCVBUF` (plafonné par `rmem_max`) ; la
//! taille **effective** est relevée ([`SocketNflog::tampon_effectif`]) et doit
//! être journalisée par l'appelant.
//!
//! Chaque datagramme est reçu par `recvfrom` : un émetteur autre que le noyau
//! (`nl_pid` ≠ 0) est écarté ([`ErreurTransport::EmetteurNonNoyau`]).
//!
//! Les groupes NFLOG sont propres à l'**espace de noms réseau** : le relais
//! d'une frontière ou d'un pont de zone tourne dans l'espace de cette
//! frontière ou de ce pont (les index d'interfaces y sont aussi résolus).

use std::os::fd::OwnedFd;
use std::time::{Duration, Instant};

use rustix::io::Errno;
use rustix::net::netlink::{self, SocketAddrNetlink};
use rustix::net::{AddressFamily, RecvFlags, SendFlags, SocketFlags, SocketType};

use crate::netlink::{decoder_datagramme, requete_liaison, requete_parametres, Message, ParametresInstance};
use crate::relais::{ErreurTransport, Transport, TAMPON_RECEPTION};

/// Délai d'attente d'un acquittement du noyau.
pub const DELAI_ACQUITTEMENT: Duration = Duration::from_secs(2);
/// Tampon de réception demandé (octets) : `SO_RCVBUFFORCE`, à défaut
/// `SO_RCVBUF` plafonné par `net.core.rmem_max`.
pub const TAMPON_SOCKET: usize = 16 << 20;

/// Échec d'ouverture de la socket NFLOG.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErreurOuverture(pub String);

impl std::fmt::Display for ErreurOuverture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ouverture NFLOG : {}", self.0)
    }
}

impl std::error::Error for ErreurOuverture {}

/// Socket NFLOG ouverte et liée.
#[derive(Debug)]
pub struct SocketNflog {
    fd: OwnedFd,
    groupes: Vec<u16>,
    tampon: TamponReception,
}

/// Tampon de réception obtenu à l'ouverture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TamponReception {
    /// Taille effective rapportée par le noyau (`SO_RCVBUF` relu ; le noyau
    /// double la valeur demandée pour sa comptabilité).
    pub effectif: usize,
    /// Vrai si `SO_RCVBUFFORCE` a réussi (sinon repli `SO_RCVBUF`).
    pub force: bool,
}

/// Lit l'adresse d'origine d'un datagramme : `Some(pid)` netlink.
fn pid_emetteur(adresse: Option<rustix::net::SocketAddrAny>) -> Option<u32> {
    SocketAddrNetlink::try_from(adresse?).ok().map(|a| a.pid())
}

fn erreur(contexte: &str, e: Errno) -> ErreurOuverture {
    ErreurOuverture(format!("{contexte} : {e}"))
}

impl SocketNflog {
    /// Ouvre la socket et lie les `groupes`.
    pub fn ouvrir(groupes: &[u16], parametres: ParametresInstance) -> Result<Self, ErreurOuverture> {
        let fd = rustix::net::socket_with(
            AddressFamily::NETLINK,
            SocketType::RAW,
            SocketFlags::CLOEXEC,
            Some(netlink::NETFILTER),
        )
        .map_err(|e| erreur("socket", e))?;
        rustix::net::bind(&fd, &SocketAddrNetlink::new(0, 0)).map_err(|e| erreur("bind", e))?;
        // SO_RCVBUFFORCE (CAP_NET_ADMIN, au-delà de rmem_max), repli SO_RCVBUF.
        let force = rustix::net::sockopt::set_socket_recv_buffer_size_force(&fd, TAMPON_SOCKET).is_ok();
        if !force {
            rustix::net::sockopt::set_socket_recv_buffer_size(&fd, TAMPON_SOCKET).map_err(|e| erreur("SO_RCVBUF", e))?;
        }
        let effectif =
            rustix::net::sockopt::socket_recv_buffer_size(&fd).map_err(|e| erreur("SO_RCVBUF (lecture)", e))?;
        rustix::net::sockopt::set_socket_timeout(&fd, rustix::net::sockopt::Timeout::Recv, Some(DELAI_ACQUITTEMENT))
            .map_err(|e| erreur("SO_RCVTIMEO", e))?;
        let mut s = Self { fd, groupes: Vec::new(), tampon: TamponReception { effectif, force } };
        let mut sequence = 1u32;
        for &g in groupes {
            s.requete(&requete_liaison(sequence, g), sequence)
                .map_err(|e| ErreurOuverture(format!("liaison au groupe {g} : {}", e.0)))?;
            sequence += 1;
            s.requete(&requete_parametres(sequence, g, parametres), sequence)
                .map_err(|e| ErreurOuverture(format!("paramètres du groupe {g} : {}", e.0)))?;
            sequence += 1;
            s.groupes.push(g);
        }
        Ok(s)
    }

    /// Groupes liés.
    pub fn groupes(&self) -> &[u16] {
        &self.groupes
    }

    /// Tampon de réception effectif (à journaliser).
    pub fn tampon_effectif(&self) -> TamponReception {
        self.tampon
    }

    /// Envoie une requête et attend son acquittement (les paquets reçus entre
    /// temps, antérieurs à la configuration, sont écartés).
    fn requete(&mut self, octets: &[u8], sequence: u32) -> Result<(), ErreurOuverture> {
        rustix::net::sendto(&self.fd, octets, SendFlags::empty(), &SocketAddrNetlink::new(0, 0))
            .map_err(|e| erreur("sendto", e))?;
        let limite = Instant::now() + DELAI_ACQUITTEMENT;
        let mut tampon = vec![0u8; TAMPON_RECEPTION];
        while Instant::now() < limite {
            let (n, reel, adresse) = rustix::net::recvfrom(&self.fd, &mut tampon[..], RecvFlags::TRUNC)
                .map_err(|e| erreur("recvfrom (acquittement)", e))?;
            if reel > n || pid_emetteur(adresse) != Some(0) {
                // Tronqué, ou émis par un autre que le noyau : écarté.
                continue;
            }
            let messages = decoder_datagramme(&tampon[..n]).map_err(|e| ErreurOuverture(e.to_string()))?;
            for m in messages {
                if let Message::Acquittement { sequence: s, erreur: e } = m {
                    if s != sequence {
                        continue;
                    }
                    return if e == 0 {
                        Ok(())
                    } else {
                        Err(ErreurOuverture(format!("refus du noyau : {}", Errno::from_raw_os_error(-e))))
                    };
                }
            }
        }
        Err(ErreurOuverture("aucun acquittement du noyau".into()))
    }
}

impl Transport for SocketNflog {
    fn recevoir(&mut self, tampon: &mut [u8]) -> Result<Option<usize>, ErreurTransport> {
        match rustix::net::recvfrom(&self.fd, tampon, RecvFlags::DONTWAIT | RecvFlags::TRUNC) {
            Ok((n, reel, adresse)) => match pid_emetteur(adresse) {
                Some(0) if reel > n => Err(ErreurTransport::Tronque(reel)),
                Some(0) => Ok(Some(n)),
                pid => Err(ErreurTransport::EmetteurNonNoyau(pid.unwrap_or(u32::MAX))),
            },
            Err(Errno::AGAIN | Errno::INTR) => Ok(None),
            Err(Errno::NOBUFS) => Err(ErreurTransport::Debordement),
            Err(e) => Err(ErreurTransport::Io(e.to_string())),
        }
    }

    fn nom_interface(&mut self, index: u32) -> Option<String> {
        rustix::net::netdevice::index_to_name(&self.fd, index).ok()
    }
}
