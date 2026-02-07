//! Tools MiyuAntiSpam — tool.antispam.captcha.generate, tool.antispam.captcha.verify.
//! Exécution seule ; pas de décision ; résultat fourni à StrongFather.

use crate::context::GovernedContext;
use crate::errors::MiyuantispamError;
use miyukini_kernel::{IdGenerator, UuidIdGenerator};

/// @id: miyuantispam_tool_captcha_generate
/// @role: mutator
/// @layer: tool
/// @human: Génère un défi CAPTCHA.
/// @do: captcha_generate_under_governance
/// tool.antispam.captcha.generate
pub fn generate(ctx: &GovernedContext) -> Result<String, MiyuantispamError> {
    if !ctx.has_mandate() {
        return Err(MiyuantispamError::NoMandate);
    }
    let gen = UuidIdGenerator;
    let id = gen.generate();
    Ok(format!("cap:{id}"))
}

/// @id: miyuantispam_tool_captcha_verify
/// @role: mutator
/// @layer: tool
/// @human: Vérifie une réponse CAPTCHA.
/// @do: captcha_verify_under_governance
/// tool.antispam.captcha.verify
pub fn verify(
    ctx: &GovernedContext,
    _challenge_id: &str,
    _response: &str,
) -> Result<bool, MiyuantispamError> {
    if !ctx.has_mandate() {
        return Err(MiyuantispamError::NoMandate);
    }
    Ok(false)
}
