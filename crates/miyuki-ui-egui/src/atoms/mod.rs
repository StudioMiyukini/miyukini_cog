// @id: MUIE-AtomsIndex @do: atom-exports @role: exports @layer: 6 @human: miyuk

//! Atomic UI components -- smallest visual building blocks.
//!
//! Each atom is a self-contained drawing function that takes an `&mut egui::Ui`
//! and domain-specific data. Atoms have no knowledge of game state; they only
//! render what they are told.

pub mod belt_slot;
pub mod d2_button;
pub mod d2_label;
pub mod gold_separator;
pub mod item_icon;
pub mod minimap_marker;
pub mod orb;
pub mod progress_bar;
pub mod quality_text;
pub mod resource_number;
pub mod skill_icon;
pub mod slot_frame;

// Re-exports
pub use belt_slot::BeltSlot;
pub use d2_button::{D2Button, D2ButtonVariant};
pub use d2_label::D2Label;
pub use gold_separator::GoldSeparator;
pub use item_icon::ItemIcon;
pub use minimap_marker::{MarkerType, MinimapMarker};
pub use orb::{Orb, OrbType};
pub use progress_bar::{D2ProgressBar, ProgressBarVariant};
pub use quality_text::{ItemQuality, QualityText};
pub use resource_number::ResourceNumber;
pub use skill_icon::{SkillIcon, SkillState};
pub use slot_frame::{SlotFrame, SlotSize};
