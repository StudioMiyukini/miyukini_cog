// @id: Sodomight-Server-Config @do: server-config @role: back-end @layer: 4 @human: miyuk
//! Server configuration.

/// Sodomight dedicated server configuration.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ServerConfig {
    /// Network bind address (host:port).
    pub bind_addr: String,
    /// Maximum simultaneous connected clients.
    pub max_clients: usize,
    /// Server tick rate in Hz (25 = 40ms per tick).
    pub tick_rate_hz: u32,
    /// Save to disk every N ticks.
    pub save_interval_ticks: u32,
    /// Path to the game data directory.
    pub data_dir: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_addr: String::from("0.0.0.0:7777"),
            max_clients: 200,
            tick_rate_hz: 25,
            save_interval_ticks: 1500,
            data_dir: String::from("data/"),
        }
    }
}

impl ServerConfig {
    /// Tick duration in milliseconds.
    pub fn tick_ms(&self) -> u64 {
        if self.tick_rate_hz == 0 {
            return 40;
        }
        1000 / u64::from(self.tick_rate_hz)
    }
}

/// Create a server config with sensible defaults.
pub fn default_server_config() -> ServerConfig {
    ServerConfig::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_bind_addr() {
        let cfg = ServerConfig::default();
        assert_eq!(cfg.bind_addr, "0.0.0.0:7777");
    }

    #[test]
    fn test_default_max_clients() {
        let cfg = ServerConfig::default();
        assert_eq!(cfg.max_clients, 200);
    }

    #[test]
    fn test_default_tick_rate() {
        let cfg = ServerConfig::default();
        assert_eq!(cfg.tick_rate_hz, 25);
    }

    #[test]
    fn test_tick_ms() {
        let cfg = ServerConfig::default();
        assert_eq!(cfg.tick_ms(), 40);
    }

    #[test]
    fn test_tick_ms_zero_rate() {
        let cfg = ServerConfig {
            tick_rate_hz: 0,
            ..ServerConfig::default()
        };
        assert_eq!(cfg.tick_ms(), 40);
    }
}
