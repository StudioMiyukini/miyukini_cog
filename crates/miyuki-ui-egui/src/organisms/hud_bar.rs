// @id: MUIE-HudBar @do: hud-bar-organism @role: organism @layer: 6 @human: miyuk

//! HUD bar organism -- the complete bottom-of-screen gameplay interface.
//!
//! Layout (left to right):
//! - Life orb
//! - Left skill icon
//! - Belt (4 columns)
//! - Menu buttons (inventory, skills, quest, map, chat)
//! - Right skill icon
//! - Mana orb
//! - XP bar (full width, very bottom)

use crate::atoms::belt_slot::BeltSlot;
use crate::atoms::orb::{Orb, OrbType};
use crate::atoms::skill_icon::{SkillIcon, SkillState};
use crate::convert::rgba_to_color32;
use egui::{Color32, Context, Pos2, Rect, Vec2};
use miyuki_ui_tokens::palette::d2::{D2MiscColors, D2_PALETTE};

/// Data needed to render the complete HUD bar.
pub struct HudBarData {
    /// Current life.
    pub life_cur: f32,
    /// Maximum life.
    pub life_max: f32,
    /// Current mana.
    pub mana_cur: f32,
    /// Maximum mana.
    pub mana_max: f32,
    /// Character level.
    pub level: i32,
    /// XP progress toward next level (0.0-1.0).
    pub xp_pct: f32,
    /// Gold held.
    pub gold: i64,
    /// Left skill name.
    pub left_skill: String,
    /// Right skill name.
    pub right_skill: String,
    /// Belt item names and quantities (4 slots).
    pub belt: [BeltItemData; 4],
}

/// Belt item data.
#[derive(Debug, Clone, Default)]
pub struct BeltItemData {
    /// Item name (empty = no item).
    pub name: String,
    /// Stack quantity.
    pub quantity: u32,
}

/// The HUD bar organism.
pub struct HudBar;

impl HudBar {
    /// Draw the full HUD bar at the bottom of the screen.
    pub fn show(ctx: &Context, data: &HudBarData, screen_w: f32, screen_h: f32) {
        // Main HUD panel (bottom bar)
        egui::Area::new("hud_bottom_bar".into())
            .fixed_pos(Pos2::new(0.0, screen_h - 80.0))
            .show(ctx, |ui| {
                ui.set_width(screen_w);

                // Background
                let rect = ui.max_rect();
                let bg = Color32::from_rgba_premultiplied(10, 8, 5, 230);
                let border = rgba_to_color32(&D2_PALETTE.border_default);
                ui.painter().rect_filled(rect, 0.0, bg);
                ui.painter()
                    .rect_stroke(rect, 0.0, egui::Stroke::new(1.0, border));

                ui.horizontal(|ui| {
                    // Life orb
                    Orb::new(data.life_cur, data.life_max, OrbType::Life)
                        .diameter(64.0)
                        .show(ui);

                    ui.add_space(8.0);

                    // Left skill
                    SkillIcon::new(&data.left_skill, SkillState::Active)
                        .size(40.0)
                        .show(ui);

                    ui.add_space(4.0);

                    // Belt (4 slots)
                    ui.vertical(|ui| {
                        for (i, belt_item) in data.belt.iter().enumerate() {
                            let key = (i + 1).to_string();
                            if belt_item.quantity > 0 {
                                BeltSlot::new(&key)
                                    .item(&belt_item.name, belt_item.quantity)
                                    .show(ui);
                            } else {
                                BeltSlot::new(&key).show(ui);
                            }
                        }
                    });

                    ui.add_space(4.0);

                    // Right skill
                    SkillIcon::new(&data.right_skill, SkillState::Active)
                        .size(40.0)
                        .show(ui);

                    ui.add_space(8.0);

                    // Mana orb
                    Orb::new(data.mana_cur, data.mana_max, OrbType::Mana)
                        .diameter(64.0)
                        .show(ui);
                });
            });

        // XP bar (very bottom)
        egui::Area::new("hud_xp_bar".into())
            .fixed_pos(Pos2::new(0.0, screen_h - 6.0))
            .show(ctx, |ui| {
                let xp_width = screen_w * data.xp_pct.clamp(0.0, 1.0);
                let bar_rect = Rect::from_min_size(Pos2::ZERO, Vec2::new(xp_width, 5.0));
                let gold = rgba_to_color32(&D2MiscColors::XP_BAR);
                ui.painter().rect_filled(bar_rect, 0.0, gold);
            });

        // Level indicator (bottom-left)
        let level_label = format!("Nv {}", data.level);
        egui::Area::new("hud_level_indicator".into())
            .fixed_pos(Pos2::new(6.0, screen_h - 18.0))
            .show(ctx, |ui| {
                let gold = rgba_to_color32(&D2_PALETTE.accent_primary);
                ui.label(egui::RichText::new(&level_label).color(gold).size(11.0));
            });

        // Gold display (bottom-right)
        let gold_label = format!("Or : {}", data.gold);
        egui::Area::new("hud_gold_display".into())
            .fixed_pos(Pos2::new(screen_w - 120.0, screen_h - 18.0))
            .show(ctx, |ui| {
                let gold = rgba_to_color32(&D2_PALETTE.accent_primary);
                ui.label(egui::RichText::new(&gold_label).color(gold).size(11.0));
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hud_bar_data_creation() {
        let data = HudBarData {
            life_cur: 500.0,
            life_max: 1000.0,
            mana_cur: 200.0,
            mana_max: 400.0,
            level: 85,
            xp_pct: 0.45,
            gold: 12345,
            left_skill: "Teleport".to_string(),
            right_skill: "Blizzard".to_string(),
            belt: Default::default(),
        };
        assert_eq!(data.level, 85);
        assert!((data.xp_pct - 0.45).abs() < f32::EPSILON);
    }
}
