use serde::{Deserialize, Serialize};

/// Hardcore flag attached to a character save.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HardcoreFlag {
    /// Character is alive.
    Alive,
    /// Character died; save is permanently locked.
    Dead,
}

/// Hardcore character metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardcoreCharacter {
    pub char_id: u64,
    pub name: String,
    pub flag: HardcoreFlag,
    /// If dead: tick at which death occurred.
    pub death_tick: Option<u64>,
    /// If dead: zone where death occurred.
    pub death_zone_id: Option<u32>,
}

impl HardcoreCharacter {
    #[must_use]
    pub fn new(char_id: u64, name: impl Into<String>) -> Self {
        Self {
            char_id,
            name: name.into(),
            flag: HardcoreFlag::Alive,
            death_tick: None,
            death_zone_id: None,
        }
    }

    /// Mark character as dead. Returns `false` if already dead (idempotent guard).
    pub fn die(&mut self, tick: u64, zone_id: u32) -> bool {
        if self.flag == HardcoreFlag::Dead {
            return false;
        }
        self.flag = HardcoreFlag::Dead;
        self.death_tick = Some(tick);
        self.death_zone_id = Some(zone_id);
        true
    }

    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.flag == HardcoreFlag::Alive
    }

    /// Validate that the save file is not tampered (dead chars cannot be resurrected).
    #[must_use]
    pub fn validate_save(&self) -> bool {
        match self.flag {
            HardcoreFlag::Alive => self.death_tick.is_none() && self.death_zone_id.is_none(),
            HardcoreFlag::Dead => self.death_tick.is_some() && self.death_zone_id.is_some(),
        }
    }
}

/// Registry of hardcore deaths for hall-of-fame display.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HallOfFame {
    pub entries: Vec<HardcoreCharacter>,
}

impl HallOfFame {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, character: HardcoreCharacter) {
        if character.flag == HardcoreFlag::Dead {
            self.entries.push(character);
        }
    }

    #[must_use]
    pub fn count(&self) -> usize {
        self.entries.len()
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_char_is_alive() {
        let c = HardcoreCharacter::new(1, "Miyuki");
        assert!(c.is_alive());
        assert!(c.validate_save());
    }

    #[test]
    fn die_marks_dead_with_metadata() {
        let mut c = HardcoreCharacter::new(1, "Miyuki");
        assert!(c.die(100, 42));
        assert!(!c.is_alive());
        assert!(c.validate_save());
        assert_eq!(c.death_zone_id, Some(42));
    }

    #[test]
    fn die_twice_returns_false() {
        let mut c = HardcoreCharacter::new(1, "Miyuki");
        c.die(100, 42);
        assert!(!c.die(200, 10));
    }

    #[test]
    fn hall_of_fame_records_dead_only() {
        let mut hall = HallOfFame::new();
        let mut c = HardcoreCharacter::new(1, "Miyuki");
        c.die(100, 42);
        hall.record(c);
        assert_eq!(hall.count(), 1);
    }
}
