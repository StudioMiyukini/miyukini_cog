// @id: MGE-Platform-Config @do: platform-config @role: back-end @layer: 1 @human: miyuk
//! Platform configuration.

/// Platform and window configuration.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PlatformConfig {
    /// Window width in pixels.
    pub width: u32,
    /// Window height in pixels.
    pub height: u32,
    /// Window title.
    pub title: String,
    /// Whether to use fullscreen mode.
    pub fullscreen: bool,
    /// Whether to enable vertical sync.
    pub vsync: bool,
}

impl Default for PlatformConfig {
    fn default() -> Self {
        Self {
            width: 1280,
            height: 720,
            title: String::from("Sodomight"),
            fullscreen: false,
            vsync: true,
        }
    }
}

impl PlatformConfig {
    /// Creates a new platform configuration with the given dimensions and title.
    pub fn new(width: u32, height: u32, title: impl Into<String>) -> Self {
        Self {
            width,
            height,
            title: title.into(),
            fullscreen: false,
            vsync: true,
        }
    }

    /// Sets whether fullscreen mode is enabled.
    pub fn with_fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = fullscreen;
        self
    }

    /// Sets whether vertical sync is enabled.
    pub fn with_vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let cfg = PlatformConfig::default();
        assert_eq!(cfg.width, 1280);
        assert_eq!(cfg.height, 720);
        assert_eq!(cfg.title, "Sodomight");
        assert!(!cfg.fullscreen);
        assert!(cfg.vsync);
    }

    #[test]
    fn test_new_config() {
        let cfg = PlatformConfig::new(800, 600, "Test");
        assert_eq!(cfg.width, 800);
        assert_eq!(cfg.height, 600);
        assert_eq!(cfg.title, "Test");
        assert!(!cfg.fullscreen);
        assert!(cfg.vsync);
    }

    #[test]
    fn test_builder_fullscreen() {
        let cfg = PlatformConfig::default().with_fullscreen(true);
        assert!(cfg.fullscreen);
    }

    #[test]
    fn test_builder_vsync() {
        let cfg = PlatformConfig::default().with_vsync(false);
        assert!(!cfg.vsync);
    }
}
