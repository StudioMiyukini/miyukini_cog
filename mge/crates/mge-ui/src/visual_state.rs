use serde::{Deserialize, Serialize};

/// Visual state of an interactive UI element.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum WidgetState {
    #[default]
    Normal,
    Hovered,
    Pressed,
    Selected,
    Disabled,
    Error,
}

/// Cooldown overlay state for skill/item use.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CooldownOverlay {
    pub fraction: f32,
    pub sweeping: bool,
}

impl CooldownOverlay {
    pub fn from_fraction(fraction: f32) -> Self {
        Self { fraction: fraction.clamp(0.0, 1.0), sweeping: fraction > 0.0 }
    }

    pub fn is_ready(&self) -> bool {
        self.fraction <= 0.0
    }
}

/// Insufficient resource indicator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceGate {
    Sufficient,
    InsufficientMana,
    InsufficientLife,
    InsufficientGold,
    LevelTooLow,
    RequirementNotMet,
}

impl ResourceGate {
    pub fn is_blocked(&self) -> bool {
        *self != Self::Sufficient
    }
}

/// Composite visual state for a skill slot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillVisualState {
    pub widget: WidgetState,
    pub cooldown: CooldownOverlay,
    pub resource: ResourceGate,
}

impl SkillVisualState {
    /// Whether the skill can be activated right now.
    pub fn is_usable(&self) -> bool {
        self.widget != WidgetState::Disabled
            && self.cooldown.is_ready()
            && !self.resource.is_blocked()
    }
}

/// Item slot visual state (inventory, vendor, stash).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemSlotState {
    Normal,
    Hovered,
    Selected,
    DragSource,
    DropTarget,
    DropInvalid,
    Locked,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_widget_state() {
        let ws = WidgetState::default();
        assert_eq!(ws, WidgetState::Normal);
    }

    #[test]
    fn cooldown_ready_when_zero() {
        let cd = CooldownOverlay::from_fraction(0.0);
        assert!(cd.is_ready());
        let cd2 = CooldownOverlay::from_fraction(0.5);
        assert!(!cd2.is_ready());
    }

    #[test]
    fn skill_usable_when_ready_and_sufficient() {
        let state = SkillVisualState {
            widget: WidgetState::Normal,
            cooldown: CooldownOverlay::from_fraction(0.0),
            resource: ResourceGate::Sufficient,
        };
        assert!(state.is_usable());
    }

    #[test]
    fn skill_not_usable_on_cooldown() {
        let state = SkillVisualState {
            widget: WidgetState::Normal,
            cooldown: CooldownOverlay::from_fraction(0.3),
            resource: ResourceGate::Sufficient,
        };
        assert!(!state.is_usable());
    }

    #[test]
    fn skill_not_usable_insufficient_resource() {
        let state = SkillVisualState {
            widget: WidgetState::Normal,
            cooldown: CooldownOverlay::from_fraction(0.0),
            resource: ResourceGate::InsufficientMana,
        };
        assert!(!state.is_usable());
    }
}
