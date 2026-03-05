// @id: MUIE-NpcOpt @do: npc-option-molecule @role: molecule @layer: 6 @human: miyuk

//! NPC dialog option molecule -- a clickable text option in NPC dialogs.

use crate::convert::rgba_to_color32;
use egui::{Response, Ui};
use miyuki_ui_tokens::palette::d2::D2_PALETTE;

/// An NPC dialog option.
pub struct NpcOption<'a> {
    /// Option text.
    pub text: &'a str,
    /// Whether this option is enabled.
    pub enabled: bool,
}

impl<'a> NpcOption<'a> {
    /// Create a new NPC option.
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            enabled: true,
        }
    }

    /// Set enabled state.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Draw the NPC option and return the egui [`Response`].
    pub fn show(self, ui: &mut Ui) -> Response {
        let text_color = if self.enabled {
            rgba_to_color32(&D2_PALETTE.accent_primary)
        } else {
            rgba_to_color32(&D2_PALETTE.text_muted)
        };

        let response = ui.add(
            egui::Label::new(egui::RichText::new(self.text).color(text_color).size(12.0)).sense(
                if self.enabled {
                    egui::Sense::click()
                } else {
                    egui::Sense::hover()
                },
            ),
        );

        // Hover underline effect for enabled options
        if self.enabled && response.hovered() {
            let rect = response.rect;
            let underline_y = rect.max.y;
            let gold = rgba_to_color32(&D2_PALETTE.accent_primary_hover);
            ui.painter().line_segment(
                [
                    egui::Pos2::new(rect.min.x, underline_y),
                    egui::Pos2::new(rect.max.x, underline_y),
                ],
                egui::Stroke::new(1.0, gold),
            );
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_npc_option_enabled() {
        let opt = NpcOption::new("Trade");
        assert!(opt.enabled);
    }

    #[test]
    fn test_npc_option_disabled() {
        let opt = NpcOption::new("Hire").enabled(false);
        assert!(!opt.enabled);
    }
}
