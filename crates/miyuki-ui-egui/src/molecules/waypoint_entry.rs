// @id: MUIE-WPEntry @do: waypoint-entry-molecule @role: molecule @layer: 6 @human: miyuk

//! Waypoint entry molecule -- name + discovered/undiscovered state.

use crate::convert::rgba_to_color32;
use egui::{Response, Ui};
use miyuki_ui_tokens::palette::d2::D2_PALETTE;

/// A waypoint entry in the waypoint map.
pub struct WaypointEntry<'a> {
    /// Waypoint zone name.
    pub name: &'a str,
    /// Whether this waypoint has been discovered.
    pub discovered: bool,
    /// Whether this is the currently active waypoint.
    pub current: bool,
}

impl<'a> WaypointEntry<'a> {
    /// Create a new waypoint entry.
    pub fn new(name: &'a str, discovered: bool) -> Self {
        Self {
            name,
            discovered,
            current: false,
        }
    }

    /// Mark this waypoint as the current location.
    pub fn current(mut self, current: bool) -> Self {
        self.current = current;
        self
    }

    /// Draw the waypoint entry and return the egui [`Response`].
    pub fn show(self, ui: &mut Ui) -> Response {
        let text_color = if self.current {
            rgba_to_color32(&D2_PALETTE.text_high)
        } else if self.discovered {
            rgba_to_color32(&D2_PALETTE.text_primary)
        } else {
            rgba_to_color32(&D2_PALETTE.text_muted)
        };

        let sense = if self.discovered {
            egui::Sense::click()
        } else {
            egui::Sense::hover()
        };

        let response = ui.add(
            egui::Label::new(egui::RichText::new(self.name).color(text_color).size(11.0))
                .sense(sense),
        );

        // Hover highlight for discovered waypoints
        if self.discovered && response.hovered() {
            let rect = response.rect;
            let highlight = egui::Color32::from_rgba_premultiplied(200, 165, 70, 20);
            ui.painter().rect_filled(rect, 0.0, highlight);
        }

        response
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waypoint_entry_discovered() {
        let wp = WaypointEntry::new("Rogue Encampment", true).current(true);
        assert!(wp.discovered);
        assert!(wp.current);
    }

    #[test]
    fn test_waypoint_entry_undiscovered() {
        let wp = WaypointEntry::new("Dark Wood", false);
        assert!(!wp.discovered);
        assert!(!wp.current);
    }
}
