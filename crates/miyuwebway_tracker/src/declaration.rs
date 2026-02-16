//! Tools MWS MiyuWebwayTracker — mws.declaration.validate, mws.declaration.verify.
//! Validation et vérification signature d'une déclaration reçue ; exécution seule.

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayTrackerError;

/// @id: miyuwebway_tracker_mws_declaration_validate
/// @role: accessor
/// @layer: tool
/// @human: Valide le format d'une déclaration reçue ; exécution seule.
/// @do: mws_declaration_validate_under_governance
/// @id: miyuwebway_tracker_mws_declaration_validate
/// @role: accessor
/// @layer: tool
/// @human: Valide le format d'une déclaration reçue ; exécution seule.
/// @do: mws_declaration_validate_under_governance
/// Stub : format non vide = valide ; vérification réelle déléguée à MWS/Origin.
pub fn validate(ctx: &GovernedContext, declaration: &[u8]) -> Result<bool, MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Ok(!declaration.is_empty())
}

/// @id: miyuwebway_tracker_mws_declaration_verify
/// @role: accessor
/// @layer: tool
/// @human: Vérifie la signature d'une déclaration reçue ; exécution seule.
/// @do: mws_declaration_verify_under_governance
/// Stub : pas de vérification signature ; retourne false (non vérifié).
pub fn verify(ctx: &GovernedContext, _declaration: &[u8]) -> Result<bool, MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    Ok(false)
}
