use serde::{Deserialize, Serialize};

/// Resource orb type (life or resource).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrbKind {
    Life,
    Resource,
}

/// State of a resource orb displayed on the HUD.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbState {
    pub kind: OrbKind,
    pub current: f32,
    pub max: f32,
}

impl OrbState {
    pub fn new(kind: OrbKind, max: f32) -> Self {
        Self { kind, current: max, max }
    }

    /// Fraction filled, clamped to `[0.0, 1.0]`.
    pub fn fraction(&self) -> f32 {
        if self.max <= 0.0 {
            return 0.0;
        }
        (self.current / self.max).clamp(0.0, 1.0)
    }

    pub fn set_current(&mut self, value: f32) {
        self.current = value.clamp(0.0, self.max);
    }

    pub fn modify(&mut self, delta: f32) {
        self.set_current(self.current + delta);
    }
}

/// A single skill slot on the action bar.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillSlot {
    pub skill_id: Option<String>,
    pub cooldown_remaining: f32,
    pub cooldown_total: f32,
    pub charges: u8,
    pub max_charges: u8,
}

impl Default for SkillSlot {
    fn default() -> Self {
        Self {
            skill_id: None,
            cooldown_remaining: 0.0,
            cooldown_total: 0.0,
            charges: 0,
            max_charges: 0,
        }
    }
}

impl SkillSlot {
    pub fn is_empty(&self) -> bool {
        self.skill_id.is_none()
    }

    pub fn is_on_cooldown(&self) -> bool {
        self.cooldown_remaining > 0.0
    }

    /// Cooldown fraction remaining, `0.0` = ready.
    pub fn cooldown_fraction(&self) -> f32 {
        if self.cooldown_total <= 0.0 {
            return 0.0;
        }
        (self.cooldown_remaining / self.cooldown_total).clamp(0.0, 1.0)
    }

    /// Advance cooldown by `dt` seconds.
    pub fn tick_cooldown(&mut self, dt: f32) {
        if self.cooldown_remaining > 0.0 {
            self.cooldown_remaining = (self.cooldown_remaining - dt).max(0.0);
        }
    }

    pub fn start_cooldown(&mut self, duration: f32) {
        self.cooldown_total = duration;
        self.cooldown_remaining = duration;
    }
}

/// A buff/debuff icon on the HUD.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuffIcon {
    pub name: String,
    pub is_debuff: bool,
    pub remaining_secs: f32,
    pub total_secs: f32,
    pub stacks: u8,
}

impl BuffIcon {
    /// Whether this buff has expired.
    pub fn is_expired(&self) -> bool {
        self.remaining_secs <= 0.0
    }

    pub fn tick(&mut self, dt: f32) {
        self.remaining_secs = (self.remaining_secs - dt).max(0.0);
    }
}

/// The full HUD state for one player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HudState {
    pub life: OrbState,
    pub resource: OrbState,
    pub skill_bar: Vec<SkillSlot>,
    pub buffs: Vec<BuffIcon>,
    pub experience_fraction: f32,
    pub level: u32,
    pub gold: u64,
}

impl HudState {
    pub fn new(max_life: f32, max_resource: f32, skill_count: usize) -> Self {
        Self {
            life: OrbState::new(OrbKind::Life, max_life),
            resource: OrbState::new(OrbKind::Resource, max_resource),
            skill_bar: vec![SkillSlot::default(); skill_count],
            buffs: Vec::new(),
            experience_fraction: 0.0,
            level: 1,
            gold: 0,
        }
    }

    /// Tick all cooldowns and buff timers.
    pub fn tick(&mut self, dt: f32) {
        for slot in &mut self.skill_bar {
            slot.tick_cooldown(dt);
        }
        for buff in &mut self.buffs {
            buff.tick(dt);
        }
        self.buffs.retain(|b| !b.is_expired());
    }

    pub fn add_buff(&mut self, buff: BuffIcon) {
        self.buffs.push(buff);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orb_fraction_clamps() {
        let mut orb = OrbState::new(OrbKind::Life, 100.0);
        assert!((orb.fraction() - 1.0).abs() < f32::EPSILON);
        orb.set_current(50.0);
        assert!((orb.fraction() - 0.5).abs() < f32::EPSILON);
        orb.set_current(-10.0);
        assert!((orb.fraction()).abs() < f32::EPSILON);
    }

    #[test]
    fn skill_cooldown_ticks_down() {
        let mut slot = SkillSlot { skill_id: Some("fireball".into()), ..SkillSlot::default() };
        slot.start_cooldown(2.0);
        assert!(slot.is_on_cooldown());
        slot.tick_cooldown(1.5);
        assert!(slot.is_on_cooldown());
        slot.tick_cooldown(1.0);
        assert!(!slot.is_on_cooldown());
    }

    #[test]
    fn hud_tick_expires_buffs() {
        let mut hud = HudState::new(100.0, 50.0, 6);
        hud.add_buff(BuffIcon {
            name: "haste".into(),
            is_debuff: false,
            remaining_secs: 1.0,
            total_secs: 5.0,
            stacks: 1,
        });
        assert_eq!(hud.buffs.len(), 1);
        hud.tick(0.5);
        assert_eq!(hud.buffs.len(), 1);
        hud.tick(0.6);
        assert!(hud.buffs.is_empty());
    }

    #[test]
    fn empty_skill_slot() {
        let slot = SkillSlot::default();
        assert!(slot.is_empty());
        assert!(!slot.is_on_cooldown());
        assert!((slot.cooldown_fraction()).abs() < f32::EPSILON);
    }
}
