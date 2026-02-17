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
mod store;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GovernedContext {
        GovernedContext::new("m".into(), 0)
    }

    #[test]
    fn test_address_tracker_default() {
        let c = ctx();
        assert_eq!(address_tracker_default(&c, None).unwrap(), "127.0.0.1:21000");
        assert_eq!(address_tracker_default(&c, Some("host.example")).unwrap(), "host.example:21000");
    }

    #[test]
    fn test_transport_send_tracker_indisponible() {
        let c = ctx();
        let e = transport_send(&c, "addr", b"payload").unwrap_err();
        assert!(matches!(e, MiyuwebwayParticipantError::ConnectionFailed(ref m) if m.contains("Tracker indisponible")));
    }

    #[test]
    fn test_port_check() {
        let c = ctx();
        assert!(port_check(&c, 21000).unwrap());
        assert!(!port_check(&c, 8080).unwrap());
    }

    #[test]
    fn test_discovery_build_send() {
        let c = ctx();
        assert_eq!(discovery_request_build(&c, Some("p")).unwrap(), b"p");
        assert!(discovery_request_build(&c, None).unwrap().is_empty());
        let e = discovery_request_send(&c, &[], b"").unwrap_err();
        assert!(matches!(e, MiyuwebwayParticipantError::ConnectionFailed(_)));
    }

    #[test]
    fn test_declaration_build_sign_validate_verify() {
        let c = ctx();
        let raw = declaration_build(&c, "payload").unwrap();
        assert_eq!(raw, b"payload");
        let signed = declaration_sign(&c, &raw).unwrap();
        assert!(declaration_validate(&c, &signed).unwrap());
        assert!(declaration_verify(&c, &signed).unwrap());
    }

    #[test]
    fn test_cog_list_get_update_merge() {
        let c = ctx();
        assert!(cog_list_get(&c).unwrap().is_empty());
        cog_list_update(&c, "cog-1", "data").unwrap();
        let list = cog_list_get(&c).unwrap();
        assert_eq!(list.len(), 1);
        assert!(list.contains(&"cog-1".to_string()));
        cog_list_merge(&c, "cog-2\ncog-3", "rule").unwrap();
        let list = cog_list_get(&c).unwrap();
        assert!(list.len() >= 3);
        assert!(list.contains(&"cog-2".to_string()));
        assert!(list.contains(&"cog-3".to_string()));
    }

    #[test]
    fn test_no_mandate_refused() {
        let c = GovernedContext::new(String::new(), 0);
        assert!(address_tracker_default(&c, None).is_err());
        assert!(cog_list_get(&c).is_err());
    }
}
