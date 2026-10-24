//! Tool MiyuAntiSpam — tool.antispam.flood.check.
//! Vérifie le flood (scope fourni) ; décision bloquer = StrongFather.
//!
//! Les seuils de flood (posts / messages par fenêtre) sont fournis par le flux
//! ou KindMother sous forme d'un [`RateLimiter`] dédié au flood : même moteur
//! borné que `rate_limit` (GCRA, table de clés bornée, fail-closed).

use std::time::Instant;

use crate::context::GovernedContext;
use crate::errors::MiyuantispamError;
use crate::rate_limit::RateLimiter;

/// @id miyuantispam_tool_flood_check
/// @role security
/// @layer tool
/// @do flood_check_under_governance
//@human / @human Vérifie le flood (scope fourni) ; seuils = flux ou KindMother.
/// tool.antispam.flood.check — compte une action pour `scope_id`
/// (ex. `post:<persona>`) dans `limiter`. Toute erreur = refus (C-12).
pub fn check(
    ctx: &GovernedContext,
    limiter: &RateLimiter,
    scope_id: &str,
) -> Result<FloodCheckResult, MiyuantispamError> {
    if !ctx.has_mandate() {
        return Err(MiyuantispamError::NoMandate);
    }
    let r = limiter.check_at(scope_id, Instant::now())?;
    let max = limiter.policy().max_attempts;
    let current_count = if r.within_limit {
        u64::from(max.saturating_sub(r.remaining))
    } else {
        u64::from(max)
    };
    Ok(FloodCheckResult {
        within_limit: r.within_limit,
        current_count,
    })
}

/// Résultat de vérification flood (pas de décision — fourni à StrongFather).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FloodCheckResult {
    /// `true` si l'action est sous le seuil de flood.
    pub within_limit: bool,
    /// Charge courante estimée dans la fenêtre (plafonnée à `max_attempts`).
    pub current_count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rate_limit::RateLimitPolicy;
    use std::time::Duration;

    fn limiter(max: u32) -> RateLimiter {
        RateLimiter::new(RateLimitPolicy {
            max_attempts: max,
            window: Duration::from_secs(3600),
            max_tracked_keys: 8,
        })
        .unwrap()
    }

    #[test]
    fn refuse_sans_mandat() {
        let l = limiter(2);
        let empty = GovernedContext::new(String::new(), 1);
        assert_eq!(
            check(&empty, &l, "post:a"),
            Err(MiyuantispamError::NoMandate)
        );
    }

    #[test]
    fn flood_detecte_au_dela_du_seuil() {
        let l = limiter(2);
        let ctx = GovernedContext::new("m".into(), 1);
        let r1 = check(&ctx, &l, "post:a").unwrap();
        assert!(r1.within_limit);
        assert_eq!(r1.current_count, 1);
        let r2 = check(&ctx, &l, "post:a").unwrap();
        assert!(r2.within_limit);
        assert_eq!(r2.current_count, 2);
        let r3 = check(&ctx, &l, "post:a").unwrap();
        assert!(!r3.within_limit);
        assert_eq!(r3.current_count, 2);
    }

    #[test]
    fn cle_vide_refusee() {
        let l = limiter(2);
        let ctx = GovernedContext::new("m".into(), 1);
        assert!(matches!(
            check(&ctx, &l, ""),
            Err(MiyuantispamError::InvalidKey(_))
        ));
    }
}
