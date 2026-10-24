//! Tool MWS MiyuWebwayTracker — mws.port.check.
//! Vérifie si un port est exclus MWS ; exécution seule ; déterministe.

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayTrackerError;

/// @id miyuwebway_tracker_mws_port_check
/// @role accessor
/// @layer tool
/// @do mws_port_check_under_governance
//@human / @human Vérifie si un port est exclus MWS ; exécution seule ; déterministe.
/// Port 21000 réservé MWS (Tracker).
const MWS_TRACKER_PORT: u16 = 21000;

pub fn check(ctx: &GovernedContext, port: u16) -> Result<bool, MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Ok(port == MWS_TRACKER_PORT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn port_tracker_reserve() {
        let ctx = GovernedContext::new("m".into(), 1);
        assert_eq!(check(&ctx, 21000), Ok(true));
        assert_eq!(check(&ctx, 8080), Ok(false));
    }

    #[test]
    fn refuse_sans_mandat() {
        let empty = GovernedContext::new(String::new(), 1);
        assert_eq!(check(&empty, 21000), Err(MiyuwebwayTrackerError::NoMandate));
    }
}
