// @id: MUIE-ProgressBar @do: progress-bar-atom @role: atom @layer: 6 @human: miyuk

//! D2-style progress bars for XP, stamina, and loading indicators.
//!
//! Three visual variants map to the different HUD bar styles:
//! - **XP** -- thin gold bar spanning the bottom of the screen.
//! - **Stamina** -- yellow-green bar above the belt area.
//! - **Loading** -- wider gold bar for loading screens.

use crate::convert::rgba_to_color32;
use egui::{Rect, Response, Rounding, Ui, Vec2};
use miyuki_ui_tokens::palette::d2::{D2MiscColors, D2_PALETTE};

/// Visual variant for the progress bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressBarVariant {
    /// Thin gold XP bar (5px height).
    Xp,
    /// Yellow-green stamina bar (6px height).
    Stamina,
    /// Wide gold loading bar (10px height).
    Loading,
}

/// A D2-style horizontal progress bar.
pub struct D2ProgressBar {
    /// Progress value (0.0 to 1.0).
    pub value: f32,
    /// Visual variant.
    pub variant: ProgressBarVariant,
    /// Width of the bar in logical pixels.
    pub width: f32,
    /// Optional hover tooltip text.
    pub tooltip: Option<String>,
}

impl D2ProgressBar {
    /// Create a new progress bar.
    pub fn new(value: f32, variant: ProgressBarVariant) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            variant,
            width: 200.0,
            tooltip: None,
        }
    }

    /// Set the bar width.
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set an optional tooltip.
    pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Draw the progress bar and return the egui [`Response`].
    pub fn show(self, ui: &mut Ui) -> Response {
        let height = match self.variant {
            ProgressBarVariant::Xp => 5.0,
            ProgressBarVariant::Stamina => 6.0,
            ProgressBarVariant::Loading => 10.0,
        };

        let size = Vec2::new(self.width, height);
        let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
        let rect = response.rect;

        // Background
        let bg_color = rgba_to_color32(&D2_PALETTE.bg_primary);
        painter.rect_filled(rect, Rounding::ZERO, bg_color);

        // Fill color based on variant
        let fill_color = match self.variant {
            ProgressBarVariant::Stamina => rgba_to_color32(&D2MiscColors::STAMINA),
            ProgressBarVariant::Xp | ProgressBarVariant::Loading => {
                rgba_to_color32(&D2MiscColors::XP_BAR)
            }
        };

        // Draw fill
        if self.value > 0.0 {
            let fill_width = self.width * self.value;
            let fill_rect = Rect::from_min_size(rect.min, Vec2::new(fill_width, height));
            painter.rect_filled(fill_rect, Rounding::ZERO, fill_color);
        }

        // Border for loading variant
        if self.variant == ProgressBarVariant::Loading {
            let border_color = rgba_to_color32(&D2_PALETTE.border_default);
            painter.rect_stroke(rect, Rounding::ZERO, egui::Stroke::new(1.0, border_color));
        }

        // Tooltip
        if let Some(ref text) = self.tooltip {
            response.clone().on_hover_text(text);
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar_clamp_value() {
        let bar = D2ProgressBar::new(1.5, ProgressBarVariant::Xp);
        assert!((bar.value - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_progress_bar_clamp_negative() {
        let bar = D2ProgressBar::new(-0.5, ProgressBarVariant::Stamina);
        assert!((bar.value - 0.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_progress_bar_builder() {
        let bar = D2ProgressBar::new(0.75, ProgressBarVariant::Loading)
            .width(400.0)
            .tooltip("75%");
        assert!((bar.value - 0.75).abs() < f32::EPSILON);
        assert!((bar.width - 400.0).abs() < f32::EPSILON);
        assert_eq!(bar.tooltip, Some("75%".to_string()));
    }
}
