//! CentralRemote — contrôle à distance de Central depuis le navigateur.
//!
//! @id     central_remote_lib
//! @do expose_central_remote_api
//@human ! @role   toolkit
//! @layer toolkit
//! @human  Protocole (commandes/événements), session auth, et serveur embarqué pour
//! le contrôle de Central via le portail web COG.
//!
//! ## Architecture
//!
//! - `protocol` : types partagés (RemoteCommand, RemoteEvent, RemoteSnapshot)
//! - `identite` : Central COG d'origine — passeports d'appareil, visas, preuve
//!   de possession, révocation (sortie du lot 2 : plus de secret partagé)
//! - `pairing` : appairage v2 (racine de confiance des clés d'appareil)
//! - `server` : serveur Axum embarqué lancé par Central quand le remote est activé
//! - `bridge` : état partagé entre Central (Dioxus) et le serveur remote

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

pub mod bridge;
pub mod discovery;
pub mod e2e;
pub mod identite;
pub mod paired_store;
pub mod pairing;
pub mod protocol;
pub mod server;
