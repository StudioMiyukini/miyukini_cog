//! @id     miyauth_resolve
//! @do     identity_resolve_under_governance
//! @role   securite
//! @layer  tool
//! @human  miyauth — tool.identity.resolve : contexte d'identité à partir d'un accès vérifié.

//! Tool MiyuAuth — tool.identity.resolve.
//! Résout le contexte d'identité ; ne décide pas de la confiance.
//! Toute confiance utilisée pour l'identité est validée par KindMother (BOUND-3).

use crate::context::GovernedContext;
use crate::errors::MiyauthError;
use crate::passeport::AccesProuve;
use crate::types::{IdentityArtefacts, IdentityContext, IdentityRole};
use miyukini_kernel::{IdGenerator, UuidIdGenerator};

/// @id miyauth_tool_identity_resolve
/// @role mutator
/// @layer tool
/// @do identity_resolve_under_governance
//@human / @human Résout le contexte d'identité à partir d'un accès vérifié (Passeport + Visa).
/// tool.identity.resolve — ne décide pas de la confiance.
///
/// - Accès prouvé (visa vérifié + possession de session) : rôle du visa (`Citizen` / `Visitor`), identifiant opaque =
///   `pw:<cog d'origine>:` + hex de l'identifiant par paire (stable pour ce COG, EX-36).
/// - Sinon : `External`, identifiant opaque de session (jamais un rôle accordé par
///   des données brutes non vérifiées).
pub fn resolve(
    ctx: &GovernedContext,
    artefacts: &IdentityArtefacts,
    acces: Option<&AccesProuve>,
) -> Result<IdentityContext, MiyauthError> {
    if !ctx.has_mandate() {
        return Err(MiyauthError::NoMandate);
    }
    if let Some(acces) = acces {
        // Espace de noms par COG émetteur : deux COG ne peuvent pas produire le
        // même identifiant (un COG étranger ne peut pas viser un citoyen local).
        return Ok(IdentityContext {
            role: acces.role(),
            opaque_id: format!("pw:{}:{}", acces.cog_origine(), hex(acces.sujet())),
        });
    }
    // Référence de session fournie par le client : jamais reprise telle quelle
    // (elle pourrait imiter un identifiant `pw:`), seulement son empreinte.
    let opaque_id = artefacts.session_ref.as_deref().filter(|s| !s.is_empty()).map_or_else(
        || format!("opaque:{}", UuidIdGenerator.generate()),
        |s| {
            use sha2::{Digest, Sha256};
            format!("sess:{}", hex(&Sha256::digest(s.as_bytes())))
        },
    );
    Ok(IdentityContext { role: IdentityRole::External, opaque_id })
}

fn hex(octets: &[u8]) -> String {
    use std::fmt::Write;
    octets.iter().fold(String::with_capacity(octets.len() * 2), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    })
}
