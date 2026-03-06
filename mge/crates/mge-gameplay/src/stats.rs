use serde::{Deserialize, Serialize};

use crate::character::{BaseStats, Class};

/// Primary attribute totals (base + spent points + gear).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryAttributes {
    pub strength: i32,
    pub dexterity: i32,
    pub vitality: i32,
    pub energy: i32,
}

impl PrimaryAttributes {
    pub fn from_base(base: &BaseStats) -> Self {
        Self {
            strength: base.strength.cast_signed(),
            dexterity: base.dexterity.cast_signed(),
            vitality: base.vitality.cast_signed(),
            energy: base.energy.cast_signed(),
        }
    }
}

/// A flat or percentage modifier applied to a stat.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Modifier {
    /// Add a flat value.
    Flat(i32),
    /// Add a percentage increase (100 = +100%).
    Percentage(i32),
}

/// Source of a modifier (for stacking/conflict checks).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatModifier {
    pub source_id: u64,
    pub stat: StatKind,
    pub modifier: Modifier,
}

/// Enumeration of all stats that can be modified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatKind {
    Strength,
    Dexterity,
    Vitality,
    Energy,
    Life,
    Mana,
    Stamina,
    AttackRating,
    Defense,
    BlockChance,
    DamageMin,
    DamageMax,
    CritChance,
    AttackSpeed,
    CastSpeed,
    MovementSpeed,
    FireResist,
    ColdResist,
    LightningResist,
    PoisonResist,
    MagicFind,
    GoldFind,
    LifeOnHit,
    ManaOnKill,
}

/// Resolved stats with modifiers applied.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedStats {
    pub life_max: f32,
    pub mana_max: f32,
    pub stamina_max: f32,

    pub attack_rating: i32,
    pub defense: i32,
    pub block_chance: i32,
    pub damage_min: i32,
    pub damage_max: i32,
    pub crit_chance: i32,

    pub attack_speed: i32,
    pub cast_speed: i32,
    pub movement_speed: i32,

    pub fire_resist: i32,
    pub cold_resist: i32,
    pub lightning_resist: i32,
    pub poison_resist: i32,

    pub magic_find: i32,
    pub gold_find: i32,
    pub life_on_hit: i32,
    pub mana_on_kill: i32,
}

impl ResolvedStats {
    /// Compute derived resource pools from primary attributes and class yields.
    #[allow(clippy::cast_precision_loss)]
    pub fn compute_pools(
        attrs: &PrimaryAttributes,
        base: &BaseStats,
        level: u32,
    ) -> (f32, f32, f32) {
        let base_life = 45.0 + base.life_per_vitality * attrs.vitality as f32;
        let level_bonus_life = (level as f32 - 1.0) * 2.0;
        let life = base_life + level_bonus_life;

        let mana = 10.0 + base.mana_per_energy * attrs.energy as f32;
        let stamina = 80.0 + base.stamina_per_vitality * attrs.vitality as f32;

        (life, mana, stamina)
    }

    /// Apply a list of modifiers to a base stat value.
    pub fn apply_modifiers(base: i32, mods: &[Modifier]) -> i32 {
        let flat: i32 = mods
            .iter()
            .filter_map(|m| if let Modifier::Flat(v) = m { Some(*v) } else { None })
            .sum();
        let pct: i32 = mods
            .iter()
            .filter_map(|m| if let Modifier::Percentage(v) = m { Some(*v) } else { None })
            .sum();
        let after_flat = base + flat;
        after_flat + (after_flat * pct / 100)
    }
}

/// Full stat sheet for a character, mutable at runtime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterStats {
    pub class: Class,
    pub level: u32,
    pub attributes: PrimaryAttributes,
    pub resolved: ResolvedStats,
    pub modifiers: Vec<StatModifier>,
}

impl CharacterStats {
    pub fn new(class: Class, level: u32) -> Self {
        let base = class.base_stats();
        let attrs = PrimaryAttributes::from_base(&base);
        let (life, mana, stamina) = ResolvedStats::compute_pools(&attrs, &base, level);
        Self {
            class,
            level,
            attributes: attrs,
            resolved: ResolvedStats {
                life_max: life,
                mana_max: mana,
                stamina_max: stamina,
                attack_rating: 0,
                defense: 0,
                block_chance: 0,
                damage_min: 1,
                damage_max: 4,
                crit_chance: 0,
                attack_speed: 0,
                cast_speed: 0,
                movement_speed: 100,
                fire_resist: 0,
                cold_resist: 0,
                lightning_resist: 0,
                poison_resist: 0,
                magic_find: 0,
                gold_find: 0,
                life_on_hit: 0,
                mana_on_kill: 0,
            },
            modifiers: Vec::new(),
        }
    }

    /// Add a modifier and recompute affected stats.
    pub fn add_modifier(&mut self, modifier: StatModifier) {
        self.modifiers.push(modifier);
        self.recompute();
    }

    /// Remove all modifiers from a source.
    pub fn remove_source(&mut self, source_id: u64) {
        self.modifiers.retain(|m| m.source_id != source_id);
        self.recompute();
    }

    fn mods_for(&self, stat: StatKind) -> Vec<Modifier> {
        self.modifiers.iter().filter(|m| m.stat == stat).map(|m| m.modifier).collect()
    }

    fn recompute(&mut self) {
        self.resolved.fire_resist =
            ResolvedStats::apply_modifiers(0, &self.mods_for(StatKind::FireResist));
        self.resolved.cold_resist =
            ResolvedStats::apply_modifiers(0, &self.mods_for(StatKind::ColdResist));
        self.resolved.lightning_resist =
            ResolvedStats::apply_modifiers(0, &self.mods_for(StatKind::LightningResist));
        self.resolved.poison_resist =
            ResolvedStats::apply_modifiers(0, &self.mods_for(StatKind::PoisonResist));
        self.resolved.magic_find =
            ResolvedStats::apply_modifiers(0, &self.mods_for(StatKind::MagicFind));
        self.resolved.defense =
            ResolvedStats::apply_modifiers(0, &self.mods_for(StatKind::Defense));
        self.resolved.attack_rating =
            ResolvedStats::apply_modifiers(0, &self.mods_for(StatKind::AttackRating));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warrior_has_more_life_than_sorceress_at_level_1() {
        let warrior = CharacterStats::new(Class::Warrior, 1);
        let sorc = CharacterStats::new(Class::Sorceress, 1);
        assert!(warrior.resolved.life_max > sorc.resolved.life_max);
    }

    #[test]
    fn sorceress_has_more_mana_than_warrior_at_level_1() {
        let warrior = CharacterStats::new(Class::Warrior, 1);
        let sorc = CharacterStats::new(Class::Sorceress, 1);
        assert!(sorc.resolved.mana_max > warrior.resolved.mana_max);
    }

    #[test]
    fn flat_modifier_adds_value() {
        let result = ResolvedStats::apply_modifiers(100, &[Modifier::Flat(50)]);
        assert_eq!(result, 150);
    }

    #[test]
    fn percentage_modifier_stacks_after_flat() {
        let result =
            ResolvedStats::apply_modifiers(100, &[Modifier::Flat(0), Modifier::Percentage(100)]);
        assert_eq!(result, 200);
    }

    #[test]
    fn modifier_add_and_remove() {
        let mut stats = CharacterStats::new(Class::Warrior, 1);
        stats.add_modifier(StatModifier {
            source_id: 42,
            stat: StatKind::FireResist,
            modifier: Modifier::Flat(30),
        });
        assert_eq!(stats.resolved.fire_resist, 30);
        stats.remove_source(42);
        assert_eq!(stats.resolved.fire_resist, 0);
    }

    #[test]
    fn life_scales_with_level() {
        let lv1 = CharacterStats::new(Class::Paladin, 1);
        let lv10 = CharacterStats::new(Class::Paladin, 10);
        assert!(lv10.resolved.life_max > lv1.resolved.life_max);
    }
}
