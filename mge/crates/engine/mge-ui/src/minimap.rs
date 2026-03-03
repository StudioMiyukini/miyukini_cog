// @id: MGE-UI-Minimap @do: minimap @role: front-end @layer: 3 @human: miyuk
//! Minimap data model and overlay (stub).
//!
//! The data model tracks player position, explored tiles, and map markers.
//! The render function is a minimal placeholder; full fog-of-war rendering
//! will be added in a later iteration.

use std::collections::HashSet;

use egui::Context;

use crate::theme::D2Colors;

// ---------------------------------------------------------------------------
// MarkerKind
// ---------------------------------------------------------------------------

/// Category of a map marker, used to choose icon and colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarkerKind {
    /// The local player character.
    Player,
    /// An enemy monster.
    Monster,
    /// A non-player character.
    Npc,
    /// A town portal or waypoint.
    Waypoint,
}

// ---------------------------------------------------------------------------
// MinimapMarker
// ---------------------------------------------------------------------------

/// A single point of interest shown on the minimap.
#[derive(Debug, Clone, PartialEq)]
pub struct MinimapMarker {
    /// World-space X position.
    pub x: f32,
    /// World-space Y position.
    pub y: f32,
    /// Marker category.
    pub kind: MarkerKind,
}

// ---------------------------------------------------------------------------
// MinimapState
// ---------------------------------------------------------------------------

/// State for the minimap: markers, explored tiles, and camera centre.
#[derive(Debug, Clone)]
pub struct MinimapState {
    /// All entities to display as blips.
    pub markers: Vec<MinimapMarker>,
    /// Set of tile coordinates `(tx, ty)` the player has visited.
    pub explored: HashSet<(i32, i32)>,
    /// World-space X coordinate shown at the minimap centre.
    pub center_x: f32,
    /// World-space Y coordinate shown at the minimap centre.
    pub center_y: f32,
}

impl MinimapState {
    /// Create an empty minimap state centred at the world origin.
    pub fn new() -> Self {
        Self {
            markers: Vec::new(),
            explored: HashSet::new(),
            center_x: 0.0,
            center_y: 0.0,
        }
    }

    /// Update the minimap centre to follow the player.
    pub fn update_center(&mut self, x: f32, y: f32) {
        self.center_x = x;
        self.center_y = y;
    }

    /// Mark tile `(tx, ty)` as explored (fog-of-war reveal).
    pub fn mark_explored(&mut self, tx: i32, ty: i32) {
        self.explored.insert((tx, ty));
    }

    /// Returns `true` when tile `(tx, ty)` has been explored.
    pub fn is_explored(&self, tx: i32, ty: i32) -> bool {
        self.explored.contains(&(tx, ty))
    }
}

impl Default for MinimapState {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minimap_player_center() {
        let mut state = MinimapState::new();
        state.update_center(42.0, -7.5);
        assert_eq!(state.center_x, 42.0);
        assert_eq!(state.center_y, -7.5);
    }

    #[test]
    fn minimap_unexplored() {
        let state = MinimapState::new();
        assert!(!state.is_explored(5, 3), "fresh state: tile (5,3) must not be explored");
    }
}

/// Draw the minimap overlay (stub).
///
/// In production this will render a corner minimap from tile data. Currently
/// shows a placeholder dark square with a label.
pub fn draw_minimap(ctx: &Context, screen_w: f32) {
    let minimap_size = 140.0;
    let margin = 8.0;
    let x = screen_w - minimap_size - margin;

    egui::Area::new("minimap".into())
        .fixed_pos(egui::Pos2::new(x, margin))
        .show(ctx, |ui| {
            let (resp, painter) = ui.allocate_painter(
                egui::Vec2::new(minimap_size, minimap_size),
                egui::Sense::click(),
            );
            let rect = resp.rect;

            // Dark background
            painter.rect_filled(
                rect,
                4.0,
                egui::Color32::from_rgba_premultiplied(10, 8, 5, 200),
            );
            painter.rect_stroke(
                rect,
                4.0,
                egui::Stroke::new(1.0, D2Colors::PANEL_BORDER),
            );

            // Placeholder label
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Minimap",
                egui::FontId::proportional(10.0),
                D2Colors::GOLD,
            );
        });
}
