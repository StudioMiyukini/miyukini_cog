// @id: Sodomight-Client-Config @do: client-config @role: back-end @layer: 4 @human: miyuk
//! Client configuration.

/// Sodomight game client configuration.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClientConfig {
    /// Remote server address (host:port).
    pub server_addr: String,
    /// Window width in pixels.
    pub window_width: u32,
    /// Window height in pixels.
    pub window_height: u32,
    /// Master audio volume \[0.0, 1.0\].
    pub master_volume: f32,
    /// Target rendering frame rate.
    pub target_fps: u32,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            server_addr: String::from("127.0.0.1:7777"),
            window_width: 1280,
            window_height: 720,
            master_volume: 1.0,
            target_fps: 60,
        }
    }
}

/// Create a `ClientConfig` with default values.
pub fn default_client_config() -> ClientConfig {
    ClientConfig::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_server_addr() {
        let cfg = ClientConfig::default();
        assert_eq!(cfg.server_addr, "127.0.0.1:7777");
    }

    #[test]
    fn test_default_window() {
        let cfg = ClientConfig::default();
        assert_eq!(cfg.window_width, 1280);
        assert_eq!(cfg.window_height, 720);
    }

    #[test]
    fn test_default_volume() {
        let cfg = ClientConfig::default();
        assert!((cfg.master_volume - 1.0).abs() < f32::EPSILON);
    }
}
