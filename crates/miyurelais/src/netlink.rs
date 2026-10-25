//! @id     miyurelais_netlink
//! @do     decoder_trames_netlink_nflog
//! @role   securite
//! @layer  toolkit
//! @human  miyurelais — décodage strict et borné des trames netlink NFLOG (NFNL_SUBSYS_ULOG) et construction des messages de configuration.

//! Trames netlink du sous-système `NFNL_SUBSYS_ULOG` (nfnetlink_log), écrites
//! en salle blanche depuis les en-têtes UAPI du noyau (`linux/netlink.h`,
//! `linux/netfilter/nfnetlink.h`, `linux/netfilter/nfnetlink_log.h`) ; aucun
//! code de libnetfilter_log (GPL, C-71).
//!
//! # Disposition
//!
//! ```text
//! datagramme = message*                       (chacun aligné sur 4 octets)
//! message    = nlmsghdr(16) | corps
//! nlmsghdr   = len(u32 natif) | type(u16 natif) | flags(u16) | seq(u32) | pid(u32)
//! corps ULOG = nfgenmsg(4) | attribut*
//! nfgenmsg   = famille(u8) | version(u8 = 0) | res_id(be16 = groupe NFLOG)
//! attribut   = nla_len(u16 natif) | nla_type(u16 natif) | valeur | bourrage à 4
//! ```
//!
//! # Décodage strict et borné
//!
//! Toute trame non conforme est **refusée** en entier ([`ErreurTrame`]) :
//! longueurs incohérentes ou non alignées, version `nfgenmsg` ≠ 0, type de
//! message inattendu, attribut inconnu, **en double** ou de taille fausse,
//! préfixe sans `NUL` final unique, horodatage hors bornes, charge utile
//! au-delà de [`CHARGE_MAX`]. Bornes : [`MESSAGES_MAX`] messages par
//! datagramme, [`ATTRIBUTS_MAX`] attributs par message. Aucune allocation
//! n'est proportionnelle à une longueur annoncée : seules les tranches
//! effectivement présentes sont copiées.

use std::fmt;

/// En-tête netlink (`struct nlmsghdr`).
pub const NLMSG_HDRLEN: usize = 16;
/// En-tête d'attribut (`struct nlattr`).
pub const NLA_HDRLEN: usize = 4;
/// `struct nfgenmsg`.
pub const NFGENMSG_LEN: usize = 4;

/// `NLMSG_NOOP`.
pub const NLMSG_NOOP: u16 = 1;
/// `NLMSG_ERROR` (porte aussi les acquittements, erreur = 0).
pub const NLMSG_ERROR: u16 = 2;
/// `NLMSG_DONE` (fin d'un lot multi-parties).
pub const NLMSG_DONE: u16 = 3;
/// `NLMSG_OVERRUN` (données perdues).
pub const NLMSG_OVERRUN: u16 = 4;

/// `NLM_F_REQUEST`.
pub const NLM_F_REQUEST: u16 = 0x01;
/// `NLM_F_MULTI`.
pub const NLM_F_MULTI: u16 = 0x02;
/// `NLM_F_ACK`.
pub const NLM_F_ACK: u16 = 0x04;

/// `NFNL_SUBSYS_ULOG`.
pub const NFNL_SUBSYS_ULOG: u16 = 4;
/// `NFULNL_MSG_PACKET` : paquet journalisé (noyau → espace utilisateur).
pub const NFULNL_MSG_PACKET: u16 = 0;
/// `NFULNL_MSG_CONFIG` : configuration d'une instance (groupe).
pub const NFULNL_MSG_CONFIG: u16 = 1;
/// Type netlink d'un paquet NFLOG.
pub const TYPE_PAQUET: u16 = (NFNL_SUBSYS_ULOG << 8) | NFULNL_MSG_PACKET;
/// Type netlink d'une configuration NFLOG.
pub const TYPE_CONFIG: u16 = (NFNL_SUBSYS_ULOG << 8) | NFULNL_MSG_CONFIG;
/// `NFNETLINK_V0`.
pub const NFNETLINK_V0: u8 = 0;

/// Masque du type d'attribut (`NLA_TYPE_MASK` : sans `NLA_F_NESTED` /
/// `NLA_F_NET_BYTEORDER`).
pub const NLA_TYPE_MASK: u16 = 0x3fff;

// Attributs d'un paquet (`enum nfulnl_attr_type`).
/// `NFULA_PACKET_HDR` : `be16 hw_protocol | u8 hook | u8 pad`.
pub const NFULA_PACKET_HDR: u16 = 1;
/// `NFULA_MARK` (be32).
pub const NFULA_MARK: u16 = 2;
/// `NFULA_TIMESTAMP` : `be64 sec | be64 usec`.
pub const NFULA_TIMESTAMP: u16 = 3;
/// `NFULA_IFINDEX_INDEV` (be32).
pub const NFULA_IFINDEX_INDEV: u16 = 4;
/// `NFULA_IFINDEX_OUTDEV` (be32).
pub const NFULA_IFINDEX_OUTDEV: u16 = 5;
/// `NFULA_IFINDEX_PHYSINDEV` (be32).
pub const NFULA_IFINDEX_PHYSINDEV: u16 = 6;
/// `NFULA_IFINDEX_PHYSOUTDEV` (be32).
pub const NFULA_IFINDEX_PHYSOUTDEV: u16 = 7;
/// `NFULA_HWADDR` : `be16 len | u16 pad | u8[8]`.
pub const NFULA_HWADDR: u16 = 8;
/// `NFULA_PAYLOAD` : paquet à partir de l'en-tête réseau (tronqué au `copy_range`).
pub const NFULA_PAYLOAD: u16 = 9;
/// `NFULA_PREFIX` : chaîne terminée par `NUL`.
pub const NFULA_PREFIX: u16 = 10;
/// `NFULA_UID` (be32).
pub const NFULA_UID: u16 = 11;
/// `NFULA_SEQ` (be32, séquence locale à l'instance).
pub const NFULA_SEQ: u16 = 12;
/// `NFULA_SEQ_GLOBAL` (be32).
pub const NFULA_SEQ_GLOBAL: u16 = 13;
/// `NFULA_GID` (be32).
pub const NFULA_GID: u16 = 14;
/// `NFULA_HWTYPE` (be16).
pub const NFULA_HWTYPE: u16 = 15;
/// `NFULA_HWHEADER` (variable).
pub const NFULA_HWHEADER: u16 = 16;
/// `NFULA_HWLEN` (be16).
pub const NFULA_HWLEN: u16 = 17;
/// `NFULA_CT` (imbriqué, variable).
pub const NFULA_CT: u16 = 18;
/// `NFULA_CT_INFO` (be32).
pub const NFULA_CT_INFO: u16 = 19;
/// `NFULA_VLAN` (imbriqué, variable).
pub const NFULA_VLAN: u16 = 20;
/// `NFULA_L2HDR` (variable).
pub const NFULA_L2HDR: u16 = 21;

// Attributs de configuration (`enum nfulnl_attr_config`).
/// `NFULA_CFG_CMD` : `u8 command`.
pub const NFULA_CFG_CMD: u16 = 1;
/// `NFULA_CFG_MODE` : `be32 copy_range | u8 copy_mode | u8 pad`.
pub const NFULA_CFG_MODE: u16 = 2;
/// `NFULA_CFG_TIMEOUT` (be32, centièmes de seconde).
pub const NFULA_CFG_TIMEOUT: u16 = 4;
/// `NFULA_CFG_QTHRESH` (be32, paquets par lot).
pub const NFULA_CFG_QTHRESH: u16 = 5;
/// `NFULA_CFG_FLAGS` (be16).
pub const NFULA_CFG_FLAGS: u16 = 6;
/// `NFULNL_CFG_CMD_BIND`.
pub const NFULNL_CFG_CMD_BIND: u8 = 1;
/// `NFULNL_CFG_CMD_UNBIND`.
pub const NFULNL_CFG_CMD_UNBIND: u8 = 2;
/// `NFULNL_COPY_PACKET`.
pub const NFULNL_COPY_PACKET: u8 = 2;
/// `NFULNL_CFG_F_SEQ` : séquence locale à l'instance (détection des pertes).
pub const NFULNL_CFG_F_SEQ: u16 = 0x0001;

/// Messages au plus par datagramme.
pub const MESSAGES_MAX: usize = 4096;
/// Attributs au plus par message.
pub const ATTRIBUTS_MAX: usize = 64;
/// Charge utile copiée au plus (octets) : borne du `copy_range` demandé.
pub const CHARGE_MAX: usize = 4096;
/// Préfixe au plus, `NUL` compris (`NF_LOG_PREFIXLEN`).
pub const PREFIXE_MAX: usize = 128;

/// Refus d'une trame netlink.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErreurTrame {
    /// Longueur d'en-tête ou de message incohérente.
    Longueur(&'static str),
    /// Trop de messages ou d'attributs.
    Borne(&'static str),
    /// Type de message inattendu.
    TypeMessage(u16),
    /// Version `nfgenmsg` ≠ 0.
    Version(u8),
    /// Attribut inconnu.
    AttributInconnu(u16),
    /// Attribut présent deux fois.
    AttributDouble(u16),
    /// Taille d'attribut fausse.
    TailleAttribut(u16),
    /// Valeur d'attribut invalide.
    Valeur(&'static str),
    /// `NLMSG_OVERRUN` : le noyau signale des données perdues.
    Debordement,
}

impl fmt::Display for ErreurTrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Longueur(m) => write!(f, "longueur : {m}"),
            Self::Borne(m) => write!(f, "borne dépassée : {m}"),
            Self::TypeMessage(t) => write!(f, "type de message inattendu {t:#06x}"),
            Self::Version(v) => write!(f, "version nfgenmsg {v}"),
            Self::AttributInconnu(t) => write!(f, "attribut inconnu {t}"),
            Self::AttributDouble(t) => write!(f, "attribut {t} en double"),
            Self::TailleAttribut(t) => write!(f, "taille de l'attribut {t}"),
            Self::Valeur(m) => write!(f, "valeur invalide : {m}"),
            Self::Debordement => f.write_str("NLMSG_OVERRUN (données perdues)"),
        }
    }
}

impl std::error::Error for ErreurTrame {}

/// Paquet journalisé, décodé d'un message `NFULNL_MSG_PACKET`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaquetNflog {
    /// Famille du crochet (`nfgenmsg.nfgen_family` : 2 IPv4, 7 pont, 10 IPv6…).
    pub famille: u8,
    /// Groupe NFLOG (`nfgenmsg.res_id`).
    pub groupe: u16,
    /// Protocole de niveau 2 (`hw_protocol`, ex. `0x0800`), si fourni.
    pub protocole_l2: Option<u16>,
    /// Crochet netfilter (`hook`), si fourni.
    pub crochet: Option<u8>,
    /// Marque du paquet.
    pub marque: Option<u32>,
    /// Horodatage du noyau (µs depuis l'époque Unix), si fourni.
    pub horodatage_us: Option<u64>,
    /// Index de l'interface d'entrée.
    pub interface_entree: Option<u32>,
    /// Index de l'interface de sortie.
    pub interface_sortie: Option<u32>,
    /// Port physique d'entrée (`NFULA_IFINDEX_PHYSINDEV`) : famille pont, le
    /// port du pont (`sv<n>`) — `interface_entree` y est le pont lui-même.
    pub interface_entree_physique: Option<u32>,
    /// Port physique de sortie (`NFULA_IFINDEX_PHYSOUTDEV`).
    pub interface_sortie_physique: Option<u32>,
    /// Préfixe de la règle `log` (sans `NUL`), octets bruts non validés.
    pub prefixe: Option<Vec<u8>>,
    /// Séquence locale de l'instance (`NFULNL_CFG_F_SEQ`).
    pub sequence: Option<u32>,
    /// Charge utile (en-tête réseau inclus), tronquée au `copy_range`.
    pub charge: Vec<u8>,
}

/// Message décodé d'un datagramme NFLOG.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    /// Paquet journalisé.
    Paquet(PaquetNflog),
    /// Acquittement (`erreur == 0`) ou erreur (`errno` négatif) d'une requête.
    Acquittement {
        /// Séquence de la requête acquittée.
        sequence: u32,
        /// 0 = succès ; sinon `-errno`.
        erreur: i32,
    },
    /// Fin d'un lot multi-parties ou message vide (`NLMSG_DONE`, `NLMSG_NOOP`).
    Controle,
}

fn u16_natif(o: &[u8], i: usize) -> Option<u16> {
    Some(u16::from_ne_bytes(o.get(i..i + 2)?.try_into().ok()?))
}

fn u32_natif(o: &[u8], i: usize) -> Option<u32> {
    Some(u32::from_ne_bytes(o.get(i..i + 4)?.try_into().ok()?))
}

fn be16(o: &[u8]) -> Option<u16> {
    Some(u16::from_be_bytes(o.try_into().ok()?))
}

fn be32(o: &[u8]) -> Option<u32> {
    Some(u32::from_be_bytes(o.try_into().ok()?))
}

fn be64(o: &[u8]) -> Option<u64> {
    Some(u64::from_be_bytes(o.try_into().ok()?))
}

const fn aligner(n: usize) -> usize {
    (n + 3) & !3
}

/// Décode un datagramme netlink reçu d'une socket `NETLINK_NETFILTER`.
///
/// Refus de tout le datagramme à la première non-conformité : un datagramme
/// à moitié lisible n'est jamais à moitié accepté.
pub fn decoder_datagramme(octets: &[u8]) -> Result<Vec<Message>, ErreurTrame> {
    use ErreurTrame::{Borne, Longueur};
    let mut messages = Vec::new();
    let mut i = 0;
    while i < octets.len() {
        if messages.len() >= MESSAGES_MAX {
            return Err(Borne("messages par datagramme"));
        }
        let reste = octets.len() - i;
        if reste < NLMSG_HDRLEN {
            return Err(Longueur("en-tête netlink tronqué"));
        }
        let longueur = u32_natif(octets, i).ok_or(Longueur("nlmsg_len"))? as usize;
        let type_ = u16_natif(octets, i + 4).ok_or(Longueur("nlmsg_type"))?;
        let sequence = u32_natif(octets, i + 8).ok_or(Longueur("nlmsg_seq"))?;
        if longueur < NLMSG_HDRLEN || longueur > reste {
            return Err(Longueur("nlmsg_len hors du datagramme"));
        }
        let corps = &octets[i + NLMSG_HDRLEN..i + longueur];
        messages.push(match type_ {
            NLMSG_NOOP | NLMSG_DONE => Message::Controle,
            NLMSG_OVERRUN => return Err(ErreurTrame::Debordement),
            NLMSG_ERROR => {
                // i32 errno puis l'en-tête de la requête d'origine (au moins).
                if corps.len() < 4 + NLMSG_HDRLEN {
                    return Err(Longueur("NLMSG_ERROR tronqué"));
                }
                let erreur = i32::from_ne_bytes(corps[0..4].try_into().map_err(|_| Longueur("errno"))?);
                Message::Acquittement { sequence, erreur }
            }
            TYPE_PAQUET => Message::Paquet(decoder_paquet(corps)?),
            t => return Err(ErreurTrame::TypeMessage(t)),
        });
        let suivant = aligner(longueur);
        if suivant > reste {
            // Dernier message non bourré : accepté seulement s'il finit le datagramme.
            if longueur != reste {
                return Err(Longueur("bourrage final"));
            }
            break;
        }
        i += suivant;
    }
    Ok(messages)
}

/// Décode le corps d'un message `NFULNL_MSG_PACKET` (après `nlmsghdr`).
#[allow(clippy::too_many_lines)]
pub fn decoder_paquet(corps: &[u8]) -> Result<PaquetNflog, ErreurTrame> {
    use ErreurTrame::{AttributDouble, AttributInconnu, Borne, Longueur, TailleAttribut, Valeur};
    if corps.len() < NFGENMSG_LEN {
        return Err(Longueur("nfgenmsg tronqué"));
    }
    if corps[1] != NFNETLINK_V0 {
        return Err(ErreurTrame::Version(corps[1]));
    }
    let mut p = PaquetNflog {
        famille: corps[0],
        groupe: u16::from_be_bytes([corps[2], corps[3]]),
        protocole_l2: None,
        crochet: None,
        marque: None,
        horodatage_us: None,
        interface_entree: None,
        interface_sortie: None,
        interface_entree_physique: None,
        interface_sortie_physique: None,
        prefixe: None,
        sequence: None,
        charge: Vec::new(),
    };
    let mut vus: u32 = 0; // types 1..=21 : un bit chacun
    let mut nombre = 0usize;
    let mut i = NFGENMSG_LEN;
    while i < corps.len() {
        nombre += 1;
        if nombre > ATTRIBUTS_MAX {
            return Err(Borne("attributs par message"));
        }
        let reste = corps.len() - i;
        if reste < NLA_HDRLEN {
            return Err(Longueur("en-tête d'attribut tronqué"));
        }
        let nla_len = u16_natif(corps, i).ok_or(Longueur("nla_len"))? as usize;
        let nla_type = u16_natif(corps, i + 2).ok_or(Longueur("nla_type"))? & NLA_TYPE_MASK;
        if nla_len < NLA_HDRLEN || nla_len > reste {
            return Err(Longueur("nla_len hors du message"));
        }
        let v = &corps[i + NLA_HDRLEN..i + nla_len];
        if nla_type == 0 || nla_type > NFULA_L2HDR {
            return Err(AttributInconnu(nla_type));
        }
        let bit = 1u32 << nla_type;
        if vus & bit != 0 {
            return Err(AttributDouble(nla_type));
        }
        vus |= bit;
        let taille = |attendue: usize| if v.len() == attendue { Ok(()) } else { Err(TailleAttribut(nla_type)) };
        match nla_type {
            NFULA_PACKET_HDR => {
                taille(4)?;
                p.protocole_l2 = be16(&v[0..2]);
                p.crochet = Some(v[2]);
            }
            NFULA_MARK => {
                taille(4)?;
                p.marque = be32(v);
            }
            NFULA_TIMESTAMP => {
                taille(16)?;
                let sec = be64(&v[0..8]).ok_or(TailleAttribut(nla_type))?;
                let usec = be64(&v[8..16]).ok_or(TailleAttribut(nla_type))?;
                if usec >= 1_000_000 {
                    return Err(Valeur("horodatage : µs ≥ 10⁶"));
                }
                let us = sec
                    .checked_mul(1_000_000)
                    .and_then(|s| s.checked_add(usec))
                    .ok_or(Valeur("horodatage hors bornes"))?;
                p.horodatage_us = Some(us);
            }
            NFULA_IFINDEX_INDEV => {
                taille(4)?;
                p.interface_entree = be32(v);
            }
            NFULA_IFINDEX_OUTDEV => {
                taille(4)?;
                p.interface_sortie = be32(v);
            }
            NFULA_PREFIX => {
                if v.is_empty() || v.len() > PREFIXE_MAX {
                    return Err(TailleAttribut(nla_type));
                }
                let (dernier, texte) = v.split_last().ok_or(TailleAttribut(nla_type))?;
                if *dernier != 0 || texte.contains(&0) {
                    return Err(Valeur("préfixe : NUL final unique attendu"));
                }
                p.prefixe = Some(texte.to_vec());
            }
            NFULA_PAYLOAD => {
                if v.len() > CHARGE_MAX {
                    return Err(Borne("charge utile au-delà du copy_range"));
                }
                p.charge = v.to_vec();
            }
            NFULA_SEQ => {
                taille(4)?;
                p.sequence = be32(v);
            }
            NFULA_IFINDEX_PHYSINDEV => {
                taille(4)?;
                p.interface_entree_physique = be32(v);
            }
            NFULA_IFINDEX_PHYSOUTDEV => {
                taille(4)?;
                p.interface_sortie_physique = be32(v);
            }
            NFULA_UID
            | NFULA_GID
            | NFULA_SEQ_GLOBAL
            | NFULA_CT_INFO => taille(4)?,
            NFULA_HWADDR => taille(12)?,
            NFULA_HWTYPE | NFULA_HWLEN => taille(2)?,
            // Variables, non utilisés : présence admise, contenu ignoré.
            NFULA_HWHEADER | NFULA_CT | NFULA_VLAN | NFULA_L2HDR => {}
            t => return Err(AttributInconnu(t)),
        }
        let suivant = aligner(nla_len);
        if suivant > reste {
            if nla_len != reste {
                return Err(Longueur("bourrage d'attribut"));
            }
            break;
        }
        i += suivant;
    }
    Ok(p)
}

// ----------------------------------------------------------------- requêtes

fn pousser_attribut(tampon: &mut Vec<u8>, type_: u16, valeur: &[u8]) {
    let longueur = (NLA_HDRLEN + valeur.len()) as u16;
    tampon.extend_from_slice(&longueur.to_ne_bytes());
    tampon.extend_from_slice(&type_.to_ne_bytes());
    tampon.extend_from_slice(valeur);
    while tampon.len() & 3 != 0 {
        tampon.push(0);
    }
}

fn message_config(sequence: u32, famille: u8, groupe: u16, attributs: &[(u16, Vec<u8>)]) -> Vec<u8> {
    let mut m = vec![0u8; NLMSG_HDRLEN];
    m.extend_from_slice(&[famille, NFNETLINK_V0]);
    m.extend_from_slice(&groupe.to_be_bytes());
    for (t, v) in attributs {
        pousser_attribut(&mut m, *t, v);
    }
    let longueur = m.len() as u32;
    m[0..4].copy_from_slice(&longueur.to_ne_bytes());
    m[4..6].copy_from_slice(&TYPE_CONFIG.to_ne_bytes());
    m[6..8].copy_from_slice(&(NLM_F_REQUEST | NLM_F_ACK).to_ne_bytes());
    m[8..12].copy_from_slice(&sequence.to_ne_bytes());
    // pid 0 : destinataire = noyau.
    m
}

/// Requête de liaison au groupe NFLOG (`NFULNL_CFG_CMD_BIND`).
pub fn requete_liaison(sequence: u32, groupe: u16) -> Vec<u8> {
    message_config(sequence, 0, groupe, &[(NFULA_CFG_CMD, vec![NFULNL_CFG_CMD_BIND])])
}

/// Requête de déliaison (`NFULNL_CFG_CMD_UNBIND`).
pub fn requete_deliaison(sequence: u32, groupe: u16) -> Vec<u8> {
    message_config(sequence, 0, groupe, &[(NFULA_CFG_CMD, vec![NFULNL_CFG_CMD_UNBIND])])
}

/// Paramètres de l'instance NFLOG demandés au noyau.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParametresInstance {
    /// Octets de paquet copiés (charge utile tronquée), ≤ [`CHARGE_MAX`].
    pub copie: u32,
    /// Paquets par lot avant envoi.
    pub seuil_lot: u32,
    /// Délai d'envoi d'un lot incomplet (centièmes de seconde).
    pub delai_lot_cs: u32,
}

impl Default for ParametresInstance {
    fn default() -> Self {
        // 128 octets : en-têtes IPv4 / IPv6 + TCP / UDP / ICMP, jamais le contenu applicatif.
        Self { copie: 128, seuil_lot: 8, delai_lot_cs: 10 }
    }
}

/// Requête de configuration : copie du paquet tronquée, séquence locale
/// (détection des pertes), seuil et délai de lot.
pub fn requete_parametres(sequence: u32, groupe: u16, p: ParametresInstance) -> Vec<u8> {
    let copie = p.copie.min(CHARGE_MAX as u32);
    let mut mode = copie.to_be_bytes().to_vec();
    mode.extend_from_slice(&[NFULNL_COPY_PACKET, 0]);
    message_config(
        sequence,
        0,
        groupe,
        &[
            (NFULA_CFG_MODE, mode),
            (NFULA_CFG_FLAGS, NFULNL_CFG_F_SEQ.to_be_bytes().to_vec()),
            (NFULA_CFG_QTHRESH, p.seuil_lot.max(1).to_be_bytes().to_vec()),
            (NFULA_CFG_TIMEOUT, p.delai_lot_cs.to_be_bytes().to_vec()),
        ],
    )
}

// ------------------------------------------------------------ construction

/// Constructeur de trames NFLOG (tests, fuzzing, graines) : produit ce que le
/// noyau émettrait.
#[derive(Debug, Clone, Default)]
pub struct TrameEssai {
    attributs: Vec<(u16, Vec<u8>)>,
}

impl TrameEssai {
    /// Trame vide.
    pub fn new() -> Self {
        Self::default()
    }

    /// Ajoute un attribut brut.
    #[must_use]
    pub fn attribut(mut self, type_: u16, valeur: &[u8]) -> Self {
        self.attributs.push((type_, valeur.to_vec()));
        self
    }

    /// Ajoute un préfixe (`NUL` final ajouté).
    #[must_use]
    pub fn prefixe(self, p: &str) -> Self {
        let mut v = p.as_bytes().to_vec();
        v.push(0);
        self.attribut(NFULA_PREFIX, &v)
    }

    /// Ajoute un attribut be32.
    #[must_use]
    pub fn be32(self, type_: u16, v: u32) -> Self {
        self.attribut(type_, &v.to_be_bytes())
    }

    /// Ajoute un horodatage.
    #[must_use]
    pub fn horodatage(self, sec: u64, usec: u64) -> Self {
        let mut v = sec.to_be_bytes().to_vec();
        v.extend_from_slice(&usec.to_be_bytes());
        self.attribut(NFULA_TIMESTAMP, &v)
    }

    /// Message `NFULNL_MSG_PACKET` complet.
    pub fn message(&self, famille: u8, groupe: u16) -> Vec<u8> {
        let mut m = vec![0u8; NLMSG_HDRLEN];
        m.extend_from_slice(&[famille, NFNETLINK_V0]);
        m.extend_from_slice(&groupe.to_be_bytes());
        for (t, v) in &self.attributs {
            pousser_attribut(&mut m, *t, v);
        }
        let longueur = m.len() as u32;
        m[0..4].copy_from_slice(&longueur.to_ne_bytes());
        m[4..6].copy_from_slice(&TYPE_PAQUET.to_ne_bytes());
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn trame() -> TrameEssai {
        TrameEssai::new()
            .attribut(NFULA_PACKET_HDR, &[0x08, 0x00, 2, 0])
            .be32(NFULA_MARK, 0x2a)
            .horodatage(1_729_760_000, 123_456)
            .be32(NFULA_IFINDEX_INDEV, 3)
            .be32(NFULA_IFINDEX_OUTDEV, 4)
            .be32(NFULA_IFINDEX_PHYSINDEV, 13)
            .be32(NFULA_IFINDEX_PHYSOUTDEV, 14)
            .prefixe("MDOU|zone-franche|transit|NON-DECLARE|")
            .be32(NFULA_SEQ, 7)
            .attribut(NFULA_PAYLOAD, &[0x45, 0, 0, 20])
    }

    #[test]
    fn paquet_complet() {
        let d = decoder_datagramme(&trame().message(7, 111)).unwrap();
        let [Message::Paquet(p)] = d.as_slice() else { panic!("{d:?}") };
        assert_eq!(p.famille, 7);
        assert_eq!(p.groupe, 111);
        assert_eq!(p.protocole_l2, Some(0x0800));
        assert_eq!(p.crochet, Some(2));
        assert_eq!(p.marque, Some(0x2a));
        assert_eq!(p.horodatage_us, Some(1_729_760_000_123_456));
        assert_eq!((p.interface_entree, p.interface_sortie), (Some(3), Some(4)));
        assert_eq!((p.interface_entree_physique, p.interface_sortie_physique), (Some(13), Some(14)));
        assert_eq!(p.prefixe.as_deref(), Some(&b"MDOU|zone-franche|transit|NON-DECLARE|"[..]));
        assert_eq!(p.sequence, Some(7));
        assert_eq!(p.charge, vec![0x45, 0, 0, 20]);
    }

    #[test]
    fn plusieurs_messages_et_fin() {
        let mut d = trame().message(2, 110);
        d.extend(trame().message(2, 110));
        let mut fin = vec![0u8; 20];
        fin[0..4].copy_from_slice(&20u32.to_ne_bytes());
        fin[4..6].copy_from_slice(&NLMSG_DONE.to_ne_bytes());
        d.extend(fin);
        let m = decoder_datagramme(&d).unwrap();
        assert_eq!(m.len(), 3);
        assert_eq!(m[2], Message::Controle);
    }

    #[test]
    fn acquittement_et_erreur() {
        let mut a = vec![0u8; NLMSG_HDRLEN + 4 + NLMSG_HDRLEN];
        let n = a.len() as u32;
        a[0..4].copy_from_slice(&n.to_ne_bytes());
        a[4..6].copy_from_slice(&NLMSG_ERROR.to_ne_bytes());
        a[8..12].copy_from_slice(&9u32.to_ne_bytes());
        a[16..20].copy_from_slice(&(-16i32).to_ne_bytes());
        assert_eq!(decoder_datagramme(&a).unwrap(), vec![Message::Acquittement { sequence: 9, erreur: -16 }]);
        a.truncate(NLMSG_HDRLEN + 4);
        let n = a.len() as u32;
        a[0..4].copy_from_slice(&n.to_ne_bytes());
        assert!(decoder_datagramme(&a).is_err(), "NLMSG_ERROR tronqué");
    }

    #[test]
    fn refus_stricts() {
        let ok = trame().message(2, 110);
        // Longueur annoncée au-delà du datagramme.
        let mut m = ok.clone();
        m[0..4].copy_from_slice(&(ok.len() as u32 + 4).to_ne_bytes());
        assert!(matches!(decoder_datagramme(&m), Err(ErreurTrame::Longueur(_))));
        // Datagramme tronqué.
        assert!(decoder_datagramme(&ok[..ok.len() - 1]).is_err());
        // Octets de queue.
        let mut m = ok.clone();
        m.extend_from_slice(&[0; 8]);
        assert!(decoder_datagramme(&m).is_err());
        // Version nfgenmsg.
        let mut m = ok.clone();
        m[17] = 1;
        assert_eq!(decoder_datagramme(&m), Err(ErreurTrame::Version(1)));
        // Type de message d'un autre sous-système.
        let mut m = ok.clone();
        m[4..6].copy_from_slice(&0x0300u16.to_ne_bytes());
        assert_eq!(decoder_datagramme(&m), Err(ErreurTrame::TypeMessage(0x0300)));
        // Débordement signalé.
        let mut m = vec![0u8; 16];
        m[0..4].copy_from_slice(&16u32.to_ne_bytes());
        m[4..6].copy_from_slice(&NLMSG_OVERRUN.to_ne_bytes());
        assert_eq!(decoder_datagramme(&m), Err(ErreurTrame::Debordement));
    }

    #[test]
    fn refus_des_attributs() {
        let double = TrameEssai::new().be32(NFULA_MARK, 1).be32(NFULA_MARK, 2).message(2, 110);
        assert_eq!(decoder_datagramme(&double), Err(ErreurTrame::AttributDouble(NFULA_MARK)));
        let inconnu = TrameEssai::new().be32(22, 1).message(2, 110);
        assert_eq!(decoder_datagramme(&inconnu), Err(ErreurTrame::AttributInconnu(22)));
        let taille = TrameEssai::new().attribut(NFULA_MARK, &[1, 2]).message(2, 110);
        assert_eq!(decoder_datagramme(&taille), Err(ErreurTrame::TailleAttribut(NFULA_MARK)));
        let sans_nul = TrameEssai::new().attribut(NFULA_PREFIX, b"MDOU|").message(2, 110);
        assert!(matches!(decoder_datagramme(&sans_nul), Err(ErreurTrame::Valeur(_))));
        let nul_interne = TrameEssai::new().attribut(NFULA_PREFIX, b"MD\0OU|\0").message(2, 110);
        assert!(matches!(decoder_datagramme(&nul_interne), Err(ErreurTrame::Valeur(_))));
        let usec = TrameEssai::new().horodatage(1, 1_000_000).message(2, 110);
        assert!(matches!(decoder_datagramme(&usec), Err(ErreurTrame::Valeur(_))));
        let sec = TrameEssai::new().horodatage(u64::MAX, 0).message(2, 110);
        assert!(matches!(decoder_datagramme(&sec), Err(ErreurTrame::Valeur(_))));
        let charge = TrameEssai::new().attribut(NFULA_PAYLOAD, &vec![0; CHARGE_MAX + 1]).message(2, 110);
        assert!(matches!(decoder_datagramme(&charge), Err(ErreurTrame::Borne(_))));
        // nla_len < 4.
        let mut m = TrameEssai::new().be32(NFULA_MARK, 1).message(2, 110);
        m[20..22].copy_from_slice(&2u16.to_ne_bytes());
        assert!(matches!(decoder_datagramme(&m), Err(ErreurTrame::Longueur(_))));
    }

    #[test]
    fn drapeaux_de_type_masques() {
        // NLA_F_NESTED sur un attribut imbriqué (CT) : admis.
        let m = TrameEssai::new().attribut(NFULA_CT | 0x8000, &[0; 8]).message(2, 110);
        assert!(decoder_datagramme(&m).is_ok());
    }

    #[test]
    fn requetes_bien_formees() {
        let l = requete_liaison(1, 111);
        assert_eq!(l.len(), 16 + 4 + 8);
        assert_eq!(u16::from_ne_bytes([l[4], l[5]]), TYPE_CONFIG);
        assert_eq!(u16::from_be_bytes([l[18], l[19]]), 111);
        assert_eq!(&l[20..25], &[5, 0, 1, 0, NFULNL_CFG_CMD_BIND]);
        let p = requete_parametres(2, 111, ParametresInstance::default());
        assert_eq!(u32::from_ne_bytes(p[0..4].try_into().unwrap()) as usize, p.len());
        assert_eq!(p.len() % 4, 0);
        // MODE : nla_len 10, copy_range be32 = 128, mode 2.
        assert_eq!(&p[20..30], &[10, 0, 2, 0, 0, 0, 0, 128, NFULNL_COPY_PACKET, 0]);
    }
}
