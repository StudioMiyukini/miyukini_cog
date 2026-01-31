//! Tools MWS MiyuWebwayParticipant — mws.declaration.* (build, sign, validate, verify).
//! Déclaration MWS : exécution seule ; pas de décision ; clé gouvernée.

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayParticipantError;

/// @id: miyuwebway_participant_mws_declaration_build
/// @role: mutator
/// @layer: tool
/// @human: Construit un message de déclaration conforme MWS ; exécution seule.
/// @do: mws_declaration_build_under_governance
pub fn build(ctx: &GovernedContext, _payload: &str) -> Result<Vec<u8>, MiyuwebwayParticipantError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayParticipantError::NoMandate);
    }
    Err(MiyuwebwayParticipantError::Unimplemented)
}

/// @id: miyuwebway_participant_mws_declaration_sign
/// @role: mutator
/// @layer: tool
/// @human: Signe une déclaration ; exécution seule ; clé gouvernée.
/// @do: mws_declaration_sign_under_governance
pub fn sign(ctx: &GovernedContext, _declaration: &[u8]) -> Result<Vec<u8>, MiyuwebwayParticipantError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayParticipantError::NoMandate);
    }
    Err(MiyuwebwayParticipantError::Unimplemented)
}

/// @id: miyuwebway_participant_mws_declaration_validate
/// @role: accessor
/// @layer: tool
/// @human: Valide le format d'une déclaration ; exécution seule.
/// @do: mws_declaration_validate_under_governance
pub fn validate(ctx: &GovernedContext, _declaration: &[u8]) -> Result<bool, MiyuwebwayParticipantError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayParticipantError::NoMandate);
    }
    Err(MiyuwebwayParticipantError::Unimplemented)
}

/// @id: miyuwebway_participant_mws_declaration_verify
/// @role: accessor
/// @layer: tool
/// @human: Vérifie la signature d'une déclaration ; exécution seule.
/// @do: mws_declaration_verify_under_governance
pub fn verify(ctx: &GovernedContext, _declaration: &[u8]) -> Result<bool, MiyuwebwayParticipantError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayParticipantError::NoMandate);
    }
    Err(MiyuwebwayParticipantError::Unimplemented)
}
