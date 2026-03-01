//! # miyualicia-http
//!
//! Client HTTP pour les dispositifs domotiques locaux d'Alicia Home Assistante.
//! Fournit un client avec timeout, retry, authentification Basic/Bearer,
//! et des adaptateurs specifiques (Shelly Gen1/Gen2, generique JSON).
//!
//! ## Loi d'Autonomie
//!
//! Ce crate communique exclusivement avec des dispositifs sur le reseau local (LAN).
//! Aucune requete vers Internet n'est supportee ni prevue.

#![forbid(unsafe_code)]

// @id: toolkit.alicia.http
// @role: http_protocol_adapter
// @layer: 6
// @human: Client HTTP pour dispositifs domotiques locaux Alicia ; Shelly, generique, timeout.
// @do: provide_http_transport_for_local_devices

pub mod adapters;
pub mod admin_cell;
pub mod client;
pub mod errors;

pub use client::HttpDeviceClient;
pub use errors::HttpError;
