// @id: Sodomight-Client-Gui @do: gui-overlay @role: front-end @layer: 4 @human: miyuk
//! Game GUI overlay -- draws HUD elements via the SpriteBatcher.
//!
//! All visual elements (health orb, mana orb, XP bar, skill bar, inventory
//! panel, combat log, minimap, tooltips) are rendered as coloured quads on
//! top of the isometric scene. A single 4x4 white texture is used as the
//! source; colour comes from the per-vertex tint.
//!
//! The GUI struct holds **display state only** -- no gameplay logic.
//!
//! ## Style
//!
//! Inspired by classic ARPG "control panel" HUDs (Diablo 2, Path of Exile):
//! - Dark gothic palette with tarnished gold accents
//! - Health / Mana "orbs" simulated via stacked quad columns
//! - Bottom control panel spanning full screen width
//! - Minimap in top-right corner

#![deny(unsafe_code)]

use mge_platform::{InputEvent, KeyCode, MouseButton};
use mge_render::{GpuTexture, SpriteBatcher, SpritePipeline};

// ---------------------------------------------------------------------------
// Layout constants
// ---------------------------------------------------------------------------

/// Control panel height (bottom bar).
const PANEL_H: f32 = 120.0;
/// Orb diameter in pixels.
const ORB_DIAMETER: f32 = 90.0;
/// Orb horizontal inset from panel edges.
const ORB_INSET: f32 = 24.0;

/// XP bar height (sits above control panel).
const XP_BAR_H: f32 = 8.0;

/// Stamina bar height (between orbs, above belt).
const STAMINA_BAR_H: f32 = 6.0;
/// Stamina bar width (fills the area between orbs).
const STAMINA_BAR_MARGIN: f32 = 10.0;

/// Active skill selector size (L-click / R-click skill icons beside orbs).
const ACTIVE_SKILL_SIZE: f32 = 48.0;
/// Gap between orb and active skill selector.
const ACTIVE_SKILL_GAP: f32 = 6.0;

/// Skill slot size in pixels (square).
const SKILL_SLOT_SIZE: f32 = 40.0;
/// Gap between consecutive skill slots.
const SKILL_SLOT_GAP: f32 = 4.0;
/// Number of skill slots.
const SKILL_SLOT_COUNT: usize = 6;

/// Belt slot size (potions).
const BELT_SLOT_SIZE: f32 = 28.0;
/// Gap between belt slots.
const BELT_SLOT_GAP: f32 = 3.0;
/// Number of belt slots.
const BELT_SLOT_COUNT: usize = 4;

/// Equipment slot size in inventory.
const EQUIP_SLOT_SIZE: f32 = 40.0;
/// Equipment slot gap.
const EQUIP_SLOT_GAP: f32 = 4.0;

/// Character panel width.
const CHAR_PANEL_W: f32 = 280.0;
/// Character panel stat row height.
const CHAR_STAT_ROW_H: f32 = 28.0;

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
/// Inventory panel header height.
const INV_HEADER_H: f32 = 28.0;
/// Inventory panel footer height (gold display).
const INV_FOOTER_H: f32 = 24.0;

/// Maximum number of combat log messages retained.
const MAX_COMBAT_LOG: usize = 8;
/// Height of a single combat log entry in pixels.
const LOG_ENTRY_H: f32 = 18.0;
/// Width of the combat log area in pixels.
const LOG_W: f32 = 340.0;
/// Margin from screen edges.
const MARGIN: f32 = 16.0;

/// Skill bar border thickness in pixels.
const SKILL_BORDER: f32 = 2.0;

/// Minimap size (square).
const MINIMAP_SIZE: f32 = 140.0;
/// Minimap border thickness.
const MINIMAP_BORDER: f32 = 2.0;

/// Tooltip max width.
const TOOLTIP_MAX_W: f32 = 220.0;
/// Tooltip padding.
const TOOLTIP_PAD: f32 = 6.0;

/// Number of vertical slices used to approximate an orb circle.
const ORB_SLICES: usize = 30;

// ---------------------------------------------------------------------------
// Colours -- Gothic ARPG palette
// ---------------------------------------------------------------------------

/// Panel background -- deep dark violet-black.
const COL_PANEL_BG: [f32; 4] = [0.05, 0.04, 0.06, 0.95];
/// Panel border / accent -- tarnished gold.
const COL_BORDER_GOLD: [f32; 4] = [0.55, 0.45, 0.25, 0.9];
/// Active/bright gold accent.
const COL_BORDER_GOLD_BRIGHT: [f32; 4] = [0.75, 0.65, 0.35, 1.0];

/// Health orb background (dark red).
const COL_HP_BG: [f32; 4] = [0.18, 0.02, 0.02, 0.9];
/// Health orb foreground (blood red).
const COL_HP_FG: [f32; 4] = [0.7, 0.05, 0.05, 1.0];
/// Mana orb background (dark blue).
const COL_MANA_BG: [f32; 4] = [0.02, 0.03, 0.18, 0.9];
/// Mana orb foreground (deep blue).
const COL_MANA_FG: [f32; 4] = [0.1, 0.15, 0.65, 1.0];

/// XP bar background.
const COL_XP_BG: [f32; 4] = [0.08, 0.07, 0.04, 0.8];
/// XP bar foreground (gold).
const COL_XP_FG: [f32; 4] = [0.85, 0.75, 0.15, 1.0];

/// Skill slot background.
const COL_SLOT_BG: [f32; 4] = [0.08, 0.07, 0.10, 0.90];
/// Skill slot border.
const COL_SLOT_BORDER: [f32; 4] = [0.55, 0.45, 0.25, 0.9];

/// Inventory panel background.
const COL_INV_PANEL_BG: [f32; 4] = [0.05, 0.04, 0.06, 0.95];

/// Combat log background — darkened for text contrast.
const COL_LOG_BG: [f32; 4] = [0.02, 0.01, 0.03, 0.85];
/// Combat log border.
const COL_LOG_BORDER: [f32; 4] = [0.35, 0.30, 0.20, 0.6];

/// Minimap background.
const COL_MINIMAP_BG: [f32; 4] = [0.03, 0.03, 0.05, 0.85];
/// Minimap border.
const COL_MINIMAP_BORDER: [f32; 4] = [0.55, 0.45, 0.25, 0.9];
/// Minimap player dot (white).
const COL_MINIMAP_PLAYER: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
/// Minimap monster dot (red).
const COL_MINIMAP_MONSTER: [f32; 4] = [0.9, 0.15, 0.15, 0.9];

/// Tooltip background.
const COL_TOOLTIP_BG: [f32; 4] = [0.04, 0.03, 0.05, 0.95];
/// Tooltip border.
const COL_TOOLTIP_BORDER: [f32; 4] = [0.55, 0.45, 0.25, 0.85];

/// Belt slot background.
const COL_BELT_BG: [f32; 4] = [0.06, 0.05, 0.08, 0.9];

/// Stamina bar background.
const COL_STAMINA_BG: [f32; 4] = [0.08, 0.07, 0.04, 0.8];
/// Stamina bar foreground (yellowish).
const COL_STAMINA_FG: [f32; 4] = [0.75, 0.65, 0.15, 1.0];

/// Active skill selector background.
const COL_ACTIVE_SKILL_BG: [f32; 4] = [0.06, 0.05, 0.08, 0.95];
/// Left-click skill accent (warm red-orange).
const COL_LEFT_SKILL: [f32; 4] = [0.7, 0.35, 0.15, 0.9];
/// Right-click skill accent (cool blue-purple).
const COL_RIGHT_SKILL: [f32; 4] = [0.2, 0.3, 0.7, 0.9];

/// Automap background (semi-transparent).
const COL_AUTOMAP_BG: [f32; 4] = [0.0, 0.0, 0.0, 0.35];
/// Automap wall/tile colour.
const COL_AUTOMAP_TILE: [f32; 4] = [0.4, 0.35, 0.2, 0.6];
/// Automap player colour.
const COL_AUTOMAP_PLAYER: [f32; 4] = [1.0, 1.0, 1.0, 0.95];
/// Automap monster colour.
const COL_AUTOMAP_MONSTER: [f32; 4] = [0.9, 0.2, 0.2, 0.8];

/// Character panel background.
const COL_CHAR_PANEL_BG: [f32; 4] = [0.05, 0.04, 0.06, 0.95];
/// Stat bar foreground (varied per stat).
const COL_STAT_STR: [f32; 4] = [0.8, 0.3, 0.2, 0.9];
const COL_STAT_DEX: [f32; 4] = [0.3, 0.7, 0.3, 0.9];
const COL_STAT_VIT: [f32; 4] = [0.8, 0.6, 0.2, 0.9];
const COL_STAT_ENE: [f32; 4] = [0.3, 0.3, 0.8, 0.9];

/// Equipment slot highlight.
const COL_EQUIP_SLOT: [f32; 4] = [0.12, 0.10, 0.14, 0.9];

/// Stone panel texture simulation colours.
const COL_STONE_DARK: [f32; 4] = [0.07, 0.06, 0.08, 0.95];
const COL_STONE_MID: [f32; 4] = [0.10, 0.09, 0.11, 0.85];
const COL_STONE_LIGHT: [f32; 4] = [0.13, 0.12, 0.14, 0.75];
/// Panel inner bevel (subtle light edge).
const COL_BEVEL_LIGHT: [f32; 4] = [0.22, 0.20, 0.18, 0.4];
/// Panel inner bevel (shadow).
const COL_BEVEL_DARK: [f32; 4] = [0.02, 0.02, 0.03, 0.6];

/// Orb highlight (shine effect).
const COL_ORB_HIGHLIGHT: [f32; 4] = [1.0, 1.0, 1.0, 0.15];
/// Orb fill separator line.
const COL_ORB_FILL_LINE: [f32; 4] = [0.0, 0.0, 0.0, 0.35];

/// Run/Walk toggle colours.
const COL_RUN_ACTIVE: [f32; 4] = [0.7, 0.55, 0.15, 0.9];
const COL_WALK_ACTIVE: [f32; 4] = [0.4, 0.5, 0.4, 0.9];

/// Menu button background (stone-like).
const COL_MENU_BTN_BG: [f32; 4] = [0.09, 0.08, 0.10, 0.9];
/// Menu button hover/active accent.
const COL_MENU_BTN_ACTIVE: [f32; 4] = [0.18, 0.15, 0.12, 0.95];

/// Potion colours for belt fills.
const COL_POTION_HP: [f32; 4] = [0.65, 0.08, 0.08, 0.85];
const COL_POTION_MANA: [f32; 4] = [0.10, 0.12, 0.60, 0.85];
const COL_POTION_REJUV: [f32; 4] = [0.50, 0.10, 0.55, 0.85];
const COL_POTION_STAMINA: [f32; 4] = [0.55, 0.50, 0.15, 0.85];

/// XP bar segment divider colour.
const COL_XP_DIVIDER: [f32; 4] = [0.30, 0.25, 0.12, 0.7];

/// Monster health bar colours.
const COL_MONSTER_HP_BG: [f32; 4] = [0.10, 0.02, 0.02, 0.8];
const COL_MONSTER_HP_FG: [f32; 4] = [0.75, 0.12, 0.12, 0.95];
const COL_MONSTER_HP_BORDER: [f32; 4] = [0.40, 0.35, 0.20, 0.7];

/// Monster name tag background.
const COL_NAME_TAG_BG: [f32; 4] = [0.02, 0.02, 0.04, 0.7];

/// Quest panel background.
const COL_QUEST_PANEL_BG: [f32; 4] = [0.05, 0.04, 0.06, 0.95];

/// Orb frame decorative colours.
const COL_ORB_FRAME_OUTER: [f32; 4] = [0.30, 0.25, 0.15, 0.9];
const COL_ORB_FRAME_INNER: [f32; 4] = [0.45, 0.38, 0.22, 0.85];

/// Skill tree tab colours.
const COL_SKILL_TAB_BG: [f32; 4] = [0.08, 0.07, 0.10, 0.95];
const COL_SKILL_TAB_ACTIVE: [f32; 4] = [0.18, 0.14, 0.10, 0.95];
const COL_SKILL_TAB_BORDER: [f32; 4] = [0.55, 0.45, 0.25, 0.8];

/// Text colour: parchment (for general use).
const COL_TEXT_PARCHMENT: [f32; 4] = [0.82, 0.78, 0.68, 1.0];
/// Text colour: red for monster names.
const COL_TEXT_MONSTER: [f32; 4] = [0.9, 0.3, 0.3, 1.0];
/// Text colour: dim white for level/subtitle.
const COL_TEXT_DIM: [f32; 4] = [0.6, 0.58, 0.52, 0.9];

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
    /// Toggle the character stats panel (C key).
    ToggleCharacter,
    /// Toggle the automap overlay (Tab key).
    ToggleAutomap,
    /// Toggle Run/Walk mode (R key).
    ToggleRunWalk,
    /// Toggle the quest log panel (Q key).
    ToggleQuestLog,
    /// Activate skill in the given slot (0-based index).
    UseSkill(usize),
    /// Click on an inventory slot (0-based grid index).
    ClickInventorySlot(usize),
    /// Click passed through to the world (screen coordinates).
    ClickWorld(f32, f32),
    /// Use a belt potion slot (0-3, mapped from keys 1-4).
    UseBeltPotion(usize),
    /// Right-click on the game world (screen coordinates) — cast right-click skill.
    RightClickWorld(f32, f32),
    /// Allocate a stat point (0=str, 1=dex, 2=vit, 3=ene).
    AllocateStat(u8),
    /// Invest a skill point into a skill (slot index in current skill tree tab).
    InvestSkill(usize),
}

// ---------------------------------------------------------------------------
// GuiItemSlot
// ---------------------------------------------------------------------------

/// Display data for a single inventory item.
#[derive(Debug, Clone, PartialEq)]
pub struct GuiItemSlot {
    /// Human-readable item name (for tooltip display).
    pub item_name: String,
    /// RGBA border colour representing item quality tier.
    pub quality_color: [f32; 4],
    /// Number of items stacked in this slot.
    pub stack_count: u32,
}

// ---------------------------------------------------------------------------
// MinimapEntity
// ---------------------------------------------------------------------------

/// A point to render on the minimap (relative tile coordinates).
#[derive(Debug, Clone)]
pub struct MinimapEntity {
    /// Tile X position.
    pub x: f32,
    /// Tile Y position.
    pub y: f32,
}

// ---------------------------------------------------------------------------
// MonsterOverlay — per-monster UI data for health bars and name tags
// ---------------------------------------------------------------------------

/// Data needed to render a monster's health bar and name above its sprite.
///
/// Pushed by the game loop each frame for all visible, alive monsters.
#[derive(Debug, Clone)]
pub struct MonsterOverlay {
    /// Screen-space X position (centre of monster sprite).
    pub screen_x: f32,
    /// Screen-space Y position (top of monster sprite).
    pub screen_y: f32,
    /// Monster display name.
    pub name: String,
    /// Monster level.
    pub level: u8,
    /// Current HP ratio 0.0..1.0.
    pub hp_ratio: f32,
}

// ---------------------------------------------------------------------------
// GameGui
// ---------------------------------------------------------------------------

/// Full-screen GUI overlay drawn on top of the isometric world.
///
/// Updated each frame from authoritative game state, then drawn via
/// [`Self::draw`] using the shared [`SpriteBatcher`].
#[allow(clippy::struct_excessive_bools)]
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
    stamina_current: i32,
    stamina_max: i32,

    // -- D2-style active skills (left-click and right-click) --
    left_skill_name: String,
    right_skill_name: String,

    // -- panel visibility --
    inventory_open: bool,
    skill_panel_open: bool,
    character_open: bool,
    automap_open: bool,

    // -- run/walk mode --
    is_running: bool,

    // -- quest log --
    quest_log_open: bool,

    // -- skill tree tab (0=Fire, 1=Ice, 2=Lightning for Sorc-style) --
    skill_tree_tab: u8,

    // -- belt potion types (4 slots: 0=empty, 1=hp, 2=mana, 3=rejuv, 4=stamina) --
    belt_potions: [u8; 4],

    // -- character stats --
    stat_strength: i32,
    stat_dexterity: i32,
    stat_vitality: i32,
    stat_energy: i32,
    stat_points_available: i32,
    defense: i32,
    damage_min: i32,
    damage_max: i32,
    attack_rating: i32,

    /// Unspent skill points.
    skill_points_available: i32,

    // -- equipment slots (7 slots: helm, armor, weapon, shield, gloves, boots, belt) --
    equipment_slots: [Option<GuiItemSlot>; 7],

    // -- inventory grid (INV_COLS * INV_ROWS) --
    inventory_slots: Vec<Option<GuiItemSlot>>,

    // -- skill bar (6 slots) --
    #[allow(dead_code)] // Reserved for skill name display in future iterations.
    skill_slots: [Option<String>; 6],

    // -- combat log --
    combat_log: Vec<String>,

    // -- tooltip --
    tooltip: Option<String>,
    /// Slot index currently hovered (for tooltip positioning).
    tooltip_slot: Option<usize>,

    // -- minimap data --
    /// Player tile position for minimap.
    minimap_player: (f32, f32),
    /// Monster positions for minimap dots.
    minimap_monsters: Vec<MinimapEntity>,
    /// Minimap map dimensions (width, height in tiles).
    minimap_map_size: (f32, f32),
    /// Condensed tilemap for minimap rendering (one byte per tile: 0=void, 1=walkable, 2=wall, 3=water).
    minimap_tiles: Vec<u8>,

    // -- automap tile data (for the overlay) --
    /// Walkable tiles for the automap.
    automap_tiles: Vec<(f32, f32)>,

    // -- monster overlay data (health bars + names above sprites) --
    /// Monster overlays for the current frame (set by game loop).
    monster_overlays: Vec<MonsterOverlay>,

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
            stamina_current: 96,
            stamina_max: 96,
            left_skill_name: "Attack".to_string(),
            right_skill_name: "Fire Bolt".to_string(),
            inventory_open: false,
            skill_panel_open: false,
            character_open: false,
            automap_open: false,
            is_running: true,
            quest_log_open: false,
            skill_tree_tab: 0,
            belt_potions: [1, 1, 2, 0], // 2x HP, 1x Mana, 1x empty
            stat_strength: 30,
            stat_dexterity: 20,
            stat_vitality: 25,
            stat_energy: 15,
            stat_points_available: 0,
            defense: 12,
            damage_min: 2,
            damage_max: 6,
            attack_rating: 50,
            skill_points_available: 0,
            equipment_slots: [
                Option::None,
                Option::None,
                Option::None,
                Option::None,
                Option::None,
                Option::None,
                Option::None,
            ],
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
            tooltip_slot: Option::None,
            minimap_player: (16.0, 16.0),
            minimap_monsters: Vec::new(),
            minimap_map_size: (64.0, 64.0),
            minimap_tiles: Vec::new(),
            automap_tiles: Vec::new(),
            monster_overlays: Vec::new(),
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

    /// Update stamina display values.
    pub fn update_stamina(&mut self, current: i32, max: i32) {
        self.stamina_current = current;
        self.stamina_max = max;
    }

    /// Update character stats for the character panel.
    #[allow(clippy::too_many_arguments)]
    pub fn update_character_stats(
        &mut self,
        strength: i32,
        dexterity: i32,
        vitality: i32,
        energy: i32,
        stat_points: i32,
        defense: i32,
        damage_min: i32,
        damage_max: i32,
        attack_rating: i32,
    ) {
        self.stat_strength = strength;
        self.stat_dexterity = dexterity;
        self.stat_vitality = vitality;
        self.stat_energy = energy;
        self.stat_points_available = stat_points;
        self.defense = defense;
        self.damage_min = damage_min;
        self.damage_max = damage_max;
        self.attack_rating = attack_rating;
    }

    /// Set the active left-click and right-click skill names.
    pub fn set_active_skills(&mut self, left: &str, right: &str) {
        self.left_skill_name = left.to_string();
        self.right_skill_name = right.to_string();
    }

    /// Update automap tile data for the overlay.
    pub fn update_automap_tiles(&mut self, tiles: &[(f32, f32)]) {
        self.automap_tiles.clear();
        self.automap_tiles.reserve(tiles.len());
        for &(x, y) in tiles {
            self.automap_tiles.push((x, y));
        }
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

    /// Update minimap data with player and monster positions.
    pub fn update_minimap(&mut self, player: (f32, f32), monsters: &[(f32, f32)]) {
        self.minimap_player = player;
        self.minimap_monsters.clear();
        self.minimap_monsters.reserve(monsters.len());
        for &(x, y) in monsters {
            self.minimap_monsters.push(MinimapEntity { x, y });
        }
    }

    /// Set condensed tilemap data for minimap rendering.
    /// Each byte: 0=void, 1=walkable (floor/grass/path), 2=wall, 3=water.
    pub fn set_minimap_tiles(&mut self, width: i32, height: i32, tiles: Vec<u8>) {
        self.minimap_map_size = (width as f32, height as f32);
        self.minimap_tiles = tiles;
    }

    /// Toggle the inventory panel open/closed.
    pub fn toggle_inventory(&mut self) {
        self.inventory_open = !self.inventory_open;
    }

    /// Toggle the skill panel open/closed.
    pub fn toggle_skills(&mut self) {
        self.skill_panel_open = !self.skill_panel_open;
    }

    /// Toggle the character stats panel open/closed (D2 C key).
    pub fn toggle_character(&mut self) {
        self.character_open = !self.character_open;
    }

    /// Toggle the automap overlay (D2 Tab key).
    pub fn toggle_automap(&mut self) {
        self.automap_open = !self.automap_open;
    }

    /// Toggle Run/Walk mode (D2 R key).
    pub fn toggle_run_walk(&mut self) {
        self.is_running = !self.is_running;
    }

    /// Toggle the quest log panel (D2 Q key).
    pub fn toggle_quest_log(&mut self) {
        self.quest_log_open = !self.quest_log_open;
    }

    /// Set belt potion slot contents (0=empty, 1=hp, 2=mana, 3=rejuv, 4=stamina).
    pub fn set_belt_potions(&mut self, potions: [u8; 4]) {
        self.belt_potions = potions;
    }

    /// Update visible monster overlays for health bar / name rendering.
    pub fn set_monster_overlays(&mut self, overlays: Vec<MonsterOverlay>) {
        self.monster_overlays = overlays;
    }

    /// Update skill points available count.
    pub fn set_skill_points(&mut self, points: i32) {
        self.skill_points_available = points;
    }

    /// Set the active skill tree tab (0, 1, or 2).
    pub fn set_skill_tree_tab(&mut self, tab: u8) {
        self.skill_tree_tab = tab.min(2);
    }

    /// Whether the character panel is open.
    #[must_use]
    pub fn is_character_open(&self) -> bool {
        self.character_open
    }

    /// Whether the automap overlay is visible.
    #[must_use]
    pub fn is_automap_open(&self) -> bool {
        self.automap_open
    }

    /// Whether the quest log is open.
    #[must_use]
    pub fn is_quest_log_open(&self) -> bool {
        self.quest_log_open
    }

    /// Whether the player is running (vs walking).
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.is_running
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

    /// Current viewport width.
    #[must_use]
    pub fn screen_w(&self) -> f32 {
        self.screen_w
    }

    /// Current viewport height.
    #[must_use]
    pub fn screen_h(&self) -> f32 {
        self.screen_h
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

    /// Return the current and maximum HP for display purposes.
    #[must_use]
    pub fn hp_display(&self) -> (i32, i32) {
        (self.hp_current, self.hp_max)
    }

    /// Return the current and maximum mana for display purposes.
    #[must_use]
    pub fn mana_display(&self) -> (i32, i32) {
        (self.mana_current, self.mana_max)
    }

    /// Return the current player level for display.
    #[must_use]
    pub fn level_display(&self) -> u8 {
        self.level
    }

    /// Return the current gold count for display.
    #[must_use]
    pub fn gold_display(&self) -> u32 {
        self.gold
    }

    /// Return the current tooltip text, if any.
    #[must_use]
    pub fn tooltip_text(&self) -> Option<&str> {
        self.tooltip.as_deref()
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
                self.update_tooltip();
                GuiAction::None
            }
            InputEvent::MouseButtonEvent {
                button: MouseButton::Left,
                pressed: true,
            } => self.handle_left_click(),
            InputEvent::MouseButtonEvent {
                button: MouseButton::Right,
                pressed: true,
            } => self.handle_right_click(),
            _ => GuiAction::None,
        }
    }

    /// Map key-down events to GUI actions.
    #[allow(clippy::unused_self)] // Will reference skill_slots in future iterations.
    fn handle_key_down(&self, key: KeyCode) -> GuiAction {
        match key {
            KeyCode::I => GuiAction::ToggleInventory,
            KeyCode::K => GuiAction::ToggleSkills,
            KeyCode::C => GuiAction::ToggleCharacter,
            KeyCode::Tab => GuiAction::ToggleAutomap,
            KeyCode::R => GuiAction::ToggleRunWalk,
            KeyCode::Q => GuiAction::ToggleQuestLog,
            // D2-style: 1-4 = belt potions, F1-F6 = skill hotkeys.
            KeyCode::Num1 => GuiAction::UseBeltPotion(0),
            KeyCode::Num2 => GuiAction::UseBeltPotion(1),
            KeyCode::Num3 => GuiAction::UseBeltPotion(2),
            KeyCode::Num4 => GuiAction::UseBeltPotion(3),
            KeyCode::F1 => GuiAction::UseSkill(0),
            KeyCode::F2 => GuiAction::UseSkill(1),
            KeyCode::F3 => GuiAction::UseSkill(2),
            KeyCode::F4 => GuiAction::UseSkill(3),
            KeyCode::F5 => GuiAction::UseSkill(4),
            KeyCode::F6 => GuiAction::UseSkill(5),
            _ => GuiAction::None,
        }
    }

    /// Handle left mouse click: test panels first, then fall through to world.
    fn handle_left_click(&self) -> GuiAction {
        // Test if click is on the bottom control panel -- consume but ignore.
        let panel_y = self.screen_h - PANEL_H - XP_BAR_H;
        if self.mouse_y >= panel_y {
            // Click on control panel area -- do not pass to world.
            return GuiAction::None;
        }

        // Test character panel stat "+" buttons if open.
        if self.character_open && self.stat_points_available > 0 {
            if let Some(stat_idx) = self.hit_test_stat_buttons(self.mouse_x, self.mouse_y) {
                return GuiAction::AllocateStat(stat_idx);
            }
        }

        // Test inventory panel hit if open.
        if self.inventory_open {
            if let Some(slot_idx) = self.hit_test_inventory(self.mouse_x, self.mouse_y) {
                return GuiAction::ClickInventorySlot(slot_idx);
            }
            // Check if click is within inventory panel bounds (consume click).
            let (px, py, pw, ph) = self.inventory_panel_bounds();
            if self.mouse_x >= px
                && self.mouse_x <= px + pw
                && self.mouse_y >= py
                && self.mouse_y <= py + ph
            {
                return GuiAction::None;
            }
        }

        GuiAction::ClickWorld(self.mouse_x, self.mouse_y)
    }

    /// Handle right mouse click: cast right-click skill at world position.
    fn handle_right_click(&self) -> GuiAction {
        // Ignore clicks on the control panel.
        let panel_y = self.screen_h - PANEL_H - XP_BAR_H;
        if self.mouse_y >= panel_y {
            return GuiAction::None;
        }

        // Test character panel stat "+" buttons if open.
        if self.character_open && self.stat_points_available > 0 {
            if let Some(stat_idx) = self.hit_test_stat_buttons(self.mouse_x, self.mouse_y) {
                return GuiAction::AllocateStat(stat_idx);
            }
        }

        // Test skill panel skill slots if open.
        if self.skill_panel_open {
            if let Some(skill_idx) = self.hit_test_skill_panel(self.mouse_x, self.mouse_y) {
                return GuiAction::InvestSkill(skill_idx);
            }
        }

        GuiAction::RightClickWorld(self.mouse_x, self.mouse_y)
    }

    /// Hit-test the stat allocation "+" buttons on the character panel.
    /// Returns the stat index (0=str, 1=dex, 2=vit, 3=ene) if a button is hit.
    fn hit_test_stat_buttons(&self, mx: f32, my: f32) -> Option<u8> {
        let panel_h = 320.0_f32;
        let panel_x = MARGIN;
        let panel_y = (self.screen_h - PANEL_H - XP_BAR_H - panel_h) / 2.0;
        let stat_start_y = panel_y + INV_HEADER_H + 8.0;
        let stat_bar_w = 160.0_f32;
        let stat_x = panel_x + 90.0;
        let btn_size = 16.0_f32;
        let btn_x = stat_x + stat_bar_w + 6.0;

        for i in 0..4_u8 {
            let row_y = stat_start_y + f32::from(i) * CHAR_STAT_ROW_H;
            if mx >= btn_x && mx <= btn_x + btn_size && my >= row_y && my <= row_y + btn_size {
                return Some(i);
            }
        }
        None
    }

    /// Hit-test skill slots in the skill panel for skill point investment.
    /// Returns the skill index in the current tab.
    fn hit_test_skill_panel(&self, mx: f32, my: f32) -> Option<usize> {
        // Skill panel is drawn on the right side.
        let panel_w = 280.0_f32;
        let panel_h = 360.0_f32;
        let panel_x = self.screen_w - panel_w - MARGIN;
        let panel_y = (self.screen_h - PANEL_H - XP_BAR_H - panel_h) / 2.0;

        let grid_start_y = panel_y + INV_HEADER_H + 40.0; // After tabs.
        let slot_size = 48.0_f32;
        let gap = 8.0_f32;
        let cols = 4_usize;
        let grid_x = panel_x + 16.0;

        for i in 0..12_usize {
            let col = i % cols;
            let row = i / cols;
            let sx = grid_x + col as f32 * (slot_size + gap);
            let sy = grid_start_y + row as f32 * (slot_size + gap);
            if mx >= sx && mx <= sx + slot_size && my >= sy && my <= sy + slot_size {
                return Some(i);
            }
        }
        None
    }

    /// Return the inventory slot index under `(mx, my)`, if any.
    fn hit_test_inventory(&self, mx: f32, my: f32) -> Option<usize> {
        let (panel_x, panel_y, _, _) = self.inventory_panel_bounds();
        let equip_area_h = EQUIP_SLOT_SIZE * 2.0 + EQUIP_SLOT_GAP + 8.0 + 4.0 + 2.0;
        let grid_origin_y = panel_y + INV_HEADER_H + equip_area_h;

        let inner_x = panel_x + INV_PANEL_PAD;
        let inner_y = grid_origin_y + INV_PANEL_PAD;
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

    /// Bounding box of the inventory panel: (x, y, w, h).
    fn inventory_panel_bounds(&self) -> (f32, f32, f32, f32) {
        let cell = INV_SLOT_SIZE + INV_SLOT_GAP;
        let panel_w = INV_COLS as f32 * cell + INV_PANEL_PAD * 2.0;
        let equip_area_h = EQUIP_SLOT_SIZE * 2.0 + EQUIP_SLOT_GAP + 8.0 + 4.0 + 2.0;
        let grid_h = INV_ROWS as f32 * cell + INV_PANEL_PAD * 2.0;
        let panel_h = INV_HEADER_H + equip_area_h + grid_h + INV_FOOTER_H;
        let x = self.screen_w - panel_w - MARGIN;
        let y = (self.screen_h - PANEL_H - XP_BAR_H - panel_h) / 2.0;
        (x, y, panel_w, panel_h)
    }

    /// Top-left corner of the inventory panel in screen space.
    /// Used by tests for hit-testing validation.
    #[cfg(test)]
    fn inventory_panel_origin(&self) -> (f32, f32) {
        let (x, y, _, _) = self.inventory_panel_bounds();
        (x, y)
    }

    /// Update tooltip based on current mouse position over inventory.
    fn update_tooltip(&mut self) {
        if !self.inventory_open {
            self.tooltip = Option::None;
            self.tooltip_slot = Option::None;
            return;
        }

        if let Some(slot_idx) = self.hit_test_inventory(self.mouse_x, self.mouse_y) {
            if let Some(Some(item)) = self.inventory_slots.get(slot_idx) {
                self.tooltip = Some(item.item_name.clone());
                self.tooltip_slot = Some(slot_idx);
            } else {
                self.tooltip = Option::None;
                self.tooltip_slot = Option::None;
            }
        } else {
            self.tooltip = Option::None;
            self.tooltip_slot = Option::None;
        }
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
        // Automap overlay (drawn behind panels but on top of world).
        if self.automap_open {
            self.draw_automap(batcher);
        }

        // Bottom control panel (drawn first as background).
        self.draw_control_panel(batcher);
        self.draw_xp_bar(batcher);
        self.draw_hp_orb(batcher);
        self.draw_mana_orb(batcher);
        self.draw_active_skills(batcher);
        self.draw_stamina_bar(batcher);
        self.draw_skill_bar(batcher);
        self.draw_belt(batcher);
        self.draw_run_walk_toggle(batcher);
        self.draw_combat_log(batcher);
        self.draw_minimap(batcher);

        if self.inventory_open {
            self.draw_inventory_panel(batcher);
        }

        if self.character_open {
            self.draw_character_panel(batcher);
        }

        if self.skill_panel_open {
            self.draw_skill_panel(batcher);
        }

        if self.quest_log_open {
            self.draw_quest_log(batcher);
        }

        // Monster health bars and name tags (above sprites, below tooltip).
        self.draw_monster_overlays(batcher);

        // Tooltip (drawn last, on top of everything).
        if self.tooltip.is_some() {
            self.draw_tooltip(batcher);
        }
    }

    /// Draw all HUD text labels using the bitmap font.
    ///
    /// Called from a separate render pass in `game.rs` with the bitmap font
    /// texture bound instead of the white GUI texture.
    pub fn draw_all_text(&self, batcher: &mut SpriteBatcher) {
        // Always-visible text.
        self.draw_minimap_text(batcher);
        self.draw_active_skill_text(batcher);
        self.draw_run_walk_text(batcher);
        self.draw_menu_button_text(batcher);
        self.draw_combat_log_text(batcher);

        // Conditional panels.
        if self.automap_open {
            self.draw_automap_label(batcher);
        }
        if self.inventory_open {
            self.draw_inventory_text(batcher);
        }
        if self.character_open {
            self.draw_character_panel_text(batcher);
        }
        if self.skill_panel_open {
            self.draw_skill_panel_text(batcher);
        }
        if self.quest_log_open {
            self.draw_quest_log_text(batcher);
        }
        self.draw_monster_overlay_text(batcher);
        if self.tooltip.is_some() {
            self.draw_tooltip_text(batcher);
        }
    }

    /// Render combat log message text.
    pub fn draw_combat_log_text(&self, batcher: &mut SpriteBatcher) {
        use crate::bitmap_font::BitmapFont;

        if self.combat_log.is_empty() {
            return;
        }

        let x = MARGIN;
        let y = MARGIN;
        let scale = 1.8_f32;
        let mut entry_y = y + 4.0;

        for msg in &self.combat_log {
            let text_y = entry_y + (LOG_ENTRY_H - BitmapFont::line_height(scale)) / 2.0;
            BitmapFont::push_text(batcher, x + 6.0, text_y, msg, COL_TEXT_PARCHMENT, scale);
            entry_y += LOG_ENTRY_H + 2.0;
        }
    }

    // -- Control Panel (bottom bar) ----------------------------------------

    /// Draw the bottom control panel with D2-style stone texture simulation.
    fn draw_control_panel(&self, batcher: &mut SpriteBatcher) {
        let panel_y = self.screen_h - PANEL_H - XP_BAR_H;
        let panel_total_h = PANEL_H + XP_BAR_H;

        // Main panel background.
        batcher.push(0.0, panel_y + 2.0, self.screen_w, panel_total_h - 2.0, UV_FULL, COL_PANEL_BG);

        // Stone texture: horizontal bands with varying darkness.
        let band_h = 8.0_f32;
        let band_count = ((panel_total_h - 2.0) / band_h) as usize;
        for i in 0..band_count {
            let by = panel_y + 2.0 + i as f32 * band_h;
            let color = if i % 3 == 0 {
                COL_STONE_DARK
            } else if i % 3 == 1 {
                COL_STONE_MID
            } else {
                COL_STONE_LIGHT
            };
            batcher.push(0.0, by, self.screen_w, band_h, UV_FULL, color);
        }

        // Re-draw base to blend on top (semi-transparent).
        batcher.push(0.0, panel_y + 2.0, self.screen_w, panel_total_h - 2.0, UV_FULL, COL_PANEL_BG);

        // Top bevel (light edge for 3D feel).
        batcher.push(0.0, panel_y + 2.0, self.screen_w, 1.0, UV_FULL, COL_BEVEL_LIGHT);
        // Bottom shadow.
        batcher.push(0.0, self.screen_h - 1.0, self.screen_w, 1.0, UV_FULL, COL_BEVEL_DARK);

        // Top border line (gold).
        batcher.push(0.0, panel_y, self.screen_w, 2.0, UV_FULL, COL_BORDER_GOLD);

        // Decorative vertical dividers beside the orbs (D2-style panel sections).
        let left_orb_right = ORB_INSET + ORB_DIAMETER + ACTIVE_SKILL_GAP + ACTIVE_SKILL_SIZE + 8.0;
        let right_orb_left = self.screen_w - ORB_INSET - ORB_DIAMETER - ACTIVE_SKILL_GAP - ACTIVE_SKILL_SIZE - 8.0;

        batcher.push(left_orb_right, panel_y + 4.0, 2.0, panel_total_h - 8.0, UV_FULL, COL_BORDER_GOLD);
        batcher.push(right_orb_left, panel_y + 4.0, 2.0, panel_total_h - 8.0, UV_FULL, COL_BORDER_GOLD);

        // D2-style menu buttons (4 small stone buttons between dividers and orbs).
        // Left side: 2 buttons (Character, Inventory) below left orb section.
        // Right side: 2 buttons (Skill Tree, Quest Log) below right orb section.
        self.draw_menu_buttons(batcher, panel_y, left_orb_right, right_orb_left);
    }

    /// Draw the 4 D2-style menu buttons in the control panel.
    ///
    /// In D2, these appear as small stone buttons with icons for accessing
    /// Character (C), Inventory (I), Skill Tree (K), and Quest Log (Q).
    fn draw_menu_buttons(
        &self,
        batcher: &mut SpriteBatcher,
        panel_y: f32,
        left_divider: f32,
        right_divider: f32,
    ) {
        let btn_w = 22.0_f32;
        let btn_h = 20.0_f32;
        let btn_gap = 3.0_f32;
        let btn_y = panel_y + PANEL_H - btn_h - 4.0; // Near bottom of panel.

        // Left side buttons: Character, Inventory (just inside left divider).
        let left_start_x = left_divider + 6.0;
        for i in 0..2 {
            let bx = left_start_x + i as f32 * (btn_w + btn_gap);
            let bg = if (i == 0 && self.character_open) || (i == 1 && self.inventory_open) {
                COL_MENU_BTN_ACTIVE
            } else {
                COL_MENU_BTN_BG
            };
            // Border.
            batcher.push(bx - 1.0, btn_y - 1.0, btn_w + 2.0, btn_h + 2.0, UV_FULL, COL_BORDER_GOLD);
            // Background.
            batcher.push(bx, btn_y, btn_w, btn_h, UV_FULL, bg);
            // 3D bevel (light top, dark bottom).
            batcher.push(bx, btn_y, btn_w, 1.0, UV_FULL, COL_BEVEL_LIGHT);
            batcher.push(bx, btn_y + btn_h - 1.0, btn_w, 1.0, UV_FULL, COL_BEVEL_DARK);
        }

        // Right side buttons: Skill Tree, Quest Log (just inside right divider).
        let right_start_x = right_divider - 6.0 - 2.0 * btn_w - btn_gap;
        for i in 0..2 {
            let bx = right_start_x + i as f32 * (btn_w + btn_gap);
            let bg = if (i == 0 && self.skill_panel_open) || (i == 1 && self.quest_log_open) {
                COL_MENU_BTN_ACTIVE
            } else {
                COL_MENU_BTN_BG
            };
            batcher.push(bx - 1.0, btn_y - 1.0, btn_w + 2.0, btn_h + 2.0, UV_FULL, COL_BORDER_GOLD);
            batcher.push(bx, btn_y, btn_w, btn_h, UV_FULL, bg);
            batcher.push(bx, btn_y, btn_w, 1.0, UV_FULL, COL_BEVEL_LIGHT);
            batcher.push(bx, btn_y + btn_h - 1.0, btn_w, 1.0, UV_FULL, COL_BEVEL_DARK);
        }
    }

    // -- HP Orb (left side of control panel) --------------------------------

    /// Draw the health orb on the left side of the control panel.
    ///
    /// The orb is approximated using vertical slices forming a circle.
    /// Each slice is a thin quad whose height corresponds to the circle
    /// equation at that x offset. The fill level is controlled by HP ratio.
    fn draw_hp_orb(&self, batcher: &mut SpriteBatcher) {
        let panel_y = self.screen_h - PANEL_H - XP_BAR_H;
        let cx = ORB_INSET + ORB_DIAMETER / 2.0;
        let cy = panel_y + 2.0 + PANEL_H / 2.0;
        let radius = ORB_DIAMETER / 2.0;
        let ratio = safe_ratio(self.hp_current, self.hp_max);

        // Outer decorative frame (D2-style stone ring around the orb).
        draw_orb_frame(batcher, cx, cy, radius);
        // Orb fill.
        draw_orb(batcher, cx, cy, radius, ratio, COL_HP_BG, COL_HP_FG);
        // Fill level separator line.
        draw_orb_fill_line(batcher, cx, cy, radius, ratio);
        // Highlight (top-left shine).
        draw_orb_highlight(batcher, cx, cy, radius);
        // Inner gold ring.
        draw_orb_ring(batcher, cx, cy, radius, COL_BORDER_GOLD);
    }

    // -- Mana Orb (right side of control panel) ----------------------------

    /// Draw the mana orb on the right side of the control panel.
    fn draw_mana_orb(&self, batcher: &mut SpriteBatcher) {
        let panel_y = self.screen_h - PANEL_H - XP_BAR_H;
        let cx = self.screen_w - ORB_INSET - ORB_DIAMETER / 2.0;
        let cy = panel_y + 2.0 + PANEL_H / 2.0;
        let radius = ORB_DIAMETER / 2.0;
        let ratio = safe_ratio(self.mana_current, self.mana_max);

        draw_orb_frame(batcher, cx, cy, radius);
        draw_orb(batcher, cx, cy, radius, ratio, COL_MANA_BG, COL_MANA_FG);
        draw_orb_fill_line(batcher, cx, cy, radius, ratio);
        draw_orb_highlight(batcher, cx, cy, radius);
        draw_orb_ring(batcher, cx, cy, radius, COL_BORDER_GOLD);
    }

    // -- XP Bar -------------------------------------------------------------

    /// Draw the XP bar above the control panel, full screen width.
    fn draw_xp_bar(&self, batcher: &mut SpriteBatcher) {
        let y = self.screen_h - PANEL_H - XP_BAR_H;

        // Background (drawn by control panel), but add separate XP track.
        batcher.push(0.0, y - XP_BAR_H, self.screen_w, XP_BAR_H, UV_FULL, COL_XP_BG);

        // Gold border lines above and below XP bar.
        batcher.push(0.0, y - XP_BAR_H, self.screen_w, 1.0, UV_FULL, COL_BORDER_GOLD);
        batcher.push(0.0, y - 1.0, self.screen_w, 1.0, UV_FULL, COL_BORDER_GOLD);

        let ratio = safe_ratio_i64(self.xp_current, self.xp_next_level);
        let fill_w = self.screen_w * ratio;
        if fill_w > 0.0 {
            batcher.push(0.0, y - XP_BAR_H + 1.0, fill_w, XP_BAR_H - 2.0, UV_FULL, COL_XP_FG);
        }

        // D2-style segment dividers (10 segments).
        let segments = 10_usize;
        for i in 1..segments {
            let dx = self.screen_w * (i as f32 / segments as f32);
            batcher.push(dx - 0.5, y - XP_BAR_H + 1.0, 1.0, XP_BAR_H - 2.0, UV_FULL, COL_XP_DIVIDER);
        }
    }

    // -- D2-style Active Skill Selectors (flanking the orbs) ----------------

    /// Draw the left-click and right-click active skill selectors.
    ///
    /// In D2, these appear next to each orb: left-click skill is near the HP
    /// orb, right-click skill is near the mana orb.
    fn draw_active_skills(&self, batcher: &mut SpriteBatcher) {
        let panel_y = self.screen_h - PANEL_H - XP_BAR_H;
        let orb_cy = panel_y + 2.0 + PANEL_H / 2.0;
        let skill_half = ACTIVE_SKILL_SIZE / 2.0;

        // Left-click skill (to the right of the HP orb).
        let left_x = ORB_INSET + ORB_DIAMETER + ACTIVE_SKILL_GAP;
        let left_y = orb_cy - skill_half;

        // Border.
        batcher.push(
            left_x - 2.0, left_y - 2.0,
            ACTIVE_SKILL_SIZE + 4.0, ACTIVE_SKILL_SIZE + 4.0,
            UV_FULL, COL_BORDER_GOLD_BRIGHT,
        );
        // Background.
        batcher.push(left_x, left_y, ACTIVE_SKILL_SIZE, ACTIVE_SKILL_SIZE, UV_FULL, COL_ACTIVE_SKILL_BG);
        // Left-click accent stripe (warm colour on left edge).
        batcher.push(left_x, left_y, 4.0, ACTIVE_SKILL_SIZE, UV_FULL, COL_LEFT_SKILL);

        // Right-click skill (to the left of the Mana orb).
        let right_x = self.screen_w - ORB_INSET - ORB_DIAMETER - ACTIVE_SKILL_GAP - ACTIVE_SKILL_SIZE;
        let right_y = orb_cy - skill_half;

        // Border.
        batcher.push(
            right_x - 2.0, right_y - 2.0,
            ACTIVE_SKILL_SIZE + 4.0, ACTIVE_SKILL_SIZE + 4.0,
            UV_FULL, COL_BORDER_GOLD_BRIGHT,
        );
        // Background.
        batcher.push(right_x, right_y, ACTIVE_SKILL_SIZE, ACTIVE_SKILL_SIZE, UV_FULL, COL_ACTIVE_SKILL_BG);
        // Right-click accent stripe (cool colour on right edge).
        batcher.push(
            right_x + ACTIVE_SKILL_SIZE - 4.0, right_y,
            4.0, ACTIVE_SKILL_SIZE,
            UV_FULL, COL_RIGHT_SKILL,
        );
    }

    // -- Stamina Bar (between orbs, above belt) ----------------------------

    /// Draw the stamina bar, D2-style, between the two orbs.
    fn draw_stamina_bar(&self, batcher: &mut SpriteBatcher) {
        let panel_y = self.screen_h - PANEL_H - XP_BAR_H;
        // Position: below the skill bar, above the belt.
        let bar_y = panel_y + PANEL_H - STAMINA_BAR_H - 6.0;

        // Compute the horizontal span between orb sections.
        let left_edge = ORB_INSET + ORB_DIAMETER + ACTIVE_SKILL_GAP + ACTIVE_SKILL_SIZE + STAMINA_BAR_MARGIN + 10.0;
        let right_edge = self.screen_w - ORB_INSET - ORB_DIAMETER - ACTIVE_SKILL_GAP - ACTIVE_SKILL_SIZE - STAMINA_BAR_MARGIN - 10.0;
        let bar_w = right_edge - left_edge;

        if bar_w <= 0.0 {
            return;
        }

        // Background.
        batcher.push(left_edge, bar_y, bar_w, STAMINA_BAR_H, UV_FULL, COL_STAMINA_BG);
        // Border.
        batcher.push(left_edge, bar_y, bar_w, 1.0, UV_FULL, COL_BORDER_GOLD);
        batcher.push(left_edge, bar_y + STAMINA_BAR_H - 1.0, bar_w, 1.0, UV_FULL, COL_BORDER_GOLD);

        // Fill.
        let ratio = safe_ratio(self.stamina_current, self.stamina_max);
        let fill_w = (bar_w - 2.0) * ratio;
        if fill_w > 0.0 {
            batcher.push(left_edge + 1.0, bar_y + 1.0, fill_w, STAMINA_BAR_H - 2.0, UV_FULL, COL_STAMINA_FG);
        }
    }

    // -- Run/Walk Toggle ---------------------------------------------------

    /// Draw the run/walk toggle indicator near the stamina bar.
    fn draw_run_walk_toggle(&self, batcher: &mut SpriteBatcher) {
        let panel_y = self.screen_h - PANEL_H - XP_BAR_H;
        let toggle_size = 18.0_f32;
        // Position: right side of stamina bar area.
        let right_edge = self.screen_w - ORB_INSET - ORB_DIAMETER - ACTIVE_SKILL_GAP - ACTIVE_SKILL_SIZE - STAMINA_BAR_MARGIN - 10.0;
        let tx = right_edge + 4.0;
        let ty = panel_y + PANEL_H - STAMINA_BAR_H - 6.0 - toggle_size / 2.0 + STAMINA_BAR_H / 2.0;

        // Clamp to reasonable position.
        if tx + toggle_size > self.screen_w {
            return;
        }

        let color = if self.is_running {
            COL_RUN_ACTIVE
        } else {
            COL_WALK_ACTIVE
        };

        // Border.
        batcher.push(tx - 1.0, ty - 1.0, toggle_size + 2.0, toggle_size + 2.0, UV_FULL, COL_BORDER_GOLD);
        // Background.
        batcher.push(tx, ty, toggle_size, toggle_size, UV_FULL, color);
    }

    // -- Automap Overlay (Tab key) -----------------------------------------

    /// Draw the semi-transparent automap overlay on the game world.
    fn draw_automap(&self, batcher: &mut SpriteBatcher) {
        // Semi-transparent full-screen overlay.
        let map_h = self.screen_h - PANEL_H - XP_BAR_H;
        batcher.push(0.0, 0.0, self.screen_w, map_h, UV_FULL, COL_AUTOMAP_BG);

        // Map covers map_size tiles. Scale to fit the screen.
        let map_tiles = self.minimap_map_size.0.max(self.minimap_map_size.1);
        let tile_px = (self.screen_w / map_tiles).min(map_h / map_tiles) * 0.6;
        let offset_x = (self.screen_w - map_tiles * tile_px) / 2.0;
        let offset_y = (map_h - map_tiles * tile_px) / 2.0;

        // Draw walkable tile grid (small dots).
        let dot_size = (tile_px * 0.6).max(2.0);
        for &(tx, ty) in &self.automap_tiles {
            let px = offset_x + tx * tile_px;
            let py = offset_y + ty * tile_px;
            batcher.push(px, py, dot_size, dot_size, UV_FULL, COL_AUTOMAP_TILE);
        }

        // Monster dots.
        let monster_dot = (tile_px * 0.8).max(3.0);
        let half_m = monster_dot / 2.0;
        for monster in &self.minimap_monsters {
            let px = offset_x + monster.x * tile_px - half_m;
            let py = offset_y + monster.y * tile_px - half_m;
            batcher.push(px, py, monster_dot, monster_dot, UV_FULL, COL_AUTOMAP_MONSTER);
        }

        // Player dot (larger, centred).
        let player_dot = (tile_px * 1.2).max(4.0);
        let half_p = player_dot / 2.0;
        let player_px = offset_x + self.minimap_player.0 * tile_px - half_p;
        let player_py = offset_y + self.minimap_player.1 * tile_px - half_p;
        batcher.push(player_px, player_py, player_dot, player_dot, UV_FULL, COL_AUTOMAP_PLAYER);
    }

    // -- Character Panel (C key) -------------------------------------------

    /// Draw the D2-style character stats panel on the left side.
    fn draw_character_panel(&self, batcher: &mut SpriteBatcher) {
        let panel_h = 320.0_f32;
        let panel_x = MARGIN;
        let panel_y = (self.screen_h - PANEL_H - XP_BAR_H - panel_h) / 2.0;

        // Panel border.
        batcher.push(
            panel_x - 2.0, panel_y - 2.0,
            CHAR_PANEL_W + 4.0, panel_h + 4.0,
            UV_FULL, COL_BORDER_GOLD,
        );

        // Panel background.
        batcher.push(panel_x, panel_y, CHAR_PANEL_W, panel_h, UV_FULL, COL_CHAR_PANEL_BG);

        // Header.
        let header_bg: [f32; 4] = [0.10, 0.08, 0.12, 0.95];
        batcher.push(panel_x, panel_y, CHAR_PANEL_W, INV_HEADER_H, UV_FULL, header_bg);
        batcher.push(panel_x, panel_y + INV_HEADER_H - 1.0, CHAR_PANEL_W, 1.0, UV_FULL, COL_BORDER_GOLD);

        // Stat rows with coloured bars.
        let stat_start_y = panel_y + INV_HEADER_H + 8.0;
        let stat_bar_w = 160.0_f32;
        let stat_bar_h = 16.0_f32;
        let stat_x = panel_x + 90.0;
        let stat_max = 100_f32; // Visual max for bar display.

        let stats: [(i32, [f32; 4]); 4] = [
            (self.stat_strength, COL_STAT_STR),
            (self.stat_dexterity, COL_STAT_DEX),
            (self.stat_vitality, COL_STAT_VIT),
            (self.stat_energy, COL_STAT_ENE),
        ];

        for (i, (value, color)) in stats.iter().enumerate() {
            let row_y = stat_start_y + i as f32 * CHAR_STAT_ROW_H;

            // Bar background.
            batcher.push(stat_x, row_y, stat_bar_w, stat_bar_h, UV_FULL, COL_SLOT_BG);
            // Bar border.
            batcher.push(stat_x, row_y, stat_bar_w, 1.0, UV_FULL, COL_BORDER_GOLD);
            batcher.push(stat_x, row_y + stat_bar_h - 1.0, stat_bar_w, 1.0, UV_FULL, COL_BORDER_GOLD);
            batcher.push(stat_x, row_y, 1.0, stat_bar_h, UV_FULL, COL_BORDER_GOLD);
            batcher.push(stat_x + stat_bar_w - 1.0, row_y, 1.0, stat_bar_h, UV_FULL, COL_BORDER_GOLD);

            // Fill.
            let ratio = (*value as f32 / stat_max).clamp(0.0, 1.0);
            let fill_w = (stat_bar_w - 2.0) * ratio;
            if fill_w > 0.0 {
                batcher.push(stat_x + 1.0, row_y + 1.0, fill_w, stat_bar_h - 2.0, UV_FULL, *color);
            }
        }

        // Separator line before secondary stats.
        let sep_y = stat_start_y + 4.0 * CHAR_STAT_ROW_H + 8.0;
        batcher.push(panel_x + 8.0, sep_y, CHAR_PANEL_W - 16.0, 1.0, UV_FULL, COL_BORDER_GOLD);

        // Secondary stat rows (Defense, Damage, AR).
        let sec_start_y = sep_y + 12.0;
        for i in 0..3 {
            let row_y = sec_start_y + i as f32 * CHAR_STAT_ROW_H;
            // Just background bars for text.
            batcher.push(stat_x, row_y, stat_bar_w, stat_bar_h, UV_FULL, COL_SLOT_BG);
            batcher.push(stat_x, row_y, stat_bar_w, 1.0, UV_FULL, COL_BORDER_GOLD);
            batcher.push(stat_x, row_y + stat_bar_h - 1.0, stat_bar_w, 1.0, UV_FULL, COL_BORDER_GOLD);
        }

        // Stat points available: draw "+" buttons next to each primary stat.
        if self.stat_points_available > 0 {
            let btn_size = 16.0_f32;
            let btn_x = stat_x + stat_bar_w + 6.0;
            let btn_color: [f32; 4] = [0.3, 0.6, 0.2, 0.9];
            for i in 0..4_usize {
                let row_y = stat_start_y + i as f32 * CHAR_STAT_ROW_H;
                // Button background.
                batcher.push(btn_x, row_y, btn_size, btn_size, UV_FULL, btn_color);
                // Border.
                batcher.push(btn_x, row_y, btn_size, 1.0, UV_FULL, COL_BORDER_GOLD);
                batcher.push(btn_x, row_y + btn_size - 1.0, btn_size, 1.0, UV_FULL, COL_BORDER_GOLD);
            }

            // Stat points indicator below secondary stats.
            let sp_y = sec_start_y + 3.0 * CHAR_STAT_ROW_H + 8.0;
            let sp_bg: [f32; 4] = [0.3, 0.25, 0.1, 0.8];
            batcher.push(panel_x + 8.0, sp_y, CHAR_PANEL_W - 16.0, 20.0, UV_FULL, sp_bg);
        }
    }

    // -- Skill Bar (centre of control panel) --------------------------------

    /// Draw the 6-slot skill bar centred in the control panel.
    fn draw_skill_bar(&self, batcher: &mut SpriteBatcher) {
        let total_w = SKILL_SLOT_COUNT as f32 * (SKILL_SLOT_SIZE + SKILL_SLOT_GAP)
            - SKILL_SLOT_GAP;
        let start_x = (self.screen_w - total_w) / 2.0;
        let panel_y = self.screen_h - PANEL_H - XP_BAR_H;
        let y = panel_y + 16.0;

        for i in 0..SKILL_SLOT_COUNT {
            let sx = start_x + i as f32 * (SKILL_SLOT_SIZE + SKILL_SLOT_GAP);

            // Gold border quad.
            batcher.push(sx, y, SKILL_SLOT_SIZE, SKILL_SLOT_SIZE, UV_FULL, COL_BORDER_GOLD_BRIGHT);

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

    // -- Belt (potion slots below skill bar) --------------------------------

    /// Draw the 4-slot potion belt centred below the skill bar.
    ///
    /// In D2, potion slots show coloured fills indicating the potion type:
    /// red for HP, blue for mana, purple for rejuvenation.
    fn draw_belt(&self, batcher: &mut SpriteBatcher) {
        let total_w = BELT_SLOT_COUNT as f32 * (BELT_SLOT_SIZE + BELT_SLOT_GAP)
            - BELT_SLOT_GAP;
        let start_x = (self.screen_w - total_w) / 2.0;
        let panel_y = self.screen_h - PANEL_H - XP_BAR_H;
        let y = panel_y + 16.0 + SKILL_SLOT_SIZE + 6.0;

        for i in 0..BELT_SLOT_COUNT {
            let sx = start_x + i as f32 * (BELT_SLOT_SIZE + BELT_SLOT_GAP);

            // Border.
            batcher.push(sx, y, BELT_SLOT_SIZE, BELT_SLOT_SIZE, UV_FULL, COL_SLOT_BORDER);

            // Inner.
            let inner = BELT_SLOT_SIZE - 2.0;
            batcher.push(sx + 1.0, y + 1.0, inner, inner, UV_FULL, COL_BELT_BG);

            // Potion fill colour (if slot has a potion).
            let potion_type = self.belt_potions[i];
            if potion_type > 0 {
                let potion_color = match potion_type {
                    2 => COL_POTION_MANA,
                    3 => COL_POTION_REJUV,
                    4 => COL_POTION_STAMINA,
                    _ => COL_POTION_HP, // 1 or any other = HP potion
                };
                // Fill from bottom up (like a liquid level) — full fill.
                let fill_pad = 4.0;
                let fill_size = BELT_SLOT_SIZE - fill_pad * 2.0;
                batcher.push(sx + fill_pad, y + fill_pad, fill_size, fill_size, UV_FULL, potion_color);

                // Glass highlight (top shimmer).
                let highlight: [f32; 4] = [1.0, 1.0, 1.0, 0.12];
                batcher.push(sx + fill_pad + 2.0, y + fill_pad + 1.0, fill_size - 4.0, 3.0, UV_FULL, highlight);
            }
        }
    }

    // -- Combat Log (top-left) ----------------------------------------------

    /// Draw the combat log with semi-transparent background and border.
    fn draw_combat_log(&self, batcher: &mut SpriteBatcher) {
        if self.combat_log.is_empty() {
            return;
        }

        let x = MARGIN;
        let y = MARGIN;
        let msg_count = self.combat_log.len();
        let log_h = msg_count as f32 * (LOG_ENTRY_H + 2.0) + 8.0;

        // Background panel.
        batcher.push(x, y, LOG_W, log_h, UV_FULL, COL_LOG_BG);

        // Border (1px lines on all four sides).
        batcher.push(x, y, LOG_W, 1.0, UV_FULL, COL_LOG_BORDER); // top
        batcher.push(x, y + log_h - 1.0, LOG_W, 1.0, UV_FULL, COL_LOG_BORDER); // bottom
        batcher.push(x, y, 1.0, log_h, UV_FULL, COL_LOG_BORDER); // left
        batcher.push(x + LOG_W - 1.0, y, 1.0, log_h, UV_FULL, COL_LOG_BORDER); // right

        // Individual message backgrounds (alternating for readability).
        let mut entry_y = y + 4.0;
        for i in 0..msg_count {
            let alpha = if i % 2 == 0 { 0.12 } else { 0.04 };
            let row_bg: [f32; 4] = [0.1, 0.08, 0.12, alpha];
            batcher.push(x + 2.0, entry_y, LOG_W - 4.0, LOG_ENTRY_H, UV_FULL, row_bg);
            entry_y += LOG_ENTRY_H + 2.0;
        }
    }

    // -- Minimap (top-right) -----------------------------------------------

    /// Draw the minimap with dungeon layout, player and monster dots.
    fn draw_minimap(&self, batcher: &mut SpriteBatcher) {
        let x = self.screen_w - MINIMAP_SIZE - MARGIN;
        let y = MARGIN;

        // Border.
        batcher.push(
            x - MINIMAP_BORDER,
            y - MINIMAP_BORDER,
            MINIMAP_SIZE + MINIMAP_BORDER * 2.0,
            MINIMAP_SIZE + MINIMAP_BORDER * 2.0,
            UV_FULL,
            COL_MINIMAP_BORDER,
        );

        // Background.
        batcher.push(x, y, MINIMAP_SIZE, MINIMAP_SIZE, UV_FULL, COL_MINIMAP_BG);

        let (map_w, map_h) = self.minimap_map_size;
        let scale_x = MINIMAP_SIZE / map_w;
        let scale_y = MINIMAP_SIZE / map_h;

        // Render dungeon tiles (condensed: every 2x2 block as one pixel for performance).
        let mw = map_w as i32;
        let mh = map_h as i32;
        if self.minimap_tiles.len() == (mw * mh) as usize {
            let step = 2; // Render every 2nd tile for performance.
            let tile_px_w = scale_x * step as f32;
            let tile_px_h = scale_y * step as f32;
            let mut ty_coord = 0;
            while ty_coord < mh {
                let mut tx_coord = 0;
                while tx_coord < mw {
                    let idx = (ty_coord * mw + tx_coord) as usize;
                    let tile_byte = self.minimap_tiles[idx];
                    let color = match tile_byte {
                        1 => [0.25, 0.22, 0.18, 0.6], // walkable (tan)
                        2 => [0.40, 0.35, 0.25, 0.8], // wall (brown)
                        3 => [0.15, 0.20, 0.50, 0.7], // water (blue)
                        _ => {
                            tx_coord += step;
                            continue; // void: skip (transparent)
                        }
                    };
                    let px = x + tx_coord as f32 * scale_x;
                    let py = y + ty_coord as f32 * scale_y;
                    batcher.push(px, py, tile_px_w, tile_px_h, UV_FULL, color);
                    tx_coord += step;
                }
                ty_coord += step;
            }
        }

        // Monster dots (drawn on top of tiles, under player).
        let dot_size = 3.0_f32;
        let half_dot = dot_size / 2.0;
        for monster in &self.minimap_monsters {
            let mx = x + monster.x * scale_x - half_dot;
            let my = y + monster.y * scale_y - half_dot;
            if mx >= x && mx + dot_size <= x + MINIMAP_SIZE
                && my >= y && my + dot_size <= y + MINIMAP_SIZE
            {
                batcher.push(mx, my, dot_size, dot_size, UV_FULL, COL_MINIMAP_MONSTER);
            }
        }

        // Player dot (white, slightly larger).
        let player_dot = 4.0_f32;
        let half_player = player_dot / 2.0;
        let px = x + self.minimap_player.0 * scale_x - half_player;
        let py = y + self.minimap_player.1 * scale_y - half_player;
        batcher.push(
            px.clamp(x, x + MINIMAP_SIZE - player_dot),
            py.clamp(y, y + MINIMAP_SIZE - player_dot),
            player_dot,
            player_dot,
            UV_FULL,
            COL_MINIMAP_PLAYER,
        );
    }

    // -- Inventory Panel ---------------------------------------------------

    /// Draw the inventory panel on the right side of the screen.
    ///
    /// D2-style: equipment slots at top (character silhouette area),
    /// then inventory grid below, then gold footer.
    fn draw_inventory_panel(&self, batcher: &mut SpriteBatcher) {
        let (panel_x, panel_y, panel_w, panel_h) = self.inventory_panel_bounds();

        // Panel border (gold).
        batcher.push(
            panel_x - 2.0, panel_y - 2.0,
            panel_w + 4.0, panel_h + 4.0,
            UV_FULL, COL_BORDER_GOLD,
        );

        // Panel background.
        batcher.push(panel_x, panel_y, panel_w, panel_h, UV_FULL, COL_INV_PANEL_BG);

        // Header background.
        let header_bg: [f32; 4] = [0.10, 0.08, 0.12, 0.95];
        batcher.push(panel_x, panel_y, panel_w, INV_HEADER_H, UV_FULL, header_bg);
        batcher.push(panel_x, panel_y + INV_HEADER_H - 1.0, panel_w, 1.0, UV_FULL, COL_BORDER_GOLD);

        // Close button "X".
        let close_size = 20.0_f32;
        let close_x = panel_x + panel_w - close_size - 4.0;
        let close_y = panel_y + (INV_HEADER_H - close_size) / 2.0;
        let close_bg: [f32; 4] = [0.5, 0.15, 0.15, 0.8];
        batcher.push(close_x, close_y, close_size, close_size, UV_FULL, close_bg);

        // --- Equipment slots area (D2-style layout) ---
        let equip_y = panel_y + INV_HEADER_H + 4.0;
        let equip_area_h = EQUIP_SLOT_SIZE * 2.0 + EQUIP_SLOT_GAP + 8.0;

        // Darker equipment area background.
        batcher.push(panel_x + 2.0, equip_y, panel_w - 4.0, equip_area_h, UV_FULL, COL_EQUIP_SLOT);

        // Equipment layout: top row (Helm, Amulet), middle row (Weapon, Armor, Shield),
        // bottom row (Gloves, Belt, Boots). Simplified to 2 rows of slots:
        // Row 1: Helm (centre), Weapon (left), Shield (right)
        // Row 2: Gloves (left), Armor (centre), Boots (right), Belt (small)
        let equip_names = ["Helm", "Armor", "Weapon", "Shield", "Gloves", "Boots", "Belt"];
        let equip_positions: [(f32, f32); 7] = [
            // Helm: top centre
            (panel_w / 2.0 - EQUIP_SLOT_SIZE / 2.0, 2.0),
            // Armor: bottom centre
            (panel_w / 2.0 - EQUIP_SLOT_SIZE / 2.0, EQUIP_SLOT_SIZE + EQUIP_SLOT_GAP + 2.0),
            // Weapon: top left
            (8.0, EQUIP_SLOT_SIZE / 2.0),
            // Shield: top right
            (panel_w - EQUIP_SLOT_SIZE - 8.0, EQUIP_SLOT_SIZE / 2.0),
            // Gloves: bottom left
            (8.0, EQUIP_SLOT_SIZE + EQUIP_SLOT_GAP + 2.0),
            // Boots: bottom right
            (panel_w - EQUIP_SLOT_SIZE - 8.0, EQUIP_SLOT_SIZE + EQUIP_SLOT_GAP + 2.0),
            // Belt: bottom centre-right
            (panel_w / 2.0 + EQUIP_SLOT_SIZE / 2.0 + 4.0, EQUIP_SLOT_SIZE + EQUIP_SLOT_GAP + 2.0),
        ];

        let _ = equip_names; // Labels rendered in text pass.

        for (i, (ex, ey)) in equip_positions.iter().enumerate() {
            let abs_x = panel_x + ex;
            let abs_y = equip_y + ey;

            // Slot border.
            batcher.push(abs_x, abs_y, EQUIP_SLOT_SIZE, EQUIP_SLOT_SIZE, UV_FULL, COL_SLOT_BORDER);
            // Slot interior.
            let inner = EQUIP_SLOT_SIZE - 2.0;
            batcher.push(abs_x + 1.0, abs_y + 1.0, inner, inner, UV_FULL, COL_SLOT_BG);

            // Equipment item (if any).
            if let Some(item) = &self.equipment_slots[i] {
                let item_pad = 4.0;
                let item_size = EQUIP_SLOT_SIZE - item_pad * 2.0;
                batcher.push(
                    abs_x + item_pad, abs_y + item_pad,
                    item_size, item_size,
                    UV_FULL, item.quality_color,
                );
            }
        }

        // Separator between equipment and inventory grid.
        let sep_y = equip_y + equip_area_h;
        batcher.push(panel_x + 4.0, sep_y, panel_w - 8.0, 1.0, UV_FULL, COL_BORDER_GOLD);

        // --- Inventory grid ---
        let grid_origin_y = sep_y + 2.0;
        let inner_x = panel_x + INV_PANEL_PAD;
        let inner_y = grid_origin_y + INV_PANEL_PAD;
        let cell = INV_SLOT_SIZE + INV_SLOT_GAP;

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

                // Item contents.
                if let Some(Some(item)) = self.inventory_slots.get(idx) {
                    let item_pad = 3.0;
                    let item_size = INV_SLOT_SIZE - item_pad * 2.0;
                    batcher.push(
                        sx + item_pad, sy + item_pad,
                        item_size, item_size,
                        UV_FULL, item.quality_color,
                    );
                }
            }
        }

        // Footer.
        let footer_y = grid_origin_y + INV_PANEL_PAD * 2.0 + INV_ROWS as f32 * cell;
        let footer_bg: [f32; 4] = [0.08, 0.06, 0.10, 0.95];
        batcher.push(panel_x, footer_y, panel_w, INV_FOOTER_H, UV_FULL, footer_bg);
        batcher.push(panel_x, footer_y, panel_w, 1.0, UV_FULL, COL_BORDER_GOLD);
    }

    // -- Skill Panel -------------------------------------------------------

    /// Draw the D2-style skill tree panel with 3 tabs on the left side.
    ///
    /// In D2, each class has 3 skill tabs (e.g. Sorceress: Fire/Cold/Lightning).
    /// Each tab shows a tree of skills that can be levelled up.
    fn draw_skill_panel(&self, batcher: &mut SpriteBatcher) {
        let panel_w = 240.0_f32;
        let tab_h = 28.0_f32;
        let panel_h = tab_h + SKILL_SLOT_COUNT as f32 * (SKILL_SLOT_SIZE + SKILL_SLOT_GAP)
            + INV_PANEL_PAD * 2.0
            + INV_HEADER_H;
        let panel_x = MARGIN;
        let panel_y = (self.screen_h - PANEL_H - XP_BAR_H - panel_h) / 2.0;

        // Panel border (gold).
        batcher.push(
            panel_x - 2.0, panel_y - 2.0,
            panel_w + 4.0, panel_h + 4.0,
            UV_FULL, COL_BORDER_GOLD,
        );

        // Panel background.
        batcher.push(panel_x, panel_y, panel_w, panel_h, UV_FULL, COL_INV_PANEL_BG);

        // Header background.
        let header_bg: [f32; 4] = [0.10, 0.08, 0.12, 0.95];
        batcher.push(panel_x, panel_y, panel_w, INV_HEADER_H, UV_FULL, header_bg);
        batcher.push(panel_x, panel_y + INV_HEADER_H - 1.0, panel_w, 1.0, UV_FULL, COL_BORDER_GOLD);

        // --- 3 Tabs (D2-style: Fire / Cold / Lightning) ---
        let tab_y = panel_y + INV_HEADER_H;
        let tab_w = panel_w / 3.0;

        for t in 0..3_u8 {
            let tx = panel_x + t as f32 * tab_w;
            let bg = if t == self.skill_tree_tab {
                COL_SKILL_TAB_ACTIVE
            } else {
                COL_SKILL_TAB_BG
            };
            batcher.push(tx, tab_y, tab_w, tab_h, UV_FULL, bg);
            // Tab border (right edge separator).
            if t < 2 {
                batcher.push(tx + tab_w - 1.0, tab_y, 1.0, tab_h, UV_FULL, COL_SKILL_TAB_BORDER);
            }
            // Active tab bottom highlight.
            if t == self.skill_tree_tab {
                batcher.push(tx, tab_y + tab_h - 2.0, tab_w, 2.0, UV_FULL, COL_BORDER_GOLD_BRIGHT);
            }
        }
        // Tab bottom border.
        batcher.push(panel_x, tab_y + tab_h - 1.0, panel_w, 1.0, UV_FULL, COL_BORDER_GOLD);

        // --- Skill slot rows (per-tab skills) ---
        let slot_start_y = tab_y + tab_h + INV_PANEL_PAD;

        // Different color accents per tab.
        let tab_accent_colors: [[f32; 4]; 3] = [
            [0.8, 0.3, 0.15, 0.9], // Fire (red-orange)
            [0.2, 0.5, 0.8, 0.9],  // Cold (ice blue)
            [0.8, 0.7, 0.2, 0.9],  // Lightning (gold)
        ];
        let accent = tab_accent_colors[self.skill_tree_tab as usize % 3];

        for i in 0..SKILL_SLOT_COUNT {
            let sy = slot_start_y + i as f32 * (SKILL_SLOT_SIZE + SKILL_SLOT_GAP);
            let sx = panel_x + INV_PANEL_PAD;
            let row_w = panel_w - INV_PANEL_PAD * 2.0;

            // Slot border.
            batcher.push(sx, sy, row_w, SKILL_SLOT_SIZE, UV_FULL, COL_SLOT_BORDER);

            // Slot interior.
            let inner_pad = 2.0;
            batcher.push(
                sx + inner_pad, sy + inner_pad,
                row_w - inner_pad * 2.0, SKILL_SLOT_SIZE - inner_pad * 2.0,
                UV_FULL, COL_SLOT_BG,
            );

            // Tab-coloured accent stripe on the left.
            batcher.push(
                sx + inner_pad, sy + inner_pad,
                6.0, SKILL_SLOT_SIZE - inner_pad * 2.0,
                UV_FULL, accent,
            );
        }
    }

    /// Render skill panel text labels via the bitmap font (3-tab layout).
    ///
    /// Called from the text render pass in `game.rs`. Uses
    /// [`BitmapFont::push_text`] to draw skill names, tab labels, and key bindings.
    pub fn draw_skill_panel_text(&self, batcher: &mut SpriteBatcher) {
        use crate::bitmap_font::BitmapFont;

        let panel_w = 240.0_f32;
        let tab_h = 28.0_f32;
        let panel_h = tab_h + SKILL_SLOT_COUNT as f32 * (SKILL_SLOT_SIZE + SKILL_SLOT_GAP)
            + INV_PANEL_PAD * 2.0
            + INV_HEADER_H;
        let panel_x = MARGIN;
        let panel_y = (self.screen_h - PANEL_H - XP_BAR_H - panel_h) / 2.0;

        // Title text.
        BitmapFont::push_text(batcher, panel_x + 8.0, panel_y + 6.0, "SKILL TREE (K)", COL_TEXT_PARCHMENT, 2.0);

        // Tab labels.
        let tab_names = ["Fire", "Cold", "Light"];
        let tab_y = panel_y + INV_HEADER_H;
        let tab_w = panel_w / 3.0;
        let tab_scale = 1.2_f32;

        for (t, name) in tab_names.iter().enumerate() {
            let tx = panel_x + t as f32 * tab_w;
            let tw = name.len() as f32 * BitmapFont::char_width(tab_scale);
            let text_color = if t as u8 == self.skill_tree_tab {
                COL_TEXT_PARCHMENT
            } else {
                COL_TEXT_DIM
            };
            BitmapFont::push_text(
                batcher,
                tx + (tab_w - tw) / 2.0,
                tab_y + (tab_h - BitmapFont::line_height(tab_scale)) / 2.0,
                name,
                text_color,
                tab_scale,
            );
        }

        // Skill names per tab.
        let tab_skills: [[&str; 6]; 3] = [
            ["1: Fire Bolt", "2: Inferno", "3: Blaze", "4: Fire Ball", "5: Fire Wall", "6: Meteor"],
            ["1: Ice Bolt", "2: Frost Nova", "3: Ice Blast", "4: Glacial Spike", "5: Blizzard", "6: Frozen Orb"],
            ["1: Charged Bolt", "2: Static", "3: Nova", "4: Lightning", "5: Chain Light", "6: Thunder Storm"],
        ];

        let slot_start_y = tab_y + tab_h + INV_PANEL_PAD;
        let text_scale = 1.5;
        let text_x = panel_x + INV_PANEL_PAD + 12.0;
        let tab = self.skill_tree_tab as usize % 3;

        for (i, name) in tab_skills[tab].iter().enumerate() {
            let sy = slot_start_y + i as f32 * (SKILL_SLOT_SIZE + SKILL_SLOT_GAP);
            let text_y = sy + (SKILL_SLOT_SIZE - BitmapFont::line_height(text_scale)) / 2.0;

            let label = self.skill_slots[i].as_deref().unwrap_or(name);
            BitmapFont::push_text(batcher, text_x, text_y, label, COL_TEXT_PARCHMENT, text_scale);
        }
    }

    /// Render inventory panel text labels (header, gold) via bitmap font.
    ///
    /// Called from the text render pass in `game.rs`.
    pub fn draw_inventory_text(&self, batcher: &mut SpriteBatcher) {
        use crate::bitmap_font::BitmapFont;

        let (panel_x, panel_y, panel_w, panel_h) = self.inventory_panel_bounds();

        // Header title.
        BitmapFont::push_text(
            batcher,
            panel_x + 8.0,
            panel_y + 6.0,
            "INVENTORY (I)",
            COL_TEXT_PARCHMENT,
            2.0,
        );

        // Close button "X" text.
        let close_x = panel_x + panel_w - 20.0 - 4.0 + 5.0;
        let close_y = panel_y + (INV_HEADER_H - BitmapFont::line_height(2.0)) / 2.0;
        BitmapFont::push_text(batcher, close_x, close_y, "X", [1.0, 0.9, 0.9, 1.0], 2.0);

        // Gold display in footer.
        let cell = INV_SLOT_SIZE + INV_SLOT_GAP;
        let footer_y = panel_y + INV_HEADER_H + INV_PANEL_PAD * 2.0 + INV_ROWS as f32 * cell;
        let gold_text = format!("Gold: {}", self.gold);
        let _ = panel_h; // used for layout validation only
        BitmapFont::push_text(
            batcher,
            panel_x + 8.0,
            footer_y + 4.0,
            &gold_text,
            [1.0, 0.85, 0.2, 1.0],
            1.5,
        );
    }

    // -- Quest Log Panel (Q key) -------------------------------------------

    /// Draw the D2-style quest log panel on the right side.
    ///
    /// In D2, the quest log shows the current act's quests with completion
    /// status. For now this is a placeholder with act structure.
    fn draw_quest_log(&self, batcher: &mut SpriteBatcher) {
        let panel_w = 280.0_f32;
        let panel_h = 320.0_f32;
        let panel_x = self.screen_w - panel_w - MARGIN;
        let panel_y = (self.screen_h - PANEL_H - XP_BAR_H - panel_h) / 2.0;

        // Panel border.
        batcher.push(
            panel_x - 2.0, panel_y - 2.0,
            panel_w + 4.0, panel_h + 4.0,
            UV_FULL, COL_BORDER_GOLD,
        );

        // Panel background.
        batcher.push(panel_x, panel_y, panel_w, panel_h, UV_FULL, COL_QUEST_PANEL_BG);

        // Header.
        let header_bg: [f32; 4] = [0.10, 0.08, 0.12, 0.95];
        batcher.push(panel_x, panel_y, panel_w, INV_HEADER_H, UV_FULL, header_bg);
        batcher.push(panel_x, panel_y + INV_HEADER_H - 1.0, panel_w, 1.0, UV_FULL, COL_BORDER_GOLD);

        // Quest rows (6 quests in Act I like D2).
        let quest_start_y = panel_y + INV_HEADER_H + 8.0;
        let quest_row_h = 36.0_f32;
        let quest_count = 6_usize;

        for i in 0..quest_count {
            let qy = quest_start_y + i as f32 * quest_row_h;

            // Row background (alternating).
            let row_bg = if i % 2 == 0 {
                [0.07, 0.06, 0.08, 0.6]
            } else {
                [0.05, 0.04, 0.06, 0.4]
            };
            batcher.push(panel_x + 4.0, qy, panel_w - 8.0, quest_row_h - 2.0, UV_FULL, row_bg);

            // Quest completion indicator (small circle/square).
            let indicator_x = panel_x + 10.0;
            let indicator_y = qy + (quest_row_h - 10.0) / 2.0;
            let completed = i < 2; // First 2 quests "completed" for demo.
            let ind_color = if completed {
                [0.2, 0.7, 0.2, 0.9] // Green check
            } else {
                [0.4, 0.35, 0.25, 0.7] // Dim gold incomplete
            };
            batcher.push(indicator_x, indicator_y, 8.0, 8.0, UV_FULL, ind_color);
        }

        // Separator before footer.
        let footer_y = quest_start_y + quest_count as f32 * quest_row_h + 4.0;
        batcher.push(panel_x + 8.0, footer_y, panel_w - 16.0, 1.0, UV_FULL, COL_BORDER_GOLD);
    }

    // -- Monster Health Bars and Name Tags ---------------------------------

    /// Draw health bars and name tags above visible monsters.
    ///
    /// Uses overlay data set by the game loop each frame via
    /// [`set_monster_overlays`](Self::set_monster_overlays).
    fn draw_monster_overlays(&self, batcher: &mut SpriteBatcher) {
        let bar_w = 40.0_f32;
        let bar_h = 4.0_f32;
        let bar_offset_y = 6.0; // Above the monster sprite top.

        for overlay in &self.monster_overlays {
            let bar_x = overlay.screen_x - bar_w / 2.0;
            let bar_y = overlay.screen_y - bar_offset_y - bar_h;

            // Background.
            batcher.push(bar_x, bar_y, bar_w, bar_h, UV_FULL, COL_MONSTER_HP_BG);
            // Border (thin).
            batcher.push(bar_x, bar_y, bar_w, 1.0, UV_FULL, COL_MONSTER_HP_BORDER);
            batcher.push(bar_x, bar_y + bar_h - 1.0, bar_w, 1.0, UV_FULL, COL_MONSTER_HP_BORDER);

            // Fill.
            let fill_w = (bar_w - 2.0) * overlay.hp_ratio.clamp(0.0, 1.0);
            if fill_w > 0.0 {
                batcher.push(bar_x + 1.0, bar_y + 1.0, fill_w, bar_h - 2.0, UV_FULL, COL_MONSTER_HP_FG);
            }

            // Name tag background (above health bar).
            let name_scale = 1.0_f32;
            let char_w = 8.0 * name_scale;
            let name_w = overlay.name.len() as f32 * char_w;
            let tag_w = name_w + 8.0;
            let tag_h = 12.0_f32;
            let tag_x = overlay.screen_x - tag_w / 2.0;
            let tag_y = bar_y - tag_h - 1.0;

            batcher.push(tag_x, tag_y, tag_w, tag_h, UV_FULL, COL_NAME_TAG_BG);
        }
    }

    // -- Tooltip -----------------------------------------------------------

    /// Draw the tooltip near the mouse cursor.
    fn draw_tooltip(&self, batcher: &mut SpriteBatcher) {
        let Some(ref text) = self.tooltip else {
            return;
        };

        // Estimate text width (8px per char at scale 1.5).
        let scale = 1.5_f32;
        let char_w = 8.0 * scale;
        let text_w = text.len() as f32 * char_w;
        let tooltip_w = text_w.min(TOOLTIP_MAX_W) + TOOLTIP_PAD * 2.0;
        let tooltip_h = 8.0 * scale + TOOLTIP_PAD * 2.0;

        // Position: offset from mouse cursor, clamp to screen.
        let tx = (self.mouse_x + 16.0).min(self.screen_w - tooltip_w - 4.0);
        let ty = (self.mouse_y - tooltip_h - 4.0).max(4.0);

        // Border.
        batcher.push(
            tx - 1.0,
            ty - 1.0,
            tooltip_w + 2.0,
            tooltip_h + 2.0,
            UV_FULL,
            COL_TOOLTIP_BORDER,
        );

        // Background.
        batcher.push(tx, ty, tooltip_w, tooltip_h, UV_FULL, COL_TOOLTIP_BG);
    }

    /// Render tooltip text via bitmap font.
    ///
    /// Called from the text render pass in `game.rs`.
    pub fn draw_tooltip_text(&self, batcher: &mut SpriteBatcher) {
        use crate::bitmap_font::BitmapFont;

        let Some(ref text) = self.tooltip else {
            return;
        };

        let scale = 1.5_f32;
        let char_w = 8.0 * scale;
        let text_w = text.len() as f32 * char_w;
        let tooltip_w = text_w.min(TOOLTIP_MAX_W) + TOOLTIP_PAD * 2.0;
        let tooltip_h = 8.0 * scale + TOOLTIP_PAD * 2.0;

        let tx = (self.mouse_x + 16.0).min(self.screen_w - tooltip_w - 4.0);
        let ty = (self.mouse_y - tooltip_h - 4.0).max(4.0);
        let _ = tooltip_w; // used for positioning validation

        // Determine text colour from item quality (use slot colour if available).
        let text_color = if let Some(slot_idx) = self.tooltip_slot {
            if let Some(Some(item)) = self.inventory_slots.get(slot_idx) {
                item.quality_color
            } else {
                COL_TEXT_PARCHMENT
            }
        } else {
            COL_TEXT_PARCHMENT
        };

        BitmapFont::push_text(
            batcher,
            tx + TOOLTIP_PAD,
            ty + TOOLTIP_PAD,
            text,
            text_color,
            scale,
        );
    }

    /// Render minimap label text via bitmap font.
    pub fn draw_minimap_text(&self, batcher: &mut SpriteBatcher) {
        use crate::bitmap_font::BitmapFont;

        let x = self.screen_w - MINIMAP_SIZE - MARGIN;
        let y = MARGIN + MINIMAP_SIZE + 2.0;

        let label = "MINIMAP";
        let scale = 1.5_f32;
        let label_w = label.len() as f32 * BitmapFont::char_width(scale);
        let label_x = x + (MINIMAP_SIZE - label_w) / 2.0;
        BitmapFont::push_text(batcher, label_x, y, label, COL_TEXT_PARCHMENT, scale);
    }

    /// Render active skill labels on the left/right skill selectors.
    pub fn draw_active_skill_text(&self, batcher: &mut SpriteBatcher) {
        use crate::bitmap_font::BitmapFont;

        let panel_y = self.screen_h - PANEL_H - XP_BAR_H;
        let orb_cy = panel_y + 2.0 + PANEL_H / 2.0;
        let scale = 1.5_f32;
        let text_h = BitmapFont::line_height(scale);

        // Left-click skill name.
        let left_x = ORB_INSET + ORB_DIAMETER + ACTIVE_SKILL_GAP + 8.0;
        let left_y = orb_cy - text_h / 2.0;
        BitmapFont::push_text(batcher, left_x, left_y, &self.left_skill_name, COL_TEXT_PARCHMENT, scale);

        // Right-click skill name.
        let right_x = self.screen_w - ORB_INSET - ORB_DIAMETER - ACTIVE_SKILL_GAP - ACTIVE_SKILL_SIZE + 4.0;
        let right_y = orb_cy - text_h / 2.0;
        BitmapFont::push_text(batcher, right_x, right_y, &self.right_skill_name, COL_TEXT_PARCHMENT, scale);
    }

    /// Render the Run/Walk label next to the toggle.
    pub fn draw_run_walk_text(&self, batcher: &mut SpriteBatcher) {
        use crate::bitmap_font::BitmapFont;

        let panel_y = self.screen_h - PANEL_H - XP_BAR_H;
        let right_edge = self.screen_w - ORB_INSET - ORB_DIAMETER - ACTIVE_SKILL_GAP - ACTIVE_SKILL_SIZE - STAMINA_BAR_MARGIN - 10.0;
        let tx = right_edge + 4.0;
        let toggle_size = 18.0_f32;
        let ty = panel_y + PANEL_H - STAMINA_BAR_H - 6.0 - toggle_size / 2.0 + STAMINA_BAR_H / 2.0;

        if tx + toggle_size > self.screen_w {
            return;
        }

        let label = if self.is_running { "R" } else { "W" };
        let scale = 1.5_f32;
        let lw = BitmapFont::char_width(scale);
        let lh = BitmapFont::line_height(scale);
        BitmapFont::push_text(
            batcher,
            tx + (toggle_size - lw) / 2.0,
            ty + (toggle_size - lh) / 2.0,
            label,
            [1.0, 1.0, 1.0, 1.0],
            scale,
        );
    }

    /// Render character panel text labels.
    pub fn draw_character_panel_text(&self, batcher: &mut SpriteBatcher) {
        use crate::bitmap_font::BitmapFont;

        let panel_h = 320.0_f32;
        let panel_x = MARGIN;
        let panel_y = (self.screen_h - PANEL_H - XP_BAR_H - panel_h) / 2.0;

        // Title.
        BitmapFont::push_text(batcher, panel_x + 8.0, panel_y + 6.0, "CHARACTER (C)", COL_TEXT_PARCHMENT, 2.0);

        // Primary stat labels and values.
        let stat_start_y = panel_y + INV_HEADER_H + 8.0;
        let label_x = panel_x + 8.0;
        let value_x = panel_x + 92.0;
        let scale = 1.5_f32;

        let labels = ["STR", "DEX", "VIT", "ENE"];
        let values = [
            self.stat_strength,
            self.stat_dexterity,
            self.stat_vitality,
            self.stat_energy,
        ];
        let colors: [[f32; 4]; 4] = [COL_STAT_STR, COL_STAT_DEX, COL_STAT_VIT, COL_STAT_ENE];

        for (i, label) in labels.iter().enumerate() {
            let row_y = stat_start_y + i as f32 * CHAR_STAT_ROW_H;
            let text_y = row_y + (16.0 - BitmapFont::line_height(scale)) / 2.0;

            BitmapFont::push_text(batcher, label_x, text_y, label, colors[i], scale);

            let val_text = format!("{}", values[i]);
            BitmapFont::push_text(batcher, value_x, text_y, &val_text, COL_TEXT_PARCHMENT, scale);
        }

        // Secondary stats.
        let sep_y = stat_start_y + 4.0 * CHAR_STAT_ROW_H + 8.0;
        let sec_start_y = sep_y + 12.0;
        let sec_labels = ["DEF", "DMG", "AR"];
        let sec_values = [
            format!("{}", self.defense),
            format!("{}-{}", self.damage_min, self.damage_max),
            format!("{}", self.attack_rating),
        ];

        for (i, label) in sec_labels.iter().enumerate() {
            let row_y = sec_start_y + i as f32 * CHAR_STAT_ROW_H;
            let text_y = row_y + (16.0 - BitmapFont::line_height(scale)) / 2.0;

            BitmapFont::push_text(batcher, label_x, text_y, label, COL_TEXT_PARCHMENT, scale);
            BitmapFont::push_text(batcher, value_x, text_y, &sec_values[i], COL_TEXT_PARCHMENT, scale);
        }

        // Stat points: draw "+" labels and stat points count.
        if self.stat_points_available > 0 {
            let stat_bar_w = 160.0_f32;
            let btn_size = 16.0_f32;
            let stat_x = panel_x + 90.0;
            let btn_x = stat_x + stat_bar_w + 6.0;
            for i in 0..4_usize {
                let row_y = stat_start_y + i as f32 * CHAR_STAT_ROW_H;
                let text_y = row_y + (btn_size - BitmapFont::line_height(scale)) / 2.0;
                let text_x = btn_x + (btn_size - BitmapFont::char_width(scale)) / 2.0;
                BitmapFont::push_text(batcher, text_x, text_y, "+", [1.0, 1.0, 1.0, 1.0], scale);
            }

            let sp_y = sec_start_y + 3.0 * CHAR_STAT_ROW_H + 8.0;
            let sp_text = format!("{} stat points available", self.stat_points_available);
            BitmapFont::push_text(batcher, panel_x + 12.0, sp_y + 3.0, &sp_text, [1.0, 0.85, 0.2, 1.0], scale);
        }
    }

    /// Render automap overlay text.
    pub fn draw_automap_label(&self, batcher: &mut SpriteBatcher) {
        use crate::bitmap_font::BitmapFont;

        let label = "AUTOMAP (Tab)";
        let scale = 1.5_f32;
        let lw = label.len() as f32 * BitmapFont::char_width(scale);
        BitmapFont::push_text(
            batcher,
            (self.screen_w - lw) / 2.0,
            4.0,
            label,
            COL_TEXT_PARCHMENT,
            scale,
        );
    }

    /// Render quest log panel text (title + quest names).
    pub fn draw_quest_log_text(&self, batcher: &mut SpriteBatcher) {
        use crate::bitmap_font::BitmapFont;

        let panel_w = 280.0_f32;
        let panel_h = 320.0_f32;
        let panel_x = self.screen_w - panel_w - MARGIN;
        let panel_y = (self.screen_h - PANEL_H - XP_BAR_H - panel_h) / 2.0;

        // Title.
        BitmapFont::push_text(batcher, panel_x + 8.0, panel_y + 6.0, "QUESTS (Q)", COL_TEXT_PARCHMENT, 2.0);

        // Act subtitle.
        BitmapFont::push_text(batcher, panel_x + panel_w - 90.0, panel_y + 10.0, "Act I", COL_TEXT_DIM, 1.5);

        // Quest names.
        let quests = [
            "Den of Evil",
            "Sisters' Burial Ground",
            "The Search for Cain",
            "The Forgotten Tower",
            "Tools of the Trade",
            "Sisters to the Slaughter",
        ];
        let quest_start_y = panel_y + INV_HEADER_H + 8.0;
        let quest_row_h = 36.0_f32;
        let scale = 1.5_f32;

        for (i, quest_name) in quests.iter().enumerate() {
            let qy = quest_start_y + i as f32 * quest_row_h;
            let text_y = qy + (quest_row_h - BitmapFont::line_height(scale)) / 2.0;
            let completed = i < 2;
            let text_color = if completed {
                [0.5, 0.5, 0.4, 0.7] // Dim for completed quests
            } else {
                COL_TEXT_PARCHMENT
            };
            BitmapFont::push_text(batcher, panel_x + 26.0, text_y, quest_name, text_color, scale);
        }
    }

    /// Render monster name tags and level indicators above monsters.
    pub fn draw_monster_overlay_text(&self, batcher: &mut SpriteBatcher) {
        use crate::bitmap_font::BitmapFont;

        let bar_h = 4.0_f32;
        let bar_offset_y = 6.0;
        let name_scale = 1.0_f32;

        for overlay in &self.monster_overlays {
            let bar_y = overlay.screen_y - bar_offset_y - bar_h;
            let tag_h = 12.0_f32;
            let tag_y = bar_y - tag_h - 1.0;

            // Monster name.
            let char_w = BitmapFont::char_width(name_scale);
            let name_w = overlay.name.len() as f32 * char_w;
            let tag_w = name_w + 8.0;
            let tag_x = overlay.screen_x - tag_w / 2.0;
            let text_y = tag_y + (tag_h - BitmapFont::line_height(name_scale)) / 2.0;

            BitmapFont::push_text(batcher, tag_x + 4.0, text_y, &overlay.name, COL_TEXT_MONSTER, name_scale);

            // Level indicator (small, to the right).
            let level_text = format!("L{}", overlay.level);
            let lx = tag_x + tag_w + 2.0;
            BitmapFont::push_text(batcher, lx, text_y, &level_text, COL_TEXT_DIM, name_scale);
        }
    }

    /// Render menu button labels (C/I/K/Q).
    pub fn draw_menu_button_text(&self, batcher: &mut SpriteBatcher) {
        use crate::bitmap_font::BitmapFont;

        let panel_y = self.screen_h - PANEL_H - XP_BAR_H;
        let btn_w = 22.0_f32;
        let btn_h = 20.0_f32;
        let btn_gap = 3.0_f32;
        let btn_y = panel_y + PANEL_H - btn_h - 4.0;

        let left_orb_right = ORB_INSET + ORB_DIAMETER + ACTIVE_SKILL_GAP + ACTIVE_SKILL_SIZE + 8.0;
        let right_orb_left = self.screen_w - ORB_INSET - ORB_DIAMETER - ACTIVE_SKILL_GAP - ACTIVE_SKILL_SIZE - 8.0;

        let scale = 1.5_f32;
        let lh = BitmapFont::line_height(scale);

        // Left buttons: C, I
        let left_labels = ["C", "I"];
        let left_start_x = left_orb_right + 6.0;
        for (i, label) in left_labels.iter().enumerate() {
            let bx = left_start_x + i as f32 * (btn_w + btn_gap);
            let lw = BitmapFont::char_width(scale);
            BitmapFont::push_text(
                batcher,
                bx + (btn_w - lw) / 2.0,
                btn_y + (btn_h - lh) / 2.0,
                label,
                COL_TEXT_PARCHMENT,
                scale,
            );
        }

        // Right buttons: K, Q
        let right_labels = ["K", "Q"];
        let right_start_x = right_orb_left - 6.0 - 2.0 * btn_w - btn_gap;
        for (i, label) in right_labels.iter().enumerate() {
            let bx = right_start_x + i as f32 * (btn_w + btn_gap);
            let lw = BitmapFont::char_width(scale);
            BitmapFont::push_text(
                batcher,
                bx + (btn_w - lw) / 2.0,
                btn_y + (btn_h - lh) / 2.0,
                label,
                COL_TEXT_PARCHMENT,
                scale,
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Orb drawing helpers
// ---------------------------------------------------------------------------

/// Draw a filled orb (circle approximation) using vertical slices.
///
/// `cx`, `cy` = centre, `radius` = radius, `ratio` = fill level 0.0..1.0.
/// The orb is drawn as background first, then filled from bottom up.
fn draw_orb(
    batcher: &mut SpriteBatcher,
    cx: f32,
    cy: f32,
    radius: f32,
    ratio: f32,
    bg_color: [f32; 4],
    fg_color: [f32; 4],
) {
    let slice_w = (radius * 2.0) / ORB_SLICES as f32;

    for i in 0..ORB_SLICES {
        // X offset from left edge of the orb bounding box.
        let local_x = i as f32 * slice_w;
        // Distance from circle centre along X.
        let dx = local_x + slice_w / 2.0 - radius;
        // Half-height of the circle at this X.
        let dy_sq = radius * radius - dx * dx;
        if dy_sq <= 0.0 {
            continue;
        }
        let half_h = dy_sq.sqrt();

        let x = cx - radius + local_x;
        let y_top = cy - half_h;
        let slice_h = half_h * 2.0;

        // Background slice.
        batcher.push(x, y_top, slice_w, slice_h, UV_FULL, bg_color);

        // Foreground slice (fill from bottom).
        if ratio > 0.0 {
            let fill_h = slice_h * ratio;
            let fill_y = y_top + slice_h - fill_h;
            batcher.push(x, fill_y, slice_w, fill_h, UV_FULL, fg_color);
        }
    }
}

/// Draw a ring around an orb using thin border quads on the perimeter.
fn draw_orb_ring(
    batcher: &mut SpriteBatcher,
    cx: f32,
    cy: f32,
    radius: f32,
    color: [f32; 4],
) {
    let ring_thickness = 2.0_f32;
    let outer_r = radius + ring_thickness / 2.0;
    let segments = ORB_SLICES * 2;
    let slice_w = (outer_r * 2.0) / segments as f32;

    for i in 0..segments {
        let local_x = i as f32 * slice_w;
        let dx = local_x + slice_w / 2.0 - outer_r;

        // Outer circle half-height.
        let outer_sq = outer_r * outer_r - dx * dx;
        if outer_sq <= 0.0 {
            continue;
        }
        let outer_half = outer_sq.sqrt();

        // Inner circle half-height.
        let inner_r = radius - ring_thickness / 2.0;
        let inner_sq = inner_r * inner_r - dx * dx;
        let inner_half = if inner_sq > 0.0 {
            inner_sq.sqrt()
        } else {
            0.0
        };

        let x = cx - outer_r + local_x;

        // Top arc.
        let top_y = cy - outer_half;
        let top_h = outer_half - inner_half;
        if top_h > 0.5 {
            batcher.push(x, top_y, slice_w, top_h, UV_FULL, color);
        }

        // Bottom arc.
        let bot_y = cy + inner_half;
        let bot_h = outer_half - inner_half;
        if bot_h > 0.5 {
            batcher.push(x, bot_y, slice_w, bot_h, UV_FULL, color);
        }
    }
}

/// Draw a decorative stone/metal frame around the orb (D2-style).
///
/// Renders two concentric rings: a thick outer "stone" ring and a thinner
/// inner "metal" ring, creating the ornate frame effect from D2.
fn draw_orb_frame(
    batcher: &mut SpriteBatcher,
    cx: f32,
    cy: f32,
    radius: f32,
) {
    // Outer stone frame (thicker, darker).
    let outer_r = radius + 5.0;
    let inner_r = radius + 1.0;
    let segments = ORB_SLICES * 2;
    let slice_w = (outer_r * 2.0) / segments as f32;

    for i in 0..segments {
        let local_x = i as f32 * slice_w;
        let dx = local_x + slice_w / 2.0 - outer_r;

        let outer_sq = outer_r * outer_r - dx * dx;
        if outer_sq <= 0.0 {
            continue;
        }
        let outer_half = outer_sq.sqrt();

        let inner_sq = inner_r * inner_r - dx * dx;
        let inner_half = if inner_sq > 0.0 { inner_sq.sqrt() } else { 0.0 };

        let x = cx - outer_r + local_x;

        // Top arc.
        let top_y = cy - outer_half;
        let top_h = outer_half - inner_half;
        if top_h > 0.5 {
            batcher.push(x, top_y, slice_w, top_h, UV_FULL, COL_ORB_FRAME_OUTER);
        }

        // Bottom arc.
        let bot_y = cy + inner_half;
        let bot_h = outer_half - inner_half;
        if bot_h > 0.5 {
            batcher.push(x, bot_y, slice_w, bot_h, UV_FULL, COL_ORB_FRAME_OUTER);
        }
    }

    // Inner metallic ring (thinner, brighter).
    draw_orb_ring(batcher, cx, cy, radius + 1.0, COL_ORB_FRAME_INNER);
}

/// Draw a horizontal line at the orb's fill level to separate filled/empty.
fn draw_orb_fill_line(
    batcher: &mut SpriteBatcher,
    cx: f32,
    cy: f32,
    radius: f32,
    ratio: f32,
) {
    if ratio <= 0.01 || ratio >= 0.99 {
        return; // No visible line at extremes.
    }

    // The fill line y position (top of fill).
    let fill_y = cy + radius - radius * 2.0 * ratio;

    // Compute circle width at fill_y.
    let dy = fill_y - cy;
    let dx_sq = radius * radius - dy * dy;
    if dx_sq <= 0.0 {
        return;
    }
    let half_w = dx_sq.sqrt();
    let line_x = cx - half_w;
    let line_w = half_w * 2.0;

    batcher.push(line_x, fill_y - 0.5, line_w, 1.0, UV_FULL, COL_ORB_FILL_LINE);
}

/// Draw a highlight/shine effect on the top-left of the orb.
fn draw_orb_highlight(
    batcher: &mut SpriteBatcher,
    cx: f32,
    cy: f32,
    radius: f32,
) {
    // Small elliptical highlight in the top-left quadrant.
    let highlight_r = radius * 0.35;
    let hx = cx - radius * 0.25;
    let hy = cy - radius * 0.35;
    let slices = 12_usize;
    let slice_w = (highlight_r * 2.0) / slices as f32;

    for i in 0..slices {
        let local_x = i as f32 * slice_w;
        let dx = local_x + slice_w / 2.0 - highlight_r;
        let dy_sq = highlight_r * highlight_r - dx * dx;
        if dy_sq <= 0.0 {
            continue;
        }
        let half_h = dy_sq.sqrt() * 0.6; // Flattened ellipse.

        let x = hx - highlight_r + local_x;
        let y_top = hy - half_h;
        let slice_h = half_h * 2.0;

        // Fade-out alpha based on distance from centre.
        let dist = (dx / highlight_r).abs();
        let alpha = (1.0 - dist) * COL_ORB_HIGHLIGHT[3];
        let color = [COL_ORB_HIGHLIGHT[0], COL_ORB_HIGHLIGHT[1], COL_ORB_HIGHLIGHT[2], alpha];
        batcher.push(x, y_top, slice_w, slice_h, UV_FULL, color);
    }
}

// ---------------------------------------------------------------------------
// White texture helper
// ---------------------------------------------------------------------------

/// Create a small 4x4 solid-white GPU texture for drawing coloured quads.
///
/// The resulting [`GpuTexture`] is used for all GUI elements -- the visible
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

    // -- handle_input: Num1 -> UseBeltPotion(0) (D2-style) ----------------

    #[test]
    fn handle_input_num1_uses_belt_potion_0() {
        let mut gui = GameGui::new(800.0, 600.0);
        let event = InputEvent::KeyDown { key: KeyCode::Num1 };
        let action = gui.handle_input(&event);
        assert_eq!(action, GuiAction::UseBeltPotion(0));
    }

    // -- handle_input: F1 -> UseSkill(0) (D2-style) -----------------------

    #[test]
    fn handle_input_f1_uses_skill_0() {
        let mut gui = GameGui::new(800.0, 600.0);
        let event = InputEvent::KeyDown { key: KeyCode::F1 };
        let action = gui.handle_input(&event);
        assert_eq!(action, GuiAction::UseSkill(0));
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
        // Account for equipment area between header and grid.
        let (panel_x, panel_y) = gui.inventory_panel_origin();
        let equip_area_h = EQUIP_SLOT_SIZE * 2.0 + EQUIP_SLOT_GAP + 8.0 + 4.0 + 2.0;
        let grid_origin_y = panel_y + INV_HEADER_H + equip_area_h;
        let slot_center_x = panel_x + INV_PANEL_PAD + INV_SLOT_SIZE / 2.0;
        let slot_center_y = grid_origin_y + INV_PANEL_PAD + INV_SLOT_SIZE / 2.0;

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

        // Click above the control panel area.
        gui.handle_input(&InputEvent::MouseMove { x: 400.0, y: 100.0 });

        let click = InputEvent::MouseButtonEvent {
            button: MouseButton::Left,
            pressed: true,
        };
        let action = gui.handle_input(&click);
        assert_eq!(action, GuiAction::ClickWorld(400.0, 100.0));
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

    // -- hp_display and mana_display --------------------------------------

    #[test]
    fn hp_display_returns_current_values() {
        let mut gui = GameGui::new(800.0, 600.0);
        gui.update_from_world(42, 100, 30, 80, 0, 100, 1, 0);
        assert_eq!(gui.hp_display(), (42, 100));
    }

    #[test]
    fn mana_display_returns_current_values() {
        let mut gui = GameGui::new(800.0, 600.0);
        gui.update_from_world(100, 100, 25, 60, 0, 100, 1, 0);
        assert_eq!(gui.mana_display(), (25, 60));
    }

    // -- skill panel visibility ------------------------------------------

    #[test]
    fn skill_panel_toggled_by_k() {
        let mut gui = GameGui::new(800.0, 600.0);
        assert!(!gui.is_skill_panel_open());
        gui.toggle_skills();
        assert!(gui.is_skill_panel_open());
        gui.toggle_skills();
        assert!(!gui.is_skill_panel_open());
    }

    // -- minimap update ---------------------------------------------------

    #[test]
    fn update_minimap_sets_data() {
        let mut gui = GameGui::new(800.0, 600.0);
        gui.update_minimap((10.0, 15.0), &[(20.0, 25.0), (5.0, 8.0)]);
        assert!((gui.minimap_player.0 - 10.0).abs() < f32::EPSILON);
        assert!((gui.minimap_player.1 - 15.0).abs() < f32::EPSILON);
        assert_eq!(gui.minimap_monsters.len(), 2);
    }

    // -- tooltip generation -----------------------------------------------

    #[test]
    fn tooltip_appears_on_inventory_hover() {
        let mut gui = GameGui::new(800.0, 600.0);
        gui.toggle_inventory();

        // Place an item in slot 0.
        let slots = vec![Some(("Dragon Sword".to_string(), [1.0, 0.5, 0.0, 1.0], 1))];
        gui.update_inventory(&slots);

        // Hover over slot 0 (account for equipment area).
        let (panel_x, panel_y) = gui.inventory_panel_origin();
        let equip_area_h = EQUIP_SLOT_SIZE * 2.0 + EQUIP_SLOT_GAP + 8.0 + 4.0 + 2.0;
        let grid_origin_y = panel_y + INV_HEADER_H + equip_area_h;
        let hover_x = panel_x + INV_PANEL_PAD + INV_SLOT_SIZE / 2.0;
        let hover_y = grid_origin_y + INV_PANEL_PAD + INV_SLOT_SIZE / 2.0;
        gui.handle_input(&InputEvent::MouseMove {
            x: hover_x as f64,
            y: hover_y as f64,
        });

        assert_eq!(gui.tooltip_text(), Some("Dragon Sword"));
    }

    // -- control panel click is consumed ----------------------------------

    #[test]
    fn click_on_control_panel_is_consumed() {
        let mut gui = GameGui::new(800.0, 600.0);
        // Click at the very bottom of the screen (within control panel).
        let panel_y = 600.0 - PANEL_H - XP_BAR_H;
        gui.handle_input(&InputEvent::MouseMove {
            x: 400.0,
            y: (panel_y + 10.0) as f64,
        });
        let click = InputEvent::MouseButtonEvent {
            button: MouseButton::Left,
            pressed: true,
        };
        let action = gui.handle_input(&click);
        assert_eq!(action, GuiAction::None);
    }
}
