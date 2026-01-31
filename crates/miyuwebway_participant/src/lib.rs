//! # MiyuWebwayParticipant — toolkit.webway.participant
//!
//! Kit d'outils MWS Participant (déclaration, transport, découverte, cog_list, port, adresse).
//! Décisions = Cores ; exécution seule ; pas de décision dans le kit.
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
    miyuwebway_participant_admin_cell, MiyuwebwayParticipantAdminCell,
    MiyuwebwayParticipantIdentification, MiyuwebwayParticipantIntegrity,
    MiyuwebwayParticipantTestManifest, TOOLKIT_ID,
};
pub use address::tracker_default as address_tracker_default;
pub use cog_list::{get as cog_list_get, merge as cog_list_merge, update as cog_list_update};
pub use context::GovernedContext;
pub use declaration::{build as declaration_build, sign as declaration_sign, validate as declaration_validate, verify as declaration_verify};
pub use discovery::{request_build as discovery_request_build, request_send as discovery_request_send};
pub use errors::MiyuwebwayParticipantError;
pub use port::check as port_check;
pub use transport::send as transport_send;
