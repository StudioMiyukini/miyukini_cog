// @id MUIT-Lib
// @do public-api
// @role exports
// @layer 5
// @human miyuk

//! Design tokens agnostiques pour l'ecosysteme UI Miyukini.
//!
//! Ce crate ne depend d'aucun framework graphique. Il definit les valeurs
//! de design (couleurs, espacements, typographie, etc.) consommees par les
//! crates adaptateurs (`miyuki-ui-dioxus`).
//!
//! # Usage
//!
//! ```rust
//! use miyuki_ui_tokens::{COG_THEME, Rgba};
//!
//! let bg = COG_THEME.palette.bg_base;
//! assert_eq!(bg.to_hex(), "#0e1015");
//! ```

pub mod animation;
pub mod color;
pub mod palette;
pub mod radius;
pub mod shadow;
pub mod spacing;
pub mod theme;
pub mod themes;
pub mod typography;
pub mod z_index;

// Re-exports principaux
pub use animation::{Easing, TransitionDuration, TransitionScale};
pub use color::Rgba;
pub use palette::Palette;
pub use radius::RadiusScale;
pub use shadow::{Shadow, ShadowScale};
pub use spacing::SpacingScale;
pub use theme::UiTheme;
pub use themes::COG_THEME;
pub use typography::{FontFamily, FontSize, FontWeight, TextStyle, TypographyScale};
pub use z_index::ZIndexScale;
