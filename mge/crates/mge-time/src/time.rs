//! Time — delta, tick count, scale, pause.
//!
//! @id mge.kernel.time.struct
//! @role data
//! @layer kernel
//! @do store_tick_based_time_state

/// Temps de simulation : delta, tick count, scale, pause.
#[derive(Debug, Clone)]
pub struct Time {
    /// Temps écoulé depuis le dernier tick (secondes).
    pub delta_secs: f32,
    /// Compteur de ticks.
    pub tick_count: u64,
    /// Facteur multiplicatif (0.5 = ralenti, 2.0 = accéléré).
    pub time_scale: f32,
    /// Si true, delta_secs = 0 pour la simulation.
    pub paused: bool,
}

impl Time {
    pub fn new() -> Self {
        Self {
            delta_secs: 0.0,
            tick_count: 0,
            time_scale: 1.0,
            paused: false,
        }
    }

    /// Avance le temps. Si fixed_timestep_ms, utilise un delta fixe ; sinon delta_requested.
    /// Si paused, delta_secs = 0.
    pub fn advance(&mut self, delta_requested_secs: f32, fixed_timestep_ms: Option<u32>) {
        if self.paused {
            self.delta_secs = 0.0;
            self.tick_count = self.tick_count.saturating_add(1);
            return;
        }

        let raw_delta = if let Some(ms) = fixed_timestep_ms {
            (ms as f32) / 1000.0
        } else {
            delta_requested_secs
        };

        self.delta_secs = raw_delta * self.time_scale;
        self.tick_count = self.tick_count.saturating_add(1);
    }
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}
