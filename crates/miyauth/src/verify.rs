//! @id     miyauth_verify
//! @do     identity_verify_under_governance
//! @role   securite
//! @layer  tool
//! @human  miyauth — tool.identity.verify : vérification Passeport + Visa scellés (Sceau v1.1).

//! Tool MiyuAuth — tool.identity.verify.
//! Vérifie l'intégrité, la validité, les liens et la révocation d'un Passeport et de
//! son Visa ; ne décide pas de l'autorisation (ALLOW/DENY = StrongFather).
//!
//! Fin du stub fail-closed du lot 1 : la vérification s'appuie sur `miyusceau`
//! (voir [`crate::passeport::verifier_visa`]). L'ancien format v1 non signé
//! (`[0x01][expiry]`) n'est plus décodable : refus (`Sceau(MagicInvalide)` ou taille).

use crate::context::GovernedContext;
use crate::errors::MiyauthError;
use crate::passeport::{verifier_visa, AccesVerifie, PolitiqueIdentite, RefusIdentite};
use crate::types::{IdentityArtefacts, VerificationResult};

/// @id miyauth_tool_identity_verify
/// @role mutator
/// @layer tool
/// @do identity_verify_under_governance
//@human / @human Vérifie un Passeport Utilisateur et son Visa de Connexion pour un service.
/// tool.identity.verify — renvoie un [`AccesVerifie`] (seul moyen d'en obtenir un).
pub fn verify(
    ctx: &GovernedContext,
    politique: &PolitiqueIdentite<'_>,
    artefacts: &IdentityArtefacts,
    audience: &str,
    maintenant: u64,
) -> Result<AccesVerifie, MiyauthError> {
    if !ctx.has_mandate() {
        return Err(MiyauthError::NoMandate);
    }
    let (Some(passeport), Some(visa)) = (artefacts.passport_raw.as_deref(), artefacts.visa_raw.as_deref()) else {
        return Err(MiyauthError::Refus(RefusIdentite::ArtefactManquant));
    };
    verifier_visa(visa, passeport, audience, politique, maintenant).map_err(MiyauthError::Refus)
}

/// Résumé à trois états d'un refus (compatibilité avec [`VerificationResult`]).
pub fn resultat(refus: &RefusIdentite) -> VerificationResult {
    match refus {
        RefusIdentite::Sceau(miyusceau::ErreurSceau::Expire | miyusceau::ErreurSceau::PasEncoreValide) => {
            VerificationResult::Expired
        }
        _ => VerificationResult::Invalid,
    }
}
