// @id: MUIT-Radius @do: radius-scale @role: token @layer: 5 @human: miyuk

/// Border radius scale.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RadiusScale {
    /// 0px -- square edges.
    pub none: f32,
    /// 2px (COG) / 1px (D2) -- subtle (badges, tags, D2 panels).
    pub sm: f32,
    /// 4px (COG) / 2px (D2) -- standard (buttons, inputs, cards).
    pub md: f32,
    /// 8px (COG) / 3px (D2) -- elevated cards, modals.
    pub lg: f32,
    /// 12px (COG) / 4px (D2) -- rounded (tooltips, toasts).
    pub xl: f32,
    /// 16px (COG) / 6px (D2) -- very rounded (avatar squares, chips).
    pub xxl: f32,
    /// 9999px -- full circle (avatars, pills).
    pub full: f32,
}

impl RadiusScale {
    /// COG standard radius scale.
    pub const fn cog() -> Self {
        Self {
            none: 0.0,
            sm: 2.0,
            md: 4.0,
            lg: 8.0,
            xl: 12.0,
            xxl: 16.0,
            full: 9999.0,
        }
    }

    /// D2 medieval radius scale (mostly square).
    pub const fn d2() -> Self {
        Self {
            none: 0.0,
            sm: 1.0,
            md: 2.0,
            lg: 3.0,
            xl: 4.0,
            xxl: 6.0,
            full: 9999.0,
        }
    }

    /// Convert a radius value to CSS string.
    pub fn to_css(value: f32) -> String {
        if value == 0.0 {
            "0".to_string()
        } else if value > 9000.0 {
            "9999px".to_string()
        } else {
            format!("{value}px")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cog_radius() {
        let r = RadiusScale::cog();
        assert!((r.none - 0.0).abs() < f32::EPSILON);
        assert!((r.sm - 2.0).abs() < f32::EPSILON);
        assert!((r.md - 4.0).abs() < f32::EPSILON);
        assert!((r.lg - 8.0).abs() < f32::EPSILON);
        assert!((r.xl - 12.0).abs() < f32::EPSILON);
        assert!((r.xxl - 16.0).abs() < f32::EPSILON);
        assert!((r.full - 9999.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_d2_radius_smaller_than_cog() {
        let cog = RadiusScale::cog();
        let d2 = RadiusScale::d2();
        assert!(d2.sm <= cog.sm);
        assert!(d2.md <= cog.md);
        assert!(d2.lg <= cog.lg);
    }

    #[test]
    fn test_to_css_zero() {
        assert_eq!(RadiusScale::to_css(0.0), "0");
    }

    #[test]
    fn test_to_css_normal() {
        assert_eq!(RadiusScale::to_css(4.0), "4px");
    }

    #[test]
    fn test_to_css_full() {
        assert_eq!(RadiusScale::to_css(9999.0), "9999px");
    }
}
