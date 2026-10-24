//! Garde du parcours de premier démarrage (setup) : jeton à usage unique (audit H6).
//!
//! Le jeton est généré au démarrage (état VIERGE uniquement), remis **une seule fois** pour
//! affichage sur la sortie d'erreur de la console locale (jamais journalisé, jamais renvoyé par
//! l'API). Il est exigé par `/api/setup/create-admin`, accepté seulement depuis la boucle locale.
//! Il expire après [`DUREE_JETON_SETUP`] (15 min). Après la création du premier admin, trop
//! d'échecs ou l'expiration, la garde est fermée définitivement pour ce processus (redémarrer
//! MiyukiniAdmin pour obtenir un nouveau jeton, tant que le COG reste VIERGE).
//!
//! Inventaire des secrets partagés #22 : exception locale acceptée (second facteur local,
//! usage unique, durée courte).

use std::sync::Mutex;
use std::time::{Duration, Instant};
use uuid::Uuid;

/// Nombre maximal de jetons erronés avant fermeture définitive de la garde.
const MAX_BAD_TOKENS: u32 = 5;
/// Durée de validité du jeton de setup (15 min).
pub const DUREE_JETON_SETUP: Duration = Duration::from_mins(15);

/// État interne de la garde.
struct Inner {
    /// Jeton attendu ; `None` = garde fermée (aucun setup possible).
    token: Option<String>,
    /// Échecs de jeton cumulés.
    bad_attempts: u32,
    /// Expiration du jeton.
    expire_a: Instant,
    /// Jeton déjà remis pour affichage (une seule fois).
    affiche: bool,
}

impl Inner {
    /// Ferme la garde si le jeton a expiré ; vrai si encore ouverte.
    fn ouvert(&mut self) -> bool {
        if self.token.is_some() && Instant::now() >= self.expire_a {
            self.token = None;
        }
        self.token.is_some()
    }
}

/// @id miyukiniadmin_setup_guard
/// @role infrastructure
/// @layer operator
/// @do guard_first_boot_setup
//@human / @human Jeton de setup à usage unique, comparaison en temps constant, fermeture après succès.
pub struct SetupGuard {
    inner: Mutex<Inner>,
}

impl std::fmt::Debug for SetupGuard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Ne jamais exposer le jeton.
        f.debug_struct("SetupGuard").field("open", &self.is_open()).finish()
    }
}

impl SetupGuard {
    /// Garde ouverte avec un nouveau jeton aléatoire (2 × UUIDv4, OS RNG), valable 15 min.
    #[must_use]
    pub fn generate() -> Self {
        Self::generate_with_ttl(DUREE_JETON_SETUP)
    }

    /// Garde ouverte avec une durée de validité donnée (bornée à 15 min).
    #[must_use]
    pub fn generate_with_ttl(ttl: Duration) -> Self {
        let token = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
        Self {
            inner: Mutex::new(Inner {
                token: Some(token),
                bad_attempts: 0,
                expire_a: Instant::now() + ttl.min(DUREE_JETON_SETUP),
                affiche: false,
            }),
        }
    }

    /// Garde fermée d'emblée (environnement non vierge) : refuse tout.
    #[must_use]
    pub fn closed() -> Self {
        Self {
            inner: Mutex::new(Inner {
                token: None,
                bad_attempts: 0,
                expire_a: Instant::now(),
                affiche: true,
            }),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Inner> {
        self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Garde encore ouverte ?
    #[must_use]
    pub fn is_open(&self) -> bool {
        self.lock().ouvert()
    }

    /// Jeton à afficher sur la sortie d'erreur de la console locale, remis **une seule
    /// fois** (None ensuite, ou si la garde est fermée). À ne PAS journaliser.
    #[must_use]
    pub fn token_for_console(&self) -> Option<String> {
        let mut g = self.lock();
        if !g.ouvert() || g.affiche {
            return None;
        }
        g.affiche = true;
        g.token.clone()
    }

    /// Vérifie le jeton présenté (temps constant). Trop d'échecs => fermeture définitive.
    pub fn authorize(&self, provided: &str) -> bool {
        let mut g = self.lock();
        if !g.ouvert() {
            return false;
        }
        let Some(expected) = g.token.clone() else {
            return false;
        };
        if constant_time_eq(expected.as_bytes(), provided.as_bytes()) {
            return true;
        }
        g.bad_attempts += 1;
        if g.bad_attempts >= MAX_BAD_TOKENS {
            g.token = None;
        }
        false
    }

    /// Ferme définitivement la garde (jeton consommé).
    pub fn close(&self) {
        self.lock().token = None;
    }
}

/// Le parcours de setup n'est accepté que depuis la boucle locale : pair loopback **et**
/// en-tête `Host` local (`localhost`, `127.0.0.1`, `[::1]`, port facultatif), ce qui écarte
/// le rebinding DNS d'une page distante vers `127.0.0.1`.
#[must_use]
pub fn requete_setup_locale(pair: std::net::IpAddr, host: Option<&str>) -> bool {
    if !pair.to_canonical().is_loopback() {
        return false;
    }
    let Some(host) = host.map(|h| h.trim().to_ascii_lowercase()) else {
        return false;
    };
    let nom = if host.starts_with('[') {
        match host.split_once(']') {
            Some((n, reste)) if reste.is_empty() || reste.starts_with(':') => format!("{n}]"),
            _ => return false,
        }
    } else {
        host.split(':').next().unwrap_or_default().to_string()
    };
    matches!(nom.as_str(), "localhost" | "127.0.0.1" | "[::1]")
}

/// Comparaison en temps constant (la longueur ne court-circuite pas la boucle).
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    let n = a.len().max(b.len());
    let mut diff = u8::from(a.len() != b.len());
    for i in 0..n {
        let x = a.get(i).copied().unwrap_or(0);
        let y = b.get(i).copied().unwrap_or(0);
        diff |= x ^ y;
    }
    diff == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_token_authorizes_then_close_refuses() {
        let g = SetupGuard::generate();
        let t = g.token_for_console().unwrap();
        assert_eq!(t.len(), 64);
        assert!(g.authorize(&t));
        g.close();
        assert!(!g.is_open());
        assert!(!g.authorize(&t), "jeton à usage unique");
    }

    #[test]
    fn wrong_or_empty_token_refused() {
        let g = SetupGuard::generate();
        assert!(!g.authorize(""));
        assert!(!g.authorize("nimporte"));
        assert!(g.is_open());
    }

    #[test]
    fn too_many_bad_tokens_close_guard() {
        let g = SetupGuard::generate();
        let t = g.token_for_console().unwrap();
        for _ in 0..MAX_BAD_TOKENS {
            assert!(!g.authorize("x"));
        }
        assert!(!g.is_open());
        assert!(!g.authorize(&t));
    }

    #[test]
    fn closed_guard_refuses_everything() {
        let g = SetupGuard::closed();
        assert!(!g.authorize(""));
        assert!(g.token_for_console().is_none());
    }

    #[test]
    fn token_handed_out_once_only() {
        let g = SetupGuard::generate();
        assert!(g.token_for_console().is_some());
        assert!(g.token_for_console().is_none(), "remis une seule fois");
        assert!(g.is_open());
    }

    #[test]
    fn expired_token_refused_and_guard_closed() {
        let g = SetupGuard::generate_with_ttl(Duration::from_millis(20));
        let t = g.token_for_console().unwrap();
        std::thread::sleep(Duration::from_millis(40));
        assert!(!g.authorize(&t), "jeton expiré");
        assert!(!g.is_open());
        assert!(g.token_for_console().is_none());
    }

    #[test]
    fn ttl_capped_at_15_minutes() {
        let g = SetupGuard::generate_with_ttl(Duration::from_hours(24));
        assert!(g.lock().expire_a <= Instant::now() + DUREE_JETON_SETUP);
    }

    #[test]
    fn setup_only_from_loopback_with_local_host() {
        let lo: std::net::IpAddr = "127.0.0.1".parse().unwrap();
        let lo6: std::net::IpAddr = "::1".parse().unwrap();
        let ext: std::net::IpAddr = "192.168.1.20".parse().unwrap();
        assert!(requete_setup_locale(lo, Some("localhost:8090")));
        assert!(requete_setup_locale(lo, Some("127.0.0.1")));
        assert!(requete_setup_locale(lo6, Some("[::1]:8090")));
        assert!(!requete_setup_locale(ext, Some("localhost")));
        assert!(!requete_setup_locale(lo, None));
        // Rebinding DNS : pair loopback, mais Host étranger.
        assert!(!requete_setup_locale(lo, Some("evil.example:8090")));
        assert!(!requete_setup_locale(lo, Some("localhost.evil.example")));
        assert!(!requete_setup_locale(lo, Some("[::1]x")));
    }

    #[test]
    fn debug_does_not_leak_token() {
        let g = SetupGuard::generate();
        let t = g.token_for_console().unwrap();
        assert!(!format!("{g:?}").contains(&t));
    }
}
