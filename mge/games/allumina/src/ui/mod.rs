//! In-game UI.
//!
//! Two layers live here:
//! * The original *text* renderers (`render_hud`, `render_inventory`, ...) used
//!   by the `--headless` driver and tests.
//! * The interactive *batch* UI (this module's [`UiState`] + the `draw`/input
//!   functions in each submodule) drawn as coloured quads and the bitmap font.
//!
//! egui was intentionally skipped: integrating `egui-wgpu` with the engine's
//! bespoke `GraphicsState` render pass is exactly the kind of GPU plumbing the
//! task said to simplify away, and sodomight proves the immediate-mode
//! quad+font approach yields a complete, working Diablo-style UI.

pub mod chat;
pub mod class_select;
pub mod equipment_ui;
pub mod hud;
pub mod inventory_ui;
#[cfg(feature = "netcode")]
pub mod lobby;
pub mod merc_ui;
pub mod minimap;
pub mod mob_tooltip;
pub mod forge;
pub mod quest_log;
pub mod skill_tree_ui;
pub mod title_screen;
pub mod tooltip;
pub mod vendor;
pub mod waypoint_ui;

use std::collections::VecDeque;

use mge_items::item::EquipSlot;
use mge_render::batch::{RenderLayer, SpriteBatch};

use crate::ecs::{Entity, World};
use crate::render::font;
use crate::render::sprite::push_quad;
use crate::render::vertex::{self, with_alpha, Color};

/// Equipment paperdoll slot order (10 slots).
pub const EQUIP_SLOTS: [EquipSlot; 10] = [
    EquipSlot::Head,
    EquipSlot::Amulet,
    EquipSlot::MainHand,
    EquipSlot::Chest,
    EquipSlot::OffHand,
    EquipSlot::Gloves,
    EquipSlot::Belt,
    EquipSlot::Boots,
    EquipSlot::Ring1,
    EquipSlot::Ring2,
];

/// Pixel size of one inventory/equipment cell.
pub const SLOT: f32 = 34.0;
pub const SLOT_PAD: f32 = 3.0;

// Draw-order within the UiScreen layer (all share y_sort 0).
pub const SUB_PANEL: u32 = 10;
pub const SUB_CONTENT: u32 = 20;
pub const SUB_TEXT: u32 = 30;
pub const SUB_TOOLTIP: u32 = 100;
pub const SUB_DRAG: u32 = 120;

/// A screen-space rectangle `[x, y, w, h]`.
pub type Rect = [f32; 4];

/// Mutable interactive-UI state, owned by the renderer.
#[derive(Debug, Clone)]
pub struct UiState {
    pub show_inventory: bool,
    pub show_character: bool,
    pub show_skills: bool,
    /// Whether the "Débloquer une branche" sub-menu is currently expanded on
    /// top of the skill panel. Only meaningful while `show_skills` is true.
    pub show_skills_unlock: bool,
    pub show_minimap: bool,
    pub show_menu: bool,
    pub show_quests: bool,
    /// Waypoint travel menu (toggled with W).
    pub show_waypoint: bool,
    /// The merchant the vendor window is open for, if any.
    pub vendor_merchant: Option<Entity>,
    /// The blacksmith the forge (salvage) window is open for, if any.
    pub forge_smith: Option<Entity>,
    /// Inventory slot currently being dragged, if any.
    pub drag: Option<usize>,
    /// Inventory slot under the cursor this frame.
    pub hover_inv: Option<usize>,
    /// Equipment slot under the cursor this frame.
    pub hover_equip: Option<EquipSlot>,
    /// The mercenary whose equipment panel is open, if any.
    pub merc_panel: Option<Entity>,
    /// Mercenary equipment slot under the cursor this frame.
    pub hover_merc_equip: Option<EquipSlot>,
    /// Rolling chat / event log.
    pub chat: VecDeque<String>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            show_inventory: false,
            show_character: false,
            show_skills: false,
            show_skills_unlock: false,
            show_minimap: true,
            show_menu: false,
            show_quests: false,
            show_waypoint: false,
            vendor_merchant: None,
            forge_smith: None,
            drag: None,
            hover_inv: None,
            hover_equip: None,
            merc_panel: None,
            hover_merc_equip: None,
            chat: VecDeque::new(),
        }
    }
}

const CHAT_MAX: usize = 60;

impl UiState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Append log lines from the sim into the chat ring buffer.
    pub fn push_log(&mut self, lines: Vec<String>) {
        for l in lines {
            self.chat.push_back(l);
            while self.chat.len() > CHAT_MAX {
                self.chat.pop_front();
            }
        }
    }

    /// Whether any blocking panel is open.
    pub fn any_panel_open(&self) -> bool {
        self.show_inventory
            || self.show_character
            || self.show_skills
            || self.show_quests
            || self.show_waypoint
            || self.vendor_merchant.is_some()
            || self.forge_smith.is_some()
            || self.merc_panel.is_some()
    }

    /// Close every open panel/menu. Returns whether anything was closed.
    pub fn close_all(&mut self) -> bool {
        let was = self.any_panel_open() || self.show_menu;
        self.show_inventory = false;
        self.show_character = false;
        self.show_skills = false;
        self.show_skills_unlock = false;
        self.show_quests = false;
        self.show_waypoint = false;
        self.show_menu = false;
        self.vendor_merchant = None;
        self.forge_smith = None;
        self.merc_panel = None;
        self.drag = None;
        was
    }
}

// ── Shared widget helpers ────────────────────────────────────────────────────

/// `true` if point `p` is inside rect `r`.
#[must_use]
pub fn point_in(r: Rect, p: [f32; 2]) -> bool {
    p[0] >= r[0] && p[1] >= r[1] && p[0] <= r[0] + r[2] && p[1] <= r[1] + r[3]
}

/// Fill a screen rect.
pub fn fill(batch: &mut SpriteBatch, r: Rect, sub: u32, color: Color) {
    push_quad(batch, [r[0], r[1]], [r[2], r[3]], RenderLayer::UiScreen, 0, sub, color);
}

/// Draw a 1-ish-px border around a rect (thickness `t`).
pub fn border(batch: &mut SpriteBatch, r: Rect, t: f32, sub: u32, color: Color) {
    fill(batch, [r[0], r[1], r[2], t], sub, color);
    fill(batch, [r[0], r[1] + r[3] - t, r[2], t], sub, color);
    fill(batch, [r[0], r[1], t, r[3]], sub, color);
    fill(batch, [r[0] + r[2] - t, r[1], t, r[3]], sub, color);
}

/// Draw a dashed border around a rect (used for empty equipment slots).
pub fn dotted_border(batch: &mut SpriteBatch, r: Rect, t: f32, sub: u32, color: Color) {
    const DASH: f32 = 3.0;
    const STEP: f32 = 6.0;
    let mut x = r[0];
    while x < r[0] + r[2] {
        let w = DASH.min(r[0] + r[2] - x);
        fill(batch, [x, r[1], w, t], sub, color);
        fill(batch, [x, r[1] + r[3] - t, w, t], sub, color);
        x += STEP;
    }
    let mut y = r[1];
    while y < r[1] + r[3] {
        let h = DASH.min(r[1] + r[3] - y);
        fill(batch, [r[0], y, t, h], sub, color);
        fill(batch, [r[0] + r[2] - t, y, t, h], sub, color);
        y += STEP;
    }
}

/// Draw a titled panel: a drop shadow, a vertically-graded dark body, a gold
/// double-border with corner studs, and (when named) a title bar with an accent
/// underline. Returns the inner content rect (unchanged from the flat version,
/// so every caller's layout still lines up).
#[allow(clippy::cast_precision_loss)]
pub fn panel(batch: &mut SpriteBatch, r: Rect, title: &[u8]) -> Rect {
    // Drop shadow, offset down-right (stable sort keeps it under the body).
    fill(batch, [r[0] + 5.0, r[1] + 6.0, r[2], r[3]], SUB_PANEL, [0.0, 0.0, 0.0, 0.42]);

    // Body: a top-lit vertical gradient built from a few horizontal bands.
    const BANDS: usize = 10;
    let top = [0.10, 0.10, 0.13, 0.93];
    let bot = [0.035, 0.035, 0.055, 0.93];
    let bh = r[3] / BANDS as f32;
    for i in 0..BANDS {
        let f = i as f32 / (BANDS as f32 - 1.0);
        fill(batch, [r[0], r[1] + bh * i as f32, r[2], bh + 0.5], SUB_PANEL, vertex::lerp(top, bot, f));
    }

    // Title bar strip + accent underline, then the title text.
    if !title.is_empty() {
        const TITLE_H: f32 = 26.0;
        fill(batch, [r[0], r[1], r[2], TITLE_H], SUB_PANEL + 1, [0.15, 0.12, 0.07, 0.95]);
        fill(batch, [r[0], r[1] + TITLE_H, r[2], 1.5], SUB_PANEL + 2, vertex::PANEL_BORDER);
        font::draw_text(batch, title, r[0] + 12.0, r[1] + 8.0, 2.0, vertex::XP, RenderLayer::UiScreen, 0, SUB_TEXT);
    }

    // Gold outer border + a thin dark inner liner (a beveled frame).
    border(batch, r, 2.0, SUB_PANEL + 2, vertex::PANEL_BORDER);
    border(batch, [r[0] + 2.0, r[1] + 2.0, r[2] - 4.0, r[3] - 4.0], 1.0, SUB_PANEL + 2, [0.28, 0.22, 0.11, 0.9]);

    // Corner studs.
    for &(sx, sy) in &[(r[0], r[1]), (r[0] + r[2], r[1]), (r[0], r[1] + r[3]), (r[0] + r[2], r[1] + r[3])] {
        crate::render::shapes::circle(batch, sx, sy, 3.5, RenderLayer::UiScreen, 0, SUB_PANEL + 3, vertex::PANEL_BORDER);
        crate::render::shapes::circle(batch, sx, sy, 1.6, RenderLayer::UiScreen, 0, SUB_PANEL + 3, [0.20, 0.16, 0.08, 1.0]);
    }

    [r[0] + 8.0, r[1] + 30.0, r[2] - 16.0, r[3] - 38.0]
}

/// Draw a vertically-filled orb (Diablo life/mana globe): a banded circle with a
/// glass-gradient background, a depth-shaded fluid fill that rises from the
/// bottom, an animated surface line, a specular glint, and a gold rim.
#[allow(clippy::cast_precision_loss, clippy::too_many_arguments)]
pub fn draw_orb(batch: &mut SpriteBatch, cx: f32, cy: f32, radius: f32, frac: f32, fill_color: Color, time: f32) {
    const BANDS: usize = 26;
    let frac = frac.clamp(0.0, 1.0);
    let wave = (time * 2.0).sin() * radius * 0.018;
    let fill_top = cy + radius - 2.0 * radius * frac + wave; // y above which it's empty
    let band_h = 2.0 * radius / BANDS as f32;
    let fluid_h = (2.0 * radius * frac).max(0.001);
    let bright = vertex::lerp(fill_color, vertex::WHITE, 0.32);
    let deep = vertex::lerp(fill_color, vertex::BLACK, 0.5);
    for i in 0..BANDS {
        let dy = -radius + band_h * (i as f32 + 0.5);
        let hw = (radius * radius - dy * dy).max(0.0).sqrt();
        if hw <= 0.5 {
            continue;
        }
        let y = cy + dy - band_h * 0.5;
        // Glass background, lighter at the top.
        let vfrac = i as f32 / (BANDS as f32 - 1.0);
        let bg = vertex::lerp([0.11, 0.11, 0.16, 0.95], [0.02, 0.02, 0.04, 0.95], vfrac);
        push_quad(batch, [cx - hw, y], [hw * 2.0, band_h + 0.5], RenderLayer::UiScreen, 0, SUB_CONTENT, bg);
        // Depth-shaded fluid fill.
        if cy + dy >= fill_top {
            let depth = (((cy + dy) - fill_top) / fluid_h).clamp(0.0, 1.0);
            let col = vertex::lerp(bright, deep, depth);
            push_quad(batch, [cx - hw, y], [hw * 2.0, band_h + 0.5], RenderLayer::UiScreen, 0, SUB_CONTENT + 1, col);
        }
    }
    // Animated surface highlight where the fluid meets air.
    if frac > 0.03 && frac < 0.985 {
        let surf_hw = (radius * radius - (fill_top - cy) * (fill_top - cy)).max(0.0).sqrt();
        if surf_hw > 1.0 {
            push_quad(batch, [cx - surf_hw, fill_top - 1.0], [surf_hw * 2.0, 2.5], RenderLayer::UiScreen, 0, SUB_CONTENT + 2, with_alpha(bright, 0.7));
        }
    }
    // Specular glint, upper-left.
    crate::render::shapes::ellipse(batch, cx - radius * 0.34, cy - radius * 0.40, radius * 0.26, radius * 0.16, RenderLayer::UiScreen, 0, SUB_CONTENT + 3, [1.0, 1.0, 1.0, 0.22]);
    // Gold rim.
    crate::render::shapes::ring(batch, cx, cy, radius, 3.0, RenderLayer::UiScreen, 0, SUB_CONTENT + 4, vertex::PANEL_BORDER);
    crate::render::shapes::ring(batch, cx, cy, radius - 3.0, 1.5, RenderLayer::UiScreen, 0, SUB_CONTENT + 4, [0.30, 0.24, 0.12, 1.0]);
}

// ── Layout: inventory grid ───────────────────────────────────────────────────

/// Full inventory panel rect.
#[must_use]
pub fn inventory_panel_rect(viewport: [f32; 2]) -> Rect {
    let pw = 10.0 * (SLOT + SLOT_PAD) + 16.0;
    let ph = 4.0 * (SLOT + SLOT_PAD) + 78.0;
    [viewport[0] - pw - 16.0, 90.0, pw, ph]
}

/// Rect of inventory slot `idx` (row-major 10x4).
#[must_use]
#[allow(clippy::cast_precision_loss)]
pub fn inventory_slot_rect(viewport: [f32; 2], idx: usize) -> Rect {
    let p = inventory_panel_rect(viewport);
    let col = (idx % 10) as f32;
    let row = (idx / 10) as f32;
    let x = p[0] + 8.0 + col * (SLOT + SLOT_PAD);
    let y = p[1] + 40.0 + row * (SLOT + SLOT_PAD);
    [x, y, SLOT, SLOT]
}

/// Inventory slot under `mouse`, if the panel is open.
#[must_use]
pub fn inventory_slot_at(viewport: [f32; 2], mouse: [f32; 2]) -> Option<usize> {
    for i in 0..40 {
        if point_in(inventory_slot_rect(viewport, i), mouse) {
            return Some(i);
        }
    }
    None
}

// ── Layout: equipment paperdoll ──────────────────────────────────────────────

#[must_use]
pub fn equipment_panel_rect(_viewport: [f32; 2]) -> Rect {
    // Tall enough for the paperdoll, the attribute allocator, and the full
    // derived-combat block (damage range, def, block, hit risk, resists, speeds).
    [24.0, 70.0, 320.0, 540.0]
}

/// Rect of a paperdoll slot.
#[must_use]
pub fn equipment_slot_rect(viewport: [f32; 2], slot: EquipSlot) -> Rect {
    let p = equipment_panel_rect(viewport);
    // Hand-placed 3-column doll. (col, row) per slot.
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
    let cell = SLOT + 14.0;
    let x = p[0] + 30.0 + col * (cell + 16.0);
    let y = p[1] + 40.0 + row * (cell + 12.0);
    [x, y, cell, cell]
}

#[must_use]
pub fn equipment_slot_at(viewport: [f32; 2], mouse: [f32; 2]) -> Option<EquipSlot> {
    EQUIP_SLOTS.into_iter().find(|&s| point_in(equipment_slot_rect(viewport, s), mouse))
}

// ── Layout: character-sheet stat rows + spend/refund buttons ─────────────────

/// Order of the four primary attributes as rendered on the character sheet
/// (matches the `equipment_ui::draw_stat_sheet` block).
pub const STAT_ROWS: [crate::systems::stats::PrimaryAttr; 4] = [
    crate::systems::stats::PrimaryAttr::Strength,
    crate::systems::stats::PrimaryAttr::Dexterity,
    crate::systems::stats::PrimaryAttr::Vitality,
    crate::systems::stats::PrimaryAttr::Energy,
];

/// Pixel size of a `+` / `-` button next to a stat row.
pub const STAT_BTN_SIZE: f32 = 14.0;

/// Y of the first stat row (relative to the equipment panel's origin). Matches
/// the offset `equipment_ui::draw` passes into `draw_stat_sheet` plus the
/// "Niveau" / "Points" header lines.
const STAT_ROW_FIRST_Y: f32 = 232.0 + 18.0 * 2.0;
/// Height between consecutive stat rows.
const STAT_ROW_PITCH: f32 = 18.0;
/// X of the `-` button (relative to the equipment panel's origin).
const STAT_MINUS_X: f32 = 110.0;
/// X of the `+` button (relative to the equipment panel's origin).
const STAT_PLUS_X: f32 = 128.0;

/// Rect for a stat-sheet `+`/`-` button.
#[must_use]
pub fn stat_button_rect(
    viewport: [f32; 2],
    attr: crate::systems::stats::PrimaryAttr,
    plus: bool,
) -> Rect {
    let p = equipment_panel_rect(viewport);
    let row = STAT_ROWS.iter().position(|a| *a == attr).unwrap_or(0) as f32;
    let x = p[0] + if plus { STAT_PLUS_X } else { STAT_MINUS_X };
    let y = p[1] + STAT_ROW_FIRST_Y + row * STAT_ROW_PITCH - 2.0;
    [x, y, STAT_BTN_SIZE, STAT_BTN_SIZE]
}

/// Which stat-sheet button (if any) is under `mouse`. The `bool` is `true`
/// for the `+` button, `false` for `-`.
#[must_use]
pub fn stat_button_at(
    viewport: [f32; 2],
    mouse: [f32; 2],
) -> Option<(crate::systems::stats::PrimaryAttr, bool)> {
    for attr in STAT_ROWS {
        if point_in(stat_button_rect(viewport, attr, true), mouse) {
            return Some((attr, true));
        }
        if point_in(stat_button_rect(viewport, attr, false), mouse) {
            return Some((attr, false));
        }
    }
    None
}

/// Short label for an equipment slot.
#[must_use]
pub fn slot_label(slot: EquipSlot) -> &'static [u8] {
    match slot {
        EquipSlot::Head => b"HEAD",
        EquipSlot::Amulet => b"AMUL",
        EquipSlot::MainHand => b"MAIN",
        EquipSlot::Chest => b"BODY",
        EquipSlot::OffHand => b"OFF",
        EquipSlot::Gloves => b"GLOV",
        EquipSlot::Belt => b"BELT",
        EquipSlot::Boots => b"BOOT",
        EquipSlot::Ring1 => b"RNG1",
        EquipSlot::Ring2 => b"RNG2",
    }
}

// ── Orchestration ────────────────────────────────────────────────────────────

/// Recompute what the cursor is hovering this frame.
pub fn update_hover(ui: &mut UiState, viewport: [f32; 2], mouse: [f32; 2]) {
    ui.hover_inv = if ui.show_inventory { inventory_slot_at(viewport, mouse) } else { None };
    ui.hover_equip = if ui.show_character { equipment_slot_at(viewport, mouse) } else { None };
    ui.hover_merc_equip = if ui.merc_panel.is_some() { merc_ui::merc_slot_at(viewport, mouse) } else { None };
}

/// Draw the full interactive UI for this frame.
pub fn draw(batch: &mut SpriteBatch, world: &World, ui: &mut UiState, viewport: [f32; 2], mouse: [f32; 2], time: f32) {
    update_hover(ui, viewport, mouse);

    hud::draw(batch, world, viewport, time);
    chat::draw(batch, ui, viewport);
    if ui.show_minimap {
        minimap::draw(batch, world, viewport);
    }
    if ui.show_inventory {
        inventory_ui::draw(batch, world, ui, viewport, time);
    }
    if ui.show_character {
        equipment_ui::draw(batch, world, ui, viewport, mouse, time);
    }
    if ui.show_skills {
        skill_tree_ui::draw(batch, world, &*ui, viewport, mouse);
    }
    if ui.show_quests {
        quest_log::draw(batch, world, viewport);
    }
    if ui.show_waypoint {
        waypoint_ui::draw(batch, world, viewport, mouse);
    }
    if let Some(merchant) = ui.vendor_merchant {
        vendor::draw(batch, world, merchant, viewport, mouse);
    }
    if ui.forge_smith.is_some() {
        forge::draw(batch, world, viewport, mouse);
    }
    if let Some(merc) = ui.merc_panel {
        // If the shown companion is gone, fall back to another in the party,
        // else close the panel.
        if !world.mercenary_tags.contains_key(&merc) {
            ui.merc_panel = world
                .player_entity
                .and_then(|p| crate::systems::mercenary::active_mercenary(world, p));
        }
        if ui.merc_panel.is_some() {
            merc_ui::draw(batch, world, ui, viewport, time);
        }
    }

    // Tooltip for the hovered item (skip while dragging).
    let mut item_tip_shown = false;
    if ui.drag.is_none() {
        let ctx = player_ctx(world);
        if let Some(idx) = ui.hover_inv {
            if let Some(item) = player_inventory_item(world, idx) {
                let worn = equipped_in_slot(world, &item);
                tooltip::draw(batch, &item, worn.as_ref(), ctx, viewport, mouse, time);
                item_tip_shown = true;
            }
        } else if let Some(slot) = ui.hover_equip {
            if let Some(item) = player_equipped_item(world, slot) {
                tooltip::draw(batch, &item, None, ctx, viewport, mouse, time);
                item_tip_shown = true;
            }
        } else if let (Some(merc), Some(slot)) = (ui.merc_panel, ui.hover_merc_equip) {
            if let Some(item) = world.merc_inventory.get(&merc).and_then(|inv| inv.get(slot)).cloned() {
                tooltip::draw(batch, &item, None, ctx, viewport, mouse, time);
                item_tip_shown = true;
            }
        }
    }
    // Mob tooltip: shown when no item tooltip wins and no panel is open. The
    // detection uses the input plugin's `cursor_world` (world-space pointer)
    // so it lines up with the actual rendered mob, not the screen origin.
    if !item_tip_shown && !ui.any_panel_open() && ui.drag.is_none() {
        if let Some(mob) = mob_tooltip::hovered_mob(world, world.input.cursor_world) {
            mob_tooltip::draw(batch, world, mob, viewport, mouse);
        }
    }

    // Drag ghost: the actual item icon floating at the cursor.
    if let Some(from) = ui.drag {
        if let Some(item) = player_inventory_item(world, from) {
            crate::render::item_icon::draw_item_icon(batch, item.item.base.kind, mouse[0], mouse[1], 32.0, item.rarity(), RenderLayer::UiScreen, 0, SUB_DRAG, time);
        }
    }

    if ui.show_menu {
        draw_menu(batch, viewport);
    }
}

fn draw_menu(batch: &mut SpriteBatch, viewport: [f32; 2]) {
    fill(batch, [0.0, 0.0, viewport[0], viewport[1]], SUB_TOOLTIP, [0.0, 0.0, 0.0, 0.6]);
    let w = 320.0;
    let h = 200.0;
    let r = [viewport[0] * 0.5 - w * 0.5, viewport[1] * 0.5 - h * 0.5, w, h];
    let inner = panel(batch, r, b"ALLUMINA");
    let lines: [&[u8]; 6] = [
        b"I  INVENTORY",
        b"C  CHARACTER",
        b"S  SKILLS",
        b"H  HIRE MERC (VILLAGE)",
        b"TAB  MINIMAP",
        b"ESC  RESUME",
    ];
    for (i, l) in lines.iter().enumerate() {
        #[allow(clippy::cast_precision_loss)]
        font::draw_text(batch, l, inner[0] + 16.0, inner[1] + 12.0 + i as f32 * 28.0, 2.0, vertex::WHITE, RenderLayer::UiScreen, 0, SUB_TEXT + 80);
    }
}

/// Helper: the player's inventory item at `idx`.
#[must_use]
pub fn player_inventory_item(world: &World, idx: usize) -> Option<crate::components::item::ItemInstance> {
    let p = world.player_entity?;
    world.inventory.get(&p).and_then(|inv| inv.get(idx).cloned())
}

/// Helper: the player's equipped item in `slot`.
#[must_use]
pub fn player_equipped_item(world: &World, slot: EquipSlot) -> Option<crate::components::item::ItemInstance> {
    let p = world.player_entity?;
    world.equipment.get(&p).and_then(|eq| eq.get(slot).cloned())
}

/// Helper: the player's current level (for tooltip requirement colouring).
#[must_use]
pub fn player_level(world: &World) -> u32 {
    world.player_entity.and_then(|p| world.progression.get(&p)).map_or(1, crate::components::player::Progression::level)
}

/// Helper: the player context (level + strength + dexterity) the tooltip needs
/// to colour unmet requirements and compute damage reduction.
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn player_ctx(world: &World) -> tooltip::PlayerCtx {
    let level = player_level(world);
    let (strength, dexterity, intelligence) = world
        .player_entity
        .and_then(|p| world.stats.get(&p))
        .map_or((0, 0, 0), |s| {
            // Read *effective* attributes (base + worn-gear Flat modifiers)
            // so a +5 STR amulet turns a red "Force requise" into grey on the
            // tooltip the moment it's worn.
            let eff = s.inner.effective_attributes();
            (
                eff.strength.max(0) as u32,
                eff.dexterity.max(0) as u32,
                eff.energy.max(0) as u32,
            )
        });
    tooltip::PlayerCtx { level, strength, dexterity, intelligence }
}

/// Helper: the item currently worn in the slot `item` would occupy, for the
/// tooltip comparison. Rings fall back from `Ring1` to `Ring2`.
#[must_use]
pub fn equipped_in_slot(world: &World, item: &crate::components::item::ItemInstance) -> Option<crate::components::item::ItemInstance> {
    let p = world.player_entity?;
    let eq = world.equipment.get(&p)?;
    let slot = item.slot()?;
    if let Some(worn) = eq.get(slot) {
        return Some(worn.clone());
    }
    if slot == EquipSlot::Ring1 {
        return eq.get(EquipSlot::Ring2).cloned();
    }
    None
}

// ── Input ────────────────────────────────────────────────────────────────────

/// Handle a left-button press. Returns `true` if the UI consumed it (so the
/// world should not also react to the click).
pub fn handle_left_press(world: &mut World, ui: &mut UiState, viewport: [f32; 2], mouse: [f32; 2]) -> bool {
    if ui.show_menu {
        return true;
    }
    // Waypoint menu: clicking a row teleports.
    if ui.show_waypoint {
        if let Some(wp_id) = waypoint_ui::row_at(world, viewport, mouse) {
            if let Some(player) = world.player_entity {
                crate::systems::waypoint::waypoint_teleport(world, player, wp_id);
                ui.show_waypoint = false;
            }
            return true;
        }
        if point_in(waypoint_ui::panel_rect(viewport), mouse) {
            return true;
        }
    }
    // Vendor: buy a stock row or sell an inventory row.
    if let Some(merchant) = ui.vendor_merchant {
        if let Some(player) = world.player_entity {
            if let Some(buy_idx) = vendor::buy_row_at(world, merchant, viewport, mouse) {
                crate::systems::vendor::buy(world, player, merchant, buy_idx);
                return true;
            }
            if let Some(sell_idx) = vendor::sell_row_at(world, player, viewport, mouse) {
                crate::systems::vendor::sell(world, player, sell_idx);
                return true;
            }
        }
        if point_in(vendor::panel_rect(viewport), mouse) {
            return true;
        }
    }
    // Forge: click an inventory row to recycle that item into materials.
    if ui.forge_smith.is_some() {
        if let Some(player) = world.player_entity {
            if let Some(idx) = forge::recycle_row_at(world, player, viewport, mouse) {
                crate::systems::forge::recycle(world, player, idx);
                return true;
            }
        }
        if point_in(forge::panel_rect(viewport), mouse) {
            return true;
        }
    }
    // Skill tree: the "+" button invests a point, the toggle opens the
    // unlock sub-menu, and unlock buttons in that sub-menu spend one point
    // to buy a foreign branch. Clicks elsewhere on either panel are
    // swallowed so they don't leak through to the game world.
    if ui.show_skills {
        if ui.show_skills_unlock {
            if let Some((class, idx)) = skill_tree_ui::unlock_at(world, viewport, mouse) {
                skill_tree_ui::unlock_branch(world, class, idx);
                return true;
            }
            if skill_tree_ui::in_unlock_panel(viewport, mouse) {
                return true;
            }
        }
        if skill_tree_ui::toggle_at(viewport, mouse) {
            ui.show_skills_unlock = !ui.show_skills_unlock;
            return true;
        }
        if let Some(node) = skill_tree_ui::plus_at(world, viewport, mouse) {
            skill_tree_ui::invest(world, node);
            return true;
        }
        // Click a learned skill's row to add/remove it from the hotbar.
        if let Some(sid) = skill_tree_ui::skill_row_at(world, viewport, mouse) {
            skill_tree_ui::toggle_on_hotbar(world, sid);
            return true;
        }
        if point_in(skill_tree_ui::panel_rect(viewport), mouse) {
            return true;
        }
    }
    // Inventory: begin dragging a filled slot.
    if ui.show_inventory {
        if let Some(idx) = inventory_slot_at(viewport, mouse) {
            if player_inventory_item(world, idx).is_some() {
                ui.drag = Some(idx);
            }
            return true;
        }
        if point_in(inventory_panel_rect(viewport), mouse) {
            return true;
        }
    }
    // Character/equipment: click a `+`/`-` to spend or refund a stat point,
    // or click a worn slot to unequip it. Buttons take precedence so they
    // win even if they overlap the paperdoll padding.
    if ui.show_character {
        if let Some((attr, plus)) = stat_button_at(viewport, mouse) {
            if let Some(p) = world.player_entity {
                if plus {
                    crate::systems::stats::spend_stat_point(world, p, attr);
                } else {
                    crate::systems::stats::refund_stat_point(world, p, attr);
                }
            }
            return true;
        }
        if let Some(slot) = equipment_slot_at(viewport, mouse) {
            if let Some(p) = world.player_entity {
                crate::plugins::inventory::unequip_to_inventory(world, p, slot);
            }
            return true;
        }
        if point_in(equipment_panel_rect(viewport), mouse) {
            return true;
        }
    }
    // Merc panel: switch companion via the selector tabs, or click a worn slot
    // to take the item back into the backpack.
    if let Some(merc) = ui.merc_panel {
        if let Some(p) = world.player_entity {
            let party = crate::systems::mercenary::companions(world, p);
            if let Some(idx) = merc_ui::tab_at(viewport, mouse, party.len()) {
                if let Some(&picked) = party.get(idx) {
                    ui.merc_panel = Some(picked);
                }
                return true;
            }
            if let Some(slot) = merc_ui::merc_slot_at(viewport, mouse) {
                crate::plugins::inventory::unequip_merc_slot(world, p, merc, slot);
                return true;
            }
        }
        if point_in(merc_ui::merc_panel_rect(viewport), mouse) {
            return true;
        }
    }
    false
}

/// Right-click in a panel transfers an item between containers: an inventory
/// item is equipped, a worn slot is unequipped to the backpack, and a merc-panel
/// slot returns the item to the player. Returns whether the click was consumed
/// (so the caller skips the world attack).
pub fn handle_right_click(world: &mut World, ui: &UiState, viewport: [f32; 2], mouse: [f32; 2]) -> bool {
    let Some(player) = world.player_entity else { return false };
    if ui.show_inventory {
        if let Some(idx) = inventory_slot_at(viewport, mouse) {
            if let Some(item) = player_inventory_item(world, idx) {
                if item.is_gem() {
                    // Right-click a gem → socket it into a piece of gear.
                    crate::plugins::inventory::socket_gem_from_inventory(world, player, idx);
                } else {
                    crate::plugins::inventory::equip_from_inventory(world, player, idx);
                }
            }
            return true;
        }
        if point_in(inventory_panel_rect(viewport), mouse) {
            return true;
        }
    }
    if ui.show_character {
        if let Some(slot) = equipment_slot_at(viewport, mouse) {
            crate::plugins::inventory::unequip_to_inventory(world, player, slot);
            return true;
        }
        if point_in(equipment_panel_rect(viewport), mouse) {
            return true;
        }
    }
    if let Some(merc) = ui.merc_panel {
        if let Some(slot) = merc_ui::merc_slot_at(viewport, mouse) {
            crate::plugins::inventory::unequip_merc_slot(world, player, merc, slot);
            return true;
        }
        if point_in(merc_ui::merc_panel_rect(viewport), mouse) {
            return true;
        }
    }
    false
}

/// Handle a left-button release (completes a drag).
pub fn handle_left_release(world: &mut World, ui: &mut UiState, viewport: [f32; 2], mouse: [f32; 2]) {
    let Some(from) = ui.drag.take() else { return };
    let Some(player) = world.player_entity else { return };
    let dragged_is_gem = player_inventory_item(world, from).is_some_and(|i| i.is_gem());
    // Dropped onto another inventory slot → socket a gem into it, else move/swap.
    if let Some(to) = inventory_slot_at(viewport, mouse) {
        if dragged_is_gem
            && crate::plugins::inventory::socket_gem_into_inventory_item(world, player, from, to)
        {
            return;
        }
        if let Some(inv) = world.inventory.get_mut(&player) {
            inv.move_slot(from, to);
        }
        return;
    }
    // Dropped onto the paperdoll → socket a gem into the worn item, else equip.
    if ui.show_character {
        if let Some(target) = equipment_slot_at(viewport, mouse) {
            if dragged_is_gem {
                crate::plugins::inventory::socket_gem_into_equipped(world, player, from, target);
                return;
            }
            let Some(item) = player_inventory_item(world, from) else { return };
            let kind = item.item.base.kind;
            // Dual-wield: a one-handed melee weapon dropped on the off-hand goes
            // there explicitly (its natural slot is MainHand).
            if target == EquipSlot::OffHand
                && crate::components::item::is_one_handed_melee(kind)
            {
                crate::plugins::inventory::equip_from_inventory_to(world, player, from, EquipSlot::OffHand);
            } else if matches!(target, EquipSlot::Ring1 | EquipSlot::Ring2)
                && item.slot() == Some(EquipSlot::Ring1)
            {
                // Drop a ring onto whichever finger the cursor is over.
                crate::plugins::inventory::equip_from_inventory_to(world, player, from, target);
            } else if item.slot().is_some_and(|s| slot_family(s) == slot_family(target)) {
                crate::plugins::inventory::equip_from_inventory(world, player, from);
            }
            return;
        }
    }

    // Dropped onto a slot of the open merc panel → equip to that merc.
    if let Some(merc) = ui.merc_panel {
        if merc_ui::merc_slot_at(viewport, mouse).is_some() {
            crate::plugins::inventory::equip_inventory_item_to_merc(world, player, from, merc);
            return;
        }
    }

    // Dropped onto a mercenary in the world → equip the item to that merc. Guard
    // against panels: only when the cursor isn't over an open panel.
    let over_panel = (ui.show_inventory && point_in(inventory_panel_rect(viewport), mouse))
        || (ui.show_character && point_in(equipment_panel_rect(viewport), mouse))
        || (ui.merc_panel.is_some() && point_in(merc_ui::merc_panel_rect(viewport), mouse));
    if !over_panel {
        if let Some(merc) = mercenary_near(world, world.input.cursor_world) {
            crate::plugins::inventory::equip_inventory_item_to_merc(world, player, from, merc);
        }
    }
}

/// The living mercenary under the world-space `cursor`, if any (drop target for
/// gifting gear). Mirrors `mob_tooltip::hovered_mob` with a slightly wider grab.
fn mercenary_near(world: &World, cursor: Option<crate::components::Position2D>) -> Option<Entity> {
    let cursor = cursor?;
    let mut best: Option<(Entity, f32)> = None;
    for e in world.mercenary_entities() {
        if world.merc_states.get(&e).is_some_and(|s| s.is_dead()) {
            continue;
        }
        let Some(&pos) = world.positions.get(&e) else { continue };
        let d2 = pos.distance_sq(cursor);
        if d2 > 0.8 * 0.8 {
            continue;
        }
        if best.map_or(true, |(_, bd)| d2 < bd) {
            best = Some((e, d2));
        }
    }
    best.map(|(e, _)| e)
}

/// Treat ring slots as interchangeable when matching drag targets.
fn slot_family(s: EquipSlot) -> EquipSlot {
    match s {
        EquipSlot::Ring1 | EquipSlot::Ring2 => EquipSlot::Ring1,
        other => other,
    }
}
