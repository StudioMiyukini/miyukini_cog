//! Tool MiyuAuth — tool.identity.verify.
//! Vérifie l'intégrité et la validité d'un Passeport ou Visa ; ne décide pas de l'autorisation.
//! Format minimal v1 : [version: 1 byte][expiry_nanos: 8 bytes LE]. expiry_nanos = 0 signifie pas d'expiration.

use crate::context::GovernedContext;
use crate::errors::MiyauthError;
use crate::types::VerificationResult;
use miyukini_kernel::time::{Clock, DefaultClock};

const FORMAT_VERSION: u8 = 0x01;
const MIN_LEN: usize = 1 + 8; // version + expiry_nanos

/// @id: miyauth_tool_identity_verify
/// @role: mutator
/// @layer: tool
/// @human: Vérifie l'intégrité et la validité d'un Passeport Utilisateur ou Visa de Connexion.
/// @do: identity_verify_under_governance
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

fn verify_artifact(raw: &[u8]) -> VerificationResult {
    if raw.len() < MIN_LEN {
        return VerificationResult::Invalid;
    }
    if raw[0] != FORMAT_VERSION {
        return VerificationResult::Invalid;
    }
    let expiry_nanos: u64 = match raw[1..9].try_into() {
        Ok(b) => u64::from_le_bytes(b),
        Err(_) => return VerificationResult::Invalid,
    };
    if expiry_nanos == 0 {
        return VerificationResult::Valid;
    }
    let clock = DefaultClock;
    let now_nanos = clock
        .now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    if now_nanos >= expiry_nanos {
        VerificationResult::Expired
    } else {
        VerificationResult::Valid
    }
}
