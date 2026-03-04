// @id: MUIT-Typography @do: font-tokens @role: token @layer: 5 @human: miyuk

/// Font family specification.
///
/// Contains `&'static str` references, so only `Serialize` is derived
/// when the `serde` feature is active. Deserialization is not supported
/// because font families are compile-time constants.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FontFamily {
    /// Primary font name (e.g. "Inter").
    pub primary: &'static str,
    /// Fallback chain (e.g. "'Segoe UI', -apple-system, sans-serif").
    pub fallback: &'static str,
}

impl FontFamily {
    /// COG UI font (Inter).
    pub const COG_UI: Self = Self {
        primary: "Inter",
        fallback: "'Segoe UI', -apple-system, sans-serif",
    };

    /// COG monospace font.
    pub const COG_MONO: Self = Self {
        primary: "JetBrains Mono",
        fallback: "'Cascadia Code', 'Fira Code', monospace",
    };

    /// D2 title font (Exocet-like).
    pub const D2_TITLE: Self = Self {
        primary: "Cinzel Decorative",
        fallback: "'Cinzel', serif",
    };

    /// D2 body font.
    pub const D2_BODY: Self = Self {
        primary: "Georgia",
        fallback: "'Times New Roman', serif",
    };

    /// D2 monospace for stats.
    pub const D2_MONO: Self = Self {
        primary: "Courier New",
        fallback: "monospace",
    };

    /// Full CSS font-family string.
    pub fn to_css(&self) -> String {
        format!("'{}', {}", self.primary, self.fallback)
    }
}

/// Font weight enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum FontWeight {
    /// 400 -- regular text.
    Regular = 400,
    /// 500 -- labels, navigation.
    Medium = 500,
    /// 600 -- section titles, headers.
    SemiBold = 600,
    /// 700 -- page titles, strong emphasis.
    Bold = 700,
    /// 800 -- display, hero titles.
    ExtraBold = 800,
}

impl FontWeight {
    /// CSS numeric value.
    pub const fn value(self) -> u16 {
        self as u16
    }
}

/// Font size step in the typographic scale.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FontSize {
    /// Size in logical pixels.
    pub px: f32,
    /// Recommended line-height multiplier.
    pub line_height: f32,
}

impl FontSize {
    /// Create a new font size with its line-height.
    pub const fn new(px: f32, line_height: f32) -> Self {
        Self { px, line_height }
    }

    /// CSS font-size string.
    pub fn to_css(&self) -> String {
        format!("{}px", self.px)
    }

    /// CSS line-height string (unitless).
    pub fn line_height_css(&self) -> String {
        format!("{}", self.line_height)
    }
}

/// Complete typographic scale.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TypographyScale {
    /// UI font family.
    pub family_ui: FontFamily,
    /// Display/title font family.
    pub family_display: FontFamily,
    /// Monospace font family.
    pub family_mono: FontFamily,

    /// 10px (COG) / 8px (D2) -- badges, legal, counters.
    pub xs: FontSize,
    /// 12px (COG) / 10px (D2) -- secondary labels, hints.
    pub sm: FontSize,
    /// 14px (COG) / 12px (D2) -- body text (base).
    pub body: FontSize,
    /// 16px (COG) / 14px (D2) -- subtitles, important labels.
    pub lg: FontSize,
    /// 18px (COG) / 16px (D2) -- section titles.
    pub xl: FontSize,
    /// 24px -- page titles.
    pub xxl: FontSize,
    /// 32px -- main titles, hero text.
    pub xxxl: FontSize,
    /// 40px (COG) / 48px (D2) -- display, splash.
    pub display: FontSize,
}

/// Pre-built text style combining size, weight, and family.
#[derive(Debug, Clone, PartialEq)]
pub struct TextStyle {
    /// Font size and line-height.
    pub size: FontSize,
    /// Font weight.
    pub weight: FontWeight,
    /// Font family.
    pub family: FontFamily,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_font_family_cog_ui_to_css() {
        let css = FontFamily::COG_UI.to_css();
        assert!(css.contains("Inter"));
        assert!(css.contains("Segoe UI"));
    }

    #[test]
    fn test_font_family_d2_title_to_css() {
        let css = FontFamily::D2_TITLE.to_css();
        assert!(css.contains("Cinzel Decorative"));
    }

    #[test]
    fn test_font_weight_values() {
        assert_eq!(FontWeight::Regular.value(), 400);
        assert_eq!(FontWeight::Medium.value(), 500);
        assert_eq!(FontWeight::SemiBold.value(), 600);
        assert_eq!(FontWeight::Bold.value(), 700);
        assert_eq!(FontWeight::ExtraBold.value(), 800);
    }

    #[test]
    fn test_font_size_to_css() {
        let fs = FontSize::new(14.0, 1.6);
        assert_eq!(fs.to_css(), "14px");
    }

    #[test]
    fn test_font_size_line_height_css() {
        let fs = FontSize::new(14.0, 1.6);
        assert_eq!(fs.line_height_css(), "1.6");
    }

    #[test]
    fn test_font_size_positive() {
        let fs = FontSize::new(14.0, 1.6);
        assert!(fs.px > 0.0);
        assert!(fs.line_height > 0.0);
    }
}
