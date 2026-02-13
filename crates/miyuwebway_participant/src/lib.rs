#![allow(missing_docs)]
//! # MiyuWebwayParticipant — toolkit.webway.participant
//!
//! Kit d'outils MWS Participant pour la connexion au réseau Miyukini Webway.
//!
//! ## Fonctionnalités
//!
//! - **Relay Client** : Connexion TLS à Origin, enregistrement, heartbeat
//! - **Tracker Client** : Annonce, découverte de COGs/lobbys
//! - **MWS Service** : Service de haut niveau coordonnant le tout
//!
//! ## Exemple d'utilisation
//!
//! ```ignore
//! use miyuwebway_participant::{MwsService, CogIdentity};
//!
//! let service = MwsService::with_defaults();
//! let identity = CogIdentity {
//!     cog_id: "my-cog".to_string(),
//!     core_version: "1.0.0".to_string(),
//!     public_address: "127.0.0.1:8080".to_string(),
//!     services: vec!["jayfestival".to_string()],
//! };
//!
//! service.start(identity).await?;
//! ```
//!
//! Décisions = Cores ; exécution seule ; pas de décision dans le kit.
//! Alignement MIP : domaine `webway`, layer tool/toolkit.

// Modules existants
pub mod admin_cell;
pub mod address;
pub mod cog_list;
pub mod context;
pub mod declaration;
pub mod discovery;
pub mod errors;
pub mod port;
pub mod transport;

// Nouveaux modules MWS
pub mod protocol;
pub mod relay_client;
pub mod tracker_client;
pub mod mws_service;

// Ré-exports admin_cell
pub use admin_cell::{
    miyuwebway_participant_admin_cell, MiyuwebwayParticipantAdminCell,
    MiyuwebwayParticipantIdentification, MiyuwebwayParticipantIntegrity,
    MiyuwebwayParticipantTestManifest, TOOLKIT_ID,
};

// Ré-exports outils gouvernés
pub use address::tracker_default as address_tracker_default;
pub use cog_list::{get as cog_list_get, merge as cog_list_merge, update as cog_list_update};
pub use context::GovernedContext;
pub use declaration::{
    build as declaration_build, sign as declaration_sign, validate as declaration_validate,
    verify as declaration_verify,
};
pub use discovery::{request_build as discovery_request_build, request_send as discovery_request_send};
pub use errors::MiyuwebwayParticipantError;
pub use port::check as port_check;
pub use transport::send as transport_send;

// Ré-exports MWS client
pub use mws_service::{CogIdentity, MwsService, MwsServiceConfig, MwsServiceState};
pub use relay_client::{RelayClient, RelayClientConfig, RelaySession, RelaySessionState};
pub use tracker_client::{TrackerAnnouncement, TrackerClient, TrackerClientConfig, TrackerState};
pub use protocol::{
    CogInfo, LobbyInfo, LobbySearchResult,
    RelayFrame, RelayMessageType,
    TrackerFrame, TrackerMessageType,
};
