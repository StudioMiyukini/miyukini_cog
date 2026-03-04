// @id: MUIT-Lib @do: public-api @role: exports @layer: 5 @human: miyuk

//! Design tokens agnostiques pour l'ecosysteme UI Miyukini.
//!
//! Ce crate ne depend d'aucun framework graphique. Il definit les valeurs
//! de design (couleurs, espacements, typographie, etc.) consommees par les
//! crates adaptateurs (`miyuki-ui-dioxus`, `miyuki-ui-egui`).
//!
//! # Usage
//!
//! ```rust
//! use miyuki_ui_tokens::{COG_THEME, D2_THEME, Rgba};
//!
//! let bg = COG_THEME.palette.bg_base;
//! assert_eq!(bg.to_hex(), "#0e1015");
//!
//! let d2_gold = D2_THEME.palette.accent_primary;
//! let array = d2_gold.to_array(); // [200, 165, 70, 255]
//! ```

pub mod color;
pub mod spacing;
pub mod typography;
pub mod radius;
pub mod shadow;
pub mod animation;
pub mod z_index;
pub mod palette;
pub mod theme;
pub mod themes;

// Re-exports principaux
pub use color::Rgba;
pub use spacing::SpacingScale;
pub use typography::{FontFamily, FontWeight, FontSize, TypographyScale, TextStyle};
pub use radius::RadiusScale;
pub use shadow::{Shadow, ShadowScale};
pub use animation::{TransitionDuration, Easing, TransitionScale};
pub use z_index::ZIndexScale;
pub use palette::Palette;
pub use theme::UiTheme;
pub use themes::{COG_THEME, D2_THEME};
