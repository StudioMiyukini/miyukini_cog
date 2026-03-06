use serde::{Deserialize, Serialize};

/// The seven playable archetypes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Class {
    Warrior,
    Ranger,
    Sorceress,
    Necromancer,
    Paladin,
    Shaman,
    Assassin,
}

impl Class {
    pub fn display_name(self) -> &'static str {
        match self {
            Class::Warrior => "Warrior",
            Class::Ranger => "Ranger",
            Class::Sorceress => "Sorceress",
            Class::Necromancer => "Necromancer",
            Class::Paladin => "Paladin",
            Class::Shaman => "Shaman",
            Class::Assassin => "Assassin",
        }
    }

    /// Base starting stats for a class.
    pub fn base_stats(self) -> BaseStats {
        match self {
            Class::Warrior => BaseStats {
                strength: 30,
                dexterity: 20,
                vitality: 25,
                energy: 10,
                life_per_vitality: 4.0,
                mana_per_energy: 1.5,
                stamina_per_vitality: 1.0,
            },
            Class::Ranger => BaseStats {
                strength: 20,
                dexterity: 30,
                vitality: 22,
                energy: 13,
                life_per_vitality: 3.0,
                mana_per_energy: 1.5,
                stamina_per_vitality: 1.0,
            },
            Class::Sorceress => BaseStats {
                strength: 10,
                dexterity: 15,
                vitality: 15,
                energy: 35,
                life_per_vitality: 2.0,
                mana_per_energy: 2.0,
                stamina_per_vitality: 1.0,
            },
            Class::Necromancer => BaseStats {
                strength: 15,
                dexterity: 15,
                vitality: 15,
                energy: 25,
                life_per_vitality: 2.5,
                mana_per_energy: 2.0,
                stamina_per_vitality: 1.0,
            },
            Class::Paladin => BaseStats {
                strength: 25,
                dexterity: 20,
                vitality: 25,
                energy: 15,
                life_per_vitality: 3.5,
                mana_per_energy: 1.5,
                stamina_per_vitality: 1.0,
            },
            Class::Shaman => BaseStats {
                strength: 15,
                dexterity: 20,
                vitality: 20,
                energy: 30,
                life_per_vitality: 2.5,
                mana_per_energy: 2.0,
                stamina_per_vitality: 1.0,
            },
            Class::Assassin => BaseStats {
                strength: 20,
                dexterity: 30,
                vitality: 20,
                energy: 15,
                life_per_vitality: 3.0,
                mana_per_energy: 1.5,
                stamina_per_vitality: 1.0,
            },
        }
    }
}

/// Primary attribute base values and per-point yields.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BaseStats {
    pub strength: u32,
    pub dexterity: u32,
    pub vitality: u32,
    pub energy: u32,
    pub life_per_vitality: f32,
    pub mana_per_energy: f32,
    pub stamina_per_vitality: f32,
}

/// Player character name validation.
pub fn validate_character_name(name: &str) -> Result<(), &'static str> {
    if name.is_empty() {
        return Err("name cannot be empty");
    }
    if name.len() > 16 {
        return Err("name too long (max 16 characters)");
    }
    if !name.chars().all(|c| c.is_ascii_alphabetic() || c == '-' || c == '_') {
        return Err("only letters, hyphens and underscores allowed");
    }
    if name.starts_with('-') || name.starts_with('_') {
        return Err("name cannot start with a symbol");
    }
    Ok(())
}

/// A new-character creation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterCreation {
    pub name: String,
    pub class: Class,
}

impl CharacterCreation {
    pub fn new(name: impl Into<String>, class: Class) -> Result<Self, &'static str> {
        let name = name.into();
        validate_character_name(&name)?;
        Ok(Self { name, class })
    }
}

/// Roster of characters available to select (main menu screen).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CharacterRoster {
    pub slots: Vec<CharacterSlot>,
    pub max_slots: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSlot {
    pub name: String,
    pub class: Class,
    pub level: u32,
    pub last_scene: String,
}

impl CharacterRoster {
    pub fn new(max_slots: usize) -> Self {
        Self { slots: Vec::new(), max_slots }
    }

    pub fn add(&mut self, slot: CharacterSlot) -> bool {
        if self.slots.len() >= self.max_slots {
            return false;
        }
        self.slots.push(slot);
        true
    }

    pub fn remove(&mut self, name: &str) -> bool {
        let before = self.slots.len();
        self.slots.retain(|s| s.name != name);
        self.slots.len() < before
    }

    pub fn get(&self, name: &str) -> Option<&CharacterSlot> {
        self.slots.iter().find(|s| s.name == name)
    }

    pub fn is_full(&self) -> bool {
        self.slots.len() >= self.max_slots
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_classes_have_display_names() {
        let classes = [
            Class::Warrior,
            Class::Ranger,
            Class::Sorceress,
            Class::Necromancer,
            Class::Paladin,
            Class::Shaman,
            Class::Assassin,
        ];
        for class in classes {
            assert!(!class.display_name().is_empty());
        }
    }

    #[test]
    fn sorceress_has_highest_energy() {
        let sorc = Class::Sorceress.base_stats();
        let warrior = Class::Warrior.base_stats();
        assert!(sorc.energy > warrior.energy);
    }

    #[test]
    fn name_validation() {
        assert!(validate_character_name("Wanderer").is_ok());
        assert!(validate_character_name("").is_err());
        assert!(validate_character_name("12345").is_err());
        assert!(validate_character_name("-start").is_err());
        assert!(validate_character_name("A-valid_Name").is_ok());
        assert!(validate_character_name("ThisNameIsTooLongForTheGame").is_err());
    }

    #[test]
    fn character_creation_validates_name() {
        assert!(CharacterCreation::new("Hero", Class::Warrior).is_ok());
        assert!(CharacterCreation::new("", Class::Warrior).is_err());
    }

    #[test]
    fn roster_max_slots() {
        let mut roster = CharacterRoster::new(3);
        for i in 0..3 {
            assert!(roster.add(CharacterSlot {
                name: format!("char{i}"),
                class: Class::Warrior,
                level: 1,
                last_scene: "camp".into(),
            }));
        }
        assert!(roster.is_full());
        assert!(!roster.add(CharacterSlot {
            name: "overflow".into(),
            class: Class::Ranger,
            level: 1,
            last_scene: "camp".into(),
        }));
    }

    #[test]
    fn roster_remove() {
        let mut roster = CharacterRoster::new(5);
        roster.add(CharacterSlot {
            name: "hero".into(),
            class: Class::Paladin,
            level: 20,
            last_scene: "camp".into(),
        });
        assert!(roster.remove("hero"));
        assert!(roster.get("hero").is_none());
    }
}
