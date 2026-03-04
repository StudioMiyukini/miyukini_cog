// @id: MUIT-Color @do: color-type @role: token @layer: 5 @human: miyuk

/// RGBA color with 8 bits per channel.
///
/// All color values in the Miyukini UI ecosystem use this type.
/// Conversions to CSS, hex, and raw arrays are provided for use
/// by framework-specific adaptors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rgba {
    /// Red channel (0-255).
    pub r: u8,
    /// Green channel (0-255).
    pub g: u8,
    /// Blue channel (0-255).
    pub b: u8,
    /// Alpha channel (0 = transparent, 255 = opaque).
    pub a: u8,
}

impl Rgba {
    /// Create a new fully opaque color.
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create a new color with explicit alpha.
    pub const fn with_alpha(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// CSS rgba() string: `"rgb(r, g, b)"` if opaque, `"rgba(r, g, b, a)"` otherwise.
    pub fn to_css(&self) -> String {
        if self.a == 255 {
            format!("rgb({}, {}, {})", self.r, self.g, self.b)
        } else {
            let alpha = f64::from(self.a) / 255.0;
            format!("rgba({}, {}, {}, {alpha:.2})", self.r, self.g, self.b)
        }
    }

    /// CSS hex string: `"#rrggbb"` (opaque) or `"#rrggbbaa"` (with alpha).
    pub fn to_hex(&self) -> String {
        if self.a == 255 {
            format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        } else {
            format!(
                "#{:02x}{:02x}{:02x}{:02x}",
                self.r, self.g, self.b, self.a
            )
        }
    }

    /// Raw `[r, g, b, a]` array for use with egui `Color32::from_rgba_premultiplied`.
    pub const fn to_array(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Apply a different alpha value, returning a new color.
    #[must_use]
    pub const fn alpha(self, a: u8) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            a,
        }
    }
}

impl core::fmt::Display for Rgba {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_is_opaque() {
        let c = Rgba::new(10, 20, 30);
        assert_eq!(c.a, 255);
    }

    #[test]
    fn test_with_alpha() {
        let c = Rgba::with_alpha(10, 20, 30, 128);
        assert_eq!(c.r, 10);
        assert_eq!(c.g, 20);
        assert_eq!(c.b, 30);
        assert_eq!(c.a, 128);
    }

    #[test]
    fn test_to_css_opaque() {
        let c = Rgba::new(255, 128, 0);
        assert_eq!(c.to_css(), "rgb(255, 128, 0)");
    }

    #[test]
    fn test_to_css_with_alpha() {
        let c = Rgba::with_alpha(255, 128, 0, 128);
        // 128/255 ~ 0.50
        assert_eq!(c.to_css(), "rgba(255, 128, 0, 0.50)");
    }

    #[test]
    fn test_to_hex_opaque() {
        let c = Rgba::new(14, 16, 21);
        assert_eq!(c.to_hex(), "#0e1015");
    }

    #[test]
    fn test_to_hex_with_alpha() {
        let c = Rgba::with_alpha(26, 159, 255, 26);
        assert_eq!(c.to_hex(), "#1a9fff1a");
    }

    #[test]
    fn test_to_array() {
        let c = Rgba::new(200, 165, 70);
        assert_eq!(c.to_array(), [200, 165, 70, 255]);
    }

    #[test]
    fn test_to_array_with_alpha() {
        let c = Rgba::with_alpha(180, 20, 20, 40);
        assert_eq!(c.to_array(), [180, 20, 20, 40]);
    }

    #[test]
    fn test_alpha_method() {
        let c = Rgba::new(100, 150, 200);
        let transparent = c.alpha(128);
        assert_eq!(transparent.r, 100);
        assert_eq!(transparent.g, 150);
        assert_eq!(transparent.b, 200);
        assert_eq!(transparent.a, 128);
    }

    #[test]
    fn test_display_opaque() {
        let c = Rgba::new(14, 16, 21);
        assert_eq!(format!("{c}"), "#0e1015");
    }

    #[test]
    fn test_display_with_alpha() {
        let c = Rgba::with_alpha(255, 0, 0, 128);
        assert_eq!(format!("{c}"), "#ff000080");
    }

    #[test]
    fn test_black_and_white() {
        let black = Rgba::new(0, 0, 0);
        assert_eq!(black.to_hex(), "#000000");

        let white = Rgba::new(255, 255, 255);
        assert_eq!(white.to_hex(), "#ffffff");
    }

    #[test]
    fn test_fully_transparent() {
        let c = Rgba::with_alpha(0, 0, 0, 0);
        assert_eq!(c.to_hex(), "#00000000");
        assert_eq!(c.to_css(), "rgba(0, 0, 0, 0.00)");
    }

    #[test]
    fn test_equality() {
        let a = Rgba::new(10, 20, 30);
        let b = Rgba::new(10, 20, 30);
        let c = Rgba::new(10, 20, 31);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_copy_semantics() {
        let a = Rgba::new(10, 20, 30);
        let b = a;
        assert_eq!(a, b); // a is still usable because Rgba is Copy
    }
}
