//! @id     games_allumina_ui_inventory_ui
//! @do     draw
//! @role   ui
//! @layer  core
//! @human  games allumina — module « ui inventory ui ».

//! Inventory window: the 10x4 backpack grid.
//!
//! `render_inventory` is the text version (headless); `draw` paints the
//! interactive grid with rarity-coloured item icons, hover highlight, and gold.

use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::components::equipment::{Inventory, INV_WIDTH};
use crate::components::item::ItemInstance;
use crate::ecs::World;
use crate::render::item_icon;
use crate::render::shapes;
use crate::render::{font, vertex};

use super::{fill, inventory_panel_rect, inventory_slot_rect, panel, Rect, UiState, SUB_CONTENT, SUB_TEXT};

/// Draw the interactive backpack grid.
#[allow(clippy::cast_possible_truncation)]
pub fn draw(batch: &mut SpriteBatch, world: &World, ui: &UiState, viewport: [f32; 2], time: f32) {
    let Some(player) = world.player_entity else { return };
    let Some(inv) = world.inventory.get(&player) else { return };

    let prect = inventory_panel_rect(viewport);
    panel(batch, prect, b"INVENTORY");

    // Gold line.
    let (g, gn) = font::fmt_u32(inv.gold.min(u64::from(u32::MAX)) as u32);
    let mut gold = Vec::new();
    gold.extend_from_slice(b"GOLD ");
    gold.extend_from_slice(&g[..gn]);
    font::draw_text(batch, &gold, prect[0] + 10.0, prect[1] + prect[3] - 22.0, 1.6, vertex::XP, RenderLayer::UiScreen, 0, SUB_TEXT);

    for idx in 0..inv.slots.len() {
        let r = inventory_slot_rect(viewport, idx);
        let hovered = ui.hover_inv == Some(idx);
        fill(batch, r, SUB_CONTENT, [0.10, 0.10, 0.13, 1.0]);
        let item = inv.get(idx);
        // Hovered cells take a rarity-coloured rim (white when empty).
        let rim = match (hovered, item) {
            (true, Some(it)) => vertex::rarity_color(it.rarity().color_name()),
            (true, None) => vertex::WHITE,
            (false, _) => [0.3, 0.3, 0.35, 1.0],
        };
        super::border(batch, r, 1.0, SUB_CONTENT + 1, rim);

        // Skip the icon of the slot being dragged (it's drawn at the cursor).
        if ui.drag == Some(idx) {
            continue;
        }
        if let Some(item) = item {
            draw_item_cell(batch, r, item, time);
        }
    }
}

/// Draw one item in its cell: a dark plate plus the shared rarity-aware icon,
/// padded to ~85% of the cell.
fn draw_item_cell(batch: &mut SpriteBatch, r: Rect, item: &ItemInstance, time: f32) {
    shapes::rounded_rect(batch, r[0] + 3.0, r[1] + 3.0, r[2] - 6.0, r[3] - 6.0, 4.0, RenderLayer::UiScreen, 0, SUB_CONTENT + 2, [0.07, 0.07, 0.10, 1.0]);
    let cx = r[0] + r[2] * 0.5;
    let cy = r[1] + r[3] * 0.5;
    let size = (r[2] - 6.0).min(r[3] - 6.0) * 0.92;
    item_icon::draw_item_icon(batch, item.item.base.kind, cx, cy, size, item.rarity(), RenderLayer::UiScreen, 0, SUB_CONTENT + 3, time);
}

fn cell_label(item: Option<&ItemInstance>) -> String {
    match item {
        None => "  .  ".to_owned(),
        Some(i) => {
            let tag: String = i.name.chars().take(4).collect();
            // Rarity initial flags the colour tier.
            let r = i.rarity().color_name().chars().next().unwrap_or('?');
            format!("{tag:<4}{r}")
        }
    }
}

pub fn render_inventory(inv: &Inventory) -> String {
    let mut out = format!("Inventory ({}/{} slots)   Gold: {}\n", inv.count(), inv.slots.len(), inv.gold);
    for (i, slot) in inv.slots.iter().enumerate() {
        out.push('|');
        out.push_str(&cell_label(slot.as_ref()));
        if (i + 1) % INV_WIDTH == 0 {
            out.push_str("|\n");
        }
    }
    out
}
