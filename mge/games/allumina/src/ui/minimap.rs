//! Minimap: a simplified top-down view of the current map in the top-right
//! corner. Walls/floor are tinted cells; the player, mobs, and loot are dots.

use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::components::world::TileType;
use crate::ecs::World;
use crate::render::shapes;
use crate::render::vertex;

use super::{border, fill, SUB_CONTENT, SUB_PANEL};

const MAP_W: f32 = 180.0;
const MAP_H: f32 = 130.0;
const MARGIN: f32 = 14.0;

/// Draw the minimap in the top-right corner.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
pub fn draw(batch: &mut SpriteBatch, world: &World, viewport: [f32; 2]) {
    let map = &world.map;
    if map.width <= 0 || map.height <= 0 {
        return;
    }
    let ox = viewport[0] - MAP_W - MARGIN;
    let oy = MARGIN;
    let cw = MAP_W / map.width as f32;
    let ch = MAP_H / map.height as f32;

    shapes::rounded_rect(batch, ox - 4.0, oy - 4.0, MAP_W + 8.0, MAP_H + 8.0, 8.0, RenderLayer::UiScreen, 0, SUB_PANEL, [0.0, 0.0, 0.0, 0.62]);
    border(batch, [ox - 3.0, oy - 3.0, MAP_W + 6.0, MAP_H + 6.0], 2.0, SUB_PANEL + 1, vertex::PANEL_BORDER);

    for ty in 0..map.height {
        for tx in 0..map.width {
            let color = match map.get(tx, ty) {
                TileType::Wall => [0.25, 0.18, 0.10, 1.0],
                TileType::Void => continue,
                TileType::Exit => [0.3, 0.8, 0.8, 1.0],
                _ => [0.18, 0.30, 0.20, 1.0],
            };
            let x = ox + tx as f32 * cw;
            let y = oy + ty as f32 * ch;
            fill(batch, [x, y, cw + 0.5, ch + 0.5], SUB_CONTENT, color);
        }
    }

    let dot = |batch: &mut SpriteBatch, pos: crate::components::Position2D, color: vertex::Color, size: f32| {
        let x = ox + pos.x * cw;
        let y = oy + pos.y * ch;
        fill(batch, [x - size * 0.5, y - size * 0.5, size, size], SUB_CONTENT + 1, color);
    };

    // Portal/exit markers (white) from the tile grid.
    for ty in 0..map.height {
        for tx in 0..map.width {
            if map.get(tx, ty) == TileType::Exit {
                let x = ox + (tx as f32 + 0.5) * cw;
                let y = oy + (ty as f32 + 0.5) * ch;
                fill(batch, [x - 2.0, y - 2.0, 4.0, 4.0], SUB_CONTENT + 1, vertex::WHITE);
            }
        }
    }

    for e in world.ground_item_entities() {
        if let Some(&p) = world.positions.get(&e) {
            dot(batch, p, vertex::ITEM, 3.0);
        }
    }
    // NPCs (merchant) in blue.
    for e in world.npcs.keys() {
        if let Some(&p) = world.positions.get(e) {
            dot(batch, p, [0.35, 0.6, 1.0, 1.0], 4.0);
        }
    }
    for e in world.mob_entities() {
        if world.health.get(&e).is_some_and(|h| !h.is_empty()) {
            if let Some(&p) = world.positions.get(&e) {
                let boss = world
                    .mob_types
                    .get(&e)
                    .and_then(|m| world.data.mob(m.def_id))
                    .is_some_and(|d| d.is_boss);
                dot(batch, p, if boss { vertex::MOB_BOSS } else { vertex::MOB }, if boss { 6.0 } else { 4.0 });
            }
        }
    }
    if let Some(p) = world.player_entity.and_then(|e| world.positions.get(&e).copied()) {
        dot(batch, p, vertex::WHITE, 5.0);
    }
}
