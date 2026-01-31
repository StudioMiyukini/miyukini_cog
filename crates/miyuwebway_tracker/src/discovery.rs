//! Tools MWS MiyuWebwayTracker — mws.discovery.response.* (build, send).
//! Réponse de découverte ; liste filtrée (critère fourni par Cores) ; envoi au demandeur.

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayTrackerError;

/// @id: miyuwebway_tracker_mws_discovery_response_build
/// @role: mutator
/// @layer: tool
/// @human: Construit une réponse de découverte ; liste filtrée ; critère fourni par Cores.
/// @do: mws_discovery_response_build_under_governance
pub fn response_build(ctx: &GovernedContext, _cog_list: &[String], _criterion_ref: &str) -> Result<Vec<u8>, MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Err(MiyuwebwayTrackerError::Unimplemented)
}

/// @id: miyuwebway_tracker_mws_discovery_response_send
/// @role: mutator
/// @layer: tool
/// @human: Envoie la réponse au demandeur ; exécution seule.
/// @do: mws_discovery_response_send_under_governance
pub fn response_send(ctx: &GovernedContext, _address: &str, _response: &[u8]) -> Result<(), MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Err(MiyuwebwayTrackerError::Unimplemented)
}
