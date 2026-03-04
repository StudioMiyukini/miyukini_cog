// @id: MUIE-StatRow @do: stat-row-molecule @role: molecule @layer: 6 @human: miyuk

//! Stat row molecule -- label + value + optional "+" button.
//!
//! Used in the character sheet for base attributes (Strength, Dexterity, etc.)
//! and derived stats (Defense, Attack Rating, etc.).

use egui::{Color32, Ui};
use miyuki_ui_tokens::palette::d2::D2_PALETTE;
use crate::convert::rgba_to_color32;

/// Stat row display variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatVariant {
    /// Base stat with "+" button if points available.
    Base,
    /// Derived stat (read-only).
    Derived,
    /// Bonus stat (green text).
    Bonus,
    /// Penalty stat (red text).
    Penalty,
}

/// A stat row molecule.
pub struct StatRow<'a> {
    /// Stat label (e.g., "Force", "Defense").
    pub label: &'a str,
    /// Stat value.
    pub value: i32,
    /// Display variant.
    pub variant: StatVariant,
    /// Whether the "+" button is enabled (only for `Base` variant).
    pub can_add: bool,
}

impl<'a> StatRow<'a> {
    /// Create a new base stat row.
    pub fn base(label: &'a str, value: i32) -> Self {
        Self {
            label,
            value,
            variant: StatVariant::Base,
            can_add: false,
        }
    }

    /// Create a new derived stat row.
    pub fn derived(label: &'a str, value: i32) -> Self {
        Self {
            label,
            value,
            variant: StatVariant::Derived,
            can_add: false,
        }
    }

    /// Enable the "+" button for point allocation.
    pub fn can_add(mut self, can_add: bool) -> Self {
        self.can_add = can_add;
        self
    }

    /// Set the variant.
    pub fn variant(mut self, variant: StatVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Draw the stat row and return a [`StatRowResponse`].
    pub fn show(self, ui: &mut Ui) -> StatRowResponse {
        let mut add_clicked = false;

        let label_color = rgba_to_color32(&D2_PALETTE.text_primary);
        let value_color = match self.variant {
            StatVariant::Base | StatVariant::Derived => {
                rgba_to_color32(&D2_PALETTE.accent_primary)
            }
            StatVariant::Bonus => Color32::from_rgb(0, 200, 0),
            StatVariant::Penalty => rgba_to_color32(&D2_PALETTE.error),
        };

        ui.horizontal(|ui| {
            // Label
            ui.label(
                egui::RichText::new(self.label)
                    .color(label_color)
                    .size(11.0),
            );

            // Value
            let val_text = self.value.to_string();
            ui.label(
                egui::RichText::new(&val_text)
                    .color(value_color)
                    .size(11.0),
            );

            // "+" button for base stats
            if self.variant == StatVariant::Base && self.can_add
                && ui.small_button("+").clicked()
            {
                add_clicked = true;
            }
        });

        StatRowResponse { add_clicked }
    }
}

/// Response from a stat row.
pub struct StatRowResponse {
    /// Whether the "+" button was clicked.
    pub add_clicked: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stat_row_base() {
        let row = StatRow::base("Force", 150).can_add(true);
        assert_eq!(row.label, "Force");
        assert_eq!(row.value, 150);
        assert_eq!(row.variant, StatVariant::Base);
        assert!(row.can_add);
    }

    #[test]
    fn test_stat_row_derived() {
        let row = StatRow::derived("Defense", 1200);
        assert_eq!(row.variant, StatVariant::Derived);
        assert!(!row.can_add);
    }

    #[test]
    fn test_stat_row_variant_override() {
        let row = StatRow::base("Test", 10).variant(StatVariant::Bonus);
        assert_eq!(row.variant, StatVariant::Bonus);
    }
}
