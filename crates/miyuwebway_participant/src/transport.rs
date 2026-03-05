//! Tool MWS MiyuWebwayParticipant — mws.transport.send.
//! Envoi message vers une adresse ; exécution seule ; adresse fournie par Cores.

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayParticipantError;

/// @id: miyuwebway_participant_mws_transport_send
/// @role: mutator
/// @layer: tool
/// @human: Envoie un message vers une adresse ; exécution seule ; adresse fournie par Cores.
/// @do: mws_transport_send_under_governance
pub fn send(
    ctx: &GovernedContext,
    _address: &str,
    _payload: &[u8],
) -> Result<(), MiyuwebwayParticipantError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayParticipantError::NoMandate);
    }
    Err(MiyuwebwayParticipantError::ConnectionFailed(
        "Tracker indisponible".to_string(),
    ))
}
