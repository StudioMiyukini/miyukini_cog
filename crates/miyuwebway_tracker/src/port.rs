//! Tool MWS MiyuWebwayTracker — mws.port.check.
//! Vérifie si un port est exclus MWS ; exécution seule ; déterministe.

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayTrackerError;

/// @id: miyuwebway_tracker_mws_port_check
/// @role: accessor
/// @layer: tool
/// @human: Vérifie si un port est exclus MWS ; exécution seule ; déterministe.
/// @do: mws_port_check_under_governance
pub fn check(ctx: &GovernedContext, _port: u16) -> Result<bool, MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Err(MiyuwebwayTrackerError::Unimplemented)
}
