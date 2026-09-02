//! @id     games_allumina_ui_equipment_ui
//! @do     expected_kind
//! @role   ui
//! @layer  core
//! @human  games allumina — module « ui equipment ui ».

//! Equipment window.
//!
//! `render_equipment` is the text version (headless); `draw` paints the
//! paperdoll (10 slots) plus the live character sheet (level, attributes with
//! `+`/`-` allocator buttons, derived stats). Hovering a worn slot shows its
//! tooltip (handled in `mod`).

use mge_items::item::{EquipSlot, ItemKind};
use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::components::equipment::EquipmentSlots;
use crate::ecs::World;
use crate::render::item_icon;
use crate::render::shapes;
use crate::render::{font, vertex};
use crate::systems::combat;
use crate::systems::stats::PrimaryAttr;

use super::{
    border, dotted_border, equipment_panel_rect, equipment_slot_rect, fill, panel, stat_button_rect,
    Rect, UiState, EQUIP_SLOTS, SUB_CONTENT, SUB_PANEL, SUB_TEXT,
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

const SLOTS: [EquipSlot; 10] = [
    EquipSlot::Head,
    EquipSlot::Amulet,
    EquipSlot::Chest,
    EquipSlot::Belt,
    EquipSlot::Gloves,
    EquipSlot::Boots,
    EquipSlot::MainHand,
    EquipSlot::OffHand,
    EquipSlot::Ring1,
    EquipSlot::Ring2,
];

pub fn render_equipment(eq: &EquipmentSlots) -> String {
    let mut out = String::from("Equipment\n");
    for slot in SLOTS {
        let label = eq.get(slot).map_or_else(
            || "(empty)".to_owned(),
            |i| format!("{} [{}]", i.name, i.rarity().color_name()),
        );
        out.push_str(&format!("  {slot:>9?}: {label}\n"));
    }
    out
}

/// Draw the interactive paperdoll + character sheet.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn draw(batch: &mut SpriteBatch, world: &World, ui: &UiState, viewport: [f32; 2], _mouse: [f32; 2], time: f32) {
    let Some(player) = world.player_entity else { return };
    let prect = equipment_panel_rect(viewport);
    panel(batch, prect, b"CHARACTER");

    // Greyed paperdoll silhouette behind the slots.
    draw_paperdoll_silhouette(batch, prect);

    let eq = world.equipment.get(&player);
    for slot in EQUIP_SLOTS {
        let r = equipment_slot_rect(viewport, slot);
        let hovered = ui.hover_equip == Some(slot);
        // Slot background is a touch lighter than the inventory cells.
        fill(batch, r, SUB_CONTENT, [0.13, 0.13, 0.17, 1.0]);
        let cx = r[0] + r[2] * 0.5;
        let cy = r[1] + r[3] * 0.5;
        let size = (r[2] - 10.0).min(r[3] - 10.0);
        match eq.and_then(|e| e.get(slot)) {
            Some(item) => {
                let rim = if hovered { vertex::rarity_color(item.rarity().color_name()) } else { [0.35, 0.30, 0.20, 1.0] };
                super::border(batch, r, 1.0, SUB_CONTENT + 1, rim);
                item_icon::draw_item_icon(batch, item.item.base.kind, cx, cy, size, item.rarity(), RenderLayer::UiScreen, 0, SUB_CONTENT + 2, time);
            }
            None => {
                // Empty: dotted rim + a faint ghost of the expected kind.
                dotted_border(batch, r, 1.0, SUB_CONTENT + 1, if hovered { vertex::WHITE } else { [0.40, 0.36, 0.28, 1.0] });
                item_icon::draw_ghost_icon(batch, expected_kind(slot), cx, cy, size, RenderLayer::UiScreen, 0, SUB_CONTENT + 2);
            }
        }
    }

    draw_stat_sheet(batch, world, player, viewport, [prect[0] + 12.0, prect[1] + 232.0]);
}

/// Faint grey humanoid behind the paperdoll slots.
fn draw_paperdoll_silhouette(batch: &mut SpriteBatch, prect: Rect) {
    let cx = prect[0] + prect[2] * 0.5;
    let top = prect[1] + 44.0;
    let body = [0.20, 0.20, 0.24, 0.6];
    let sub = SUB_PANEL + 2;
    let ys = 0;
    // Head, torso, arms, legs.
    shapes::circle(batch, cx, top + 18.0, 16.0, RenderLayer::UiScreen, ys, sub, body);
    shapes::rounded_rect(batch, cx - 26.0, top + 34.0, 52.0, 84.0, 14.0, RenderLayer::UiScreen, ys, sub, body);
    shapes::rounded_rect(batch, cx - 44.0, top + 40.0, 16.0, 64.0, 8.0, RenderLayer::UiScreen, ys, sub, body);
    shapes::rounded_rect(batch, cx + 28.0, top + 40.0, 16.0, 64.0, 8.0, RenderLayer::UiScreen, ys, sub, body);
    shapes::rounded_rect(batch, cx - 22.0, top + 116.0, 18.0, 70.0, 8.0, RenderLayer::UiScreen, ys, sub, body);
    shapes::rounded_rect(batch, cx + 4.0, top + 116.0, 18.0, 70.0, 8.0, RenderLayer::UiScreen, ys, sub, body);
}

/// Render the live stat block below the paperdoll: level, unspent stat
/// points header, then one row per primary attribute with `+`/`-` buttons
/// (drawn via [`super::stat_button_rect`] so hit-tests stay in sync), then
/// the derived DEF/DMG summary.
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss, clippy::cast_precision_loss)]
fn draw_stat_sheet(
    batch: &mut SpriteBatch,
    world: &World,
    player: crate::ecs::Entity,
    viewport: [f32; 2],
    at: [f32; 2],
) {
    let Some(stats) = world.stats.get(&player) else { return };
    let a = &stats.inner.attributes;
    let r = &stats.inner.resolved;
    let level = world
        .progression
        .get(&player)
        .map_or(stats.inner.level, |p| p.level());
    let unspent =
        world.progression.get(&player).map_or(0, |p| p.inner.stat_points);

    let mut y = at[1];
    let label = |batch: &mut SpriteBatch, y: &mut f32, text: &[u8], value: u32, color: vertex::Color| {
        let mut s = text.to_vec();
        let (v, vn) = font::fmt_u32(value);
        s.extend_from_slice(&v[..vn]);
        font::draw_text(batch, &s, at[0], *y, 1.5, color, RenderLayer::UiScreen, 0, SUB_TEXT);
        *y += 18.0;
    };

    // Header: current level + unspent allocation pool.
    label(batch, &mut y, b"Niveau: ", level, [1.0, 0.95, 0.65, 1.0]);
    let points_color = if unspent > 0 {
        // Highlight when the player has points waiting.
        [0.55, 1.0, 0.55, 1.0]
    } else {
        [0.70, 0.70, 0.75, 1.0]
    };
    label(batch, &mut y, b"Points: ", unspent, points_color);

    // One row per primary attribute, with `+`/`-` buttons. The button
    // rectangles come from `super::stat_button_rect` so hit-testing matches
    // exactly what we paint.
    draw_attr_row(batch, viewport, at[0], &mut y, b"STR ", a.strength, PrimaryAttr::Strength, unspent, [0.95, 0.45, 0.40, 1.0]);
    draw_attr_row(batch, viewport, at[0], &mut y, b"DEX ", a.dexterity, PrimaryAttr::Dexterity, unspent, [0.50, 0.90, 0.50, 1.0]);
    draw_attr_row(batch, viewport, at[0], &mut y, b"CON ", a.vitality, PrimaryAttr::Vitality, unspent, [0.95, 0.65, 0.30, 1.0]);
    draw_attr_row(batch, viewport, at[0], &mut y, b"INT ", a.energy, PrimaryAttr::Energy, unspent, [0.45, 0.65, 1.0, 1.0]);

    // ── Derived combat stats, in two columns ────────────────────────────────
    y += 6.0;
    fill(batch, [at[0], y, 284.0, 1.0], SUB_CONTENT + 1, [0.42, 0.36, 0.26, 0.8]);
    y += 8.0;

    let col_l = at[0];
    let col_r = at[0] + 150.0;
    let row_h = 16.0;
    let put = |batch: &mut SpriteBatch, x: f32, yy: f32, s: &str, color: vertex::Color| {
        font::draw_text(batch, s.as_bytes(), x, yy, 1.3, color, RenderLayer::UiScreen, 0, SUB_TEXT);
    };

    const STEEL: vertex::Color = [0.75, 0.78, 0.82, 1.0];
    const SPEED: vertex::Color = [0.55, 0.88, 0.95, 1.0];
    const GREY: vertex::Color = [0.70, 0.70, 0.75, 1.0];

    // Damage range inflicted + defence.
    put(batch, col_l, y, &format!("DMG {}-{}", r.damage_min.max(0), r.damage_max.max(0)), [0.95, 0.90, 0.45, 1.0]);
    put(batch, col_r, y, &format!("DEF {}", r.defense.max(0)), STEEL);
    y += row_h;

    // Block chance + chance to be hit by the last foe (contests its Attack
    // Rating against our dodge); falls back to a generic AR when none is known.
    put(batch, col_l, y, &format!("Bloc {}%", r.block_chance.clamp(0, 100)), [0.55, 0.75, 0.95, 1.0]);
    let enemy_ar = world.last_enemy.as_ref().map_or(30.0, |le| le.attack_rating);
    let hit = ((1.0 - combat::dodge_chance_vs(world, player, enemy_ar)) * 100.0).round() as i32;
    put(batch, col_r, y, &format!("Touche {}%", hit.clamp(0, 100)), [0.95, 0.55, 0.50, 1.0]);
    y += row_h;

    // The three speeds (shown as the +%/-% modifier they apply).
    put(batch, col_l, y, &format!("Vit att {:+}%", r.attack_speed), SPEED);
    put(batch, col_r, y, &format!("Vit sort {:+}%", r.cast_speed), SPEED);
    y += row_h;
    put(batch, col_l, y, &format!("Vit dep {:+}%", r.movement_speed), SPEED);
    y += row_h + 4.0;

    // Resistances (one element per cell, two per row).
    put(batch, col_l, y, "Resistances", [0.85, 0.85, 0.90, 1.0]);
    y += row_h;
    put(batch, col_l, y, &format!("Feu {}%", r.fire_resist), [1.0, 0.45, 0.15, 1.0]);
    put(batch, col_r, y, &format!("Froid {}%", r.cold_resist), [0.45, 0.80, 1.0, 1.0]);
    y += row_h;
    put(batch, col_l, y, &format!("Foudre {}%", r.lightning_resist), [1.0, 0.92, 0.35, 1.0]);
    put(batch, col_r, y, &format!("Poison {}%", r.poison_resist), [0.45, 0.85, 0.35, 1.0]);
    y += row_h + 4.0;

    // Threat from the last enemy fought: their name/level, and the damage one of
    // their hits would deal after our defence (run through the real `mitigate`).
    match &world.last_enemy {
        Some(le) => {
            let name: String = super::mob_tooltip::ascii_safe(&le.name).chars().take(18).collect();
            put(batch, col_l, y, &format!("Ennemi: {name} (Niv {})", le.level), GREY);
            y += row_h;
            let avg = (le.damage_min + le.damage_max) * 0.5;
            let taken = combat::mitigate(avg, combat::effective_defense(world, player), le.attack_rating, 0.0, 0.0);
            put(batch, col_l, y, &format!("Degats recus/coup ~{}", taken.round().max(0.0) as i32), [0.95, 0.65, 0.30, 1.0]);
        }
        None => put(batch, col_l, y, "Aucun ennemi combattu", GREY),
    }
}

#[allow(clippy::too_many_arguments, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn draw_attr_row(
    batch: &mut SpriteBatch,
    viewport: [f32; 2],
    text_x: f32,
    y: &mut f32,
    label: &[u8],
    value: i32,
    attr: PrimaryAttr,
    unspent: u32,
    color: vertex::Color,
) {
    // "STR 20" (the attribute name and current value).
    let mut s = label.to_vec();
    let (v, vn) = font::fmt_u32(value.max(0) as u32);
    s.extend_from_slice(&v[..vn]);
    font::draw_text(batch, &s, text_x, *y, 1.5, color, RenderLayer::UiScreen, 0, SUB_TEXT);

    // [-] button: only meaningful when this attribute is above the 20-point
    // floor; greyed out otherwise.
    let minus_rect = stat_button_rect(viewport, attr, false);
    let can_refund = value > 20;
    paint_button(batch, minus_rect, b"-", can_refund);

    // [+] button: only meaningful when the player has unspent points.
    let plus_rect = stat_button_rect(viewport, attr, true);
    let can_spend = unspent > 0;
    paint_button(batch, plus_rect, b"+", can_spend);

    *y += 18.0;
}

fn paint_button(batch: &mut SpriteBatch, rect: Rect, glyph: &[u8], active: bool) {
    let (bg, fg) = if active {
        ([0.18, 0.18, 0.22, 1.0], [1.0, 1.0, 1.0, 1.0])
    } else {
        ([0.10, 0.10, 0.12, 1.0], [0.45, 0.45, 0.48, 1.0])
    };
    fill(batch, rect, SUB_CONTENT + 1, bg);
    border(batch, rect, 1.0, SUB_CONTENT + 2, [0.40, 0.36, 0.28, 1.0]);
    // Centre the glyph in the button (rough offsets for the 5x7 font).
    let tx = rect[0] + rect[2] * 0.5 - 2.0;
    let ty = rect[1] + rect[3] * 0.5 - 3.0;
    font::draw_text(batch, glyph, tx, ty, 1.5, fg, RenderLayer::UiScreen, 0, SUB_TEXT);
}
