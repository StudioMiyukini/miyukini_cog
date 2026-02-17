//! Tool MWS MiyuWebwayParticipant — mws.port.check.
//! Vérifie si un port est exclus MWS ; exécution seule ; déterministe.

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayParticipantError;

/// @id: miyuwebway_participant_mws_port_check
/// @role: accessor
/// @layer: tool
/// @human: Vérifie si un port est exclus MWS ; exécution seule ; déterministe.
/// @do: mws_port_check_under_governance
/// Port réservé MWS Tracker (défaut).
const MWS_TRACKER_PORT: u16 = 21000;

pub fn check(ctx: &GovernedContext, port: u16) -> Result<bool, MiyuwebwayParticipantError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayParticipantError::NoMandate);
    }
    Ok(port == MWS_TRACKER_PORT)
}
