// @id: MUIT-Shadow @do: shadow-scale @role: token @layer: 5 @human: miyuk

use crate::color::Rgba;

/// A single shadow definition.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Shadow {
    /// Horizontal offset in px.
    pub offset_x: f32,
    /// Vertical offset in px.
    pub offset_y: f32,
    /// Blur radius in px.
    pub blur: f32,
    /// Spread radius in px.
    pub spread: f32,
    /// Shadow color.
    pub color: Rgba,
}

impl Shadow {
    /// No shadow.
    pub const NONE: Self = Self {
        offset_x: 0.0,
        offset_y: 0.0,
        blur: 0.0,
        spread: 0.0,
        color: Rgba::with_alpha(0, 0, 0, 0),
    };

    /// Create a new shadow.
    pub const fn new(offset_x: f32, offset_y: f32, blur: f32, spread: f32, color: Rgba) -> Self {
        Self {
            offset_x,
            offset_y,
            blur,
            spread,
            color,
        }
    }

    /// CSS box-shadow string.
    pub fn to_css(&self) -> String {
        if self.blur == 0.0 && self.offset_x == 0.0 && self.offset_y == 0.0 {
            "none".to_string()
        } else {
            format!(
                "{}px {}px {}px {}px {}",
                self.offset_x,
                self.offset_y,
                self.blur,
                self.spread,
                self.color.to_css()
            )
        }
    }
}

/// Shadow elevation scale.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ShadowScale {
    /// No shadow.
    pub none: Shadow,
    /// Small shadow: `0 1px 2px`.
    pub sm: Shadow,
    /// Medium shadow: `0 4px 12px`.
    pub md: Shadow,
    /// Large shadow: `0 8px 24px`.
    pub lg: Shadow,
    /// Extra-large shadow: `0 16px 48px`.
    pub xl: Shadow,
}

impl ShadowScale {
    /// COG shadow scale (strong shadows on dark backgrounds).
    pub const fn cog() -> Self {
        Self {
            none: Shadow::NONE,
            sm: Shadow::new(0.0, 1.0, 2.0, 0.0, Rgba::with_alpha(0, 0, 0, 77)),
            md: Shadow::new(0.0, 4.0, 12.0, 0.0, Rgba::with_alpha(0, 0, 0, 102)),
            lg: Shadow::new(0.0, 8.0, 24.0, 0.0, Rgba::with_alpha(0, 0, 0, 128)),
            xl: Shadow::new(0.0, 16.0, 48.0, 0.0, Rgba::with_alpha(0, 0, 0, 153)),
        }
    }

    /// D2 minimal shadows (ambiance by color, not elevation).
    pub const fn d2() -> Self {
        Self {
            none: Shadow::NONE,
            sm: Shadow::new(0.0, 1.0, 2.0, 0.0, Rgba::with_alpha(0, 0, 0, 128)),
            md: Shadow::new(0.0, 2.0, 6.0, 0.0, Rgba::with_alpha(0, 0, 0, 153)),
            lg: Shadow::new(0.0, 4.0, 12.0, 0.0, Rgba::with_alpha(0, 0, 0, 179)),
            xl: Shadow::new(0.0, 8.0, 24.0, 0.0, Rgba::with_alpha(0, 0, 0, 204)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shadow_none_to_css() {
        assert_eq!(Shadow::NONE.to_css(), "none");
    }

    #[test]
    fn test_shadow_sm_to_css() {
        let scale = ShadowScale::cog();
        let css = scale.sm.to_css();
        assert!(css.contains("1px"));
        assert!(css.contains("2px"));
    }

    #[test]
    fn test_cog_shadows_increase_in_blur() {
        let s = ShadowScale::cog();
        assert!(s.sm.blur < s.md.blur);
        assert!(s.md.blur < s.lg.blur);
        assert!(s.lg.blur < s.xl.blur);
    }

    #[test]
    fn test_d2_shadows_smaller_than_cog() {
        let cog = ShadowScale::cog();
        let d2 = ShadowScale::d2();
        // D2 shadows should be less dramatic (smaller blur at same level)
        assert!(d2.md.blur <= cog.md.blur);
        assert!(d2.lg.blur <= cog.lg.blur);
    }
}
