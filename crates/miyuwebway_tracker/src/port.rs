//! Tool MWS MiyuWebwayTracker — mws.port.check.
//! Vérifie si un port est exclus MWS ; exécution seule ; déterministe.

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayTrackerError;

/// @id: miyuwebway_tracker_mws_port_check
/// @role: accessor
/// @layer: tool
/// @human: Vérifie si un port est exclus MWS ; exécution seule ; déterministe.
/// @do: mws_port_check_under_governance
/// Port 21000 réservé MWS (Tracker).
const MWS_TRACKER_PORT: u16 = 21000;

pub fn check(ctx: &GovernedContext, port: u16) -> Result<bool, MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Ok(port == MWS_TRACKER_PORT)
}
