//! Waypoint travel menu (toggled with W).
//!
//! Lists every waypoint, highlighting:
//! * the player's current zone in gold,
//! * activated waypoints in white,
//! * undiscovered waypoints greyed out (and unclickable).
//!
//! Clicking an activated row delegates to `systems::waypoint::waypoint_teleport`,
//! which starts a 1s interruptible cast.

use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::components::world::Waypoint;
use crate::ecs::World;
use crate::render::font;
use crate::render::vertex::{self, Color};
use crate::ui::{border, fill, panel, point_in, Rect, SUB_CONTENT, SUB_TEXT};

const ROW_H: f32 = 28.0;
const PANEL_W: f32 = 320.0;
const PANEL_H: f32 = 280.0;

/// The fixed screen rectangle the panel occupies (centred on screen).
#[must_use]
pub fn panel_rect(viewport: [f32; 2]) -> Rect {
    let x = (viewport[0] - PANEL_W) * 0.5;
    let y = (viewport[1] - PANEL_H) * 0.5;
    [x, y, PANEL_W, PANEL_H]
}

/// The row rect for waypoint index `i` inside the panel.
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn row_rect(viewport: [f32; 2], i: usize) -> Rect {
    let p = panel_rect(viewport);
    let inner_x = p[0] + 12.0;
    let inner_y = p[1] + 44.0 + i as f32 * (ROW_H + 4.0);
    [inner_x, inner_y, PANEL_W - 24.0, ROW_H]
}

/// Waypoints listed in the menu (every WP — undiscovered ones render greyed).
fn rows(world: &World) -> Vec<Waypoint> {
    world.waypoints.clone()
}

/// Activated WP id under `mouse`, if any. Greyed (unknown) rows return `None`.
#[must_use]
pub fn row_at(world: &World, viewport: [f32; 2], mouse: [f32; 2]) -> Option<u32> {
    let player = world.player_entity?;
    let known = world.discovered_wps.get(&player)?;
    for (i, wp) in rows(world).iter().enumerate() {
        if known.contains(wp.id) && point_in(row_rect(viewport, i), mouse) {
            return Some(wp.id);
        }
    }
    None
}

const GOLD: Color = [1.0, 0.84, 0.25, 1.0];
const GREY: Color = [0.45, 0.45, 0.50, 1.0];
const ROW_BG: Color = [0.10, 0.10, 0.16, 0.85];
const ROW_BG_ACTIVE: Color = [0.18, 0.16, 0.08, 0.95];

pub fn draw(batch: &mut SpriteBatch, world: &World, viewport: [f32; 2], _mouse: [f32; 2]) {
    let r = panel_rect(viewport);
    let inner = panel(batch, r, b"WAYPOINTS");

    let player = world.player_entity;
    let known = player.and_then(|p| world.discovered_wps.get(&p));
    let current_zone = world.zone;

    for (i, wp) in rows(world).iter().enumerate() {
        let row = row_rect(viewport, i);
        let is_known = known.is_some_and(|d| d.contains(wp.id));
        let is_current = wp.zone == current_zone;
        let bg = if is_current { ROW_BG_ACTIVE } else { ROW_BG };
        fill(batch, row, SUB_CONTENT, bg);
        border(batch, row, 1.0, SUB_CONTENT + 1, if is_current { GOLD } else { vertex::PANEL_BORDER });

        let label_color = if !is_known {
            GREY
        } else if is_current {
            GOLD
        } else {
            vertex::WHITE
        };
        let label = if is_known {
            format!("{}  ({})", wp.name, zone_short(wp.zone))
        } else {
            "??? (non découvert)".to_string()
        };
        font::draw_text(
            batch,
            label.as_bytes(),
            row[0] + 10.0,
            row[1] + 8.0,
            2.0,
            label_color,
            RenderLayer::UiScreen,
            0,
            SUB_TEXT,
        );
    }

    // Footer hint.
    font::draw_text(
        batch,
        b"W/ESC = close, click = travel (1s cast)",
        inner[0] + 6.0,
        inner[1] + inner[3] - 18.0,
        1.5,
        GREY,
        RenderLayer::UiScreen,
        0,
        SUB_TEXT,
    );
}

fn zone_short(zone: crate::components::world::ZoneId) -> &'static str {
    zone.name()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::player::ClassType;
    use crate::data::maps::{village_map, WP_PLAINS, WP_VILLAGE};
    use crate::data::GameData;
    use crate::plugins::player::spawn_player_system;

    #[test]
    fn click_on_unknown_row_is_ignored() {
        let mut world = World::new(village_map(), GameData::load(), 2);
        world.chosen_class = ClassType::Soldat;
        spawn_player_system(&mut world);
        let p = world.player_entity.unwrap();
        // Only Village is auto-known by default.
        world.discovered_wps.entry(p).or_default().activate(WP_VILLAGE);
        let viewport = [1280.0, 720.0];
        // Find the row for the Plains WP (second in the list, undiscovered).
        let plains_row_index = rows(&world).iter().position(|w| w.id == WP_PLAINS).unwrap();
        let row = row_rect(viewport, plains_row_index);
        let mouse = [row[0] + 4.0, row[1] + 4.0];
        assert!(super::row_at(&world, viewport, mouse).is_none());
    }

    #[test]
    fn click_on_known_row_returns_id() {
        let mut world = World::new(village_map(), GameData::load(), 2);
        world.chosen_class = ClassType::Soldat;
        spawn_player_system(&mut world);
        let p = world.player_entity.unwrap();
        world.discovered_wps.entry(p).or_default().activate(WP_VILLAGE);
        let viewport = [1280.0, 720.0];
        let village_index = rows(&world).iter().position(|w| w.id == WP_VILLAGE).unwrap();
        let row = row_rect(viewport, village_index);
        let mouse = [row[0] + 4.0, row[1] + 4.0];
        assert_eq!(super::row_at(&world, viewport, mouse), Some(WP_VILLAGE));
    }
}
