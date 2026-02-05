//! Atoms JayFestival — composants UI de base (Phase 2).
//!
//! Réexport uniquement ; pas de bloc métier (plan [24]).
//! Ordre : IconWrapper → Button → Input → Label → Badge → Checkbox → Select.

pub mod badge;
pub mod button;
pub mod checkbox;
pub mod icon_wrapper;
pub mod input;
pub mod label;
pub mod select;

pub use badge::{badge, BadgeVariant};
pub use button::{button, ButtonSize, ButtonVariant};
pub use checkbox::checkbox;
pub use icon_wrapper::{icon_wrapper, icon_wrapper_render, IconSize};
pub use input::input;
pub use label::{label, LabelLevel};
pub use select::select;
