//! Tools MWS MiyuWebwayTracker — mws.discovery.response.* (build, send).
//! Réponse de découverte ; liste filtrée (critère fourni par Cores) ; envoi au demandeur.
//!
//! Non implémentés : le critère de filtrage des Cores n'est pas encore
//! interprétable et la délégation réseau n'existe pas. Construire une réponse
//! en ignorant le critère exposerait des COGs rejetés (fail-open) : refus
//! explicite (C-12, C-14).

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayTrackerError;

/// @id miyuwebway_tracker_mws_discovery_response_build
/// @role mutator
/// @layer tool
/// @do mws_discovery_response_build_under_governance
//@human / @human Construit une réponse de découverte ; liste filtrée ; critère fourni par Cores.
/// Refus explicite : application du critère et sérialisation MWS non implémentées.
pub fn response_build(
    ctx: &GovernedContext,
    _cog_list: &[String],
    _criterion_ref: &str,
) -> Result<Vec<u8>, MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Err(MiyuwebwayTrackerError::Unimplemented(
        "mws.discovery.response.build",
    ))
}

/// @id miyuwebway_tracker_mws_discovery_response_send
/// @role mutator
/// @layer tool
/// @do mws_discovery_response_send_under_governance
//@human / @human Envoie la réponse au demandeur ; exécution seule.
/// Refus explicite : délégation réseau non implémentée.
pub fn response_send(
    ctx: &GovernedContext,
    _address: &str,
    _response: &[u8],
) -> Result<(), MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Err(MiyuwebwayTrackerError::Unimplemented(
        "mws.discovery.response.send",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decouverte_refuse_explicitement() {
        let ctx = GovernedContext::new("m".into(), 2);
        assert_eq!(
            response_build(&ctx, &["cog-a".to_string()], "exclude:rejected"),
            Err(MiyuwebwayTrackerError::Unimplemented(
                "mws.discovery.response.build"
            ))
        );
        assert_eq!(
            response_send(&ctx, "127.0.0.1:21000", b"x"),
            Err(MiyuwebwayTrackerError::Unimplemented(
                "mws.discovery.response.send"
            ))
        );
    }

    #[test]
    fn decouverte_refuse_sans_mandat() {
        let empty = GovernedContext::new(String::new(), 2);
        assert_eq!(
            response_build(&empty, &[], ""),
            Err(MiyuwebwayTrackerError::NoMandate)
        );
        assert_eq!(
            response_send(&empty, "", b""),
            Err(MiyuwebwayTrackerError::NoMandate)
        );
    }
}
