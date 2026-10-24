//! Tools MWS MiyuWebwayTracker — mws.transport.receive, mws.transport.send.
//! Réception sur endpoint (port 21000 officiel) ; envoi (adresse fournie).
//!
//! Non implémentés : la délégation réseau vers le Tracker (Origin) n'existe
//! pas encore ; refus explicite (C-14), jamais de succès silencieux.

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayTrackerError;

/// @id miyuwebway_tracker_mws_transport_receive
/// @role mutator
/// @layer tool
/// @do mws_transport_receive_under_governance
//@human / @human Reçoit un message sur un endpoint ; exécution seule ; port 21000 officiel.
/// Refus explicite : délégation réseau vers Tracker (Origin) non implémentée.
pub fn receive(ctx: &GovernedContext) -> Result<Vec<u8>, MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Err(MiyuwebwayTrackerError::Unimplemented(
        "mws.transport.receive",
    ))
}

/// @id miyuwebway_tracker_mws_transport_send
/// @role mutator
/// @layer tool
/// @do mws_transport_send_under_governance
//@human / @human Envoie un message (réponse, liste statuts) ; adresse fournie.
/// Refus explicite : délégation réseau non implémentée.
pub fn send(
    ctx: &GovernedContext,
    _address: &str,
    _payload: &[u8],
) -> Result<(), MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Err(MiyuwebwayTrackerError::Unimplemented("mws.transport.send"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transport_refuse_explicitement() {
        let ctx = GovernedContext::new("m".into(), 2);
        assert_eq!(
            receive(&ctx),
            Err(MiyuwebwayTrackerError::Unimplemented(
                "mws.transport.receive"
            ))
        );
        assert_eq!(
            send(&ctx, "127.0.0.1:21000", b"x"),
            Err(MiyuwebwayTrackerError::Unimplemented("mws.transport.send"))
        );
    }

    #[test]
    fn transport_refuse_sans_mandat() {
        let empty = GovernedContext::new(String::new(), 2);
        assert_eq!(receive(&empty), Err(MiyuwebwayTrackerError::NoMandate));
        assert_eq!(
            send(&empty, "a", b"x"),
            Err(MiyuwebwayTrackerError::NoMandate)
        );
    }
}
