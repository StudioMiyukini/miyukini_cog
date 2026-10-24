//! Garde du parcours de premier démarrage (setup) : jeton à usage unique (audit H6).
//!
//! Le jeton est généré au démarrage (état VIERGE uniquement), affiché UNIQUEMENT dans la console
//! locale, jamais renvoyé par l'API. Il est exigé par `/api/setup/create-admin`. Après la création
//! du premier admin (ou trop d'échecs), la garde est fermée définitivement pour ce processus.

use std::sync::Mutex;
use uuid::Uuid;

/// Nombre maximal de jetons erronés avant fermeture définitive de la garde.
const MAX_BAD_TOKENS: u32 = 5;

/// État interne de la garde.
struct Inner {
    /// Jeton attendu ; `None` = garde fermée (aucun setup possible).
    token: Option<String>,
    /// Échecs de jeton cumulés.
    bad_attempts: u32,
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
    /// Garde ouverte avec un nouveau jeton aléatoire (2 × UUIDv4, OS RNG).
    #[must_use]
    pub fn generate() -> Self {
        let token = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
        Self {
            inner: Mutex::new(Inner {
                token: Some(token),
                bad_attempts: 0,
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
            }),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Inner> {
        self.inner.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Garde encore ouverte ?
    #[must_use]
    pub fn is_open(&self) -> bool {
        self.lock().token.is_some()
    }

    /// Jeton à afficher dans la console locale (None si fermée). À ne PAS journaliser ailleurs.
    #[must_use]
    pub fn token_for_console(&self) -> Option<String> {
        self.lock().token.clone()
    }

    /// Vérifie le jeton présenté (temps constant). Trop d'échecs => fermeture définitive.
    pub fn authorize(&self, provided: &str) -> bool {
        let mut g = self.lock();
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
    fn debug_does_not_leak_token() {
        let g = SetupGuard::generate();
        let t = g.token_for_console().unwrap();
        assert!(!format!("{g:?}").contains(&t));
    }
}
