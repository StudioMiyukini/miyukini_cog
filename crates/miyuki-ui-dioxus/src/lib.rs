// @id: MUID-Lib @do: public-api @role: exports @layer: 6 @human: miyuk

//! Dioxus 0.6 UI component library for the Miyukini ecosystem.
//!
//! Provides an Atomic Design component hierarchy (atoms, molecules, organisms,
//! templates) built on top of [`miyuki_ui_tokens`] design tokens.
//!
//! # Quick start
//!
//! ```rust,ignore
//! use miyuki_ui_dioxus::context::provide_theme;
//! use miyuki_ui_dioxus::atoms::{Button, ButtonVariant};
//! use miyuki_ui_tokens::COG_THEME;
//!
//! fn App() -> Element {
//!     provide_theme(COG_THEME.clone());
//!     rsx! {
//!         Button {
//!             variant: ButtonVariant::Primary,
//!             on_click: move |_| { /* action */ },
//!             "Click me"
//!         }
//!     }
//! }
//! ```

pub mod context;
pub mod styles;

pub mod atoms;
pub mod molecules;
pub mod organisms;
pub mod templates;

// Re-export the tokens crate for convenience.
pub use miyuki_ui_tokens;
