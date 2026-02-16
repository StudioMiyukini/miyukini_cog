//! Tools MWS MiyuWebwayTracker — mws.discovery.response.* (build, send).
//! Réponse de découverte ; liste filtrée (critère fourni par Cores) ; envoi au demandeur.

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayTrackerError;

/// @id: miyuwebway_tracker_mws_discovery_response_build
/// @role: mutator
/// @layer: tool
/// @human: Construit une réponse de découverte ; liste filtrée ; critère fourni par Cores.
/// @do: mws_discovery_response_build_under_governance
/// Stub : retourne réponse vide ; sérialisation réelle MWS déléguée.
pub fn response_build(ctx: &GovernedContext, cog_list: &[String], _criterion_ref: &str) -> Result<Vec<u8>, MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Ok(cog_list.join("\n").into_bytes())
}

/// @id: miyuwebway_tracker_mws_discovery_response_send
/// @role: mutator
/// @layer: tool
/// @human: Envoie la réponse au demandeur ; exécution seule.
/// @do: mws_discovery_response_send_under_governance
/// Stub : délégation réseau non implémentée.
pub fn response_send(ctx: &GovernedContext, _address: &str, _response: &[u8]) -> Result<(), MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Err(MiyuwebwayTrackerError::TrackerUnavailable("response_send: network delegation not implemented".into()))
}
