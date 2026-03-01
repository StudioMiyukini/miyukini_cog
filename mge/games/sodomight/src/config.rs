// @id: Sodomight-Game-Config @do: game-config @role: back-end @layer: 4 @human: miyuk
//! Game configuration.

/// Main game configuration.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GameConfig {
    /// Window width in pixels.
    pub window_width: u32,
    /// Window height in pixels.
    pub window_height: u32,
    /// Window title.
    pub title: String,
    /// Target rendering frame rate.
    pub target_fps: u32,
    /// Server-authoritative tick rate in Hz.
    pub tick_rate_hz: u32,
    /// Path to game data directory.
    pub data_dir: String,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            window_width: 1280,
            window_height: 720,
            title: String::from("Sodomight"),
            target_fps: 60,
            tick_rate_hz: 25,
            data_dir: String::from("data/"),
        }
    }
}

impl GameConfig {
    /// Milliseconds between game ticks.
    pub fn tick_ms(&self) -> u64 {
        if self.tick_rate_hz == 0 {
            return 40;
        }
        1000 / u64::from(self.tick_rate_hz)
    }

    /// Milliseconds between render frames.
    pub fn frame_ms(&self) -> u64 {
        if self.target_fps == 0 {
            return 16;
        }
        1000 / u64::from(self.target_fps)
    }
}

/// Create a `GameConfig` with production defaults.
pub fn default_game_config() -> GameConfig {
    GameConfig::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_window_size() {
        let cfg = GameConfig::default();
        assert_eq!(cfg.window_width, 1280);
        assert_eq!(cfg.window_height, 720);
    }

    #[test]
    fn test_default_fps() {
        let cfg = GameConfig::default();
        assert_eq!(cfg.target_fps, 60);
    }

    #[test]
    fn test_tick_ms_25hz() {
        let cfg = GameConfig::default();
        assert_eq!(cfg.tick_ms(), 40);
    }

    #[test]
    fn test_frame_ms_60fps() {
        let cfg = GameConfig::default();
        assert_eq!(cfg.frame_ms(), 16);
    }

    #[test]
    fn test_tick_ms_zero_rate() {
        let mut cfg = GameConfig::default();
        cfg.tick_rate_hz = 0;
        assert_eq!(cfg.tick_ms(), 40);
    }
}
