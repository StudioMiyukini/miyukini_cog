use serde::{Deserialize, Serialize};

use crate::character::Class;

/// Unique skill identifier within a class's tree.
pub type SkillId = u32;

/// The type of skill action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkillKind {
    Melee,
    Ranged,
    Spell,
    Summon,
    Trap,
    Aura,
    Passive,
}

/// Skill cost per use.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SkillCost {
    /// Mana cost at level 1.
    pub mana_base: f32,
    /// Mana cost added per skill level.
    pub mana_per_level: f32,
}

impl SkillCost {
    #[allow(clippy::cast_precision_loss)]
    pub fn at_level(&self, level: u32) -> f32 {
        let l = level.max(1);
        self.mana_base + self.mana_per_level * (l as f32 - 1.0)
    }
}

/// Definition of a skill in the skill tree (static data).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillDef {
    pub id: SkillId,
    pub name: String,
    pub kind: SkillKind,
    pub class: Class,
    /// Minimum character level to unlock.
    pub min_level: u32,
    /// Prerequisite skill IDs (all must be learned at level >= 1).
    pub prerequisites: Vec<SkillId>,
    pub cost: SkillCost,
    pub max_level: u32,
}

impl SkillDef {
    pub fn cost_at(&self, level: u32) -> f32 {
        self.cost.at_level(level)
    }
}

/// A player's learned skill entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearnedSkill {
    pub id: SkillId,
    pub level: u32,
}

/// The skill tree state for one character.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillTree {
    pub class: Class,
    pub learned: Vec<LearnedSkill>,
    pub available_points: u32,
}

impl SkillTree {
    pub fn new(class: Class) -> Self {
        Self { class, learned: Vec::new(), available_points: 0 }
    }

    /// Get level of a skill (0 = not learned).
    pub fn level_of(&self, id: SkillId) -> u32 {
        self.learned.iter().find(|s| s.id == id).map_or(0, |s| s.level)
    }

    /// Check prerequisites and minimum character level.
    pub fn can_learn(&self, def: &SkillDef, char_level: u32) -> Result<(), &'static str> {
        if def.class != self.class {
            return Err("wrong class");
        }
        if char_level < def.min_level {
            return Err("character level too low");
        }
        let current = self.level_of(def.id);
        if current >= def.max_level {
            return Err("skill already at max level");
        }
        for &prereq in &def.prerequisites {
            if self.level_of(prereq) == 0 {
                return Err("prerequisite not learned");
            }
        }
        if self.available_points == 0 {
            return Err("no skill points available");
        }
        Ok(())
    }

    /// Spend a skill point on a skill. Returns Err if prerequisites not met.
    pub fn invest(&mut self, def: &SkillDef, char_level: u32) -> Result<u32, &'static str> {
        self.can_learn(def, char_level)?;
        let entry = self.learned.iter_mut().find(|s| s.id == def.id);
        if let Some(e) = entry {
            e.level += 1;
            self.available_points -= 1;
            Ok(e.level)
        } else {
            self.learned.push(LearnedSkill { id: def.id, level: 1 });
            self.available_points -= 1;
            Ok(1)
        }
    }

    /// Add skill points (e.g., on level up).
    pub fn add_points(&mut self, n: u32) {
        self.available_points += n;
    }
}

/// Minimal hardcoded skill definitions for each class (first 3 skills).
/// Real trees will be data-driven; this satisfies T05.
pub fn class_skill_stubs(class: Class) -> Vec<SkillDef> {
    let make = |id: SkillId, name: &str, kind: SkillKind, min_level: u32, prereqs: Vec<SkillId>| {
        SkillDef {
            id,
            name: name.into(),
            kind,
            class,
            min_level,
            prerequisites: prereqs,
            cost: SkillCost { mana_base: 4.0, mana_per_level: 1.0 },
            max_level: 20,
        }
    };

    let (base_id, names): (u32, [(&str, SkillKind, u32); 3]) = match class {
        Class::Warrior => (
            100,
            [
                ("Bash", SkillKind::Melee, 1),
                ("Whirlwind", SkillKind::Melee, 12),
                ("War Cry", SkillKind::Passive, 18),
            ],
        ),
        Class::Ranger => (
            200,
            [
                ("Multiple Shot", SkillKind::Ranged, 1),
                ("Guided Arrow", SkillKind::Ranged, 12),
                ("Strafe", SkillKind::Ranged, 18),
            ],
        ),
        Class::Sorceress => (
            300,
            [
                ("Fireball", SkillKind::Spell, 1),
                ("Blizzard", SkillKind::Spell, 12),
                ("Meteor", SkillKind::Spell, 18),
            ],
        ),
        Class::Necromancer => (
            400,
            [
                ("Raise Skeleton", SkillKind::Summon, 1),
                ("Bone Spear", SkillKind::Spell, 6),
                ("Revive", SkillKind::Summon, 18),
            ],
        ),
        Class::Paladin => (
            500,
            [
                ("Holy Strike", SkillKind::Melee, 1),
                ("Conviction", SkillKind::Aura, 12),
                ("Fist of the Heavens", SkillKind::Spell, 18),
            ],
        ),
        Class::Shaman => (
            600,
            [
                ("Shock Totem", SkillKind::Trap, 1),
                ("Nature Aura", SkillKind::Aura, 6),
                ("Chain Lightning", SkillKind::Spell, 18),
            ],
        ),
        Class::Assassin => (
            700,
            [
                ("Shadow Strike", SkillKind::Melee, 1),
                ("Death Sentry", SkillKind::Trap, 12),
                ("Dragon Tail", SkillKind::Melee, 18),
            ],
        ),
    };

    let s0 = make(base_id, names[0].0, names[0].1, names[0].2, vec![]);
    let s1 = make(base_id + 1, names[1].0, names[1].1, names[1].2, vec![base_id]);
    let s2 = make(base_id + 2, names[2].0, names[2].1, names[2].2, vec![base_id + 1]);
    vec![s0, s1, s2]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn warrior_skills() -> Vec<SkillDef> {
        class_skill_stubs(Class::Warrior)
    }

    #[test]
    fn invest_first_skill() {
        let defs = warrior_skills();
        let mut tree = SkillTree::new(Class::Warrior);
        tree.add_points(3);
        let result = tree.invest(&defs[0], 1);
        assert!(result.is_ok());
        assert_eq!(tree.level_of(defs[0].id), 1);
        assert_eq!(tree.available_points, 2);
    }

    #[test]
    fn prerequisite_blocks_second_skill() {
        let defs = warrior_skills();
        let mut tree = SkillTree::new(Class::Warrior);
        tree.add_points(3);
        let result = tree.invest(&defs[1], 20);
        assert_eq!(result, Err("prerequisite not learned"));
    }

    #[test]
    fn level_gates_skill() {
        let defs = warrior_skills();
        let mut tree = SkillTree::new(Class::Warrior);
        tree.add_points(5);
        assert!(tree.invest(&defs[0], 1).is_ok());
        let result = tree.invest(&defs[1], 5);
        assert_eq!(result, Err("character level too low"));
    }

    #[test]
    fn chain_unlock() {
        let defs = warrior_skills();
        let mut tree = SkillTree::new(Class::Warrior);
        tree.add_points(5);
        assert!(tree.invest(&defs[0], 1).is_ok());
        assert!(tree.invest(&defs[1], 12).is_ok());
        assert!(tree.invest(&defs[2], 18).is_ok());
    }

    #[test]
    fn all_classes_have_skill_stubs() {
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
            let stubs = class_skill_stubs(class);
            assert_eq!(stubs.len(), 3);
        }
    }

    #[test]
    fn mana_cost_scales() {
        let cost = SkillCost { mana_base: 4.0, mana_per_level: 1.0 };
        assert!((cost.at_level(1) - 4.0).abs() < f32::EPSILON);
        assert!((cost.at_level(5) - 8.0).abs() < f32::EPSILON);
    }
}
