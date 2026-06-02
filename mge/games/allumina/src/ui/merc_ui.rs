//! Mercenary equipment panel (toggled with `P`).
//!
//! A compact paperdoll for the player's active companion: the same ten slots as
//! the hero's sheet, fed from the merc's [`MercInventory`] instead of the
//! player's `equipment`. Items are equipped by dragging from the backpack onto a
//! slot (or onto the merc in the world) and removed by clicking a worn slot —
//! both routed through `plugins::inventory`. A short derived-stat line shows the
//! companion's defence, damage, and block so the effect of gear is visible.

use mge_items::item::{EquipSlot, ItemKind};
use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::components::mercenary::MercType;
use crate::ecs::{Entity, World};
use crate::systems::mercenary::companions;
use crate::render::item_icon;
use crate::render::shapes;
use crate::render::{font, vertex};

use super::{
    border, dotted_border, fill, panel, point_in, Rect, UiState, EQUIP_SLOTS, SUB_CONTENT,
    SUB_PANEL, SUB_TEXT,
};

/// The item kind a paperdoll slot expects (drives the empty-slot ghost icon).
fn expected_kind(slot: EquipSlot) -> ItemKind {
    match slot {
        EquipSlot::Head => ItemKind::Helm,
        EquipSlot::Chest => ItemKind::BodyArmor,
        EquipSlot::Belt => ItemKind::Belt,
        EquipSlot::Gloves => ItemKind::Gloves,
        EquipSlot::Boots => ItemKind::Boots,
        EquipSlot::OffHand => ItemKind::Shield,
        EquipSlot::Ring1 | EquipSlot::Ring2 => ItemKind::Ring,
        EquipSlot::Amulet => ItemKind::Amulet,
        EquipSlot::MainHand => ItemKind::Sword,
    }
}

/// The active mercenary owned by `player`, if any (drives the panel target).
#[must_use]
pub fn active_merc(world: &World, player: Entity) -> Option<Entity> {
    crate::systems::mercenary::active_mercenary(world, player)
}

/// Full panel rect, centred horizontally so it sits between the (left) hero
/// sheet and the (right) backpack.
#[must_use]
pub fn merc_panel_rect(viewport: [f32; 2]) -> Rect {
    let (w, h) = (300.0, 360.0);
    [(viewport[0] - w) * 0.5, 90.0, w, h]
}

/// Rect of one paperdoll slot, in the same 3-column arrangement as the hero's.
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn merc_slot_rect(viewport: [f32; 2], slot: EquipSlot) -> Rect {
    let p = merc_panel_rect(viewport);
    let (col, row) = match slot {
        EquipSlot::Head => (1.0, 0.0),
        EquipSlot::Amulet => (2.0, 0.0),
        EquipSlot::MainHand => (0.0, 1.0),
        EquipSlot::Chest => (1.0, 1.0),
        EquipSlot::OffHand => (2.0, 1.0),
        EquipSlot::Gloves => (0.0, 2.0),
        EquipSlot::Belt => (1.0, 2.0),
        EquipSlot::Boots => (2.0, 2.0),
        EquipSlot::Ring1 => (0.0, 3.0),
        EquipSlot::Ring2 => (2.0, 3.0),
    };
    let cell = super::SLOT + 14.0;
    let x = p[0] + 30.0 + col * (cell + 16.0);
    let y = p[1] + 40.0 + row * (cell + 12.0);
    [x, y, cell, cell]
}

/// The merc slot under `mouse`, if the panel is open.
#[must_use]
pub fn merc_slot_at(viewport: [f32; 2], mouse: [f32; 2]) -> Option<EquipSlot> {
    EQUIP_SLOTS.into_iter().find(|&s| point_in(merc_slot_rect(viewport, s), mouse))
}

/// Rect of companion-selector tab `idx` of `count`, laid out in the title bar's
/// right edge.
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn tab_rect(viewport: [f32; 2], idx: usize, count: usize) -> Rect {
    let p = merc_panel_rect(viewport);
    let (tw, gap) = (22.0, 4.0);
    let total = count as f32 * (tw + gap) - gap;
    let x0 = p[0] + p[2] - total - 8.0;
    [x0 + idx as f32 * (tw + gap), p[1] + 4.0, tw, 18.0]
}

/// The companion-tab index under `mouse`, given how many `count` tabs show.
#[must_use]
pub fn tab_at(viewport: [f32; 2], mouse: [f32; 2], count: usize) -> Option<usize> {
    (0..count).find(|&i| point_in(tab_rect(viewport, i, count), mouse))
}

/// Draw the merc paperdoll + a short derived-stat line.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn draw(batch: &mut SpriteBatch, world: &World, ui: &UiState, viewport: [f32; 2], time: f32) {
    let Some(merc) = ui.merc_panel else { return };
    let prect = merc_panel_rect(viewport);

    let merc_type = world.mercenary_tags.get(&merc).map_or(MercType::Guardian, |t| t.merc_type);
    let level = world.merc_states.get(&merc).map_or(1, |s| s.level);
    let title = format!("{} (Niv {level})", merc_type.display_name());
    panel(batch, prect, title.to_uppercase().as_bytes());

    // Companion selector tabs (one per owned merc) in the title bar's right edge.
    let Some(player) = world.player_entity else { return };
    let party = companions(world, player);
    if party.len() > 1 {
        for (i, &c) in party.iter().enumerate() {
            let r = tab_rect(viewport, i, party.len());
            let selected = Some(c) == ui.merc_panel;
            fill(batch, r, SUB_TEXT, if selected { [0.30, 0.40, 0.26, 1.0] } else { [0.12, 0.14, 0.11, 1.0] });
            border(batch, r, 1.0, SUB_TEXT + 1, if selected { vertex::WHITE } else { [0.40, 0.45, 0.34, 1.0] });
            let kind = world.mercenary_tags.get(&c).map_or(MercType::Guardian, |t| t.merc_type);
            let initial = kind.display_name().as_bytes().first().copied().unwrap_or(b'?');
            font::draw_text(batch, &[initial], r[0] + r[2] * 0.5 - 3.0, r[1] + 4.0, 1.5, vertex::WHITE, RenderLayer::UiScreen, 0, SUB_TEXT + 2);
        }
    }

    draw_silhouette(batch, prect);

    let inv = world.merc_inventory.get(&merc);
    for slot in EQUIP_SLOTS {
        let r = merc_slot_rect(viewport, slot);
        let hovered = ui.hover_merc_equip == Some(slot);
        fill(batch, r, SUB_CONTENT, [0.13, 0.13, 0.17, 1.0]);
        let cx = r[0] + r[2] * 0.5;
        let cy = r[1] + r[3] * 0.5;
        let size = (r[2] - 10.0).min(r[3] - 10.0);
        match inv.and_then(|i| i.get(slot)) {
            Some(item) => {
                let rim = if hovered { vertex::rarity_color(item.rarity().color_name()) } else { [0.30, 0.35, 0.28, 1.0] };
                border(batch, r, 1.0, SUB_CONTENT + 1, rim);
                item_icon::draw_item_icon(batch, item.item.base.kind, cx, cy, size, item.rarity(), RenderLayer::UiScreen, 0, SUB_CONTENT + 2, time);
            }
            None => {
                dotted_border(batch, r, 1.0, SUB_CONTENT + 1, if hovered { vertex::WHITE } else { [0.36, 0.40, 0.30, 1.0] });
                item_icon::draw_ghost_icon(batch, expected_kind(slot), cx, cy, size, RenderLayer::UiScreen, 0, SUB_CONTENT + 2);
            }
        }
    }

    // Derived-stat line: defence / damage / block from the merc's resolved sheet.
    if let Some(s) = world.stats.get(&merc) {
        let r = &s.inner.resolved;
        let line = format!("DEF {}   DMG {}-{}   Bloc {}%", r.defense.max(0), r.damage_min.max(0), r.damage_max.max(0), r.block_chance.clamp(0, 100));
        font::draw_text(batch, line.as_bytes(), prect[0] + 14.0, prect[1] + prect[3] - 44.0, 1.4, [0.78, 0.82, 0.7, 1.0], RenderLayer::UiScreen, 0, SUB_TEXT);
    }
    font::draw_text(batch, b"Glisser un objet ici ou cliquer pour retirer", prect[0] + 14.0, prect[1] + prect[3] - 24.0, 1.1, [0.6, 0.62, 0.55, 1.0], RenderLayer::UiScreen, 0, SUB_TEXT);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_slot_round_trips_through_hit_test() {
        let vp = [1280.0, 720.0];
        for slot in EQUIP_SLOTS {
            let r = merc_slot_rect(vp, slot);
            // The slot's centre must hit-test back to the same slot, and sit
            // inside the panel.
            let c = [r[0] + r[2] * 0.5, r[1] + r[3] * 0.5];
            assert_eq!(merc_slot_at(vp, c), Some(slot), "{slot:?} centre mis-hits");
            assert!(point_in(merc_panel_rect(vp), c), "{slot:?} sits outside the panel");
        }
        // A point well outside hits nothing.
        assert_eq!(merc_slot_at(vp, [5.0, 5.0]), None);
    }
}

/// Faint companion-green humanoid behind the slots.
fn draw_silhouette(batch: &mut SpriteBatch, prect: Rect) {
    let cx = prect[0] + prect[2] * 0.5;
    let top = prect[1] + 44.0;
    let body = [0.20, 0.28, 0.20, 0.55];
    let sub = SUB_PANEL + 2;
    shapes::circle(batch, cx, top + 18.0, 16.0, RenderLayer::UiScreen, 0, sub, body);
    shapes::rounded_rect(batch, cx - 26.0, top + 34.0, 52.0, 84.0, 14.0, RenderLayer::UiScreen, 0, sub, body);
    shapes::rounded_rect(batch, cx - 44.0, top + 40.0, 16.0, 64.0, 8.0, RenderLayer::UiScreen, 0, sub, body);
    shapes::rounded_rect(batch, cx + 28.0, top + 40.0, 16.0, 64.0, 8.0, RenderLayer::UiScreen, 0, sub, body);
    shapes::rounded_rect(batch, cx - 22.0, top + 116.0, 18.0, 70.0, 8.0, RenderLayer::UiScreen, 0, sub, body);
    shapes::rounded_rect(batch, cx + 4.0, top + 116.0, 18.0, 70.0, 8.0, RenderLayer::UiScreen, 0, sub, body);
}
