// @id: MUIE-ResNum @do: resource-number-atom @role: atom @layer: 6 @human: miyuk

//! Resource number display -- current/max text overlay for orbs and bars.
//!
//! Used to display life/mana values on top of orbs, or standalone.

use crate::convert::rgba_to_color32;
use egui::{Response, Ui};
use miyuki_ui_tokens::palette::d2::D2_PALETTE;

/// A resource number display.
pub struct ResourceNumber {
    /// Current value.
    pub current: i32,
    /// Maximum value.
    pub max: i32,
    /// Font size.
    pub font_size: f32,
    /// Whether to show the value in bright gold (important) or standard.
    pub bright: bool,
}

impl ResourceNumber {
    /// Create a new resource number display.
    pub fn new(current: i32, max: i32) -> Self {
        Self {
            current,
            max,
            font_size: 11.0,
            bright: false,
        }
    }

    /// Set font size.
    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    /// Use bright gold color.
    pub fn bright(mut self, bright: bool) -> Self {
        self.bright = bright;
        self
    }

    /// Draw the resource number and return the egui [`Response`].
    pub fn show(self, ui: &mut Ui) -> Response {
        let color = if self.bright {
            rgba_to_color32(&D2_PALETTE.text_high)
        } else {
            rgba_to_color32(&D2_PALETTE.accent_primary)
        };

        // Color the current value red if critically low (<25%)
        let cur_color = if self.max > 0 && self.current * 4 < self.max {
            rgba_to_color32(&D2_PALETTE.error)
        } else {
            color
        };

        let text = format!("{}/{}", self.current, self.max);
        let rich = egui::RichText::new(&text)
            .color(cur_color)
            .size(self.font_size);
        ui.label(rich)
    }
}

/// Shorthand: draw a current/max resource number.
pub fn resource_number(ui: &mut Ui, current: i32, max: i32) -> Response {
    ResourceNumber::new(current, max).show(ui)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_number_builder() {
        let rn = ResourceNumber::new(50, 100).font_size(14.0).bright(true);
        assert_eq!(rn.current, 50);
        assert_eq!(rn.max, 100);
        assert!((rn.font_size - 14.0).abs() < f32::EPSILON);
        assert!(rn.bright);
    }

    #[test]
    fn test_resource_number_default() {
        let rn = ResourceNumber::new(75, 200);
        assert!((rn.font_size - 11.0).abs() < f32::EPSILON);
        assert!(!rn.bright);
    }
}
