// @id: MGE-UI-SkillBar @do: skill-bar @role: front-end @layer: 3 @human: miyuk
//! Skill bar data model — left/right mouse-button skill slots with cooldown tracking.

// ---------------------------------------------------------------------------
// SkillSlot
// ---------------------------------------------------------------------------

/// A single assignable skill slot (left or right mouse button).
#[derive(Debug, Clone, PartialEq)]
pub struct SkillSlot {
    /// Skill identifier, `None` when the slot is empty.
    pub skill_id: Option<String>,
    /// Seconds of cooldown remaining.
    pub cooldown_remaining: f32,
    /// Total cooldown duration in seconds. Zero means no cooldown.
    pub cooldown_total: f32,
}

impl SkillSlot {
    /// Create an empty slot with no cooldown.
    pub fn empty() -> Self {
        Self {
            skill_id: None,
            cooldown_remaining: 0.0,
            cooldown_total: 0.0,
        }
    }

    /// Cooldown progress in the range `[0.0, 1.0]`.
    ///
    /// Returns `0.0` when the skill is ready (no cooldown active or
    /// `cooldown_total` is zero). Returns `1.0` immediately after the skill
    /// is triggered (cooldown_remaining == cooldown_total).
    pub fn cooldown_percent(&self) -> f32 {
        if self.cooldown_total <= 0.0 {
            return 0.0;
        }
        (self.cooldown_remaining / self.cooldown_total).clamp(0.0, 1.0)
    }

    /// Returns `true` when there is no active cooldown.
    pub fn is_ready(&self) -> bool {
        self.cooldown_remaining <= 0.0
    }
}

impl Default for SkillSlot {
    fn default() -> Self {
        Self::empty()
    }
}

// ---------------------------------------------------------------------------
// SkillBarState
// ---------------------------------------------------------------------------

/// State for the two primary skill slots (left and right mouse buttons).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct SkillBarState {
    /// Left mouse button skill.
    pub left: SkillSlot,
    /// Right mouse button skill.
    pub right: SkillSlot,
}

impl SkillBarState {
    /// Create a new `SkillBarState` with both slots empty.
    pub fn new() -> Self {
        Self::default()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skill_bar_cooldown_display() {
        let slot = SkillSlot {
            skill_id: Some("fireball".to_string()),
            cooldown_remaining: 2.5,
            cooldown_total: 5.0,
        };
        // Cooldown active → percent > 0
        assert!(slot.cooldown_percent() > 0.0, "cooldown_percent must be > 0 when cooldown > 0");
        assert!(!slot.is_ready(), "slot must not be ready while cooldown is active");
    }

    #[test]
    fn skill_bar_none() {
        let slot = SkillSlot::empty();
        assert_eq!(slot.skill_id, None, "empty slot must have no skill_id");
        assert_eq!(slot.cooldown_percent(), 0.0, "empty slot cooldown_percent must be 0.0");
        assert!(slot.is_ready(), "empty slot must always be ready");
    }
}
