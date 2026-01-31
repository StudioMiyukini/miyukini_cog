//! Tool MiyuAuth — tool.identity.resolve.
//! Résout le contexte d'identité à partir des artefacts fournis ; ne décide pas de la confiance.

use crate::context::GovernedContext;
use crate::errors::MiyauthError;
use crate::types::{IdentityArtefacts, IdentityContext};

/// @id: miyauth_tool_identity_resolve
/// @role: mutator
/// @layer: tool
/// @human: Résout le contexte d'identité à partir des artefacts fournis.
/// @do: identity_resolve_under_governance
/// tool.identity.resolve — ne décide pas de la confiance.
pub fn resolve(
    ctx: &GovernedContext,
    _artefacts: &IdentityArtefacts,
) -> Result<IdentityContext, MiyauthError> {
    if !ctx.has_mandate() {
        return Err(MiyauthError::NoMandate);
    }
    Err(MiyauthError::Unimplemented)
}
