// @id: MUIT-ThemeD2 @do: d2-theme @role: config @layer: 5 @human: miyuk

use crate::theme::UiTheme;
use crate::palette::d2::D2_PALETTE;
use crate::spacing::SpacingScale;
use crate::typography::{FontFamily, FontSize, TypographyScale};
use crate::radius::RadiusScale;
use crate::shadow::ShadowScale;
use crate::animation::TransitionScale;
use crate::z_index::ZIndexScale;

/// Complete D2 "Sodomight Medieval" theme.
///
/// Dark, warm, gothic UI inspired by Diablo II. Gold primary accent
/// with life-red secondary accent. Serif font families (Cinzel, Georgia).
///
/// References:
/// - `BRIEF-miyuki-ui-lib-d2-analysis.md` section 2 (Fabrice)
/// - `BRIEF-miyuki-ui-lib-art-direction.md` section 3 (Lise)
pub const D2_THEME: UiTheme = UiTheme {
    name: "Sodomight Medieval (D2)",
    palette: D2_PALETTE,
    spacing: SpacingScale::standard(), // Same 4px grid
    typography: TypographyScale {
        family_ui: FontFamily::D2_BODY,
        family_display: FontFamily::D2_TITLE,
        family_mono: FontFamily::D2_MONO,
        xs:      FontSize::new(8.0, 1.4),   // D2: smaller base
        sm:      FontSize::new(10.0, 1.5),
        body:    FontSize::new(12.0, 1.6),
        lg:      FontSize::new(14.0, 1.5),
        xl:      FontSize::new(16.0, 1.4),
        xxl:     FontSize::new(24.0, 1.3),
        xxxl:    FontSize::new(32.0, 1.2),
        display: FontSize::new(48.0, 1.1),
    },
    radius: RadiusScale::d2(),
    shadow: ShadowScale::d2(),
    animation: TransitionScale::standard(),
    z_index: ZIndexScale::standard(),
};

