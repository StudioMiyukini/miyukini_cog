// @id: MUIT-Palette @do: palette-types @role: token @layer: 5 @human: miyuk

pub mod cog;
pub mod d2;

use crate::color::Rgba;

/// Unified palette structure used by both COG and D2 themes.
///
/// The palette contains all colors organized by semantic role.
/// Framework-specific code reads from this palette to apply colors.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Palette {
    // -- Backgrounds (6 levels) --
    /// Deepest background (application base).
    pub bg_base: Rgba,
    /// Primary content background.
    pub bg_primary: Rgba,
    /// Secondary/structural background (headers, sidebars).
    pub bg_secondary: Rgba,
    /// Surface background (cards, panels).
    pub bg_surface: Rgba,
    /// Elevated surface (dropdowns, popovers).
    pub bg_elevated: Rgba,
    /// Overlay background (hover, interactive).
    pub bg_overlay: Rgba,

    // -- Text (4 levels) --
    /// Highest contrast text (titles, important).
    pub text_high: Rgba,
    /// Primary readable text.
    pub text_primary: Rgba,
    /// Secondary text (labels, descriptions).
    pub text_secondary: Rgba,
    /// Muted text (placeholders, hints, disabled).
    pub text_muted: Rgba,

    // -- Accent primary --
    /// Primary accent color (CTA, links, active).
    pub accent_primary: Rgba,
    /// Primary accent hover state.
    pub accent_primary_hover: Rgba,
    /// Primary accent active/pressed state.
    pub accent_primary_active: Rgba,
    /// Primary accent subtle (10% alpha backgrounds).
    pub accent_primary_subtle: Rgba,

    // -- Accent secondary --
    /// Secondary accent (sakura for COG, life red for D2).
    pub accent_secondary: Rgba,
    /// Secondary accent hover state.
    pub accent_secondary_hover: Rgba,
    /// Secondary accent subtle (10% alpha backgrounds).
    pub accent_secondary_subtle: Rgba,

    // -- Semantic colors --
    /// Success state.
    pub success: Rgba,
    /// Success subtle background.
    pub success_subtle: Rgba,
    /// Warning state.
    pub warning: Rgba,
    /// Warning subtle background.
    pub warning_subtle: Rgba,
    /// Error/danger state.
    pub error: Rgba,
    /// Error subtle background.
    pub error_subtle: Rgba,
    /// Information state.
    pub info: Rgba,
    /// Information subtle background.
    pub info_subtle: Rgba,

    // -- Borders (4 levels) --
    /// Subtle border (separators, grid lines).
    pub border_subtle: Rgba,
    /// Default border (cards, inputs).
    pub border_default: Rgba,
    /// Strong border (hover, focus).
    pub border_strong: Rgba,
    /// Accent border (active/selected element).
    pub border_accent: Rgba,
}
