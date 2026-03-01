// @id: MGE-Platform-Window @do: window-config @role: back-end @layer: 1 @human: miyuk
//! Window configuration and metadata.

use crate::config::PlatformConfig;

/// Runtime window state (size, title, DPI).
#[derive(Debug, Clone)]
pub struct WindowConfig {
    /// Window width in pixels.
    pub width: u32,
    /// Window height in pixels.
    pub height: u32,
    /// Window title text.
    pub title: String,
    /// DPI scale factor (1.0 = standard, 2.0 = retina/HiDPI).
    pub dpi_scale: f32,
}

impl WindowConfig {
    /// Creates a `WindowConfig` from a `PlatformConfig`, defaulting DPI to 1.0.
    pub fn from_platform_config(cfg: &PlatformConfig) -> Self {
        Self {
            width: cfg.width,
            height: cfg.height,
            title: cfg.title.clone(),
            dpi_scale: 1.0,
        }
    }

    /// Converts a logical size to physical pixels using the DPI scale.
    pub fn logical_to_physical(&self, logical: f32) -> f32 {
        logical * self.dpi_scale
    }

    /// Converts a physical pixel size to logical units using the DPI scale.
    pub fn physical_to_logical(&self, physical: f32) -> f32 {
        if self.dpi_scale == 0.0 {
            return physical;
        }
        physical / self.dpi_scale
    }

    /// Returns the aspect ratio (width / height). Returns 1.0 if height is zero.
    pub fn aspect_ratio(&self) -> f32 {
        if self.height == 0 {
            return 1.0;
        }
        #[allow(clippy::cast_precision_loss)]
        let ratio = self.width as f32 / self.height as f32;
        ratio
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_platform_config() {
        let cfg = PlatformConfig::new(1920, 1080, "Game");
        let win = WindowConfig::from_platform_config(&cfg);
        assert_eq!(win.width, 1920);
        assert_eq!(win.height, 1080);
        assert_eq!(win.title, "Game");
        assert!((win.dpi_scale - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_aspect_ratio() {
        let win = WindowConfig {
            width: 1920,
            height: 1080,
            title: String::from("Test"),
            dpi_scale: 1.0,
        };
        let ratio = win.aspect_ratio();
        let expected = 1920.0_f32 / 1080.0;
        assert!((ratio - expected).abs() < 0.001);
    }

    #[test]
    fn test_logical_to_physical() {
        let win = WindowConfig {
            width: 800,
            height: 600,
            title: String::from("Test"),
            dpi_scale: 2.0,
        };
        let physical = win.logical_to_physical(100.0);
        assert!((physical - 200.0).abs() < f32::EPSILON);

        let logical = win.physical_to_logical(200.0);
        assert!((logical - 100.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_zero_height_aspect() {
        let win = WindowConfig {
            width: 800,
            height: 0,
            title: String::from("Test"),
            dpi_scale: 1.0,
        };
        assert!((win.aspect_ratio() - 1.0).abs() < f32::EPSILON);
    }
}
