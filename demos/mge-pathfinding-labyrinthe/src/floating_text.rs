//! Texte flottant style Action RPG / Ragnarok Online

use crate::grid::Vec2;

const DODGE_DURATION: f32 = 1.0;
const DAMAGE_DURATION: f32 = 1.0;
const FLOAT_SPEED: f32 = 25.0;

#[derive(Debug, Clone)]
pub enum FloatingTextKind {
    Dodge,
    Damage(i32),
}

/// Texte flottant qui monte puis disparaît
#[derive(Debug, Clone)]
pub struct FloatingText {
    pub base_pos: Vec2,
    pub offset_y: f32,
    pub timer: f32,
    pub kind: FloatingTextKind,
}

impl FloatingText {
    pub fn new_dodge(entity_pos: Vec2, half_size: f32) -> Self {
        Self {
            base_pos: Vec2::new(entity_pos.x, entity_pos.y - half_size - 2.0),
            offset_y: 0.0,
            timer: DODGE_DURATION,
            kind: FloatingTextKind::Dodge,
        }
    }

    pub fn new_damage(entity_pos: Vec2, half_size: f32, dmg: i32) -> Self {
        Self {
            base_pos: Vec2::new(entity_pos.x, entity_pos.y - half_size - 2.0),
            offset_y: 0.0,
            timer: DAMAGE_DURATION,
            kind: FloatingTextKind::Damage(dmg),
        }
    }

    pub fn update(&mut self, dt: f32) -> bool {
        self.timer -= dt;
        if self.timer > 0.0 {
            self.offset_y -= FLOAT_SPEED * dt;
            true
        } else {
            false
        }
    }

    pub fn screen_y(&self) -> f32 {
        self.base_pos.y + self.offset_y
    }
}
