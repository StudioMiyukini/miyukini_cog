//! Isometric tilemap rendering. The sprite pipeline only draws axis-aligned
//! quads, so each diamond tile is approximated by a small stack of centred
//! horizontal bands. On top of the flat diamonds we layer cheap detailing:
//! per-tile grass hue noise + speckles, a flagstone dais on the spawn, raised
//! walls with a drop shadow and a brighter cap, and an animated portal pulse on
//! exits. Off-screen tiles are culled.

use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::components::world::{ChestTier, Decor, DecorKind, ObstacleKind, TileType, ZoneId};
use crate::components::Position2D;
use crate::ecs::World;
use crate::plugins::isometric::{TILE_H, TILE_W};

use super::camera::Camera2D;
use super::shapes;
use super::vertex::{self, with_alpha, Color};

/// Multiply a colour's RGB, keeping alpha.
fn shade(c: Color, f: f32) -> Color {
    [(c[0] * f).clamp(0.0, 1.0), (c[1] * f).clamp(0.0, 1.0), (c[2] * f).clamp(0.0, 1.0), c[3]]
}

/// Base colour for a tile type, with a subtle checkerboard on the floor.
fn tile_color(kind: TileType, x: i32, y: i32) -> Option<Color> {
    let checker = (x + y) % 2 == 0;
    match kind {
        TileType::Floor | TileType::MobSpawn => {
            Some(if checker { vertex::FLOOR } else { vertex::FLOOR_ALT })
        }
        TileType::PlayerSpawn => Some(vertex::SPAWN),
        TileType::Wall => Some(vertex::WALL),
        TileType::Exit => Some(vertex::EXIT),
        TileType::Void => None,
    }
}

/// Inclusive `(tx0, tx1, ty0, ty1)` tile range covering the camera's view,
/// clamped to the map. A few tiles of padding keep tall walls/props from
/// popping at the screen edge. Returns an empty range for a zero-size map.
#[allow(clippy::cast_possible_truncation)]
fn visible_tile_bounds(cam: &Camera2D, width: i32, height: i32) -> (i32, i32, i32, i32) {
    if width <= 0 || height <= 0 {
        return (0, -1, 0, -1);
    }
    let [vw, vh] = cam.viewport;
    let corners = [
        cam.screen_to_world(0.0, 0.0),
        cam.screen_to_world(vw, 0.0),
        cam.screen_to_world(0.0, vh),
        cam.screen_to_world(vw, vh),
    ];
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (f32::MAX, f32::MIN, f32::MAX, f32::MIN);
    for c in corners {
        min_x = min_x.min(c.x);
        max_x = max_x.max(c.x);
        min_y = min_y.min(c.y);
        max_y = max_y.max(c.y);
    }
    const PAD: i32 = 3;
    let tx0 = (min_x.floor() as i32 - PAD).clamp(0, width - 1);
    let tx1 = (max_x.ceil() as i32 + PAD).clamp(0, width - 1);
    let ty0 = (min_y.floor() as i32 - PAD).clamp(0, height - 1);
    let ty1 = (max_y.ceil() as i32 + PAD).clamp(0, height - 1);
    (tx0, tx1, ty0, ty1)
}

/// Render the whole tilemap floor (and walls) for the current camera view.
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::too_many_lines)]
pub fn draw_tilemap(batch: &mut SpriteBatch, cam: &Camera2D, world: &World) {
    let map = &world.map;
    let w = TILE_W * cam.zoom;
    let h = TILE_H * cam.zoom;
    let bands = ((h / 4.0) as usize).clamp(3, 10);

    // Bound the scan to the tiles the camera can actually see. Projecting the
    // four screen corners back to world space gives a diamond whose tile-space
    // bounding box covers every visible tile (the on-screen test below trims the
    // diamond's corners). This avoids walking the whole map every frame.
    let (tx0, tx1, ty0, ty1) = visible_tile_bounds(cam, map.width, map.height);
    // The Village reads as a settlement: a packed-earth + cobblestone ground
    // instead of the wild grass of the hunting zones.
    let in_town = matches!(world.zone, ZoneId::Village);

    for ty in ty0..=ty1 {
        for tx in tx0..=tx1 {
            let kind = map.get(tx, ty);
            let Some(base) = tile_color(kind, tx, ty) else { continue };
            let center = cam.world_to_screen(Position2D::from_tile(tx, ty));
            if !cam.on_screen(center, w) {
                continue;
            }
            let ys = ((tx + ty) as f32 * 100.0) as i32;

            match kind {
                TileType::Floor | TileType::MobSpawn => {
                    if in_town {
                        draw_town_floor(batch, center, w, h, tx, ty, ys, bands);
                    } else {
                        draw_grass(batch, center, w, h, base, tx, ty, ys, bands);
                    }
                }
                TileType::PlayerSpawn => {
                    draw_dais(batch, center, w, h, ys, bands);
                }
                TileType::Wall => {
                    draw_wall(batch, center, w, h, ys, bands);
                }
                TileType::Exit => {
                    shapes::diamond(batch, center, w, h, base, RenderLayer::Terrain, ys, 0, bands);
                    draw_portal(batch, center, w, h, ys, cam.time);
                }
                TileType::Void => {}
            }
        }
    }

    // Decor props sit between the tile and the entities.
    for d in &map.decors {
        let pos = Position2D::from_tile(d.x, d.y);
        let s = cam.world_to_screen(pos);
        if !cam.on_screen(s, w * 2.0) {
            continue;
        }
        let ys = ((d.x + d.y) as f32 * 100.0) as i32 + 10;
        draw_decor(batch, s, w, h, ys, d, cam.time);
    }

    // Procedural obstacles (live in the World resource, not on `map`, so they
    // can be touched/queried by AI without re-reading the map).
    for &(ox, oy, kind) in &world.obstacles {
        let pos = Position2D::new(ox, oy);
        let s = cam.world_to_screen(pos);
        if !cam.on_screen(s, w * 2.0) {
            continue;
        }
        let ys = ((ox + oy) * 100.0) as i32 + 12;
        draw_obstacle(batch, s, w, h, ys, kind);
    }

    // Chests on top of obstacles; pick lid colour from the chest tier.
    for (_, chest) in &world.chests {
        let pos = Position2D::new(chest.x, chest.y);
        let s = cam.world_to_screen(pos);
        if !cam.on_screen(s, w * 2.0) {
            continue;
        }
        let ys = ((chest.x + chest.y) * 100.0) as i32 + 14;
        draw_chest(batch, s, w, h, ys, chest.tier, chest.opened);
    }
}

/// Dispatch each decor to its kind-specific painter.
fn draw_decor(
    batch: &mut SpriteBatch,
    center: [f32; 2],
    w: f32,
    h: f32,
    ys: i32,
    d: &Decor,
    time: f32,
) {
    match d.kind {
        DecorKind::Fountain => draw_fountain(batch, center, w, h, ys, time),
        DecorKind::Torch => draw_torch(batch, center, w, h, ys, time),
        DecorKind::Bench => draw_bench(batch, center, w, h, ys),
        DecorKind::MarketStall => draw_market_stall(batch, center, w, h, ys, d.x as u32),
        DecorKind::Barrel => draw_barrel(batch, center, w, h, ys),
        DecorKind::Crate => draw_crate(batch, center, w, h, ys),
        DecorKind::Dummy => draw_dummy(batch, center, w, h, ys),
        DecorKind::WeaponRack => draw_weapon_rack(batch, center, w, h, ys),
        DecorKind::Flag => draw_flag(batch, center, w, h, ys, time),
        DecorKind::Altar => draw_altar(batch, center, w, h, ys),
        DecorKind::Candle => draw_candle(batch, center, w, h, ys, time),
        DecorKind::Table => draw_table(batch, center, w, h, ys),
        DecorKind::Fireplace => draw_fireplace(batch, center, w, h, ys, time),
        DecorKind::Sign => draw_sign(batch, center, w, h, ys),
        DecorKind::Tree => draw_tree(batch, center, w, h, ys, d.x, d.y),
        DecorKind::Bush => draw_bush(batch, center, w, h, ys, d.x, d.y),
        DecorKind::Flower => draw_flower(batch, center, w, h, ys, d.x, d.y),
    }
}

const STONE: Color = [0.55, 0.55, 0.58, 1.0];
const STONE_DARK: Color = [0.36, 0.36, 0.40, 1.0];
const WATER: Color = [0.25, 0.55, 0.90, 1.0];
const WATER_HL: Color = [0.55, 0.80, 1.0, 1.0];
const WOOD: Color = [0.42, 0.28, 0.16, 1.0];
const WOOD_LIGHT: Color = [0.58, 0.40, 0.22, 1.0];
const CLOTH_RED: Color = [0.78, 0.22, 0.20, 1.0];
const CLOTH_BLUE: Color = [0.25, 0.40, 0.78, 1.0];
const CLOTH_GREEN: Color = [0.30, 0.62, 0.32, 1.0];
const CLOTH_YELLOW: Color = [0.90, 0.78, 0.30, 1.0];
const GOLD: Color = [0.95, 0.82, 0.30, 1.0];
const FIRE_HOT: Color = [1.0, 0.55, 0.12, 1.0];
const FIRE_CORE: Color = [1.0, 0.92, 0.55, 1.0];
const LEAF: Color = [0.17, 0.33, 0.16, 1.0];
const LEAF_DARK: Color = [0.11, 0.23, 0.11, 1.0];
const LEAF_LIGHT: Color = [0.30, 0.46, 0.22, 1.0];
const BUSH: Color = [0.20, 0.36, 0.18, 1.0];
const BARK: Color = [0.32, 0.22, 0.13, 1.0];
const BARK_DARK: Color = [0.22, 0.15, 0.09, 1.0];

fn draw_fountain(batch: &mut SpriteBatch, c: [f32; 2], w: f32, h: f32, ys: i32, time: f32) {
    let pulse = 0.5 + 0.5 * (time * 2.4).sin();
    // Stone rim.
    shapes::ellipse(batch, c[0], c[1], w * 0.42, h * 0.42, RenderLayer::Decals, ys, 0, STONE);
    shapes::ellipse(batch, c[0], c[1], w * 0.34, h * 0.34, RenderLayer::Decals, ys, 1, STONE_DARK);
    // Water disc + highlight.
    shapes::ellipse(batch, c[0], c[1], w * 0.28, h * 0.28, RenderLayer::Decals, ys, 2, WATER);
    shapes::ellipse(batch, c[0] - w * 0.05, c[1] - h * 0.05, w * 0.10, h * 0.08, RenderLayer::Decals, ys, 3, WATER_HL);
    // Central pillar + spout, faintly pulsing.
    let pillar_h = h * 0.55;
    shapes::rect(batch, c[0] - w * 0.06, c[1] - pillar_h, w * 0.12, pillar_h, RenderLayer::Decals, ys, 4, STONE);
    shapes::circle(batch, c[0], c[1] - pillar_h, w * 0.09, RenderLayer::Decals, ys, 5, STONE_DARK);
    let plume = h * (0.18 + 0.08 * pulse);
    shapes::ellipse(batch, c[0], c[1] - pillar_h - plume * 0.6, w * 0.07, plume, RenderLayer::Decals, ys, 6, with_alpha(WATER_HL, 0.7));
}

fn draw_torch(batch: &mut SpriteBatch, c: [f32; 2], w: f32, h: f32, ys: i32, time: f32) {
    let pulse = 0.5 + 0.5 * (time * 6.0).sin();
    // Pole.
    shapes::rect(batch, c[0] - w * 0.025, c[1] - h * 0.5, w * 0.05, h * 0.5, RenderLayer::Decals, ys, 0, WOOD);
    // Flame.
    let fy = c[1] - h * 0.55;
    shapes::circle(batch, c[0], fy, w * (0.10 + 0.02 * pulse), RenderLayer::Decals, ys, 1, FIRE_HOT);
    shapes::circle(batch, c[0], fy - h * 0.04, w * 0.06, RenderLayer::Decals, ys, 2, FIRE_CORE);
    // Glow.
    shapes::circle(batch, c[0], fy, w * 0.22, RenderLayer::Decals, ys, 3, with_alpha([1.0, 0.65, 0.25, 1.0], 0.18));
}

fn draw_bench(batch: &mut SpriteBatch, c: [f32; 2], w: f32, _h: f32, ys: i32) {
    shapes::rounded_rect(batch, c[0] - w * 0.30, c[1] - 4.0, w * 0.60, 8.0, 3.0, RenderLayer::Decals, ys, 0, WOOD_LIGHT);
    shapes::rect(batch, c[0] - w * 0.27, c[1] + 3.0, 4.0, 6.0, RenderLayer::Decals, ys, 1, WOOD);
    shapes::rect(batch, c[0] + w * 0.21, c[1] + 3.0, 4.0, 6.0, RenderLayer::Decals, ys, 1, WOOD);
}

fn draw_market_stall(batch: &mut SpriteBatch, c: [f32; 2], w: f32, h: f32, ys: i32, salt: u32) {
    // Counter.
    shapes::rect(batch, c[0] - w * 0.35, c[1] - h * 0.05, w * 0.70, h * 0.18, RenderLayer::Decals, ys, 0, WOOD);
    // Awning posts.
    shapes::rect(batch, c[0] - w * 0.35, c[1] - h * 0.55, 3.0, h * 0.50, RenderLayer::Decals, ys, 1, WOOD);
    shapes::rect(batch, c[0] + w * 0.32, c[1] - h * 0.55, 3.0, h * 0.50, RenderLayer::Decals, ys, 1, WOOD);
    // Coloured cloth awning (varies per stall via salt).
    let cloth = match salt % 4 {
        0 => CLOTH_RED,
        1 => CLOTH_BLUE,
        2 => CLOTH_GREEN,
        _ => CLOTH_YELLOW,
    };
    shapes::tri_down(batch, c[0], c[1] - h * 0.55, w * 0.40, h * 0.22, RenderLayer::Decals, ys, 2, cloth);
}

fn draw_barrel(batch: &mut SpriteBatch, c: [f32; 2], w: f32, h: f32, ys: i32) {
    shapes::ellipse(batch, c[0], c[1] + h * 0.05, w * 0.18, h * 0.18, RenderLayer::Decals, ys, 0, WOOD);
    shapes::ellipse(batch, c[0], c[1] - h * 0.05, w * 0.18, h * 0.18, RenderLayer::Decals, ys, 1, WOOD_LIGHT);
    shapes::line(batch, [c[0] - w * 0.17, c[1]], [c[0] + w * 0.17, c[1]], 1.5, RenderLayer::Decals, ys, 2, WOOD);
}

fn draw_crate(batch: &mut SpriteBatch, c: [f32; 2], w: f32, h: f32, ys: i32) {
    shapes::rect(batch, c[0] - w * 0.18, c[1] - h * 0.12, w * 0.36, h * 0.24, RenderLayer::Decals, ys, 0, WOOD_LIGHT);
    shapes::line(batch, [c[0] - w * 0.18, c[1]], [c[0] + w * 0.18, c[1]], 1.0, RenderLayer::Decals, ys, 1, WOOD);
    shapes::line(batch, [c[0], c[1] - h * 0.12], [c[0], c[1] + h * 0.12], 1.0, RenderLayer::Decals, ys, 1, WOOD);
}

fn draw_dummy(batch: &mut SpriteBatch, c: [f32; 2], w: f32, h: f32, ys: i32) {
    // Vertical post + crossbar + straw head.
    shapes::rect(batch, c[0] - 2.0, c[1] - h * 0.45, 4.0, h * 0.45, RenderLayer::Decals, ys, 0, WOOD);
    shapes::rect(batch, c[0] - w * 0.18, c[1] - h * 0.30, w * 0.36, 4.0, RenderLayer::Decals, ys, 1, WOOD);
    shapes::circle(batch, c[0], c[1] - h * 0.42, w * 0.09, RenderLayer::Decals, ys, 2, [0.78, 0.65, 0.30, 1.0]);
}

fn draw_weapon_rack(batch: &mut SpriteBatch, c: [f32; 2], w: f32, h: f32, ys: i32) {
    shapes::rect(batch, c[0] - w * 0.22, c[1] + h * 0.05, w * 0.44, 4.0, RenderLayer::Decals, ys, 0, WOOD);
    for i in 0..4 {
        let dx = -w * 0.18 + (i as f32) * w * 0.12;
        shapes::rect(batch, c[0] + dx, c[1] - h * 0.35, 2.5, h * 0.40, RenderLayer::Decals, ys, 1, [0.62, 0.62, 0.66, 1.0]);
    }
}

fn draw_flag(batch: &mut SpriteBatch, c: [f32; 2], w: f32, h: f32, ys: i32, time: f32) {
    let sway = (time * 2.0).sin() * w * 0.04;
    shapes::rect(batch, c[0] - 1.5, c[1] - h * 0.70, 3.0, h * 0.70, RenderLayer::Decals, ys, 0, WOOD);
    shapes::tri_down(batch, c[0] + w * 0.10 + sway, c[1] - h * 0.65, w * 0.14, h * 0.16, RenderLayer::Decals, ys, 1, CLOTH_RED);
}

fn draw_altar(batch: &mut SpriteBatch, c: [f32; 2], w: f32, h: f32, ys: i32) {
    shapes::rounded_rect(batch, c[0] - w * 0.32, c[1] - h * 0.14, w * 0.64, h * 0.30, 3.0, RenderLayer::Decals, ys, 0, [0.92, 0.88, 0.74, 1.0]);
    shapes::rect(batch, c[0] - w * 0.20, c[1] - h * 0.26, w * 0.40, h * 0.12, RenderLayer::Decals, ys, 1, GOLD);
    shapes::circle(batch, c[0], c[1] - h * 0.10, w * 0.06, RenderLayer::Decals, ys, 2, FIRE_CORE);
}

fn draw_candle(batch: &mut SpriteBatch, c: [f32; 2], w: f32, _h: f32, ys: i32, time: f32) {
    let pulse = 0.6 + 0.4 * (time * 5.0 + (c[0] * 0.13).sin()).sin();
    shapes::rect(batch, c[0] - 1.5, c[1] - 6.0, 3.0, 8.0, RenderLayer::Decals, ys, 0, [0.95, 0.92, 0.72, 1.0]);
    shapes::circle(batch, c[0], c[1] - 8.0, w * 0.05 * pulse.max(0.6), RenderLayer::Decals, ys, 1, FIRE_CORE);
}

fn draw_table(batch: &mut SpriteBatch, c: [f32; 2], w: f32, h: f32, ys: i32) {
    shapes::rounded_rect(batch, c[0] - w * 0.28, c[1] - h * 0.10, w * 0.56, h * 0.24, 3.0, RenderLayer::Decals, ys, 0, WOOD_LIGHT);
    // Tankards as small specks.
    for (dx, dy) in [(-0.16, -0.04), (0.05, 0.00), (0.18, 0.04)] {
        shapes::circle(batch, c[0] + w * dx, c[1] + h * dy, 2.5, RenderLayer::Decals, ys, 1, [0.68, 0.60, 0.30, 1.0]);
    }
}

fn draw_fireplace(batch: &mut SpriteBatch, c: [f32; 2], w: f32, h: f32, ys: i32, time: f32) {
    let pulse = 0.5 + 0.5 * (time * 4.0).sin();
    shapes::rect(batch, c[0] - w * 0.28, c[1] - h * 0.35, w * 0.56, h * 0.40, RenderLayer::Decals, ys, 0, STONE_DARK);
    shapes::rect(batch, c[0] - w * 0.20, c[1] - h * 0.08, w * 0.40, h * 0.16, RenderLayer::Decals, ys, 1, FIRE_HOT);
    shapes::ellipse(batch, c[0], c[1] - h * 0.10, w * (0.18 + 0.04 * pulse), h * (0.10 + 0.04 * pulse), RenderLayer::Decals, ys, 2, FIRE_CORE);
}

fn draw_sign(batch: &mut SpriteBatch, c: [f32; 2], w: f32, h: f32, ys: i32) {
    shapes::rect(batch, c[0] - 1.5, c[1] - h * 0.30, 3.0, h * 0.30, RenderLayer::Decals, ys, 0, WOOD);
    shapes::rect(batch, c[0] - w * 0.16, c[1] - h * 0.42, w * 0.32, h * 0.18, RenderLayer::Decals, ys, 1, WOOD_LIGHT);
}

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
fn draw_tree(batch: &mut SpriteBatch, c: [f32; 2], w: f32, h: f32, ys: i32, tx: i32, ty: i32) {
    let n = shapes::tile_noise(tx, ty, 7);
    let size = 0.90 + n * 0.35;
    // Ground shadow grounds the trunk.
    shapes::ellipse(batch, c[0], c[1] + h * 0.06, w * 0.26 * size, h * 0.09 * size, RenderLayer::Decals, ys, 0, [0.0, 0.0, 0.0, 0.28]);
    // Trunk with a shaded inner side.
    let trunk_h = h * 0.34;
    shapes::rect(batch, c[0] - w * 0.045, c[1] - trunk_h + h * 0.06, w * 0.09, trunk_h, RenderLayer::Decals, ys, 1, BARK);
    shapes::rect(batch, c[0] + w * 0.005, c[1] - trunk_h + h * 0.06, w * 0.04, trunk_h, RenderLayer::Decals, ys, 2, BARK_DARK);
    // Layered crown: dark base mass, mid leaves, lit highlights.
    let cy = c[1] - h * 0.28;
    shapes::circle(batch, c[0], cy + h * 0.05, w * 0.34 * size, RenderLayer::Decals, ys, 3, LEAF_DARK);
    shapes::circle(batch, c[0] - w * 0.14, cy, w * 0.22 * size, RenderLayer::Decals, ys, 4, LEAF);
    shapes::circle(batch, c[0] + w * 0.15, cy + h * 0.01, w * 0.21 * size, RenderLayer::Decals, ys, 4, LEAF);
    shapes::circle(batch, c[0], cy - h * 0.10, w * 0.20 * size, RenderLayer::Decals, ys, 5, LEAF);
    shapes::circle(batch, c[0] - w * 0.10, cy - h * 0.12, w * 0.11 * size, RenderLayer::Decals, ys, 6, LEAF_LIGHT);
    shapes::circle(batch, c[0] + w * 0.08, cy - h * 0.06, w * 0.09 * size, RenderLayer::Decals, ys, 6, LEAF_LIGHT);
}

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
fn draw_bush(batch: &mut SpriteBatch, c: [f32; 2], w: f32, h: f32, ys: i32, tx: i32, ty: i32) {
    let n = shapes::tile_noise(tx, ty, 9);
    let size = 0.85 + n * 0.40;
    // Shadow + a dense clump of leafy mounds with a couple of lit tufts.
    shapes::ellipse(batch, c[0], c[1] + h * 0.06, w * 0.24 * size, h * 0.07 * size, RenderLayer::Decals, ys, 0, [0.0, 0.0, 0.0, 0.22]);
    shapes::ellipse(batch, c[0], c[1], w * 0.24 * size, h * 0.16 * size, RenderLayer::Decals, ys, 1, LEAF_DARK);
    shapes::ellipse(batch, c[0] - w * 0.12, c[1] - h * 0.01, w * 0.13 * size, h * 0.10 * size, RenderLayer::Decals, ys, 2, BUSH);
    shapes::ellipse(batch, c[0] + w * 0.11, c[1] - h * 0.02, w * 0.12 * size, h * 0.09 * size, RenderLayer::Decals, ys, 2, BUSH);
    shapes::ellipse(batch, c[0] - w * 0.05, c[1] - h * 0.05, w * 0.09 * size, h * 0.06 * size, RenderLayer::Decals, ys, 3, LEAF_LIGHT);
    shapes::ellipse(batch, c[0] + w * 0.06, c[1] - h * 0.04, w * 0.07 * size, h * 0.05 * size, RenderLayer::Decals, ys, 3, LEAF_LIGHT);
}

#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
fn draw_flower(batch: &mut SpriteBatch, c: [f32; 2], w: f32, _h: f32, ys: i32, tx: i32, ty: i32) {
    let n = shapes::tile_noise(tx, ty, 11);
    let petal = if n < 0.33 {
        [0.92, 0.30, 0.28, 1.0]
    } else if n < 0.66 {
        [0.95, 0.88, 0.32, 1.0]
    } else {
        [0.72, 0.40, 0.92, 1.0]
    };
    // Stem.
    shapes::rect(batch, c[0] - 0.8, c[1] - 4.0, 1.6, 6.0, RenderLayer::Decals, ys, 0, LEAF);
    // Petal cluster.
    shapes::circle(batch, c[0], c[1] - 5.0, w * 0.05, RenderLayer::Decals, ys, 1, petal);
    shapes::circle(batch, c[0], c[1] - 5.0, w * 0.025, RenderLayer::Decals, ys, 2, FIRE_CORE);
}

/// A grass tile: hue-varied diamond plus a couple of darker speckles.
#[allow(clippy::too_many_arguments, clippy::cast_precision_loss)]
fn draw_grass(
    batch: &mut SpriteBatch,
    center: [f32; 2],
    w: f32,
    h: f32,
    base: Color,
    tx: i32,
    ty: i32,
    ys: i32,
    bands: usize,
) {
    let n = shapes::tile_noise(tx, ty, 1);
    // Some tiles are drier/browner (worn earth amid the grass) for a D2-style
    // mottled wilderness rather than a uniform green lawn.
    let dry = shapes::tile_noise(tx, ty, 41);
    let f = 0.88 + 0.22 * n;
    let varied = if dry > 0.70 {
        // Shift toward khaki/brown for a dry patch.
        let g = 0.80 + 0.30 * n;
        [
            (base[0] * (g + 0.22)).clamp(0.0, 1.0),
            (base[1] * (g + 0.02)).clamp(0.0, 1.0),
            (base[2] * (g - 0.10)).clamp(0.0, 1.0),
            1.0,
        ]
    } else {
        [(base[0] * f).clamp(0.0, 1.0), (base[1] * (f + 0.04)).clamp(0.0, 1.0), (base[2] * f).clamp(0.0, 1.0), 1.0]
    };
    shapes::diamond(batch, center, w, h, varied, RenderLayer::Terrain, ys, 0, bands);

    // Bare-earth patch: a cluster of brown blotches on the worn tiles, like the
    // trampled dirt that breaks up D2's outdoor grass.
    if shapes::tile_noise(tx, ty, 40) > 0.80 {
        let dirt = [0.34, 0.27, 0.17, 1.0];
        for salt in [42u32, 43, 44] {
            let dx = (shapes::tile_noise(tx, ty, salt) - 0.5) * w * 0.40;
            let dy = (shapes::tile_noise(tx, ty, salt + 5) - 0.5) * h * 0.40;
            let r = w * (0.12 + 0.08 * shapes::tile_noise(tx, ty, salt + 9));
            shapes::ellipse(batch, center[0] + dx, center[1] + dy, r, r * 0.6, RenderLayer::Terrain, ys, 1, shade(dirt, 0.85 + 0.3 * n));
        }
    }

    // Scattered upright grass blades: thin tufts, alternating lighter and darker
    // green, give the ground a fine texture that holds up next to the larger
    // figures (replacing the old pair of flat blotches).
    let blade_w = (w * 0.026).max(1.0);
    for i in 0..6u32 {
        let nx = shapes::tile_noise(tx, ty, 4 + i);
        if nx < 0.34 {
            continue;
        }
        let ny = shapes::tile_noise(tx, ty, 14 + i);
        let nz = shapes::tile_noise(tx, ty, 24 + i);
        let ox = (nx - 0.5) * w * 0.52;
        let oy = (ny - 0.5) * h * 0.52;
        let bh = h * (0.10 + 0.12 * nz);
        // Front-lit blades brighten, shaded ones recede — reads as depth.
        let col = shade(varied, if nz < 0.5 { 1.20 } else { 0.74 });
        shapes::rect(
            batch,
            center[0] + ox - blade_w * 0.5,
            center[1] + oy - bh,
            blade_w,
            bh,
            RenderLayer::Decals,
            ys,
            1,
            col,
        );
    }

    // A rare wildflower: a small warm dot on a stem.
    let fl = shapes::tile_noise(tx, ty, 33);
    if fl > 0.92 {
        let fx = center[0] + (shapes::tile_noise(tx, ty, 34) - 0.5) * w * 0.42;
        let fy = center[1] + (shapes::tile_noise(tx, ty, 35) - 0.5) * h * 0.42;
        let petal = if fl > 0.965 { [0.96, 0.86, 0.36, 1.0] } else { [0.93, 0.70, 0.86, 1.0] };
        shapes::rect(batch, fx - blade_w * 0.5, fy - blade_w, blade_w, blade_w * 2.2, RenderLayer::Decals, ys, 2, shade(varied, 0.7));
        shapes::rect(batch, fx - blade_w, fy - blade_w * 2.0, blade_w * 2.0, blade_w * 1.6, RenderLayer::Decals, ys, 2, petal);
    }
}

/// A Village ground tile: packed earth with scattered cobblestones, so the town
/// reads as a settled, paved square rather than wild grass. Some tiles stay
/// mostly bare dirt, others gather a few stones, with sparse grass tufts in the
/// cracks.
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::too_many_arguments)]
fn draw_town_floor(
    batch: &mut SpriteBatch,
    center: [f32; 2],
    w: f32,
    h: f32,
    tx: i32,
    ty: i32,
    ys: i32,
    bands: usize,
) {
    let n = shapes::tile_noise(tx, ty, 1);
    // Packed-earth base, faintly mottled.
    let g = 0.86 + 0.26 * n;
    let dirt = [(0.42 * g).clamp(0.0, 1.0), (0.34 * g).clamp(0.0, 1.0), (0.24 * g).clamp(0.0, 1.0), 1.0];
    shapes::diamond(batch, center, w, h, dirt, RenderLayer::Terrain, ys, 0, bands);

    // Cobblestones: rounded grey stones with a lit cap, denser on "paved" tiles.
    let paved = shapes::tile_noise(tx, ty, 80);
    let count = if paved > 0.55 { 5 } else { 2 };
    for i in 0..count {
        let sx = shapes::tile_noise(tx, ty, 50 + i);
        if sx < 0.28 {
            continue;
        }
        let sy = shapes::tile_noise(tx, ty, 60 + i);
        let sz = shapes::tile_noise(tx, ty, 70 + i);
        let ox = (sx - 0.5) * w * 0.56;
        let oy = (sy - 0.5) * h * 0.56;
        let r = w * (0.07 + 0.05 * sz);
        let cob = if sz < 0.5 { [0.52, 0.50, 0.48, 1.0] } else { [0.45, 0.43, 0.42, 1.0] };
        shapes::ellipse(batch, center[0] + ox, center[1] + oy, r, r * 0.66, RenderLayer::Decals, ys, 1, shade(cob, 0.78));
        shapes::ellipse(batch, center[0] + ox, center[1] + oy - r * 0.18, r * 0.82, r * 0.5, RenderLayer::Decals, ys, 2, cob);
    }

    // A couple of grass tufts sprouting between the stones.
    if shapes::tile_noise(tx, ty, 90) > 0.62 {
        let blade_w = (w * 0.024).max(1.0);
        for i in 0..2u32 {
            let bx = (shapes::tile_noise(tx, ty, 92 + i) - 0.5) * w * 0.45;
            let by = (shapes::tile_noise(tx, ty, 94 + i) - 0.5) * h * 0.45;
            let bh = h * 0.10;
            shapes::rect(batch, center[0] + bx, center[1] + by - bh, blade_w, bh, RenderLayer::Decals, ys, 3, [0.26, 0.34, 0.18, 1.0]);
        }
    }
}

/// The flagstone dais on the player spawn: grey diamond with joint lines.
fn draw_dais(batch: &mut SpriteBatch, center: [f32; 2], w: f32, h: f32, ys: i32, bands: usize) {
    let stone = [0.42, 0.42, 0.46, 1.0];
    shapes::diamond(batch, center, w, h, stone, RenderLayer::Terrain, ys, 0, bands);
    shapes::diamond(batch, center, w * 0.78, h * 0.78, shade(stone, 1.12), RenderLayer::Decals, ys, 1, bands.min(6));
    // Cross joint lines (flagstone seams).
    shapes::line(batch, [center[0] - w * 0.45, center[1]], [center[0] + w * 0.45, center[1]], 1.5, RenderLayer::Decals, ys, 2, shade(stone, 0.6));
    shapes::line(batch, [center[0], center[1] - h * 0.45], [center[0], center[1] + h * 0.45], 1.5, RenderLayer::Decals, ys, 2, shade(stone, 0.6));
}

/// A raised wall: drop shadow, dark front face, and a brighter top cap.
fn draw_wall(batch: &mut SpriteBatch, center: [f32; 2], w: f32, h: f32, ys: i32, bands: usize) {
    let lift = h * 0.62;
    // Drop shadow on the ground behind/under the block.
    shapes::diamond(batch, [center[0], center[1] + h * 0.18], w, h, [0.0, 0.0, 0.0, 0.32], RenderLayer::Decals, ys, 0, bands);
    // Footprint.
    shapes::diamond(batch, center, w, h, shade(vertex::WALL, 0.7), RenderLayer::Terrain, ys, 0, bands);
    // Front face (vertical extrusion) with a couple of mortar joints.
    let face = shade(vertex::WALL, 0.85);
    shapes::rect(batch, center[0] - w * 0.46, center[1] - lift, w * 0.92, lift + h * 0.1, RenderLayer::Decals, ys, 1, face);
    for i in 1..3 {
        let jy = center[1] - lift + lift * (i as f32 / 3.0);
        shapes::rect(batch, center[0] - w * 0.46, jy, w * 0.92, 1.5, RenderLayer::Decals, ys, 2, shade(vertex::WALL, 0.55));
    }
    // Bright top cap.
    shapes::diamond(batch, [center[0], center[1] - lift], w, h, shade(vertex::WALL, 1.25), RenderLayer::Decals, ys, 3, bands);
}

/// Animated portal glow on an exit tile.
fn draw_portal(batch: &mut SpriteBatch, center: [f32; 2], w: f32, h: f32, ys: i32, time: f32) {
    let pulse = 0.5 + 0.5 * (time * 2.2).sin();
    let cx = center[0];
    let cy = center[1] - h * 0.28;
    let glow = vertex::lerp([0.35, 0.45, 0.98, 1.0], [0.65, 0.30, 0.95, 1.0], pulse);
    // Outer halo, swirling body, bright core.
    shapes::ellipse(batch, cx, cy, w * 0.34 + pulse * 3.0, h * 0.7 + pulse * 3.0, RenderLayer::Decals, ys, 4, with_alpha(glow, 0.25));
    shapes::ring(batch, cx, cy, w * 0.30, 2.5, RenderLayer::Decals, ys, 5, with_alpha([0.8, 0.9, 1.0, 1.0], 0.7));
    shapes::ellipse(batch, cx, cy, w * 0.20, h * 0.46, RenderLayer::Decals, ys, 6, with_alpha(glow, 0.6));
    shapes::ellipse(batch, cx, cy, w * 0.09, h * 0.22, RenderLayer::Decals, ys, 7, with_alpha(vertex::WHITE, 0.5 + 0.4 * pulse));
}

const ROCK: Color = [0.55, 0.55, 0.58, 1.0];
const ROCK_DARK: Color = [0.35, 0.35, 0.38, 1.0];
const DEAD_WOOD: Color = [0.20, 0.13, 0.10, 1.0];
const THORN: Color = [0.16, 0.30, 0.16, 1.0];
const RUINS: Color = [0.62, 0.58, 0.50, 1.0];
const CHEST_BRONZE: Color = [0.55, 0.36, 0.18, 1.0];
const CHEST_SILVER: Color = [0.78, 0.80, 0.85, 1.0];
const CHEST_GOLD: Color = [0.96, 0.78, 0.20, 1.0];
const CHEST_DARK: Color = [0.25, 0.16, 0.08, 1.0];

#[allow(clippy::cast_precision_loss)]
fn draw_obstacle(
    batch: &mut SpriteBatch,
    c: [f32; 2],
    w: f32,
    h: f32,
    ys: i32,
    kind: ObstacleKind,
) {
    use mge_render::batch::RenderLayer;
    match kind {
        ObstacleKind::Rock => {
            shapes::ellipse(batch, c[0], c[1] + h * 0.05, w * 0.22, h * 0.16, RenderLayer::Decals, ys, 0, ROCK_DARK);
            shapes::ellipse(batch, c[0], c[1] - h * 0.02, w * 0.18, h * 0.13, RenderLayer::Decals, ys, 1, ROCK);
        }
        ObstacleKind::BoulderCluster => {
            for (dx, dy, r) in [(-0.18, 0.04, 0.20), (0.16, 0.10, 0.22), (0.02, -0.10, 0.18)] {
                shapes::ellipse(batch, c[0] + w * dx, c[1] + h * dy, w * r, h * r * 0.72, RenderLayer::Decals, ys, 0, ROCK_DARK);
                shapes::ellipse(batch, c[0] + w * dx, c[1] + h * (dy - 0.04), w * (r - 0.04), h * (r - 0.04) * 0.72, RenderLayer::Decals, ys, 1, ROCK);
            }
        }
        ObstacleKind::DeadTree => {
            shapes::rect(batch, c[0] - 2.0, c[1] - h * 0.45, 4.0, h * 0.45, RenderLayer::Decals, ys, 0, DEAD_WOOD);
            // A couple of broken branches.
            shapes::line(batch, [c[0], c[1] - h * 0.35], [c[0] - w * 0.18, c[1] - h * 0.52], 2.0, RenderLayer::Decals, ys, 1, DEAD_WOOD);
            shapes::line(batch, [c[0], c[1] - h * 0.28], [c[0] + w * 0.20, c[1] - h * 0.46], 2.0, RenderLayer::Decals, ys, 1, DEAD_WOOD);
        }
        ObstacleKind::Pond => {
            shapes::ellipse(batch, c[0], c[1] + h * 0.02, w * 0.50, h * 0.32, RenderLayer::Decals, ys, 0, [0.18, 0.32, 0.55, 1.0]);
            shapes::ellipse(batch, c[0], c[1] - h * 0.02, w * 0.40, h * 0.24, RenderLayer::Decals, ys, 1, [0.25, 0.55, 0.90, 1.0]);
            shapes::ellipse(batch, c[0] - w * 0.10, c[1] - h * 0.06, w * 0.14, h * 0.06, RenderLayer::Decals, ys, 2, with_alpha([0.85, 0.95, 1.0, 1.0], 0.6));
        }
        ObstacleKind::Ruins => {
            shapes::rect(batch, c[0] - w * 0.45, c[1] - h * 0.10, w * 0.90, h * 0.22, RenderLayer::Decals, ys, 0, RUINS);
            // Crumbling top.
            shapes::rect(batch, c[0] - w * 0.40, c[1] - h * 0.20, w * 0.18, h * 0.10, RenderLayer::Decals, ys, 1, RUINS);
            shapes::rect(batch, c[0] + w * 0.10, c[1] - h * 0.20, w * 0.20, h * 0.10, RenderLayer::Decals, ys, 1, RUINS);
        }
        ObstacleKind::Thornbush => {
            shapes::ellipse(batch, c[0], c[1] + h * 0.02, w * 0.20, h * 0.14, RenderLayer::Decals, ys, 0, THORN);
            // Tiny spikes.
            for dx in [-0.12, -0.04, 0.04, 0.12] {
                shapes::line(batch, [c[0] + w * dx, c[1] - h * 0.10], [c[0] + w * dx, c[1] - h * 0.18], 1.0, RenderLayer::Decals, ys, 1, DEAD_WOOD);
            }
        }
    }
}

#[allow(clippy::cast_precision_loss)]
fn draw_chest(
    batch: &mut SpriteBatch,
    c: [f32; 2],
    w: f32,
    h: f32,
    ys: i32,
    tier: ChestTier,
    opened: bool,
) {
    use mge_render::batch::RenderLayer;
    let body = match tier {
        ChestTier::Normal => CHEST_BRONZE,
        ChestTier::Silver => CHEST_SILVER,
        ChestTier::Gold => CHEST_GOLD,
    };
    // Body.
    shapes::rect(batch, c[0] - w * 0.20, c[1] - h * 0.10, w * 0.40, h * 0.22, RenderLayer::Decals, ys, 0, CHEST_DARK);
    shapes::rect(batch, c[0] - w * 0.18, c[1] - h * 0.08, w * 0.36, h * 0.18, RenderLayer::Decals, ys, 1, body);
    // Lid: closed = dome on top, opened = tilted back band + glow.
    if opened {
        shapes::rect(batch, c[0] - w * 0.20, c[1] - h * 0.22, w * 0.40, h * 0.06, RenderLayer::Decals, ys, 2, body);
        shapes::ellipse(batch, c[0], c[1] - h * 0.04, w * 0.18, h * 0.08, RenderLayer::Decals, ys, 3, with_alpha([1.0, 0.95, 0.55, 1.0], 0.55));
    } else {
        shapes::rect(batch, c[0] - w * 0.20, c[1] - h * 0.18, w * 0.40, h * 0.10, RenderLayer::Decals, ys, 2, body);
        // Bevel & lock.
        shapes::rect(batch, c[0] - w * 0.20, c[1] - h * 0.08, w * 0.40, 1.5, RenderLayer::Decals, ys, 3, CHEST_DARK);
        shapes::rect(batch, c[0] - 2.0, c[1] - h * 0.10, 4.0, h * 0.06, RenderLayer::Decals, ys, 4, CHEST_DARK);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floor_checkerboards() {
        let a = tile_color(TileType::Floor, 0, 0);
        let b = tile_color(TileType::Floor, 1, 0);
        assert!(a.is_some() && b.is_some());
        assert_ne!(a, b);
    }

    #[test]
    fn void_is_skipped() {
        assert!(tile_color(TileType::Void, 0, 0).is_none());
    }

    #[test]
    fn visible_bounds_cover_the_centre_and_stay_in_map() {
        let cam = Camera2D::new(Position2D::from_tile(20, 20), 1.0, [800.0, 600.0]);
        let (tx0, tx1, ty0, ty1) = visible_tile_bounds(&cam, 40, 40);
        // The centred tile is always inside the visible window.
        assert!(tx0 <= 20 && 20 <= tx1, "x range {tx0}..={tx1} excludes the centre");
        assert!(ty0 <= 20 && 20 <= ty1, "y range {ty0}..={ty1} excludes the centre");
        // Bounds never leave the map.
        assert!(tx0 >= 0 && tx1 <= 39 && ty0 >= 0 && ty1 <= 39);
        // And a normal view sees only a fraction of a large map, not all of it.
        assert!((tx1 - tx0) < 39 || (ty1 - ty0) < 39, "the whole map should not be in view");
    }

    #[test]
    fn visible_bounds_of_empty_map_is_an_empty_range() {
        let cam = Camera2D::new(Position2D::from_tile(0, 0), 1.0, [800.0, 600.0]);
        let (tx0, tx1, _, _) = visible_tile_bounds(&cam, 0, 0);
        assert!(tx1 < tx0, "a zero-size map yields an empty inclusive range");
    }
}
