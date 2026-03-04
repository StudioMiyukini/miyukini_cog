// @id: MUIE-PartyMem @do: party-member-molecule @role: molecule @layer: 6 @human: miyuk

//! Party member molecule -- name, class, level, and action buttons.

use egui::{Color32, Response, Ui};
use miyuki_ui_tokens::palette::d2::D2_PALETTE;
use crate::convert::rgba_to_color32;

/// Data for a party member.
#[derive(Debug, Clone)]
pub struct PartyMemberData {
    /// Player name.
    pub name: String,
    /// Character class.
    pub class: String,
    /// Character level.
    pub level: i32,
    /// Current life as percentage (0.0-1.0).
    pub life_pct: f32,
    /// Whether this player is hostile.
    pub hostile: bool,
}

/// A party member molecule.
pub struct PartyMember<'a> {
    /// Party member data.
    pub data: &'a PartyMemberData,
}

impl<'a> PartyMember<'a> {
    /// Create a new party member display.
    pub fn new(data: &'a PartyMemberData) -> Self {
        Self { data }
    }

    /// Draw the party member and return the egui [`Response`].
    pub fn show(self, ui: &mut Ui) -> Response {
        let name_color = if self.data.hostile {
            rgba_to_color32(&D2_PALETTE.error)
        } else {
            rgba_to_color32(&D2_PALETTE.text_primary)
        };

        let response = ui.horizontal(|ui| {
            // Name and class
            ui.vertical(|ui| {
                let name_text = format!("{} (Nv {})", self.data.name, self.data.level);
                ui.label(
                    egui::RichText::new(&name_text)
                        .color(name_color)
                        .size(11.0),
                );
                ui.label(
                    egui::RichText::new(&self.data.class)
                        .color(rgba_to_color32(&D2_PALETTE.text_secondary))
                        .size(10.0),
                );
            });

            // Simple life bar
            let bar_width = 60.0;
            let bar_height = 6.0;
            let (bar_resp, painter) = ui.allocate_painter(
                egui::Vec2::new(bar_width, bar_height),
                egui::Sense::hover(),
            );
            let bar_rect = bar_resp.rect;
            painter.rect_filled(bar_rect, 0.0, Color32::from_rgb(40, 10, 10));
            let fill_width = bar_width * self.data.life_pct.clamp(0.0, 1.0);
            let fill_rect = egui::Rect::from_min_size(
                bar_rect.min,
                egui::Vec2::new(fill_width, bar_height),
            );
            painter.rect_filled(fill_rect, 0.0, Color32::from_rgb(180, 20, 20));
        });

        response.response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_party_member_data() {
        let data = PartyMemberData {
            name: "TestPlayer".to_string(),
            class: "Sorceress".to_string(),
            level: 85,
            life_pct: 0.75,
            hostile: false,
        };
        let member = PartyMember::new(&data);
        assert_eq!(member.data.name, "TestPlayer");
        assert!(!member.data.hostile);
    }
}
