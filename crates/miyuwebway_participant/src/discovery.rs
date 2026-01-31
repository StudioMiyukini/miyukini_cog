//! Tools MWS MiyuWebwayParticipant — mws.discovery.request.* (build, send).
//! Requête de découverte ; exécution seule ; Trackers fournis par Cores.

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayParticipantError;

/// @id: miyuwebway_participant_mws_discovery_request_build
/// @role: mutator
/// @layer: tool
/// @human: Construit une requête de découverte ; exécution seule.
/// @do: mws_discovery_request_build_under_governance
pub fn request_build(ctx: &GovernedContext, _payload: Option<&str>) -> Result<Vec<u8>, MiyuwebwayParticipantError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayParticipantError::NoMandate);
    }
    Err(MiyuwebwayParticipantError::Unimplemented)
}

/// @id: miyuwebway_participant_mws_discovery_request_send
/// @role: mutator
/// @layer: tool
/// @human: Envoie une requête de découverte ; Trackers fournis par Cores.
/// @do: mws_discovery_request_send_under_governance
pub fn request_send(ctx: &GovernedContext, _trackers: &[String], _request: &[u8]) -> Result<Vec<u8>, MiyuwebwayParticipantError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayParticipantError::NoMandate);
    }
    Err(MiyuwebwayParticipantError::Unimplemented)
}
