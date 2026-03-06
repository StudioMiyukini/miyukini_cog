use serde::{Deserialize, Serialize};

/// A feedback event to be dispatched to visual/audio systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeedbackEvent {
    /// Flash the screen edge red (damage taken).
    DamageFlash { intensity: f32 },
    /// Flash the life orb.
    LifeOrbPulse,
    /// Low-life warning pulse.
    LowLifeWarning,
    /// Item pickup confirmation.
    ItemPickup { name: String },
    /// Gold pickup.
    GoldPickup { amount: u64 },
    /// Level up celebration.
    LevelUp { new_level: u32 },
    /// Quest objective progress.
    QuestProgress { quest_name: String, objective: String },
    /// Quest completed.
    QuestComplete { quest_name: String },
    /// Skill on cooldown (click rejected).
    SkillOnCooldown,
    /// Not enough resource (click rejected).
    InsufficientResource,
    /// Item cannot be equipped.
    CannotEquip { reason: String },
    /// Inventory full.
    InventoryFull,
    /// Error beep / rejection.
    ErrorBeep,
}

/// Accessibility options that affect feedback.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilitySettings {
    /// Show damage numbers.
    pub show_damage_numbers: bool,
    /// Show health bars over enemies.
    pub show_enemy_health_bars: bool,
    /// Colorblind mode.
    pub colorblind_mode: ColorblindMode,
    /// Screen shake intensity multiplier (`0.0` = off).
    pub screen_shake_intensity: f32,
    /// Flash effects intensity (`0.0` = off, for photosensitive users).
    pub flash_intensity: f32,
    /// Show item labels on ground (alt-mode always on).
    pub always_show_loot_labels: bool,
    /// Font size multiplier.
    pub font_scale: f32,
}

impl Default for AccessibilitySettings {
    fn default() -> Self {
        Self {
            show_damage_numbers: true,
            show_enemy_health_bars: true,
            colorblind_mode: ColorblindMode::None,
            screen_shake_intensity: 1.0,
            flash_intensity: 1.0,
            always_show_loot_labels: false,
            font_scale: 1.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum ColorblindMode {
    #[default]
    None,
    Protanopia,
    Deuteranopia,
    Tritanopia,
}

/// Feedback queue that accumulates events per frame.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FeedbackQueue {
    events: Vec<FeedbackEvent>,
}

impl FeedbackQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, event: FeedbackEvent) {
        self.events.push(event);
    }

    pub fn drain(&mut self) -> Vec<FeedbackEvent> {
        std::mem::take(&mut self.events)
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feedback_queue_push_drain() {
        let mut queue = FeedbackQueue::new();
        queue.push(FeedbackEvent::LevelUp { new_level: 10 });
        queue.push(FeedbackEvent::ErrorBeep);
        assert_eq!(queue.len(), 2);
        let drained = queue.drain();
        assert_eq!(drained.len(), 2);
        assert!(queue.is_empty());
    }

    #[test]
    fn accessibility_defaults() {
        let settings = AccessibilitySettings::default();
        assert!(settings.show_damage_numbers);
        assert!((settings.font_scale - 1.0).abs() < f32::EPSILON);
        assert_eq!(settings.colorblind_mode, ColorblindMode::None);
    }
}
