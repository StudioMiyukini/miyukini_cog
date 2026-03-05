// @id: MUIE-GoldSep @do: gold-separator-atom @role: atom @layer: 6 @human: miyuk

//! Gold separator line -- D2-style horizontal divider.
//!
//! Used between sections in panels (inventory, character sheet, tooltips).
//! Optionally features a central diamond ornament.

use crate::convert::rgba_to_color32;
use egui::{Pos2, Response, Stroke, Ui, Vec2};
use miyuki_ui_tokens::palette::d2::D2_PALETTE;

/// Separator style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeparatorStyle {
    /// Simple horizontal line.
    Simple,
    /// Line with a central diamond ornament.
    Ornate,
}

/// A gold separator line.
pub struct GoldSeparator {
    /// Visual style.
    pub style: SeparatorStyle,
    /// Width in logical pixels (0 = fill available width).
    pub width: f32,
}

impl GoldSeparator {
    /// Create a simple separator.
    pub fn simple() -> Self {
        Self {
            style: SeparatorStyle::Simple,
            width: 0.0,
        }
    }

    /// Create an ornate separator with diamond.
    pub fn ornate() -> Self {
        Self {
            style: SeparatorStyle::Ornate,
            width: 0.0,
        }
    }

    /// Set explicit width (0 = auto-fill).
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Draw the separator and return the egui [`Response`].
    pub fn show(self, ui: &mut Ui) -> Response {
        let available_width = if self.width > 0.0 {
            self.width
        } else {
            ui.available_width()
        };

        let height = match self.style {
            SeparatorStyle::Simple => 3.0,
            SeparatorStyle::Ornate => 9.0,
        };

        let size = Vec2::new(available_width, height);
        let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
        let rect = response.rect;

        let gold = rgba_to_color32(&D2_PALETTE.border_strong);
        let dark_gold = rgba_to_color32(&D2_PALETTE.border_default);

        match self.style {
            SeparatorStyle::Simple => {
                // Single horizontal line
                let y = rect.center().y;
                painter.line_segment(
                    [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
                    Stroke::new(1.0, dark_gold),
                );
            }
            SeparatorStyle::Ornate => {
                let center = rect.center();
                let y = center.y;

                // Horizontal lines on each side of the diamond
                let diamond_half = 4.0;
                painter.line_segment(
                    [
                        Pos2::new(rect.min.x, y),
                        Pos2::new(center.x - diamond_half - 2.0, y),
                    ],
                    Stroke::new(1.0, dark_gold),
                );
                painter.line_segment(
                    [
                        Pos2::new(center.x + diamond_half + 2.0, y),
                        Pos2::new(rect.max.x, y),
                    ],
                    Stroke::new(1.0, dark_gold),
                );

                // Central diamond ornament
                let diamond = [
                    Pos2::new(center.x, y - diamond_half),
                    Pos2::new(center.x + diamond_half, y),
                    Pos2::new(center.x, y + diamond_half),
                    Pos2::new(center.x - diamond_half, y),
                ];
                painter.add(egui::Shape::convex_polygon(
                    diamond.to_vec(),
                    gold,
                    Stroke::new(1.0, dark_gold),
                ));
            }
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_separator_simple_default() {
        let sep = GoldSeparator::simple();
        assert_eq!(sep.style, SeparatorStyle::Simple);
        assert!((sep.width - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_separator_ornate() {
        let sep = GoldSeparator::ornate().width(200.0);
        assert_eq!(sep.style, SeparatorStyle::Ornate);
        assert!((sep.width - 200.0).abs() < f32::EPSILON);
    }
}
