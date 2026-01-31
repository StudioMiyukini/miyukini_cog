//! Tool MiyuAuth — tool.identity.role.
//! Détermine le rôle identité (citoyen, visiteur, externe) à partir du contexte validé.

use crate::context::GovernedContext;
use crate::errors::MiyauthError;
use crate::types::{IdentityContext, IdentityRole};

/// @id: miyauth_tool_identity_role
/// @role: mutator
/// @layer: tool
/// @human: Détermine le rôle identité à partir du contexte validé par KindMother.
/// @do: identity_role_under_governance
/// tool.identity.role — ne décide pas de l'autorisation.
pub fn role(ctx: &GovernedContext, context: &IdentityContext) -> Result<IdentityRole, MiyauthError> {
    if !ctx.has_mandate() {
        return Err(MiyauthError::NoMandate);
    }
    Ok(context.role)
}
