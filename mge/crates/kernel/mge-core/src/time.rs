//! Gestion du temps de jeu.
//!
//! <!-- @id: MGE-Core-Time @do: time-management @role: back-end @layer: 1 @human: miyuk -->
//!
//! [`TickRate`] definit la frequence de simulation (25 Hz pour Sodomight).
//! [`GameTime`] suit le temps ecoule, le delta entre ticks, et le compteur de ticks.

use std::time::{Duration, Instant};

/// Tick rate configurable.
///
/// Sodomight utilise 25 Hz (40 ms par tick). Ce choix est inspire de Diablo 2
/// qui tourne a 25 FPS logiques. La constante [`TickRate::SODOMIGHT`] est fournie
/// pour ce cas d'usage.
#[derive(Debug, Clone, Copy)]
pub struct TickRate {
    /// Nombre de ticks logiques par seconde.
    pub ticks_per_second: u32,
}

impl TickRate {
    /// 25 Hz -- tick rate natif de Sodomight (40 ms par tick).
    pub const SODOMIGHT: Self = Self {
        ticks_per_second: 25,
    };

    /// 60 Hz -- tick rate standard pour les jeux temps reel rapides.
    pub const SIXTY_HZ: Self = Self {
        ticks_per_second: 60,
    };

    /// Duree d'un tick logique.
    ///
    /// Pour 25 Hz : `1_000_000_000 ns / 25 = 40_000_000 ns = 40 ms`.
    #[must_use]
    pub fn tick_duration(self) -> Duration {
        Duration::from_secs(1) / self.ticks_per_second
    }
}

impl Default for TickRate {
    fn default() -> Self {
        Self::SODOMIGHT
    }
}

/// Temps de jeu -- delta time, tick count, elapsed.
///
/// Chaque appel a [`GameTime::advance`] met a jour le delta, le temps ecoule,
/// et incremente le compteur de ticks. Le delta est calcule par rapport au
/// dernier appel a `advance`, pas par rapport au tick rate theorique.
#[derive(Debug, Clone)]
pub struct GameTime {
    /// Numero du tick courant (0 au demarrage, incremente a chaque `advance`).
    pub tick: u64,
    /// Temps total ecoule depuis la creation du `GameTime`.
    pub elapsed: Duration,
    /// Duree entre le dernier `advance` et le precedent.
    pub delta: Duration,
    /// `delta` converti en secondes (f32), pour les calculs de physique.
    pub delta_secs: f32,
    /// Instant de creation (reference pour `elapsed`).
    start: Instant,
    /// Instant du dernier appel a `advance`.
    last_tick: Instant,
}

impl GameTime {
    /// Cree un nouveau `GameTime` avec tick 0, delta zero, horloge demarre maintenant.
    #[must_use]
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            tick: 0,
            elapsed: Duration::ZERO,
            delta: Duration::ZERO,
            delta_secs: 0.0,
            start: now,
            last_tick: now,
        }
    }

    /// Avance le temps d'un tick.
    ///
    /// Met a jour `delta`, `delta_secs`, `elapsed`, et incremente `tick`.
    /// Appele par la boucle de jeu a chaque tick logique.
    pub fn advance(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_tick;
        self.delta_secs = self.delta.as_secs_f32();
        self.elapsed = now - self.start;
        self.last_tick = now;
        self.tick += 1;
    }
}

impl Default for GameTime {
    fn default() -> Self {
        Self::new()
    }
}
