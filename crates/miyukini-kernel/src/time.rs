//! Abstraction temps : `now()`, injection en test. Timezone = choix du produit.

use std::time::SystemTime;

/// Contrat : accès à l'instant courant.
pub trait Clock {
    fn now(&self) -> SystemTime;
}

/// Implémentation par défaut : `SystemTime::now()`. Minimal et remplaçable.
#[derive(Debug, Default)]
pub struct DefaultClock;

impl DefaultClock {
    pub fn new() -> Self {
        Self
    }
}

impl Clock for DefaultClock {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}
