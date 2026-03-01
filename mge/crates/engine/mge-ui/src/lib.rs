//! # mge-ui
//!
//! Game UI layer for the Miyukini Game Engine -- Diablo II-style dark medieval interface.
//!
//! Built on **egui 0.28** + wgpu. Provides HUD, inventory grid, character panel,
//! skill tree, tooltips, NPC dialogs, and full menu screens (main menu, character
//! select, lobby browser, pause menu).

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Modules
// ---------------------------------------------------------------------------

mod theme;
mod hud;
mod inventory;
mod character;
mod skill_tree;
mod dialog;
mod tooltip;
mod minimap;

/// Menu screens (main menu, character select, lobby browser, pause).
pub mod menus;

// ---------------------------------------------------------------------------
// Re-exports
// ---------------------------------------------------------------------------

pub use theme::{apply_d2_theme, D2Colors, item_quality_color};
pub use hud::{draw_hud, BeltSlotData, HudData, SkillSlotData};
pub use inventory::{draw_inventory, EquipSlots, UiItem, CELL_SIZE, GRID_COLS, GRID_ROWS};
pub use character::{draw_character_panel, CharacterData};
pub use skill_tree::draw_skill_tree;
pub use menus::main_menu::{draw_main_menu, MainMenuAction};
pub use menus::char_select::draw_char_select;
pub use menus::lobby_browser::draw_lobby_browser;
pub use menus::pause_menu::{draw_pause_menu, PauseAction};
pub use dialog::draw_npc_dialog;
pub use tooltip::{draw_tooltip, ItemTooltipData};
pub use minimap::draw_minimap;

// ---------------------------------------------------------------------------
// UiScreen
// ---------------------------------------------------------------------------

/// Which top-level screen is currently active.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UiScreen {
    /// Main menu (title screen).
    MainMenu,
    /// Character selection screen.
    CharacterSelect,
    /// Multiplayer lobby browser.
    LobbyBrowser,
    /// Active gameplay.
    InGame,
    /// Game is paused.
    Paused,
}

// ---------------------------------------------------------------------------
// OpenPanels
// ---------------------------------------------------------------------------

/// In-game panels that can be open simultaneously.
///
/// Each field represents a toggleable panel overlay. Using bools is intentional
/// here because any combination of panels can be open at the same time.
#[derive(Debug, Clone, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct OpenPanels {
    /// Inventory panel (`I` key).
    pub inventory: bool,
    /// Character stats panel (`C` key).
    pub character: bool,
    /// Skill tree panel (`T` key).
    pub skill_tree: bool,
    /// Quest log panel (`Q` key).
    pub quest_log: bool,
    /// Enlarged minimap.
    pub minimap_big: bool,
}

// ---------------------------------------------------------------------------
// UiState
// ---------------------------------------------------------------------------

/// Global UI state shared across all panels and screens.
pub struct UiState {
    /// Current top-level screen.
    pub screen: UiScreen,
    /// Which in-game panels are open.
    pub panels: OpenPanels,
    /// Item ID currently hovered (for tooltip display).
    pub hovered_item: Option<String>,
    /// Skill ID currently hovered (for tooltip display).
    pub hovered_skill: Option<String>,
    /// Global error message with remaining display duration in seconds.
    pub error_message: Option<(String, f32)>,
    /// NPC dialog text currently displayed.
    pub npc_dialog: Option<String>,
}

impl UiState {
    /// Create a new `UiState` starting at the main menu.
    pub fn new() -> Self {
        Self {
            screen: UiScreen::MainMenu,
            panels: OpenPanels::default(),
            hovered_item: None,
            hovered_skill: None,
            error_message: None,
            npc_dialog: None,
        }
    }

    /// Toggle the inventory panel.
    pub fn toggle_inventory(&mut self) {
        self.panels.inventory = !self.panels.inventory;
    }

    /// Toggle the character stats panel.
    pub fn toggle_character(&mut self) {
        self.panels.character = !self.panels.character;
    }

    /// Toggle the skill tree panel.
    pub fn toggle_skill_tree(&mut self) {
        self.panels.skill_tree = !self.panels.skill_tree;
    }

    /// Show a transient error message for the given duration.
    pub fn show_error(&mut self, msg: &str, duration_secs: f32) {
        self.error_message = Some((msg.to_string(), duration_secs));
    }

    /// Tick down the error message timer. Clears the message when expired.
    pub fn tick_error(&mut self, dt: f32) {
        if let Some((_, ref mut t)) = self.error_message {
            *t -= dt;
            if *t <= 0.0 {
                self.error_message = None;
            }
        }
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self::new()
    }
}
