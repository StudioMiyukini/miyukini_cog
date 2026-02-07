//! Adapter JayFestival → MiyuClock (horloge, attestation date IRL).
//!
//! @id: miyuclock_now
//! @do: retourne l'instant présent selon l'horloge locale (MiyuClock tool.time.now)
//! @layer: domain
//!
//! @id: miyuclock_attest_date
//! @do: atteste la date/heure IRL pour un usage (trace, affichage) — MiyuClock comme référentiel
//! @layer: domain
//!
//! P1 : MiyuClock atteste l'horaire et la date IRL ; JayKoa organise les données et l'interface.

use miyuclock::context::GovernedContext;
use miyuclock::time;
use miyuclock::errors::MiyuclockError;
use std::time::SystemTime;
use std::fmt;

/// Contexte gouverné alpha pour les appels MiyuClock depuis JayFestival.
fn jayfestival_clock_ctx() -> GovernedContext {
    GovernedContext::new("jayfestival_alpha".to_string(), 1)
}

/// Erreur d'intégration MiyuClock.
#[derive(Debug)]
pub struct MiyuclockAdapterError {
    /// Message d'erreur (horloge, trace).
    pub message: String,
}

impl fmt::Display for MiyuclockAdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for MiyuclockAdapterError {}

impl From<MiyuclockError> for MiyuclockAdapterError {
    fn from(e: MiyuclockError) -> Self {
        MiyuclockAdapterError {
            message: format!("{e:?}"),
        }
    }
}

/// Retourne l'instant présent selon l'horloge locale (MiyuClock tool.time.now).
///
/// @id: miyuclock_now
/// @do: retourne l'instant présent selon l'horloge locale (MiyuClock tool.time.now)
/// @layer: domain
pub fn miyuclock_now() -> Result<SystemTime, MiyuclockAdapterError> {
    let ctx = jayfestival_clock_ctx();
    time::now(&ctx).map_err(MiyuclockAdapterError::from)
}

/// Atteste la date/heure IRL pour un usage (trace, affichage) — MiyuClock comme référentiel.
///
/// @id: miyuclock_attest_date
/// @do: atteste la date/heure IRL pour un usage (trace, affichage) — MiyuClock comme référentiel
/// @layer: domain
pub fn miyuclock_attest_date() -> Result<SystemTime, MiyuclockAdapterError> {
    miyuclock_now()
}
