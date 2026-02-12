//! Modules UI Dioxus pour Lord of the Castle.
//!
//! Organisation modulaire des composants d'interface :
//! - main_menu : écran titre et sélection de slot
//! - character_creation : écran de création de personnage
//! - game_screen : écran de jeu principal (orchestration)
//! - game_header : barre d'informations supérieure
//! - game_footer : barre d'actions inférieure
//! - sidebar : panneaux latéraux (préparation, combat)
//! - game_area : zone de rendu des entités
//! - overlays : panneaux modaux (stats, skills, inventaire, victoire, défaite)
//! - entity_render : rendu des entités de jeu
//!
//! @id: lord_of_the_castle.ui
//! @do: organize_ui_components
//! @role: ui
//! @layer: ui
//! @human: Point d'entrée des modules UI pour Lord of the Castle

pub mod main_menu;
pub mod character_creation;
pub mod game_screen;
pub mod game_header;
pub mod game_footer;
pub mod sidebar;
pub mod game_area;
pub mod overlays;
pub mod entity_render;

// Réexports publics
pub use main_menu::MainMenu;
pub use character_creation::CharacterCreationScreen;
pub use game_screen::GameScreen;
pub use game_header::GameHeader;
pub use game_footer::GameFooter;
pub use sidebar::{PreparationSidebar, BattleSidebar};
pub use game_area::{GameArea, get_cursor_world, set_cursor_world};
pub use overlays::{
    Panel, panel_button,
    StatsPanel, SkillsPanel, InventoryPanel,
    GameOverOverlay, WaveWonOverlay,
};
