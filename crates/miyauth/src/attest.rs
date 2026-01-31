//! Tool MiyuAuth — tool.identity.attest.
//! Produit une attestation d'identité à partir du contexte validé ; ne crée pas la confiance.

use crate::context::GovernedContext;
use crate::errors::MiyauthError;
use crate::types::{IdentityContext, Attestation};

/// @id: miyauth_tool_identity_attest
/// @role: mutator
/// @layer: tool
/// @human: Produit une attestation d'identité à partir du contexte validé.
/// @do: identity_attest_under_governance
/// tool.identity.attest — ne crée pas la confiance.
pub fn attest(ctx: &GovernedContext, _context: &IdentityContext) -> Result<Attestation, MiyauthError> {
    if !ctx.has_mandate() {
        return Err(MiyauthError::NoMandate);
    }
    Err(MiyauthError::Unimplemented)
}
