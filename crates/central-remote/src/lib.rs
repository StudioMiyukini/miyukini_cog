//! CentralRemote — contrôle à distance de Central depuis le navigateur.
//!
//! @id: central_remote_lib @do: expose_central_remote_api
//! @role: toolkit @layer: toolkit
//! @human: Protocole (commandes/événements), session auth, et serveur embarqué pour
//! le contrôle de Central via le portail web COG.
//!
//! ## Architecture
//!
//! - `protocol` : types partagés (RemoteCommand, RemoteEvent, RemoteSnapshot)
//! - `session` : gestion des sessions authentifiées (token HMAC, expiration)
//! - `server` : serveur Axum embarqué lancé par Central quand le remote est activé
//! - `bridge` : état partagé entre Central (Dioxus) et le serveur remote

pub mod bridge;
pub mod discovery;
pub mod pairing;
pub mod protocol;
pub mod server;
pub mod session;
