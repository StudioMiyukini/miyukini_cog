//! Tool MiyuAuth — tool.identity.verify.
//! Vérifie l'intégrité et la validité d'un Passeport ou Visa ; ne décide pas de l'autorisation.

use crate::context::GovernedContext;
use crate::errors::MiyauthError;
use crate::types::VerificationResult;

/// @id: miyauth_tool_identity_verify
/// @role: mutator
/// @layer: tool
/// @human: Vérifie l'intégrité et la validité d'un Passeport Utilisateur ou Visa de Connexion.
/// @do: identity_verify_under_governance
/// tool.identity.verify — ne décide pas de l'autorisation (ALLOW/DENY = StrongFather).
pub fn verify(
    ctx: &GovernedContext,
    _passport_or_visa_raw: &[u8],
) -> Result<VerificationResult, MiyauthError> {
    if !ctx.has_mandate() {
        return Err(MiyauthError::NoMandate);
    }
    Err(MiyauthError::Unimplemented)
}
