use anyhow::{Context, Result};
use mge_audio::{AudioBus, AudioCue};
use mge_content::{load_bootstrap, GameBootstrap};
use mge_core::{RuntimeConfig, SceneSummary};
use mge_render::atlas::{AtlasHandle, MaterialHandle, SpriteRect};
use mge_render::batch::{RenderLayer, SortKey, SpriteBatch, SpriteInstance};
use mge_render::camera::IsoCamera;
use mge_render::GraphicsState;
use mge_replication::ReplicationPlan;
use mge_save::{PlayerProfile, SaveManager};
use mge_server_core::AuthoritativeSim;
use mge_sim::constants::*;
use mge_sim::entity::*;
// PlayerState, SpatialGrid, ZoneState will replace local fields incrementally
#[allow(unused_imports)]
use mge_sim::player_state::PlayerState;
#[allow(unused_imports)]
use mge_sim::spatial::SpatialGrid;
#[allow(unused_imports)]
use mge_sim::skill_disk::{
    self, DiskBonuses, SkillGrid, SlotContent, ScrollStat, SpecialEffect,
    GRID_COLS, GRID_ROWS, GRID_TOTAL, CENTER_INDEX,
    random_scroll_drop, random_passive_scroll,
};
use mge_sim::inventory::{Backpack, InvItem, BACKPACK_UPGRADE_COST};
use mge_sim::civil_skills::{CivilSkills, CivilSkillId, CIVIL_SKILL_COUNT};
use mge_sim::gather_node::{GatherNode, GatherNodeKind, generate_gather_nodes};
use mge_sim::world_map::{self, zone_coord};
#[allow(unused_imports)]
use mge_sim::zone_state::ZoneState;
use std::{path::PathBuf, sync::Arc};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{ElementState, MouseButton, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{KeyCode, PhysicalKey},
    window::{Window, WindowAttributes, WindowId},
};

// ---------------------------------------------------------------------------
// GPU handles
// ---------------------------------------------------------------------------
const ATLAS_0: AtlasHandle = AtlasHandle::new(0);
const MAT: MaterialHandle = MaterialHandle::new(0);
const UNIT: SpriteRect = SpriteRect { x: 0, y: 0, w: 1, h: 1 };

// ---------------------------------------------------------------------------
// Bitmap font — 5x7 pixel glyphs, row-major, bit4=left column
// Each glyph: [u8; 7] (7 rows), bits 4..0 = columns left→right
// ---------------------------------------------------------------------------
#[allow(clippy::cast_possible_truncation)]
fn glyph(ch: u8) -> [u8; 7] {
    match ch {
        // Digits
        b'0' => [0x0E,0x11,0x13,0x15,0x19,0x11,0x0E],
        b'1' => [0x04,0x0C,0x04,0x04,0x04,0x04,0x0E],
        b'2' => [0x0E,0x11,0x01,0x02,0x04,0x08,0x1F],
        b'3' => [0x0E,0x11,0x01,0x06,0x01,0x11,0x0E],
        b'4' => [0x02,0x06,0x0A,0x12,0x1F,0x02,0x02],
        b'5' => [0x1F,0x10,0x1E,0x01,0x01,0x11,0x0E],
        b'6' => [0x06,0x08,0x10,0x1E,0x11,0x11,0x0E],
        b'7' => [0x1F,0x01,0x02,0x04,0x08,0x08,0x08],
        b'8' => [0x0E,0x11,0x11,0x0E,0x11,0x11,0x0E],
        b'9' => [0x0E,0x11,0x11,0x0F,0x01,0x02,0x0C],
        // Uppercase letters
        b'A' => [0x0E,0x11,0x11,0x1F,0x11,0x11,0x11],
        b'B' => [0x1E,0x11,0x11,0x1E,0x11,0x11,0x1E],
        b'C' => [0x0E,0x11,0x10,0x10,0x10,0x11,0x0E],
        b'D' => [0x1C,0x12,0x11,0x11,0x11,0x12,0x1C],
        b'E' => [0x1F,0x10,0x10,0x1E,0x10,0x10,0x1F],
        b'F' => [0x1F,0x10,0x10,0x1E,0x10,0x10,0x10],
        b'G' => [0x0E,0x11,0x10,0x17,0x11,0x11,0x0E],
        b'H' => [0x11,0x11,0x11,0x1F,0x11,0x11,0x11],
        b'I' => [0x0E,0x04,0x04,0x04,0x04,0x04,0x0E],
        b'J' => [0x07,0x02,0x02,0x02,0x02,0x12,0x0C],
        b'K' => [0x11,0x12,0x14,0x18,0x14,0x12,0x11],
        b'L' => [0x10,0x10,0x10,0x10,0x10,0x10,0x1F],
        b'M' => [0x11,0x1B,0x15,0x15,0x11,0x11,0x11],
        b'N' => [0x11,0x11,0x19,0x15,0x13,0x11,0x11],
        b'O' => [0x0E,0x11,0x11,0x11,0x11,0x11,0x0E],
        b'P' => [0x1E,0x11,0x11,0x1E,0x10,0x10,0x10],
        b'Q' => [0x0E,0x11,0x11,0x11,0x15,0x12,0x0D],
        b'R' => [0x1E,0x11,0x11,0x1E,0x14,0x12,0x11],
        b'S' => [0x0E,0x11,0x10,0x0E,0x01,0x11,0x0E],
        b'T' => [0x1F,0x04,0x04,0x04,0x04,0x04,0x04],
        b'U' => [0x11,0x11,0x11,0x11,0x11,0x11,0x0E],
        b'V' => [0x11,0x11,0x11,0x11,0x0A,0x0A,0x04],
        b'W' => [0x11,0x11,0x11,0x15,0x15,0x15,0x0A],
        b'X' => [0x11,0x11,0x0A,0x04,0x0A,0x11,0x11],
        b'Y' => [0x11,0x11,0x0A,0x04,0x04,0x04,0x04],
        b'Z' => [0x1F,0x01,0x02,0x04,0x08,0x10,0x1F],
        // Lowercase → same as uppercase
        b'a'..=b'z' => glyph(ch - 32),
        // Punctuation
        b' ' => [0x00,0x00,0x00,0x00,0x00,0x00,0x00],
        b'!' => [0x04,0x04,0x04,0x04,0x04,0x00,0x04],
        b'.' => [0x00,0x00,0x00,0x00,0x00,0x00,0x04],
        b',' => [0x00,0x00,0x00,0x00,0x00,0x04,0x08],
        b':' => [0x00,0x00,0x04,0x00,0x00,0x04,0x00],
        b'-' => [0x00,0x00,0x00,0x1F,0x00,0x00,0x00],
        b'+' => [0x00,0x04,0x04,0x1F,0x04,0x04,0x00],
        b'/' => [0x01,0x01,0x02,0x04,0x08,0x10,0x10],
        b'%' => [0x19,0x1A,0x02,0x04,0x08,0x0B,0x13],
        b'(' => [0x02,0x04,0x08,0x08,0x08,0x04,0x02],
        b')' => [0x08,0x04,0x02,0x02,0x02,0x04,0x08],
        b'?' => [0x0E,0x11,0x01,0x02,0x04,0x00,0x04],
        b'_' => [0x00,0x00,0x00,0x00,0x00,0x00,0x1F],
        b'=' => [0x00,0x00,0x1F,0x00,0x1F,0x00,0x00],
        b'*' => [0x00,0x04,0x15,0x0E,0x15,0x04,0x00],
        b'>' => [0x08,0x04,0x02,0x01,0x02,0x04,0x08],
        b'<' => [0x02,0x04,0x08,0x10,0x08,0x04,0x02],
        b'\'' => [0x04,0x04,0x00,0x00,0x00,0x00,0x00],
        b'#' => [0x0A,0x0A,0x1F,0x0A,0x1F,0x0A,0x0A],
        b'[' => [0x0E,0x08,0x08,0x08,0x08,0x08,0x0E],
        b']' => [0x0E,0x02,0x02,0x02,0x02,0x02,0x0E],
        _ => [0x1F,0x11,0x11,0x11,0x11,0x11,0x1F], // box for unknown
    }
}

/// Format u32 into a stack buffer, returns (buf, len).
fn fmt_u32(v: u32) -> ([u8; 10], usize) {
    if v == 0 { return (*b"0\0\0\0\0\0\0\0\0\0", 1); }
    let mut buf = [0u8; 10];
    let mut n = v;
    let mut i = 10;
    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    let len = 10 - i;
    let mut out = [0u8; 10];
    out[..len].copy_from_slice(&buf[i..]);
    (out, len)
}

/// Format f32 as integer into stack buffer.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn fmt_f32(v: f32) -> ([u8; 10], usize) {
    fmt_u32(v.max(0.0) as u32)
}

// (format_effect removed — old disk node system replaced by scroll grid)

// ---------------------------------------------------------------------------
// 3/4 view tile size (ALttP style: wider than tall)
// ---------------------------------------------------------------------------
const TW: f32 = 48.0; // tile width in pixels
const TH: f32 = 36.0; // tile height in pixels (3/4 squash)

/// Offset to convert world coords to global terrain indices.
/// World ranges from (-200, -200) to (300, 300); global from (0,0) to (500,500).
const GLOBAL_OFF: f32 = CHUNK_OFFSET as f32 * MAP_W as f32; // 200.0

/// Stitch all 25 zone terrains into one continuous 500×500 global terrain.
#[allow(clippy::cast_sign_loss)]
fn stitch_global_terrain(zone_terrains: &[[[u8; MAP_W]; MAP_H]]) -> Vec<u8> {
    let mut global = vec![0u8; WORLD_W * WORLD_H];
    for (i, &zone_id) in ZoneId::ALL.iter().enumerate() {
        let coord = zone_coord(zone_id);
        let ox = ((coord.cx + CHUNK_OFFSET) as usize) * MAP_W;
        let oy = ((coord.cy + CHUNK_OFFSET) as usize) * MAP_H;
        for y in 0..MAP_H {
            for x in 0..MAP_W {
                global[(oy + y) * WORLD_W + (ox + x)] = zone_terrains[i][y][x];
            }
        }
    }
    global
}

/// Generate the Rogue Camp terrain (100×100): fully open, plaza center + dirt + grass.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation,
        clippy::needless_range_loop, clippy::cast_possible_wrap, clippy::cast_sign_loss)]
fn generate_camp_terrain() -> [[u8; MAP_W]; MAP_H] {
    let mut t = [[1u8; MAP_W]; MAP_H]; // all grass — fully open, no walls
    let cx = MAP_W as f32 / 2.0;
    let cy = MAP_H as f32 / 2.0;

    for y in 0..MAP_H {
        for x in 0..MAP_W {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist < 12.0 {
                t[y][x] = 3; // stone plaza center
            } else if dist < 28.0 {
                t[y][x] = 2; // dirt interior
            }
            // else 1 = grass (default)
        }
    }
    // Wide dirt paths to edges (N/S/E/W) — 5 tiles wide
    for i in 0..MAP_W {
        for off in -2i32..=2 {
            let c = (MAP_W / 2) as i32 + off;
            if c >= 0 && (c as usize) < MAP_W {
                if t[i][c as usize] == 1 { t[i][c as usize] = 2; }
                if t[c as usize][i] == 1 { t[c as usize][i] = 2; }
            }
        }
    }
    t
}

// Trees removed — will be entities for civil skill (woodcutting).

// ---------------------------------------------------------------------------
// Procedural terrain generation
// ---------------------------------------------------------------------------

/// Deterministic hash for procedural generation.
fn tile_hash(x: u32, y: u32, seed: u32) -> u32 {
    let mut h = x.wrapping_mul(374_761_393)
        .wrapping_add(y.wrapping_mul(668_265_263))
        .wrapping_add(seed);
    h = (h ^ (h >> 13)).wrapping_mul(1_274_126_177);
    h ^ (h >> 16)
}

/// Smooth noise value (0.0..1.0) by averaging 2×2 neighborhood hashes.
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
fn smooth_noise(x: usize, y: usize, seed: u32) -> f32 {
    let v00 = tile_hash(x as u32, y as u32, seed) as f32 / u32::MAX as f32;
    let v10 = tile_hash(x as u32 + 1, y as u32, seed) as f32 / u32::MAX as f32;
    let v01 = tile_hash(x as u32, y as u32 + 1, seed) as f32 / u32::MAX as f32;
    let v11 = tile_hash(x as u32 + 1, y as u32 + 1, seed) as f32 / u32::MAX as f32;
    (v00 + v10 + v01 + v11) * 0.25
}

/// Check if a neighbor chunk exists in the world.
#[allow(clippy::cast_precision_loss)]
fn has_neighbor(cx: i32, cy: i32, dx: i32, dy: i32) -> bool {
    world_map::zone_at_world([
        (cx + dx) as f32 * MAP_W as f32 + 12.0,
        (cy + dy) as f32 * MAP_H as f32 + 12.0,
    ]).is_some()
}

/// Find a walkable position by trying multiple hash offsets, falling back to center cross.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn find_walkable_pos(ix: u32, iy: u32, seed: u32, terrain: &[[u8; MAP_W]; MAP_H]) -> (f32, f32) {
    for attempt in 0..8u32 {
        let h = tile_hash(ix.wrapping_add(attempt * 37), iy, seed);
        let h2 = tile_hash(iy, ix.wrapping_add(attempt * 37), seed.wrapping_add(3));
        let tx = (h % (MAP_W as u32 - 4) + 2) as usize;
        let ty = (h2 % (MAP_H as u32 - 4) + 2) as usize;
        if terrain[ty][tx] != 0 {
            return (tx as f32, ty as f32);
        }
    }
    // Fallback: center of the guaranteed walkable cross
    (MAP_W as f32 / 2.0, MAP_H as f32 / 2.0)
}

/// Generate terrain for a zone based on its type.
#[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss,
        clippy::needless_range_loop, clippy::if_same_then_else)]
fn generate_terrain(zone_id: ZoneId) -> [[u8; MAP_W]; MAP_H] {
    if zone_id == ZoneId::RogueCamp {
        return generate_camp_terrain();
    }

    let coord = zone_coord(zone_id);
    let zt = zone_id.zone_type();
    let seed = zone_id as u32 * 7919 + 42;
    let mut terrain = [[0u8; MAP_W]; MAP_H];

    // Check which edges connect to other zones (keep open)
    let open_n = has_neighbor(coord.cx, coord.cy, 0, -1);
    let open_s = has_neighbor(coord.cx, coord.cy, 0, 1);
    let open_e = has_neighbor(coord.cx, coord.cy, 1, 0);
    let open_w = has_neighbor(coord.cx, coord.cy, -1, 0);

    for y in 0..MAP_H {
        for x in 0..MAP_W {
            let h = tile_hash(x as u32, y as u32, seed);
            let n = smooth_noise(x, y, seed);
            let frac = (h % 1000) as f32 / 1000.0;

            // Border handling: make edges walkable if neighbor exists, wall otherwise
            let at_edge = x == 0 || y == 0 || x == MAP_W - 1 || y == MAP_H - 1;
            let near_edge_x = x <= 1 || x >= MAP_W - 2;
            let near_edge_y = y <= 1 || y >= MAP_H - 2;
            let edge_open = (x == 0 && open_w) || (x == MAP_W - 1 && open_e)
                || (y == 0 && open_n) || (y == MAP_H - 1 && open_s)
                || (x <= 1 && open_w) || (x >= MAP_W - 2 && open_e)
                || (y <= 1 && open_n) || (y >= MAP_H - 2 && open_s);

            terrain[y][x] = match zt {
                ZoneType::Town => unreachable!(),

                ZoneType::Wilderness => {
                    if at_edge && !edge_open { 0 }
                    else if frac < 0.15 { 2 } // dirt clearing
                    else if frac < 0.03 { 3 } // occasional stone
                    else { 1 } // grass
                }

                ZoneType::Cave => {
                    // Cave: wall borders + interior pillars
                    let dist_edge = x.min(MAP_W - 1 - x).min(y).min(MAP_H - 1 - y);
                    if at_edge && !edge_open { 0 }
                    else if dist_edge <= 1 && !edge_open { 0 }
                    else if n < 0.25 && dist_edge > 3 { 0 } // interior pillars
                    else if frac < 0.08 { 3 } // ore/loot spots
                    else { 4 } // cave floor
                }

                ZoneType::Plains => {
                    if at_edge && !edge_open && near_edge_x && near_edge_y { 0 }
                    else if n < 0.20 && !near_edge_x && !near_edge_y { 3 } // stone ruins
                    else if frac < 0.08 { 2 } // dirt patches
                    else { 1 } // grass
                }

                ZoneType::Graveyard => {
                    if at_edge && !edge_open { 0 }
                    else if n < 0.18 { 3 } // tombstones/stone
                    else if frac < 0.30 { 2 } // dirt
                    else { 1 } // grass
                }

                ZoneType::Field => {
                    if at_edge && !edge_open && near_edge_x && near_edge_y { 0 }
                    else if n < 0.22 { 3 } // rocky outcrops
                    else if frac < 0.20 { 2 } // dirt
                    else { 1 } // grass
                }

                ZoneType::Forest => {
                    if at_edge && !edge_open { 0 }
                    else if frac < 0.12 { 2 } // dirt paths
                    else { 1 } // grass (trees placed separately)
                }

                ZoneType::Marsh => {
                    if at_edge && !edge_open { 0 }
                    else if n < 0.20 && !near_edge_x && !near_edge_y { 5 } // water pools
                    else if frac < 0.15 { 2 } // muddy dirt
                    else { 1 } // grass
                }

                ZoneType::Highlands => {
                    if at_edge && !edge_open { 0 }
                    else if n < 0.15 && !near_edge_x && !near_edge_y { 0 } // cliff walls
                    else if frac < 0.30 { 3 } // stone
                    else if frac < 0.45 { 2 } // dirt
                    else { 1 } // grass
                }
            };
        }
    }

    // Ensure there's always a walkable path through the center
    for y in (MAP_H / 2 - 1)..=(MAP_H / 2 + 1) {
        for x in 2..MAP_W - 2 {
            if terrain[y][x] == 0 { terrain[y][x] = 1; }
        }
    }
    for x in (MAP_W / 2 - 1)..=(MAP_W / 2 + 1) {
        for y in 2..MAP_H - 2 {
            if terrain[y][x] == 0 { terrain[y][x] = 1; }
        }
    }

    terrain
}

// generate_trees removed — trees will be entities for civil skill (woodcutting).

/// Generate enemies for a zone based on biome type and difficulty tier.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn generate_enemies(zone_id: ZoneId, terrain: &[[u8; MAP_W]; MAP_H]) -> Vec<Enemy> {
    let zt = zone_id.zone_type();
    if zt == ZoneType::Town { return vec![]; }

    let tier = zone_id.tier();
    let seed = zone_id as u32 * 6311 + 997;

    // Enemy count scales with tier (scaled for 100×100 zones)
    let base_count: usize = match zt {
        ZoneType::Cave => 80,
        ZoneType::Forest => 70,
        ZoneType::Graveyard => 65,
        ZoneType::Town => 0,
        ZoneType::Wilderness | ZoneType::Plains | ZoneType::Field
            | ZoneType::Marsh | ZoneType::Highlands => 55,
    };
    let count = base_count + (tier as usize) * 5;

    // HP multiplier per tier (+20% per tier)
    let tier_hp_mult = 1.0_f32 + 0.20 * f32::from(tier);

    // Affix pool
    let affixes = [
        MonsterAffix::ExtraStrong,
        MonsterAffix::FireEnchanted,
        MonsterAffix::ColdEnchanted,
        MonsterAffix::LightningEnchanted,
        MonsterAffix::Cursed,
    ];

    // Enemy pool per biome
    let pool: &[EnemyKind] = match zt {
        ZoneType::Wilderness => &[EnemyKind::Zombie, EnemyKind::Zombie, EnemyKind::Fallen,
                                   EnemyKind::QuillRat, EnemyKind::FallenShaman],
        ZoneType::Cave => &[EnemyKind::Zombie, EnemyKind::Skeleton, EnemyKind::Fallen,
                             EnemyKind::QuillRat, EnemyKind::FallenShaman],
        ZoneType::Plains => &[EnemyKind::Skeleton, EnemyKind::Skeleton, EnemyKind::Fallen,
                               EnemyKind::FallenShaman, EnemyKind::QuillRat],
        ZoneType::Graveyard => &[EnemyKind::Skeleton, EnemyKind::Skeleton, EnemyKind::Skeleton,
                                  EnemyKind::Zombie, EnemyKind::FallenShaman],
        ZoneType::Field => &[EnemyKind::Skeleton, EnemyKind::Fallen, EnemyKind::QuillRat,
                              EnemyKind::FallenShaman],
        ZoneType::Forest => &[EnemyKind::Fallen, EnemyKind::Fallen, EnemyKind::FallenShaman,
                               EnemyKind::QuillRat, EnemyKind::Zombie],
        ZoneType::Marsh => &[EnemyKind::Zombie, EnemyKind::Zombie, EnemyKind::QuillRat,
                              EnemyKind::Skeleton],
        ZoneType::Highlands => &[EnemyKind::Skeleton, EnemyKind::Fallen, EnemyKind::FallenShaman,
                                  EnemyKind::BloodRaven],
        ZoneType::Town => return vec![],
    };

    let mut enemies = Vec::with_capacity(count + 1);
    for i in 0..count {
        let (x, y) = find_walkable_pos(i as u32, 1, seed, terrain);
        let h = tile_hash(i as u32, 1, seed);
        let kind = pool[h as usize % pool.len()];
        let mut enemy = Enemy::new(kind, x, y);

        // Tier HP scaling
        enemy.max_hp *= tier_hp_mult;
        enemy.hp = enemy.max_hp;

        // Roll affix: chance = tier * 5% (0% at tier 0, 25% at tier 5)
        let affix_roll = (tile_hash(i as u32, 99, seed) % 100) as u8;
        if tier > 0 && affix_roll < tier * 5 {
            let affix = affixes[tile_hash(i as u32, 100, seed) as usize % affixes.len()];
            enemy.affix = Some(affix);
            enemy.max_hp *= affix.hp_mult();
            enemy.hp = enemy.max_hp;
        }

        enemies.push(enemy);
    }

    // Boss in select zones
    let has_boss = matches!(zone_id,
        ZoneId::DenOfEvil | ZoneId::BurialGrounds |
        ZoneId::DarkWoodNW | ZoneId::TamoeHighlands | ZoneId::CatacombsEntry
    );
    let has_blood_raven = matches!(zone_id, ZoneId::ColdPlainsN);

    if has_boss {
        let (bx, by) = find_walkable_pos(900, 900, seed, terrain);
        let mut boss = Enemy::new(EnemyKind::Boss, bx, by);
        boss.max_hp *= tier_hp_mult;
        boss.hp = boss.max_hp;
        enemies.push(boss);
    }
    if has_blood_raven {
        let (bx, by) = find_walkable_pos(901, 901, seed, terrain);
        let mut br = Enemy::new(EnemyKind::BloodRaven, bx, by);
        br.max_hp *= tier_hp_mult;
        br.hp = br.max_hp;
        enemies.push(br);
    }

    enemies
}

/// Generate breakable objects for a zone.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn generate_breakables(zone_id: ZoneId, terrain: &[[u8; MAP_W]; MAP_H]) -> Vec<Breakable> {
    let zt = zone_id.zone_type();
    if zt == ZoneType::Town { return vec![]; }

    let seed = zone_id as u32 * 4219 + 53;
    let count: u32 = match zt {
        ZoneType::Cave => 40,
        ZoneType::Graveyard => 30,
        _ => 20,
    };

    (0..count).map(|i| {
        let (x, y) = find_walkable_pos(i, 2, seed, terrain);
        Breakable {
            pos: [x, y],
            hp: 1.0,
            alive: true,
        }
    }).collect()
}

/// Generate shrines for a zone.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn generate_shrines(zone_id: ZoneId, terrain: &[[u8; MAP_W]; MAP_H]) -> Vec<Shrine> {
    let zt = zone_id.zone_type();
    if zt == ZoneType::Town { return vec![]; }

    let seed = zone_id as u32 * 2903 + 71;
    let kinds = [ShrineKind::Experience, ShrineKind::Damage, ShrineKind::Health];

    (0..8).map(|i| {
        let h = tile_hash(i, 3, seed);
        let (x, y) = find_walkable_pos(i, 3, seed, terrain);
        Shrine {
            pos: [x, y],
            kind: kinds[h as usize % kinds.len()],
            used: false,
        }
    }).collect()
}

/// Generate chests for a zone.
#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
fn generate_chests(zone_id: ZoneId, terrain: &[[u8; MAP_W]; MAP_H]) -> Vec<Chest> {
    let zt = zone_id.zone_type();
    if zt == ZoneType::Town { return vec![]; }

    let seed = zone_id as u32 * 1637 + 89;
    let count = match zt {
        ZoneType::Cave | ZoneType::Graveyard => 15,
        _ => 10,
    };

    (0..count).map(|i| {
        let (x, y) = find_walkable_pos(i, 4, seed, terrain);
        Chest {
            pos: [x, y],
            opened: false,
        }
    }).collect()
}

// ---------------------------------------------------------------------------
// Color palette (dark fantasy, ALttP-inspired)
// ---------------------------------------------------------------------------
// Terrain
const C_DARK_GRASS: [f32; 4] = [0.06, 0.12, 0.04, 1.0];
const C_GRASS: [f32; 4] = [0.18, 0.32, 0.12, 1.0];
const C_DIRT: [f32; 4] = [0.38, 0.28, 0.16, 1.0];
const C_STONE: [f32; 4] = [0.45, 0.40, 0.34, 1.0];
const C_WATER: [f32; 4] = [0.10, 0.22, 0.38, 0.9];
// Structures
const C_WALL: [f32; 4] = [0.32, 0.26, 0.18, 1.0];
const C_WALL_TOP: [f32; 4] = [0.40, 0.34, 0.24, 1.0];
const C_WALL_DK: [f32; 4] = [0.20, 0.15, 0.09, 1.0];
const C_WOOD: [f32; 4] = [0.36, 0.22, 0.10, 1.0];
const C_WOOD_DK: [f32; 4] = [0.22, 0.14, 0.06, 1.0];
const C_ROOF: [f32; 4] = [0.55, 0.30, 0.12, 1.0];
const C_ROOF_DK: [f32; 4] = [0.40, 0.22, 0.08, 1.0];
const C_METAL: [f32; 4] = [0.50, 0.48, 0.44, 1.0];
const C_METAL_DK: [f32; 4] = [0.35, 0.33, 0.30, 1.0];
// Fire
const C_FIRE_CORE: [f32; 4] = [1.0, 0.92, 0.45, 1.0];
const C_FIRE_MID: [f32; 4] = [0.95, 0.50, 0.08, 0.85];
const C_FIRE_GLOW: [f32; 4] = [0.55, 0.20, 0.03, 0.22];
const C_EMBER: [f32; 4] = [0.28, 0.08, 0.02, 1.0];
const C_TORCH_FLAME: [f32; 4] = [1.0, 0.70, 0.15, 0.80];
// Nature
const C_SHADOW: [f32; 4] = [0.0, 0.0, 0.0, 0.25];
// Player
const C_PLAYER_BODY: [f32; 4] = [0.65, 0.12, 0.10, 1.0];
const C_PLAYER_DK: [f32; 4] = [0.42, 0.08, 0.06, 1.0];
const C_SKIN: [f32; 4] = [0.74, 0.60, 0.46, 1.0];
const C_HAIR: [f32; 4] = [0.30, 0.18, 0.08, 1.0];
const C_HELMET: [f32; 4] = [0.58, 0.55, 0.50, 1.0];
const C_SHIELD: [f32; 4] = [0.42, 0.24, 0.12, 1.0];
const C_SHIELD_HL: [f32; 4] = [0.55, 0.35, 0.18, 1.0];
const C_WEAPON: [f32; 4] = [0.62, 0.60, 0.56, 1.0];
const C_BOOTS: [f32; 4] = [0.30, 0.18, 0.08, 1.0];
// NPCs
const C_HEALER: [f32; 4] = [0.35, 0.55, 0.80, 1.0];
const C_MERCHANT: [f32; 4] = [0.68, 0.58, 0.22, 1.0];
const C_SMITH: [f32; 4] = [0.55, 0.48, 0.40, 1.0];
const C_WARRIOR: [f32; 4] = [0.28, 0.42, 0.22, 1.0];
// Enemies
const C_ZOMBIE: [f32; 4] = [0.30, 0.40, 0.22, 1.0];
const C_ZOMBIE_DK: [f32; 4] = [0.18, 0.26, 0.12, 1.0];
const C_SKELETON: [f32; 4] = [0.78, 0.75, 0.68, 1.0];
const C_SKELETON_DK: [f32; 4] = [0.52, 0.50, 0.44, 1.0];
const C_FALLEN: [f32; 4] = [0.62, 0.22, 0.16, 1.0];
const C_FALLEN_DK: [f32; 4] = [0.42, 0.14, 0.10, 1.0];
// UI
const C_HUD_BG: [f32; 4] = [0.06, 0.05, 0.03, 0.95];
const C_HUD_FRAME: [f32; 4] = [0.35, 0.28, 0.18, 1.0];
const C_HP: [f32; 4] = [0.75, 0.08, 0.08, 1.0];
const C_HP_BG: [f32; 4] = [0.30, 0.04, 0.04, 1.0];
const C_MP: [f32; 4] = [0.08, 0.14, 0.75, 1.0];
const C_MP_BG: [f32; 4] = [0.04, 0.06, 0.30, 1.0];
const C_XP: [f32; 4] = [0.68, 0.58, 0.14, 1.0];
const C_SLOT: [f32; 4] = [0.18, 0.15, 0.12, 0.90];
const C_SLOT_FR: [f32; 4] = [0.38, 0.30, 0.20, 1.0];
const C_HP_BAR: [f32; 4] = [0.88, 0.12, 0.12, 0.92];
const C_HP_BAR_BG: [f32; 4] = [0.22, 0.06, 0.06, 0.72];
const C_NPC_LABEL: [f32; 4] = [0.92, 0.88, 0.55, 0.88];
const C_INTERACT: [f32; 4] = [1.0, 0.95, 0.50, 0.72];
const C_MINIMAP_BG: [f32; 4] = [0.05, 0.05, 0.03, 0.82];
const C_MINIMAP_FR: [f32; 4] = [0.42, 0.34, 0.20, 0.92];
const C_MINIMAP_PLAYER: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const C_MINIMAP_NPC: [f32; 4] = [0.30, 0.60, 1.0, 1.0];
const C_MINIMAP_ENEMY: [f32; 4] = [1.0, 0.22, 0.18, 1.0];
const C_DMG_TEXT: [f32; 4] = [1.0, 0.92, 0.22, 1.0];
const C_HEAL_TEXT: [f32; 4] = [0.22, 1.0, 0.32, 1.0];
const C_WP_GLOW: [f32; 4] = [0.18, 0.38, 0.78, 0.35];
const C_WP_CORE: [f32; 4] = [0.35, 0.65, 1.0, 0.82];

// Gameplay constants from mge_sim::constants::*

// ---------------------------------------------------------------------------
// Camp-specific world positions
// ---------------------------------------------------------------------------
#[rustfmt::skip]
const PALISADE: &[[f32; 2]] = &[
    [6.5,1.5],[8.0,1.5],[9.5,1.5],[11.0,1.5],[12.5,1.5],[14.0,1.5],[15.5,1.5],[17.0,1.5],
    [18.5,2.5],[19.5,3.5],[20.5,5.0],
    [21.0,6.5],[21.0,8.0],[21.0,9.5],[21.0,11.0],[21.0,12.5],[21.0,14.0],[21.0,15.5],[21.0,17.0],
    [20.5,18.5],[19.5,19.5],[18.5,20.5],
    [17.0,21.5],[15.5,21.5],[14.0,21.5],[11.0,21.5],[9.5,21.5],[8.0,21.5],[6.5,21.5],
    [5.0,20.5],[4.0,19.5],[3.0,18.5],
    [2.5,17.0],[2.5,15.5],[2.5,14.0],[2.5,12.5],[2.5,11.0],[2.5,9.5],[2.5,8.0],[2.5,6.5],
    [3.0,5.0],[4.0,3.5],[5.0,2.5],
];

// ---------------------------------------------------------------------------
// Per-tile color variation
// ---------------------------------------------------------------------------
#[allow(clippy::cast_precision_loss)]
fn vary(base: [f32; 4], x: u32, y: u32) -> [f32; 4] {
    let h = x.wrapping_mul(374_761_393).wrapping_add(y.wrapping_mul(668_265_263));
    let v = ((h >> 16) & 0xFF) as f32 / 255.0 * 0.10 - 0.05; // ±5% variation
    // Slight per-channel variation for richer colors
    let h2 = h.wrapping_mul(1_274_126_177);
    let v2 = ((h2 >> 20) & 0xFF) as f32 / 255.0 * 0.04 - 0.02;
    [
        (base[0] + v + v2).clamp(0.0, 1.0),
        (base[1] + v).clamp(0.0, 1.0),
        (base[2] + v - v2).clamp(0.0, 1.0),
        base[3],
    ]
}

// EnemyKind from mge_sim::entity

const C_BOSS: [f32; 4] = [0.55, 0.10, 0.45, 1.0];
const C_BOSS_DK: [f32; 4] = [0.35, 0.06, 0.30, 1.0];
const C_QUILL_RAT: [f32; 4] = [0.48, 0.32, 0.18, 1.0];
const C_QUILL_RAT_DK: [f32; 4] = [0.32, 0.20, 0.10, 1.0];
const C_SHAMAN: [f32; 4] = [0.72, 0.35, 0.12, 1.0];
const C_SHAMAN_DK: [f32; 4] = [0.50, 0.22, 0.08, 1.0];
const C_BLOOD_RAVEN: [f32; 4] = [0.30, 0.08, 0.35, 1.0];
const C_BLOOD_RAVEN_DK: [f32; 4] = [0.18, 0.04, 0.22, 1.0];
const C_CAVE_FLOOR: [f32; 4] = [0.20, 0.17, 0.15, 1.0];
const C_FROST_BOLT: [f32; 4] = [0.40, 0.70, 1.0, 0.88];
const C_FROST_CORE: [f32; 4] = [0.85, 0.92, 1.0, 1.0];
const C_PORTAL: [f32; 4] = [0.20, 0.35, 0.85, 0.70];
const C_PORTAL_CORE: [f32; 4] = [0.55, 0.75, 1.0, 0.88];
const C_ZONE_ARROW: [f32; 4] = [1.0, 0.92, 0.40, 0.70];
const C_QUEST_BG: [f32; 4] = [0.08, 0.06, 0.04, 0.88];
const C_QUEST_DONE: [f32; 4] = [0.22, 0.72, 0.22, 0.90];
const C_QUEST_ACTIVE: [f32; 4] = [0.82, 0.72, 0.22, 0.90];
const C_BREAKABLE: [f32; 4] = [0.38, 0.28, 0.14, 1.0];
const C_BREAKABLE_DK: [f32; 4] = [0.24, 0.16, 0.08, 1.0];
const C_GOLD_DROP: [f32; 4] = [0.90, 0.78, 0.15, 1.0];
const C_HP_POT: [f32; 4] = [0.85, 0.15, 0.12, 1.0];
const C_MP_POT: [f32; 4] = [0.15, 0.20, 0.85, 1.0];
const C_FIREBALL: [f32; 4] = [1.0, 0.55, 0.08, 0.92];
const C_FIREBALL_CORE: [f32; 4] = [1.0, 0.90, 0.40, 1.0];
const C_STATS_BG: [f32; 4] = [0.08, 0.06, 0.04, 0.92];
const C_STATS_FR: [f32; 4] = [0.45, 0.36, 0.22, 1.0];
const C_STATS_TEXT: [f32; 4] = [0.88, 0.82, 0.65, 1.0];
const C_DEAD_OVERLAY: [f32; 4] = [0.15, 0.02, 0.02, 0.65];

// Enemy, DamageFloat, PotionKind, DropKind, ItemDrop from mge_sim::entity

/// Render-only: map enemy kind to tint colors.
fn enemy_colors(kind: EnemyKind) -> ([f32; 4], [f32; 4]) {
    match kind {
        EnemyKind::Zombie => (C_ZOMBIE, C_ZOMBIE_DK),
        EnemyKind::Skeleton => (C_SKELETON, C_SKELETON_DK),
        EnemyKind::Fallen => (C_FALLEN, C_FALLEN_DK),
        EnemyKind::QuillRat => (C_QUILL_RAT, C_QUILL_RAT_DK),
        EnemyKind::FallenShaman => (C_SHAMAN, C_SHAMAN_DK),
        EnemyKind::Boss => (C_BOSS, C_BOSS_DK),
        EnemyKind::BloodRaven => (C_BLOOD_RAVEN, C_BLOOD_RAVEN_DK),
    }
}

// ItemRarity, EquipSlot, Equipment from mge_sim::entity

// EquipSlot impl, EQUIP_SLOT_COUNT from mge_sim

// Equipment, LogEntry, LOG_MAX, LOG_LIFE from mge_sim

// SkillId, skill_name, SKILL_COUNT, spell constants from mge_sim

const C_LIGHTNING: [f32; 4] = [0.60, 0.70, 1.0, 0.95];
const C_LIGHTNING_CORE: [f32; 4] = [0.90, 0.95, 1.0, 1.0];
const C_SHRINE: [f32; 4] = [0.25, 0.55, 0.80, 0.85];
const C_SHRINE_GLOW: [f32; 4] = [0.35, 0.65, 0.95, 0.50];
const C_CHEST: [f32; 4] = [0.50, 0.35, 0.15, 1.0];
const C_CHEST_LID: [f32; 4] = [0.60, 0.45, 0.20, 1.0];
const C_MERC_BODY: [f32; 4] = [0.30, 0.45, 0.28, 1.0];
const C_MERC_HEAD: [f32; 4] = [0.75, 0.60, 0.45, 1.0];
const C_POISON: [f32; 4] = [0.20, 0.65, 0.10, 0.80];

// Gather nodes — trees, ore, herbs
const C_TRUNK: [f32; 4] = [0.30, 0.18, 0.08, 1.0];
const C_TRUNK_DK: [f32; 4] = [0.20, 0.12, 0.05, 1.0];
const C_CANOPY: [f32; 4] = [0.12, 0.38, 0.10, 1.0];
const C_CANOPY_DK: [f32; 4] = [0.08, 0.26, 0.06, 1.0];
const C_CANOPY_HI: [f32; 4] = [0.20, 0.48, 0.16, 1.0];
const C_HARDWOOD: [f32; 4] = [0.18, 0.30, 0.12, 1.0];
const C_HARDWOOD_DK: [f32; 4] = [0.10, 0.20, 0.06, 1.0];
const C_ORE_ROCK: [f32; 4] = [0.40, 0.36, 0.30, 1.0];
const C_ORE_ROCK_DK: [f32; 4] = [0.28, 0.24, 0.18, 1.0];
const C_ORE_NUGGET: [f32; 4] = [0.70, 0.55, 0.20, 1.0];
const C_RARE_ORE: [f32; 4] = [0.55, 0.48, 0.58, 1.0];
const C_RARE_NUGGET: [f32; 4] = [0.85, 0.70, 0.90, 1.0];
const C_HERB: [f32; 4] = [0.15, 0.50, 0.18, 1.0];
const C_HERB_FLOWER: [f32; 4] = [0.80, 0.65, 0.20, 1.0];
const C_MUSHROOM_CAP: [f32; 4] = [0.65, 0.25, 0.15, 1.0];
const C_MUSHROOM_STEM: [f32; 4] = [0.80, 0.75, 0.65, 1.0];
const C_DEPLETED: [f32; 4] = [0.25, 0.22, 0.18, 0.5];

// Projectile, LightningBolt, Mercenary, ShrineKind, Shrine, Chest,
// ActiveBuff, Breakable, Quest, NpcInfo, NPCS from mge_sim::entity

// ---------------------------------------------------------------------------
// App
// ---------------------------------------------------------------------------
#[allow(clippy::struct_excessive_bools)]
struct SodomightApp {
    bootstrap: GameBootstrap,
    save_manager: SaveManager,
    profile: PlayerProfile,
    sim: AuthoritativeSim,
    audio: AudioBus,
    replication: ReplicationPlan,
    window: Option<Arc<Window>>,
    window_id: Option<WindowId>,
    renderer: Option<GraphicsState>,
    fatal_error: Option<anyhow::Error>,
    batch: SpriteBatch,
    camera: IsoCamera,
    cam_focus: [f32; 2],
    viewport: [f32; 2],
    player_pos: [f32; 2],
    keys: u8,
    move_target: Option<[f32; 2]>,
    mouse_screen: [f32; 2],
    player_hp: f32,
    player_mp: f32,
    player_xp: f32,
    player_level: u32,
    attack_cd: u32,
    attack_target: Option<usize>,
    enemies: Vec<Enemy>,
    floats: Vec<DamageFloat>,
    drops: Vec<ItemDrop>,
    projectiles: Vec<Projectile>,
    breakables: Vec<Breakable>,
    frame: u32,
    npc_interact: Option<usize>,
    interact_timer: u32,
    show_minimap: bool,
    show_stats: bool,
    show_quests: bool,
    player_gold: u32,
    belt: [Option<PotionKind>; 4],
    player_dead: bool,
    player_respawn_timer: u32,
    running: bool,
    facing_right: bool,
    // Zone system
    current_zone: ZoneId,
    zone_enemies: Vec<Vec<Enemy>>,
    zone_breakables: Vec<Vec<Breakable>>,
    // Skill system
    active_skill: SkillId,
    // Skill grid (13×13 scroll slot grid)
    skill_grid: SkillGrid,
    grid_unlock_points: u32,  // 1 per level, used to unlock slots
    disk_bonuses: DiskBonuses,
    show_disk: bool,
    disk_scroll: [f32; 2],   // pan offset for scrollable grid view
    disk_dragging: bool,     // mouse drag to pan
    scroll_inventory: Vec<skill_disk::ScrollItem>, // scrolls the player has
    // Civil skills (artisanat / vie)
    civil: CivilSkills,
    show_civil: bool,
    // Stat points
    stat_str: u32,
    stat_dex: u32,
    stat_vit: u32,
    stat_ene: u32,
    stat_points: u32,
    // Town portal
    portal_active: bool,
    portal_pos: [f32; 2],
    portal_zone: ZoneId,
    // Quests
    quests: Vec<Quest>,
    // Zone transition cooldown
    zone_cd: u32,
    // Equipment system
    equipment: [Option<Equipment>; EQUIP_SLOT_COUNT],
    show_inventory: bool,
    backpack: Backpack,
    inv_hover_slot: Option<usize>,  // selected slot in backpack tab
    // Combat log
    log: Vec<LogEntry>,
    // Waypoints discovered
    waypoints: [bool; ZONE_COUNT],
    show_waypoint_menu: bool,
    // NPC dialog
    npc_dialog: Option<usize>,
    // Mercenary
    mercenary: Option<Mercenary>,
    // Summons (player minions)
    summons: Vec<Summon>,
    // Combo tracker
    combo: ComboTracker,
    // Party system (MMO)
    party: Party,
    // Chat system (MMO)
    chat: ChatLog,
    show_chat: bool,
    // World boss
    world_boss: WorldBoss,
    // Active aura (from merc or self)
    active_aura: Option<AuraKind>,
    // Shrines per zone
    zone_shrines: Vec<Vec<Shrine>>,
    shrines: Vec<Shrine>,
    // Chests per zone
    zone_chests: Vec<Vec<Chest>>,
    chests: Vec<Chest>,
    // Active buffs
    buffs: Vec<ActiveBuff>,
    // Lightning bolts (visual)
    lightning_bolts: Vec<LightningBolt>,
    // Poison timer (frames remaining)
    poison_timer: u32,
    // Difficulty level
    difficulty: Difficulty,
    // Player elemental resistances
    resistances: Resistances,
    // World-space: current zone's origin in the global coordinate system
    world_offset: [f32; 2],
    // Gather nodes (trees, ore, herbs) — per zone storage + active
    zone_gather_nodes: Vec<Vec<GatherNode>>,
    gather_nodes: Vec<GatherNode>,
    harvest_target: Option<usize>,  // index into gather_nodes
    harvest_cd: u32,                // cooldown between harvest hits
    // Global terrain — all zones stitched into one continuous 500×500 map
    global_terrain: Vec<u8>,
}

impl SodomightApp {
    fn new(bootstrap: GameBootstrap, save_manager: SaveManager) -> Result<Self> {
        let profile = save_manager.load_profile()?.unwrap_or(PlayerProfile {
            display_name: "Wanderer".to_owned(),
            last_scene: bootstrap.startup_scene.id.clone(),
            level: 1,
        });
        let scene = SceneSummary {
            id: bootstrap.startup_scene.id.clone(),
            biome: bootstrap.startup_scene.biome.clone(),
            ambient_rgb: [
                bootstrap.startup_scene.ambient_rgb.r,
                bootstrap.startup_scene.ambient_rgb.g,
                bootstrap.startup_scene.ambient_rgb.b,
            ],
        };
        let config = RuntimeConfig { tick_rate_hz: 60, startup_scene: scene.id.clone() };

        // Generate all zone data procedurally
        let zone_terrains: Vec<_> = ZoneId::ALL.iter().map(|&z| generate_terrain(z)).collect();
        let zone_enemies: Vec<_> = ZoneId::ALL.iter().enumerate()
            .map(|(i, &z)| generate_enemies(z, &zone_terrains[i])).collect();
        let zone_breakables: Vec<_> = ZoneId::ALL.iter().enumerate()
            .map(|(i, &z)| generate_breakables(z, &zone_terrains[i])).collect();
        let zone_shrines: Vec<_> = ZoneId::ALL.iter().enumerate()
            .map(|(i, &z)| generate_shrines(z, &zone_terrains[i])).collect();
        let zone_chests: Vec<_> = ZoneId::ALL.iter().enumerate()
            .map(|(i, &z)| generate_chests(z, &zone_terrains[i])).collect();
        let zone_gather_nodes: Vec<_> = ZoneId::ALL.iter().enumerate()
            .map(|(i, &z)| generate_gather_nodes(z.zone_type(), z.tier(), &zone_terrains[i], 42 + i as u32)).collect();

        // Stitch all zone terrains into one global 500×500 map
        let global_terrain = stitch_global_terrain(&zone_terrains);

        let quests = vec![
            Quest { name: "Den of Evil", desc: "Clear the Den of Evil", done: false },
            Quest { name: "Blood Raven", desc: "Defeat Blood Raven in Cold Plains", done: false },
            Quest { name: "Search & Rescue", desc: "Find 100 gold", done: false },
            Quest { name: "Dark Wood", desc: "Find the Tree of Inifuss", done: false },
            Quest { name: "Tamoe Highlands", desc: "Reach the Tamoe Highlands", done: false },
        ];

        let start = [50.0_f32, 50.0];
        let initial_enemies = zone_enemies[0].clone();
        let initial_breakables = zone_breakables[0].clone();
        let initial_shrines = zone_shrines[0].clone();
        let initial_chests = zone_chests[0].clone();
        let initial_gather = zone_gather_nodes[0].clone();
        Ok(Self {
            replication: ReplicationPlan::bootstrap(scene.id.clone()),
            sim: AuthoritativeSim::new(config, scene),
            audio: AudioBus::new(bootstrap.audio.listener_bus.clone()),
            bootstrap,
            fatal_error: None,
            profile,
            renderer: None,
            save_manager,
            window: None,
            window_id: None,
            batch: SpriteBatch::new(),
            camera: IsoCamera::default(),
            cam_focus: start,
            viewport: [1280.0, 720.0],
            player_pos: start,
            keys: 0,
            move_target: None,
            mouse_screen: [0.0, 0.0],
            player_hp: PLAYER_MAX_HP,
            player_mp: PLAYER_MAX_MP,
            player_xp: 0.0,
            player_level: 1,
            attack_cd: 0,
            attack_target: None,
            enemies: initial_enemies,
            floats: Vec::new(),
            drops: Vec::new(),
            projectiles: Vec::new(),
            breakables: initial_breakables,
            frame: 0,
            npc_interact: None,
            interact_timer: 0,
            show_minimap: true,
            show_stats: false,
            show_quests: false,
            player_gold: 0,
            belt: [Some(PotionKind::Health), Some(PotionKind::Health), Some(PotionKind::Mana), None],
            player_dead: false,
            player_respawn_timer: 0,
            running: false,
            facing_right: true,
            current_zone: ZoneId::RogueCamp,
            zone_enemies,
            zone_breakables,
            active_skill: SkillId::Melee,
            skill_grid: SkillGrid::new(),
            grid_unlock_points: 0,
            disk_bonuses: DiskBonuses::default(),
            show_disk: false,
            disk_scroll: [0.0, 0.0],
            disk_dragging: false,
            scroll_inventory: Vec::new(),
            civil: CivilSkills::new(),
            show_civil: false,
            stat_str: 10,
            stat_dex: 10,
            stat_vit: 10,
            stat_ene: 10,
            stat_points: 0,
            portal_active: false,
            portal_pos: [0.0, 0.0],
            portal_zone: ZoneId::RogueCamp,
            quests,
            zone_cd: 0,
            equipment: [None; EQUIP_SLOT_COUNT],
            show_inventory: false,
            backpack: Backpack::default(),
            inv_hover_slot: None,
            log: Vec::new(),
            waypoints: {
                let mut wp = [false; ZONE_COUNT];
                wp[ZoneId::RogueCamp as usize] = true;
                wp
            },
            show_waypoint_menu: false,
            npc_dialog: None,
            mercenary: None,
            summons: Vec::new(),
            combo: ComboTracker::default(),
            party: Party::new_solo(b"HERO", 1),
            chat: ChatLog::new(),
            show_chat: false,
            world_boss: WorldBoss::new([50.0, 50.0]),
            active_aura: None,
            zone_shrines,
            shrines: initial_shrines,
            zone_chests,
            chests: initial_chests,
            zone_gather_nodes,
            gather_nodes: initial_gather,
            harvest_target: None,
            harvest_cd: 0,
            buffs: Vec::new(),
            lightning_bolts: Vec::new(),
            poison_timer: 0,
            difficulty: Difficulty::Normal,
            resistances: Resistances::default(),
            world_offset: zone_coord(ZoneId::RogueCamp).world_origin(),
            global_terrain,
        })
    }

    fn set_fatal(&mut self, event_loop: &ActiveEventLoop, error: anyhow::Error) {
        self.fatal_error = Some(error);
        event_loop.exit();
    }

    fn save_progress(&self) {
        let _ = self.save_manager.save_profile(&self.profile);
    }

    fn is_camp(&self) -> bool { self.current_zone == ZoneId::RogueCamp }

    /// Check if a local tile position is walkable using the global terrain.
    /// Converts local coords to global indices via world_offset.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_possible_wrap)]
    fn tile_walkable(global_terrain: &[u8], world_off: [f32; 2], x: f32, y: f32) -> bool {
        let gx = (x + world_off[0] + GLOBAL_OFF) as i32;
        let gy = (y + world_off[1] + GLOBAL_OFF) as i32;
        if gx < 0 || gy < 0 || gx >= WORLD_W as i32 || gy >= WORLD_H as i32 {
            return false; // world edge = impassable
        }
        let t = global_terrain[gy as usize * WORLD_W + gx as usize];
        t != 0 && t != 5 // 0 = wall, 5 = water — both impassable
    }

    /// Move `pos` by (dx, dy) with wall sliding. Try full move first,
    /// then each axis independently to allow sliding along walls.
    fn try_move(global_terrain: &[u8], world_off: [f32; 2], pos: &mut [f32; 2], dx: f32, dy: f32) {
        let nx = pos[0] + dx;
        let ny = pos[1] + dy;
        if Self::tile_walkable(global_terrain, world_off, nx, ny) {
            pos[0] = nx;
            pos[1] = ny;
            return;
        }
        if dx != 0.0 && Self::tile_walkable(global_terrain, world_off, nx, pos[1]) {
            pos[0] = nx;
            return;
        }
        if dy != 0.0 && Self::tile_walkable(global_terrain, world_off, pos[0], ny) {
            pos[1] = ny;
        }
    }

    /// Player position in world-space.
    fn player_world_pos(&self) -> [f32; 2] {
        [self.player_pos[0] + self.world_offset[0],
         self.player_pos[1] + self.world_offset[1]]
    }

    /// Camera focus in world-space.
    fn cam_world(&self) -> [f32; 2] {
        self.player_world_pos()
    }

    /// World-space to screen.
    fn w2s_world(&self, wx: f32, wy: f32) -> [f32; 2] {
        let cam = self.cam_world();
        [
            (wx - cam[0]) * TW + self.viewport[0] * 0.5,
            (wy - cam[1]) * TH + self.viewport[1] * 0.5,
        ]
    }


    // ======================================================================
    // 3/4 camera (ALttP style): X mapped 1:1, Y squashed
    // ======================================================================

    fn w2s(&self, wx: f32, wy: f32) -> [f32; 2] {
        [
            (wx - self.cam_focus[0]) * TW + self.viewport[0] * 0.5,
            (wy - self.cam_focus[1]) * TH + self.viewport[1] * 0.5,
        ]
    }

    fn s2w(&self, sx: f32, sy: f32) -> [f32; 2] {
        [
            (sx - self.viewport[0] * 0.5) / TW + self.cam_focus[0],
            (sy - self.viewport[1] * 0.5) / TH + self.cam_focus[1],
        ]
    }

    // ======================================================================
    // Sprite helpers
    // ======================================================================

    /// Place sprite at world pos. `off` and `size` in pixels.
    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation, clippy::too_many_arguments)]
    fn spr(&mut self, wx: f32, wy: f32, off: [f32; 2], size: [f32; 2],
           layer: RenderLayer, sub: u32, tint: [f32; 4]) {
        let [sx, sy] = self.w2s(wx, wy);
        self.batch.push(SpriteInstance {
            screen_pos: [sx + off[0], sy + off[1]],
            src: UNIT, atlas: ATLAS_0, material: MAT,
            sort_key: SortKey { layer, y_sort: (wy * 100.0) as i32, sub_order: sub },
            tint, scale: size,
        });
    }

    fn hud(&mut self, x: f32, y: f32, w: f32, h: f32, sub: u32, tint: [f32; 4]) {
        self.batch.push(SpriteInstance {
            screen_pos: [x, y],
            src: UNIT, atlas: ATLAS_0, material: MAT,
            sort_key: SortKey { layer: RenderLayer::UiScreen, y_sort: 0, sub_order: sub },
            tint, scale: [w, h],
        });
    }

    /// Draw bitmap text in screen-space (HUD). `px` = pixel size per font dot.
    /// Returns the total width in pixels drawn.
    #[allow(clippy::cast_precision_loss)]
    fn draw_text(&mut self, text: &[u8], x: f32, y: f32, px: f32, tint: [f32; 4], sub: u32) -> f32 {
        let char_w = 6.0 * px; // 5 cols + 1 gap
        for (ci, &ch) in text.iter().enumerate() {
            let rows = glyph(ch);
            let cx = x + ci as f32 * char_w;
            for (row, &bits) in rows.iter().enumerate() {
                if bits == 0 { continue; }
                let ry = y + row as f32 * px;
                let mut col = 0u8;
                while col < 5 {
                    if bits & (0x10 >> col) != 0 {
                        let start = col;
                        while col < 5 && bits & (0x10 >> col) != 0 {
                            col += 1;
                        }
                        let rx = cx + f32::from(start) * px;
                        let rw = f32::from(col - start) * px;
                        self.batch.push(SpriteInstance {
                            screen_pos: [rx, ry],
                            src: UNIT, atlas: ATLAS_0, material: MAT,
                            sort_key: SortKey { layer: RenderLayer::UiScreen, y_sort: 0, sub_order: sub },
                            tint, scale: [rw, px],
                        });
                    } else {
                        col += 1;
                    }
                }
            }
        }
        text.len() as f32 * char_w
    }

    /// Draw bitmap text in world-space (above entities). Returns width.
    #[allow(dead_code, clippy::cast_precision_loss, clippy::cast_possible_truncation,
            clippy::cast_possible_wrap, clippy::too_many_arguments)]
    fn draw_text_world(&mut self, text: &[u8], wx: f32, wy: f32, off: [f32; 2],
                       px: f32, tint: [f32; 4], sub: u32) -> f32 {
        let [sx, sy] = self.w2s(wx, wy);
        self.draw_text_at(text, sx + off[0], sy + off[1], px, tint,
                          RenderLayer::UiWorld, (wy * 100.0) as i32, sub)
    }

    /// Draw text at screen coords with arbitrary layer and y-sort.
    #[allow(clippy::cast_precision_loss, clippy::too_many_arguments)]
    fn draw_text_at(&mut self, text: &[u8], x: f32, y: f32, px: f32, tint: [f32; 4],
                    layer: RenderLayer, y_sort: i32, sub: u32) -> f32 {
        let char_w = 6.0 * px;
        for (ci, &ch) in text.iter().enumerate() {
            let rows = glyph(ch);
            let cx = x + ci as f32 * char_w;
            for (row, &bits) in rows.iter().enumerate() {
                if bits == 0 { continue; }
                let ry = y + row as f32 * px;
                let mut col = 0u8;
                while col < 5 {
                    if bits & (0x10 >> col) != 0 {
                        let start = col;
                        while col < 5 && bits & (0x10 >> col) != 0 {
                            col += 1;
                        }
                        let rx = cx + f32::from(start) * px;
                        let rw = f32::from(col - start) * px;
                        self.batch.push(SpriteInstance {
                            screen_pos: [rx, ry],
                            src: UNIT, atlas: ATLAS_0, material: MAT,
                            sort_key: SortKey { layer, y_sort, sub_order: sub },
                            tint, scale: [rw, px],
                        });
                    } else {
                        col += 1;
                    }
                }
            }
        }
        text.len() as f32 * char_w
    }

    // ======================================================================
    // Game logic (unchanged)
    // ======================================================================

    fn dist(a: [f32; 2], b: [f32; 2]) -> f32 {
        let dx = a[0] - b[0];
        let dy = a[1] - b[1];
        (dx * dx + dy * dy).sqrt()
    }

    fn move_toward(pos: &mut [f32; 2], target: [f32; 2], speed: f32) -> bool {
        let dx = target[0] - pos[0];
        let dy = target[1] - pos[1];
        let d = (dx * dx + dy * dy).sqrt();
        if d < ARRIVE_THRESHOLD { return true; }
        pos[0] += dx / d * speed;
        pos[1] += dy / d * speed;
        false
    }

    fn update_click_move(&mut self) {
        if let Some(target) = self.move_target {
            let spd = if self.running { CLICK_RUN_SPEED } else { CLICK_MOVE_SPEED };
            let dx = target[0] - self.player_pos[0];
            let dy = target[1] - self.player_pos[1];
            let d = (dx * dx + dy * dy).sqrt();
            if d < ARRIVE_THRESHOLD {
                self.move_target = None;
            } else {
                let old_pos = self.player_pos;
                Self::try_move(&self.global_terrain, self.world_offset, &mut self.player_pos, dx / d * spd, dy / d * spd);
                // Cancel move target if stuck (can't make progress)
                #[allow(clippy::float_cmp)]
                if self.player_pos == old_pos {
                    self.move_target = None;
                }
            }
        }
    }

    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_precision_loss)]
    fn update_enemies(&mut self) {
        let pp = self.player_pos;
        let pw = self.player_world_pos();
        let wo = self.world_offset;
        let gt = &self.global_terrain;
        // Check if any shaman is alive (for Fallen flee behavior)
        let shaman_alive = self.enemies.iter().any(|e| e.alive && e.kind == EnemyKind::FallenShaman);
        for enemy in &mut self.enemies {
            if !enemy.alive {
                if enemy.fade_timer > 0 {
                    enemy.fade_timer -= 1;
                }
                if enemy.respawn_timer > 0 {
                    enemy.respawn_timer -= 1;
                    if enemy.respawn_timer == 0 {
                        enemy.alive = true;
                        enemy.hp = enemy.max_hp;
                        enemy.pos = enemy.spawn;
                        enemy.fade_timer = 0;
                        enemy.freeze_timer = 0;
                        enemy.slow_timer = 0;
                    }
                }
                continue;
            }
            let ew = [enemy.pos[0] + wo[0], enemy.pos[1] + wo[1]];
            if !world_map::in_dormant_radius(pw, ew) {
                continue;
            }
            if enemy.freeze_timer > 0 { enemy.freeze_timer -= 1; }
            if enemy.slow_timer > 0 { enemy.slow_timer -= 1; }
            if enemy.attack_cd > 0 { enemy.attack_cd -= 1; }
            if !world_map::in_sim_radius(pw, ew) {
                continue;
            }
            if enemy.freeze_timer > 0 { continue; }
            let spd = enemy.effective_speed();
            let d = Self::dist(enemy.pos, pp);
            let fleeing = enemy.kind == EnemyKind::Fallen
                && !shaman_alive
                && enemy.hp < enemy.max_hp * 0.5
                && d < ENEMY_CHASE_RANGE;
            if fleeing {
                let dx = enemy.pos[0] - pp[0];
                let dy = enemy.pos[1] - pp[1];
                let fd = (dx * dx + dy * dy).sqrt().max(0.01);
                Self::try_move(gt, wo, &mut enemy.pos, dx / fd * spd * 1.5, dy / fd * spd * 1.5);
            } else if d < ENEMY_CHASE_RANGE && d > ENEMY_ATTACK_RANGE {
                let dx = pp[0] - enemy.pos[0];
                let dy = pp[1] - enemy.pos[1];
                Self::try_move(gt, wo, &mut enemy.pos, dx / d * spd, dy / d * spd);
            } else if d >= ENEMY_CHASE_RANGE {
                let sd = Self::dist(enemy.pos, enemy.spawn);
                if sd > 1.0 {
                    let dx = enemy.spawn[0] - enemy.pos[0];
                    let dy = enemy.spawn[1] - enemy.pos[1];
                    Self::try_move(gt, wo, &mut enemy.pos, dx / sd * spd * 0.5, dy / sd * spd * 0.5);
                }
            }
            enemy.pos[0] = enemy.pos[0].clamp(0.5, MAP_W as f32 - 0.5);
            enemy.pos[1] = enemy.pos[1].clamp(0.5, MAP_H as f32 - 0.5);
        }
    }

    fn kill_enemy(&mut self, idx: usize) {
        self.enemies[idx].alive = false;
        self.enemies[idx].respawn_timer = if self.enemies[idx].is_boss() { 1800 } else { 600 };
        self.enemies[idx].fade_timer = 30;
        let pos = self.enemies[idx].pos;
        let kind = self.enemies[idx].kind;
        let xp = self.enemies[idx].xp_value() * self.xp_multiplier() * self.difficulty.monster_xp_mult();
        self.player_xp += xp;
        #[allow(clippy::cast_precision_loss)]
        let needed = self.player_level as f32 * XP_PER_LEVEL;
        if self.player_xp >= needed {
            self.player_xp -= needed;
            self.player_level += 1;
            self.player_hp = self.max_hp();
            self.player_mp = self.max_mp();
            self.stat_points += 5;
            self.grid_unlock_points += 1;
            self.push_log(b"LEVEL UP!", [1.0, 0.95, 0.40, 1.0]);
        }
        // Discover waypoint for current zone on kill
        self.waypoints[self.current_zone as usize] = true;
        // Log the kill
        let name: &[u8] = match kind {
            EnemyKind::Zombie => b"ZOMBIE", EnemyKind::Skeleton => b"SKELETON",
            EnemyKind::Fallen => b"FALLEN", EnemyKind::QuillRat => b"QUILL RAT",
            EnemyKind::FallenShaman => b"SHAMAN", EnemyKind::Boss => b"BOSS",
            EnemyKind::BloodRaven => b"BLOOD RAVEN",
        };
        let mut msg = [0u8; 40];
        let prefix = b"KILLED ";
        msg[..prefix.len()].copy_from_slice(prefix);
        let end = (prefix.len() + name.len()).min(40);
        msg[prefix.len()..end].copy_from_slice(&name[..end - prefix.len()]);
        self.push_log(&msg[..end], C_DMG_TEXT);
        self.spawn_loot(pos, self.enemies[idx].is_boss());
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
    fn spawn_loot(&mut self, pos: [f32; 2], is_boss: bool) {
        let h = (pos[0] * 1000.0 + pos[1] * 777.0 + self.frame as f32) as u32;
        // Gold always drops
        self.drops.push(ItemDrop {
            pos: [pos[0] + 0.3, pos[1] + 0.2],
            kind: DropKind::Gold,
            gold_amount: if is_boss { 50 + h % 30 } else { 5 + h % 10 },
            life: 600,
        });
        // Chance for potions
        if h.is_multiple_of(4) || is_boss {
            self.drops.push(ItemDrop {
                pos: [pos[0] - 0.3, pos[1] + 0.1],
                kind: DropKind::HealthPotion,
                gold_amount: 0,
                life: 600,
            });
        }
        if h.is_multiple_of(5) || is_boss {
            self.drops.push(ItemDrop {
                pos: [pos[0] + 0.1, pos[1] - 0.3],
                kind: DropKind::ManaPotion,
                gold_amount: 0,
                life: 600,
            });
        }
        // Equipment drop chance
        let equip_roll = h.wrapping_mul(2_654_435_761);
        let drop_chance = if is_boss { 2 } else { 8 }; // boss: ~50%, normal: ~12.5%
        if equip_roll.is_multiple_of(drop_chance) {
            let slot = match equip_roll % 6 {
                0 => EquipSlot::Helm, 1 => EquipSlot::Armor, 2 => EquipSlot::Weapon,
                3 => EquipSlot::Boots, 4 => EquipSlot::Gloves, _ => EquipSlot::Ring,
            };
            let rarity = if is_boss {
                match equip_roll / 6 % 4 {
                    0 => ItemRarity::Rare, 1 | 2 => ItemRarity::Magic, _ => ItemRarity::Unique,
                }
            } else {
                match equip_roll / 6 % 8 {
                    0 => ItemRarity::Rare, 1..=3 => ItemRarity::Magic, _ => ItemRarity::Normal,
                }
            };
            self.drops.push(ItemDrop {
                pos: [pos[0] - 0.2, pos[1] - 0.2],
                kind: DropKind::Equipment(slot, rarity),
                gold_amount: 0,
                life: 900,
            });
        }
        // Scroll drop chance: ~10% normal, ~40% boss
        let scroll_roll = h.wrapping_mul(1_103_515_245).wrapping_add(12345);
        let scroll_chance = if is_boss { 5 } else { 10 }; // 1/5 boss, 1/10 normal
        if scroll_roll % scroll_chance == 0 {
            let content = random_scroll_drop(scroll_roll, self.player_level);
            self.drops.push(ItemDrop {
                pos: [pos[0] + 0.2, pos[1] + 0.3],
                kind: DropKind::Scroll(content),
                gold_amount: 0,
                life: 900,
            });
        }
    }

    fn pickup_drops(&mut self) {
        let pp = self.player_pos;
        let mut i = 0;
        while i < self.drops.len() {
            self.drops[i].life = self.drops[i].life.saturating_sub(1);
            if self.drops[i].life == 0 {
                self.drops.swap_remove(i);
                continue;
            }
            if Self::dist(pp, self.drops[i].pos) < 0.8 {
                match self.drops[i].kind {
                    DropKind::Gold => {
                        self.player_gold += self.drops[i].gold_amount;
                    }
                    DropKind::HealthPotion => {
                        if let Some(slot) = self.belt.iter_mut().find(|s| s.is_none()) {
                            *slot = Some(PotionKind::Health);
                        } else if !self.backpack.is_full() {
                            self.backpack.add(InvItem::Potion(PotionKind::Health));
                        } else {
                            i += 1;
                            continue;
                        }
                    }
                    DropKind::ManaPotion => {
                        if let Some(slot) = self.belt.iter_mut().find(|s| s.is_none()) {
                            *slot = Some(PotionKind::Mana);
                        } else if !self.backpack.is_full() {
                            self.backpack.add(InvItem::Potion(PotionKind::Mana));
                        } else {
                            i += 1;
                            continue;
                        }
                    }
                    DropKind::Equipment(slot, rarity) => {
                        let eq = Equipment { slot, rarity, ilvl: self.player_level };
                        // Auto-equip if slot empty
                        let idx = slot as usize;
                        if self.equipment[idx].is_none() {
                            self.equipment[idx] = Some(eq);
                            self.push_log_equip(slot, rarity);
                        } else if !self.backpack.is_full() {
                            // Put in backpack for manual management
                            self.backpack.add(InvItem::Equipment(eq));
                            self.push_log_equip(slot, rarity);
                        } else {
                            i += 1;
                            continue; // backpack full, leave on ground
                        }
                    }
                    DropKind::Scroll(content) => {
                        let scroll = skill_disk::ScrollItem { content };
                        if !self.backpack.is_full() {
                            self.backpack.add(InvItem::Scroll(scroll));
                            // Also add to scroll_inventory for skill grid compat
                            self.scroll_inventory.push(scroll);
                            let label = content.short_label();
                            let mut msg = [0u8; 32];
                            msg[..8].copy_from_slice(b"SCROLL: ");
                            let end = (8 + label.len()).min(32);
                            msg[8..end].copy_from_slice(&label[..end - 8]);
                            self.push_log(&msg[..end], [0.70, 0.85, 1.0, 1.0]);
                        } else {
                            i += 1;
                            continue; // backpack full
                        }
                    }
                }
                self.drops.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }

    #[allow(clippy::cast_precision_loss)]
    fn use_potion(&mut self, slot: usize) {
        if slot >= 4 { return; }
        if let Some(kind) = self.belt[slot].take() {
            match kind {
                PotionKind::Health => {
                    // VIT boosts HP potion effectiveness: +1% per VIT point
                    let bonus = 1.0 + self.total_vit() as f32 * 0.01;
                    let heal = POTION_HP_RESTORE * bonus;
                    self.player_hp = (self.player_hp + heal).min(self.max_hp());
                    self.floats.push(DamageFloat {
                        wx: self.player_pos[0], wy: self.player_pos[1],
                        dy: 0.0, value: heal, is_heal: true, life: 40,
                    });
                }
                PotionKind::Mana => {
                    // ENE boosts MP potion effectiveness: +1% per ENE point
                    let bonus = 1.0 + self.total_ene() as f32 * 0.01;
                    let restore = POTION_MP_RESTORE * bonus;
                    self.player_mp = (self.player_mp + restore).min(self.max_mp());
                }
            }
        }
    }

    #[allow(clippy::similar_names)]
    fn fire_projectile(&mut self, target_sx: f32, target_sy: f32) {
        let (cost, damage, range, speed, is_frost) = match self.active_skill {
            SkillId::Fireball => (FIREBALL_COST, self.fireball_damage(), FIREBALL_RANGE, FIREBALL_SPEED, false),
            SkillId::FrostBolt => (FROST_BOLT_COST, self.frost_bolt_damage(), FROST_BOLT_RANGE, FROST_BOLT_SPEED, true),
            SkillId::ChainLightning => {
                self.cast_chain_lightning();
                return;
            }
            SkillId::Teleport => {
                self.cast_teleport(target_sx, target_sy);
                return;
            }
            SkillId::Melee => {
                self.attack_breakable();
                return;
            }
        };
        if self.player_mp < cost { return; }
        self.player_mp -= cost;
        let [twx, twy] = self.s2w(target_sx, target_sy);
        let dx = twx - self.player_pos[0];
        let dy = twy - self.player_pos[1];
        let d = (dx * dx + dy * dy).sqrt();
        if d < 0.01 { return; }
        let _ = speed; // speed is baked into FIREBALL_SPEED / FROST_BOLT_SPEED constants
        self.projectiles.push(Projectile {
            pos: self.player_pos,
            dir: [dx / d, dy / d],
            dist_left: range,
            damage,
            is_frost,
            friendly: true,
        });
    }

    #[allow(clippy::cast_precision_loss)]
    fn update_projectiles(&mut self) {
        let mut hits: Vec<(usize, f32, [f32; 2], bool)> = Vec::new(); // (idx, dmg, pos, is_frost)
        let map_w = MAP_W as f32;
        let map_h = MAP_H as f32;
        for proj in &mut self.projectiles {
            let spd = if proj.is_frost { FROST_BOLT_SPEED } else if proj.friendly { FIREBALL_SPEED } else { ENEMY_PROJ_SPEED };
            proj.pos[0] += proj.dir[0] * spd;
            proj.pos[1] += proj.dir[1] * spd;
            proj.dist_left -= spd;
            if proj.friendly {
                for (i, enemy) in self.enemies.iter().enumerate() {
                    if !enemy.alive { continue; }
                    if Self::dist(proj.pos, enemy.pos) < 0.8 {
                        hits.push((i, proj.damage, proj.pos, proj.is_frost));
                        proj.dist_left = 0.0;
                        break;
                    }
                }
            }
            if proj.pos[0] < 0.0 || proj.pos[0] > map_w
                || proj.pos[1] < 0.0 || proj.pos[1] > map_h {
                proj.dist_left = 0.0;
            }
        }
        self.projectiles.retain(|p| p.dist_left > 0.0);
        for (idx, raw_dmg, pos, is_frost) in hits {
            // Apply elemental resistance
            let element = if is_frost { Element::Cold } else { Element::Fire };
            let dmg = self.enemies[idx].resistances.apply(raw_dmg, element);
            self.enemies[idx].hp -= dmg;
            self.floats.push(DamageFloat {
                wx: pos[0], wy: pos[1], dy: 0.0, value: dmg, is_heal: false, life: 40,
            });
            // Frost bolt: apply freeze/slow
            if is_frost && self.enemies[idx].alive {
                let cold_bonus = (self.disk_bonuses.cold_dmg_pct * 10.0) as u32;
                if cold_bonus >= 3 && self.enemies[idx].freeze_timer == 0 {
                    // High cold bonus: freeze for a short time
                    self.enemies[idx].freeze_timer = FREEZE_DURATION + cold_bonus * 10;
                }
                self.enemies[idx].slow_timer = SLOW_DURATION + cold_bonus * 15;
            }
            if self.enemies[idx].hp <= 0.0 {
                self.kill_enemy(idx);
                if self.attack_target == Some(idx) { self.attack_target = None; }
            }
        }
    }

    fn update_player_death(&mut self) {
        if self.player_dead {
            if self.player_respawn_timer > 0 {
                self.player_respawn_timer -= 1;
            } else {
                self.player_dead = false;
                self.player_hp = self.max_hp();
                self.player_mp = self.max_mp();
                self.attack_target = None;
                self.move_target = None;
                // Respawn at campfire in Rogue Camp
                if self.current_zone == ZoneId::RogueCamp {
                    self.player_pos = [50.0, 50.0];
                } else {
                    self.transition_zone(ZoneId::RogueCamp, [50.0, 50.0]);
                }
            }
            return;
        }
        #[allow(clippy::cast_precision_loss)]
        if self.player_hp <= 0.0 {
            self.player_dead = true;
            self.player_respawn_timer = RESPAWN_DELAY;
            self.player_hp = 0.0;
            self.attack_target = None;
            self.move_target = None;
            self.poison_timer = 0;
            // XP penalty: lose 10%
            let penalty = self.player_xp * 0.10;
            self.player_xp = (self.player_xp - penalty).max(0.0);
            if penalty > 0.0 {
                self.push_log(b"DEATH PENALTY: -10% XP", [1.0, 0.3, 0.3, 1.0]);
            }
        }
    }

    #[allow(clippy::cast_precision_loss)]
    fn update_combat(&mut self) {
        if self.player_dead { return; }
        if self.attack_cd > 0 { self.attack_cd -= 1; }
        if let Some(idx) = self.attack_target {
            if idx < self.enemies.len() && self.enemies[idx].alive {
                let d = Self::dist(self.player_pos, self.enemies[idx].pos);
                if d <= ATTACK_RANGE && self.attack_cd == 0 {
                    let dmg = self.melee_damage() * if self.damage_buff_active() { 1.5 } else { 1.0 };
                    self.enemies[idx].hp -= dmg;
                    self.attack_cd = self.attack_cooldown();
                    self.floats.push(DamageFloat {
                        wx: self.enemies[idx].pos[0], wy: self.enemies[idx].pos[1],
                        dy: 0.0, value: dmg, is_heal: false, life: 40,
                    });
                    // Knockback chance on melee
                    let kb_hash = (self.frame.wrapping_mul(1_103_515_245) >> 16) as f32 / 65536.0;
                    if kb_hash < KNOCKBACK_CHANCE && !self.enemies[idx].is_boss() {
                        let dx = self.enemies[idx].pos[0] - self.player_pos[0];
                        let dy = self.enemies[idx].pos[1] - self.player_pos[1];
                        let kd = (dx * dx + dy * dy).sqrt().max(0.01);
                        self.enemies[idx].pos[0] = (self.enemies[idx].pos[0] + dx / kd * KNOCKBACK_FORCE).clamp(0.5, MAP_W as f32 - 0.5);
                        self.enemies[idx].pos[1] = (self.enemies[idx].pos[1] + dy / kd * KNOCKBACK_FORCE).clamp(0.5, MAP_H as f32 - 0.5);
                    }
                    // Life steal
                    let ls = self.equip_life_steal();
                    if ls > 0.0 {
                        let heal = dmg * ls;
                        self.player_hp = (self.player_hp + heal).min(self.max_hp());
                    }
                    // Mana steal
                    let ms = self.equip_mana_steal();
                    if ms > 0.0 {
                        let mana = dmg * ms;
                        self.player_mp = (self.player_mp + mana).min(self.max_mp());
                    }
                    if self.enemies[idx].hp <= 0.0 {
                        self.kill_enemy(idx);
                        self.attack_target = None;
                    }
                } else if d > ATTACK_RANGE {
                    let spd = if self.running { CLICK_RUN_SPEED } else { CLICK_MOVE_SPEED };
                    Self::move_toward(&mut self.player_pos, self.enemies[idx].pos, spd);
                }
            } else {
                self.attack_target = None;
            }
        }
        let pp = self.player_pos;
        let mut phys_damage = 0.0_f32;
        let mut elem_damage = 0.0_f32;
        let mut elem_type = Element::Physical;
        let mut hits = 0_u32;
        let mut got_poisoned = false;
        let mut got_cursed = false;
        let diff_mult = self.difficulty.monster_dmg_mult();
        for enemy in &mut self.enemies {
            if !enemy.alive || enemy.attack_cd > 0 || enemy.freeze_timer > 0 { continue; }
            if Self::dist(enemy.pos, pp) <= ENEMY_ATTACK_RANGE {
                let dmg = enemy.attack_damage() * diff_mult;
                let atk_elem = enemy.attack_element();
                if atk_elem == Element::Physical {
                    phys_damage += dmg;
                } else {
                    elem_damage += dmg;
                    elem_type = atk_elem;
                }
                enemy.attack_cd = ENEMY_ATTACK_CD;
                hits += 1;
                if matches!(enemy.kind, EnemyKind::Zombie | EnemyKind::FallenShaman) {
                    got_poisoned = true;
                }
                if enemy.affix == Some(MonsterAffix::Cursed) {
                    got_cursed = true;
                }
            }
        }
        if got_poisoned && self.poison_timer == 0 {
            self.poison_timer = 180;
            self.push_log(b"POISONED!", C_POISON);
        }
        if got_cursed {
            self.push_log(b"CURSED!", [0.70, 0.20, 0.20, 1.0]);
        }
        if hits > 0 {
            let dodge = self.dodge_chance();
            let hash = (self.frame.wrapping_mul(2_971_215_073) >> 16) as f32 / 65536.0;
            if hash < dodge {
                self.floats.push(DamageFloat {
                    wx: pp[0], wy: pp[1], dy: 0.0, value: 0.0, is_heal: false, life: 30,
                });
            } else {
                // Physical damage reduced by armor/VIT
                let phys = (phys_damage - self.damage_reduction()).max(0.0);
                // Elemental damage reduced by resistances
                let eff_res = self.effective_resistances();
                let elem = eff_res.apply(elem_damage, elem_type);
                let total = (phys + elem).max(1.0);
                self.player_hp -= total;
                self.floats.push(DamageFloat {
                    wx: pp[0], wy: pp[1], dy: 0.0, value: total, is_heal: false, life: 40,
                });
            }
        }
        // Poison tick
        if self.poison_timer > 0 {
            self.poison_timer -= 1;
            self.player_hp -= POISON_TICK_DMG;
        }
        // Buff timers
        self.buffs.retain_mut(|b| { b.remaining = b.remaining.saturating_sub(1); b.remaining > 0 });
        // Health shrine: regen boost
        let shrine_regen = if self.buffs.iter().any(|b| b.kind == ShrineKind::Health) { 0.05 } else { 0.0 };
        let mhp = self.max_hp();
        let mmp = self.max_mp();
        self.player_hp = (self.player_hp + self.hp_regen() + shrine_regen).clamp(0.0, mhp);
        self.player_mp = (self.player_mp + self.mp_regen()).min(mmp);
        // Lightning bolt visuals
        self.lightning_bolts.retain_mut(|b| { b.life = b.life.saturating_sub(1); b.life > 0 });
        self.floats.retain_mut(|f| {
            f.life = f.life.saturating_sub(1);
            f.dy -= 0.02;
            f.life > 0
        });
    }

    fn transition_zone(&mut self, target: ZoneId, spawn: [f32; 2]) {
        // Save current zone data
        let zi = self.current_zone as usize;
        self.zone_enemies[zi] = std::mem::take(&mut self.enemies);
        self.zone_breakables[zi] = std::mem::take(&mut self.breakables);
        self.zone_shrines[zi] = std::mem::take(&mut self.shrines);
        self.zone_chests[zi] = std::mem::take(&mut self.chests);
        self.zone_gather_nodes[zi] = std::mem::take(&mut self.gather_nodes);
        // Load target zone
        self.current_zone = target;
        let ti = target as usize;
        self.enemies = self.zone_enemies[ti].clone();
        self.breakables = self.zone_breakables[ti].clone();
        self.shrines = self.zone_shrines[ti].clone();
        self.chests = self.zone_chests[ti].clone();
        self.gather_nodes = self.zone_gather_nodes[ti].clone();
        // Update world offset for seamless positioning
        self.world_offset = zone_coord(target).world_origin();
        self.player_pos = spawn;
        self.cam_focus = spawn;
        self.move_target = None;
        self.attack_target = None;
        self.harvest_target = None;
        self.harvest_cd = 0;
        self.drops.clear();
        self.projectiles.clear();
        self.floats.clear();
        self.lightning_bolts.clear();
        self.npc_interact = None;
        self.zone_cd = 30;
        self.apply_enemy_modifiers();
    }

    /// Assign random affixes to some enemies and apply difficulty scaling.
    #[allow(clippy::cast_possible_truncation)]
    fn apply_enemy_modifiers(&mut self) {
        let affixes = [
            MonsterAffix::ExtraStrong,
            MonsterAffix::FireEnchanted,
            MonsterAffix::ColdEnchanted,
            MonsterAffix::LightningEnchanted,
            MonsterAffix::Cursed,
        ];
        for (i, enemy) in self.enemies.iter_mut().enumerate() {
            if !enemy.alive || enemy.is_boss() { continue; }
            // ~20% chance of affix on non-boss enemies
            let h = (i as u32).wrapping_mul(2_654_435_761).wrapping_add(self.frame);
            if h.is_multiple_of(5) {
                let affix = affixes[(h / 5) as usize % affixes.len()];
                enemy.affix = Some(affix);
            }
            enemy.apply_modifiers(self.difficulty);
        }
        // Bosses get difficulty scaling but no random affix
        for enemy in &mut self.enemies {
            if enemy.alive && enemy.is_boss() {
                enemy.apply_modifiers(self.difficulty);
            }
        }
    }

    /// Seamless zone transition: when the player walks out of the current
    /// chunk in world-space, detect which zone they entered and swap data.
    #[allow(clippy::cast_precision_loss)]
    fn check_zone_transition(&mut self) {
        if self.zone_cd > 0 { self.zone_cd -= 1; return; }
        let world_pos = self.player_world_pos();
        // Check if the player is still inside the current chunk
        let cur_coord = zone_coord(self.current_zone);
        if cur_coord.contains(world_pos) {
            return; // still in current zone, nothing to do
        }
        // Find which zone the player walked into
        if let Some(target_zone) = world_map::zone_at_world(world_pos) {
            if target_zone == self.current_zone { return; }
            // Convert world position to local coords in the new zone
            let target_coord = zone_coord(target_zone);
            let local = target_coord.to_local(world_pos);
            self.transition_zone_seamless(target_zone, local);
        }
    }

    // clamp_player_to_world_bounds removed — global terrain handles world edges.

    /// Seamless transition: swap zone data while preserving continuous position.
    fn transition_zone_seamless(&mut self, target: ZoneId, local_pos: [f32; 2]) {
        // Save current zone data
        let zi = self.current_zone as usize;
        self.zone_enemies[zi] = std::mem::take(&mut self.enemies);
        self.zone_breakables[zi] = std::mem::take(&mut self.breakables);
        self.zone_shrines[zi] = std::mem::take(&mut self.shrines);
        self.zone_chests[zi] = std::mem::take(&mut self.chests);
        self.zone_gather_nodes[zi] = std::mem::take(&mut self.gather_nodes);
        // Load target zone
        self.current_zone = target;
        let ti = target as usize;
        self.enemies = self.zone_enemies[ti].clone();
        self.breakables = self.zone_breakables[ti].clone();
        self.shrines = self.zone_shrines[ti].clone();
        self.chests = self.zone_chests[ti].clone();
        self.gather_nodes = self.zone_gather_nodes[ti].clone();
        // Update world offset and position (continuous — no teleport)
        self.world_offset = zone_coord(target).world_origin();
        self.player_pos = local_pos;
        // Keep move target relative if active
        self.move_target = None;
        self.attack_target = None;
        self.harvest_target = None;
        self.harvest_cd = 0;
        // Keep projectiles/drops/floats alive briefly for visual continuity
        self.projectiles.clear();
        self.lightning_bolts.clear();
        self.npc_interact = None;
        self.zone_cd = 10; // shorter cooldown for seamless feel
        self.apply_enemy_modifiers();
    }

    fn update_enemy_ranged(&mut self) {
        let pp = self.player_pos;
        let frame = self.frame;
        // Collect ranged attack data to avoid borrow issues
        let shots: Vec<_> = self.enemies.iter_mut().filter_map(|enemy| {
            if !enemy.alive || !enemy.is_ranged() || enemy.attack_cd > 0 || enemy.freeze_timer > 0 { return None; }
            let d = Self::dist(enemy.pos, pp);
            if !(ENEMY_ATTACK_RANGE..=ENEMY_RANGED_RANGE).contains(&d) { return None; }
            // Fire rate varies by type
            let fire_rate = match enemy.kind {
                EnemyKind::FallenShaman => 180,
                EnemyKind::BloodRaven => 100,
                _ => 140,
            };
            if !frame.is_multiple_of(fire_rate) { return None; }
            enemy.attack_cd = fire_rate;
            let dx = pp[0] - enemy.pos[0];
            let dy = pp[1] - enemy.pos[1];
            Some((enemy.pos, [dx / d, dy / d], enemy.attack_damage()))
        }).collect();
        for (pos, dir, dmg) in shots {
            self.projectiles.push(Projectile {
                pos, dir, dist_left: ENEMY_RANGED_RANGE,
                damage: dmg, is_frost: false, friendly: false,
            });
        }
    }

    fn update_enemy_projectile_hits(&mut self) {
        // Check enemy projectiles hitting player
        let pp = self.player_pos;
        let mut total_dmg = 0.0_f32;
        self.projectiles.retain(|p| {
            if p.friendly { return true; }
            if Self::dist(p.pos, pp) < 0.6 {
                total_dmg += p.damage;
                return false;
            }
            true
        });
        #[allow(clippy::cast_precision_loss)]
        if total_dmg > 0.0 {
            // Dodge check (same pseudo-random as melee)
            let dodge = self.dodge_chance();
            let hash = (self.frame.wrapping_mul(2_654_435_761) >> 16) as f32 / 65536.0;
            if hash < dodge {
                self.floats.push(DamageFloat {
                    wx: pp[0], wy: pp[1], dy: 0.0, value: 0.0, is_heal: false, life: 30,
                });
            } else {
                let reduced = (total_dmg - self.damage_reduction()).max(1.0);
                self.player_hp -= reduced;
                self.floats.push(DamageFloat {
                    wx: pp[0], wy: pp[1], dy: 0.0, value: reduced, is_heal: false, life: 40,
                });
            }
        }
    }

    fn update_quests(&mut self) {
        // Quest 0: Den of Evil — all enemies in den dead
        if !self.quests[0].done {
            let den_enemies = &self.zone_enemies[ZoneId::DenOfEvil as usize];
            // Check if current zone is den; if so, use self.enemies
            let all_dead = if self.current_zone == ZoneId::DenOfEvil {
                self.enemies.iter().all(|e| !e.alive)
            } else {
                den_enemies.iter().all(|e| !e.alive)
            };
            if all_dead && !den_enemies.is_empty() {
                self.quests[0].done = true;
            }
        }
        // Quest 1: Blood Raven — killed in cold plains
        if !self.quests[1].done {
            let check_enemies = if self.current_zone == ZoneId::ColdPlainsN {
                &self.enemies
            } else {
                &self.zone_enemies[ZoneId::ColdPlainsN as usize]
            };
            if check_enemies.iter().any(|e| e.kind == EnemyKind::BloodRaven && !e.alive) {
                self.quests[1].done = true;
            }
        }
        // Quest 2: Collect 100 gold
        if !self.quests[2].done && self.player_gold >= 100 {
            self.quests[2].done = true;
        }
    }

    fn waypoint_travel(&mut self, zone_idx: usize) {
        if zone_idx >= ZONE_COUNT { return; }
        if !self.waypoints[zone_idx] { return; }
        if zone_idx >= ZONE_COUNT { return; }
        let target = ZoneId::from_index(zone_idx);
        if target == self.current_zone {
            self.show_waypoint_menu = false;
            return;
        }
        self.show_waypoint_menu = false;
        self.transition_zone(target, [50.0, 50.0]);
        self.push_log(b"WAYPOINT TRAVEL", C_WP_CORE);
    }

    fn use_town_portal(&mut self) {
        if self.is_camp() { return; } // already in camp
        if self.portal_active && self.portal_zone == self.current_zone {
            // Enter existing portal → go to camp
            let d = Self::dist(self.player_pos, self.portal_pos);
            if d < 1.5 {
                self.portal_active = false;
                self.transition_zone(ZoneId::RogueCamp, [50.0, 50.0]);
                return;
            }
        }
        // Create new portal at player position
        self.portal_active = true;
        self.portal_pos = self.player_pos;
        self.portal_zone = self.current_zone;
    }

    fn check_portal_entry(&mut self) {
        if !self.portal_active { return; }
        // In camp, check if player walks into portal → return to portal zone
        if self.is_camp() && self.portal_zone != ZoneId::RogueCamp {
            let portal_in_camp = [12.0_f32, 13.0]; // portal appears near campfire in camp
            if Self::dist(self.player_pos, portal_in_camp) < 1.5 {
                let target = self.portal_zone;
                let pos = self.portal_pos;
                self.portal_active = false;
                self.transition_zone(target, pos);
            }
        }
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
    fn attack_breakable(&mut self) {
        let pp = self.player_pos;
        for breakable in &mut self.breakables {
            if !breakable.alive { continue; }
            if Self::dist(pp, breakable.pos) < ATTACK_RANGE && self.attack_cd == 0 {
                breakable.alive = false;
                self.attack_cd = 10;
                let h = (breakable.pos[0] * 333.0 + breakable.pos[1] * 777.0 + self.frame as f32) as u32;
                self.drops.push(ItemDrop {
                    pos: breakable.pos,
                    kind: DropKind::Gold,
                    gold_amount: 3 + h % 8,
                    life: 600,
                });
                if h.is_multiple_of(3) {
                    self.drops.push(ItemDrop {
                        pos: [breakable.pos[0] + 0.3, breakable.pos[1]],
                        kind: DropKind::HealthPotion,
                        gold_amount: 0,
                        life: 600,
                    });
                }
                return; // only break one per click
            }
        }
    }

    fn cycle_difficulty(&mut self) {
        self.difficulty = match self.difficulty {
            Difficulty::Normal => Difficulty::Nightmare,
            Difficulty::Nightmare => Difficulty::Hell,
            Difficulty::Hell => Difficulty::Normal,
        };
        let label = self.difficulty.label();
        let mut msg = [0u8; 40];
        let prefix = b"DIFFICULTY: ";
        msg[..prefix.len()].copy_from_slice(prefix);
        let end = (prefix.len() + label.len()).min(40);
        msg[prefix.len()..end].copy_from_slice(&label[..end - prefix.len()]);
        self.push_log(&msg[..end], [1.0, 0.60, 0.20, 1.0]);
    }

    fn allocate_stat(&mut self, stat: u8) {
        if self.stat_points == 0 { return; }
        self.stat_points -= 1;
        match stat {
            0 => self.stat_str += 1,
            1 => self.stat_dex += 1,
            2 => self.stat_vit += 1,
            3 => self.stat_ene += 1,
            _ => { self.stat_points += 1; }
        }
    }

    /// Unlock a grid slot (costs 1 unlock point per level).
    fn unlock_grid_slot(&mut self, col: usize, row: usize) {
        if self.grid_unlock_points == 0 { return; }
        if !self.skill_grid.can_unlock(col, row) { return; }
        self.skill_grid.unlock(col, row);
        self.grid_unlock_points -= 1;
        self.push_log(b"SLOT UNLOCKED", [0.40, 0.80, 1.0, 1.0]);
    }

    /// Insert a scroll from inventory into a grid slot.
    fn insert_scroll(&mut self, col: usize, row: usize, inv_idx: usize) {
        if inv_idx >= self.scroll_inventory.len() { return; }
        let scroll = self.scroll_inventory[inv_idx];
        if self.skill_grid.insert(col, row, scroll.content) {
            self.scroll_inventory.remove(inv_idx);
            self.disk_bonuses = self.skill_grid.compute_bonuses();
            let label = scroll.content.short_label();
            let mut msg = [0u8; 40];
            msg[..7].copy_from_slice(b"INSERT ");
            let end = (7 + label.len()).min(40);
            msg[7..end].copy_from_slice(&label[..end - 7]);
            self.push_log(&msg[..end], [0.40, 0.80, 1.0, 1.0]);
        }
    }

    /// Remove a scroll from a grid slot back to inventory.
    fn remove_scroll(&mut self, col: usize, row: usize) {
        if let Some(content) = self.skill_grid.remove(col, row) {
            self.scroll_inventory.push(skill_disk::ScrollItem { content });
            self.disk_bonuses = self.skill_grid.compute_bonuses();
            self.push_log(b"SCROLL REMOVED", [0.7, 0.65, 0.55, 0.8]);
        }
    }

    /// Total equipment stat bonuses: (str, dex, vit, ene)
    fn equip_bonuses(&self) -> (u32, u32, u32, u32) {
        let mut s = 0u32;
        let mut d = 0u32;
        let mut v = 0u32;
        let mut e = 0u32;
        for eq in self.equipment.iter().flatten() {
            let (bs, bd, bv, be) = eq.bonuses();
            s += bs; d += bd; v += bv; e += be;
        }
        (s, d, v, e)
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn total_str(&self) -> u32 { self.stat_str + self.equip_bonuses().0 + self.disk_bonuses.bonus_str as u32 }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn total_dex(&self) -> u32 { self.stat_dex + self.equip_bonuses().1 + self.disk_bonuses.bonus_dex as u32 }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn total_vit(&self) -> u32 { self.stat_vit + self.equip_bonuses().2 + self.disk_bonuses.bonus_vit as u32 }
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn total_ene(&self) -> u32 { self.stat_ene + self.equip_bonuses().3 + self.disk_bonuses.bonus_ene as u32 }

    // === Aggregate equipment special effects ===
    fn equip_flat_damage(&self) -> f32 {
        self.equipment.iter().flatten().map(|e| e.flat_damage()).sum()
    }
    fn equip_flat_defense(&self) -> f32 {
        self.equipment.iter().flatten().map(|e| e.flat_defense()).sum()
    }
    fn equip_hp_percent(&self) -> f32 {
        self.equipment.iter().flatten().map(|e| e.hp_percent()).sum()
    }
    fn equip_move_speed(&self) -> f32 {
        self.equipment.iter().flatten().map(|e| e.move_speed_bonus()).sum()
    }
    fn equip_cd_reduction(&self) -> u32 {
        self.equipment.iter().flatten().map(|e| e.cooldown_reduction()).sum()
    }
    fn equip_spell_percent(&self) -> f32 {
        self.equipment.iter().flatten().map(|e| e.spell_percent()).sum()
    }
    fn equip_life_steal(&self) -> f32 {
        let base: f32 = self.equipment.iter().flatten().map(|e| e.life_steal()).sum();
        let mult = if self.disk_bonuses.vaal_pact { 2.0 } else { 1.0 };
        (base + self.disk_bonuses.life_steal_pct) * mult
    }
    fn equip_mana_steal(&self) -> f32 {
        let base: f32 = self.equipment.iter().flatten().map(|e| e.mana_steal()).sum();
        base + self.disk_bonuses.mana_steal_pct
    }
    fn walk_speed(&self) -> f32 {
        (PLAYER_SPEED + self.equip_move_speed()) * (1.0 + self.disk_bonuses.move_speed_pct)
    }
    fn run_speed(&self) -> f32 {
        (PLAYER_RUN_SPEED + self.equip_move_speed()) * (1.0 + self.disk_bonuses.move_speed_pct)
    }

    // === STR: +melee damage, +weapon flat damage ===
    #[allow(clippy::cast_precision_loss)]
    fn melee_damage(&self) -> f32 {
        let str_bonus = self.total_str() as f32 * 0.2;
        let base = ATTACK_DAMAGE + str_bonus + self.equip_flat_damage();
        // Disk passive: melee_dmg_pct from allocated nodes
        let disk_mult = 1.0 + self.disk_bonuses.melee_dmg_pct;
        base * disk_mult
    }

    // === DEX: -attack cooldown, +dodge, +gloves CD reduction ===
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn attack_cooldown(&self) -> u32 {
        let base_reduction = (self.total_dex() as f32 * 0.6) as u32 + self.equip_cd_reduction();
        let cd = ATTACK_COOLDOWN_BASE.saturating_sub(base_reduction);
        // Disk attack speed reduces cooldown by percentage
        let cd_f = cd as f32 * (1.0 - self.disk_bonuses.attack_speed_pct);
        (cd_f as u32).max(15)
    }
    #[allow(clippy::cast_precision_loss)]
    fn dodge_chance(&self) -> f32 {
        let base = self.total_dex() as f32 * 0.008;
        (base + self.disk_bonuses.dodge_pct).min(0.60) // raised cap to 60% for disk builds
    }

    // === VIT: +HP, +HP regen, +helm HP% ===
    #[allow(clippy::cast_precision_loss)]
    fn max_hp(&self) -> f32 {
        let base = PLAYER_MAX_HP + self.total_vit() as f32 * 2.0
            + (self.player_level - 1) as f32 * 1.5
            + self.disk_bonuses.max_hp_flat;
        let hp = base * (1.0 + self.equip_hp_percent() + self.disk_bonuses.max_hp_pct);
        // Keystone: Glass Cannon = -30% max HP (already in max_hp_pct as negative)
        // Keystone: Unbreakable = +30% HP (already in max_hp_pct)
        hp.max(1.0)
    }
    #[allow(clippy::cast_precision_loss)]
    fn hp_regen(&self) -> f32 {
        let base = HP_REGEN_BASE + self.total_vit() as f32 * 0.001 + self.disk_bonuses.hp_regen;
        // Keystone: Vaal Pact — no HP regen (negative hp_regen from disk bonuses)
        base.max(0.0)
    }

    // === ENE: +MP, +MP regen, +spell damage ===
    #[allow(clippy::cast_precision_loss)]
    fn max_mp(&self) -> f32 {
        let base = PLAYER_MAX_MP + self.total_ene() as f32 * 1.5
            + (self.player_level - 1) as f32 * 1.0
            + self.disk_bonuses.max_mp_flat;
        base * (1.0 + self.disk_bonuses.max_mp_pct)
    }
    #[allow(clippy::cast_precision_loss)]
    fn mp_regen(&self) -> f32 {
        MP_REGEN_BASE + self.total_ene() as f32 * 0.002 + self.disk_bonuses.mp_regen
    }
    #[allow(clippy::cast_precision_loss)]
    fn spell_bonus(&self) -> f32 {
        1.0 + self.total_ene() as f32 * 0.02 + self.equip_spell_percent()
            + self.disk_bonuses.spell_dmg_pct
    }
    #[allow(clippy::cast_precision_loss)]
    fn fireball_damage(&self) -> f32 {
        FIREBALL_DAMAGE * self.spell_bonus() * self.disk_bonuses.fire_mult()
    }
    #[allow(clippy::cast_precision_loss)]
    fn frost_bolt_damage(&self) -> f32 {
        FROST_BOLT_DAMAGE * self.spell_bonus() * self.disk_bonuses.cold_mult()
    }
    #[allow(clippy::cast_precision_loss)]
    fn chain_lightning_damage(&self) -> f32 {
        CHAIN_LIGHTNING_DAMAGE * self.spell_bonus() * self.disk_bonuses.lightning_mult()
    }
    fn xp_multiplier(&self) -> f32 {
        if self.buffs.iter().any(|b| b.kind == ShrineKind::Experience) { 1.5 } else { 1.0 }
    }
    fn damage_buff_active(&self) -> bool {
        self.buffs.iter().any(|b| b.kind == ShrineKind::Damage)
    }

    /// Damage reduction from VIT + armor + disk passives
    #[allow(clippy::cast_precision_loss)]
    fn damage_reduction(&self) -> f32 {
        (self.total_vit() as f32 * 0.08 + self.equip_flat_defense() + self.disk_bonuses.armor_flat).max(0.0)
    }

    fn push_log(&mut self, msg: &[u8], color: [f32; 4]) {
        self.log.push(LogEntry::new(msg, color));
        if self.log.len() > LOG_MAX { self.log.remove(0); }
    }

    fn push_log_equip(&mut self, slot: EquipSlot, rarity: ItemRarity) {
        let mut msg = [0u8; 40];
        let prefix = b"EQUIPPED ";
        msg[..prefix.len()].copy_from_slice(prefix);
        let rn = rarity.name();
        let n = prefix.len();
        let e1 = (n + rn.len()).min(40);
        msg[n..e1].copy_from_slice(&rn[..e1 - n]);
        let sn = slot.name();
        let e2 = (e1 + sn.len()).min(40);
        msg[e1..e2].copy_from_slice(&sn[..e2 - e1]);
        self.push_log(&msg[..e2], rarity.color());
    }

    fn update_log(&mut self) {
        for entry in &mut self.log {
            entry.life = entry.life.saturating_sub(1);
        }
        self.log.retain(|e| e.life > 0);
    }

    fn update_npc_interact(&mut self) {
        if self.interact_timer > 0 {
            self.interact_timer -= 1;
            if self.interact_timer == 0 { self.npc_interact = None; }
        }
    }

    fn find_enemy_at_world(&self, wx: f32, wy: f32) -> Option<usize> {
        self.enemies.iter().enumerate()
            .find(|(_, e)| e.alive && Self::dist(e.pos, [wx, wy]) < 0.8)
            .map(|(i, _)| i)
    }

    #[allow(clippy::unused_self)]
    fn find_npc_at_world(&self, wx: f32, wy: f32) -> Option<usize> {
        NPCS.iter().enumerate()
            .find(|(_, npc)| Self::dist(npc.pos, [wx, wy]) < 1.0)
            .map(|(i, _)| i)
    }

    /// Returns true if the screen position is over an open UI panel (inventory, etc.)
    fn mouse_over_ui(&self, sx: f32, sy: f32) -> bool {
        if self.show_inventory {
            let vw = self.viewport[0];
            let vh = self.viewport[1];
            let pw = 240.0;
            let h = (vh - 16.0).min(540.0);
            let y = 8.0;
            // Left panel
            if sx >= 2.0 && sx <= 4.0 + pw + 2.0 && sy >= y - 2.0 && sy <= y + h + 2.0 {
                return true;
            }
            // Right panel
            let rx = vw - pw - 4.0;
            if sx >= rx - 2.0 && sx <= vw - 2.0 && sy >= y - 2.0 && sy <= y + h + 2.0 {
                return true;
            }
        }
        false
    }

    #[allow(clippy::cast_precision_loss)]
    fn handle_click(&mut self, sx: f32, sy: f32) {
        if self.mouse_over_ui(sx, sy) { return; }
        let [wx, wy] = self.s2w(sx, sy);
        if let Some(idx) = self.find_enemy_at_world(wx, wy) {
            self.attack_target = Some(idx);
            self.move_target = None;
            return;
        }
        if self.is_camp() {
            if let Some(idx) = self.find_npc_at_world(wx, wy) {
                let d = Self::dist(self.player_pos, NPCS[idx].pos);
                if d <= NPC_INTERACT_RANGE {
                    self.npc_interact = Some(idx);
                    self.interact_timer = 180;
                    self.attack_target = None;
                    self.move_target = None;
                    self.npc_dialog = Some(idx);
                    match idx {
                        0 => { // Akara: heal + scroll shop
                            if self.player_gold >= SCROLL_COST && self.scroll_inventory.len() < 24 {
                                // Buy a random scroll
                                self.player_gold -= SCROLL_COST;
                                let seed = self.frame.wrapping_mul(2_654_435_761).wrapping_add(self.player_level);
                                let content = random_passive_scroll(seed, self.player_level);
                                self.scroll_inventory.push(skill_disk::ScrollItem { content });
                                let label = content.short_label();
                                let mut msg = [0u8; 40];
                                msg[..14].copy_from_slice(b"AKARA: SCROLL ");
                                let end = (14 + label.len()).min(40);
                                msg[14..end].copy_from_slice(&label[..end - 14]);
                                self.push_log(&msg[..end], [0.70, 0.85, 1.0, 1.0]);
                            } else {
                                // Fallback: heal + cure poison
                                self.player_hp = self.max_hp();
                                self.player_mp = self.max_mp();
                                self.poison_timer = 0;
                                self.floats.push(DamageFloat {
                                    wx: self.player_pos[0], wy: self.player_pos[1],
                                    dy: 0.0, value: self.max_hp(), is_heal: true, life: 50,
                                });
                                self.push_log(b"AKARA: BE HEALED", [0.40, 1.0, 0.40, 1.0]);
                            }
                        }
                        1 => { // Gheed: gambling (30g for random item) + potions
                            if self.player_gold >= GAMBLE_COST {
                                self.gheed_gamble();
                            } else {
                                // Fallback: sell potions
                                let cost = 10_u32;
                                for slot in &mut self.belt {
                                    if slot.is_none() && self.player_gold >= cost {
                                        *slot = Some(PotionKind::Health);
                                        self.player_gold -= cost;
                                    }
                                }
                                self.push_log(b"GHEED: POTIONS ONLY", C_GOLD_DROP);
                            }
                        }
                        2 => { // Charsi: upgrade equipment (60g)
                            if self.player_gold >= UPGRADE_COST {
                                self.charsi_upgrade();
                            } else {
                                self.player_hp = self.max_hp();
                                self.push_log(b"CHARSI: ALL FIXED", C_STATS_TEXT);
                            }
                        }
                        3 => { // Kashya: hire merc or quest info
                            let merc_t = MercType::Rogue; // default; TODO: selection UI
                            let cost = merc_t.hire_cost();
                            if self.mercenary.is_none() && self.player_gold >= cost {
                                self.player_gold -= cost;
                                self.mercenary = Some(Mercenary::new(self.player_pos, merc_t));
                                self.active_aura = merc_t.aura();
                                self.push_log(b"KASHYA: MERC HIRED", C_QUEST_ACTIVE);
                            } else {
                                self.show_quests = true;
                                self.push_log(b"KASHYA: CHECK QUESTS", C_QUEST_ACTIVE);
                            }
                        }
                        _ => {}
                    }
                    return;
                }
                self.move_target = Some(NPCS[idx].pos);
                self.attack_target = None;
                return;
            }
        }
        let clamped = [wx.clamp(0.5, MAP_W as f32 - 0.5), wy.clamp(0.5, MAP_H as f32 - 0.5)];
        self.move_target = Some(clamped);
        self.attack_target = None;
    }

    // ======================================================================
    // Mercenary AI
    // ======================================================================

    #[allow(clippy::cast_precision_loss)]
    fn update_mercenary(&mut self) {
        let Some(merc) = &mut self.mercenary else { return };
        if !merc.alive {
            merc.respawn_timer = merc.respawn_timer.saturating_sub(1);
            if merc.respawn_timer == 0 {
                merc.alive = true;
                merc.hp = merc.max_hp;
                merc.pos = self.player_pos;
            }
            return;
        }
        // Follow player
        let pp = self.player_pos;
        let d = Self::dist(merc.pos, pp);
        let merc_speed = merc.merc_type.base_speed();
        if d > 2.0 {
            Self::move_toward(&mut merc.pos, pp, merc_speed);
        }
        // Attack nearest enemy
        if merc.attack_cd > 0 { merc.attack_cd -= 1; }
        let merc_pos = merc.pos;
        let merc_cd = merc.attack_cd;
        let merc_range = self.mercenary.as_ref().map_or(1.5, |m| m.merc_type.attack_range());
        let merc_dmg_base = self.mercenary.as_ref().map_or(4.0, |m| m.effective_damage());
        // Find nearest enemy
        let mut target_idx = None;
        let mut min_dist = merc_range;
        for (i, e) in self.enemies.iter().enumerate() {
            if !e.alive { continue; }
            let ed = Self::dist(merc_pos, e.pos);
            if ed < min_dist {
                min_dist = ed;
                target_idx = Some(i);
            }
        }
        let dmg_buff = self.damage_buff_active();
        let aura_dmg = if self.active_aura == Some(AuraKind::Might) { 1.0 + AURA_MIGHT_DMG_PCT } else { 1.0 };
        if let Some(idx) = target_idx {
            if merc_cd == 0 {
                let dmg = merc_dmg_base * if dmg_buff { 1.5 } else { 1.0 } * aura_dmg;
                self.enemies[idx].hp -= dmg;
                if let Some(merc) = &mut self.mercenary { merc.attack_cd = MERC_ATTACK_CD; }
                self.floats.push(DamageFloat {
                    wx: self.enemies[idx].pos[0], wy: self.enemies[idx].pos[1],
                    dy: 0.0, value: dmg, is_heal: false, life: 30,
                });
                if self.enemies[idx].hp <= 0.0 {
                    // Merc gets XP and kill credit
                    let xp = self.enemies[idx].xp_value() * 0.5;
                    if let Some(merc) = &mut self.mercenary {
                        merc.kills += 1;
                        merc.add_xp(xp);
                    }
                    self.kill_enemy(idx);
                }
            } else if min_dist > 1.0 {
                if let Some(merc) = &mut self.mercenary {
                    Self::move_toward(&mut merc.pos, self.enemies[idx].pos, merc_speed);
                }
            }
        }
        // Merc takes damage from nearby enemies
        let merc_pos2 = self.mercenary.as_ref().map_or([0.0, 0.0], |m| m.pos);
        let mut merc_dmg = 0.0_f32;
        for enemy in &self.enemies {
            if !enemy.alive { continue; }
            if Self::dist(enemy.pos, merc_pos2) <= ENEMY_ATTACK_RANGE && enemy.attack_cd == 0 {
                merc_dmg += enemy.attack_damage() * 0.5;
            }
        }
        if let Some(merc) = &mut self.mercenary {
            if merc_dmg > 0.0 {
                merc.hp -= merc_dmg;
                if merc.hp <= 0.0 {
                    merc.alive = false;
                    merc.respawn_timer = 300;
                }
            }
            if merc.alive { merc.hp = (merc.hp + 0.015).min(merc.max_hp); }
        }
        if merc_dmg > 0.0 {
            if let Some(merc) = &self.mercenary {
                if !merc.alive {
                    self.push_log(b"MERCENARY DIED", [1.0, 0.3, 0.3, 1.0]);
                }
            }
        }
    }

    // ======================================================================
    // Summons AI
    // ======================================================================

    fn spawn_summon(&mut self, kind: SummonKind) {
        if self.summons.len() >= SUMMON_MAX { return; }
        let cost = kind.mana_cost();
        if self.player_mp < cost { return; }
        self.player_mp -= cost;
        let offset = self.summons.len() as f32 * 0.5;
        let pos = [self.player_pos[0] + offset, self.player_pos[1] + 1.0];
        let spell_pct = self.disk_bonuses.spell_dmg_pct;
        self.summons.push(Summon::new(kind, pos, spell_pct));
        self.push_log(kind.name(), [0.6, 0.8, 0.5, 1.0]);
    }

    #[allow(clippy::cast_precision_loss)]
    fn update_summons(&mut self) {
        let pp = self.player_pos;
        // Snapshot enemy positions for borrow-safe logic
        let enemy_count = self.enemies.len();
        let mut enemy_alive = vec![false; enemy_count];
        let mut enemy_pos = vec![[0.0f32; 2]; enemy_count];
        for (i, e) in self.enemies.iter().enumerate() {
            enemy_alive[i] = e.alive;
            enemy_pos[i] = e.pos;
        }

        let aura_dmg = if self.active_aura == Some(AuraKind::Might) { 1.0 + AURA_MIGHT_DMG_PCT } else { 1.0 };

        for si in 0..self.summons.len() {
            if !self.summons[si].alive { continue; }
            // Follow player loosely
            let spos = self.summons[si].pos;
            let d = Self::dist(spos, pp);
            if d > 5.0 {
                let spd = self.summons[si].speed();
                Self::move_toward(&mut self.summons[si].pos, pp, spd);
                continue; // don't attack while catching up
            }
            // Find nearest enemy
            let atk_range = self.summons[si].attack_range();
            let mut best_idx = None;
            let mut best_dist = atk_range + 3.0;
            for ei in 0..enemy_count {
                if !enemy_alive[ei] { continue; }
                let ed = Self::dist(spos, enemy_pos[ei]);
                if ed < best_dist {
                    best_dist = ed;
                    best_idx = Some(ei);
                }
            }
            if self.summons[si].attack_cd > 0 {
                self.summons[si].attack_cd -= 1;
            }
            if let Some(ei) = best_idx {
                if best_dist <= atk_range && self.summons[si].attack_cd == 0 {
                    let dmg = self.summons[si].damage * aura_dmg;
                    self.enemies[ei].hp -= dmg;
                    self.summons[si].attack_cd = self.summons[si].kind.attack_cd();
                    self.floats.push(DamageFloat {
                        wx: enemy_pos[ei][0], wy: enemy_pos[ei][1],
                        dy: 0.0, value: dmg, is_heal: false, life: 25,
                    });
                    if self.enemies[ei].hp <= 0.0 {
                        enemy_alive[ei] = false;
                        self.kill_enemy(ei);
                    }
                } else if best_dist > atk_range {
                    let spd = self.summons[si].speed();
                    Self::move_toward(&mut self.summons[si].pos, enemy_pos[ei], spd);
                }
            }
            // Summons take damage from nearby enemies
            let spos2 = self.summons[si].pos;
            let mut sum_dmg = 0.0_f32;
            for e in &self.enemies {
                if !e.alive { continue; }
                if Self::dist(e.pos, spos2) <= ENEMY_ATTACK_RANGE {
                    sum_dmg += e.attack_damage() * 0.3; // summons take 30% enemy damage
                }
            }
            if sum_dmg > 0.0 {
                self.summons[si].hp -= sum_dmg;
                if self.summons[si].hp <= 0.0 {
                    self.summons[si].alive = false;
                }
            }
        }
        // Remove dead summons
        self.summons.retain(|s| s.alive);
    }

    // ======================================================================
    // Combo tracker
    // ======================================================================

    fn update_combo(&mut self) {
        self.combo.tick();
    }

    // ======================================================================
    // World boss ticker
    // ======================================================================

    fn update_world_boss(&mut self) {
        self.world_boss.tick_spawn();
        if !self.world_boss.alive { return; }
        // Check if player is in boss zone and within range
        let bp = self.world_boss.pos;
        let d = Self::dist(self.player_pos, bp);
        if d > 15.0 { return; } // out of range

        // Boss attacks player
        if self.world_boss.attack_cd > 0 {
            self.world_boss.attack_cd -= 1;
        }
        if d <= 2.0 && self.world_boss.attack_cd == 0 && !self.player_dead {
            let phase_mult = self.world_boss.phase_damage_mult();
            let dmg = WORLD_BOSS_DAMAGE * phase_mult;
            self.world_boss.attack_cd = 80;
            // Apply damage to player (damage reduction)
            let reduced = (dmg - self.damage_reduction()).max(1.0);
            self.player_hp -= reduced;
            self.floats.push(DamageFloat {
                wx: self.player_pos[0], wy: self.player_pos[1],
                dy: 0.0, value: reduced, is_heal: false, life: 40,
            });
        }
    }

    // ======================================================================
    // Shrine interaction
    // ======================================================================

    fn update_shrine_interact(&mut self) {
        let pp = self.player_pos;
        let mut activated: Vec<ShrineKind> = Vec::new();
        for shrine in &mut self.shrines {
            if shrine.used { continue; }
            if Self::dist(pp, shrine.pos) < 1.0 {
                shrine.used = true;
                activated.push(shrine.kind);
            }
        }
        for kind in activated {
            self.buffs.push(ActiveBuff { kind, remaining: SHRINE_DURATION });
            let msg: &[u8] = match kind {
                ShrineKind::Experience => b"XP SHRINE: +50% XP",
                ShrineKind::Health => b"HEALTH SHRINE: REGEN",
                ShrineKind::Damage => b"DAMAGE SHRINE: +50%",
            };
            self.push_log(msg, C_SHRINE);
        }
    }

    // ======================================================================
    // Chest interaction
    // ======================================================================

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
    fn update_chest_interact(&mut self) {
        let pp = self.player_pos;
        let opened_positions: Vec<[f32; 2]> = self.chests.iter_mut()
            .filter(|c| !c.opened && Self::dist(pp, c.pos) < 1.0)
            .map(|c| { c.opened = true; c.pos })
            .collect();
        for pos in opened_positions {
            self.push_log(b"CHEST OPENED", C_GOLD_DROP);
            let h = (pos[0] * 1000.0 + pos[1] * 777.0 + self.frame as f32) as u32;
            self.drops.push(ItemDrop {
                pos: [pos[0] + 0.3, pos[1] + 0.2],
                kind: DropKind::Gold,
                gold_amount: 15 + h % 20,
                life: 600,
            });
            if h.is_multiple_of(3) {
                self.drops.push(ItemDrop {
                    pos: [pos[0] - 0.2, pos[1] + 0.1],
                    kind: DropKind::HealthPotion,
                    gold_amount: 0,
                    life: 600,
                });
            }
            if h.is_multiple_of(4) {
                let slot = match h % 6 {
                    0 => EquipSlot::Helm, 1 => EquipSlot::Armor, 2 => EquipSlot::Weapon,
                    3 => EquipSlot::Boots, 4 => EquipSlot::Gloves, _ => EquipSlot::Ring,
                };
                let rarity = match h / 6 % 6 {
                    0 => ItemRarity::Rare, 1..=2 => ItemRarity::Magic, _ => ItemRarity::Normal,
                };
                self.drops.push(ItemDrop {
                    pos: [pos[0] - 0.3, pos[1] - 0.2],
                    kind: DropKind::Equipment(slot, rarity),
                    gold_amount: 0,
                    life: 900,
                });
            }
            // Chests have ~25% chance to drop a scroll
            if h.wrapping_mul(7_919) % 4 == 0 {
                let scroll_seed = h.wrapping_mul(1_103_515_245);
                let content = random_scroll_drop(scroll_seed, self.player_level);
                self.drops.push(ItemDrop {
                    pos: [pos[0] + 0.3, pos[1] - 0.3],
                    kind: DropKind::Scroll(content),
                    gold_amount: 0,
                    life: 900,
                });
            }
        }
    }

    // ======================================================================
    // Gather nodes — respawn tick + harvest interaction
    // ======================================================================

    fn update_gather_nodes(&mut self) {
        for node in &mut self.gather_nodes {
            node.update();
        }
        if self.harvest_cd > 0 {
            self.harvest_cd -= 1;
        }
    }

    /// Try to harvest the nearest gather node. Called when player presses E.
    #[allow(clippy::cast_possible_truncation)]
    fn try_harvest(&mut self) {
        if self.player_dead || self.harvest_cd > 0 { return; }
        let pp = self.player_pos;
        // Find nearest alive node in range
        let mut best_idx = None;
        let mut best_dist = f32::MAX;
        for (i, node) in self.gather_nodes.iter().enumerate() {
            if !node.is_alive() { continue; }
            let dx = node.pos[0] - pp[0];
            let dy = node.pos[1] - pp[1];
            let d = (dx * dx + dy * dy).sqrt();
            if d < node.kind.interact_range() && d < best_dist {
                best_dist = d;
                best_idx = Some(i);
            }
        }
        if let Some(idx) = best_idx {
            let node = &mut self.gather_nodes[idx];
            // Face toward node
            self.facing_right = node.pos[0] > pp[0];
            // Check civil skill level requirement
            let resource = node.kind.primary_resource(node.tier);
            if !self.civil.can_gather(resource) {
                self.push_log(b"SKILL TOO LOW", [1.0, 0.3, 0.3, 1.0]);
                return;
            }
            let node_pos = node.pos;
            let node_tier = node.tier;
            let node_kind = node.kind;
            // Hit the node
            if let Some(res) = node.hit() {
                let node_alive = node.is_alive();
                self.harvest_cd = node_kind.harvest_cooldown();
                // Add resource + XP via civil skills
                if let Some((qty, _xp)) = self.civil.gather(res) {
                    // Visual feedback: heal-style float showing quantity
                    self.floats.push(DamageFloat {
                        wx: node_pos[0], wy: node_pos[1],
                        dy: 0.0, value: qty as f32, is_heal: true, life: 40,
                    });
                    // Log the resource name
                    self.push_log(node_kind.label(), [0.4, 0.9, 0.3, 1.0]);
                }
                // Bonus resource chance when depleted
                if !node_alive {
                    if let Some(bonus) = node_kind.bonus_resource(node_tier) {
                        let h = (node_pos[0] * 997.0 + self.frame as f32) as u32;
                        if h % 4 == 0 {  // 25% chance
                            let _ = self.civil.gather(bonus);
                            self.push_log(b"BONUS DROP!", [1.0, 0.85, 0.2, 1.0]);
                        }
                    }
                }
            }
        }
    }

    // ======================================================================
    // Chain Lightning
    // ======================================================================

    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_possible_wrap)]
    fn cast_chain_lightning(&mut self) {
        if self.player_mp < CHAIN_LIGHTNING_COST { return; }
        self.player_mp -= CHAIN_LIGHTNING_COST;
        let dmg = self.chain_lightning_damage() * if self.damage_buff_active() { 1.5 } else { 1.0 };
        let mut origin = self.player_pos;
        let mut hit_set = Vec::new();
        for _ in 0..CHAIN_LIGHTNING_BOUNCES {
            // Find nearest unhit enemy within range
            let mut best_idx = None;
            let mut best_dist = CHAIN_LIGHTNING_RANGE;
            for (i, e) in self.enemies.iter().enumerate() {
                if !e.alive || hit_set.contains(&i) { continue; }
                let d = Self::dist(origin, e.pos);
                if d < best_dist {
                    best_dist = d;
                    best_idx = Some(i);
                }
            }
            let Some(idx) = best_idx else { break };
            let target_pos = self.enemies[idx].pos;
            self.lightning_bolts.push(LightningBolt {
                from: origin, to: target_pos, life: 12,
            });
            // Damage decreases per bounce, apply lightning resistance
            let raw_bounce = dmg * (0.8_f32).powi(hit_set.len() as i32);
            let bounce_dmg = self.enemies[idx].resistances.apply(raw_bounce, Element::Lightning);
            self.enemies[idx].hp -= bounce_dmg;
            self.floats.push(DamageFloat {
                wx: target_pos[0], wy: target_pos[1],
                dy: 0.0, value: bounce_dmg, is_heal: false, life: 30,
            });
            if self.enemies[idx].hp <= 0.0 {
                self.enemies[idx].alive = false;
                self.enemies[idx].respawn_timer = if self.enemies[idx].is_boss() { 1800 } else { 600 };
                self.enemies[idx].fade_timer = 30;
                // Can't call kill_enemy here (borrow issues), so track for later
            }
            hit_set.push(idx);
            origin = target_pos;
        }
        // Process kills from chain lightning
        for &idx in &hit_set {
            if !self.enemies[idx].alive {
                let pos = self.enemies[idx].pos;
                let kind = self.enemies[idx].kind;
                let xp = self.enemies[idx].xp_value() * self.xp_multiplier();
                self.player_xp += xp;
                #[allow(clippy::cast_precision_loss)]
                let needed = self.player_level as f32 * XP_PER_LEVEL;
                if self.player_xp >= needed {
                    self.player_xp -= needed;
                    self.player_level += 1;
                    self.player_hp = self.max_hp();
                    self.player_mp = self.max_mp();
                    self.stat_points += 5;
                    self.grid_unlock_points += 1;
                    self.push_log(b"LEVEL UP!", [1.0, 0.95, 0.40, 1.0]);
                }
                self.waypoints[self.current_zone as usize] = true;
                self.spawn_loot(pos, matches!(kind, EnemyKind::Boss | EnemyKind::BloodRaven));
            }
        }
        if hit_set.is_empty() {
            // No targets, refund some mana
            self.player_mp += CHAIN_LIGHTNING_COST * 0.5;
        }
    }

    // ======================================================================
    // Teleport
    // ======================================================================

    #[allow(clippy::cast_precision_loss, clippy::similar_names, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn cast_teleport(&mut self, tgt_sx: f32, tgt_sy: f32) {
        if self.player_mp < TELEPORT_COST { return; }
        let [twx, twy] = self.s2w(tgt_sx, tgt_sy);
        let dx = twx - self.player_pos[0];
        let dy = twy - self.player_pos[1];
        let d = (dx * dx + dy * dy).sqrt();
        if d < 0.5 { return; }
        let range = TELEPORT_RANGE + self.disk_bonuses.spell_dmg_pct * 2.0;
        let actual_d = d.min(range);
        let dest = [
            (self.player_pos[0] + dx / d * actual_d).clamp(0.5, MAP_W as f32 - 0.5),
            (self.player_pos[1] + dy / d * actual_d).clamp(0.5, MAP_H as f32 - 0.5),
        ];
        // Check destination is walkable
        if Self::tile_walkable(&self.global_terrain, self.world_offset, dest[0], dest[1]) {
            self.player_mp -= TELEPORT_COST;
            self.player_pos = dest;
            self.move_target = None;
            self.attack_target = None;
        }
    }

    // ======================================================================
    // Player effective resistances
    // ======================================================================

    fn effective_resistances(&self) -> Resistances {
        let penalty = self.difficulty.resist_penalty();
        Resistances {
            fire: (self.resistances.fire - penalty).clamp(-1.0, 0.75),
            cold: (self.resistances.cold - penalty).clamp(-1.0, 0.75),
            lightning: (self.resistances.lightning - penalty).clamp(-1.0, 0.75),
            poison: (self.resistances.poison - penalty).clamp(-1.0, 0.75),
        }
    }

    // ======================================================================
    // Gheed gambling
    // ======================================================================

    fn gheed_gamble(&mut self) {
        if self.player_gold < GAMBLE_COST { return; }
        self.player_gold -= GAMBLE_COST;
        let h = self.frame.wrapping_mul(2_654_435_761);
        let slot = match h % 6 {
            0 => EquipSlot::Helm, 1 => EquipSlot::Armor, 2 => EquipSlot::Weapon,
            3 => EquipSlot::Boots, 4 => EquipSlot::Gloves, _ => EquipSlot::Ring,
        };
        let rarity = match h / 6 % 10 {
            0 => ItemRarity::Unique,
            1..=2 => ItemRarity::Rare,
            3..=5 => ItemRarity::Magic,
            _ => ItemRarity::Normal,
        };
        let eq = Equipment { slot, rarity, ilvl: self.player_level };
        let idx = slot as usize;
        if let Some(old) = self.equipment[idx] {
            if rarity.stat_mult() <= old.rarity.stat_mult() && old.ilvl >= self.player_level {
                self.push_log(b"GAMBLE: WORSE ITEM", [0.6, 0.5, 0.4, 1.0]);
                return;
            }
        }
        self.equipment[idx] = Some(eq);
        self.push_log_equip(slot, rarity);
    }

    // ======================================================================
    // Charsi upgrade
    // ======================================================================

    fn sell_equipment(&mut self) {
        if !self.show_inventory { return; }
        // Sell the worst equipped item (lowest rarity, then lowest ilvl)
        let mut worst: Option<(usize, u32)> = None;
        for (i, eq) in self.equipment.iter().enumerate() {
            if let Some(e) = eq {
                let score = e.rarity.stat_mult() * 100 + e.ilvl;
                if worst.is_none() || score < worst.unwrap().1 {
                    worst = Some((i, score));
                }
            }
        }
        if let Some((idx, _)) = worst {
            let eq = self.equipment[idx].unwrap();
            let gold = eq.sell_value();
            let sell_mult = self.civil.sell_price_multiplier();
            let adjusted_gold = (gold as f32 * sell_mult) as u32;
            self.player_gold += adjusted_gold;
            self.equipment[idx] = None;
            self.civil.record_sale();
            let gold = adjusted_gold;
            let mut msg = [0u8; 40];
            msg[..5].copy_from_slice(b"SOLD ");
            let sn = eq.slot.name();
            let end = (5 + sn.len()).min(30);
            msg[5..end].copy_from_slice(&sn[..end - 5]);
            let (gv, gl) = fmt_u32(gold);
            msg[end..=end].copy_from_slice(b" ");
            let e2 = (end + 1 + gl).min(38);
            msg[end + 1..e2].copy_from_slice(&gv[..e2 - end - 1]);
            msg[e2] = b'G';
            self.push_log(&msg[..=e2], C_GOLD_DROP);
        }
    }

    fn charsi_upgrade(&mut self) {
        if self.player_gold < UPGRADE_COST { return; }
        // Find first equipped item that can be upgraded
        for i in 0..EQUIP_SLOT_COUNT {
            if let Some(eq) = self.equipment[i] {
                let next_rarity = match eq.rarity {
                    ItemRarity::Normal => ItemRarity::Magic,
                    ItemRarity::Magic => ItemRarity::Rare,
                    ItemRarity::Rare => ItemRarity::Unique,
                    ItemRarity::Unique => continue, // already max
                };
                self.player_gold -= UPGRADE_COST;
                self.equipment[i] = Some(Equipment { slot: eq.slot, rarity: next_rarity, ilvl: eq.ilvl });
                self.push_log_equip(eq.slot, next_rarity);
                return;
            }
        }
        self.push_log(b"NO ITEM TO UPGRADE", [0.6, 0.5, 0.4, 1.0]);
    }

    // ======================================================================
    // Backpack management
    // ======================================================================

    /// Use/equip item from backpack slot. Equipment goes to equip slot,
    /// potions go to belt, scrolls go to scroll inventory.
    fn backpack_use(&mut self, slot_idx: usize) {
        let item = match self.backpack.get(slot_idx) {
            Some(item) => *item,
            None => return,
        };
        match item {
            InvItem::Equipment(eq) => {
                let equip_idx = eq.slot as usize;
                // Swap: put current equip into backpack, equip new
                if let Some(old) = self.equipment[equip_idx].take() {
                    self.backpack.remove(slot_idx);
                    self.backpack.add(InvItem::Equipment(old));
                } else {
                    self.backpack.remove(slot_idx);
                }
                self.equipment[equip_idx] = Some(eq);
                self.push_log_equip(eq.slot, eq.rarity);
            }
            InvItem::Potion(kind) => {
                if let Some(belt_slot) = self.belt.iter_mut().find(|s| s.is_none()) {
                    *belt_slot = Some(kind);
                    self.backpack.remove(slot_idx);
                }
            }
            InvItem::Scroll(_scroll) => {
                // Scrolls are already in scroll_inventory, just log
                self.push_log(b"USE IN SKILL GRID", [0.6, 0.7, 0.9, 0.9]);
            }
            InvItem::Gold(amount) => {
                self.player_gold += amount;
                self.backpack.remove(slot_idx);
            }
        }
    }

    /// Sell item from backpack slot for gold.
    fn backpack_sell(&mut self, slot_idx: usize) {
        let item = match self.backpack.remove(slot_idx) {
            Some(item) => item,
            None => return,
        };
        let gold = match item {
            InvItem::Equipment(eq) => {
                let v = eq.sell_value();
                let sell_mult = self.civil.sell_price_multiplier();
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let adjusted = (v as f32 * sell_mult) as u32;
                self.civil.record_sale();
                adjusted
            }
            InvItem::Potion(_) => 3,
            InvItem::Scroll(_) => 8,
            InvItem::Gold(g) => g,
        };
        self.player_gold += gold;
        let (gv, gl) = fmt_u32(gold);
        let mut msg = [0u8; 20];
        msg[..5].copy_from_slice(b"SOLD ");
        msg[5..5 + gl].copy_from_slice(&gv[..gl]);
        msg[5 + gl] = b'G';
        self.push_log(&msg[..6 + gl], C_GOLD_DROP);
    }

    /// Drop item from backpack onto the ground.
    fn backpack_drop(&mut self, slot_idx: usize) {
        let item = match self.backpack.remove(slot_idx) {
            Some(item) => item,
            None => return,
        };
        let kind = match item {
            InvItem::Equipment(eq) => DropKind::Equipment(eq.slot, eq.rarity),
            InvItem::Potion(PotionKind::Health) => DropKind::HealthPotion,
            InvItem::Potion(PotionKind::Mana) => DropKind::ManaPotion,
            InvItem::Scroll(s) => DropKind::Scroll(s.content),
            InvItem::Gold(g) => {
                self.drops.push(ItemDrop {
                    pos: self.player_pos, kind: DropKind::Gold,
                    gold_amount: g, life: 600,
                });
                return;
            }
        };
        self.drops.push(ItemDrop {
            pos: self.player_pos, kind, gold_amount: 0, life: 600,
        });
        // If it was a scroll, also remove from scroll_inventory
        if let InvItem::Scroll(s) = item {
            if let Some(idx) = self.scroll_inventory.iter().position(|si| si.content == s.content) {
                self.scroll_inventory.remove(idx);
            }
        }
    }

    /// Upgrade backpack capacity (buy extra row from NPC).
    fn backpack_upgrade(&mut self) {
        if !self.backpack.can_upgrade() { return; }
        if self.player_gold < BACKPACK_UPGRADE_COST { return; }
        self.player_gold -= BACKPACK_UPGRADE_COST;
        let new_cap = self.backpack.upgrade();
        let (cv, cl) = fmt_u32(new_cap as u32);
        let mut msg = [0u8; 24];
        msg[..10].copy_from_slice(b"BAG SLOTS:");
        msg[10..10 + cl].copy_from_slice(&cv[..cl]);
        self.push_log(&msg[..10 + cl], [0.6, 0.8, 0.6, 1.0]);
    }

    // ======================================================================
    // Scene composition
    // ======================================================================

    fn populate_batch(&mut self) {
        self.batch.clear();
        self.draw_terrain();
        self.draw_waypoints();
        // Camp-only structures
        if self.is_camp() {
            self.draw_palisade();
            self.draw_structures();
            self.draw_campfire();
            self.draw_decorations();
            self.draw_npcs();
        }
        self.draw_zone_exits();
        self.draw_shrines();
        self.draw_chests();
        self.draw_breakables();
        self.draw_gather_nodes();
        self.draw_enemies();
        self.draw_drops();
        self.draw_projectiles();
        self.draw_lightning_bolts();
        if self.portal_active { self.draw_portal(); }
        self.draw_player();
        self.draw_mercenary();
        self.draw_floats();
        self.draw_buffs_hud();
        self.draw_hud();
        self.draw_combat_log();
        if self.player_dead { self.draw_death_overlay(); }
        if self.show_minimap { self.draw_minimap(); }
        if self.show_stats { self.draw_stats(); }
        if self.show_quests { self.draw_quest_log(); }
        if self.show_inventory { self.draw_inventory(); }
        if self.show_waypoint_menu { self.draw_waypoint_menu(); }
        if let Some(idx) = self.npc_dialog { self.draw_npc_dialog(idx); }
        if self.show_disk { self.draw_skill_disk(); }
        if self.show_civil { self.draw_civil_skills(); }
        self.batch.sort();
    }

    // ======================================================================
    // Terrain — 3/4 view: tiles are TW x TH rectangles
    // ======================================================================

    /// Draw terrain tiles for a single zone at the given world offset.
    /// Render terrain from the global map — single pass, no neighbor stitching.
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn draw_terrain(&mut self) {
        let cam = self.cam_world();
        let half_w = self.viewport[0] * 0.5 / TW + 2.0;
        let half_h = self.viewport[1] * 0.5 / TH + 2.0;
        // Visible range in world-space → global indices
        let gx0 = ((cam[0] - half_w + GLOBAL_OFF).floor().max(0.0)) as u32;
        let gy0 = ((cam[1] - half_h + GLOBAL_OFF).floor().max(0.0)) as u32;
        let gx1 = ((cam[0] + half_w + GLOBAL_OFF).ceil() as u32).min(WORLD_W as u32);
        let gy1 = ((cam[1] + half_h + GLOBAL_OFF).ceil() as u32).min(WORLD_H as u32);

        for gy in gy0..gy1 {
            for gx in gx0..gx1 {
                let tt = self.global_terrain[gy as usize * WORLD_W + gx as usize];
                let base = match tt {
                    1 => C_GRASS, 2 => C_DIRT, 3 => C_STONE, 4 => C_CAVE_FLOOR,
                    5 => C_WATER,
                    _ => C_DARK_GRASS,
                };
                // Global → world coords for rendering
                let wx = gx as f32 - GLOBAL_OFF + 0.5;
                let wy = gy as f32 - GLOBAL_OFF + 0.5;
                let [sx, sy] = self.w2s_world(wx, wy);
                self.batch.push(SpriteInstance {
                    screen_pos: [sx - TW * 0.5, sy - TH * 0.5],
                    src: UNIT, atlas: ATLAS_0, material: MAT,
                    sort_key: SortKey { layer: RenderLayer::Terrain, y_sort: 0, sub_order: gx },
                    tint: vary(base, gx, gy), scale: [TW, TH],
                });

                // Decals for visual variety
                let h = gx.wrapping_mul(2_654_435_761).wrapping_add(gy.wrapping_mul(340_573_321));
                if tt == 1 && h.is_multiple_of(4) {
                    // Grass tufts (darker green patches)
                    let [dsx, dsy] = self.w2s_world(gx as f32 - GLOBAL_OFF + 0.3, gy as f32 - GLOBAL_OFF + 0.6);
                    self.batch.push(SpriteInstance {
                        screen_pos: [dsx - 3.0, dsy - 8.0],
                        src: UNIT, atlas: ATLAS_0, material: MAT,
                        sort_key: SortKey { layer: RenderLayer::Decals, y_sort: 0, sub_order: gx + 10000 },
                        tint: [0.10, 0.28, 0.06, 0.55], scale: [6.0, 8.0],
                    });
                } else if tt == 1 && h.is_multiple_of(11) {
                    // Flowers (small bright dots)
                    let flower_c = match (h >> 8) % 3 {
                        0 => [0.8, 0.7, 0.2, 0.6],  // yellow
                        1 => [0.7, 0.3, 0.5, 0.5],  // pink
                        _ => [0.3, 0.5, 0.8, 0.5],  // blue
                    };
                    let [dsx, dsy] = self.w2s_world(gx as f32 - GLOBAL_OFF + 0.5, gy as f32 - GLOBAL_OFF + 0.5);
                    self.batch.push(SpriteInstance {
                        screen_pos: [dsx - 2.0, dsy - 2.0],
                        src: UNIT, atlas: ATLAS_0, material: MAT,
                        sort_key: SortKey { layer: RenderLayer::Decals, y_sort: 0, sub_order: gx + 10000 },
                        tint: flower_c, scale: [4.0, 4.0],
                    });
                } else if tt == 2 && h.is_multiple_of(6) {
                    // Dirt pebbles
                    let [dsx, dsy] = self.w2s_world(gx as f32 - GLOBAL_OFF + 0.5, gy as f32 - GLOBAL_OFF + 0.5);
                    self.batch.push(SpriteInstance {
                        screen_pos: [dsx - 4.0, dsy - 3.0],
                        src: UNIT, atlas: ATLAS_0, material: MAT,
                        sort_key: SortKey { layer: RenderLayer::Decals, y_sort: 0, sub_order: gx + 10000 },
                        tint: [0.30, 0.24, 0.16, 0.35], scale: [8.0, 6.0],
                    });
                } else if tt == 3 && h.is_multiple_of(8) {
                    // Stone cracks
                    let [dsx, dsy] = self.w2s_world(gx as f32 - GLOBAL_OFF + 0.4, gy as f32 - GLOBAL_OFF + 0.5);
                    self.batch.push(SpriteInstance {
                        screen_pos: [dsx - 3.0, dsy - 2.0],
                        src: UNIT, atlas: ATLAS_0, material: MAT,
                        sort_key: SortKey { layer: RenderLayer::Decals, y_sort: 0, sub_order: gx + 10000 },
                        tint: [0.35, 0.32, 0.26, 0.4], scale: [6.0, 4.0],
                    });
                } else if tt == 4 && h.is_multiple_of(9) {
                    // Cave moss
                    let [dsx, dsy] = self.w2s_world(gx as f32 - GLOBAL_OFF + 0.4, gy as f32 - GLOBAL_OFF + 0.5);
                    self.batch.push(SpriteInstance {
                        screen_pos: [dsx - 3.0, dsy - 2.0],
                        src: UNIT, atlas: ATLAS_0, material: MAT,
                        sort_key: SortKey { layer: RenderLayer::Decals, y_sort: 0, sub_order: gx + 10000 },
                        tint: [0.12, 0.18, 0.10, 0.35], scale: [6.0, 4.0],
                    });
                } else if tt == 5 && h.is_multiple_of(7) {
                    // Water ripple highlight
                    let [dsx, dsy] = self.w2s_world(gx as f32 - GLOBAL_OFF + 0.5, gy as f32 - GLOBAL_OFF + 0.4);
                    self.batch.push(SpriteInstance {
                        screen_pos: [dsx - 5.0, dsy - 2.0],
                        src: UNIT, atlas: ATLAS_0, material: MAT,
                        sort_key: SortKey { layer: RenderLayer::Decals, y_sort: 0, sub_order: gx + 10000 },
                        tint: [0.20, 0.35, 0.50, 0.25], scale: [10.0, 4.0],
                    });
                }
            }
        }
    }

    // ======================================================================
    // Palisade — 3/4 view: visible front face + top
    // ======================================================================

    #[allow(clippy::cast_possible_truncation)]
    fn draw_palisade(&mut self) {
        for (i, &[wx, wy]) in PALISADE.iter().enumerate() {
            let s = i as u32;
            // Shadow (offset down-right in 3/4)
            self.spr(wx, wy, [-4.0, 2.0], [10.0, 6.0], RenderLayer::Decals, s, C_SHADOW);
            // Front face (visible wall)
            self.spr(wx, wy, [-4.0, -16.0], [8.0, 20.0], RenderLayer::Props, s, C_WALL_DK);
            // Top face (lighter, squashed)
            self.spr(wx, wy, [-4.0, -18.0], [8.0, 4.0], RenderLayer::Props, s + 200, C_WALL_TOP);
            // Pointed tip
            self.spr(wx, wy, [-2.0, -22.0], [4.0, 6.0], RenderLayer::Props, s + 300, C_WALL);
        }
    }

    // Trees removed — will be entities for civil skill (woodcutting).

    // ======================================================================
    // Structures — 3/4 view: front wall + roof visible from above
    // ======================================================================

    fn draw_structures(&mut self) {
        self.draw_tent(8.5, 5.5, 0);
        self.draw_tent(15.0, 5.5, 10);
        self.draw_forge(8.5, 17.0);
        self.draw_wagon(15.0, 17.0);
        self.draw_torch(6.5, 6.0, 40);
        self.draw_torch(10.5, 6.0, 42);
        self.draw_torch(13.5, 6.0, 44);
        self.draw_torch(17.0, 6.0, 46);
        self.draw_torch(12.0, 21.0, 48);
        self.draw_torch(13.0, 21.0, 50);
    }

    fn draw_tent(&mut self, wx: f32, wy: f32, b: u32) {
        // Shadow
        self.spr(wx, wy, [-26.0, 6.0], [52.0, 12.0], RenderLayer::Decals, b, C_SHADOW);
        // Front wall (visible face)
        self.spr(wx, wy, [-22.0, -8.0], [44.0, 18.0], RenderLayer::Props, b + 1, C_WALL);
        // Dark entrance
        self.spr(wx, wy, [-8.0, -2.0], [16.0, 12.0], RenderLayer::Props, b + 2, [0.06, 0.04, 0.02, 0.92]);
        // Roof (sloped, visible from above — wider than walls, overhangs)
        self.spr(wx, wy, [-26.0, -30.0], [52.0, 26.0], RenderLayer::Props, b + 3, C_ROOF);
        // Roof ridge
        self.spr(wx, wy, [-26.0, -30.0], [52.0, 4.0], RenderLayer::Props, b + 4, C_ROOF_DK);
        // Roof shadow on front wall
        self.spr(wx, wy, [-24.0, -8.0], [48.0, 4.0], RenderLayer::Props, b + 5, [0.0, 0.0, 0.0, 0.15]);
    }

    fn draw_forge(&mut self, wx: f32, wy: f32) {
        self.spr(wx, wy, [-20.0, 4.0], [40.0, 10.0], RenderLayer::Decals, 20, C_SHADOW);
        // Stone base (front face)
        self.spr(wx, wy, [-16.0, -12.0], [32.0, 20.0], RenderLayer::Props, 21, C_STONE);
        // Anvil on top
        self.spr(wx - 0.5, wy, [-8.0, -16.0], [12.0, 6.0], RenderLayer::Props, 22, C_METAL);
        self.spr(wx - 0.5, wy, [-6.0, -18.0], [8.0, 4.0], RenderLayer::Props, 23, C_METAL_DK);
        // Fire pit
        self.spr(wx + 0.7, wy, [-6.0, -10.0], [12.0, 14.0], RenderLayer::Props, 24, C_EMBER);
        self.spr(wx + 0.7, wy, [-5.0, -18.0], [10.0, 12.0], RenderLayer::VfxAlpha, 25, C_FIRE_MID);
        self.spr(wx + 0.7, wy, [-3.0, -22.0], [6.0, 8.0], RenderLayer::VfxAlpha, 26, C_FIRE_CORE);
        // Glow
        self.spr(wx + 0.7, wy, [-18.0, -10.0], [36.0, 24.0], RenderLayer::Decals, 27, C_FIRE_GLOW);
    }

    fn draw_wagon(&mut self, wx: f32, wy: f32) {
        self.spr(wx, wy, [-22.0, 4.0], [44.0, 10.0], RenderLayer::Decals, 30, C_SHADOW);
        // Wagon body (front face visible)
        self.spr(wx, wy, [-18.0, -10.0], [36.0, 16.0], RenderLayer::Props, 31, C_WOOD);
        // Planks detail
        self.spr(wx, wy, [-18.0, -4.0], [36.0, 2.0], RenderLayer::Props, 32, C_WOOD_DK);
        self.spr(wx, wy, [-18.0, -10.0], [36.0, 2.0], RenderLayer::Props, 33, C_WOOD_DK);
        // Wheels (visible from front — circle shapes)
        self.spr(wx, wy, [-20.0, -4.0], [8.0, 10.0], RenderLayer::Props, 34, C_WOOD_DK);
        self.spr(wx, wy, [12.0, -4.0], [8.0, 10.0], RenderLayer::Props, 35, C_WOOD_DK);
        // Canvas top (3/4 view — sloped)
        self.spr(wx, wy, [-20.0, -24.0], [40.0, 16.0], RenderLayer::Props, 36, [0.52, 0.45, 0.32, 1.0]);
        // Canvas ridge
        self.spr(wx, wy, [-20.0, -24.0], [40.0, 3.0], RenderLayer::Props, 37, [0.42, 0.36, 0.24, 1.0]);
    }

    fn draw_torch(&mut self, wx: f32, wy: f32, sub: u32) {
        // Post (front-facing, visible height)
        self.spr(wx, wy, [-2.0, -18.0], [4.0, 22.0], RenderLayer::Props, sub, C_WOOD_DK);
        // Flame (above post)
        self.spr(wx, wy, [-4.0, -26.0], [8.0, 10.0], RenderLayer::VfxAlpha, sub + 1, C_TORCH_FLAME);
        self.spr(wx, wy, [-3.0, -30.0], [6.0, 6.0], RenderLayer::VfxAlpha, sub + 2, C_FIRE_CORE);
        // Ground glow
        self.spr(wx, wy, [-14.0, -6.0], [28.0, 18.0], RenderLayer::Decals, sub, C_FIRE_GLOW);
    }

    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    fn draw_campfire(&mut self) {
        let (wx, wy) = (12.0, 12.0);
        // Ground glow
        self.spr(wx, wy, [-22.0, -14.0], [44.0, 32.0], RenderLayer::Decals, 0, C_FIRE_GLOW);
        // Stone ring
        let stones: &[[f32; 2]] = &[
            [-10.0, -4.0], [-8.0, -8.0], [-2.0, -10.0], [4.0, -10.0],
            [10.0, -8.0], [12.0, -4.0], [4.0, 2.0], [-2.0, 2.0],
        ];
        for (i, &[ox, oy]) in stones.iter().enumerate() {
            self.spr(wx, wy, [ox, oy], [6.0, 5.0], RenderLayer::Props, i as u32 + 1, C_STONE);
        }
        // Embers
        self.spr(wx, wy, [-6.0, -6.0], [12.0, 10.0], RenderLayer::Props, 0, C_EMBER);
        // Fire (front-facing flames with height)
        self.spr(wx, wy, [-8.0, -20.0], [16.0, 18.0], RenderLayer::VfxAlpha, 0, C_FIRE_MID);
        self.spr(wx, wy, [-5.0, -28.0], [10.0, 14.0], RenderLayer::VfxAlpha, 1, C_FIRE_CORE);
        self.spr(wx, wy, [-3.0, -34.0], [6.0, 8.0], RenderLayer::VfxAlpha, 2,
                 [1.0, 0.80, 0.30, 0.55]);
    }

    fn draw_decorations(&mut self) {
        // Waypoint
        self.spr(12.0, 6.0, [-12.0, -4.0], [24.0, 14.0], RenderLayer::Decals, 500, C_WP_GLOW);
        self.spr(12.0, 6.0, [-8.0, -8.0], [16.0, 12.0], RenderLayer::Props, 500, C_STONE);
        // Waypoint pillar (visible height)
        self.spr(12.0, 6.0, [-3.0, -20.0], [6.0, 16.0], RenderLayer::Props, 501, C_STONE);
        self.spr(12.0, 6.0, [-5.0, -16.0], [10.0, 10.0], RenderLayer::VfxAlpha, 502, C_WP_CORE);

        // Stash chest (front face visible)
        self.spr(10.0, 8.0, [-8.0, -8.0], [16.0, 12.0], RenderLayer::Props, 510, C_WOOD_DK);
        self.spr(10.0, 8.0, [-8.0, -10.0], [16.0, 4.0], RenderLayer::Props, 511, C_METAL);
        self.spr(10.0, 8.0, [-1.0, -7.0], [2.0, 3.0], RenderLayer::Props, 512,
                 [0.68, 0.58, 0.18, 1.0]);

        // Barrels (3/4 view — cylinder with visible top)
        self.draw_barrel(6.5, 16.5, 520);
        self.draw_barrel(7.0, 17.5, 525);
        self.draw_barrel(6.0, 17.0, 530);
        // Crates
        self.draw_crate(17.0, 16.0, 540);
        self.draw_crate(17.5, 17.0, 545);
    }

    fn draw_barrel(&mut self, wx: f32, wy: f32, sub: u32) {
        self.spr(wx, wy, [-5.0, 2.0], [10.0, 5.0], RenderLayer::Decals, sub, C_SHADOW);
        // Barrel front face
        self.spr(wx, wy, [-6.0, -12.0], [12.0, 16.0], RenderLayer::Props, sub + 1, C_WOOD);
        // Metal bands
        self.spr(wx, wy, [-6.0, -6.0], [12.0, 2.0], RenderLayer::Props, sub + 2, C_METAL_DK);
        self.spr(wx, wy, [-6.0, -12.0], [12.0, 2.0], RenderLayer::Props, sub + 3, C_METAL_DK);
        // Barrel top (3/4 ellipse)
        self.spr(wx, wy, [-6.0, -14.0], [12.0, 5.0], RenderLayer::Props, sub + 4, C_WOOD_DK);
    }

    fn draw_crate(&mut self, wx: f32, wy: f32, sub: u32) {
        self.spr(wx, wy, [-6.0, 2.0], [12.0, 5.0], RenderLayer::Decals, sub, C_SHADOW);
        // Front face
        self.spr(wx, wy, [-7.0, -12.0], [14.0, 16.0], RenderLayer::Props, sub + 1, C_WOOD_DK);
        // Planks
        self.spr(wx, wy, [-7.0, -5.0], [14.0, 2.0], RenderLayer::Props, sub + 2, C_WOOD);
        self.spr(wx, wy, [-1.0, -12.0], [2.0, 16.0], RenderLayer::Props, sub + 3, C_WOOD);
        // Top face (3/4)
        self.spr(wx, wy, [-7.0, -14.0], [14.0, 5.0], RenderLayer::Props, sub + 4, C_WOOD);
    }

    // ======================================================================
    // Characters — 3/4 view: front-facing sprites with visible body/head
    // ======================================================================

    fn draw_npc(&mut self, wx: f32, wy: f32, body: [f32; 4], sub: u32) {
        let dk = [body[0] * 0.65, body[1] * 0.65, body[2] * 0.65, 1.0];
        // Shadow
        self.spr(wx, wy, [-8.0, 2.0], [16.0, 6.0], RenderLayer::Decals, sub, C_SHADOW);
        // Feet
        self.spr(wx, wy, [-3.0, -2.0], [3.0, 6.0], RenderLayer::Entities, sub, C_BOOTS);
        self.spr(wx, wy, [0.0, -2.0], [3.0, 6.0], RenderLayer::Entities, sub, C_BOOTS);
        // Body (front-facing torso)
        self.spr(wx, wy, [-7.0, -20.0], [14.0, 20.0], RenderLayer::Entities, sub + 1, body);
        // Belt
        self.spr(wx, wy, [-7.0, -8.0], [14.0, 3.0], RenderLayer::Entities, sub + 2, dk);
        // Head
        self.spr(wx, wy, [-5.0, -28.0], [10.0, 10.0], RenderLayer::Entities, sub + 3, C_SKIN);
        // Hair/hat
        self.spr(wx, wy, [-5.0, -32.0], [10.0, 6.0], RenderLayer::Entities, sub + 4, dk);
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    fn draw_npcs(&mut self) {
        self.draw_npc(NPCS[0].pos[0], NPCS[0].pos[1], C_HEALER, 50);
        self.draw_npc(NPCS[1].pos[0], NPCS[1].pos[1], C_MERCHANT, 60);
        self.draw_npc(NPCS[2].pos[0], NPCS[2].pos[1], C_SMITH, 70);
        self.draw_npc(NPCS[3].pos[0], NPCS[3].pos[1], C_WARRIOR, 80);

        // NPC name labels with text
        for (i, npc) in NPCS.iter().enumerate() {
            let [sx, sy] = self.w2s(npc.pos[0], npc.pos[1]);
            let tw = npc.name.len() as f32 * 9.0;
            // Background
            self.batch.push(SpriteInstance {
                screen_pos: [sx - tw * 0.5 - 3.0, sy - 44.0],
                src: UNIT, atlas: ATLAS_0, material: MAT,
                sort_key: SortKey { layer: RenderLayer::UiWorld, y_sort: 0, sub_order: 100 + i as u32 },
                tint: [0.0, 0.0, 0.0, 0.55], scale: [tw + 6.0, 14.0],
            });
            // Name text
            self.draw_text_at(npc.name, sx - tw * 0.5, sy - 42.0, 1.5,
                              C_NPC_LABEL, RenderLayer::UiWorld, 0, 110 + i as u32);
        }

        // Interact indicator
        if let Some(idx) = self.npc_interact {
            if idx < NPCS.len() {
                let [sx, sy] = self.w2s(NPCS[idx].pos[0], NPCS[idx].pos[1]);
                let pulse = ((self.frame % 30) as f32 / 30.0 * std::f32::consts::TAU).sin() * 0.15 + 0.85;
                let c = [C_INTERACT[0] * pulse, C_INTERACT[1] * pulse, C_INTERACT[2], C_INTERACT[3]];
                self.batch.push(SpriteInstance {
                    screen_pos: [sx - 4.0, sy - 52.0],
                    src: UNIT, atlas: ATLAS_0, material: MAT,
                    sort_key: SortKey { layer: RenderLayer::UiWorld, y_sort: 0, sub_order: 200 },
                    tint: c, scale: [8.0, 12.0],
                });
            }
        }
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    fn draw_enemies(&mut self) {
        let frame = self.frame;
        let attack_target = self.attack_target;
        let vw = self.viewport[0];
        let vh = self.viewport[1];
        // Collect alive + fading enemies with frustum culling
        let draw_data: Vec<_> = self.enemies.iter().enumerate().filter_map(|(i, enemy)| {
            let alpha = if enemy.alive {
                1.0_f32
            } else if enemy.fade_timer > 0 {
                enemy.fade_timer as f32 / 30.0
            } else {
                return None;
            };
            // Frustum cull: skip if off-screen (with margin)
            let [sx, sy] = self.w2s(enemy.pos[0], enemy.pos[1]);
            if sx < -80.0 || sx > vw + 80.0 || sy < -80.0 || sy > vh + 80.0 {
                return None;
            }
            let moving = enemy.alive && Self::dist(enemy.pos, enemy.spawn) > 0.3;
            let head_c = match enemy.kind {
                EnemyKind::Skeleton => C_SKELETON,
                EnemyKind::Boss => C_BOSS,
                _ => [0.50, 0.55, 0.35, 1.0],
            };
            Some((i, enemy.pos, enemy_colors(enemy.kind), moving, head_c,
                  enemy.hp, enemy.max_hp, attack_target == Some(i),
                  alpha, enemy.is_boss(), enemy.affix, enemy.freeze_timer > 0, enemy.slow_timer > 0))
        }).collect();

        for (i, pos, (body, dk), moving, head_c, hp, max_hp, targeted, alpha, is_boss, affix, frozen, slowed) in draw_data {
            let [wx, wy] = pos;
            let sub = 600 + i as u32 * 10;
            let scale_mul = if is_boss { 1.5 } else { 1.0 };

            // Apply alpha for fade
            let ab = [body[0], body[1], body[2], body[3] * alpha];
            let adk = [dk[0], dk[1], dk[2], dk[3] * alpha];
            let ahc = [head_c[0], head_c[1], head_c[2], head_c[3] * alpha];
            let ashad = [C_SHADOW[0], C_SHADOW[1], C_SHADOW[2], C_SHADOW[3] * alpha];

            // Shadow
            let sw = 14.0 * scale_mul;
            self.spr(wx, wy, [-sw * 0.5, 2.0], [sw, 5.0 * scale_mul], RenderLayer::Decals, sub, ashad);

            // Walk animation
            let leg_off = if moving {
                if ((frame / 8 + i as u32) % 4) < 2 { 1.0 } else { -1.0 }
            } else { 0.0 };

            let bw = 12.0 * scale_mul;
            let bh = 18.0 * scale_mul;
            // Feet
            self.spr(wx, wy, [-3.0 * scale_mul + leg_off, -2.0], [2.0 * scale_mul, 5.0 * scale_mul], RenderLayer::Entities, sub, adk);
            self.spr(wx, wy, [1.0 * scale_mul - leg_off, -2.0], [2.0 * scale_mul, 5.0 * scale_mul], RenderLayer::Entities, sub, adk);
            // Body
            self.spr(wx, wy, [-bw * 0.5, -bh], [bw, bh], RenderLayer::Entities, sub + 1, ab);
            // Arms
            self.spr(wx, wy, [-8.0 * scale_mul, -16.0 * scale_mul], [3.0 * scale_mul, 12.0 * scale_mul], RenderLayer::Entities, sub + 2, adk);
            self.spr(wx, wy, [5.0 * scale_mul, -16.0 * scale_mul], [3.0 * scale_mul, 12.0 * scale_mul], RenderLayer::Entities, sub + 2, adk);
            // Head
            self.spr(wx, wy, [-4.0 * scale_mul, -26.0 * scale_mul], [8.0 * scale_mul, 10.0 * scale_mul], RenderLayer::Entities, sub + 3, ahc);

            // Boss crown
            if is_boss && alpha > 0.5 {
                self.spr(wx, wy, [-6.0, -44.0], [12.0, 6.0], RenderLayer::Entities, sub + 4, C_GOLD_DROP);
            }

            if alpha < 1.0 { continue; } // don't show HP bar for fading

            // HP bar
            let [sx, sy] = self.w2s(wx, wy);
            let bar_w = if is_boss { 36.0 } else { 22.0 };
            let bar_y = if is_boss { -48.0 } else { -34.0 };
            let hp_ratio: f32 = (hp / max_hp).clamp(0.0, 1.0);
            self.batch.push(SpriteInstance {
                screen_pos: [sx - bar_w * 0.5, sy + bar_y],
                src: UNIT, atlas: ATLAS_0, material: MAT,
                sort_key: SortKey { layer: RenderLayer::UiWorld, y_sort: 0, sub_order: sub + 8 },
                tint: C_HP_BAR_BG, scale: [bar_w, 3.0],
            });
            self.batch.push(SpriteInstance {
                screen_pos: [sx - bar_w * 0.5, sy + bar_y],
                src: UNIT, atlas: ATLAS_0, material: MAT,
                sort_key: SortKey { layer: RenderLayer::UiWorld, y_sort: 0, sub_order: sub + 9 },
                tint: C_HP_BAR, scale: [bar_w * hp_ratio, 3.0],
            });

            if targeted {
                self.batch.push(SpriteInstance {
                    screen_pos: [sx - 10.0 * scale_mul, sy - 30.0 * scale_mul],
                    src: UNIT, atlas: ATLAS_0, material: MAT,
                    sort_key: SortKey { layer: RenderLayer::UiWorld, y_sort: 0, sub_order: sub + 7 },
                    tint: [1.0, 0.3, 0.2, 0.18], scale: [20.0 * scale_mul, 36.0 * scale_mul],
                });
            }

            // Monster affix label above HP bar
            if let Some(afx) = affix {
                let label = afx.label();
                let color = afx.color();
                self.draw_text_at(label, sx - label.len() as f32 * 3.0, sy + bar_y - 10.0,
                                  1.0, color, RenderLayer::UiWorld, 0, sub + 10);
            }

            // Freeze/slow visual overlay
            if frozen {
                self.spr(wx, wy, [-8.0 * scale_mul, -28.0 * scale_mul],
                         [16.0 * scale_mul, 32.0 * scale_mul],
                         RenderLayer::VfxAlpha, sub + 5, [0.40, 0.70, 1.0, 0.35]);
            } else if slowed {
                self.spr(wx, wy, [-6.0 * scale_mul, -24.0 * scale_mul],
                         [12.0 * scale_mul, 28.0 * scale_mul],
                         RenderLayer::VfxAlpha, sub + 5, [0.30, 0.55, 0.85, 0.20]);
            }
        }
    }

    fn draw_player(&mut self) {
        if self.player_dead { return; }
        let [wx, wy] = self.player_pos;
        let moving = self.keys != 0 || self.move_target.is_some();
        let fr = if self.facing_right { 1.0_f32 } else { -1.0 };

        // Walk animation (faster when running)
        let anim_div = if self.running { 4 } else { 6 };
        let leg_off = if moving {
            if (self.frame / anim_div) % 4 < 2 { 1.5 } else { -1.5 }
        } else { 0.0 };

        // Shadow
        self.spr(wx, wy, [-10.0, 2.0], [20.0, 7.0], RenderLayer::Decals, 0, C_SHADOW);
        // Feet
        self.spr(wx, wy, [-4.0 + leg_off, -2.0], [3.0, 7.0], RenderLayer::Entities, 0, C_BOOTS);
        self.spr(wx, wy, [1.0 - leg_off, -2.0], [3.0, 7.0], RenderLayer::Entities, 0, C_BOOTS);
        // Legs
        self.spr(wx, wy, [-4.0 + leg_off * 0.5, -8.0], [3.0, 8.0], RenderLayer::Entities, 0, C_PLAYER_DK);
        self.spr(wx, wy, [1.0 - leg_off * 0.5, -8.0], [3.0, 8.0], RenderLayer::Entities, 0, C_PLAYER_DK);
        // Body (tunic)
        self.spr(wx, wy, [-8.0, -26.0], [16.0, 20.0], RenderLayer::Entities, 1, C_PLAYER_BODY);
        // Belt
        self.spr(wx, wy, [-8.0, -12.0], [16.0, 3.0], RenderLayer::Entities, 2, C_PLAYER_DK);
        // Shoulders
        self.spr(wx, wy, [-10.0, -26.0], [20.0, 5.0], RenderLayer::Entities, 3, C_PLAYER_DK);
        // Shield (facing-dependent side)
        let shield_x = if self.facing_right { -14.0 } else { 7.0 };
        let shield_hl_x = if self.facing_right { -13.0 } else { 8.0 };
        self.spr(wx, wy, [shield_x, -22.0], [7.0, 14.0], RenderLayer::Entities, 4, C_SHIELD);
        self.spr(wx, wy, [shield_hl_x, -20.0], [3.0, 8.0], RenderLayer::Entities, 5, C_SHIELD_HL);
        // Weapon (opposite side — swing on attack)
        let swing = if self.attack_cd > 14 { 5.0 * fr } else { 0.0 };
        let wpn_x = if self.facing_right { 9.0 + swing } else { -13.0 + swing };
        self.spr(wx, wy, [wpn_x, -30.0 + swing.abs() * 0.5], [4.0, 22.0], RenderLayer::Entities, 6, C_WEAPON);
        // Head
        self.spr(wx, wy, [-5.0, -36.0], [10.0, 12.0], RenderLayer::Entities, 7, C_SKIN);
        // Hair
        self.spr(wx, wy, [-5.0, -40.0], [10.0, 6.0], RenderLayer::Entities, 8, C_HAIR);
        // Helmet
        self.spr(wx, wy, [-6.0, -42.0], [12.0, 6.0], RenderLayer::Entities, 9, C_HELMET);
        // Helmet crest
        self.spr(wx, wy, [-1.0, -44.0], [2.0, 4.0], RenderLayer::Entities, 10, C_METAL);

        // Player HP bar
        let [sx, sy] = self.w2s(wx, wy);
        let bar_w = 28.0;
        let hp_ratio = (self.player_hp / PLAYER_MAX_HP).clamp(0.0, 1.0);
        self.batch.push(SpriteInstance {
            screen_pos: [sx - bar_w * 0.5, sy - 52.0],
            src: UNIT, atlas: ATLAS_0, material: MAT,
            sort_key: SortKey { layer: RenderLayer::UiWorld, y_sort: 0, sub_order: 10 },
            tint: C_HP_BAR_BG, scale: [bar_w, 3.0],
        });
        self.batch.push(SpriteInstance {
            screen_pos: [sx - bar_w * 0.5, sy - 52.0],
            src: UNIT, atlas: ATLAS_0, material: MAT,
            sort_key: SortKey { layer: RenderLayer::UiWorld, y_sort: 0, sub_order: 11 },
            tint: [0.22, 0.92, 0.22, 0.92], scale: [bar_w * hp_ratio, 3.0],
        });
    }

    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    fn draw_floats(&mut self) {
        let floats_data: Vec<_> = self.floats.iter().enumerate().map(|(i, f)| {
            (i, f.wx, f.wy + f.dy, f.life, f.value, f.is_heal)
        }).collect();
        for (i, wx, wy, life, value, is_heal) in floats_data {
            let [sx, sy] = self.w2s(wx, wy);
            let alpha = (life as f32 / 40.0).clamp(0.0, 1.0);
            let c = if is_heal { C_HEAL_TEXT } else { C_DMG_TEXT };
            let tint = [c[0], c[1], c[2], c[3] * alpha];
            let (buf, len) = fmt_f32(value);
            let prefix = if is_heal { b"+" } else { b"-" };
            let mut text = [0u8; 12];
            text[0] = prefix[0];
            text[1..=len].copy_from_slice(&buf[..len]);
            let total = 1 + len;
            let px = if value >= 20.0 { 2.0 } else { 1.5 };
            let tw = total as f32 * 6.0 * px;
            // Shadow
            self.draw_text_at(&text[..total], sx - tw * 0.5 + 1.0, sy - 39.0,
                              px, [0.0, 0.0, 0.0, alpha * 0.5],
                              RenderLayer::UiWorld, 0, 900 + i as u32);
            // Text
            self.draw_text_at(&text[..total], sx - tw * 0.5, sy - 40.0,
                              px, tint, RenderLayer::UiWorld, 0, 910 + i as u32);
        }
    }

    // ======================================================================
    // Drops, projectiles, overlays
    // ======================================================================

    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    fn draw_drops(&mut self) {
        let drop_data: Vec<_> = self.drops.iter().enumerate().map(|(i, d)| {
            (i, d.pos, d.kind, d.life)
        }).collect();
        for (i, pos, kind, life) in drop_data {
            let sub = 1200 + i as u32 * 3;
            let alpha = if life < 60 { life as f32 / 60.0 } else { 1.0 };
            let bounce = ((self.frame as f32 * 0.08 + i as f32).sin() * 2.0).abs();
            match kind {
                DropKind::Gold => {
                    let c = [C_GOLD_DROP[0], C_GOLD_DROP[1], C_GOLD_DROP[2], alpha];
                    self.spr(pos[0], pos[1], [-3.0, -6.0 - bounce], [6.0, 6.0], RenderLayer::Props, sub, c);
                }
                DropKind::HealthPotion => {
                    let c = [C_HP_POT[0], C_HP_POT[1], C_HP_POT[2], alpha];
                    self.spr(pos[0], pos[1], [-2.5, -8.0 - bounce], [5.0, 8.0], RenderLayer::Props, sub, c);
                }
                DropKind::ManaPotion => {
                    let c = [C_MP_POT[0], C_MP_POT[1], C_MP_POT[2], alpha];
                    self.spr(pos[0], pos[1], [-2.5, -8.0 - bounce], [5.0, 8.0], RenderLayer::Props, sub, c);
                }
                DropKind::Equipment(slot, rarity) => {
                    let rc = rarity.color();
                    let c = [rc[0], rc[1], rc[2], alpha];
                    // Item glow
                    self.spr(pos[0], pos[1], [-8.0, -10.0 - bounce], [16.0, 12.0],
                             RenderLayer::Props, sub, [c[0] * 0.3, c[1] * 0.3, c[2] * 0.3, alpha * 0.3]);
                    // Item shape
                    self.spr(pos[0], pos[1], [-4.0, -8.0 - bounce], [8.0, 8.0],
                             RenderLayer::Props, sub + 1, c);
                    // Label with rarity-colored text
                    let name = slot.name();
                    let [sx, sy] = self.w2s(pos[0], pos[1]);
                    let tw = name.len() as f32 * 6.0;
                    self.draw_text_at(name, sx - tw * 0.5, sy - 18.0 - bounce, 1.0,
                                      [c[0], c[1], c[2], alpha * 0.9],
                                      RenderLayer::UiWorld, 0, sub + 2);
                }
                DropKind::Scroll(content) => {
                    // Scroll color: cyan-ish for passive, white for special
                    let sc = match content {
                        SlotContent::Passive { .. } => [0.50, 0.75, 1.0, alpha],
                        SlotContent::Special(_) => [0.95, 0.90, 0.70, alpha],
                        SlotContent::Active { .. } => [0.90, 0.55, 0.55, alpha],
                    };
                    // Scroll glow
                    self.spr(pos[0], pos[1], [-6.0, -8.0 - bounce], [12.0, 10.0],
                             RenderLayer::Props, sub, [sc[0] * 0.3, sc[1] * 0.3, sc[2] * 0.3, alpha * 0.35]);
                    // Scroll shape (small rectangle)
                    self.spr(pos[0], pos[1], [-3.0, -7.0 - bounce], [6.0, 9.0],
                             RenderLayer::Props, sub + 1, sc);
                    // Label
                    let label = content.short_label();
                    let [sx, sy] = self.w2s(pos[0], pos[1]);
                    let tw = label.len() as f32 * 6.0;
                    self.draw_text_at(label, sx - tw * 0.5, sy - 18.0 - bounce, 1.0,
                                      [sc[0], sc[1], sc[2], alpha * 0.9],
                                      RenderLayer::UiWorld, 0, sub + 2);
                }
            }
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn draw_projectiles(&mut self) {
        let proj_data: Vec<_> = self.projectiles.iter().enumerate().map(|(i, p)| {
            (i, p.pos, p.is_frost, p.friendly)
        }).collect();
        for (i, pos, is_frost, friendly) in proj_data {
            let sub = 1400 + i as u32;
            if is_frost {
                self.spr(pos[0], pos[1], [-6.0, -6.0], [12.0, 12.0],
                         RenderLayer::VfxAlpha, sub, [0.30, 0.55, 0.90, 0.20]);
                self.spr(pos[0], pos[1], [-3.0, -3.0], [6.0, 6.0],
                         RenderLayer::VfxAlpha, sub + 1, C_FROST_BOLT);
                self.spr(pos[0], pos[1], [-1.5, -1.5], [3.0, 3.0],
                         RenderLayer::VfxAlpha, sub + 2, C_FROST_CORE);
            } else if friendly {
                self.spr(pos[0], pos[1], [-8.0, -8.0], [16.0, 16.0],
                         RenderLayer::VfxAlpha, sub, [1.0, 0.35, 0.05, 0.25]);
                self.spr(pos[0], pos[1], [-4.0, -4.0], [8.0, 8.0],
                         RenderLayer::VfxAlpha, sub + 1, C_FIREBALL);
                self.spr(pos[0], pos[1], [-2.0, -2.0], [4.0, 4.0],
                         RenderLayer::VfxAlpha, sub + 2, C_FIREBALL_CORE);
            } else {
                // Enemy projectile (red spike)
                self.spr(pos[0], pos[1], [-3.0, -3.0], [6.0, 6.0],
                         RenderLayer::VfxAlpha, sub, [0.80, 0.20, 0.10, 0.85]);
                self.spr(pos[0], pos[1], [-1.5, -1.5], [3.0, 3.0],
                         RenderLayer::VfxAlpha, sub + 1, [1.0, 0.50, 0.20, 1.0]);
            }
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn draw_breakables(&mut self) {
        let data: Vec<_> = self.breakables.iter().enumerate()
            .filter_map(|(i, b)| if b.alive { Some((i, b.pos)) } else { None })
            .collect();
        for (i, pos) in data {
            let sub = 1500 + i as u32;
            self.spr(pos[0], pos[1], [-5.0, 2.0], [10.0, 5.0], RenderLayer::Decals, sub, C_SHADOW);
            self.spr(pos[0], pos[1], [-6.0, -10.0], [12.0, 14.0], RenderLayer::Props, sub + 1, C_BREAKABLE);
            self.spr(pos[0], pos[1], [-6.0, -4.0], [12.0, 2.0], RenderLayer::Props, sub + 2, C_BREAKABLE_DK);
            self.spr(pos[0], pos[1], [-6.0, -10.0], [12.0, 2.0], RenderLayer::Props, sub + 3, C_BREAKABLE_DK);
            self.spr(pos[0], pos[1], [-6.0, -12.0], [12.0, 5.0], RenderLayer::Props, sub + 4, C_BREAKABLE_DK);
        }
    }

    // ======================================================================
    // Gather nodes (trees, ore, herbs, mushrooms)
    // ======================================================================

    #[allow(clippy::cast_precision_loss)]
    fn draw_gather_nodes(&mut self) {
        let data: Vec<_> = self.gather_nodes.iter().enumerate()
            .map(|(i, n)| (i, n.pos, n.kind, n.is_alive(), n.is_depleted(), n.hp, n.max_hp))
            .collect();
        for (i, pos, kind, alive, depleted, hp, max_hp) in data {
            let sub = 2000 + i as u32 * 10;
            if depleted {
                // Stump / rubble for depleted nodes
                self.spr(pos[0], pos[1], [-4.0, 1.0], [8.0, 3.0], RenderLayer::Decals, sub, C_SHADOW);
                self.spr(pos[0], pos[1], [-3.0, -2.0], [6.0, 4.0], RenderLayer::Props, sub + 1, C_DEPLETED);
                continue;
            }
            if !alive { continue; }
            match kind {
                GatherNodeKind::Tree => {
                    // Shadow
                    self.spr(pos[0], pos[1], [-6.0, 2.0], [12.0, 4.0], RenderLayer::Decals, sub, C_SHADOW);
                    // Trunk
                    self.spr(pos[0], pos[1], [-3.0, -18.0], [6.0, 22.0], RenderLayer::Props, sub + 1, C_TRUNK);
                    self.spr(pos[0], pos[1], [-2.0, -16.0], [2.0, 18.0], RenderLayer::Props, sub + 2, C_TRUNK_DK);
                    // Canopy (layered for depth)
                    self.spr(pos[0], pos[1], [-14.0, -32.0], [28.0, 20.0], RenderLayer::VfxAlpha, sub + 3, C_CANOPY);
                    self.spr(pos[0], pos[1], [-10.0, -36.0], [20.0, 14.0], RenderLayer::VfxAlpha, sub + 4, C_CANOPY_HI);
                    self.spr(pos[0], pos[1], [-12.0, -26.0], [24.0, 8.0], RenderLayer::VfxAlpha, sub + 5, C_CANOPY_DK);
                }
                GatherNodeKind::HardwoodTree => {
                    // Bigger trunk, darker canopy
                    self.spr(pos[0], pos[1], [-8.0, 3.0], [16.0, 5.0], RenderLayer::Decals, sub, C_SHADOW);
                    self.spr(pos[0], pos[1], [-4.0, -22.0], [8.0, 26.0], RenderLayer::Props, sub + 1, C_TRUNK);
                    self.spr(pos[0], pos[1], [-3.0, -20.0], [3.0, 22.0], RenderLayer::Props, sub + 2, C_TRUNK_DK);
                    self.spr(pos[0], pos[1], [-18.0, -40.0], [36.0, 26.0], RenderLayer::VfxAlpha, sub + 3, C_HARDWOOD);
                    self.spr(pos[0], pos[1], [-14.0, -44.0], [28.0, 18.0], RenderLayer::VfxAlpha, sub + 4, C_CANOPY_HI);
                    self.spr(pos[0], pos[1], [-16.0, -34.0], [32.0, 10.0], RenderLayer::VfxAlpha, sub + 5, C_HARDWOOD_DK);
                }
                GatherNodeKind::OreRock => {
                    // Rock base + nugget highlights
                    self.spr(pos[0], pos[1], [-6.0, 1.0], [12.0, 4.0], RenderLayer::Decals, sub, C_SHADOW);
                    self.spr(pos[0], pos[1], [-8.0, -8.0], [16.0, 12.0], RenderLayer::Props, sub + 1, C_ORE_ROCK);
                    self.spr(pos[0], pos[1], [-6.0, -10.0], [12.0, 6.0], RenderLayer::Props, sub + 2, C_ORE_ROCK_DK);
                    // Nugget sparkles
                    self.spr(pos[0], pos[1], [-2.0, -6.0], [3.0, 3.0], RenderLayer::Props, sub + 3, C_ORE_NUGGET);
                    self.spr(pos[0], pos[1], [3.0, -4.0], [2.0, 2.0], RenderLayer::Props, sub + 4, C_ORE_NUGGET);
                }
                GatherNodeKind::RareOreVein => {
                    // Larger rock, purple-ish nuggets
                    self.spr(pos[0], pos[1], [-8.0, 2.0], [16.0, 5.0], RenderLayer::Decals, sub, C_SHADOW);
                    self.spr(pos[0], pos[1], [-10.0, -10.0], [20.0, 14.0], RenderLayer::Props, sub + 1, C_RARE_ORE);
                    self.spr(pos[0], pos[1], [-8.0, -12.0], [16.0, 8.0], RenderLayer::Props, sub + 2, C_ORE_ROCK_DK);
                    // Glowing nuggets
                    let pulse = ((self.frame as f32 * 0.04).sin() * 0.15 + 0.85).abs();
                    let glow = [C_RARE_NUGGET[0] * pulse, C_RARE_NUGGET[1] * pulse, C_RARE_NUGGET[2] * pulse, 1.0];
                    self.spr(pos[0], pos[1], [-3.0, -8.0], [4.0, 4.0], RenderLayer::Props, sub + 3, glow);
                    self.spr(pos[0], pos[1], [4.0, -6.0], [3.0, 3.0], RenderLayer::Props, sub + 4, glow);
                    self.spr(pos[0], pos[1], [-1.0, -3.0], [2.0, 2.0], RenderLayer::Props, sub + 5, glow);
                }
                GatherNodeKind::HerbBush => {
                    // Small bush with flowers
                    self.spr(pos[0], pos[1], [-4.0, 1.0], [8.0, 3.0], RenderLayer::Decals, sub, C_SHADOW);
                    self.spr(pos[0], pos[1], [-6.0, -6.0], [12.0, 10.0], RenderLayer::Props, sub + 1, C_HERB);
                    self.spr(pos[0], pos[1], [-2.0, -8.0], [3.0, 3.0], RenderLayer::Props, sub + 2, C_HERB_FLOWER);
                    self.spr(pos[0], pos[1], [2.0, -6.0], [2.0, 2.0], RenderLayer::Props, sub + 3, C_HERB_FLOWER);
                }
                GatherNodeKind::Mushroom => {
                    // Small mushroom
                    self.spr(pos[0], pos[1], [-2.0, 1.0], [4.0, 2.0], RenderLayer::Decals, sub, C_SHADOW);
                    self.spr(pos[0], pos[1], [-1.0, -4.0], [2.0, 6.0], RenderLayer::Props, sub + 1, C_MUSHROOM_STEM);
                    self.spr(pos[0], pos[1], [-4.0, -8.0], [8.0, 5.0], RenderLayer::Props, sub + 2, C_MUSHROOM_CAP);
                    self.spr(pos[0], pos[1], [-3.0, -9.0], [6.0, 2.0], RenderLayer::Props, sub + 3, [0.75, 0.30, 0.18, 1.0]);
                }
            }
            // HP bar when damaged
            if hp < max_hp {
                let ratio = hp as f32 / max_hp as f32;
                let bar_w = 14.0;
                self.spr(pos[0], pos[1], [-bar_w * 0.5 - 1.0, -38.0], [bar_w + 2.0, 4.0], RenderLayer::UiWorld, sub + 8, [0.0, 0.0, 0.0, 0.6]);
                self.spr(pos[0], pos[1], [-bar_w * 0.5, -37.0], [bar_w * ratio, 2.0], RenderLayer::UiWorld, sub + 9, [0.3, 0.8, 0.2, 0.9]);
            }
        }
    }

    #[allow(clippy::cast_precision_loss)]
    fn draw_portal(&mut self) {
        let [wx, wy] = self.portal_pos;
        if self.portal_zone != self.current_zone { return; }
        let pulse = ((self.frame as f32 * 0.06).sin() * 0.12 + 0.88).abs();
        let c = [C_PORTAL[0] * pulse, C_PORTAL[1] * pulse, C_PORTAL[2], C_PORTAL[3]];
        self.spr(wx, wy, [-12.0, -6.0], [24.0, 16.0], RenderLayer::VfxAlpha, 1600, c);
        self.spr(wx, wy, [-8.0, -30.0], [16.0, 32.0], RenderLayer::VfxAlpha, 1601, c);
        self.spr(wx, wy, [-5.0, -26.0], [10.0, 24.0], RenderLayer::VfxAlpha, 1602, C_PORTAL_CORE);
    }

    #[allow(clippy::cast_precision_loss)]
    fn draw_zone_exits(&mut self) {
        let zone = self.current_zone;
        for exit in ZONE_EXITS {
            if exit.from != zone { continue; }
            let pulse = ((self.frame as f32 * 0.05).sin() * 0.2 + 0.8).abs();
            let c = [C_ZONE_ARROW[0], C_ZONE_ARROW[1], C_ZONE_ARROW[2], C_ZONE_ARROW[3] * pulse];
            match exit.dir {
                0 => { // north
                    self.spr(12.0, 0.5, [-12.0, -4.0], [24.0, 8.0], RenderLayer::UiWorld, 1700, c);
                }
                1 => { // south
                    self.spr(12.0, 23.0, [-12.0, -4.0], [24.0, 8.0], RenderLayer::UiWorld, 1701, c);
                }
                2 => { // east
                    self.spr(23.0, 12.0, [-4.0, -12.0], [8.0, 24.0], RenderLayer::UiWorld, 1702, c);
                }
                3 => { // west
                    self.spr(1.0, 12.0, [-4.0, -12.0], [8.0, 24.0], RenderLayer::UiWorld, 1703, c);
                }
                _ => {}
            }
        }
    }

    #[allow(clippy::cast_precision_loss, clippy::many_single_char_names, clippy::cast_possible_truncation)]
    fn draw_quest_log(&mut self) {
        let x = 20.0;
        let y = 40.0;
        let w = 220.0;
        let row_h = 20.0;
        let quest_data: Vec<_> = self.quests.iter().enumerate().map(|(i, q)| {
            (i, q.done, q.name.as_bytes())
        }).collect();
        let h = 30.0 + quest_data.len() as f32 * row_h;
        self.hud(x - 2.0, y - 2.0, w + 4.0, h + 4.0, 980, C_STATS_FR);
        self.hud(x, y, w, h, 981, C_QUEST_BG);
        self.hud(x, y, w, 22.0, 982, C_STATS_FR);
        self.draw_text(b"QUESTS", x + 8.0, y + 5.0, 1.5, [1.0, 1.0, 1.0, 0.95], 983);
        for (i, done, name_bytes) in quest_data {
            let qy = y + 28.0 + i as f32 * row_h;
            let c = if done { C_QUEST_DONE } else { C_QUEST_ACTIVE };
            // Checkbox
            self.hud(x + 6.0, qy + 2.0, 12.0, 12.0, 984 + i as u32, c);
            if done {
                self.draw_text(b"X", x + 8.0, qy + 4.0, 1.5, [0.0, 0.0, 0.0, 0.8], 990 + i as u32);
            }
            // Quest name text
            self.draw_text(name_bytes, x + 22.0, qy + 4.0, 1.5, c, 996 + i as u32);
        }
    }

    #[allow(clippy::cast_precision_loss)]
    fn draw_death_overlay(&mut self) {
        self.hud(0.0, 0.0, self.viewport[0], self.viewport[1], 950, C_DEAD_OVERLAY);
        let cx = self.viewport[0] * 0.5;
        let cy = self.viewport[1] * 0.4;
        // "YOU DIED" text
        let died = b"YOU DIED";
        let died_tw = died.len() as f32 * 18.0;
        self.hud(cx - died_tw * 0.5 - 6.0, cy - 14.0, died_tw + 12.0, 28.0, 951,
                 [0.0, 0.0, 0.0, 0.55]);
        self.draw_text(died, cx - died_tw * 0.5, cy - 10.0, 3.0,
                       [0.85, 0.10, 0.10, 0.95], 954);
        // Respawn bar
        #[allow(clippy::cast_precision_loss)]
        let ratio = 1.0 - (self.player_respawn_timer as f32 / RESPAWN_DELAY as f32);
        self.hud(cx - 50.0, cy + 20.0, 100.0, 6.0, 952, [0.2, 0.2, 0.2, 0.8]);
        self.hud(cx - 50.0, cy + 20.0, 100.0 * ratio, 6.0, 953, [0.9, 0.9, 0.5, 0.9]);
        self.draw_text(b"RESPAWNING...", cx - 55.0, cy + 30.0, 1.5,
                       [0.8, 0.8, 0.6, 0.8], 955);
    }

    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::similar_names)]
    #[allow(clippy::similar_names, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn draw_stats(&mut self) {
        let cx = self.viewport[0] * 0.5;
        let cy = self.viewport[1] * 0.5;
        let w = 260.0;
        let h = 420.0;
        let white = [1.0, 1.0, 1.0, 0.95];
        let dim = [0.6, 0.55, 0.45, 0.7];
        let green = [0.4, 0.9, 0.4, 0.9];
        self.hud(cx - w * 0.5 - 2.0, cy - h * 0.5 - 2.0, w + 4.0, h + 4.0, 960, C_STATS_FR);
        self.hud(cx - w * 0.5, cy - h * 0.5, w, h, 961, C_STATS_BG);
        self.hud(cx - w * 0.5, cy - h * 0.5, w, 22.0, 962, C_STATS_FR);
        self.draw_text(b"CHARACTER", cx - w * 0.5 + 8.0, cy - h * 0.5 + 5.0, 1.5, white, 963);
        let x0 = cx - w * 0.5 + 10.0;
        let bw = w - 20.0;
        let mut y = cy - h * 0.5 + 30.0;
        let row_h = 14.0;
        let gap = 4.0;
        // Level
        let (lv_buf, lv_len) = fmt_u32(self.player_level);
        let mut lv_line = [0u8; 16];
        lv_line[..6].copy_from_slice(b"LEVEL ");
        lv_line[6..6 + lv_len].copy_from_slice(&lv_buf[..lv_len]);
        self.hud(x0, y, bw, row_h, 1960, C_STATS_TEXT);
        self.draw_text(&lv_line[..6 + lv_len], x0 + 2.0, y + 1.0, 1.5, white, 1961);
        y += row_h + gap;
        // HP bar
        let mhp = self.max_hp();
        let hp_r = (self.player_hp / mhp).clamp(0.0, 1.0);
        self.hud(x0, y, bw, row_h, 964, C_HP_BG);
        self.hud(x0, y, bw * hp_r, row_h, 965, C_HP);
        let (hp_buf, hp_len) = fmt_f32(self.player_hp);
        let (hpm_buf, hpm_len) = fmt_f32(mhp);
        let mut hp_line = [0u8; 24];
        hp_line[..3].copy_from_slice(b"HP ");
        hp_line[3..3 + hp_len].copy_from_slice(&hp_buf[..hp_len]);
        hp_line[3 + hp_len] = b'/';
        hp_line[4 + hp_len..4 + hp_len + hpm_len].copy_from_slice(&hpm_buf[..hpm_len]);
        let hp_total = 4 + hp_len + hpm_len;
        self.draw_text(&hp_line[..hp_total], x0 + 2.0, y + 2.0, 1.5, white, 1962);
        y += row_h + gap;
        // MP bar
        let mmp = self.max_mp();
        let mp_r = (self.player_mp / mmp).clamp(0.0, 1.0);
        self.hud(x0, y, bw, row_h, 966, C_MP_BG);
        self.hud(x0, y, bw * mp_r, row_h, 967, C_MP);
        let (mp_buf, mp_len) = fmt_f32(self.player_mp);
        let (mpm_buf, mpm_len) = fmt_f32(mmp);
        let mut mp_line = [0u8; 24];
        mp_line[..3].copy_from_slice(b"MP ");
        mp_line[3..3 + mp_len].copy_from_slice(&mp_buf[..mp_len]);
        mp_line[3 + mp_len] = b'/';
        mp_line[4 + mp_len..4 + mp_len + mpm_len].copy_from_slice(&mpm_buf[..mpm_len]);
        let mp_total = 4 + mp_len + mpm_len;
        self.draw_text(&mp_line[..mp_total], x0 + 2.0, y + 2.0, 1.5, white, 1963);
        y += row_h + gap;
        // XP bar
        let xp_need = self.player_level as f32 * XP_PER_LEVEL;
        let xp_r = (self.player_xp / xp_need).clamp(0.0, 1.0);
        self.hud(x0, y, bw, row_h, 968, [0.10, 0.08, 0.04, 1.0]);
        self.hud(x0, y, bw * xp_r, row_h, 969, C_XP);
        let (xp_buf, xp_len) = fmt_f32(self.player_xp);
        let (xpn_buf, xpn_len) = fmt_f32(xp_need);
        let mut xp_line = [0u8; 24];
        xp_line[..3].copy_from_slice(b"XP ");
        xp_line[3..3 + xp_len].copy_from_slice(&xp_buf[..xp_len]);
        xp_line[3 + xp_len] = b'/';
        xp_line[4 + xp_len..4 + xp_len + xpn_len].copy_from_slice(&xpn_buf[..xpn_len]);
        let xp_total = 4 + xp_len + xpn_len;
        self.draw_text(&xp_line[..xp_total], x0 + 2.0, y + 2.0, 1.5, white, 1964);
        y += row_h + gap + 4.0;
        // --- Separator ---
        self.hud(x0, y, bw, 1.0, 1965, [0.5, 0.45, 0.35, 0.4]);
        y += 6.0;
        // --- STATS with effect details ---
        let stat_bar_c = [0.18, 0.15, 0.10, 0.8];
        let stat_fill = [0.45, 0.38, 0.22, 0.9];
        let stat_row = 28.0; // taller rows for stat + effect

        // Precompute stat values for display
        let ts = self.total_str();
        let td = self.total_dex();
        let tv = self.total_vit();
        let te = self.total_ene();
        let melee_dmg = self.melee_damage();
        let atk_cd = self.attack_cooldown();
        let dodge_pct = (self.dodge_chance() * 100.0) as u32;
        let max_hp_val = self.max_hp();
        let dmg_red = self.damage_reduction();
        let max_mp_val = self.max_mp();
        let spell_pct = ((self.spell_bonus() - 1.0) * 100.0) as u32;

        // STR
        let (sv, sl) = fmt_u32(ts);
        self.hud(x0, y, bw, stat_row, 970, stat_bar_c);
        let fill_w = (ts as f32 / 50.0 * bw).min(bw);
        self.hud(x0, y, fill_w, stat_row, 975, stat_fill);
        let mut line = [0u8; 16];
        line[..4].copy_from_slice(b"STR ");
        line[4..4 + sl].copy_from_slice(&sv[..sl]);
        self.draw_text(&line[..4 + sl], x0 + 2.0, y + 2.0, 1.5, white, 1970);
        // Effect: DMG XX (+0.4/pt)
        let (dv, dl) = fmt_f32(melee_dmg);
        let mut eff = [0u8; 24];
        eff[..4].copy_from_slice(b"DMG ");
        eff[4..4 + dl].copy_from_slice(&dv[..dl]);
        eff[4 + dl..4 + dl + 8].copy_from_slice(b" +0.4/PT");
        self.draw_text(&eff[..dl + 12], x0 + 2.0, y + 15.0, 1.0, green, 1980);
        y += stat_row + gap;

        // DEX
        let (sv, sl) = fmt_u32(td);
        self.hud(x0, y, bw, stat_row, 971, stat_bar_c);
        let fill_w = (td as f32 / 50.0 * bw).min(bw);
        self.hud(x0, y, fill_w, stat_row, 976, stat_fill);
        let mut line = [0u8; 16];
        line[..4].copy_from_slice(b"DEX ");
        line[4..4 + sl].copy_from_slice(&sv[..sl]);
        self.draw_text(&line[..4 + sl], x0 + 2.0, y + 2.0, 1.5, white, 1971);
        // Effect: CD XX  DODGE XX%
        let (cv, cl) = fmt_u32(atk_cd);
        let (dv2, dl2) = fmt_u32(dodge_pct);
        let mut eff = [0u8; 30];
        eff[..3].copy_from_slice(b"CD ");
        eff[3..3 + cl].copy_from_slice(&cv[..cl]);
        let mut p = 3 + cl;
        eff[p..p + 7].copy_from_slice(b" DODGE ");
        p += 7;
        eff[p..p + dl2].copy_from_slice(&dv2[..dl2]);
        p += dl2;
        eff[p] = b'%';
        p += 1;
        self.draw_text(&eff[..p], x0 + 2.0, y + 15.0, 1.0, green, 1981);
        y += stat_row + gap;

        // VIT
        let (sv, sl) = fmt_u32(tv);
        self.hud(x0, y, bw, stat_row, 972, stat_bar_c);
        let fill_w = (tv as f32 / 50.0 * bw).min(bw);
        self.hud(x0, y, fill_w, stat_row, 977, stat_fill);
        let mut line = [0u8; 16];
        line[..4].copy_from_slice(b"VIT ");
        line[4..4 + sl].copy_from_slice(&sv[..sl]);
        self.draw_text(&line[..4 + sl], x0 + 2.0, y + 2.0, 1.5, white, 1972);
        // Effect: HP XXX  DEF X.X
        let (hv, hl) = fmt_f32(max_hp_val);
        let (rv, rl) = fmt_f32(dmg_red);
        let mut eff = [0u8; 30];
        eff[..3].copy_from_slice(b"HP ");
        eff[3..3 + hl].copy_from_slice(&hv[..hl]);
        let mut p = 3 + hl;
        eff[p..p + 5].copy_from_slice(b" DEF ");
        p += 5;
        eff[p..p + rl].copy_from_slice(&rv[..rl]);
        p += rl;
        self.draw_text(&eff[..p], x0 + 2.0, y + 15.0, 1.0, green, 1982);
        y += stat_row + gap;

        // ENE
        let (sv, sl) = fmt_u32(te);
        self.hud(x0, y, bw, stat_row, 973, stat_bar_c);
        let fill_w = (te as f32 / 50.0 * bw).min(bw);
        self.hud(x0, y, fill_w, stat_row, 978, stat_fill);
        let mut line = [0u8; 16];
        line[..4].copy_from_slice(b"ENE ");
        line[4..4 + sl].copy_from_slice(&sv[..sl]);
        self.draw_text(&line[..4 + sl], x0 + 2.0, y + 2.0, 1.5, white, 1973);
        // Effect: MP XXX  SPELL +XX%
        let (mv, ml) = fmt_f32(max_mp_val);
        let (spv, spl) = fmt_u32(spell_pct);
        let mut eff = [0u8; 30];
        eff[..3].copy_from_slice(b"MP ");
        eff[3..3 + ml].copy_from_slice(&mv[..ml]);
        let mut p = 3 + ml;
        eff[p..p + 8].copy_from_slice(b" SPELL +");
        p += 8;
        eff[p..p + spl].copy_from_slice(&spv[..spl]);
        p += spl;
        eff[p] = b'%';
        p += 1;
        self.draw_text(&eff[..p], x0 + 2.0, y + 15.0, 1.0, green, 1983);
        y += stat_row + gap;

        // Stat points
        if self.stat_points > 0 {
            let (sp_buf, sp_len) = fmt_u32(self.stat_points);
            let mut sp_line = [0u8; 20];
            sp_line[..7].copy_from_slice(b"POINTS ");
            sp_line[7..7 + sp_len].copy_from_slice(&sp_buf[..sp_len]);
            let sp_total = 7 + sp_len;
            let sp_tw = sp_total as f32 * 9.0 + 6.0;
            self.hud(x0, y, sp_tw, row_h, 979, C_XP);
            self.draw_text(&sp_line[..sp_total], x0 + 2.0, y + 2.0, 1.5, white, 1975);
            y += row_h + 2.0;
            self.draw_text(b"F6 STR  F7 DEX  F8 VIT  F9 ENE", x0 + 2.0, y, 1.0, dim, 1976);
            y += 10.0;
        }
        y += gap;
        // Gold
        let (gb, gl) = fmt_u32(self.player_gold);
        let mut gld = [0u8; 16];
        gld[..5].copy_from_slice(b"GOLD ");
        gld[5..5 + gl].copy_from_slice(&gb[..gl]);
        let gt = 5 + gl;
        let gw = (gt as f32 * 9.0 + 6.0).min(bw);
        self.hud(x0, y, gw, row_h, 974, C_GOLD_DROP);
        self.draw_text(&gld[..gt], x0 + 2.0, y + 2.0, 1.5, [0.15, 0.10, 0.0, 1.0], 1977);
        y += row_h + gap;

        // Resistances
        let eff = self.effective_resistances();
        let res_data: [(&[u8], f32, [f32; 4]); 4] = [
            (b"FIRE ", eff.fire, [1.0, 0.45, 0.10, 1.0]),
            (b"COLD ", eff.cold, [0.40, 0.70, 1.0, 1.0]),
            (b"LGHT ", eff.lightning, [0.60, 0.70, 1.0, 1.0]),
            (b"POIS ", eff.poison, [0.20, 0.65, 0.10, 1.0]),
        ];
        self.draw_text(b"RESISTANCES", x0 + 2.0, y, 1.0, C_STATS_TEXT, 1980);
        y += 10.0;
        for (ri, (label, val, color)) in res_data.iter().enumerate() {
            let pct = (*val * 100.0) as i32;
            let (vb, vl) = fmt_u32(pct.unsigned_abs());
            let mut line = [0u8; 16];
            line[..label.len()].copy_from_slice(label);
            let off = label.len();
            if pct < 0 { line[off] = b'-'; line[off + 1..off + 1 + vl].copy_from_slice(&vb[..vl]); line[off + 1 + vl] = b'%'; }
            else { line[off..off + vl].copy_from_slice(&vb[..vl]); line[off + vl] = b'%'; }
            let total = if pct < 0 { off + 2 + vl } else { off + vl + 1 };
            self.draw_text(&line[..total], x0 + 2.0, y, 1.0, *color, 1981 + ri as u32);
            y += 9.0;
        }
    }

    // ======================================================================
    // HUD
    // ======================================================================

    #[allow(clippy::cast_precision_loss)]
    fn draw_hud(&mut self) {
        let vw = self.viewport[0];
        let vh = self.viewport[1];
        // ---- Thinner D2-style HUD: 64px bar (was 100px) ----
        let bar_h = 64.0;
        let bar_y = vh - bar_h;

        // Main bar background + top frame line
        self.hud(0.0, bar_y, vw, bar_h, 0, C_HUD_BG);
        self.hud(0.0, bar_y, vw, 2.0, 50, C_HUD_FRAME);

        // HP orb — compact 52px (was 78px)
        let orb = 52.0;
        let orb_y = bar_y + 4.0;
        let (hx, hy) = (6.0, orb_y);
        self.hud(hx - 2.0, hy - 2.0, orb + 4.0, orb + 4.0, 2, C_HUD_FRAME);
        self.hud(hx, hy, orb, orb, 3, C_HP_BG);
        let hp_fill = orb * (self.player_hp / self.max_hp()).clamp(0.0, 1.0);
        self.hud(hx, hy + orb - hp_fill, orb, hp_fill, 4, C_HP);
        // HP text
        let (hp_buf, hp_len) = fmt_f32(self.player_hp);
        let hp_tw = hp_len as f32 * 10.0;
        self.draw_text(&hp_buf[..hp_len], hx + (orb - hp_tw) * 0.5, hy + orb * 0.5 - 5.0,
                       1.5, [1.0, 1.0, 1.0, 0.9], 55);

        // MP orb — compact 52px
        let (mx, my) = (vw - 6.0 - orb, orb_y);
        self.hud(mx - 2.0, my - 2.0, orb + 4.0, orb + 4.0, 5, C_HUD_FRAME);
        self.hud(mx, my, orb, orb, 6, C_MP_BG);
        let mp_fill = orb * (self.player_mp / self.max_mp()).clamp(0.0, 1.0);
        self.hud(mx, my + orb - mp_fill, orb, mp_fill, 7, C_MP);
        // MP text
        let (mp_buf, mp_len) = fmt_f32(self.player_mp);
        let mp_tw = mp_len as f32 * 10.0;
        self.draw_text(&mp_buf[..mp_len], mx + (orb - mp_tw) * 0.5, my + orb * 0.5 - 5.0,
                       1.5, [1.0, 1.0, 1.0, 0.9], 57);

        // ---- Central area: skills + belt ----
        let inner_l = hx + orb + 8.0;       // after HP orb
        let inner_r = mx - 8.0;              // before MP orb
        let inner_w = inner_r - inner_l;

        // Skill buttons — compact 28px (was 40px)
        let sk = 28.0;
        let sk_gap = 3.0;
        let sky = bar_y + 6.0;

        // Left skill pair (F1 Melee, F2 Fireball)
        let melee_hl = if self.active_skill == SkillId::Melee { C_XP } else { C_SLOT };
        let fire_hl = if self.active_skill == SkillId::Fireball { C_FIREBALL } else { C_SLOT };
        self.hud(inner_l, sky, sk, sk, 19, C_SLOT_FR);
        self.hud(inner_l + 1.0, sky + 1.0, sk - 2.0, sk - 2.0, 20, melee_hl);
        self.draw_text(b"F1", inner_l + 2.0, sky + sk - 9.0, 0.8, [0.6, 0.6, 0.5, 0.5], 61);
        self.hud(inner_l + sk + sk_gap, sky, sk, sk, 19, C_SLOT_FR);
        self.hud(inner_l + sk + sk_gap + 1.0, sky + 1.0, sk - 2.0, sk - 2.0, 21, fire_hl);
        self.draw_text(b"F2", inner_l + sk + sk_gap + 2.0, sky + sk - 9.0, 0.8, [0.6, 0.6, 0.5, 0.5], 62);

        // Belt slots — compact 26px (was 36px), centered
        let slot = 26.0;
        let gap = 3.0;
        let belt_w = 4.0 * slot + 3.0 * gap;
        let bx = inner_l + (inner_w - belt_w) * 0.5;
        let by = bar_y + 36.0;
        let belt_labels: [&[u8]; 4] = [b"1", b"2", b"3", b"4"];
        for i in 0_u32..4 {
            let sx = bx + i as f32 * (slot + gap);
            self.hud(sx - 1.0, by - 1.0, slot + 2.0, slot + 2.0, 10 + i, C_SLOT_FR);
            self.hud(sx, by, slot, slot, 15 + i, C_SLOT);
            self.draw_text(belt_labels[i as usize], sx + 1.0, by + 1.0,
                           0.8, [0.6, 0.6, 0.5, 0.4], 94 + i);
            if let Some(kind) = self.belt[i as usize] {
                let c = match kind {
                    PotionKind::Health => C_HP_POT,
                    PotionKind::Mana => C_MP_POT,
                };
                self.hud(sx + 6.0, by + 2.0, 14.0, 22.0, 90 + i, c);
            }
        }

        // Right skill trio (F3 Frost, F4 Lightning, F5 Teleport)
        let frost_hl = if self.active_skill == SkillId::FrostBolt { C_FROST_BOLT } else { C_SLOT };
        let lightning_hl = if self.active_skill == SkillId::ChainLightning { C_LIGHTNING } else { C_SLOT };
        let tp_hl = if self.active_skill == SkillId::Teleport { C_PORTAL_CORE } else { C_SLOT };
        let rsk = inner_r - 3.0 * (sk + sk_gap) + sk_gap;
        self.hud(rsk, sky, sk, sk, 19, C_SLOT_FR);
        self.hud(rsk + 1.0, sky + 1.0, sk - 2.0, sk - 2.0, 22, frost_hl);
        self.draw_text(b"F3", rsk + 2.0, sky + sk - 9.0, 0.8, [0.6, 0.6, 0.5, 0.5], 63);
        self.hud(rsk + sk + sk_gap, sky, sk, sk, 19, C_SLOT_FR);
        self.hud(rsk + sk + sk_gap + 1.0, sky + 1.0, sk - 2.0, sk - 2.0, 23, lightning_hl);
        self.draw_text(b"F4", rsk + sk + sk_gap + 2.0, sky + sk - 9.0, 0.8, [0.6, 0.6, 0.5, 0.5], 66);
        self.hud(rsk + 2.0 * (sk + sk_gap), sky, sk, sk, 19, C_SLOT_FR);
        self.hud(rsk + 2.0 * (sk + sk_gap) + 1.0, sky + 1.0, sk - 2.0, sk - 2.0, 24, tp_hl);
        self.draw_text(b"F5", rsk + 2.0 * (sk + sk_gap) + 2.0, sky + sk - 9.0, 0.8, [0.6, 0.6, 0.5, 0.5], 67);

        // Gold counter — centered above belt
        let (gold_buf, gold_len) = fmt_u32(self.player_gold);
        let gold_tw = gold_len as f32 * 7.0 + 14.0;
        let gold_cx = inner_l + inner_w * 0.5;
        self.hud(gold_cx - gold_tw * 0.5, bar_y + 5.0, gold_tw, 13.0, 45, [0.0, 0.0, 0.0, 0.55]);
        self.hud(gold_cx - gold_tw * 0.5 + 2.0, bar_y + 6.5, 8.0, 8.0, 46, C_GOLD_DROP);
        self.draw_text(&gold_buf[..gold_len], gold_cx - gold_tw * 0.5 + 12.0, bar_y + 6.0,
                       1.2, [0.95, 0.85, 0.35, 0.95], 59);

        // Run indicator
        if self.running {
            self.hud(gold_cx + gold_tw * 0.5 + 4.0, bar_y + 5.0, 28.0, 13.0, 47, [0.20, 0.55, 0.20, 0.75]);
            self.draw_text(b"RUN", gold_cx + gold_tw * 0.5 + 6.0, bar_y + 6.0,
                           1.2, [0.9, 1.0, 0.9, 0.9], 60);
        }

        // Level badge — centered above gold
        let (lv_buf, lv_len) = fmt_u32(self.player_level);
        let mut lv_text = [0u8; 14];
        lv_text[..3].copy_from_slice(b"LV ");
        lv_text[3..3 + lv_len].copy_from_slice(&lv_buf[..lv_len]);
        let lv_total = 3 + lv_len;
        let lv_tw = lv_total as f32 * 7.0 + 4.0;
        self.hud(gold_cx - lv_tw * 0.5, bar_y + 20.0, lv_tw, 12.0, 40, [0.48, 0.40, 0.14, 0.88]);
        self.draw_text(&lv_text[..lv_total], gold_cx - lv_tw * 0.5 + 2.0, bar_y + 21.0,
                       1.2, [1.0, 0.95, 0.60, 1.0], 64);

        // XP bar — thinner 4px (was 6px)
        let xp_ratio = (self.player_xp / (self.player_level as f32 * XP_PER_LEVEL)).clamp(0.0, 1.0);
        self.hud(0.0, vh - 4.0, vw, 4.0, 30, [0.10, 0.08, 0.04, 1.0]);
        self.hud(0.0, vh - 4.0, vw * xp_ratio, 4.0, 31, C_XP);

        // Zone name indicator (top-left)
        let zone_name: &[u8] = self.current_zone.label().as_bytes();
        let zone_tw = zone_name.len() as f32 * 9.0 + 6.0;
        self.hud(10.0, 10.0, zone_tw, 16.0, 48, [0.0, 0.0, 0.0, 0.55]);
        let zone_c = if self.is_camp() { C_NPC_LABEL } else { C_STATS_TEXT };
        self.draw_text(zone_name, 13.0, 12.0, 1.5, zone_c, 65);

        // Difficulty indicator (top-left, below zone name)
        if self.difficulty != Difficulty::Normal {
            let diff_label = self.difficulty.label();
            let diff_c = match self.difficulty {
                Difficulty::Nightmare => [1.0, 0.60, 0.20, 1.0],
                Difficulty::Hell => [1.0, 0.20, 0.15, 1.0],
                Difficulty::Normal => [0.8, 0.8, 0.8, 1.0],
            };
            let diff_tw = diff_label.len() as f32 * 9.0 + 6.0;
            self.hud(10.0, 28.0, diff_tw, 16.0, 49, [0.0, 0.0, 0.0, 0.55]);
            self.draw_text(diff_label, 13.0, 30.0, 1.5, diff_c, 68);
        }
    }

    // ======================================================================
    // Combat log
    // ======================================================================

    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    fn draw_combat_log(&mut self) {
        let vh = self.viewport[1];
        let vw = self.viewport[0];
        let bar_y = vh - 64.0;
        let log_x = vw * 0.5 - 120.0;
        let base_y = bar_y - 14.0;
        let log_data: Vec<_> = self.log.iter().enumerate().map(|(i, e)| {
            (i, e.text, e.len, e.color, e.life)
        }).collect();
        for (i, text, len, color, life) in log_data.iter().rev().take(6) {
            let alpha = (*life as f32 / LOG_LIFE as f32).clamp(0.0, 1.0);
            let y = base_y - *i as f32 * 12.0;
            let c = [color[0], color[1], color[2], color[3] * alpha];
            self.draw_text(&text[..*len], log_x, y, 1.0, c, 2000 + *i as u32);
        }
    }

    // ======================================================================
    // Inventory screen
    // ======================================================================

    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::many_single_char_names)]
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::many_single_char_names)]
    fn draw_inventory(&mut self) {
        let vh = self.viewport[1];
        let vw = self.viewport[0];
        let pw = 240.0;  // panel width
        let h = (vh - 16.0).min(540.0);
        let y = 8.0;
        let white = [1.0, 1.0, 1.0, 0.95];
        let dim = [0.6, 0.55, 0.45, 0.7];
        let green = [0.4, 0.9, 0.4, 0.9];
        let cyan = [0.4, 0.85, 0.85, 0.85];

        // --- Left panel: Equipment (flush left) ---
        let lx = 4.0;
        self.hud(lx - 2.0, y - 2.0, pw + 4.0, h + 4.0, 2100, C_STATS_FR);
        self.hud(lx, y, pw, h, 2101, C_STATS_BG);
        self.draw_text(b"EQUIPEMENT", lx + 8.0, y + 4.0, 1.5, white, 2102);
        // Gold display
        let (gb, gl) = fmt_u32(self.player_gold);
        let mut gold_line = [0u8; 14];
        gold_line[..gl].copy_from_slice(&gb[..gl]);
        gold_line[gl] = b'G';
        self.draw_text(&gold_line[..=gl], lx + pw - (gl as f32 + 1.0) * 8.0 - 8.0, y + 4.0, 1.5, C_GOLD_DROP, 2109);
        let content_y = y + 22.0;
        self.draw_inv_equip(lx + 8.0, content_y, pw - 16.0, white, dim, green, cyan);
        // Footer
        self.draw_text(b"E:USE G:SELL X:DROP", lx + 8.0, y + h - 14.0, 1.0, [0.5, 0.5, 0.45, 0.6], 2199);

        // --- Right panel: Backpack (flush right) ---
        let rx = vw - pw - 4.0;
        self.hud(rx - 2.0, y - 2.0, pw + 4.0, h + 4.0, 2500, C_STATS_FR);
        self.hud(rx, y, pw, h, 2501, C_STATS_BG);
        self.draw_text(b"SAC", rx + 8.0, y + 4.0, 1.5, white, 2502);
        let content_y_r = y + 22.0;
        self.draw_inv_backpack(rx + 8.0, content_y_r, pw - 16.0, white, dim);
        self.draw_text(b"TAB:STATS", rx + 8.0, y + h - 14.0, 1.0, [0.5, 0.5, 0.45, 0.6], 2599);
    }

    // --- Tab 0: Equipment ---
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::many_single_char_names)]
    fn draw_inv_equip(&mut self, sx: f32, start_y: f32, iw: f32, _white: [f32; 4], dim: [f32; 4], green: [f32; 4], cyan: [f32; 4]) {
        let slot_names: [&[u8]; EQUIP_SLOT_COUNT] = [b"HELM", b"ARMOR", b"SWORD", b"BOOTS", b"GLOVES", b"RING"];
        let row_h = 38.0;
        let mut sy = start_y;

        for (i, name) in slot_names.iter().enumerate() {
            self.hud(sx, sy, iw, 33.0, 2110 + i as u32, [0.12, 0.10, 0.08, 0.8]);
            self.draw_text(name, sx + 4.0, sy + 1.0, 1.0, dim, 2120 + i as u32);
            if let Some(eq) = self.equipment[i] {
                let rn = eq.rarity.name();
                let sn = eq.slot.name();
                let mut label = [0u8; 30];
                let e1 = rn.len().min(24);
                label[..e1].copy_from_slice(&rn[..e1]);
                let e2 = (e1 + sn.len()).min(24);
                label[e1..e2].copy_from_slice(&sn[..e2 - e1]);
                let (iv, il) = fmt_u32(eq.ilvl);
                let e3 = (e2 + 4).min(28);
                label[e2..e3].copy_from_slice(&b" iL "[..e3 - e2]);
                let e4 = (e3 + il).min(30);
                label[e3..e4].copy_from_slice(&iv[..e4 - e3]);
                self.draw_text(&label[..e4], sx + 4.0, sy + 10.0, 1.5, eq.rarity.color(), 2130 + i as u32);
                // Stat bonuses
                let (bs, bd, bv, be) = eq.bonuses();
                let mut bonus = [0u8; 30];
                let mut p = 0;
                if bs > 0 { let (b, l) = fmt_u32(bs); bonus[p] = b'+'; p += 1; bonus[p..p + l].copy_from_slice(&b[..l]); p += l; bonus[p..p + 2].copy_from_slice(b"S "); p += 2; }
                if bd > 0 { let (b, l) = fmt_u32(bd); bonus[p] = b'+'; p += 1; bonus[p..p + l].copy_from_slice(&b[..l]); p += l; bonus[p..p + 2].copy_from_slice(b"D "); p += 2; }
                if bv > 0 { let (b, l) = fmt_u32(bv); bonus[p] = b'+'; p += 1; bonus[p..p + l].copy_from_slice(&b[..l]); p += l; bonus[p..p + 2].copy_from_slice(b"V "); p += 2; }
                if be > 0 { let (b, l) = fmt_u32(be); bonus[p] = b'+'; p += 1; bonus[p..p + l].copy_from_slice(&b[..l]); p += l; bonus[p..p + 2].copy_from_slice(b"E "); p += 2; }
                let eff = eq.slot.effect_label();
                if p + eff.len() + 1 < 30 {
                    bonus[p] = b'|'; p += 1;
                    bonus[p..p + eff.len()].copy_from_slice(eff); p += eff.len();
                }
                self.draw_text(&bonus[..p], sx + 4.0, sy + 22.0, 1.0, green, 2140 + i as u32);
                let ls = eq.life_steal();
                let ms = eq.mana_steal();
                if ls > 0.0 || ms > 0.0 {
                    let steal_pct = ((ls + ms) * 100.0) as u32;
                    let (sv, sl) = fmt_u32(steal_pct);
                    let mut st = [0u8; 16];
                    let tag = if ls > 0.0 { b"STEAL " } else { b"MSTEAL" };
                    st[..6].copy_from_slice(tag);
                    st[6..6 + sl].copy_from_slice(&sv[..sl]);
                    st[6 + sl] = b'%';
                    self.draw_text(&st[..7 + sl], sx + iw - 60.0, sy + 22.0, 1.0, cyan, 2180 + i as u32);
                }
                let (gv, gl) = fmt_u32(eq.sell_value());
                let mut sv = [0u8; 10];
                sv[..gl].copy_from_slice(&gv[..gl]);
                sv[gl] = b'G';
                self.draw_text(&sv[..=gl], sx + iw - (gl as f32 + 1.0) * 7.0 - 4.0, sy + 1.0, 1.0, C_GOLD_DROP, 2190 + i as u32);
            } else {
                self.draw_text(b"-- VIDE --", sx + 4.0, sy + 14.0, 1.5, [0.4, 0.35, 0.30, 0.4], 2130 + i as u32);
            }
            sy += row_h;
        }

        // Skill grid summary
        sy += 4.0;
        self.hud(sx, sy, iw, 1.0, 2170, [0.5, 0.45, 0.35, 0.4]);
        sy += 5.0;
        let unlocked = self.skill_grid.count_unlocked() as u32;
        let filled = self.skill_grid.count_filled() as u32;
        let (uc_buf, uc_len) = fmt_u32(unlocked);
        let (fc_buf, fc_len) = fmt_u32(filled);
        let mut sl_line = [0u8; 30];
        sl_line[..7].copy_from_slice(b"GRILLE ");
        sl_line[7..7 + fc_len].copy_from_slice(&fc_buf[..fc_len]);
        sl_line[7 + fc_len] = b'/';
        sl_line[8 + fc_len..8 + fc_len + uc_len].copy_from_slice(&uc_buf[..uc_len]);
        let sl_end = 8 + fc_len + uc_len;
        self.draw_text(&sl_line[..sl_end], sx, sy, 1.0, [0.7, 0.65, 0.55, 0.8], 2175);
        self.draw_text(b"K=GRILLE", sx + iw - 60.0, sy, 1.0, [0.6, 0.55, 0.45, 0.7], 2179);
    }

    // --- Backpack list ---
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::many_single_char_names)]
    fn draw_inv_backpack(&mut self, sx: f32, start_y: f32, iw: f32, _white: [f32; 4], dim: [f32; 4]) {
        let mut sy = start_y;

        // Capacity header
        let count = self.backpack.count() as u32;
        let cap = self.backpack.capacity as u32;
        let (cnt_buf, cnt_len) = fmt_u32(count);
        let (cap_buf, cap_len) = fmt_u32(cap);
        let mut hdr = [0u8; 20];
        hdr[..cnt_len].copy_from_slice(&cnt_buf[..cnt_len]);
        hdr[cnt_len] = b'/';
        hdr[1 + cnt_len..1 + cnt_len + cap_len].copy_from_slice(&cap_buf[..cap_len]);
        let hdr_end = 1 + cnt_len + cap_len;
        self.draw_text(&hdr[..hdr_end], sx + iw - (hdr_end as f32) * 8.0, sy, 1.0, dim, 2200);
        sy += 14.0;

        // List layout: one row per slot, icon + label
        let row_h = 22.0;
        let icon_sz = 16.0;
        let selected = self.inv_hover_slot;

        // Snapshot backpack data for borrow safety
        let max_slots = self.backpack.capacity;
        let mut item_snap: Vec<Option<(InvItem, [f32; 4], &'static [u8])>> = Vec::with_capacity(max_slots);
        for i in 0..max_slots {
            if let Some(item) = self.backpack.get(i) {
                item_snap.push(Some((*item, item.color(), item.label())));
            } else {
                item_snap.push(None);
            }
        }

        for idx in 0..max_slots {
            let cy = sy + idx as f32 * row_h;
            let z = 2210 + idx as u32;
            let is_selected = selected == Some(idx);

            // Row background
            let bg = if is_selected {
                [0.25, 0.22, 0.18, 0.95]
            } else if idx % 2 == 0 {
                [0.10, 0.08, 0.06, 0.7]
            } else {
                [0.13, 0.11, 0.09, 0.7]
            };
            self.hud(sx, cy, iw, row_h - 2.0, z, bg);

            // Selection highlight bar
            if is_selected {
                self.hud(sx, cy, 2.0, row_h - 2.0, z + 100, [0.9, 0.8, 0.4, 0.9]);
            }

            if let Some((_, color, label)) = &item_snap[idx] {
                // Colored icon square
                self.hud(sx + 4.0, cy + 2.0, icon_sz, icon_sz, z + 200, *color);
                // Item label — full width
                self.draw_text(label, sx + icon_sz + 8.0, cy + 4.0, 1.2,
                               [color[0], color[1], color[2], 0.95], z + 300);
            } else {
                self.draw_text(b"-", sx + 4.0, cy + 4.0, 1.0, [0.3, 0.28, 0.24, 0.4], z + 300);
            }
        }

        // Detail panel for selected item
        let detail_y = sy + max_slots as f32 * row_h + 4.0;
        if let Some(sel_idx) = selected {
            if let Some(Some((item, color, _label))) = item_snap.get(sel_idx) {
                self.hud(sx, detail_y, iw, 36.0, 2400, [0.10, 0.08, 0.06, 0.9]);
                let detail_label = match item {
                    InvItem::Equipment(eq) => {
                        let rn = eq.rarity.name();
                        let sn = eq.slot.name();
                        let mut buf = [0u8; 24];
                        let e1 = rn.len().min(14);
                        buf[..e1].copy_from_slice(&rn[..e1]);
                        buf[e1] = b' ';
                        let e2 = (e1 + 1 + sn.len()).min(24);
                        buf[e1 + 1..e2].copy_from_slice(&sn[..e2 - e1 - 1]);
                        (buf, e2)
                    }
                    InvItem::Potion(PotionKind::Health) => {
                        let mut buf = [0u8; 24];
                        buf[..12].copy_from_slice(b"POTION SANTE");
                        (buf, 12)
                    }
                    InvItem::Potion(PotionKind::Mana) => {
                        let mut buf = [0u8; 24];
                        buf[..11].copy_from_slice(b"POTION MANA");
                        (buf, 11)
                    }
                    InvItem::Scroll(s) => {
                        let sl = s.content.short_label();
                        let mut buf = [0u8; 24];
                        buf[..8].copy_from_slice(b"SCROLL: ");
                        let end = (8 + sl.len()).min(24);
                        buf[8..end].copy_from_slice(&sl[..end - 8]);
                        (buf, end)
                    }
                    InvItem::Gold(g) => {
                        let (gv, gl) = fmt_u32(*g);
                        let mut buf = [0u8; 24];
                        buf[..5].copy_from_slice(b"GOLD ");
                        buf[5..5 + gl].copy_from_slice(&gv[..gl]);
                        (buf, 5 + gl)
                    }
                };
                self.draw_text(&detail_label.0[..detail_label.1], sx + 4.0, detail_y + 2.0, 1.5,
                               *color, 2401);
                self.draw_text(b"E=EQUIP G=SELL X=DROP", sx + 4.0, detail_y + 20.0, 1.0, dim, 2402);
            }
        }

        // Upgrade button
        if self.backpack.can_upgrade() {
            let uy = detail_y + 42.0;
            let (cv, cl) = fmt_u32(BACKPACK_UPGRADE_COST);
            let mut btn = [0u8; 24];
            btn[..12].copy_from_slice(b"+4 SLOTS -- ");
            btn[12..12 + cl].copy_from_slice(&cv[..cl]);
            btn[12 + cl] = b'G';
            let btn_c = if self.player_gold >= BACKPACK_UPGRADE_COST { [0.6, 0.8, 0.4, 0.9] } else { dim };
            self.draw_text(&btn[..13 + cl], sx, uy, 1.0, btn_c, 2410);
        }
    }

    // --- Stats panel (available via C key or TAB when inventory is open) ---
    #[allow(dead_code, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::many_single_char_names)]
    fn draw_inv_stats(&mut self, sx: f32, start_y: f32, iw: f32, white: [f32; 4], green: [f32; 4]) {
        let mut sy = start_y;
        let col2 = sx + iw * 0.5;

        self.draw_text(b"COMBAT", sx, sy, 1.5, white, 2150);
        sy += 16.0;

        // Melee DMG | ATK CD
        let melee_dmg = self.melee_damage();
        let (dv, dl) = fmt_f32(melee_dmg);
        let mut line = [0u8; 16];
        line[..4].copy_from_slice(b"DMG ");
        line[4..4 + dl].copy_from_slice(&dv[..dl]);
        self.draw_text(&line[..4 + dl], sx, sy, 1.0, green, 2151);
        let cd = self.attack_cooldown();
        let (cv, cl) = fmt_u32(cd);
        let mut line2 = [0u8; 12];
        line2[..3].copy_from_slice(b"CD ");
        line2[3..3 + cl].copy_from_slice(&cv[..cl]);
        self.draw_text(&line2[..3 + cl], col2, sy, 1.0, white, 2154);
        sy += 12.0;

        // DEF | DODGE
        let def = self.damage_reduction();
        let (rv, rl) = fmt_f32(def);
        let mut line = [0u8; 12];
        line[..4].copy_from_slice(b"DEF ");
        line[4..4 + rl].copy_from_slice(&rv[..rl]);
        self.draw_text(&line[..4 + rl], sx, sy, 1.0, white, 2156);
        let dodge_pct = (self.dodge_chance() * 100.0) as u32;
        let (dv3, dl3) = fmt_u32(dodge_pct);
        let mut line2 = [0u8; 12];
        line2[..4].copy_from_slice(b"DDG ");
        line2[4..4 + dl3].copy_from_slice(&dv3[..dl3]);
        line2[4 + dl3] = b'%';
        self.draw_text(&line2[..5 + dl3], col2, sy, 1.0, white, 2155);
        sy += 12.0;

        // Life steal | Mana steal
        let ls = self.equip_life_steal();
        let ms = self.equip_mana_steal();
        if ls > 0.0 {
            let lsp = (ls * 100.0) as u32;
            let (lv, ll) = fmt_u32(lsp);
            let mut line = [0u8; 12];
            line[..3].copy_from_slice(b"LS ");
            line[3..3 + ll].copy_from_slice(&lv[..ll]);
            line[3 + ll] = b'%';
            self.draw_text(&line[..4 + ll], sx, sy, 1.0, [0.8, 0.3, 0.3, 0.9], 2158);
        }
        if ms > 0.0 {
            let msp = (ms * 100.0) as u32;
            let (mv, ml) = fmt_u32(msp);
            let mut line = [0u8; 12];
            line[..3].copy_from_slice(b"MS ");
            line[3..3 + ml].copy_from_slice(&mv[..ml]);
            line[3 + ml] = b'%';
            self.draw_text(&line[..4 + ml], col2, sy, 1.0, [0.3, 0.3, 0.8, 0.9], 2159);
        }
        if ls > 0.0 || ms > 0.0 { sy += 12.0; }

        // Spell damages
        sy += 4.0;
        self.hud(sx, sy, iw, 1.0, 2160, [0.5, 0.45, 0.35, 0.4]);
        sy += 6.0;
        self.draw_text(b"SORTS", sx, sy, 1.5, white, 2161);
        sy += 16.0;

        let fb_dmg = self.fireball_damage();
        let frost_dmg = self.frost_bolt_damage();
        let cl_dmg = self.chain_lightning_damage();
        let (fv, fl) = fmt_f32(fb_dmg);
        let mut line = [0u8; 12];
        line[..4].copy_from_slice(b"FEU ");
        line[4..4 + fl].copy_from_slice(&fv[..fl]);
        self.draw_text(&line[..4 + fl], sx, sy, 1.0, [1.0, 0.5, 0.2, 0.9], 2152);
        let (fv2, fl2) = fmt_f32(frost_dmg);
        let mut line2 = [0u8; 12];
        line2[..4].copy_from_slice(b"ICE ");
        line2[4..4 + fl2].copy_from_slice(&fv2[..fl2]);
        self.draw_text(&line2[..4 + fl2], col2, sy, 1.0, [0.3, 0.6, 1.0, 0.9], 2153);
        sy += 12.0;
        let (clv, cll) = fmt_f32(cl_dmg);
        let mut line = [0u8; 12];
        line[..4].copy_from_slice(b"ECL ");
        line[4..4 + cll].copy_from_slice(&clv[..cll]);
        self.draw_text(&line[..4 + cll], sx, sy, 1.0, C_LIGHTNING, 2157);
        sy += 18.0;

        // Resistances
        self.hud(sx, sy, iw, 1.0, 2162, [0.5, 0.45, 0.35, 0.4]);
        sy += 6.0;
        self.draw_text(b"RESISTANCES", sx, sy, 1.5, white, 2163);
        sy += 16.0;
        let eres = self.effective_resistances();
        let res_names: [(&[u8], [f32; 4], f32); 4] = [
            (b"FEU  ", [1.0, 0.5, 0.2, 0.9], eres.fire),
            (b"FROID", [0.3, 0.6, 1.0, 0.9], eres.cold),
            (b"ECLR ", [0.6, 0.7, 1.0, 0.9], eres.lightning),
            (b"POIS ", [0.3, 0.8, 0.2, 0.9], eres.poison),
        ];
        for (name, color, val) in res_names {
            let pct = (val * 100.0) as i32;
            let abs = pct.unsigned_abs();
            let (rv2, rl2) = fmt_u32(abs);
            let mut line = [0u8; 16];
            line[..5].copy_from_slice(name);
            line[5] = if pct < 0 { b'-' } else { b'+' };
            line[6..6 + rl2].copy_from_slice(&rv2[..rl2]);
            line[6 + rl2] = b'%';
            self.draw_text(&line[..7 + rl2], sx, sy, 1.0, color, 2164);
            sy += 12.0;
        }

        // Skill grid summary
        sy += 4.0;
        self.hud(sx, sy, iw, 1.0, 2170, [0.5, 0.45, 0.35, 0.4]);
        sy += 6.0;
        let unlocked = self.skill_grid.count_unlocked() as u32;
        let filled = self.skill_grid.count_filled() as u32;
        let (uc_buf, uc_len) = fmt_u32(unlocked);
        let (fc_buf, fc_len) = fmt_u32(filled);
        let mut sl_line = [0u8; 30];
        sl_line[..7].copy_from_slice(b"GRILLE ");
        sl_line[7..7 + fc_len].copy_from_slice(&fc_buf[..fc_len]);
        sl_line[7 + fc_len] = b'/';
        sl_line[8 + fc_len..8 + fc_len + uc_len].copy_from_slice(&uc_buf[..uc_len]);
        let sl_end = 8 + fc_len + uc_len;
        self.draw_text(&sl_line[..sl_end], sx, sy, 1.0, [0.7, 0.65, 0.55, 0.8], 2175);
        if self.grid_unlock_points > 0 {
            let (dp_buf, dp_len) = fmt_u32(self.grid_unlock_points);
            let mut dp_line = [0u8; 30];
            dp_line[..8].copy_from_slice(b"UNLOCK: ");
            dp_line[8..8 + dp_len].copy_from_slice(&dp_buf[..dp_len]);
            self.draw_text(&dp_line[..8 + dp_len], sx + iw * 0.5, sy, 1.0, C_XP, 2176);
        }
    }

    // ======================================================================
    // Waypoint menu
    // ======================================================================

    // ======================================================================
    // Skill Grid — 13×13 scrollable grid of scroll slots
    // ======================================================================

    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::many_single_char_names)]
    fn draw_skill_disk(&mut self) {
        let vw = self.viewport[0];
        let vh = self.viewport[1];

        // Dark overlay
        self.hud(0.0, 0.0, vw, vh, 3000, [0.05, 0.05, 0.08, 0.92]);

        // Title
        self.draw_text(b"SKILL GRID", vw * 0.5 - 40.0, 8.0, 2.0, [1.0, 0.95, 0.70, 1.0], 3001);

        // Unlock points available
        if self.grid_unlock_points > 0 {
            let (dp_buf, dp_len) = fmt_u32(self.grid_unlock_points);
            let mut dp_line = [0u8; 30];
            dp_line[..8].copy_from_slice(b"UNLOCK: ");
            dp_line[8..8 + dp_len].copy_from_slice(&dp_buf[..dp_len]);
            self.draw_text(&dp_line[..8 + dp_len], vw * 0.5 - 30.0, 28.0, 1.0, [1.0, 0.85, 0.30, 1.0], 3002);
        }

        // Instructions
        self.draw_text(b"DRAG=PAN  CLICK=UNLOCK/INSERT  K/ESC=CLOSE", 10.0, vh - 16.0, 1.0, [0.5, 0.5, 0.5, 0.7], 3003);

        // Legend
        self.hud(10.0, vh - 52.0, 10.0, 10.0, 3004, [0.55, 0.85, 0.55, 1.0]);
        self.draw_text(b"Passif", 24.0, vh - 51.0, 1.0, [0.7, 0.7, 0.7, 0.8], 3005);
        self.hud(80.0, vh - 52.0, 10.0, 10.0, 3006, [0.90, 0.55, 0.55, 1.0]);
        self.draw_text(b"Actif", 94.0, vh - 51.0, 1.0, [0.7, 0.7, 0.7, 0.8], 3007);
        self.hud(150.0, vh - 52.0, 10.0, 10.0, 3008, [0.85, 0.85, 0.85, 1.0]);
        self.draw_text(b"Special", 164.0, vh - 51.0, 1.0, [0.7, 0.7, 0.7, 0.8], 3009);

        // Canvas center on screen
        let cx = vw * 0.5 + self.disk_scroll[0];
        let cy = vh * 0.5 + self.disk_scroll[1];

        let mx = self.mouse_screen[0];
        let my = self.mouse_screen[1];
        let frame_val = self.frame;
        let slot_sz = skill_disk::SLOT_SIZE;
        let half_sz = slot_sz * 0.5;

        // Snapshot grid state for borrow-safe rendering
        let mut unlocked_snap = [false; GRID_TOTAL];
        let mut filled_snap = [false; GRID_TOTAL];
        let mut slot_labels: [[u8; 6]; GRID_TOTAL] = [[0u8; 6]; GRID_TOTAL];
        let mut slot_label_lens = [0usize; GRID_TOTAL];

        for i in 0..GRID_TOTAL {
            unlocked_snap[i] = self.skill_grid.unlocked[i];
            if let Some(ref content) = self.skill_grid.slots[i] {
                filled_snap[i] = true;
                let label = content.short_label();
                let len = label.len().min(6);
                slot_labels[i][..len].copy_from_slice(&label[..len]);
                slot_label_lens[i] = len;
            }
        }

        // Draw each grid cell
        for row in 0..GRID_ROWS {
            for col in 0..GRID_COLS {
                let idx = skill_disk::grid_index(col, row);
                let pos = skill_disk::slot_canvas_pos(col, row);
                let sx = cx + pos[0] - half_sz;
                let sy = cy + pos[1] - half_sz;

                // Cull off-screen
                if sx + slot_sz < 0.0 || sx > vw || sy + slot_sz < 0.0 || sy > vh { continue; }

                let is_center = col == skill_disk::GRID_CENTER_X && row == skill_disk::GRID_CENTER_Y;
                let z = 3100 + idx as u32;

                if is_center {
                    // Draw center diamond
                    let ds = slot_sz * 0.6;
                    let dh = ds * 0.5;
                    let dcx = cx + pos[0];
                    let dcy = cy + pos[1];
                    self.hud(dcx - dh, dcy - dh, ds, ds, z, [1.0, 1.0, 1.0, 0.9]);
                    self.hud(dcx - dh + 2.0, dcy - dh + 2.0, ds - 4.0, ds - 4.0, z + 1, [0.2, 0.2, 0.25, 0.9]);
                    // Arrows (4 cardinal)
                    self.hud(dcx - 1.0, dcy - dh - 6.0, 2.0, 6.0, z + 2, [0.7, 0.7, 0.7, 0.6]); // up
                    self.hud(dcx - 1.0, dcy + dh, 2.0, 6.0, z + 3, [0.7, 0.7, 0.7, 0.6]); // down
                    self.hud(dcx - dh - 6.0, dcy - 1.0, 6.0, 2.0, z + 4, [0.7, 0.7, 0.7, 0.6]); // left
                    self.hud(dcx + dh, dcy - 1.0, 6.0, 2.0, z + 5, [0.7, 0.7, 0.7, 0.6]); // right
                    continue;
                }

                let kind = match skill_disk::slot_kind_at(col, row) {
                    Some(k) => k,
                    None => continue,
                };
                let base_color = skill_disk::slot_color(kind);
                let unlocked = unlocked_snap[idx];
                let filled = filled_snap[idx];
                let can_unlock = !unlocked && self.grid_unlock_points > 0
                    && skill_disk::can_allocate(idx, &unlocked_snap);

                // Slot color
                let color = if filled {
                    // Bright filled
                    [base_color[0], base_color[1], base_color[2], 0.95]
                } else if unlocked {
                    // Unlocked empty — dimmed
                    [base_color[0] * 0.4, base_color[1] * 0.4, base_color[2] * 0.4, 0.6]
                } else if can_unlock {
                    // Pulsing unlockable
                    let pulse = ((frame_val as f32 * 0.05).sin() * 0.2 + 0.5) as f32;
                    [base_color[0] * pulse, base_color[1] * pulse, base_color[2] * pulse, 0.7]
                } else {
                    // Locked — very dim
                    [base_color[0] * 0.12, base_color[1] * 0.12, base_color[2] * 0.12, 0.3]
                };

                // Slot background (rounded-looking square)
                self.hud(sx + 1.0, sy + 1.0, slot_sz - 2.0, slot_sz - 2.0, z, color);

                // Thin border
                let border_c = if filled {
                    [1.0, 0.95, 0.70, 0.8]
                } else if unlocked {
                    [base_color[0] * 0.6, base_color[1] * 0.6, base_color[2] * 0.6, 0.5]
                } else {
                    [0.2, 0.2, 0.2, 0.3]
                };
                self.hud(sx, sy, slot_sz, 1.0, z + 200, border_c);
                self.hud(sx, sy + slot_sz - 1.0, slot_sz, 1.0, z + 201, border_c);
                self.hud(sx, sy, 1.0, slot_sz, z + 202, border_c);
                self.hud(sx + slot_sz - 1.0, sy, 1.0, slot_sz, z + 203, border_c);

                // Scroll label inside filled slots
                if filled && slot_label_lens[idx] > 0 {
                    let label = &slot_labels[idx][..slot_label_lens[idx]];
                    let lx = sx + 3.0;
                    let ly = sy + slot_sz * 0.5 - 4.0;
                    self.draw_text(label, lx, ly, 0.8, [1.0, 1.0, 1.0, 0.9], z + 300);
                }

                // Hover tooltip
                let slot_cx = cx + pos[0];
                let slot_cy = cy + pos[1];
                let hover = (mx - slot_cx).abs() < half_sz + 2.0 && (my - slot_cy).abs() < half_sz + 2.0;
                if hover {
                    let kind_label = skill_disk::slot_kind_label(kind);
                    let tw = 120.0;
                    let th = if filled { 32.0 } else { 22.0 };
                    let tx = (slot_cx + half_sz + 6.0).min(vw - tw - 4.0);
                    let ty = (slot_cy - 4.0).min(vh - th - 4.0).max(4.0);
                    self.hud(tx - 2.0, ty - 2.0, tw + 4.0, th + 4.0, 3900, [0.1, 0.1, 0.12, 0.95]);
                    self.draw_text(kind_label, tx, ty, 1.0, [1.0, 0.95, 0.80, 1.0], 3901);

                    if filled && slot_label_lens[idx] > 0 {
                        let label = &slot_labels[idx][..slot_label_lens[idx]];
                        self.draw_text(label, tx, ty + 12.0, 1.0, [0.7, 0.8, 0.7, 0.9], 3902);
                    } else if !unlocked && can_unlock {
                        self.draw_text(b"CLICK=UNLOCK", tx, ty + 12.0, 1.0, [1.0, 0.85, 0.30, 0.9], 3902);
                    } else if unlocked && !filled {
                        self.draw_text(b"EMPTY SLOT", tx, ty + 12.0, 1.0, [0.5, 0.5, 0.5, 0.7], 3902);
                    }
                }
            }
        }

        // Scroll inventory panel (right side)
        let inv_x = vw - 160.0;
        let inv_y = 50.0;
        let inv_w = 150.0;
        let inv_count = self.scroll_inventory.len();
        let inv_h = 20.0 + inv_count as f32 * 14.0;
        self.hud(inv_x - 2.0, inv_y - 2.0, inv_w + 4.0, inv_h.max(40.0) + 4.0, 3800, [0.08, 0.08, 0.10, 0.9]);
        self.draw_text(b"PARCHEMINS", inv_x + 4.0, inv_y + 2.0, 1.0, [1.0, 0.95, 0.70, 0.9], 3801);
        if inv_count == 0 {
            self.draw_text(b"(aucun)", inv_x + 4.0, inv_y + 16.0, 1.0, [0.5, 0.5, 0.5, 0.6], 3802);
        } else {
            // Snapshot labels for borrow safety
            let max_show = inv_count.min(20);
            let mut inv_labels: [[u8; 8]; 20] = [[0u8; 8]; 20];
            let mut inv_label_lens = [0usize; 20];
            for i in 0..max_show {
                let label = self.scroll_inventory[i].content.short_label();
                let len = label.len().min(8);
                inv_labels[i][..len].copy_from_slice(&label[..len]);
                inv_label_lens[i] = len;
            }
            for i in 0..max_show {
                let iy = inv_y + 16.0 + i as f32 * 14.0;
                self.draw_text(&inv_labels[i][..inv_label_lens[i]], inv_x + 4.0, iy, 1.0,
                    [0.7, 0.8, 0.7, 0.9], 3810 + i as u32);
            }
        }
    }

    /// Click handler for grid: unlock slot or insert first compatible scroll.
    #[allow(clippy::cast_precision_loss)]
    fn disk_click(&mut self, click_sx: f32, click_sy: f32) {
        let cx = self.viewport[0] * 0.5 + self.disk_scroll[0];
        let cy = self.viewport[1] * 0.5 + self.disk_scroll[1];
        let half = skill_disk::SLOT_SIZE * 0.5 + 2.0;

        for row in 0..GRID_ROWS {
            for col in 0..GRID_COLS {
                // Skip center
                if col == skill_disk::GRID_CENTER_X && row == skill_disk::GRID_CENTER_Y { continue; }
                let pos = skill_disk::slot_canvas_pos(col, row);
                let sx = cx + pos[0];
                let sy = cy + pos[1];
                if (click_sx - sx).abs() > half || (click_sy - sy).abs() > half { continue; }

                let idx = skill_disk::grid_index(col, row);
                let unlocked = self.skill_grid.unlocked[idx];

                if !unlocked {
                    // Try to unlock
                    self.unlock_grid_slot(col, row);
                    return;
                }

                // Unlocked — try to insert first compatible scroll from inventory
                if self.skill_grid.slots[idx].is_some() {
                    // Already filled — remove scroll back to inventory
                    self.remove_scroll(col, row);
                    return;
                }

                let kind = match skill_disk::slot_kind_at(col, row) {
                    Some(k) => k,
                    None => return,
                };
                // Find first compatible scroll in inventory
                let compat_idx = self.scroll_inventory.iter().position(|s| {
                    s.content.required_slot_kind() == kind
                });
                if let Some(inv_idx) = compat_idx {
                    self.insert_scroll(col, row, inv_idx);
                }
                return;
            }
        }
    }

    // ==================================================================
    // Civil Skills panel (L key) — tableau des compétences civiles
    // ==================================================================
    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    fn draw_civil_skills(&mut self) {
        let vw = self.viewport[0];
        let vh = self.viewport[1];
        let panel_w = 320.0_f32;
        let row_h = 18.0_f32;
        let gap = 3.0_f32;
        let header_h = 24.0_f32;
        let footer_h = 16.0_f32;
        let panel_h = header_h + (row_h + gap) * CIVIL_SKILL_COUNT as f32 + footer_h + 10.0;
        let px = (vw - panel_w) * 0.5;
        let py = (vh - panel_h) * 0.5;

        let white: [f32; 4] = [1.0, 1.0, 1.0, 0.95];
        let dim: [f32; 4] = [0.5, 0.5, 0.5, 0.7];
        let gold: [f32; 4] = [0.9, 0.8, 0.3, 1.0];
        let bg: [f32; 4] = [0.06, 0.05, 0.04, 0.94];
        let frame: [f32; 4] = [0.4, 0.32, 0.18, 1.0];
        let bar_bg: [f32; 4] = [0.15, 0.12, 0.08, 0.8];

        // Z-order base for civil panel (above disk overlay)
        let z0: u32 = 2200;

        // Background + frame
        self.hud(px - 2.0, py - 2.0, panel_w + 4.0, panel_h + 4.0, z0, frame);
        self.hud(px, py, panel_w, panel_h, z0 + 1, bg);
        // Header
        self.hud(px, py, panel_w, header_h, z0 + 2, frame);
        self.draw_text(b"COMPETENCES CIVILES", px + 8.0, py + 5.0, 1.5, gold, z0 + 3);
        self.draw_text(b"L = FERMER", px + panel_w - 80.0, py + 7.0, 1.0, dim, z0 + 3);

        let mut y = py + header_h + 4.0;
        let x0 = px + 6.0;
        let bar_w = panel_w - 12.0;

        // Snapshot skill data to avoid borrow conflict with self.hud()
        let mut levels = [0u32; CIVIL_SKILL_COUNT];
        let mut xps = [0.0f32; CIVIL_SKILL_COUNT];
        let mut progresses = [0.0f32; CIVIL_SKILL_COUNT];
        let mut xp_needs = [0.0f32; CIVIL_SKILL_COUNT];
        for i in 0..CIVIL_SKILL_COUNT {
            levels[i] = self.civil.skills[i].level;
            xps[i] = self.civil.skills[i].xp;
            progresses[i] = self.civil.skills[i].progress();
            xp_needs[i] = self.civil.skills[i].xp_to_next();
        }

        for i in 0..CIVIL_SKILL_COUNT {
            let id = CivilSkillId::ALL[i];
            let c = id.color();
            let skill_color: [f32; 4] = [c[0], c[1], c[2], 0.95];
            let bar_fill: [f32; 4] = [c[0] * 0.6, c[1] * 0.6, c[2] * 0.6, 0.7];

            // Skill name + level
            let name = id.name().as_bytes();
            let name_len = name.len().min(16);
            let (lv_buf, lv_len) = fmt_u32(levels[i]);
            let mut line = [0u8; 32];
            line[..name_len].copy_from_slice(&name[..name_len]);
            line[name_len..name_len + 4].copy_from_slice(b" Lv ");
            line[name_len + 4..name_len + 4 + lv_len].copy_from_slice(&lv_buf[..lv_len]);
            let text_len = name_len + 4 + lv_len;

            // XP bar background
            self.hud(x0, y, bar_w, row_h, z0 + 4 + i as u32 * 3, bar_bg);
            // XP bar fill
            if progresses[i] > 0.0 {
                self.hud(x0, y, bar_w * progresses[i], row_h, z0 + 5 + i as u32 * 3, bar_fill);
            }
            // Skill name + level text
            self.draw_text(&line[..text_len], x0 + 4.0, y + 3.0, 1.2, skill_color, z0 + 6 + i as u32 * 3);

            // XP numbers on right side
            let (xp_buf, xp_len) = fmt_f32(xps[i]);
            let (need_buf, need_len) = fmt_f32(xp_needs[i]);
            let mut xp_line = [0u8; 24];
            xp_line[..xp_len].copy_from_slice(&xp_buf[..xp_len]);
            xp_line[xp_len] = b'/';
            xp_line[xp_len + 1..xp_len + 1 + need_len].copy_from_slice(&need_buf[..need_len]);
            let xp_total_len = xp_len + 1 + need_len;
            let xp_text_x = x0 + bar_w - (xp_total_len as f32 * 6.0) - 4.0;
            self.draw_text(&xp_line[..xp_total_len], xp_text_x, y + 3.0, 1.0, white, z0 + 6 + i as u32 * 3);

            y += row_h + gap;
        }

        // Footer — description de la compétence survolée
        // Check mouse hover to determine which skill row is under cursor
        let my = self.mouse_screen[1];
        let mx = self.mouse_screen[0];
        let rows_start = py + header_h + 4.0;
        if mx >= px && mx <= px + panel_w && my >= rows_start {
            let row_idx = ((my - rows_start) / (row_h + gap)) as usize;
            if row_idx < CIVIL_SKILL_COUNT {
                let id = CivilSkillId::ALL[row_idx];
                let desc = id.description().as_bytes();
                let desc_len = desc.len().min(50);
                self.draw_text(&desc[..desc_len], x0, y + 2.0, 1.0, dim, z0 + 40);
            }
        }
    }

    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::many_single_char_names)]
    fn draw_waypoint_menu(&mut self) {
        let vh = self.viewport[1];
        let w = 200.0;
        let h = 140.0;
        let x = 10.0;
        let y = vh - h - 80.0; // above the HP orb, left side
        let white = [1.0, 1.0, 1.0, 0.95];

        self.hud(x - 2.0, y - 2.0, w + 4.0, h + 4.0, 2200, C_STATS_FR);
        self.hud(x, y, w, h, 2201, C_STATS_BG);
        self.hud(x, y, w, 22.0, 2202, C_STATS_FR);
        self.draw_text(b"WAYPOINTS", x + 8.0, y + 5.0, 1.5, white, 2203);

        let mut row = 0_u32;
        for i in 0..ZONE_COUNT {
            let discovered = self.waypoints[i];
            if !discovered { continue; } // only show discovered waypoints
            let zone = ZoneId::from_index(i);
            let name = zone.label().as_bytes();
            let zy = y + 30.0 + row as f32 * 24.0;
            let is_current = self.current_zone as usize == i;
            let c = if is_current {
                [0.5, 1.0, 0.5, 1.0]
            } else {
                C_WP_CORE
            };
            self.draw_text(name, x + 10.0, zy + 4.0, 1.5, c, 2210 + row);
            row += 1;
        }
    }

    // ======================================================================
    // NPC dialog bubble
    // ======================================================================

    #[allow(clippy::cast_precision_loss)]
    fn draw_npc_dialog(&mut self, idx: usize) {
        if idx >= NPCS.len() { return; }
        let [sx, sy] = self.w2s(NPCS[idx].pos[0], NPCS[idx].pos[1]);
        let msg: &[u8] = match idx {
            0 => if self.player_gold >= SCROLL_COST && self.scroll_inventory.len() < 24 { b"SCROLL 20G?" } else { b"HEAL + CURE" },
            1 => if self.player_gold >= GAMBLE_COST { b"GAMBLE 30G?" } else { b"POTIONS 10G" },
            2 => if self.player_gold >= UPGRADE_COST { b"UPGRADE 60G?" } else { b"NEED REPAIRS?" },
            3 => if self.mercenary.is_none() { b"HIRE MERC 80G?" } else { b"SLAY THE EVIL" },
            _ => b"...",
        };
        let tw = msg.len() as f32 * 9.0;
        let bx = sx - tw * 0.5 - 6.0;
        let by = sy - 62.0;
        self.batch.push(SpriteInstance {
            screen_pos: [bx, by],
            src: UNIT, atlas: ATLAS_0, material: MAT,
            sort_key: SortKey { layer: RenderLayer::UiWorld, y_sort: 0, sub_order: 300 },
            tint: [0.0, 0.0, 0.0, 0.70], scale: [tw + 12.0, 18.0],
        });
        self.draw_text_at(msg, bx + 6.0, by + 3.0, 1.5, [1.0, 1.0, 0.85, 0.95],
                          RenderLayer::UiWorld, 0, 301);
    }

    // ======================================================================
    // Waypoint objects in world
    // ======================================================================

    #[allow(clippy::cast_precision_loss)]
    fn draw_waypoints(&mut self) {
        // Draw waypoint glow at zone center if discovered
        if !self.is_camp() && self.waypoints[self.current_zone as usize] {
            let wx = 12.0;
            let wy = 12.0;
            let pulse = ((self.frame as f32 * 0.04).sin() * 0.15 + 0.85).abs();
            let c = [C_WP_GLOW[0] * pulse, C_WP_GLOW[1] * pulse, C_WP_GLOW[2], C_WP_GLOW[3]];
            self.spr(wx, wy, [-16.0, -8.0], [32.0, 16.0], RenderLayer::Decals, 1800, c);
            self.spr(wx, wy, [-8.0, -4.0], [16.0, 8.0], RenderLayer::Decals, 1801, C_WP_CORE);
        }
    }

    // ======================================================================
    // Shrines
    // ======================================================================

    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    fn draw_shrines(&mut self) {
        let shrines: Vec<_> = self.shrines.iter().enumerate()
            .filter(|(_, s)| !s.used)
            .map(|(i, s)| (i, s.pos, s.kind))
            .collect();
        for (i, pos, kind) in shrines {
            let pulse = ((self.frame as f32 * 0.05 + i as f32 * 2.0).sin() * 0.12 + 0.88).abs();
            let c = [C_SHRINE_GLOW[0] * pulse, C_SHRINE_GLOW[1] * pulse, C_SHRINE_GLOW[2], C_SHRINE_GLOW[3]];
            self.spr(pos[0], pos[1], [-12.0, -6.0], [24.0, 12.0], RenderLayer::Decals, 3000 + i as u32, c);
            // Shrine pillar
            self.spr(pos[0], pos[1], [-4.0, -16.0], [8.0, 18.0], RenderLayer::Props, 3010 + i as u32, C_SHRINE);
            self.spr(pos[0], pos[1], [-6.0, -18.0], [12.0, 4.0], RenderLayer::Props, 3020 + i as u32, [0.4, 0.7, 1.0, 0.9]);
            // Label
            let label: &[u8] = match kind {
                ShrineKind::Experience => b"XP",
                ShrineKind::Health => b"HP",
                ShrineKind::Damage => b"DMG",
            };
            let [sx, sy] = self.w2s(pos[0], pos[1]);
            self.draw_text_at(label, sx - label.len() as f32 * 4.5, sy - 28.0, 1.0,
                              [0.8, 0.9, 1.0, 0.9], RenderLayer::UiWorld, 0, 3030 + i as u32);
        }
    }

    // ======================================================================
    // Chests
    // ======================================================================

    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss)]
    fn draw_chests(&mut self) {
        let chests: Vec<_> = self.chests.iter().enumerate()
            .map(|(i, c)| (i, c.pos, c.opened))
            .collect();
        for (i, pos, opened) in chests {
            if opened {
                self.spr(pos[0], pos[1], [-6.0, -4.0], [12.0, 8.0], RenderLayer::Props, 3100 + i as u32, [0.3, 0.2, 0.1, 0.5]);
            } else {
                // Closed chest
                self.spr(pos[0], pos[1], [-6.0, -6.0], [12.0, 8.0], RenderLayer::Props, 3100 + i as u32, C_CHEST);
                self.spr(pos[0], pos[1], [-6.0, -10.0], [12.0, 5.0], RenderLayer::Props, 3110 + i as u32, C_CHEST_LID);
                // Gold gleam
                let pulse = ((self.frame as f32 * 0.06 + i as f32).sin() * 0.3 + 0.7).abs();
                self.spr(pos[0], pos[1], [-2.0, -8.0], [4.0, 3.0], RenderLayer::Props, 3120 + i as u32,
                         [1.0, 0.9, 0.3, pulse]);
            }
        }
    }

    // ======================================================================
    // Mercenary
    // ======================================================================

    fn draw_mercenary(&mut self) {
        let Some(merc) = &self.mercenary else { return };
        if !merc.alive { return; }
        let pos = merc.pos;
        let hp_r = merc.hp / merc.max_hp;
        // Shadow
        self.spr(pos[0], pos[1], [-6.0, 4.0], [12.0, 6.0], RenderLayer::Decals, 3200, C_SHADOW);
        // Body
        self.spr(pos[0], pos[1], [-6.0, -14.0], [12.0, 18.0], RenderLayer::Entities, 3201, C_MERC_BODY);
        // Head
        self.spr(pos[0], pos[1], [-4.0, -20.0], [8.0, 8.0], RenderLayer::Entities, 3202, C_MERC_HEAD);
        // Weapon (spear)
        self.spr(pos[0], pos[1], [5.0, -22.0], [2.0, 20.0], RenderLayer::Entities, 3203, [0.5, 0.5, 0.5, 0.9]);
        // HP bar
        let [sx, sy] = self.w2s(pos[0], pos[1]);
        self.batch.push(SpriteInstance {
            screen_pos: [sx - 10.0, sy - 26.0],
            src: UNIT, atlas: ATLAS_0, material: MAT,
            sort_key: SortKey { layer: RenderLayer::UiWorld, y_sort: 0, sub_order: 3210 },
            tint: [0.1, 0.1, 0.1, 0.6], scale: [20.0, 3.0],
        });
        self.batch.push(SpriteInstance {
            screen_pos: [sx - 10.0, sy - 26.0],
            src: UNIT, atlas: ATLAS_0, material: MAT,
            sort_key: SortKey { layer: RenderLayer::UiWorld, y_sort: 0, sub_order: 3211 },
            tint: [0.3, 0.7, 0.3, 0.8], scale: [20.0 * hp_r, 3.0],
        });
    }

    // ======================================================================
    // Lightning bolts (chain lightning visual)
    // ======================================================================

    #[allow(clippy::cast_possible_truncation, clippy::cast_precision_loss, clippy::cast_sign_loss)]
    fn draw_lightning_bolts(&mut self) {
        let bolts: Vec<_> = self.lightning_bolts.iter().enumerate()
            .map(|(i, b)| (i, b.from, b.to, b.life))
            .collect();
        for (i, from, to, life) in bolts {
            let alpha = life as f32 / 12.0;
            let [sx1, sy1] = self.w2s(from[0], from[1]);
            let [sx2, sy2] = self.w2s(to[0], to[1]);
            // Draw as a thick line using sprites
            let dx = sx2 - sx1;
            let dy = sy2 - sy1;
            let len = (dx * dx + dy * dy).sqrt();
            let steps = (len / 6.0).max(2.0) as u32;
            for s in 0..steps {
                let t = s as f32 / steps as f32;
                let jitter_x = ((self.frame.wrapping_add(s) as f32 * 7.3).sin() * 3.0) * alpha;
                let jitter_y = ((self.frame.wrapping_add(s) as f32 * 11.1).cos() * 3.0) * alpha;
                let px = sx1 + dx * t + jitter_x;
                let py = sy1 + dy * t + jitter_y;
                self.batch.push(SpriteInstance {
                    screen_pos: [px - 2.0, py - 2.0],
                    src: UNIT, atlas: ATLAS_0, material: MAT,
                    sort_key: SortKey { layer: RenderLayer::VfxAlpha, y_sort: 0, sub_order: 3300 + i as u32 * 50 + s },
                    tint: [C_LIGHTNING[0], C_LIGHTNING[1], C_LIGHTNING[2], alpha * 0.8],
                    scale: [4.0, 4.0],
                });
                // Bright core
                self.batch.push(SpriteInstance {
                    screen_pos: [px - 1.0, py - 1.0],
                    src: UNIT, atlas: ATLAS_0, material: MAT,
                    sort_key: SortKey { layer: RenderLayer::VfxAlpha, y_sort: 0, sub_order: 3300 + i as u32 * 50 + s + 1000 },
                    tint: [C_LIGHTNING_CORE[0], C_LIGHTNING_CORE[1], C_LIGHTNING_CORE[2], alpha],
                    scale: [2.0, 2.0],
                });
            }
        }
    }

    // ======================================================================
    // Active buffs HUD indicator
    // ======================================================================

    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    fn draw_buffs_hud(&mut self) {
        let mut bx = 10.0;
        let by = self.viewport[1] - 120.0;
        let buff_data: Vec<(usize, ShrineKind, u32)> = self.buffs.iter().enumerate()
            .map(|(i, b)| (i, b.kind, b.remaining))
            .collect();
        for (i, kind, remaining) in buff_data {
            let (label, color): (&[u8], [f32; 4]) = match kind {
                ShrineKind::Experience => (b"XP+", [0.8, 0.8, 0.2, 0.9]),
                ShrineKind::Health => (b"HP+", [0.2, 0.9, 0.2, 0.9]),
                ShrineKind::Damage => (b"DMG", [1.0, 0.4, 0.2, 0.9]),
            };
            let secs = remaining / 60;
            let (tv, tl) = fmt_u32(secs);
            let tw = label.len() as f32 * 7.0 + tl as f32 * 7.0 + 12.0;
            self.hud(bx, by, tw, 12.0, 3400 + i as u32, [0.0, 0.0, 0.0, 0.5]);
            self.draw_text(label, bx + 2.0, by + 1.0, 1.0, color, 3410 + i as u32);
            let offset = label.len() as f32 * 7.0 + 4.0;
            self.draw_text(&tv[..tl], bx + offset, by + 1.0, 1.0, [1.0, 1.0, 1.0, 0.8], 3420 + i as u32);
            bx += tw + 4.0;
        }
        // Poison indicator
        if self.poison_timer > 0 {
            let secs = self.poison_timer / 60;
            let (tv, tl) = fmt_u32(secs);
            let tw = 4.0 * 7.0 + tl as f32 * 7.0 + 12.0;
            self.hud(bx, by, tw, 12.0, 3450, [0.1, 0.2, 0.0, 0.6]);
            self.draw_text(b"PSN ", bx + 2.0, by + 1.0, 1.0, C_POISON, 3451);
            self.draw_text(&tv[..tl], bx + 30.0, by + 1.0, 1.0, C_POISON, 3452);
        }
    }

    // ======================================================================
    // Minimap
    // ======================================================================

    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    fn draw_minimap(&mut self) {
        let vw = self.viewport[0];
        let mm_w = 150.0;
        let mm_h = mm_w * (MAP_H as f32 / MAP_W as f32);
        let mm_x = vw - mm_w - 10.0;
        let mm_y = 10.0;
        let sx = mm_w / MAP_W as f32;
        let sy = mm_h / MAP_H as f32;

        self.hud(mm_x - 2.0, mm_y - 2.0, mm_w + 4.0, mm_h + 4.0, 800, C_MINIMAP_FR);
        self.hud(mm_x, mm_y, mm_w, mm_h, 801, C_MINIMAP_BG);

        let wo = self.world_offset;
        for ty in 0_u32..MAP_H as u32 {
            for tx in 0_u32..MAP_W as u32 {
                let gx = (tx as f32 + wo[0] + GLOBAL_OFF) as usize;
                let gy = (ty as f32 + wo[1] + GLOBAL_OFF) as usize;
                let tt = if gx < WORLD_W && gy < WORLD_H {
                    self.global_terrain[gy * WORLD_W + gx]
                } else { 0 };
                if tt == 0 { continue; }
                let c = match tt {
                    1 => [0.14, 0.24, 0.10, 0.5],
                    2 => [0.28, 0.20, 0.12, 0.5],
                    3 => [0.35, 0.30, 0.24, 0.6],
                    4 => [0.16, 0.14, 0.12, 0.5],
                    _ => continue,
                };
                self.hud(mm_x + tx as f32 * sx, mm_y + ty as f32 * sy, sx, sy, 802, c);
            }
        }

        let px = mm_x + self.player_pos[0] * sx;
        let py = mm_y + self.player_pos[1] * sy;
        self.hud(px - 3.0, py - 3.0, 6.0, 6.0, 850, C_MINIMAP_PLAYER);

        if self.is_camp() {
            for (i, npc) in NPCS.iter().enumerate() {
                let nx = mm_x + npc.pos[0] * sx;
                let ny = mm_y + npc.pos[1] * sy;
                self.hud(nx - 2.0, ny - 2.0, 4.0, 4.0, 860 + i as u32, C_MINIMAP_NPC);
            }
        }

        let enemy_positions: Vec<_> = self.enemies.iter().enumerate()
            .filter_map(|(i, e)| if e.alive { Some((i, e.pos)) } else { None })
            .collect();
        for (i, pos) in enemy_positions {
            let ex = mm_x + pos[0] * sx;
            let ey = mm_y + pos[1] * sy;
            self.hud(ex - 2.0, ey - 2.0, 4.0, 4.0, 880 + i as u32, C_MINIMAP_ENEMY);
        }
        // Mercenary dot
        if let Some(merc) = &self.mercenary {
            if merc.alive {
                let mx = mm_x + merc.pos[0] * sx;
                let my = mm_y + merc.pos[1] * sy;
                self.hud(mx - 2.0, my - 2.0, 4.0, 4.0, 895, C_MERC_BODY);
            }
        }
        // Shrine dots
        let shrine_data: Vec<(usize, [f32; 2])> = self.shrines.iter().enumerate()
            .filter(|(_, s)| !s.used).map(|(i, s)| (i, s.pos)).collect();
        for (i, pos) in shrine_data {
            let shx = mm_x + pos[0] * sx;
            let shy = mm_y + pos[1] * sy;
            self.hud(shx - 2.0, shy - 2.0, 4.0, 4.0, 896 + i as u32, C_SHRINE);
        }
        // Chest dots
        let chest_data: Vec<(usize, [f32; 2])> = self.chests.iter().enumerate()
            .filter(|(_, c)| !c.opened).map(|(i, c)| (i, c.pos)).collect();
        for (i, pos) in chest_data {
            let chx = mm_x + pos[0] * sx;
            let chy = mm_y + pos[1] * sy;
            self.hud(chx - 2.0, chy - 2.0, 4.0, 4.0, 900 + i as u32, C_GOLD_DROP);
        }
    }
}

// =========================================================================
// winit event loop
// =========================================================================
impl ApplicationHandler for SodomightApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);
        let size = PhysicalSize::new(self.bootstrap.window.width, self.bootstrap.window.height);
        let attributes: WindowAttributes = Window::default_attributes()
            .with_title(self.bootstrap.window.title.clone())
            .with_inner_size(size);

        let window = match event_loop.create_window(attributes) {
            Ok(w) => Arc::new(w),
            Err(e) => {
                self.set_fatal(event_loop, anyhow::anyhow!("window create failed: {e}"));
                return;
            }
        };

        let renderer =
            match pollster::block_on(GraphicsState::new(window.clone(), &self.bootstrap.render)) {
                Ok(r) => r,
                Err(e) => { self.set_fatal(event_loop, e); return; }
            };

        self.window_id = Some(window.id());
        self.window = Some(window);
        self.renderer = Some(renderer);
        self.audio.push_cue(AudioCue::TownAmbience);
    }

    #[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if Some(window_id) != self.window_id { return; }

        match event {
            WindowEvent::CloseRequested => {
                self.save_progress();
                event_loop.exit();
            }
            WindowEvent::Resized(size) => {
                if let Some(r) = &mut self.renderer { r.resize(size); }
                self.camera.set_viewport(size.width, size.height);
                self.viewport = [size.width as f32, size.height as f32];
            }
            WindowEvent::CursorMoved { position, .. } => {
                let new_pos = [position.x as f32, position.y as f32];
                if self.show_disk && self.disk_dragging {
                    self.disk_scroll[0] += new_pos[0] - self.mouse_screen[0];
                    self.disk_scroll[1] += new_pos[1] - self.mouse_screen[1];
                }
                self.mouse_screen = new_pos;
            }
            WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Left, .. } => {
                if self.show_disk {
                    self.disk_click(self.mouse_screen[0], self.mouse_screen[1]);
                } else if !self.player_dead {
                    self.handle_click(self.mouse_screen[0], self.mouse_screen[1]);
                }
            }
            WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Right, .. } => {
                if self.show_disk {
                    self.disk_dragging = true;
                } else if !self.player_dead && !self.mouse_over_ui(self.mouse_screen[0], self.mouse_screen[1]) {
                    self.fire_projectile(self.mouse_screen[0], self.mouse_screen[1]);
                }
            }
            WindowEvent::MouseInput { state: ElementState::Released, button: MouseButton::Right, .. } => {
                self.disk_dragging = false;
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    let bit = match code {
                        KeyCode::KeyW | KeyCode::ArrowUp => 1_u8,
                        KeyCode::KeyS | KeyCode::ArrowDown => 2,
                        KeyCode::KeyA | KeyCode::ArrowLeft => 4,
                        KeyCode::KeyD | KeyCode::ArrowRight => 8,
                        _ => 0,
                    };
                    if bit != 0 {
                        match event.state {
                            ElementState::Pressed => {
                                self.keys |= bit;
                                self.move_target = None;
                            }
                            ElementState::Released => self.keys &= !bit,
                        }
                    }
                    if event.state == ElementState::Pressed && !self.player_dead {
                        match code {
                            KeyCode::Tab => {
                                if self.show_inventory {
                                    self.show_stats = !self.show_stats;
                                } else {
                                    self.show_minimap = !self.show_minimap;
                                }
                            }
                            KeyCode::KeyC => self.show_stats = !self.show_stats,
                            KeyCode::KeyQ => self.show_quests = !self.show_quests,
                            KeyCode::KeyI => {
                                self.show_inventory = !self.show_inventory;
                                self.show_waypoint_menu = false;
                                if !self.show_inventory { self.inv_hover_slot = None; }
                            }
                            KeyCode::KeyR => self.running = !self.running,
                            KeyCode::KeyG => {
                                if self.show_inventory {
                                    if let Some(idx) = self.inv_hover_slot {
                                        self.backpack_sell(idx);
                                        if self.backpack.count() == 0 {
                                            self.inv_hover_slot = None;
                                        } else if let Some(sel) = self.inv_hover_slot {
                                            if sel >= self.backpack.capacity || self.backpack.get(sel).is_none() {
                                                self.inv_hover_slot = None;
                                            }
                                        }
                                    } else {
                                        self.sell_equipment();
                                    }
                                }
                            }
                            KeyCode::KeyE => {
                                if self.show_inventory {
                                    if let Some(idx) = self.inv_hover_slot {
                                        self.backpack_use(idx);
                                    }
                                } else {
                                    self.try_harvest();
                                }
                            }
                            KeyCode::KeyX => {
                                if self.show_inventory {
                                    if let Some(idx) = self.inv_hover_slot {
                                        self.backpack_drop(idx);
                                        if self.backpack.count() == 0 {
                                            self.inv_hover_slot = None;
                                        }
                                    }
                                }
                            }
                            KeyCode::KeyU => {
                                if self.show_inventory && self.is_camp() {
                                    self.backpack_upgrade();
                                }
                            }
                            KeyCode::ArrowUp => {
                                if self.show_inventory {
                                    self.inv_hover_slot = Some(match self.inv_hover_slot {
                                        None => 0,
                                        Some(s) => s.saturating_sub(1),
                                    });
                                }
                            }
                            KeyCode::ArrowDown => {
                                if self.show_inventory {
                                    let max = self.backpack.capacity.saturating_sub(1);
                                    self.inv_hover_slot = Some(match self.inv_hover_slot {
                                        None => 0,
                                        Some(s) => (s + 1).min(max),
                                    });
                                }
                            }
                            KeyCode::KeyT => self.use_town_portal(),
                            KeyCode::KeyP => {
                                self.show_waypoint_menu = !self.show_waypoint_menu;
                                self.show_inventory = false;
                            }
                            KeyCode::Digit1 => {
                                if self.show_waypoint_menu { self.waypoint_travel(0); }
                                else { self.use_potion(0); }
                            }
                            KeyCode::Digit2 => {
                                if self.show_waypoint_menu { self.waypoint_travel(1); }
                                else { self.use_potion(1); }
                            }
                            KeyCode::Digit3 => {
                                if self.show_waypoint_menu { self.waypoint_travel(2); }
                                else { self.use_potion(2); }
                            }
                            KeyCode::Digit4 => {
                                if self.show_waypoint_menu { self.waypoint_travel(3); }
                                else { self.use_potion(3); }
                            }
                            // Skill switching (F1-F5)
                            KeyCode::F1 => self.active_skill = SkillId::Melee,
                            KeyCode::F2 => self.active_skill = SkillId::Fireball,
                            KeyCode::F3 => self.active_skill = SkillId::FrostBolt,
                            KeyCode::F4 => self.active_skill = SkillId::ChainLightning,
                            KeyCode::F5 => self.active_skill = SkillId::Teleport,
                            // Stat allocation (F6-F9)
                            KeyCode::F6 => self.allocate_stat(0), // Str
                            KeyCode::F7 => self.allocate_stat(1), // Dex
                            KeyCode::F8 => self.allocate_stat(2), // Vit
                            KeyCode::F9 => self.allocate_stat(3), // Ene
                            // K = toggle skill disk, L = toggle civil skills
                            KeyCode::KeyK => self.show_disk = !self.show_disk,
                            KeyCode::KeyL => self.show_civil = !self.show_civil,
                            // Summons: F10=Skeleton, F11=Golem, F12=Wolf
                            KeyCode::F10 => self.spawn_summon(SummonKind::Skeleton),
                            KeyCode::F11 => self.spawn_summon(SummonKind::Golem),
                            KeyCode::F12 => self.spawn_summon(SummonKind::Wolf),
                            // Chat toggle
                            KeyCode::Enter => self.show_chat = !self.show_chat,
                            // Difficulty toggle
                            KeyCode::KeyN => self.cycle_difficulty(),
                            // Escape closes dialogs
                            KeyCode::Escape => {
                                self.show_inventory = false;
                                self.show_stats = false;
                                self.show_quests = false;
                                self.show_waypoint_menu = false;
                                self.show_disk = false;
                                self.show_civil = false;
                                self.npc_dialog = None;
                            }
                            _ => {}
                        }
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                self.frame = self.frame.wrapping_add(1);
                let _ = self.sim.tick();
                self.profile.level = self.player_level;
                self.profile.last_scene = self.sim.runtime().scene().id.clone();

                self.update_player_death();

                if !self.player_dead {
                    // WASD (cardinal)
                    let mut dx = 0.0_f32;
                    let mut dy = 0.0_f32;
                    if self.keys & 1 != 0 { dy -= 1.0; }
                    if self.keys & 2 != 0 { dy += 1.0; }
                    if self.keys & 4 != 0 { dx -= 1.0; }
                    if self.keys & 8 != 0 { dx += 1.0; }
                    let len = (dx * dx + dy * dy).sqrt();
                    if len > 0.0 {
                        let spd = if self.running { self.run_speed() } else { self.walk_speed() };
                        Self::try_move(&self.global_terrain, self.world_offset, &mut self.player_pos, dx / len * spd, dy / len * spd);
                        // Update facing direction
                        if dx > 0.0 { self.facing_right = true; }
                        else if dx < 0.0 { self.facing_right = false; }
                    }

                    self.update_click_move();

                    // Update facing based on attack target or move target
                    if let Some(idx) = self.attack_target {
                        if idx < self.enemies.len() {
                            self.facing_right = self.enemies[idx].pos[0] > self.player_pos[0];
                        }
                    } else if let Some(t) = self.move_target {
                        if t[0] > self.player_pos[0] { self.facing_right = true; }
                        else if t[0] < self.player_pos[0] { self.facing_right = false; }
                    }

                }

                // Zone transition must run BEFORE clamping so the player can cross boundaries
                self.check_zone_transition();
                // World bounds enforced by global terrain (tile=0 at edges).

                self.cam_focus = self.player_pos;

                self.update_enemies();
                self.update_combat();
                self.update_enemy_ranged();
                self.update_projectiles();
                self.update_enemy_projectile_hits();
                self.update_mercenary();
                self.update_summons();
                self.update_combo();
                self.update_world_boss();
                self.update_shrine_interact();
                self.update_chest_interact();
                self.update_gather_nodes();
                self.pickup_drops();
                self.update_npc_interact();
                self.check_portal_entry();
                self.update_quests();
                self.update_log();
                // Auto-close NPC dialog after interact timer expires
                if self.npc_dialog.is_some() && self.npc_interact.is_none() {
                    self.npc_dialog = None;
                }

                self.populate_batch();

                if let Some(r) = &mut self.renderer {
                    if let Err(e) = r.render(&self.batch) {
                        self.set_fatal(event_loop, e);
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(w) = &self.window { w.request_redraw(); }
    }
}

fn main() -> Result<()> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let bootstrap_path = manifest_dir.join("data").join("bootstrap.ron");
    let bootstrap = load_bootstrap(&bootstrap_path)?;
    let save_manager = SaveManager::new(&bootstrap.game_id)?;
    let mut app = SodomightApp::new(bootstrap, save_manager)?;
    let event_loop = EventLoop::new().context("failed to create winit event loop")?;

    event_loop.run_app(&mut app).context("failed while running Sodomight app")?;

    if let Some(error) = app.fatal_error {
        return Err(error);
    }

    let _ = app.replication;
    Ok(())
}
