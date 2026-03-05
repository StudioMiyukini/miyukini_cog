// @id: MUIT-Spacing @do: spacing-scale @role: token @layer: 5 @human: miyuk

/// Spacing scale based on a 4px unit.
///
/// All spacing values in the UI system are multiples of the base unit.
/// This ensures visual consistency and alignment across all components.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SpacingScale {
    /// Base unit in logical pixels (typically 4.0).
    pub unit: f32,
    /// 0px -- no spacing.
    pub space_0: f32,
    /// 4px (1x) -- micro-gap (icon-text in badge).
    pub space_1: f32,
    /// 8px (2x) -- small gap (group items, badge padding).
    pub space_2: f32,
    /// 12px (3x) -- medium-small (button vertical padding).
    pub space_3: f32,
    /// 16px (4x) -- standard (card padding, grid gap).
    pub space_4: f32,
    /// 20px (5x) -- medium-large.
    pub space_5: f32,
    /// 24px (6x) -- large (section padding, title margin).
    pub space_6: f32,
    /// 32px (8x) -- x-large (section separation).
    pub space_8: f32,
    /// 40px (10x) -- layout (header height).
    pub space_10: f32,
    /// 48px (12x) -- layout large.
    pub space_12: f32,
    /// 64px (16x) -- macro (page margins, hero padding).
    pub space_16: f32,
}

impl SpacingScale {
    /// Standard Miyukini spacing scale (base 4px).
    pub const fn standard() -> Self {
        Self {
            unit: 4.0,
            space_0: 0.0,
            space_1: 4.0,
            space_2: 8.0,
            space_3: 12.0,
            space_4: 16.0,
            space_5: 20.0,
            space_6: 24.0,
            space_8: 32.0,
            space_10: 40.0,
            space_12: 48.0,
            space_16: 64.0,
        }
    }

    /// Convert a spacing value to CSS pixel string (e.g. "16px").
    pub fn to_css_px(value: f32) -> String {
        if value == 0.0 {
            "0".to_string()
        } else {
            format!("{value}px")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_scale_base_unit() {
        let s = SpacingScale::standard();
        assert!((s.unit - 4.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_standard_scale_values() {
        let s = SpacingScale::standard();
        assert!((s.space_0 - 0.0).abs() < f32::EPSILON);
        assert!((s.space_1 - 4.0).abs() < f32::EPSILON);
        assert!((s.space_2 - 8.0).abs() < f32::EPSILON);
        assert!((s.space_3 - 12.0).abs() < f32::EPSILON);
        assert!((s.space_4 - 16.0).abs() < f32::EPSILON);
        assert!((s.space_5 - 20.0).abs() < f32::EPSILON);
        assert!((s.space_6 - 24.0).abs() < f32::EPSILON);
        assert!((s.space_8 - 32.0).abs() < f32::EPSILON);
        assert!((s.space_10 - 40.0).abs() < f32::EPSILON);
        assert!((s.space_12 - 48.0).abs() < f32::EPSILON);
        assert!((s.space_16 - 64.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_all_values_positive_or_zero() {
        let s = SpacingScale::standard();
        assert!(s.space_0 >= 0.0);
        assert!(s.space_1 >= 0.0);
        assert!(s.space_2 >= 0.0);
        assert!(s.space_3 >= 0.0);
        assert!(s.space_4 >= 0.0);
        assert!(s.space_5 >= 0.0);
        assert!(s.space_6 >= 0.0);
        assert!(s.space_8 >= 0.0);
        assert!(s.space_10 >= 0.0);
        assert!(s.space_12 >= 0.0);
        assert!(s.space_16 >= 0.0);
    }

    #[test]
    fn test_to_css_px_zero() {
        assert_eq!(SpacingScale::to_css_px(0.0), "0");
    }

    #[test]
    fn test_to_css_px_value() {
        assert_eq!(SpacingScale::to_css_px(16.0), "16px");
    }

    #[test]
    fn test_scale_is_monotonically_increasing() {
        let s = SpacingScale::standard();
        let values = [
            s.space_0, s.space_1, s.space_2, s.space_3, s.space_4, s.space_5, s.space_6, s.space_8,
            s.space_10, s.space_12, s.space_16,
        ];
        for window in values.windows(2) {
            assert!(
                window[1] >= window[0],
                "Spacing scale must be monotonically increasing"
            );
        }
    }
}
