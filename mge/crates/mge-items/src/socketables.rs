use serde::{Deserialize, Serialize};

use mge_gameplay::stats::{Modifier, StatKind};

use crate::item::ItemFamily;

/// Discriminant for the kind of socketable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SocketableKind {
    Gem,
    Rune,
    Jewel,
}

// ─── Gems ────────────────────────────────────────────────────────────────────

/// Gem colour (element/attribute).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GemColor {
    Ruby,     // Fire / Life
    Sapphire, // Cold / Mana
    Topaz,    // Lightning / Magic Find
    Emerald,  // Poison / Dexterity
    Diamond,  // Physical / Attack Rating
    Amethyst, // Strength
    Skull,    // Life on Hit / Life
}

/// Gem quality grade (Chipped → Perfect).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum GemGrade {
    Chipped,
    Flawed,
    Normal,
    Flawless,
    Perfect,
}

/// Modifier bundles for a gem, split by socket context.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GemMods {
    pub weapon: Vec<(StatKind, Modifier)>,
    pub armor: Vec<(StatKind, Modifier)>,
    pub shield: Vec<(StatKind, Modifier)>,
}

/// A gem instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gem {
    pub color: GemColor,
    pub grade: GemGrade,
}

impl Gem {
    /// Compute stat mods for this gem based on grade and context.
    pub fn mods(&self) -> GemMods {
        let g = match self.grade {
            GemGrade::Chipped => 1_i32,
            GemGrade::Flawed => 2,
            GemGrade::Normal => 3,
            GemGrade::Flawless => 4,
            GemGrade::Perfect => 5,
        };
        match self.color {
            GemColor::Ruby => GemMods {
                weapon: vec![(StatKind::DamageMin, Modifier::Flat(5 * g))],
                armor: vec![(StatKind::Life, Modifier::Flat(10 * g))],
                shield: vec![(StatKind::FireResist, Modifier::Flat(10 * g))],
            },
            GemColor::Sapphire => GemMods {
                weapon: vec![(StatKind::DamageMin, Modifier::Flat(3 * g))],
                armor: vec![(StatKind::Mana, Modifier::Flat(10 * g))],
                shield: vec![(StatKind::ColdResist, Modifier::Flat(10 * g))],
            },
            GemColor::Topaz => GemMods {
                weapon: vec![(StatKind::AttackRating, Modifier::Flat(15 * g))],
                armor: vec![(StatKind::MagicFind, Modifier::Flat(5 * g))],
                shield: vec![(StatKind::LightningResist, Modifier::Flat(10 * g))],
            },
            GemColor::Emerald => GemMods {
                weapon: vec![(StatKind::DamageMax, Modifier::Flat(4 * g))],
                armor: vec![(StatKind::Dexterity, Modifier::Flat(5 * g))],
                shield: vec![(StatKind::PoisonResist, Modifier::Flat(10 * g))],
            },
            GemColor::Diamond => GemMods {
                weapon: vec![(StatKind::AttackRating, Modifier::Flat(25 * g))],
                armor: vec![(StatKind::Defense, Modifier::Flat(8 * g))],
                shield: vec![(StatKind::BlockChance, Modifier::Flat(3 * g))],
            },
            GemColor::Amethyst => GemMods {
                weapon: vec![(StatKind::AttackRating, Modifier::Flat(20 * g))],
                armor: vec![(StatKind::Strength, Modifier::Flat(4 * g))],
                shield: vec![(StatKind::Strength, Modifier::Flat(3 * g))],
            },
            GemColor::Skull => GemMods {
                weapon: vec![(StatKind::LifeOnHit, Modifier::Flat(3 * g))],
                armor: vec![(StatKind::Life, Modifier::Flat(8 * g))],
                shield: vec![(StatKind::Life, Modifier::Flat(5 * g))],
            },
        }
    }

    /// Mods applicable to the given item family.
    pub fn mods_for_family(&self, family: ItemFamily) -> Vec<(StatKind, Modifier)> {
        let all = self.mods();
        match family {
            ItemFamily::Weapon => all.weapon,
            ItemFamily::Armor => all.armor,
            _ => vec![],
        }
    }
}

// ─── Runes ───────────────────────────────────────────────────────────────────

/// All runes in sequence from lowest to highest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum RuneKind {
    El,
    Eld,
    Tir,
    Nef,
    Eth,
    Ith,
    Tal,
    Ral,
    Ort,
    Thul,
    Amn,
    Sol,
    Shael,
    Dol,
    Hel,
    Io,
    Lum,
    Ko,
    Fal,
    Lem,
    Pul,
    Um,
    Mal,
    Ist,
    Gul,
    Vex,
    Ohm,
    Lo,
    Sur,
    Ber,
    Jah,
    Cham,
    Zod,
}

impl RuneKind {
    /// Minimum character level to socket this rune.
    pub fn level_req(self) -> u32 {
        match self {
            Self::El | Self::Eld | Self::Tir | Self::Nef | Self::Eth | Self::Ith => 11,
            Self::Tal | Self::Ral | Self::Ort | Self::Thul => 13,
            Self::Amn | Self::Sol => 25,
            Self::Shael | Self::Dol => 29,
            Self::Hel | Self::Io => 33,
            Self::Lum | Self::Ko | Self::Fal | Self::Lem => 37,
            Self::Pul | Self::Um | Self::Mal | Self::Ist => 45,
            Self::Gul | Self::Vex | Self::Ohm | Self::Lo => 51,
            Self::Sur | Self::Ber | Self::Jah | Self::Cham | Self::Zod => 65,
        }
    }

    /// Stat mod when socketed in a weapon (None = no individual weapon effect).
    pub fn weapon_mod(self) -> Option<(StatKind, Modifier)> {
        match self {
            Self::El => Some((StatKind::AttackRating, Modifier::Flat(50))),
            Self::Ral => Some((StatKind::DamageMin, Modifier::Flat(5))),
            Self::Amn => Some((StatKind::LifeOnHit, Modifier::Flat(7))),
            Self::Sol => Some((StatKind::DamageMin, Modifier::Flat(9))),
            Self::Ber => Some((StatKind::CritChance, Modifier::Flat(20))),
            _ => None,
        }
    }

    /// Stat mod when socketed in armour.
    pub fn armor_mod(self) -> Option<(StatKind, Modifier)> {
        match self {
            Self::Eth => Some((StatKind::Defense, Modifier::Percentage(15))),
            Self::Io => Some((StatKind::Vitality, Modifier::Flat(10))),
            Self::Ko => Some((StatKind::Dexterity, Modifier::Flat(10))),
            Self::Lum => Some((StatKind::Energy, Modifier::Flat(10))),
            Self::Fal => Some((StatKind::Strength, Modifier::Flat(10))),
            _ => None,
        }
    }
}

// ─── Jewels ──────────────────────────────────────────────────────────────────

/// A jewel carries random magical properties and fits in sockets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jewel {
    pub uid: u64,
    pub mods: Vec<(StatKind, Modifier)>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perfect_ruby_weapon_mod_is_25_flat_damage() {
        let gem = Gem { color: GemColor::Ruby, grade: GemGrade::Perfect };
        let (stat, modifier) = gem.mods().weapon[0];
        assert_eq!(stat, StatKind::DamageMin);
        assert_eq!(modifier, Modifier::Flat(25));
    }

    #[test]
    fn chipped_ruby_weapon_mod_is_5_flat_damage() {
        let gem = Gem { color: GemColor::Ruby, grade: GemGrade::Chipped };
        let (_, modifier) = gem.mods().weapon[0];
        assert_eq!(modifier, Modifier::Flat(5));
    }

    #[test]
    fn zod_rune_has_highest_level_req() {
        assert_eq!(RuneKind::Zod.level_req(), 65);
        assert!(RuneKind::Zod.level_req() > RuneKind::El.level_req());
    }

    #[test]
    fn el_rune_adds_attack_rating_in_weapon() {
        let (stat, _) = RuneKind::El.weapon_mod().expect("El has weapon mod");
        assert_eq!(stat, StatKind::AttackRating);
    }
}
