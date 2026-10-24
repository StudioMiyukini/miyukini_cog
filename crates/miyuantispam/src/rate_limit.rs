//! Tool MiyuAntiSpam — tool.antispam.rate_limit.check.
//! Vérifie la limite de tentatives (scope fourni) ; décision bloquer = StrongFather.
//!
//! Implémentation : algorithme GCRA (équivalent d'un seau à jetons) par clé,
//! en mémoire, O(1) par clé. Les seuils (`max_attempts`, `window`) sont fournis
//! par le flux ou KindMother via [`RateLimitPolicy`]. La table des clés est
//! bornée (`max_tracked_keys`, C-13) : les entrées expirées sont purgées ; si la
//! table reste pleine, une **nouvelle** clé est refusée (fail-closed, C-12) — on
//! n'évince jamais un compteur actif (sinon un attaquant pourrait réinitialiser
//! les compteurs d'autrui en inondant la table de clés).

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::context::GovernedContext;
use crate::errors::MiyuantispamError;

/// Longueur maximale d'une clé de limitation (octets).
pub const MAX_KEY_BYTES: usize = 256;

/// Politique de limitation (fournie par le flux ou KindMother).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RateLimitPolicy {
    /// Nombre maximal de tentatives autorisées par fenêtre (rafale incluse).
    pub max_attempts: u32,
    /// Fenêtre de temps sur laquelle `max_attempts` s'applique.
    pub window: Duration,
    /// Nombre maximal de clés suivies simultanément (borne mémoire).
    pub max_tracked_keys: usize,
}

impl RateLimitPolicy {
    /// Valide la politique et calcule l'intervalle d'émission (`window / max_attempts`).
    fn emission_interval(&self) -> Result<Duration, MiyuantispamError> {
        if self.max_attempts == 0 {
            return Err(MiyuantispamError::InvalidPolicy("max_attempts must be > 0"));
        }
        if self.window.is_zero() {
            return Err(MiyuantispamError::InvalidPolicy("window must be > 0"));
        }
        if self.max_tracked_keys == 0 {
            return Err(MiyuantispamError::InvalidPolicy(
                "max_tracked_keys must be > 0",
            ));
        }
        let interval = self.window / self.max_attempts;
        if interval.is_zero() {
            return Err(MiyuantispamError::InvalidPolicy(
                "window too short for max_attempts",
            ));
        }
        Ok(interval)
    }
}

/// Résultat de vérification rate limit (fourni à StrongFather).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RateLimitCheckResult {
    /// `true` si la tentative est dans la limite (et a été comptée).
    pub within_limit: bool,
    /// Tentatives restantes dans la fenêtre courante après celle-ci.
    pub remaining: u32,
    /// Délai avant la prochaine tentative autorisée (0 si `within_limit`).
    pub retry_after: Duration,
}

/// Limiteur borné en mémoire, partagé (`&self`, `Send + Sync`).
#[derive(Debug)]
pub struct RateLimiter {
    policy: RateLimitPolicy,
    interval: Duration,
    /// Clé → instant théorique d'arrivée (TAT, GCRA).
    state: Mutex<HashMap<String, Instant>>,
}

impl RateLimiter {
    /// Construit un limiteur ; refuse une politique invalide.
    pub fn new(policy: RateLimitPolicy) -> Result<Self, MiyuantispamError> {
        let interval = policy.emission_interval()?;
        Ok(Self {
            policy,
            interval,
            state: Mutex::new(HashMap::new()),
        })
    }

    /// Politique appliquée.
    pub fn policy(&self) -> RateLimitPolicy {
        self.policy
    }

    /// Nombre de clés actuellement suivies.
    pub fn tracked_keys(&self) -> Result<usize, MiyuantispamError> {
        self.state
            .lock()
            .map(|g| g.len())
            .map_err(|_| MiyuantispamError::StateUnavailable)
    }

    /// Compte une tentative pour `key` à l'instant `now` (horloge injectable).
    ///
    /// Une tentative refusée n'est pas comptée.
    pub fn check_at(
        &self,
        key: &str,
        now: Instant,
    ) -> Result<RateLimitCheckResult, MiyuantispamError> {
        if key.is_empty() {
            return Err(MiyuantispamError::InvalidKey("empty"));
        }
        if key.len() > MAX_KEY_BYTES {
            return Err(MiyuantispamError::InvalidKey("too long"));
        }
        let mut map = self
            .state
            .lock()
            .map_err(|_| MiyuantispamError::StateUnavailable)?;

        let tat = if let Some(t) = map.get(key) {
            *t
        } else {
            if map.len() >= self.policy.max_tracked_keys {
                // Purge des entrées expirées (TAT passé = état identique à « absent »).
                map.retain(|_, t| *t > now);
            }
            if map.len() >= self.policy.max_tracked_keys {
                return Err(MiyuantispamError::CapacityExceeded {
                    max: self.policy.max_tracked_keys,
                });
            }
            now
        };

        // Charge en cours au-delà de `now`, plus la tentative courante.
        let backlog = tat.saturating_duration_since(now);
        let Some(after) = backlog.checked_add(self.interval) else {
            return Err(MiyuantispamError::StateUnavailable);
        };
        if after > self.policy.window {
            return Ok(RateLimitCheckResult {
                within_limit: false,
                remaining: 0,
                retry_after: after.saturating_sub(self.policy.window),
            });
        }
        let Some(new_tat) = now.checked_add(after) else {
            return Err(MiyuantispamError::StateUnavailable);
        };
        map.insert(key.to_string(), new_tat);
        let slack = self.policy.window.saturating_sub(after);
        let remaining = slack.as_nanos() / self.interval.as_nanos();
        Ok(RateLimitCheckResult {
            within_limit: true,
            remaining: u32::try_from(remaining).unwrap_or(u32::MAX),
            retry_after: Duration::ZERO,
        })
    }
}

/// @id miyuantispam_tool_rate_limit_check
/// @role security
/// @layer tool
/// @do rate_limit_check_under_governance
//@human / @human Vérifie la limite de tentatives (scope fourni).
/// tool.antispam.rate_limit.check — compte une tentative pour `scope_id`
/// (ex. `search:<ip>`) dans `limiter`. Toute erreur = refus (C-12).
pub fn check(
    ctx: &GovernedContext,
    limiter: &RateLimiter,
    scope_id: &str,
) -> Result<RateLimitCheckResult, MiyuantispamError> {
    if !ctx.has_mandate() {
        return Err(MiyuantispamError::NoMandate);
    }
    limiter.check_at(scope_id, Instant::now())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GovernedContext {
        GovernedContext::new("mandat-test".into(), 1)
    }

    fn policy(max: u32, window_s: u64, keys: usize) -> RateLimitPolicy {
        RateLimitPolicy {
            max_attempts: max,
            window: Duration::from_secs(window_s),
            max_tracked_keys: keys,
        }
    }

    #[test]
    fn refuse_sans_mandat() {
        let l = RateLimiter::new(policy(3, 60, 10)).unwrap();
        let empty = GovernedContext::new(String::new(), 1);
        assert_eq!(check(&empty, &l, "k"), Err(MiyuantispamError::NoMandate));
    }

    #[test]
    fn politique_invalide_refusee() {
        assert!(matches!(
            RateLimiter::new(policy(0, 60, 10)),
            Err(MiyuantispamError::InvalidPolicy(_))
        ));
        assert!(matches!(
            RateLimiter::new(policy(3, 0, 10)),
            Err(MiyuantispamError::InvalidPolicy(_))
        ));
        assert!(matches!(
            RateLimiter::new(policy(3, 60, 0)),
            Err(MiyuantispamError::InvalidPolicy(_))
        ));
        let tiny = RateLimitPolicy {
            max_attempts: 10,
            window: Duration::from_nanos(5),
            max_tracked_keys: 1,
        };
        assert!(matches!(
            RateLimiter::new(tiny),
            Err(MiyuantispamError::InvalidPolicy(_))
        ));
    }

    #[test]
    fn limite_depassee_refusee() {
        let l = RateLimiter::new(policy(3, 60, 10)).unwrap();
        let t0 = Instant::now();
        let r1 = l.check_at("login:1.2.3.4", t0).unwrap();
        assert!(r1.within_limit);
        assert_eq!(r1.remaining, 2);
        assert_eq!(l.check_at("login:1.2.3.4", t0).unwrap().remaining, 1);
        assert_eq!(l.check_at("login:1.2.3.4", t0).unwrap().remaining, 0);
        let r4 = l.check_at("login:1.2.3.4", t0).unwrap();
        assert!(!r4.within_limit);
        assert_eq!(r4.remaining, 0);
        assert_eq!(r4.retry_after, Duration::from_secs(20));
        // Un refus n'est pas compté : toujours refusé, même délai.
        let r5 = l.check_at("login:1.2.3.4", t0).unwrap();
        assert!(!r5.within_limit);
        assert_eq!(r5.retry_after, Duration::from_secs(20));
    }

    #[test]
    fn cles_independantes() {
        let l = RateLimiter::new(policy(1, 60, 10)).unwrap();
        let t0 = Instant::now();
        assert!(l.check_at("a", t0).unwrap().within_limit);
        assert!(!l.check_at("a", t0).unwrap().within_limit);
        assert!(l.check_at("b", t0).unwrap().within_limit);
    }

    #[test]
    fn recharge_progressive() {
        let l = RateLimiter::new(policy(3, 60, 10)).unwrap();
        let t0 = Instant::now();
        for _ in 0..3 {
            assert!(l.check_at("k", t0).unwrap().within_limit);
        }
        assert!(
            !l.check_at("k", t0 + Duration::from_secs(19))
                .unwrap()
                .within_limit
        );
        assert!(
            l.check_at("k", t0 + Duration::from_secs(20))
                .unwrap()
                .within_limit
        );
        // Après une fenêtre complète : capacité pleine.
        let r = l.check_at("k", t0 + Duration::from_secs(200)).unwrap();
        assert!(r.within_limit);
        assert_eq!(r.remaining, 2);
    }

    #[test]
    fn table_bornee_refuse_nouvelle_cle() {
        let l = RateLimiter::new(policy(2, 60, 3)).unwrap();
        let t0 = Instant::now();
        for k in ["a", "b", "c"] {
            assert!(l.check_at(k, t0).unwrap().within_limit);
        }
        assert_eq!(l.tracked_keys().unwrap(), 3);
        assert_eq!(
            l.check_at("d", t0),
            Err(MiyuantispamError::CapacityExceeded { max: 3 })
        );
        assert_eq!(l.tracked_keys().unwrap(), 3);
        // Les clés existantes restent suivies normalement.
        assert!(l.check_at("a", t0).unwrap().within_limit);
        assert!(!l.check_at("a", t0).unwrap().within_limit);
    }

    #[test]
    fn table_purge_les_entrees_expirees() {
        let l = RateLimiter::new(policy(2, 60, 2)).unwrap();
        let t0 = Instant::now();
        assert!(l.check_at("a", t0).unwrap().within_limit);
        assert!(l.check_at("b", t0).unwrap().within_limit);
        let later = t0 + Duration::from_secs(61);
        assert!(l.check_at("c", later).unwrap().within_limit);
        assert!(l.tracked_keys().unwrap() <= 2);
    }

    #[test]
    fn cle_invalide_refusee() {
        let l = RateLimiter::new(policy(2, 60, 2)).unwrap();
        let t0 = Instant::now();
        assert_eq!(
            l.check_at("", t0),
            Err(MiyuantispamError::InvalidKey("empty"))
        );
        let long = "x".repeat(MAX_KEY_BYTES + 1);
        assert_eq!(
            l.check_at(&long, t0),
            Err(MiyuantispamError::InvalidKey("too long"))
        );
        assert_eq!(l.tracked_keys().unwrap(), 0);
    }

    #[test]
    fn outil_compte_via_horloge_reelle() {
        let l = RateLimiter::new(policy(2, 3600, 4)).unwrap();
        assert!(check(&ctx(), &l, "s").unwrap().within_limit);
        assert!(check(&ctx(), &l, "s").unwrap().within_limit);
        assert!(!check(&ctx(), &l, "s").unwrap().within_limit);
    }
}
