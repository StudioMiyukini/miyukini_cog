//! @id     games_allumina_ui_vendor
//! @do     definir_row_h
//! @role   ui
//! @layer  core
//! @human  games allumina — module « ui vendor ».

//! Vendor window: buy from the merchant's stock (left) or sell from the
//! backpack (right). Click a row to transact.

use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::ecs::{Entity, World};
use crate::render::font;
use crate::render::vertex;
use crate::systems::vendor::{price, sell_value};

use super::{border, fill, panel, point_in, Rect, SUB_CONTENT, SUB_TEXT};

const ROW_H: f32 = 26.0;
const COL_W: f32 = 236.0;
const ROWS: usize = 10;

/// Centred vendor panel rect.
#[must_use]
pub fn panel_rect(viewport: [f32; 2]) -> Rect {
    let w = COL_W * 2.0 + 36.0;
    let h = ROWS as f32 * ROW_H + 70.0;
    [viewport[0] * 0.5 - w * 0.5, viewport[1] * 0.5 - h * 0.5, w, h]
}

#[allow(clippy::cast_precision_loss)]
fn buy_row_rect(viewport: [f32; 2], i: usize) -> Rect {
    let p = panel_rect(viewport);
    [p[0] + 12.0, p[1] + 50.0 + i as f32 * ROW_H, COL_W, ROW_H - 3.0]
}

#[allow(clippy::cast_precision_loss)]
fn sell_row_rect(viewport: [f32; 2], i: usize) -> Rect {
    let p = panel_rect(viewport);
    [p[0] + COL_W + 24.0, p[1] + 50.0 + i as f32 * ROW_H, COL_W, ROW_H - 3.0]
}

/// Occupied backpack slot indices (the sellable rows).
fn sellable(world: &World, player: Entity) -> Vec<usize> {
    world
        .inventory
        .get(&player)
        .map(|inv| (0..inv.slots.len()).filter(|&i| inv.get(i).is_some()).collect())
        .unwrap_or_default()
}

/// Stock index under the cursor, if any.
#[must_use]
pub fn buy_row_at(world: &World, merchant: Entity, viewport: [f32; 2], mouse: [f32; 2]) -> Option<usize> {
    let n = world.merchants.get(&merchant).map_or(0, |m| m.stock.len());
    (0..n.min(ROWS)).find(|&i| point_in(buy_row_rect(viewport, i), mouse))
}

/// Inventory slot index under the cursor, if any.
#[must_use]
pub fn sell_row_at(world: &World, player: Entity, viewport: [f32; 2], mouse: [f32; 2]) -> Option<usize> {
    let occupied = sellable(world, player);
    occupied
        .iter()
        .take(ROWS)
        .enumerate()
        .find(|(row, _)| point_in(sell_row_rect(viewport, *row), mouse))
        .map(|(_, &slot)| slot)
}

/// Draw the vendor window.
#[allow(clippy::cast_possible_truncation)]
pub fn draw(batch: &mut SpriteBatch, world: &World, merchant: Entity, viewport: [f32; 2], mouse: [f32; 2]) {
    let Some(player) = world.player_entity else { return };
    let prect = panel_rect(viewport);
    panel(batch, prect, b"MERCHANT");

    // Player's gold.
    let gold = world.inventory.get(&player).map_or(0, |i| i.gold);
    let (g, gn) = font::fmt_u32(gold.min(u64::from(u32::MAX)) as u32);
    let mut gline = Vec::new();
    gline.extend_from_slice(b"GOLD ");
    gline.extend_from_slice(&g[..gn]);
    font::draw_text(batch, &gline, prect[0] + prect[2] - 140.0, prect[1] + 10.0, 1.6, vertex::XP, RenderLayer::UiScreen, 0, SUB_TEXT);

    font::draw_text(batch, b"BUY", prect[0] + 12.0, prect[1] + 32.0, 1.4, [0.6, 0.9, 0.6, 1.0], RenderLayer::UiScreen, 0, SUB_TEXT);
    font::draw_text(batch, b"SELL", prect[0] + COL_W + 24.0, prect[1] + 32.0, 1.4, [0.9, 0.8, 0.5, 1.0], RenderLayer::UiScreen, 0, SUB_TEXT);

    // Buy rows.
    if let Some(m) = world.merchants.get(&merchant) {
        for (i, item) in m.stock.iter().take(ROWS).enumerate() {
            let r = buy_row_rect(viewport, i);
            draw_row(batch, r, &item.name, price(item), vertex::rarity_color(item.rarity().color_name()), point_in(r, mouse));
        }
    }

    // Sell rows.
    if let Some(inv) = world.inventory.get(&player) {
        for (row, slot) in sellable(world, player).into_iter().take(ROWS).enumerate() {
            if let Some(item) = inv.get(slot) {
                let r = sell_row_rect(viewport, row);
                draw_row(batch, r, &item.name, sell_value(item), vertex::rarity_color(item.rarity().color_name()), point_in(r, mouse));
            }
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
fn draw_row(batch: &mut SpriteBatch, r: Rect, name: &str, cost: u64, color: vertex::Color, hovered: bool) {
    fill(batch, r, SUB_CONTENT, if hovered { [0.18, 0.18, 0.22, 1.0] } else { [0.10, 0.10, 0.13, 1.0] });
    border(batch, r, 1.0, SUB_CONTENT + 1, if hovered { vertex::WHITE } else { [0.3, 0.3, 0.35, 1.0] });
    // Name (truncated) tinted by rarity.
    let label: Vec<u8> = name.bytes().filter(u8::is_ascii).take(18).collect();
    font::draw_text(batch, &label, r[0] + 6.0, r[1] + 6.0, 1.3, color, RenderLayer::UiScreen, 0, SUB_TEXT);
    // Price (right-aligned-ish).
    let (c, cn) = font::fmt_u32(cost.min(u64::from(u32::MAX)) as u32);
    font::draw_text(batch, &c[..cn], r[0] + r[2] - 60.0, r[1] + 6.0, 1.3, vertex::XP, RenderLayer::UiScreen, 0, SUB_TEXT);
}
