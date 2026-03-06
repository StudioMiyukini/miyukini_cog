use serde::{Deserialize, Serialize};

/// Affixes that can be applied to an elite or champion monster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EliteAffix {
    Cursed,
    Fanaticism,
    ExtraSpeed,
    StoneSkin,
    ColdEnchanted,
    FireEnchanted,
    LightningEnchanted,
    ManaBurn,
    Teleportation,
    Aura,
}

/// Type of pack modification applied to a champion group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PackModifier {
    /// Pack is larger than normal.
    Horde,
    /// Pack members share a health pool.
    SharedHealth,
    /// Pack members auto-revive once.
    Undying,
}

/// Immunity types that reduce all damage of a type to 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImmunityKind {
    Physical,
    Fire,
    Cold,
    Lightning,
    Poison,
    Magic,
}

/// An elite (unique-style) monster variant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EliteVariant {
    pub base_monster_id: u32,
    pub name: String,
    /// Up to 6 affixes (D2 rule).
    pub affixes: Vec<EliteAffix>,
    pub immunities: Vec<ImmunityKind>,
    /// HP multiplier (e.g., 2.0 = double HP).
    pub hp_multiplier: f32,
    /// XP multiplier.
    pub xp_multiplier: f32,
}

impl EliteVariant {
    #[must_use]
    pub fn new(base_monster_id: u32, name: impl Into<String>) -> Self {
        Self {
            base_monster_id,
            name: name.into(),
            affixes: Vec::new(),
            immunities: Vec::new(),
            hp_multiplier: 2.0,
            xp_multiplier: 3.0,
        }
    }

    /// Add an affix. No-op if already at 6 affixes (D2 cap).
    #[must_use]
    pub fn with_affix(mut self, affix: EliteAffix) -> Self {
        if self.affixes.len() < 6 {
            self.affixes.push(affix);
        }
        self
    }

    #[must_use]
    pub fn with_immunity(mut self, immunity: ImmunityKind) -> Self {
        if !self.immunities.contains(&immunity) {
            self.immunities.push(immunity);
        }
        self
    }
}

/// A champion pack definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChampionPack {
    pub base_monster_id: u32,
    /// Number of members in the pack.
    pub size: u8,
    pub affix: EliteAffix,
    pub modifier: Option<PackModifier>,
    pub immunities: Vec<ImmunityKind>,
    pub hp_multiplier: f32,
    pub xp_multiplier: f32,
}

impl ChampionPack {
    #[must_use]
    pub fn new(base_monster_id: u32, size: u8, affix: EliteAffix) -> Self {
        Self {
            base_monster_id,
            size,
            affix,
            modifier: None,
            immunities: Vec::new(),
            hp_multiplier: 1.5,
            xp_multiplier: 2.0,
        }
    }
}

/// Act 1 sample elite variants.
#[must_use]
pub fn act1_elites() -> Vec<EliteVariant> {
    vec![
        EliteVariant::new(1, "Corpsefire")
            .with_affix(EliteAffix::ColdEnchanted)
            .with_affix(EliteAffix::ExtraSpeed),
        EliteVariant::new(6, "Pitspawn Fouldog")
            .with_affix(EliteAffix::FireEnchanted)
            .with_immunity(ImmunityKind::Fire),
        EliteVariant::new(4, "Bone Ash")
            .with_affix(EliteAffix::LightningEnchanted)
            .with_affix(EliteAffix::StoneSkin),
    ]
}

// ─── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elite_affix_cap_at_six() {
        let mut elite = EliteVariant::new(1, "Test");
        for _ in 0..8 {
            elite = elite.with_affix(EliteAffix::Cursed);
        }
        assert_eq!(elite.affixes.len(), 6);
    }

    #[test]
    fn elite_immunity_no_duplicates() {
        let elite = EliteVariant::new(1, "Test")
            .with_immunity(ImmunityKind::Fire)
            .with_immunity(ImmunityKind::Fire);
        assert_eq!(elite.immunities.len(), 1);
    }

    #[test]
    fn act1_elites_all_have_affixes() {
        let elites = act1_elites();
        assert!(elites.iter().all(|e| !e.affixes.is_empty()));
    }

    #[test]
    fn champion_pack_defaults() {
        let pack = ChampionPack::new(1, 5, EliteAffix::ExtraSpeed);
        assert_eq!(pack.size, 5);
        assert!(pack.modifier.is_none());
    }
}
