//! @id     miyuwebway_tracker_lib
//! @do     declarer_module_address
//! @role   orchestration
//! @layer  core
//! @human  miyuwebway tracker — module « lib ».

#![allow(missing_docs)]
//! # MiyuWebwayTracker — toolkit.webway.tracker
//!
//! Kit d'outils MWS Tracker (déclaration, transport, découverte, cog_list, port, adresse).
//! Décisions (accepter, rejeter, filtrer) = Cores ; exécution seule dans le kit.
//! Alignement MIP : domaine `webway`, layer tool/toolkit.

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

pub mod address;
pub mod admin_cell;
pub mod cog_list;
pub mod context;
pub mod declaration;
pub mod discovery;
pub mod errors;
pub mod port;
pub mod transport;

pub use address::tracker_default as address_tracker_default;
pub use admin_cell::{
    miyuwebway_tracker_admin_cell, MiyuwebwayTrackerAdminCell, MiyuwebwayTrackerIdentification,
    MiyuwebwayTrackerIntegrity, MiyuwebwayTrackerTestManifest, TOOLKIT_ID,
};
pub use cog_list::{
    filter as cog_list_filter, get as cog_list_get, merge as cog_list_merge,
    update as cog_list_update, CogListStore, MAX_COG_ENTRIES, MAX_COG_ENTRY_BYTES,
};
pub use context::GovernedContext;
pub use declaration::{validate as declaration_validate, verify as declaration_verify};
pub use discovery::{
    response_build as discovery_response_build, response_send as discovery_response_send,
};
pub use errors::MiyuwebwayTrackerError;
pub use port::check as port_check;
pub use transport::{receive as transport_receive, send as transport_send};
