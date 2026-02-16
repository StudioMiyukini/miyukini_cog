//! Tool MiyuAuth — tool.identity.resolve.
//! Résout le contexte d'identité à partir des artefacts fournis ; ne décide pas de la confiance.
//! Toute confiance utilisée pour l'identité est validée par KindMother (BOUND-3).

use crate::context::GovernedContext;
use crate::errors::MiyauthError;
use crate::types::{IdentityArtefacts, IdentityContext, IdentityRole};
use miyukini_kernel::{IdGenerator, UuidIdGenerator};

/// @id: miyauth_tool_identity_resolve
/// @role: mutator
/// @layer: tool
/// @human: Résout le contexte d'identité à partir des artefacts fournis.
/// @do: identity_resolve_under_governance
/// tool.identity.resolve — ne décide pas de la confiance.
pub fn resolve(
    ctx: &GovernedContext,
    artefacts: &IdentityArtefacts,
) -> Result<IdentityContext, MiyauthError> {
    if !ctx.has_mandate() {
        return Err(MiyauthError::NoMandate);
    }
    let role = derive_role(artefacts);
    let opaque_id = artefacts
        .session_ref
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(String::from)
        .unwrap_or_else(|| format!("opaque:{}", UuidIdGenerator.generate()));
    Ok(IdentityContext { role, opaque_id })
}

/// Dérive le rôle à partir des artefacts (données déjà validées par KindMother).
/// Citizen = Passeport non vide ; Visitor = Visa non vide sans Passeport ; sinon External.
fn derive_role(artefacts: &IdentityArtefacts) -> IdentityRole {
    let has_passport = artefacts
        .passport_raw
        .as_deref()
        .map_or(false, |b| !b.is_empty());
    let has_visa = artefacts
        .visa_raw
        .as_deref()
        .map_or(false, |b| !b.is_empty());
    if has_passport {
        IdentityRole::Citizen
    } else if has_visa {
        IdentityRole::Visitor
    } else {
        IdentityRole::External
    }
}
