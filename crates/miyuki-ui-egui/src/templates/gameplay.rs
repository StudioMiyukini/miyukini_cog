// @id: MUIE-TmplGameplay @do: gameplay-template @role: template @layer: 6 @human: miyuk

//! Gameplay layout template -- HUD + game area + overlay panels.
//!
//! This template orchestrates the in-game UI: the HUD bar is always at the
//! bottom, panels (inventory, character, skill tree) overlay the game area,
//! and the minimap sits in the top-right corner.

use crate::organisms::hud_bar::{HudBar, HudBarData};
use crate::organisms::minimap::{Minimap, MinimapEntity};
use egui::Context;

/// Configuration for the gameplay layout.
pub struct GameplayConfig {
    /// Screen width.
    pub screen_w: f32,
    /// Screen height.
    pub screen_h: f32,
    /// Minimap size.
    pub minimap_size: f32,
}

impl Default for GameplayConfig {
    fn default() -> Self {
        Self {
            screen_w: 800.0,
            screen_h: 600.0,
            minimap_size: 120.0,
        }
    }
}

/// The gameplay layout template.
pub struct GameplayLayout;

impl GameplayLayout {
    /// Draw the gameplay HUD and minimap.
    ///
    /// Panels (inventory, character, skill tree, etc.) should be drawn
    /// separately by the caller based on their open/closed state.
    pub fn show(
        ctx: &Context,
        config: &GameplayConfig,
        hud_data: &HudBarData,
        minimap_entities: &[MinimapEntity],
    ) {
        // HUD bar at the bottom
        HudBar::show(ctx, hud_data, config.screen_w, config.screen_h);

        // Minimap at top-right
        if !minimap_entities.is_empty() {
            Minimap::show(ctx, minimap_entities, config.screen_w, config.minimap_size);
        }
    }
}
