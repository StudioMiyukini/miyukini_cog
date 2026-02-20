//! Rendu avec caméra centrée joueur — MGE Démo Pathfinding

use crate::camera::Camera;
use crate::chunk::{ChunkManager, CHUNK_SIZE, TILE_SIZE};
use crate::entity::Entity;
use crate::faction::AggroBehavior;
use crate::floating_text::FloatingText;
use crate::grid::{Terrain, Vec2};
use crate::invocation::Invocation;
use crate::rally::RallyPoint;

const COLOR_OBSTACLE: u32 = 0xFF_2C_2C_2C;
const COLOR_ROAD: u32 = 0xFF_8B_73_55;
const COLOR_GRASS: u32 = 0xFF_E8_E4_D8;
const COLOR_SAND: u32 = 0xFF_DE_C9_6B;
const COLOR_FOREST: u32 = 0xFF_3D_5C_33;
const COLOR_ENTITY: u32 = 0xFF_E8_3C_50;
const COLOR_INVOCATION: u32 = 0xFF_4A_C4_D4;
/// Orange — comportement agressif
const COLOR_AGGRO: u32 = 0xFF_E8_95_30;
/// Jaune — comportement passif (riposte seulement)
const COLOR_PASSIVE: u32 = 0xFF_E8_E8_30;
/// Blanc — animal (riposte seulement)
const COLOR_NEUTRAL: u32 = 0xFF_E8_E8_E8;
const COLOR_HITBOX_OUTLINE: u32 = 0xFF_80_80_80;
const COLOR_ENTITY_COMBAT: u32 = 0xFF_FF_66_66;
const COLOR_HP_BAR_BG: u32 = 0xFF_40_20_20;
const COLOR_HP_BAR_FILL: u32 = 0xFF_30_C0_30;
const COLOR_DODGE_TEXT: u32 = 0xFF_FF_CC_00;
const COLOR_DAMAGE_TEXT: u32 = 0xFF_FF_33_33;
const COLOR_BG: u32 = 0xFF_1A_1A_1A;
/// Vert pour la balise de ralliement
const COLOR_RALLY_POINT: u32 = 0xFF_30_E8_30;

fn aggro_behavior_color(behavior: AggroBehavior) -> u32 {
    match behavior {
        AggroBehavior::Aggressive => COLOR_AGGRO,
        AggroBehavior::Passive => COLOR_PASSIVE,
        AggroBehavior::Neutral => COLOR_NEUTRAL,
    }
}

fn terrain_color(terrain: Terrain) -> u32 {
    match terrain {
        Terrain::Obstacle => COLOR_OBSTACLE,
        Terrain::Road => COLOR_ROAD,
        Terrain::Grass => COLOR_GRASS,
        Terrain::Sand => COLOR_SAND,
        Terrain::Forest => COLOR_FOREST,
    }
}

pub fn draw_frame(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    camera: &Camera,
    chunk_mgr: &ChunkManager,
    hitbox_half_size: f32,
    entity: &Entity,
    invocations: &[Invocation],
    enemies: &[Entity],
    in_combat_player: bool,
    floating_texts: &[FloatingText],
    rally_point: Option<&RallyPoint>,
) {
    // Fond
    for p in buffer.iter_mut() {
        *p = COLOR_BG;
    }

    // Limites visibles en coordonnées monde
    let (vis_min_x, vis_min_y, vis_max_x, vis_max_y) = camera.visible_bounds(width, height);

    // Tiles visibles (en coordonnées monde)
    let tile_min_x = (vis_min_x / TILE_SIZE).floor() as i32 - 1;
    let tile_min_y = (vis_min_y / TILE_SIZE).floor() as i32 - 1;
    let tile_max_x = (vis_max_x / TILE_SIZE).ceil() as i32 + 1;
    let tile_max_y = (vis_max_y / TILE_SIZE).ceil() as i32 + 1;

    // Bornes du monde (en tiles)
    let world_tile_max_x = chunk_mgr.world_width * CHUNK_SIZE;
    let world_tile_max_y = chunk_mgr.world_height * CHUNK_SIZE;

    // Rendu des tiles visibles
    let ts = TILE_SIZE as usize;
    for gy in tile_min_y..tile_max_y {
        if gy < 0 || gy >= world_tile_max_y {
            continue;
        }
        for gx in tile_min_x..tile_max_x {
            if gx < 0 || gx >= world_tile_max_x {
                continue;
            }
            let terrain = chunk_mgr.terrain_at(gx, gy);
            let cell_color = terrain_color(terrain);

            let world_x = gx as f32 * TILE_SIZE;
            let world_y = gy as f32 * TILE_SIZE;
            let (sx, sy) = camera.world_to_screen(Vec2::new(world_x, world_y), width, height);

            for dy in 0..ts {
                for dx in 0..ts {
                    let x = sx + dx as i32;
                    let y = sy + dy as i32;
                    if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                        buffer[y as usize * width + x as usize] = cell_color;
                    }
                }
            }
        }
    }

    // Joueur
    let entity_color = if in_combat_player {
        COLOR_ENTITY_COMBAT
    } else {
        COLOR_ENTITY
    };
    if entity.is_alive() {
        draw_entity_at(buffer, width, height, camera, &entity.position, hitbox_half_size, entity_color);
        draw_hitbox_at(buffer, width, height, camera, &entity.position, hitbox_half_size);
        draw_health_bar_at(buffer, width, height, camera, &entity.position, hitbox_half_size, entity.combat.pv, entity.combat.pv_max);
    }

    // Invocations
    for inv in invocations {
        if inv.entity.is_alive() {
            let inv_color = if inv.state == crate::invocation::InvocationState::Combat {
                COLOR_ENTITY_COMBAT
            } else {
                COLOR_INVOCATION
            };
            draw_entity_at(buffer, width, height, camera, &inv.entity.position, hitbox_half_size, inv_color);
            draw_hitbox_at(buffer, width, height, camera, &inv.entity.position, hitbox_half_size);
            draw_health_bar_at(buffer, width, height, camera, &inv.entity.position, hitbox_half_size, inv.entity.combat.pv, inv.entity.combat.pv_max);
        }
    }

    // Ennemis
    let any_inv_combat = invocations
        .iter()
        .any(|inv| inv.state == crate::invocation::InvocationState::Combat);
    for enemy in enemies {
        if enemy.is_alive() {
            let enemy_color = if in_combat_player || any_inv_combat {
                COLOR_ENTITY_COMBAT
            } else {
                aggro_behavior_color(enemy.aggro_behavior)
            };
            draw_entity_at(buffer, width, height, camera, &enemy.position, hitbox_half_size, enemy_color);
            draw_hitbox_at(buffer, width, height, camera, &enemy.position, hitbox_half_size);
            draw_health_bar_at(buffer, width, height, camera, &enemy.position, hitbox_half_size, enemy.combat.pv, enemy.combat.pv_max);
        }
    }

    // Balise de ralliement (cercle vert pulsant)
    if let Some(rally) = rally_point {
        if rally.timer > 0.0 {
            let pulse = ((rally.timer * 4.0) as i32 % 2) == 0;
            let radius = if pulse { 18.0 } else { 12.0 };
            draw_rally_point_at(buffer, width, height, camera, &rally.position, radius);
        }
    }

    // Textes flottants
    for ft in floating_texts {
        draw_floating_text(buffer, width, height, camera, ft);
    }
}

fn draw_rally_point_at(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    camera: &Camera,
    pos: &Vec2,
    radius: f32,
) {
    let (cx, cy) = camera.world_to_screen(*pos, width, height);
    let r = radius as i32;
    for dy in -r..=r {
        for dx in -r..=r {
            let d = ((dx * dx + dy * dy) as f32).sqrt();
            if d <= radius + 0.5 {
                let x = cx + dx;
                let y = cy + dy;
                if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                    buffer[y as usize * width + x as usize] = COLOR_RALLY_POINT;
                }
            }
        }
    }
}

fn draw_entity_at(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    camera: &Camera,
    pos: &Vec2,
    half_size: f32,
    color: u32,
) {
    let (cx, cy) = camera.world_to_screen(*pos, width, height);
    let s = half_size as i32;
    for dy in -s..=s {
        for dx in -s..=s {
            let x = cx + dx;
            let y = cy + dy;
            if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                buffer[y as usize * width + x as usize] = color;
            }
        }
    }
}

fn draw_hitbox_at(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    camera: &Camera,
    pos: &Vec2,
    half_size: f32,
) {
    let (cx, cy) = camera.world_to_screen(*pos, width, height);
    let s = half_size.ceil() as i32;
    for d in -s..=s {
        let pixels = [
            (cx - s, cy + d),
            (cx + s, cy + d),
            (cx + d, cy - s),
            (cx + d, cy + s),
        ];
        for (px, py) in pixels {
            if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                buffer[py as usize * width + px as usize] = COLOR_HITBOX_OUTLINE;
            }
        }
    }
}

fn draw_health_bar_at(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    camera: &Camera,
    pos: &Vec2,
    half_size: f32,
    pv: i32,
    pv_max: i32,
) {
    let (cx, cy) = camera.world_to_screen(*pos, width, height);
    let bar_w = (half_size * 2.0) as i32;
    let bar_h = 2i32;
    let left = cx - bar_w / 2;
    let top = cy + half_size as i32 + 1;

    let ratio = if pv_max > 0 {
        (pv as f32 / pv_max as f32).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let fill_w = (bar_w as f32 * ratio).round() as i32;

    for dy in 0..bar_h {
        for dx in 0..bar_w {
            let x = left + dx;
            let y = top + dy;
            if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                let color = if dx < fill_w {
                    COLOR_HP_BAR_FILL
                } else {
                    COLOR_HP_BAR_BG
                };
                buffer[y as usize * width + x as usize] = color;
            }
        }
    }
}

fn draw_floating_text(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    camera: &Camera,
    ft: &FloatingText,
) {
    let world_pos = Vec2::new(ft.base_pos.x, ft.screen_y());
    let (cx, cy) = camera.world_to_screen(world_pos, width, height);

    match &ft.kind {
        crate::floating_text::FloatingTextKind::Dodge => {
            draw_text_dodge(buffer, width, height, cx, cy, COLOR_DODGE_TEXT);
        }
        crate::floating_text::FloatingTextKind::Damage(dmg) => {
            draw_text_number(buffer, width, height, cx, cy, *dmg, COLOR_DAMAGE_TEXT);
        }
    }
}

fn draw_text_dodge(buffer: &mut [u32], width: usize, height: usize, cx: i32, cy: i32, color: u32) {
    let left = cx - 12;
    let top = cy;
    let chars: [[u8; 4]; 5] = [
        [0xF, 0x9, 0x9, 0xF],
        [0x7, 0x9, 0x9, 0x7],
        [0xF, 0x9, 0x9, 0xF],
        [0xF, 0x8, 0xB, 0xF],
        [0xF, 0x4, 0x4, 0xF],
    ];
    for (ci, pattern) in chars.iter().enumerate() {
        let px = left + (ci as i32) * 5;
        draw_char_4x4(buffer, width, height, px, top, pattern, color);
    }
}

fn draw_text_number(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    cx: i32,
    cy: i32,
    n: i32,
    color: u32,
) {
    let digits: Vec<u8> = if n <= 0 {
        vec![0]
    } else {
        let mut v = Vec::new();
        let mut x = n;
        while x > 0 {
            v.push((x % 10) as u8);
            x /= 10;
        }
        v.reverse();
        v
    };
    let total_w = digits.len() as i32 * 4;
    let left = cx - total_w / 2;
    for (i, &d) in digits.iter().enumerate() {
        let pattern = DIGIT_3X4[d as usize];
        draw_char_3x4(
            buffer,
            width,
            height,
            left + (i as i32) * 4,
            cy,
            &pattern,
            color,
        );
    }
}

const DIGIT_3X4: [[u8; 4]; 10] = [
    [0x7, 0x5, 0x5, 0x7],
    [0x2, 0x6, 0x2, 0x7],
    [0x7, 0x1, 0x7, 0x7],
    [0x7, 0x3, 0x1, 0x7],
    [0x5, 0x5, 0x7, 0x1],
    [0x7, 0x4, 0x1, 0x7],
    [0x7, 0x4, 0x5, 0x7],
    [0x7, 0x1, 0x1, 0x1],
    [0x7, 0x5, 0x5, 0x7],
    [0x7, 0x5, 0x7, 0x1],
];

fn draw_char_4x4(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    px: i32,
    py: i32,
    pattern: &[u8; 4],
    color: u32,
) {
    for row in 0..4i32 {
        let row_bits = pattern[row as usize];
        for col in 0..4i32 {
            if (row_bits >> (3 - col)) & 1 != 0 {
                let x = px + col;
                let y = py + row;
                if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                    buffer[y as usize * width + x as usize] = color;
                }
            }
        }
    }
}

fn draw_char_3x4(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    px: i32,
    py: i32,
    pattern: &[u8; 4],
    color: u32,
) {
    for row in 0..4i32 {
        let row_bits = pattern[row as usize];
        for col in 0..3i32 {
            if (row_bits >> (2 - col)) & 1 != 0 {
                let x = px + col;
                let y = py + row;
                if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
                    buffer[y as usize * width + x as usize] = color;
                }
            }
        }
    }
}
