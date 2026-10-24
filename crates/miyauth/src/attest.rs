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
    // Fail-closed (audit C8) : une attestation non signée serait une fausse preuve.
    // Lot 2 incr. 4 : la doc gelée v1.2 n'attribue aucun code de type Sceau à une
    // attestation (types 0x01 à 0x07) ni de format : refus maintenu jusqu'à un
    // amendement. Le visa vérifié (`AccesVerifie`) tient lieu d'attestation.
    Err(MiyauthError::NotImplemented(
        "identity.attest (aucun type Sceau d'attestation dans la doc gelée v1.2)",
    ))
}
