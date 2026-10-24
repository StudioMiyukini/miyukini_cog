// @id MUIT-Theme
// @do theme-composition
// @role token
// @layer 5
// @human miyuk

use crate::animation::TransitionScale;
use crate::palette::Palette;
use crate::radius::RadiusScale;
use crate::shadow::ShadowScale;
use crate::spacing::SpacingScale;
use crate::typography::TypographyScale;
use crate::z_index::ZIndexScale;

/// Complete UI theme composing all design token categories.
///
/// A `UiTheme` is the single source of truth for all visual decisions
/// in a given context (COG desktop, etc.).
///
/// # Usage
///
/// ```rust
/// use miyuki_ui_tokens::COG_THEME;
///
/// let theme = &COG_THEME;
/// let bg = theme.palette.bg_base.to_hex();
/// let spacing = theme.spacing.space_4; // 16.0
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct UiTheme {
    /// Human-readable theme name.
    pub name: &'static str,
    /// Color palette.
    pub palette: Palette,
    /// Spacing scale.
    pub spacing: SpacingScale,
    /// Typography scale.
    pub typography: TypographyScale,
    /// Border radius scale.
    pub radius: RadiusScale,
    /// Shadow elevation scale.
    pub shadow: ShadowScale,
    /// Transition/animation scale.
    pub animation: TransitionScale,
    /// Z-index stacking scale.
    pub z_index: ZIndexScale,
}

#[cfg(test)]
mod tests {
    use crate::themes::COG_THEME;

    #[test]
    fn test_cog_theme_has_name() {
        assert!(!COG_THEME.name.is_empty());
        assert!(COG_THEME.name.contains("COG"));
    }

    #[test]
    fn test_theme_clone() {
        let cloned = COG_THEME.clone();
        assert_eq!(cloned.name, COG_THEME.name);
        assert_eq!(cloned.palette, COG_THEME.palette);
    }
}
