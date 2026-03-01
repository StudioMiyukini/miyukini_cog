// @id: Sodomight-Client-Gui @do: gui-overlay @role: front-end @layer: 4 @human: miyuk
//! Game GUI overlay — draws HUD elements via the SpriteBatcher.
//!
//! All visual elements (health bar, mana bar, XP bar, skill bar, inventory
//! panel, combat log) are rendered as coloured quads on top of the isometric
//! scene. A single 4x4 white texture is used as the source; colour comes
//! from the per-vertex tint.
//!
//! The GUI struct holds **display state only** — no gameplay logic.

#![deny(unsafe_code)]

use mge_platform::{InputEvent, KeyCode, MouseButton};
use mge_render::{GpuTexture, SpriteBatcher, SpritePipeline};

// ---------------------------------------------------------------------------
// Layout constants
// ---------------------------------------------------------------------------

/// Health bar width in pixels.
const HP_BAR_W: f32 = 200.0;
/// Health / mana bar height in pixels.
const BAR_H: f32 = 20.0;
/// Horizontal margin from screen edges.
const MARGIN: f32 = 16.0;
/// Vertical offset from screen bottom for HP / mana bars.
const BAR_BOTTOM_OFFSET: f32 = 40.0;

/// XP bar height in pixels.
const XP_BAR_H: f32 = 6.0;

/// Skill slot size in pixels (square).
const SKILL_SLOT_SIZE: f32 = 40.0;
/// Gap between consecutive skill slots.
const SKILL_SLOT_GAP: f32 = 4.0;
/// Number of skill slots.
const SKILL_SLOT_COUNT: usize = 6;

/// Inventory slot size in pixels (square).
const INV_SLOT_SIZE: f32 = 32.0;
/// Gap between inventory slots.
const INV_SLOT_GAP: f32 = 2.0;
/// Inventory grid columns.
const INV_COLS: usize = 10;
/// Inventory grid rows.
const INV_ROWS: usize = 4;
/// Padding inside the inventory panel background.
const INV_PANEL_PAD: f32 = 8.0;

/// Maximum number of combat log messages retained.
const MAX_COMBAT_LOG: usize = 8;
/// Height of a single combat log entry in pixels.
const LOG_ENTRY_H: f32 = 18.0;
/// Width of the combat log area in pixels.
const LOG_W: f32 = 320.0;

/// Skill bar border thickness in pixels.
const SKILL_BORDER: f32 = 2.0;

// ---------------------------------------------------------------------------
// Colours (RGBA f32)
// ---------------------------------------------------------------------------

const COL_HP_BG: [f32; 4] = [0.3, 0.05, 0.05, 0.85];
const COL_HP_FG: [f32; 4] = [0.85, 0.12, 0.12, 1.0];
const COL_MANA_BG: [f32; 4] = [0.05, 0.05, 0.3, 0.85];
const COL_MANA_FG: [f32; 4] = [0.15, 0.25, 0.9, 1.0];
const COL_XP_BG: [f32; 4] = [0.12, 0.10, 0.05, 0.7];
const COL_XP_FG: [f32; 4] = [0.85, 0.75, 0.15, 1.0];
const COL_SLOT_BG: [f32; 4] = [0.10, 0.10, 0.12, 0.85];
const COL_SLOT_BORDER: [f32; 4] = [0.35, 0.35, 0.40, 0.9];
const COL_INV_PANEL_BG: [f32; 4] = [0.08, 0.08, 0.10, 0.92];
const COL_LOG_BG: [f32; 4] = [0.0, 0.0, 0.0, 0.45];

/// Full UV rect covering the entire white texture.
const UV_FULL: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

// ---------------------------------------------------------------------------
// GuiAction
// ---------------------------------------------------------------------------

/// Result of processing a single input event through the GUI layer.
///
/// The game loop inspects this to decide whether to forward the event to
/// the world simulation or to consume it as a GUI interaction.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum GuiAction {
    /// Input was not consumed by the GUI.
    #[default]
    None,
    /// Toggle the inventory panel.
    ToggleInventory,
    /// Toggle the skill panel.
    ToggleSkills,
    /// Activate skill in the given slot (0-based index).
    UseSkill(usize),
    /// Click on an inventory slot (0-based grid index).
    ClickInventorySlot(usize),
    /// Click passed through to the world (screen coordinates).
    ClickWorld(f32, f32),
}

// ---------------------------------------------------------------------------
// GuiItemSlot
// ---------------------------------------------------------------------------

/// Display data for a single inventory item.
#[derive(Debug, Clone, PartialEq)]
pub struct GuiItemSlot {
    /// Human-readable item name (for future tooltip use).
    pub item_name: String,
    /// RGBA border colour representing item quality tier.
    pub quality_color: [f32; 4],
    /// Number of items stacked in this slot.
    pub stack_count: u32,
}

// ---------------------------------------------------------------------------
// GameGui
// ---------------------------------------------------------------------------

/// Full-screen GUI overlay drawn on top of the isometric world.
///
/// Updated each frame from authoritative game state, then drawn via
/// [`Self::draw`] using the shared [`SpriteBatcher`].
pub struct GameGui {
    // -- viewport --
    screen_w: f32,
    screen_h: f32,

    // -- HUD values (refreshed every frame) --
    hp_current: i32,
    hp_max: i32,
    mana_current: i32,
    mana_max: i32,
    xp_current: i64,
    xp_next_level: i64,
    level: u8,
    gold: u32,

    // -- panel visibility --
    inventory_open: bool,
    skill_panel_open: bool,

    // -- inventory grid (INV_COLS * INV_ROWS) --
    inventory_slots: Vec<Option<GuiItemSlot>>,

    // -- skill bar (6 slots) --
    #[allow(dead_code)] // Reserved for skill name display in future iterations.
    skill_slots: [Option<String>; 6],

    // -- combat log --
    combat_log: Vec<String>,

    // -- tooltip (future use) --
    #[allow(dead_code)] // Reserved for hover tooltip rendering.
    tooltip: Option<String>,

    // -- cached mouse position for hit-testing --
    mouse_x: f32,
    mouse_y: f32,
}

impl GameGui {
    /// Create a new GUI overlay sized for the given viewport.
    #[must_use]
    pub fn new(screen_w: f32, screen_h: f32) -> Self {
        let total_slots = INV_COLS * INV_ROWS;
        Self {
            screen_w,
            screen_h,
            hp_current: 100,
            hp_max: 100,
            mana_current: 50,
            mana_max: 50,
            xp_current: 0,
            xp_next_level: 100,
            level: 1,
            gold: 0,
            inventory_open: false,
            skill_panel_open: false,
            inventory_slots: vec![Option::None; total_slots],
            skill_slots: [
                Option::None,
                Option::None,
                Option::None,
                Option::None,
                Option::None,
                Option::None,
            ],
            combat_log: Vec::new(),
            tooltip: Option::None,
            mouse_x: 0.0,
            mouse_y: 0.0,
        }
    }

    // -- State updates (called each frame by the game loop) ----------------

    /// Refresh HUD values from the authoritative game world.
    #[allow(clippy::too_many_arguments)]
    pub fn update_from_world(
        &mut self,
        hp: i32,
        hp_max: i32,
        mana: i32,
        mana_max: i32,
        xp: i64,
        xp_next: i64,
        level: u8,
        gold: u32,
    ) {
        self.hp_current = hp;
        self.hp_max = hp_max;
        self.mana_current = mana;
        self.mana_max = mana_max;
        self.xp_current = xp;
        self.xp_next_level = xp_next;
        self.level = level;
        self.gold = gold;
    }

    /// Replace the inventory grid contents.
    ///
    /// Each element is `None` (empty slot) or `Some((name, quality_rgba, stack))`.
    pub fn update_inventory(&mut self, slots: &[Option<(String, [f32; 4], u32)>]) {
        let total = INV_COLS * INV_ROWS;
        self.inventory_slots.clear();
        self.inventory_slots.reserve(total);

        for slot in slots.iter().take(total) {
            self.inventory_slots.push(slot.as_ref().map(|(name, col, stack)| {
                GuiItemSlot {
                    item_name: name.clone(),
                    quality_color: *col,
                    stack_count: *stack,
                }
            }));
        }

        // Pad remaining slots with None.
        while self.inventory_slots.len() < total {
            self.inventory_slots.push(Option::None);
        }
    }

    /// Toggle the inventory panel open/closed.
    pub fn toggle_inventory(&mut self) {
        self.inventory_open = !self.inventory_open;
    }

    /// Toggle the skill panel open/closed.
    pub fn toggle_skills(&mut self) {
        self.skill_panel_open = !self.skill_panel_open;
    }

    /// Append a combat log message, capping at [`MAX_COMBAT_LOG`].
    pub fn push_combat_message(&mut self, msg: String) {
        self.combat_log.push(msg);
        while self.combat_log.len() > MAX_COMBAT_LOG {
            self.combat_log.remove(0);
        }
    }

    /// Update the viewport dimensions (e.g. on window resize).
    pub fn set_screen_size(&mut self, w: f32, h: f32) {
        self.screen_w = w;
        self.screen_h = h;
    }

    /// Whether the inventory panel is currently visible.
    #[must_use]
    pub fn is_inventory_open(&self) -> bool {
        self.inventory_open
    }

    /// Whether the skill panel is currently visible.
    #[must_use]
    pub fn is_skill_panel_open(&self) -> bool {
        self.skill_panel_open
    }

    /// Read-only access to the combat log.
    #[must_use]
    pub fn combat_log(&self) -> &[String] {
        &self.combat_log
    }

    // -- Input handling ----------------------------------------------------

    /// Process an input event and return the resulting GUI action.
    ///
    /// If the GUI consumes the input (e.g. toggling a panel, clicking an
    /// inventory slot) it returns an action other than [`GuiAction::None`].
    /// Otherwise the caller should forward the event to the world.
    pub fn handle_input(&mut self, event: &InputEvent) -> GuiAction {
        match event {
            InputEvent::KeyDown { key } => self.handle_key_down(*key),
            InputEvent::MouseMove { x, y } => {
                self.mouse_x = *x as f32;
                self.mouse_y = *y as f32;
                GuiAction::None
            }
            InputEvent::MouseButtonEvent {
                button: MouseButton::Left,
                pressed: true,
            } => self.handle_left_click(),
            _ => GuiAction::None,
        }
    }

    /// Map key-down events to GUI actions.
    #[allow(clippy::unused_self)] // Will reference skill_slots in future iterations.
    fn handle_key_down(&self, key: KeyCode) -> GuiAction {
        match key {
            KeyCode::I => GuiAction::ToggleInventory,
            KeyCode::K => GuiAction::ToggleSkills,
            KeyCode::Num1 => GuiAction::UseSkill(0),
            KeyCode::Num2 => GuiAction::UseSkill(1),
            KeyCode::Num3 => GuiAction::UseSkill(2),
            KeyCode::Num4 => GuiAction::UseSkill(3),
            KeyCode::Num5 => GuiAction::UseSkill(4),
            KeyCode::Num6 => GuiAction::UseSkill(5),
            _ => GuiAction::None,
        }
    }

    /// Handle left mouse click: test inventory slots first, then fall through
    /// to a world click.
    fn handle_left_click(&self) -> GuiAction {
        // Test inventory panel hit if open.
        if self.inventory_open {
            if let Some(slot_idx) = self.hit_test_inventory(self.mouse_x, self.mouse_y) {
                return GuiAction::ClickInventorySlot(slot_idx);
            }
        }

        GuiAction::ClickWorld(self.mouse_x, self.mouse_y)
    }

    /// Return the inventory slot index under `(mx, my)`, if any.
    fn hit_test_inventory(&self, mx: f32, my: f32) -> Option<usize> {
        let (panel_x, panel_y) = self.inventory_panel_origin();

        let inner_x = panel_x + INV_PANEL_PAD;
        let inner_y = panel_y + INV_PANEL_PAD;
        let cell = INV_SLOT_SIZE + INV_SLOT_GAP;

        let grid_w = INV_COLS as f32 * cell;
        let grid_h = INV_ROWS as f32 * cell;

        // Quick bounds check.
        if mx < inner_x || mx > inner_x + grid_w || my < inner_y || my > inner_y + grid_h {
            return Option::None;
        }

        let col = ((mx - inner_x) / cell) as usize;
        let row = ((my - inner_y) / cell) as usize;

        if col < INV_COLS && row < INV_ROWS {
            Some(row * INV_COLS + col)
        } else {
            Option::None
        }
    }

    /// Top-left corner of the inventory panel in screen space.
    fn inventory_panel_origin(&self) -> (f32, f32) {
        let cell = INV_SLOT_SIZE + INV_SLOT_GAP;
        let panel_w = INV_COLS as f32 * cell + INV_PANEL_PAD * 2.0;
        let panel_h = INV_ROWS as f32 * cell + INV_PANEL_PAD * 2.0;
        let x = self.screen_w - panel_w - MARGIN;
        let y = (self.screen_h - panel_h) / 2.0;
        (x, y)
    }

    // -- Drawing -----------------------------------------------------------

    /// Draw the full GUI overlay into the batcher.
    ///
    /// The batcher should have been `begin()`-ed for this frame.
    /// `gui_texture` must be the small white texture created by
    /// [`create_white_texture`].
    ///
    /// After this call the batcher contains all GUI quads. The caller must
    /// `flush()` and `draw()` with the gui texture bound.
    pub fn draw(&self, batcher: &mut SpriteBatcher) {
        self.draw_xp_bar(batcher);
        self.draw_health_bar(batcher);
        self.draw_mana_bar(batcher);
        self.draw_skill_bar(batcher);
        self.draw_combat_log(batcher);

        if self.inventory_open {
            self.draw_inventory_panel(batcher);
        }
    }

    /// Draw the HP bar at the bottom-left of the screen.
    fn draw_health_bar(&self, batcher: &mut SpriteBatcher) {
        let x = MARGIN;
        let y = self.screen_h - BAR_BOTTOM_OFFSET;

        // Background.
        batcher.push(x, y, HP_BAR_W, BAR_H, UV_FULL, COL_HP_BG);

        // Foreground (proportional to HP%).
        let ratio = safe_ratio(self.hp_current, self.hp_max);
        let fill_w = HP_BAR_W * ratio;
        if fill_w > 0.0 {
            batcher.push(x, y, fill_w, BAR_H, UV_FULL, COL_HP_FG);
        }
    }

    /// Draw the mana bar at the bottom-right of the screen.
    fn draw_mana_bar(&self, batcher: &mut SpriteBatcher) {
        let x = self.screen_w - HP_BAR_W - MARGIN;
        let y = self.screen_h - BAR_BOTTOM_OFFSET;

        batcher.push(x, y, HP_BAR_W, BAR_H, UV_FULL, COL_MANA_BG);

        let ratio = safe_ratio(self.mana_current, self.mana_max);
        let fill_w = HP_BAR_W * ratio;
        if fill_w > 0.0 {
            batcher.push(x, y, fill_w, BAR_H, UV_FULL, COL_MANA_FG);
        }
    }

    /// Draw the XP bar across the full screen width at the very bottom.
    fn draw_xp_bar(&self, batcher: &mut SpriteBatcher) {
        let x = 0.0;
        let y = self.screen_h - XP_BAR_H;

        batcher.push(x, y, self.screen_w, XP_BAR_H, UV_FULL, COL_XP_BG);

        let ratio = safe_ratio_i64(self.xp_current, self.xp_next_level);
        let fill_w = self.screen_w * ratio;
        if fill_w > 0.0 {
            batcher.push(x, y, fill_w, XP_BAR_H, UV_FULL, COL_XP_FG);
        }
    }

    /// Draw the 6-slot skill bar centred at the bottom of the screen.
    fn draw_skill_bar(&self, batcher: &mut SpriteBatcher) {
        let total_w = SKILL_SLOT_COUNT as f32 * (SKILL_SLOT_SIZE + SKILL_SLOT_GAP) - SKILL_SLOT_GAP;
        let start_x = (self.screen_w - total_w) / 2.0;
        let y = self.screen_h - BAR_BOTTOM_OFFSET - SKILL_SLOT_SIZE - 8.0;

        for i in 0..SKILL_SLOT_COUNT {
            let sx = start_x + i as f32 * (SKILL_SLOT_SIZE + SKILL_SLOT_GAP);

            // Border quad.
            batcher.push(sx, y, SKILL_SLOT_SIZE, SKILL_SLOT_SIZE, UV_FULL, COL_SLOT_BORDER);

            // Inner background (inset by border thickness).
            let inner = SKILL_SLOT_SIZE - SKILL_BORDER * 2.0;
            batcher.push(
                sx + SKILL_BORDER,
                y + SKILL_BORDER,
                inner,
                inner,
                UV_FULL,
                COL_SLOT_BG,
            );
        }
    }

    /// Draw the inventory panel on the right side of the screen.
    fn draw_inventory_panel(&self, batcher: &mut SpriteBatcher) {
        let (panel_x, panel_y) = self.inventory_panel_origin();
        let cell = INV_SLOT_SIZE + INV_SLOT_GAP;
        let panel_w = INV_COLS as f32 * cell + INV_PANEL_PAD * 2.0;
        let panel_h = INV_ROWS as f32 * cell + INV_PANEL_PAD * 2.0;

        // Panel background.
        batcher.push(panel_x, panel_y, panel_w, panel_h, UV_FULL, COL_INV_PANEL_BG);

        let inner_x = panel_x + INV_PANEL_PAD;
        let inner_y = panel_y + INV_PANEL_PAD;

        for row in 0..INV_ROWS {
            for col in 0..INV_COLS {
                let sx = inner_x + col as f32 * cell;
                let sy = inner_y + row as f32 * cell;
                let idx = row * INV_COLS + col;

                // Slot border.
                batcher.push(sx, sy, INV_SLOT_SIZE, INV_SLOT_SIZE, UV_FULL, COL_SLOT_BORDER);

                // Slot interior.
                let inner = INV_SLOT_SIZE - 2.0;
                batcher.push(sx + 1.0, sy + 1.0, inner, inner, UV_FULL, COL_SLOT_BG);

                // Item contents (if any).
                if let Some(Some(item)) = self.inventory_slots.get(idx) {
                    let item_pad = 3.0;
                    let item_size = INV_SLOT_SIZE - item_pad * 2.0;
                    batcher.push(
                        sx + item_pad,
                        sy + item_pad,
                        item_size,
                        item_size,
                        UV_FULL,
                        item.quality_color,
                    );
                }
            }
        }
    }

    /// Draw the combat log in the top-left corner.
    fn draw_combat_log(&self, batcher: &mut SpriteBatcher) {
        if self.combat_log.is_empty() {
            return;
        }

        let x = MARGIN;
        let mut y = MARGIN;

        for _msg in &self.combat_log {
            batcher.push(x, y, LOG_W, LOG_ENTRY_H, UV_FULL, COL_LOG_BG);
            y += LOG_ENTRY_H + 2.0;
        }
    }
}

// ---------------------------------------------------------------------------
// White texture helper
// ---------------------------------------------------------------------------

/// Create a small 4x4 solid-white GPU texture for drawing coloured quads.
///
/// The resulting [`GpuTexture`] is used for all GUI elements — the visible
/// colour comes entirely from the per-vertex tint.
pub fn create_white_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    pipeline: &SpritePipeline,
) -> GpuTexture {
    let (w, h) = (4_u32, 4_u32);
    let pixel_count = (w * h) as usize;
    let mut data = Vec::with_capacity(pixel_count * 4);
    for _ in 0..pixel_count {
        data.extend_from_slice(&[255, 255, 255, 255]);
    }

    let img = image::RgbaImage::from_raw(w, h, data)
        .expect("white texture image creation must not fail (static size)");

    GpuTexture::from_image(device, queue, pipeline, &img, "gui_white_4x4")
}

// ---------------------------------------------------------------------------
// Utility
// ---------------------------------------------------------------------------

/// Compute `current / max` clamped to `[0.0, 1.0]`, returning 0 if `max <= 0`.
fn safe_ratio(current: i32, max: i32) -> f32 {
    if max <= 0 {
        return 0.0;
    }
    (current as f32 / max as f32).clamp(0.0, 1.0)
}

/// Same as [`safe_ratio`] but for `i64` values (XP).
fn safe_ratio_i64(current: i64, max: i64) -> f32 {
    if max <= 0 {
        return 0.0;
    }
    (current as f64 / max as f64).clamp(0.0, 1.0) as f32
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- Construction defaults --------------------------------------------

    #[test]
    fn new_default_values() {
        let gui = GameGui::new(800.0, 600.0);
        assert_eq!(gui.hp_current, 100);
        assert_eq!(gui.hp_max, 100);
        assert_eq!(gui.mana_current, 50);
        assert_eq!(gui.mana_max, 50);
        assert_eq!(gui.xp_current, 0);
        assert_eq!(gui.xp_next_level, 100);
        assert_eq!(gui.level, 1);
        assert_eq!(gui.gold, 0);
        assert!(!gui.inventory_open);
        assert!(!gui.skill_panel_open);
        assert_eq!(gui.inventory_slots.len(), INV_COLS * INV_ROWS);
        assert!(gui.combat_log.is_empty());
        assert!(gui.tooltip.is_none());
    }

    // -- update_from_world ------------------------------------------------

    #[test]
    fn update_from_world_sets_values() {
        let mut gui = GameGui::new(800.0, 600.0);
        gui.update_from_world(75, 200, 30, 80, 450, 1000, 5, 9999);
        assert_eq!(gui.hp_current, 75);
        assert_eq!(gui.hp_max, 200);
        assert_eq!(gui.mana_current, 30);
        assert_eq!(gui.mana_max, 80);
        assert_eq!(gui.xp_current, 450);
        assert_eq!(gui.xp_next_level, 1000);
        assert_eq!(gui.level, 5);
        assert_eq!(gui.gold, 9999);
    }

    // -- toggle_inventory -------------------------------------------------

    #[test]
    fn toggle_inventory_flips_flag() {
        let mut gui = GameGui::new(800.0, 600.0);
        assert!(!gui.is_inventory_open());
        gui.toggle_inventory();
        assert!(gui.is_inventory_open());
        gui.toggle_inventory();
        assert!(!gui.is_inventory_open());
    }

    // -- toggle_skills ----------------------------------------------------

    #[test]
    fn toggle_skills_flips_flag() {
        let mut gui = GameGui::new(800.0, 600.0);
        assert!(!gui.is_skill_panel_open());
        gui.toggle_skills();
        assert!(gui.is_skill_panel_open());
        gui.toggle_skills();
        assert!(!gui.is_skill_panel_open());
    }

    // -- push_combat_message ----------------------------------------------

    #[test]
    fn push_combat_message_caps_at_max() {
        let mut gui = GameGui::new(800.0, 600.0);
        for i in 0..12 {
            gui.push_combat_message(format!("msg {i}"));
        }
        assert_eq!(gui.combat_log().len(), MAX_COMBAT_LOG);
        // Oldest messages should have been removed.
        assert_eq!(gui.combat_log()[0], "msg 4");
        assert_eq!(gui.combat_log()[MAX_COMBAT_LOG - 1], "msg 11");
    }

    // -- handle_input: I key -> ToggleInventory ---------------------------

    #[test]
    fn handle_input_i_key_toggles_inventory() {
        let mut gui = GameGui::new(800.0, 600.0);
        let event = InputEvent::KeyDown { key: KeyCode::I };
        let action = gui.handle_input(&event);
        assert_eq!(action, GuiAction::ToggleInventory);
    }

    // -- handle_input: K key -> ToggleSkills -------------------------------

    #[test]
    fn handle_input_k_key_toggles_skills() {
        let mut gui = GameGui::new(800.0, 600.0);
        let event = InputEvent::KeyDown { key: KeyCode::K };
        let action = gui.handle_input(&event);
        assert_eq!(action, GuiAction::ToggleSkills);
    }

    // -- handle_input: Num1 -> UseSkill(0) --------------------------------

    #[test]
    fn handle_input_num1_uses_skill_0() {
        let mut gui = GameGui::new(800.0, 600.0);
        let event = InputEvent::KeyDown { key: KeyCode::Num1 };
        let action = gui.handle_input(&event);
        assert_eq!(action, GuiAction::UseSkill(0));
    }

    // -- handle_input: Num6 -> UseSkill(5) --------------------------------

    #[test]
    fn handle_input_num6_uses_skill_5() {
        let mut gui = GameGui::new(800.0, 600.0);
        let event = InputEvent::KeyDown { key: KeyCode::Num6 };
        let action = gui.handle_input(&event);
        assert_eq!(action, GuiAction::UseSkill(5));
    }

    // -- GuiAction default is None ----------------------------------------

    #[test]
    fn gui_action_default_is_none() {
        let action = GuiAction::default();
        assert_eq!(action, GuiAction::None);
    }

    // -- safe_ratio -------------------------------------------------------

    #[test]
    fn safe_ratio_normal() {
        let r = safe_ratio(50, 100);
        assert!((r - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn safe_ratio_zero_max_returns_zero() {
        assert!((safe_ratio(10, 0)).abs() < f32::EPSILON);
        assert!((safe_ratio(10, -5)).abs() < f32::EPSILON);
    }

    #[test]
    fn safe_ratio_clamps_over_max() {
        let r = safe_ratio(150, 100);
        assert!((r - 1.0).abs() < f32::EPSILON);
    }

    // -- safe_ratio_i64 ---------------------------------------------------

    #[test]
    fn safe_ratio_i64_normal() {
        let r = safe_ratio_i64(250, 1000);
        assert!((r - 0.25).abs() < 1e-5);
    }

    // -- update_inventory -------------------------------------------------

    #[test]
    fn update_inventory_replaces_slots() {
        let mut gui = GameGui::new(800.0, 600.0);

        let mut slots: Vec<Option<(String, [f32; 4], u32)>> = Vec::new();
        slots.push(Some(("Sword".to_string(), [1.0, 0.0, 0.0, 1.0], 1)));
        slots.push(Option::None);
        slots.push(Some(("Potion".to_string(), [0.0, 1.0, 0.0, 1.0], 5)));

        gui.update_inventory(&slots);

        assert_eq!(gui.inventory_slots.len(), INV_COLS * INV_ROWS);
        assert!(gui.inventory_slots[0].is_some());
        assert!(gui.inventory_slots[1].is_none());
        assert!(gui.inventory_slots[2].is_some());
        // Remaining slots padded with None.
        assert!(gui.inventory_slots[3].is_none());

        let sword = gui.inventory_slots[0].as_ref().expect("slot 0 should have item");
        assert_eq!(sword.item_name, "Sword");
        assert_eq!(sword.stack_count, 1);
    }

    // -- handle_input: left click on inventory slot -----------------------

    #[test]
    fn handle_input_click_inventory_slot() {
        let mut gui = GameGui::new(800.0, 600.0);
        gui.toggle_inventory();

        // Place mouse over first inventory slot.
        let (panel_x, panel_y) = gui.inventory_panel_origin();
        let slot_center_x = panel_x + INV_PANEL_PAD + INV_SLOT_SIZE / 2.0;
        let slot_center_y = panel_y + INV_PANEL_PAD + INV_SLOT_SIZE / 2.0;

        // Send mouse move first to update cached position.
        gui.handle_input(&InputEvent::MouseMove {
            x: slot_center_x as f64,
            y: slot_center_y as f64,
        });

        let click = InputEvent::MouseButtonEvent {
            button: MouseButton::Left,
            pressed: true,
        };
        let action = gui.handle_input(&click);
        assert_eq!(action, GuiAction::ClickInventorySlot(0));
    }

    // -- handle_input: left click passes through to world -----------------

    #[test]
    fn handle_input_click_world_when_inventory_closed() {
        let mut gui = GameGui::new(800.0, 600.0);
        assert!(!gui.is_inventory_open());

        gui.handle_input(&InputEvent::MouseMove { x: 400.0, y: 300.0 });

        let click = InputEvent::MouseButtonEvent {
            button: MouseButton::Left,
            pressed: true,
        };
        let action = gui.handle_input(&click);
        assert_eq!(action, GuiAction::ClickWorld(400.0, 300.0));
    }

    // -- set_screen_size --------------------------------------------------

    #[test]
    fn set_screen_size_updates_dimensions() {
        let mut gui = GameGui::new(800.0, 600.0);
        gui.set_screen_size(1920.0, 1080.0);
        assert!((gui.screen_w - 1920.0).abs() < f32::EPSILON);
        assert!((gui.screen_h - 1080.0).abs() < f32::EPSILON);
    }

    // -- unhandled key returns None ---------------------------------------

    #[test]
    fn handle_input_unrelated_key_returns_none() {
        let mut gui = GameGui::new(800.0, 600.0);
        let event = InputEvent::KeyDown { key: KeyCode::W };
        let action = gui.handle_input(&event);
        assert_eq!(action, GuiAction::None);
    }

    // -- combat log preserves order after overflow -----------------------

    #[test]
    fn combat_log_order_after_overflow() {
        let mut gui = GameGui::new(800.0, 600.0);
        for i in 0..20 {
            gui.push_combat_message(format!("event {i}"));
        }
        assert_eq!(gui.combat_log().len(), MAX_COMBAT_LOG);
        // Most recent should be last.
        assert_eq!(gui.combat_log()[MAX_COMBAT_LOG - 1], "event 19");
        // Oldest surviving should be event 12 (20 - 8).
        assert_eq!(gui.combat_log()[0], "event 12");
    }
}
