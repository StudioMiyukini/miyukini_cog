//! Tools MiyuAntiSpam — tool.antispam.captcha.generate, tool.antispam.captcha.verify.
//! Exécution seule ; pas de décision ; résultat fourni à StrongFather.
//!
//! Non implémentés : aucun défi n'est réellement produit ni stocké. Les deux
//! outils **refusent explicitement** (C-14) au lieu de renvoyer un identifiant
//! de défi invérifiable ou un faux résultat.

use crate::context::GovernedContext;
use crate::errors::MiyuantispamError;

/// @id miyuantispam_tool_captcha_generate
/// @role mutator
/// @layer tool
/// @do captcha_generate_under_governance
//@human / @human Génère un défi CAPTCHA.
/// tool.antispam.captcha.generate — refus explicite tant que non implémenté.
pub fn generate(ctx: &GovernedContext) -> Result<String, MiyuantispamError> {
    if !ctx.has_mandate() {
        return Err(MiyuantispamError::NoMandate);
    }
    Err(MiyuantispamError::Unimplemented(
        "tool.antispam.captcha.generate",
    ))
}

/// @id miyuantispam_tool_captcha_verify
/// @role mutator
/// @layer tool
/// @do captcha_verify_under_governance
//@human / @human Vérifie une réponse CAPTCHA.
/// tool.antispam.captcha.verify — refus explicite tant que non implémenté.
pub fn verify(
    ctx: &GovernedContext,
    _challenge_id: &str,
    _response: &str,
) -> Result<bool, MiyuantispamError> {
    if !ctx.has_mandate() {
        return Err(MiyuantispamError::NoMandate);
    }
    Err(MiyuantispamError::Unimplemented(
        "tool.antispam.captcha.verify",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_refuse_explicitement() {
        let ctx = GovernedContext::new("m".into(), 1);
        assert!(matches!(
            generate(&ctx),
            Err(MiyuantispamError::Unimplemented(_))
        ));
        let empty = GovernedContext::new(String::new(), 1);
        assert_eq!(generate(&empty), Err(MiyuantispamError::NoMandate));
    }

    #[test]
    fn verify_refuse_explicitement() {
        let ctx = GovernedContext::new("m".into(), 1);
        assert!(matches!(
            verify(&ctx, "cap:x", "réponse"),
            Err(MiyuantispamError::Unimplemented(_))
        ));
        let empty = GovernedContext::new(String::new(), 1);
        assert_eq!(
            verify(&empty, "cap:x", "r"),
            Err(MiyuantispamError::NoMandate)
        );
    }
}
