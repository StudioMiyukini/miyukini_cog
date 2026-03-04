// @id: MUIE-SkillIcon @do: skill-icon-atom @role: atom @layer: 6 @human: miyuk

//! Skill icon atom -- displays a skill with its state (active/inactive/locked).
//!
//! Used in the skill hotbar and skill tree. The icon is a square frame
//! with state-dependent coloring and an optional level indicator.

use egui::{Color32, Response, Rounding, Stroke, Ui, Vec2};
use miyuki_ui_tokens::palette::d2::{D2MiscColors, D2_PALETTE};
use crate::convert::rgba_to_color32;

/// Skill visual state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillState {
    /// Skill is available and currently selected.
    Active,
    /// Skill is available but not selected.
    Inactive,
    /// Skill is locked (prerequisites not met).
    Locked,
    /// Skill is on cooldown.
    Cooldown,
}

/// A skill icon atom.
pub struct SkillIcon<'a> {
    /// Skill name (shown in tooltip).
    pub name: &'a str,
    /// Current state.
    pub state: SkillState,
    /// Skill level (0 = unlearned).
    pub level: u32,
    /// Maximum skill level.
    pub max_level: u32,
    /// Cooldown progress (0.0 = ready, 1.0 = full cooldown). Only used if `state == Cooldown`.
    pub cooldown_pct: f32,
    /// Icon size in logical pixels.
    pub size: f32,
}

impl<'a> SkillIcon<'a> {
    /// Create a new skill icon.
    pub fn new(name: &'a str, state: SkillState) -> Self {
        Self {
            name,
            state,
            level: 0,
            max_level: 20,
            cooldown_pct: 0.0,
            size: 40.0,
        }
    }

    /// Set the skill level.
    pub fn level(mut self, level: u32) -> Self {
        self.level = level;
        self
    }

    /// Set the max skill level.
    pub fn max_level(mut self, max: u32) -> Self {
        self.max_level = max;
        self
    }

    /// Set cooldown progress.
    pub fn cooldown(mut self, pct: f32) -> Self {
        self.cooldown_pct = pct.clamp(0.0, 1.0);
        self
    }

    /// Set icon size.
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Draw the skill icon and return the egui [`Response`].
    pub fn show(self, ui: &mut Ui) -> Response {
        let icon_size = Vec2::splat(self.size);
        let (response, painter) = ui.allocate_painter(icon_size, egui::Sense::click());
        let rect = response.rect;

        // Background color based on state
        let bg_color = match self.state {
            SkillState::Active => Color32::from_rgb(60, 50, 30),
            SkillState::Inactive => Color32::from_rgb(40, 35, 25),
            SkillState::Locked => Color32::from_rgb(20, 18, 14),
            SkillState::Cooldown => Color32::from_rgb(30, 28, 22),
        };

        painter.rect_filled(rect, Rounding::same(3.0), bg_color);

        // Icon placeholder (in production, draw from sprite atlas)
        if self.state != SkillState::Locked {
            let icon_rect = rect.shrink(4.0);
            let icon_color = match self.state {
                SkillState::Active => Color32::from_rgb(100, 90, 60),
                SkillState::Inactive => Color32::from_rgb(70, 65, 50),
                SkillState::Cooldown => Color32::from_rgb(50, 45, 35),
                SkillState::Locked => unreachable!(),
            };
            painter.rect_filled(icon_rect, Rounding::same(2.0), icon_color);
        }

        // Cooldown overlay
        if self.state == SkillState::Cooldown && self.cooldown_pct > 0.0 {
            let cd_height = self.size * self.cooldown_pct;
            let cd_rect = egui::Rect::from_min_size(
                rect.min,
                Vec2::new(self.size, cd_height),
            );
            painter.rect_filled(
                cd_rect,
                Rounding::same(3.0),
                Color32::from_rgba_premultiplied(0, 0, 0, 160),
            );
        }

        // Border
        let border_color = match self.state {
            SkillState::Active => rgba_to_color32(&D2MiscColors::SKILL_ACTIVE),
            SkillState::Inactive | SkillState::Cooldown => rgba_to_color32(&D2_PALETTE.border_default),
            SkillState::Locked => rgba_to_color32(&D2_PALETTE.border_subtle),
        };
        let border_width = if self.state == SkillState::Active { 2.0 } else { 1.0 };
        painter.rect_stroke(rect, Rounding::same(3.0), Stroke::new(border_width, border_color));

        // Level indicator (bottom-right)
        if self.level > 0 {
            let level_text = self.level.to_string();
            let text_color = if self.level >= self.max_level {
                rgba_to_color32(&D2_PALETTE.text_high)
            } else {
                rgba_to_color32(&D2_PALETTE.accent_primary)
            };
            painter.text(
                rect.right_bottom() + egui::vec2(-3.0, -2.0),
                egui::Align2::RIGHT_BOTTOM,
                &level_text,
                egui::FontId::proportional(9.0),
                text_color,
            );
        }

        // Tooltip
        let level_str = format!("{}/{}", self.level, self.max_level);
        let tooltip = format!("{}\nNiveau : {level_str}", self.name);
        response.on_hover_text(tooltip)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skill_icon_builder() {
        let icon = SkillIcon::new("Fireball", SkillState::Active)
            .level(10)
            .max_level(20)
            .size(48.0);
        assert_eq!(icon.name, "Fireball");
        assert_eq!(icon.state, SkillState::Active);
        assert_eq!(icon.level, 10);
        assert_eq!(icon.max_level, 20);
        assert!((icon.size - 48.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_cooldown_clamp() {
        let icon = SkillIcon::new("Test", SkillState::Cooldown).cooldown(1.5);
        assert!((icon.cooldown_pct - 1.0).abs() < f32::EPSILON);
    }
}
