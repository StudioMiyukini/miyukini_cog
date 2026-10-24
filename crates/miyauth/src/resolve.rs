//! Tool MiyuAuth — tool.identity.resolve.
//! Résout le contexte d'identité à partir des artefacts fournis ; ne décide pas de la confiance.
//! Toute confiance utilisée pour l'identité est validée par KindMother (BOUND-3).

use crate::context::GovernedContext;
use crate::errors::MiyauthError;
use crate::verify::VerifiedArtifact;
use crate::types::{IdentityArtefacts, IdentityContext, IdentityRole};
use miyukini_kernel::{IdGenerator, UuidIdGenerator};

/// @id miyauth_tool_identity_resolve
/// @role mutator
/// @layer tool
/// @do identity_resolve_under_governance
//@human / @human Résout le contexte d'identité à partir des artefacts fournis.
/// tool.identity.resolve — ne décide pas de la confiance.
pub fn resolve(
    ctx: &GovernedContext,
    artefacts: &IdentityArtefacts,
    passport: Option<&VerifiedArtifact>,
    visa: Option<&VerifiedArtifact>,
) -> Result<IdentityContext, MiyauthError> {
    if !ctx.has_mandate() {
        return Err(MiyauthError::NoMandate);
    }
    let role = derive_role(passport, visa);
    let opaque_id = artefacts
        .session_ref
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(String::from)
        .unwrap_or_else(|| format!("opaque:{}", UuidIdGenerator.generate()));
    Ok(IdentityContext { role, opaque_id })
}

/// Dérive le rôle uniquement à partir d'artefacts VÉRIFIÉS (`Verifie`, construit par `verify`).
/// Citizen = Passeport vérifié ; Visitor = Visa vérifié ; sinon External (rôle le plus faible).
/// La simple présence de données brutes dans `IdentityArtefacts` n'accorde jamais de rôle.
fn derive_role(
    passport: Option<&VerifiedArtifact>,
    visa: Option<&VerifiedArtifact>,
) -> IdentityRole {
    if passport.is_some() {
        IdentityRole::Citizen
    } else if visa.is_some() {
        IdentityRole::Visitor
    } else {
        IdentityRole::External
    }
}
