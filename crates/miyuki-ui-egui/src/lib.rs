// @id: MUIE-Lib @do: public-api @role: exports @layer: 6 @human: miyuk

//! D2-style egui components for the Miyukini Game Engine.
//!
//! This crate provides a complete set of UI components following the Diablo II
//! visual language (dark gothic theme, gold accents, stone textures). It consumes
//! design tokens from [`miyuki_ui_tokens`] and converts them into egui styles.
//!
//! # Architecture (Atomic Design)
//!
//! - **Atoms** -- Smallest visual primitives (orbs, slots, labels, buttons).
//! - **Molecules** -- Compositions of atoms (inventory slot, stat row, tooltip line).
//! - **Organisms** -- Full panels (HUD bar, inventory panel, character sheet).
//! - **Templates** -- Screen layouts (gameplay, menu, loading).
//!
//! # Quick start
//!
//! ```rust,no_run
//! use miyuki_ui_tokens::D2_THEME;
//! use miyuki_ui_egui::theme;
//!
//! // Apply the D2 theme to your egui context:
//! // theme::apply_theme(&ctx, &D2_THEME);
//! ```

pub mod convert;
pub mod theme;
pub mod atoms;
pub mod molecules;
pub mod organisms;
pub mod templates;

// Re-exports for convenience
pub use convert::rgba_to_color32;
pub use theme::apply_theme;
