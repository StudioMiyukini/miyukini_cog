use serde::{Deserialize, Serialize};

use crate::zone::ZoneId;

/// Category of a secondary activity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecondaryKind {
    /// Explore a specific zone.
    Exploration,
    /// Kill a set number of a particular enemy type.
    Bounty,
    /// Collect a specific item or resource.
    Collection,
    /// Interact with a special object or NPC.
    Interaction,
}

/// A secondary objective (non-critical side-activity).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryObjective {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub kind: SecondaryKind,
    pub zone_id: ZoneId,
    /// Target count (kills, items, etc.). 0 = n/a.
    pub target_count: u32,
    /// Progress toward `target_count`.
    pub current_count: u32,
    pub completed: bool,
}

impl SecondaryObjective {
    #[must_use]
    pub fn new(
        id: u32,
        name: impl Into<String>,
        description: impl Into<String>,
        kind: SecondaryKind,
        zone_id: ZoneId,
        target_count: u32,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description: description.into(),
            kind,
            zone_id,
            target_count,
            current_count: 0,
            completed: false,
        }
    }

    /// Increment progress. Returns `true` if this call completed the objective.
    pub fn increment(&mut self, amount: u32) -> bool {
        if self.completed {
            return false;
        }
        self.current_count = self.current_count.saturating_add(amount);
        if self.target_count > 0 && self.current_count >= self.target_count {
            self.completed = true;
            true
        } else {
            false
        }
    }

    /// Force-complete (e.g., from a script trigger).
    pub fn force_complete(&mut self) {
        self.completed = true;
    }

    /// Fraction complete [0.0, 1.0].
    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn progress_fraction(&self) -> f32 {
        if self.target_count == 0 {
            return if self.completed { 1.0 } else { 0.0 };
        }
        (self.current_count as f32 / self.target_count as f32).min(1.0)
    }
}

/// Act 1 secondary objectives.
#[must_use]
pub fn act1_secondary_objectives() -> Vec<SecondaryObjective> {
    vec![
        SecondaryObjective::new(
            1,
            "Fallen Culling",
            "Kill 30 Fallen in the Blood Moor.",
            SecondaryKind::Bounty,
            10,
            30,
        ),
        SecondaryObjective::new(
            2,
            "Wilderness Explorer",
            "Fully explore the Wilderness zone.",
            SecondaryKind::Exploration,
            10,
            0,
        ),
        SecondaryObjective::new(
            3,
            "Skull Collection",
            "Collect 5 skulls from the Burial Grounds.",
            SecondaryKind::Collection,
            21,
            5,
        ),
        SecondaryObjective::new(
            4,
            "Speak with Akara",
            "Talk to Akara in the Rogue Encampment.",
            SecondaryKind::Interaction,
            0,
            0,
        ),
        SecondaryObjective::new(
            5,
            "Cave Cleanse",
            "Clear all monsters in the Cave level.",
            SecondaryKind::Bounty,
            22,
            20,
        ),
    ]
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment_completes_on_target() {
        let mut obj = SecondaryObjective::new(1, "Test", "Test", SecondaryKind::Bounty, 10, 5);
        assert!(!obj.increment(4));
        assert!(obj.increment(1));
        assert!(obj.completed);
    }

    #[test]
    fn increment_after_complete_returns_false() {
        let mut obj = SecondaryObjective::new(1, "Test", "Test", SecondaryKind::Bounty, 10, 1);
        obj.increment(1);
        assert!(!obj.increment(1));
    }

    #[test]
    fn progress_fraction_clamped() {
        let mut obj = SecondaryObjective::new(1, "Test", "Test", SecondaryKind::Bounty, 10, 10);
        obj.current_count = 15;
        assert!((obj.progress_fraction() - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn act1_secondary_has_five_entries() {
        assert_eq!(act1_secondary_objectives().len(), 5);
    }
}
