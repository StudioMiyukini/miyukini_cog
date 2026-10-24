// @id MUIT-Radius
// @do radius-scale
// @role token
// @layer 5
// @human miyuk

/// Border radius scale.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RadiusScale {
    /// 0px -- square edges.
    pub none: f32,
    /// 2px (COG) -- subtle (badges, tags).
    pub sm: f32,
    /// 4px (COG) -- standard (buttons, inputs, cards).
    pub md: f32,
    /// 8px (COG) -- elevated cards, modals.
    pub lg: f32,
    /// 12px (COG) -- rounded (tooltips, toasts).
    pub xl: f32,
    /// 16px (COG) -- very rounded (avatar squares, chips).
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
