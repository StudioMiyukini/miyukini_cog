// @id: MUIT-ThemeCog @do: cog-theme @role: config @layer: 5 @human: miyuk

use crate::animation::TransitionScale;
use crate::palette::cog::COG_PALETTE;
use crate::radius::RadiusScale;
use crate::shadow::ShadowScale;
use crate::spacing::SpacingScale;
use crate::theme::UiTheme;
use crate::typography::{FontFamily, FontSize, TypographyScale};
use crate::z_index::ZIndexScale;

/// Complete COG "Miyukini Gaming" theme.
///
/// Dark, modern UI inspired by Steam and Discord. Blue primary accent
/// with sakura (pink) secondary accent. Inter font family.
///
/// Reference: `BRIEF-miyuki-ui-lib-art-direction.md` section 2 (Lise).
pub const COG_THEME: UiTheme = UiTheme {
    name: "Miyukini Gaming (COG)",
    palette: COG_PALETTE,
    spacing: SpacingScale::standard(),
    typography: TypographyScale {
        family_ui: FontFamily::COG_UI,
        family_display: FontFamily::COG_UI,
        family_mono: FontFamily::COG_MONO,
        xs: FontSize::new(10.0, 1.4),
        sm: FontSize::new(12.0, 1.5),
        body: FontSize::new(14.0, 1.6),
        lg: FontSize::new(16.0, 1.5),
        xl: FontSize::new(18.0, 1.4),
        xxl: FontSize::new(24.0, 1.3),
        xxxl: FontSize::new(32.0, 1.2),
        display: FontSize::new(40.0, 1.1),
    },
    radius: RadiusScale::cog(),
    shadow: ShadowScale::cog(),
    animation: TransitionScale::standard(),
    z_index: ZIndexScale::standard(),
};
