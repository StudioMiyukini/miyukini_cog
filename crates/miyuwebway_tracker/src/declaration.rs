//! Tools MWS MiyuWebwayTracker — mws.declaration.validate, mws.declaration.verify.
//! Validation et vérification signature d'une déclaration reçue ; exécution seule.
//!
//! Non implémentés : le format de déclaration MWS n'est pas encore figé et la
//! vérification de signature (Ed25519, lot 2) n'existe pas. Refus explicite
//! (C-14) — un `Ok(true)` sur simple non-vacuité (ancien stub) laissait passer
//! n'importe quelle déclaration (fail-open).

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayTrackerError;

/// @id miyuwebway_tracker_mws_declaration_validate
/// @role accessor
/// @layer tool
/// @do mws_declaration_validate_under_governance
//@human / @human Valide le format d'une déclaration reçue ; exécution seule.
/// Refus explicite : validation de format non implémentée.
pub fn validate(
    ctx: &GovernedContext,
    _declaration: &[u8],
) -> Result<bool, MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Err(MiyuwebwayTrackerError::Unimplemented(
        "mws.declaration.validate",
    ))
}

/// @id miyuwebway_tracker_mws_declaration_verify
/// @role accessor
/// @layer tool
/// @do mws_declaration_verify_under_governance
//@human / @human Vérifie la signature d'une déclaration reçue ; exécution seule.
/// Refus explicite (audit C8, C-14) : aucune vérification de signature n'existe
/// encore ; lot 2 : Ed25519 via les primitives autorisées (C-24).
pub fn verify(ctx: &GovernedContext, _declaration: &[u8]) -> Result<bool, MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Err(MiyuwebwayTrackerError::Unimplemented(
        "mws.declaration.verify",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declaration_refuse_explicitement() {
        let ctx = GovernedContext::new("m".into(), 2);
        assert_eq!(
            validate(&ctx, b"declaration non vide"),
            Err(MiyuwebwayTrackerError::Unimplemented(
                "mws.declaration.validate"
            ))
        );
        assert_eq!(
            verify(&ctx, b"declaration signee ?"),
            Err(MiyuwebwayTrackerError::Unimplemented(
                "mws.declaration.verify"
            ))
        );
    }

    #[test]
    fn declaration_refuse_sans_mandat() {
        let empty = GovernedContext::new(String::new(), 2);
        assert_eq!(
            validate(&empty, b"x"),
            Err(MiyuwebwayTrackerError::NoMandate)
        );
        assert_eq!(verify(&empty, b"x"), Err(MiyuwebwayTrackerError::NoMandate));
    }
}
