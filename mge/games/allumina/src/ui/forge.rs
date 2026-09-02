//! @id     games_allumina_ui_forge
//! @do     definir_row_h
//! @role   ui
//! @layer  core
//! @human  games allumina — module « ui forge ».

//! Forge window: recycle a backpack item (left column) into crafting
//! materials, whose running totals show on the right. Click an item row to
//! salvage it.

use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::ecs::{Entity, World};
use crate::render::font;
use crate::render::vertex;
use crate::systems::forge::{salvage_yield, Material};

use super::{border, fill, panel, point_in, Rect, SUB_CONTENT, SUB_TEXT};

const ROW_H: f32 = 26.0;
const COL_W: f32 = 250.0;
const MAT_W: f32 = 170.0;
const ROWS: usize = 10;

/// Centred forge panel rect.
#[must_use]
pub fn panel_rect(viewport: [f32; 2]) -> Rect {
    let w = COL_W + MAT_W + 36.0;
    let h = ROWS as f32 * ROW_H + 70.0;
    [viewport[0] * 0.5 - w * 0.5, viewport[1] * 0.5 - h * 0.5, w, h]
}

#[allow(clippy::cast_precision_loss)]
fn item_row_rect(viewport: [f32; 2], i: usize) -> Rect {
    let p = panel_rect(viewport);
    [p[0] + 12.0, p[1] + 50.0 + i as f32 * ROW_H, COL_W, ROW_H - 3.0]
}

/// Occupied backpack slot indices (the recyclable rows).
fn recyclable(world: &World, player: Entity) -> Vec<usize> {
    world
        .inventory
        .get(&player)
        .map(|inv| (0..inv.slots.len()).filter(|&i| inv.get(i).is_some()).collect())
        .unwrap_or_default()
}

/// Inventory slot index under the cursor, if any.
#[must_use]
pub fn recycle_row_at(world: &World, player: Entity, viewport: [f32; 2], mouse: [f32; 2]) -> Option<usize> {
    recyclable(world, player)
        .iter()
        .take(ROWS)
        .enumerate()
        .find(|(row, _)| point_in(item_row_rect(viewport, *row), mouse))
        .map(|(_, &slot)| slot)
}

/// Draw the forge window.
#[allow(clippy::cast_possible_truncation)]
pub fn draw(batch: &mut SpriteBatch, world: &World, viewport: [f32; 2], mouse: [f32; 2]) {
    let Some(player) = world.player_entity else { return };
    let prect = panel_rect(viewport);
    panel(batch, prect, b"FORGE");

    font::draw_text(batch, b"RECYCLER", prect[0] + 12.0, prect[1] + 32.0, 1.4, [0.9, 0.7, 0.4, 1.0], RenderLayer::UiScreen, 0, SUB_TEXT);
    font::draw_text(batch, b"MATERIAUX", prect[0] + COL_W + 24.0, prect[1] + 32.0, 1.4, [0.7, 0.85, 0.95, 1.0], RenderLayer::UiScreen, 0, SUB_TEXT);

    // Recyclable item rows.
    if let Some(inv) = world.inventory.get(&player) {
        for (row, slot) in recyclable(world, player).into_iter().take(ROWS).enumerate() {
            if let Some(item) = inv.get(slot) {
                let r = item_row_rect(viewport, row);
                draw_item_row(batch, r, &item.name, &salvage_yield(item), vertex::rarity_color(item.rarity().color_name()), point_in(r, mouse));
            }
        }
    }

    // Material totals.
    if let Some(inv) = world.inventory.get(&player) {
        let mx = prect[0] + COL_W + 24.0;
        for (i, mat) in Material::ALL.into_iter().enumerate() {
            let y = prect[1] + 50.0 + i as f32 * ROW_H;
            let label: Vec<u8> = mat.label().bytes().filter(u8::is_ascii).collect();
            font::draw_text(batch, &label, mx, y + 5.0, 1.3, [0.82, 0.82, 0.88, 1.0], RenderLayer::UiScreen, 0, SUB_TEXT);
            let (c, cn) = font::fmt_u32(inv.materials.get(mat));
            font::draw_text(batch, &c[..cn], mx + MAT_W - 44.0, y + 5.0, 1.3, vertex::XP, RenderLayer::UiScreen, 0, SUB_TEXT);
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
fn draw_item_row(
    batch: &mut SpriteBatch,
    r: Rect,
    name: &str,
    yield_: &[(Material, u32)],
    color: vertex::Color,
    hovered: bool,
) {
    fill(batch, r, SUB_CONTENT, if hovered { [0.18, 0.18, 0.22, 1.0] } else { [0.10, 0.10, 0.13, 1.0] });
    border(batch, r, 1.0, SUB_CONTENT + 1, if hovered { vertex::WHITE } else { [0.3, 0.3, 0.35, 1.0] });
    // Item name (truncated) tinted by rarity.
    let label: Vec<u8> = name.bytes().filter(u8::is_ascii).take(16).collect();
    font::draw_text(batch, &label, r[0] + 6.0, r[1] + 6.0, 1.3, color, RenderLayer::UiScreen, 0, SUB_TEXT);
    // Yield summary on the right (e.g. "3F 1A"), first letters of each material.
    let mut summary = Vec::new();
    for (m, n) in yield_.iter().take(3) {
        let (c, cn) = font::fmt_u32(*n);
        summary.extend_from_slice(&c[..cn]);
        summary.push(m.label().bytes().next().unwrap_or(b'?'));
        summary.push(b' ');
    }
    font::draw_text(batch, &summary, r[0] + r[2] - 72.0, r[1] + 6.0, 1.2, [0.7, 0.85, 0.95, 1.0], RenderLayer::UiScreen, 0, SUB_TEXT);
}
