// @id: Sodomight-Server-Tick @do: tick-state @role: back-end @layer: 4 @human: miyuk
//! Server tick state tracking.

/// Tracks the server's current tick and runtime statistics.
#[derive(Debug, Clone, Default)]
pub struct TickState {
    /// Total ticks elapsed since server start.
    pub tick_count: u64,
    /// Number of currently connected clients.
    pub connected_clients: usize,
    /// Total messages processed across all ticks.
    pub messages_processed: u64,
}

impl TickState {
    /// Create a new tick state at tick zero.
    pub fn new() -> Self {
        Self::default()
    }

    /// Advance to the next tick.
    pub fn advance(&mut self) {
        self.tick_count += 1;
    }

    /// Returns true if it's time to save (based on `save_interval_ticks`).
    pub fn should_save(&self, save_interval_ticks: u32) -> bool {
        if save_interval_ticks == 0 {
            return false;
        }
        self.tick_count.is_multiple_of(u64::from(save_interval_ticks)) && self.tick_count > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick_advance() {
        let mut state = TickState::new();
        assert_eq!(state.tick_count, 0);
        state.advance();
        assert_eq!(state.tick_count, 1);
        state.advance();
        assert_eq!(state.tick_count, 2);
    }

    #[test]
    fn test_tick_should_save_true() {
        let state = TickState {
            tick_count: 1500,
            ..TickState::default()
        };
        assert!(state.should_save(1500));
    }

    #[test]
    fn test_tick_should_save_false() {
        let state = TickState {
            tick_count: 1,
            ..TickState::default()
        };
        assert!(!state.should_save(1500));
    }

    #[test]
    fn test_tick_should_save_zero_interval() {
        let state = TickState {
            tick_count: 100,
            ..TickState::default()
        };
        assert!(!state.should_save(0));
    }
}
