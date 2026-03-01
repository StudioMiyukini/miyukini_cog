//! Boucle de jeu a tick rate fixe avec accumulation.
//!
//! <!-- @id: MGE-Core-GameLoop @do: game-loop @role: back-end @layer: 1 @human: miyuk -->
//!
//! Implementation d'une boucle de jeu a pas de temps fixe (fixed timestep).
//! Le pattern utilise est "fix your timestep" de Glenn Fiedler :
//! - Accumulation du temps reel entre frames
//! - Execution de N ticks logiques par frame
//! - Interpolation pour le rendu entre deux ticks
//!
//! La "spiral of death" est evitee via [`LoopConfig::max_ticks_per_frame`]
//! qui borne le nombre de ticks de rattrapage par frame.

use std::time::{Duration, Instant};

use crate::time::{GameTime, TickRate};

/// Configuration de la boucle de jeu.
#[derive(Debug, Clone)]
pub struct LoopConfig {
    /// Frequence de simulation logique.
    pub tick_rate: TickRate,
    /// Nombre maximum de ticks de rattrapage par frame.
    ///
    /// Evite la "spiral of death" : si le jeu prend du retard,
    /// on borne le rattrapage a `max_ticks_per_frame` ticks par frame
    /// et on accepte le ralentissement plutot que de bloquer indefiniment.
    pub max_ticks_per_frame: u32,
}

impl Default for LoopConfig {
    fn default() -> Self {
        Self {
            tick_rate: TickRate::SODOMIGHT,
            max_ticks_per_frame: 5,
        }
    }
}

/// Boucle de jeu principale -- logique de tick fixe + rendu variable.
///
/// Utilisation typique :
/// ```rust,no_run
/// use mge_core::game_loop::{GameLoop, LoopConfig};
///
/// let mut game_loop = GameLoop::new(LoopConfig::default());
///
/// // Dans la event loop winit :
/// let ticks = game_loop.begin_frame();
/// for _ in 0..ticks {
///     // Executer la logique de jeu pour un tick
/// }
/// let alpha = game_loop.alpha();
/// // Rendu avec interpolation `alpha`
/// ```
pub struct GameLoop {
    config: LoopConfig,
    accumulator: Duration,
    last_frame: Instant,
    /// Temps de jeu (mis a jour par `begin_frame` a chaque tick).
    pub time: GameTime,
}

impl GameLoop {
    /// Cree une nouvelle boucle de jeu avec la configuration donnee.
    #[must_use]
    pub fn new(config: LoopConfig) -> Self {
        Self {
            config,
            accumulator: Duration::ZERO,
            last_frame: Instant::now(),
            time: GameTime::new(),
        }
    }

    /// Appele chaque frame par la event loop winit.
    ///
    /// Calcule le temps ecoule depuis la derniere frame, accumule,
    /// et retourne le nombre de ticks logiques a executer cette frame.
    /// Chaque tick appelle `GameTime::advance` en interne.
    pub fn begin_frame(&mut self) -> u32 {
        let now = Instant::now();
        let frame_time = now - self.last_frame;
        self.last_frame = now;
        self.accumulator += frame_time;

        let tick_dur = self.config.tick_rate.tick_duration();
        let mut ticks = 0u32;

        while self.accumulator >= tick_dur && ticks < self.config.max_ticks_per_frame {
            self.accumulator -= tick_dur;
            self.time.advance();
            ticks += 1;
        }

        ticks
    }

    /// Interpolation `[0.0 .. 1.0]` pour le rendu entre deux ticks.
    ///
    /// `0.0` = on vient juste d'executer un tick.
    /// `1.0` = on est a un tick de retard (le prochain tick est imminent).
    ///
    /// Utilise par le renderer pour interpoler les positions visuelles
    /// entre l'etat du tick precedent et l'etat du tick courant.
    #[must_use]
    pub fn alpha(&self) -> f32 {
        let tick_dur = self.config.tick_rate.tick_duration();
        if tick_dur.is_zero() {
            return 0.0;
        }
        self.accumulator.as_secs_f32() / tick_dur.as_secs_f32()
    }

    /// Reference immutable vers la configuration.
    #[must_use]
    pub fn config(&self) -> &LoopConfig {
        &self.config
    }
}
