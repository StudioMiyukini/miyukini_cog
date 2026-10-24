//! Tools MWS MiyuWebwayParticipant — mws.declaration.* (build, sign, validate, verify).
//! Déclaration MWS : exécution seule ; pas de décision ; clé gouvernée.

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayParticipantError;

/// @id miyuwebway_participant_mws_declaration_build
/// @role mutator
/// @layer tool
/// @do mws_declaration_build_under_governance
//@human / @human Construit un message de déclaration conforme MWS ; exécution seule.
pub fn build(ctx: &GovernedContext, payload: &str) -> Result<Vec<u8>, MiyuwebwayParticipantError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayParticipantError::NoMandate);
    }
    Ok(payload.as_bytes().to_vec())
}

/// @id miyuwebway_participant_mws_declaration_sign
/// @role mutator
/// @layer tool
/// @do mws_declaration_sign_under_governance
//@human / @human Signe une déclaration ; exécution seule ; clé gouvernée.
pub fn sign(
    ctx: &GovernedContext,
    declaration: &[u8],
) -> Result<Vec<u8>, MiyuwebwayParticipantError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayParticipantError::NoMandate);
    }
    // Fail-closed (audit C8) : renvoyer l'entrée simulerait une signature inexistante.
    // lot 2 : Ed25519 (signature avec clé gouvernée).
    let _ = declaration;
    Err(MiyuwebwayParticipantError::Unimplemented)
}

/// @id miyuwebway_participant_mws_declaration_validate
/// @role accessor
/// @layer tool
/// @do mws_declaration_validate_under_governance
//@human / @human Valide le format d'une déclaration ; exécution seule.
pub fn validate(
    ctx: &GovernedContext,
    declaration: &[u8],
) -> Result<bool, MiyuwebwayParticipantError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayParticipantError::NoMandate);
    }
    Ok(!declaration.is_empty())
}

/// @id miyuwebway_participant_mws_declaration_verify
/// @role accessor
/// @layer tool
/// @do mws_declaration_verify_under_governance
//@human / @human Vérifie la signature d'une déclaration ; exécution seule.
pub fn verify(
    ctx: &GovernedContext,
    _declaration: &[u8],
) -> Result<bool, MiyuwebwayParticipantError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayParticipantError::NoMandate);
    }
    // Fail-closed (audit C8) : ne jamais renvoyer Ok(true) sans vérification réelle.
    // lot 2 : Ed25519 (vérification de signature).
    Err(MiyuwebwayParticipantError::Unimplemented)
}
