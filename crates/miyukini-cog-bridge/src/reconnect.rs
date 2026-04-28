//! Reconnexion automatique avec backoff exponentiel.
//!
//! En cas de perte de connexion, le bridge tente de se reconnecter
//! avec un délai croissant (1s, 2s, 4s, 8s... max 60s).

use std::time::Duration;

/// Configuration de la stratégie de reconnexion.
#[derive(Debug, Clone)]
pub struct ReconnectConfig {
    /// Délai initial entre les tentatives.
    pub initial_delay: Duration,
    /// Délai maximum entre les tentatives.
    pub max_delay: Duration,
    /// Facteur multiplicateur (typiquement 2.0 pour backoff exponentiel).
    pub backoff_factor: f64,
    /// Nombre maximum de tentatives (0 = illimité).
    pub max_attempts: u32,
}

impl Default for ReconnectConfig {
    fn default() -> Self {
        Self {
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(60),
            backoff_factor: 2.0,
            max_attempts: 0, // illimité
        }
    }
}

/// État de la stratégie de reconnexion.
pub struct ReconnectState {
    config: ReconnectConfig,
    current_delay: Duration,
    attempts: u32,
}

impl ReconnectState {
    pub fn new(config: ReconnectConfig) -> Self {
        let initial = config.initial_delay;
        Self {
            config,
            current_delay: initial,
            attempts: 0,
        }
    }

    /// Retourne le délai avant la prochaine tentative, ou None si max atteint.
    pub fn next_delay(&mut self) -> Option<Duration> {
        if self.config.max_attempts > 0 && self.attempts >= self.config.max_attempts {
            return None;
        }

        let delay = self.current_delay;
        self.attempts += 1;

        // Backoff exponentiel
        let next = Duration::from_secs_f64(
            self.current_delay.as_secs_f64() * self.config.backoff_factor,
        );
        self.current_delay = next.min(self.config.max_delay);

        Some(delay)
    }

    /// Réinitialise après une connexion réussie.
    pub fn reset(&mut self) {
        self.current_delay = self.config.initial_delay;
        self.attempts = 0;
    }

    /// Nombre de tentatives effectuées.
    pub fn attempts(&self) -> u32 {
        self.attempts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backoff_exponential() {
        let mut state = ReconnectState::new(ReconnectConfig {
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(16),
            backoff_factor: 2.0,
            max_attempts: 0,
        });

        assert_eq!(state.next_delay(), Some(Duration::from_secs(1)));
        assert_eq!(state.next_delay(), Some(Duration::from_secs(2)));
        assert_eq!(state.next_delay(), Some(Duration::from_secs(4)));
        assert_eq!(state.next_delay(), Some(Duration::from_secs(8)));
        // Capped at max_delay
        assert_eq!(state.next_delay(), Some(Duration::from_secs(16)));
        assert_eq!(state.next_delay(), Some(Duration::from_secs(16)));
    }

    #[test]
    fn max_attempts() {
        let mut state = ReconnectState::new(ReconnectConfig {
            max_attempts: 3,
            ..Default::default()
        });

        assert!(state.next_delay().is_some());
        assert!(state.next_delay().is_some());
        assert!(state.next_delay().is_some());
        assert!(state.next_delay().is_none());
    }

    #[test]
    fn reset_after_success() {
        let mut state = ReconnectState::new(ReconnectConfig::default());
        state.next_delay();
        state.next_delay();
        assert_eq!(state.attempts(), 2);

        state.reset();
        assert_eq!(state.attempts(), 0);
    }
}
