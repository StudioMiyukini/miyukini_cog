//! Machine d'état de KindMother
//!
//! Définit les états possibles du moteur et les transitions.

use std::fmt;

/// État du moteur KindMother
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KMState {
    /// Moteur en cours de démarrage
    Booting,
    /// Moteur opérationnel et sain
    Healthy,
    /// Moteur dégradé mais fonctionnel
    Degraded,
    /// Moteur en quarantaine (opérations bloquées)
    Quarantined,
}

impl fmt::Display for KMState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KMState::Booting => write!(f, "Booting"),
            KMState::Healthy => write!(f, "Healthy"),
            KMState::Degraded => write!(f, "Degraded"),
            KMState::Quarantined => write!(f, "Quarantined"),
        }
    }
}

impl KMState {
    /// Vérifie si l'état permet l'exécution d'opérations
    pub fn allows_operations(&self) -> bool {
        matches!(self, KMState::Healthy | KMState::Degraded)
    }

    /// Vérifie si l'état est en quarantaine
    pub fn is_quarantined(&self) -> bool {
        matches!(self, KMState::Quarantined)
    }
}
