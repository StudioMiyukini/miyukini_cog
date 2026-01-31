//! # MiyuWebwayTracker — toolkit.webway.tracker
//!
//! Kit d'outils MWS Tracker (déclaration, transport, découverte, cog_list, port, adresse).
//! Décisions (accepter, rejeter, filtrer) = Cores ; exécution seule dans le kit.
//! Alignement MIP : domaine `webway`, layer tool/toolkit.

pub mod admin_cell;
pub mod address;
pub mod cog_list;
pub mod context;
pub mod declaration;
pub mod discovery;
pub mod errors;
pub mod port;
pub mod transport;

pub use admin_cell::{
    miyuwebway_tracker_admin_cell, MiyuwebwayTrackerAdminCell, MiyuwebwayTrackerIdentification,
    MiyuwebwayTrackerIntegrity, MiyuwebwayTrackerTestManifest, TOOLKIT_ID,
};
pub use address::tracker_default as address_tracker_default;
pub use cog_list::{filter as cog_list_filter, get as cog_list_get, merge as cog_list_merge, update as cog_list_update};
pub use context::GovernedContext;
pub use declaration::{validate as declaration_validate, verify as declaration_verify};
pub use discovery::{response_build as discovery_response_build, response_send as discovery_response_send};
pub use errors::MiyuwebwayTrackerError;
pub use port::check as port_check;
pub use transport::{receive as transport_receive, send as transport_send};
