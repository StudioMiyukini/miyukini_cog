//! Tool MWS MiyuWebwayParticipant — mws.address.tracker_default.
//! Résout l'adresse Tracker (port 21000) ; exécution seule ; déterministe.

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayParticipantError;

/// @id: miyuwebway_participant_mws_address_tracker_default
/// @role: accessor
/// @layer: tool
/// @human: Résout l'adresse Tracker (port 21000) ; exécution seule ; déterministe.
/// @do: mws_address_tracker_default_under_governance
pub fn tracker_default(
    ctx: &GovernedContext,
    host: Option<&str>,
) -> Result<String, MiyuwebwayParticipantError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayParticipantError::NoMandate);
    }
    let addr = host
        .map(|h| format!("{h}:21000"))
        .unwrap_or_else(|| "127.0.0.1:21000".to_string());
    Ok(addr)
}
