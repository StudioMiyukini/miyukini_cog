// @id: MUIT-Themes @do: theme-catalog @role: exports @layer: 5 @human: miyuk

//! Pre-built theme constants.
//!
//! - [`COG_THEME`] -- Miyukini Gaming dark theme (Steam/Discord inspired).
//! - [`D2_THEME`] -- Sodomight Medieval theme (Diablo 2 inspired).

mod cog;
mod d2;

pub use cog::COG_THEME;
pub use d2::D2_THEME;
