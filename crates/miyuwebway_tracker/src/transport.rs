//! Tools MWS MiyuWebwayTracker — mws.transport.receive, mws.transport.send.
//! Réception sur endpoint (port 21000 officiel) ; envoi (adresse fournie).

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayTrackerError;

/// @id: miyuwebway_tracker_mws_transport_receive
/// @role: mutator
/// @layer: tool
/// @human: Reçoit un message sur un endpoint ; exécution seule ; port 21000 officiel.
/// @do: mws_transport_receive_under_governance
/// Stub : délégation réseau vers Tracker (Origin) non implémentée.
pub fn receive(ctx: &GovernedContext) -> Result<Vec<u8>, MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Err(MiyuwebwayTrackerError::TrackerUnavailable(
        "transport_receive: network delegation not implemented".into(),
    ))
}

/// @id: miyuwebway_tracker_mws_transport_send
/// @role: mutator
/// @layer: tool
/// @human: Envoie un message (réponse, liste statuts) ; adresse fournie.
/// @do: mws_transport_send_under_governance
/// Stub : délégation réseau non implémentée.
pub fn send(
    ctx: &GovernedContext,
    _address: &str,
    _payload: &[u8],
) -> Result<(), MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Err(MiyuwebwayTrackerError::TrackerUnavailable(
        "transport_send: network delegation not implemented".into(),
    ))
}
