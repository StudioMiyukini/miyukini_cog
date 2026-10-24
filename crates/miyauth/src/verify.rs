//! Tool MiyuAuth — tool.identity.verify.
//! Vérifie l'intégrité et la validité d'un Passeport ou Visa ; ne décide pas de l'autorisation.
//!
//! FAIL-CLOSED (audit C8) : le format v1 `[version: 1 byte][expiry_nanos: 8 bytes LE]` ne porte
//! AUCUNE signature : n'importe qui peut le forger. Tant que le Passeport signé n'existe pas
//! (lot 2 : Ed25519), tout artefact v1 est `Invalid`. `expiry = 0` (sans expiration) n'est
//! jamais valide. Seule `verify_verified` peut construire un `Verifie<T>`.

use crate::context::GovernedContext;
use crate::errors::MiyauthError;
use crate::types::VerificationResult;

const FORMAT_VERSION: u8 = 0x01;
const MIN_LEN: usize = 1 + 8; // version + expiry_nanos

/// Artefact dont la vérification a réussi. Champ privé : seul ce module peut le construire
/// (via `verify_verified`), donc `resolve` ne peut recevoir que des artefacts vérifiés.
#[derive(Debug, Clone)]
pub struct Verifie<T> {
    inner: T,
}

/// Alias explicite : artefact brut (Passeport/Visa) vérifié.
pub type VerifiedArtifact = Verifie<Vec<u8>>;

impl<T> Verifie<T> {
    /// Accès en lecture à la valeur vérifiée.
    pub fn get(&self) -> &T {
        &self.inner
    }
}

/// @id miyauth_tool_identity_verify
/// @role mutator
/// @layer tool
/// @do identity_verify_under_governance
//@human / @human Vérifie l'intégrité et la validité d'un Passeport Utilisateur ou Visa de Connexion.
/// tool.identity.verify — ne décide pas de l'autorisation (ALLOW/DENY = StrongFather).
pub fn verify(
    ctx: &GovernedContext,
    passport_or_visa_raw: &[u8],
) -> Result<VerificationResult, MiyauthError> {
    if !ctx.has_mandate() {
        return Err(MiyauthError::NoMandate);
    }
    Ok(verify_artifact(passport_or_visa_raw))
}

/// Comme `verify`, mais renvoie un `Verifie` seulement si le résultat est `Valid`.
/// Sinon `MiyauthError::VerificationFailed`.
pub fn verify_verified(
    ctx: &GovernedContext,
    raw: &[u8],
) -> Result<VerifiedArtifact, MiyauthError> {
    match verify(ctx, raw)? {
        VerificationResult::Valid => Ok(Verifie {
            inner: raw.to_vec(),
        }),
        other => Err(MiyauthError::VerificationFailed(other)),
    }
}

fn verify_artifact(raw: &[u8]) -> VerificationResult {
    if raw.len() < MIN_LEN || raw[0] != FORMAT_VERSION {
        return VerificationResult::Invalid;
    }
    // Lot 2 : Ed25519 — vérifier ici la signature d'un Passeport signé (format v2) puis
    // l'expiration (expiry > 0 obligatoire). Le format v1 est non signé donc forgeable :
    // refus systématique (fail-closed), quelle que soit la valeur d'expiry.
    VerificationResult::Invalid
}
