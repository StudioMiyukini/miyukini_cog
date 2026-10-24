//! Securite de l'API : verification des clients (Argon2id) et limitation de debit en memoire.
//!
//! Fail-closed (audit C9) : aucun client configure => aucun token ; scopes issus de la config.

// @id service.alicia.api.security
// @role client_auth_and_rate_limit
// @layer 7
// @do verify_client_secret_and_limit_rate
// @human Verification du secret client (Argon2id, temps constant) et limiteur de debit par IP.

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use argon2::password_hash::{PasswordHash, PasswordVerifier};
use argon2::Argon2;

use crate::auth::JwtScope;
use crate::config::ApiConfig;

/// Verifie `client_id` + `secret` contre le registre configure.
///
/// Retourne les scopes du client (issus de la config, jamais du nom du client) ou `None`.
/// Un client inconnu declenche quand meme une verification factice pour limiter
/// l'enumeration par timing. La comparaison du hash est en temps constant (crate argon2).
pub fn authenticate_client(config: &ApiConfig, client_id: &str, secret: &str) -> Option<Vec<JwtScope>> {
    // Hash factice Argon2id valide (secret inconnu) pour egaliser le temps de reponse.
    const DUMMY: &str = "$argon2id$v=19$m=19456,t=2,p=1$ZHVtbXlzYWx0ZHVtbXk$Kq3b3m3mQ4h7m1o0cYk3x5v0o4d8Qm0pJm3wq0ZkZ9o";
    if client_id.is_empty() || secret.is_empty() || config.clients.is_empty() {
        return None;
    }
    let client = config.clients.iter().find(|c| c.client_id == client_id);
    let hash_str = client.map_or(DUMMY, |c| c.secret_hash.as_str());
    let Ok(parsed) = PasswordHash::new(hash_str) else {
        return None;
    };
    let ok = Argon2::default()
        .verify_password(secret.as_bytes(), &parsed)
        .is_ok();
    match client {
        Some(c) if ok => Some(c.scopes.clone()),
        _ => None,
    }
}

/// Limiteur de debit a fenetre fixe (60 s) par IP, en memoire.
#[derive(Clone)]
pub struct RateLimiter {
    max_per_min: u32,
    window: Duration,
    hits: Arc<Mutex<HashMap<Option<IpAddr>, (Instant, u32)>>>,
}

impl RateLimiter {
    /// Cree un limiteur ; `max_per_min == 0` desactive la limitation.
    pub fn new(max_per_min: u32) -> Self {
        Self {
            max_per_min,
            window: Duration::from_secs(60),
            hits: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Enregistre une requete. `Err(secondes)` si la limite est depassee.
    /// `ip == None` (pas de ConnectInfo) partage un seul compteur.
    pub fn check(&self, ip: Option<IpAddr>) -> Result<(), u64> {
        if self.max_per_min == 0 {
            return Ok(());
        }
        let now = Instant::now();
        let mut map = self.hits.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if map.len() > 10_000 {
            let w = self.window;
            map.retain(|_, (start, _)| now.duration_since(*start) < w);
        }
        let entry = map.entry(ip).or_insert((now, 0));
        if now.duration_since(entry.0) >= self.window {
            *entry = (now, 0);
        }
        if entry.1 >= self.max_per_min {
            let left = self.window.saturating_sub(now.duration_since(entry.0)).as_secs();
            return Err(left.max(1));
        }
        entry.1 += 1;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ApiClientConfig;
    use argon2::password_hash::{PasswordHasher, SaltString};

    pub(crate) fn hash(secret: &str) -> String {
        let salt = SaltString::from_b64("c2VsdGVzdHNlbHRlc3Q").unwrap();
        Argon2::default()
            .hash_password(secret.as_bytes(), &salt)
            .unwrap()
            .to_string()
    }

    fn cfg() -> ApiConfig {
        ApiConfig {
            clients: vec![
                ApiClientConfig {
                    client_id: "ops".into(),
                    secret_hash: hash("bon-secret"),
                    scopes: vec![JwtScope::Admin],
                },
                ApiClientConfig {
                    client_id: "admin".into(),
                    secret_hash: hash("autre"),
                    scopes: vec![JwtScope::Read],
                },
            ],
            ..Default::default()
        }
    }

    #[test]
    fn good_secret_returns_configured_scopes() {
        assert_eq!(authenticate_client(&cfg(), "ops", "bon-secret"), Some(vec![JwtScope::Admin]));
    }

    #[test]
    fn scope_comes_from_config_not_client_name() {
        // Le client nomme "admin" n'a que Read.
        assert_eq!(authenticate_client(&cfg(), "admin", "autre"), Some(vec![JwtScope::Read]));
    }

    #[test]
    fn wrong_secret_or_unknown_client_refused() {
        assert!(authenticate_client(&cfg(), "ops", "mauvais").is_none());
        assert!(authenticate_client(&cfg(), "inconnu", "bon-secret").is_none());
    }

    #[test]
    fn no_clients_configured_refuses_all() {
        assert!(authenticate_client(&ApiConfig::default(), "admin", "x").is_none());
    }

    #[test]
    fn rate_limiter_blocks_after_max() {
        let rl = RateLimiter::new(2);
        let ip = Some("10.0.0.1".parse().unwrap());
        assert!(rl.check(ip).is_ok());
        assert!(rl.check(ip).is_ok());
        assert!(rl.check(ip).is_err());
        assert!(rl.check(Some("10.0.0.2".parse().unwrap())).is_ok());
    }

    #[test]
    fn rate_limiter_zero_disables() {
        let rl = RateLimiter::new(0);
        for _ in 0..1000 {
            assert!(rl.check(None).is_ok());
        }
    }
}
