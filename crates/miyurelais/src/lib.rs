//! @id     miyurelais_lib
//! @do     declarer_modules_relais_nflog
//! @role   securite
//! @layer  operateur
//! @human  miyurelais — relais NFLOG des Vigies (lot 3) : source de paquets netlink en Rust pur, lignes MDOU|, métadonnées IPS, capteur muet ⇒ voie aveugle.

//! # MiyuRelais — relais NFLOG des Vigies
//!
//! Lot 3 « Frontières », reste : « source de paquets NFQUEUE / NFLOG pour
//! l'IPS et les Vigies » et « format de ligne du relais NFLOG à confirmer
//! quand le relais existe » (`docs/implementation/PLAN-IMPLEMENTATION.md` §5).
//!
//! Chaîne :
//!
//! ```text
//! nftables  log group <G> prefix "MDOU|<lieu>|<chaine>|<MOTIF>|"   (Douane, A-004)
//!    │ netlink NETLINK_NETFILTER, sous-système NFNL_SUBSYS_ULOG
//!    ▼
//! socket::SocketNflog      liaison au groupe, réception non bloquante
//! netlink::decoder_*       décodage strict et borné (refus en bloc)
//! relais::Relais           séquence (pertes), veille (capteur muet), lignes
//! ligne::convertir         ligne canonique MDOU| (format consigné au module)
//!    │ miyuvigie::SourceJournal
//!    ▼
//! Vigie::relever_journal → LotVigie → CaringNanny → WorrySentinel
//!                          (échec / muet ⇒ voie aveugle ⇒ coupée, A-004)
//! ligne::meta_flux → miyudouane::MetaFlux → MoteurIps (observation)
//! ```
//!
//! Rust pur, aucune bibliothèque C ni code GPL (libnetfilter_log /
//! libnetfilter_queue exclues, C-71) ; netlink par `rustix` (API sûre), aucun
//! `unsafe`.
//!
//! **NFQUEUE (verdicts en ligne) n'est pas implémenté** : voir
//! [`NFQUEUE_NON_IMPLEMENTE`].

#![forbid(unsafe_code)]
// C-10 : aucun unwrap / expect / panic! / todo! / unimplemented! / unreachable! hors tests.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::todo,
        clippy::unimplemented,
        clippy::unreachable
    )
)]

pub mod ip;
pub mod ligne;
pub mod netlink;
pub mod relais;
#[cfg(target_os = "linux")]
pub mod socket;
pub mod veille;

pub use ligne::{convertir, meta_flux, Contexte, Conversion, RefusPaquet, PREFIXE_TEMOIN};
pub use netlink::{decoder_datagramme, decoder_paquet, ErreurTrame, Message, PaquetNflog, ParametresInstance, TrameEssai};
pub use relais::{ConfigRelais, ErreurConfig, ErreurTransport, Relais, Statistiques, Transport, TransportEssai};
#[cfg(target_os = "linux")]
pub use socket::{ErreurOuverture, SocketNflog};
pub use veille::{EtatVeille, Veille};

/// Pourquoi la source NFQUEUE (verdicts en ligne) n'est pas implémentée ici.
///
/// - **Chemin critique** : avec NFQUEUE, chaque paquet attend le verdict de
///   l'espace utilisateur. Sans `bypass`, une panne du relais coupe tout le
///   trafic de la frontière (fail-closed, mais disponibilité) ; avec `bypass`,
///   elle laisse tout passer (fail-open, interdit par C-12). Le choix et la
///   supervision (redémarrage, délai) relèvent d'une décision d'architecture.
/// - **Correction** : identifiants de paquet, verdicts par lot, GSO
///   (`NFQA_CFG_F_GSO`), file pleine (`NFQA_CFG_F_FAIL_OPEN` à proscrire),
///   paquets modifiés : autant de chemins à spécifier et à revoir en sécurité.
/// - **Inutile pour le blocage** : nftables bloque déjà tout flux hors arrêté
///   (`policy drop`) ; l'IPS en ligne n'ajoute que le bannissement et la
///   coupure dynamique, que l'IPS reçoit aussi par observation
///   ([`ligne::meta_flux`] depuis NFLOG) puis applique en régénérant le jeu de
///   règles.
///
/// Le décodeur netlink ([`netlink`]) est réutilisable pour
/// `NFNL_SUBSYS_QUEUE` (même trame `nfgenmsg` + attributs).
pub const NFQUEUE_NON_IMPLEMENTE: &str = "NFQUEUE : verdicts en ligne non implémentés (chemin critique, décision d'architecture requise)";
