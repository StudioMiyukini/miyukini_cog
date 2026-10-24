//! Tool MiyuAuth — tool.identity.attest.
//! Produit une attestation d'identité à partir du contexte validé ; ne crée pas la confiance.

use crate::context::GovernedContext;
use crate::errors::MiyauthError;
use crate::types::{Attestation, IdentityContext};

/// @id miyauth_tool_identity_attest
/// @role mutator
/// @layer tool
/// @do identity_attest_under_governance
//@human / @human Produit une attestation d'identité à partir du contexte validé.
/// tool.identity.attest — ne crée pas la confiance.
pub fn attest(
    ctx: &GovernedContext,
    _context: &IdentityContext,
) -> Result<Attestation, MiyauthError> {
    if !ctx.has_mandate() {
        return Err(MiyauthError::NoMandate);
    }
    // Fail-closed (audit C8) : une attestation non liée au contexte et non signée serait
    // une fausse preuve. Lot 2 : Ed25519 — attestation signée dérivée de `_context`.
    Err(MiyauthError::NotImplemented(
        "identity.attest (attestation signée Ed25519, lot 2)",
    ))
}
