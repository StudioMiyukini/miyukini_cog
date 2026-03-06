use serde::{Deserialize, Serialize};

use mge_gameplay::stats::{Modifier, StatKind};

use crate::item::{ItemFamily, ItemKind};
use crate::socketables::RuneKind;

/// One stat modifier bonus provided by a runeword.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunewordMod {
    pub stat: StatKind,
    pub modifier: Modifier,
}

/// Static definition of a runeword.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunewordDef {
    pub id: u32,
    pub name: String,
    /// Exact rune sequence required (order matters).
    pub runes: Vec<RuneKind>,
    /// Item family the base item must belong to.
    pub allowed_family: ItemFamily,
    /// If non-empty, only these specific kinds are allowed within the family.
    pub allowed_kinds: Vec<ItemKind>,
    /// The base item must have exactly this many sockets.
    pub required_sockets: u8,
    pub mods: Vec<RunewordMod>,
}

/// Reason a runeword cannot be formed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunewordError {
    WrongFamily,
    WrongSocketCount,
    WrongRuneSequence,
    ItemNotNormal,
}

impl RunewordDef {
    /// Validate whether the given item and rune sequence can form this runeword.
    pub fn validate(
        &self,
        item_family: ItemFamily,
        item_kind: ItemKind,
        socket_count: u8,
        rune_sequence: &[RuneKind],
        is_normal_rarity: bool,
    ) -> Result<(), RunewordError> {
        if item_family != self.allowed_family {
            return Err(RunewordError::WrongFamily);
        }
        if !self.allowed_kinds.is_empty() && !self.allowed_kinds.contains(&item_kind) {
            return Err(RunewordError::WrongFamily);
        }
        if socket_count != self.required_sockets {
            return Err(RunewordError::WrongSocketCount);
        }
        if !is_normal_rarity {
            return Err(RunewordError::ItemNotNormal);
        }
        if rune_sequence != self.runes.as_slice() {
            return Err(RunewordError::WrongRuneSequence);
        }
        Ok(())
    }
}

/// MVP runeword list.
pub fn mvp_runewords() -> Vec<RunewordDef> {
    vec![
        RunewordDef {
            id: 1,
            name: "Steel".into(),
            runes: vec![RuneKind::Tir, RuneKind::El],
            allowed_family: ItemFamily::Weapon,
            allowed_kinds: vec![ItemKind::Sword, ItemKind::Axe, ItemKind::Mace],
            required_sockets: 2,
            mods: vec![
                RunewordMod { stat: StatKind::AttackSpeed, modifier: Modifier::Flat(25) },
                RunewordMod { stat: StatKind::DamageMin, modifier: Modifier::Flat(3) },
            ],
        },
        RunewordDef {
            id: 2,
            name: "Ancient's Pledge".into(),
            runes: vec![RuneKind::Ral, RuneKind::Ort, RuneKind::Tal],
            allowed_family: ItemFamily::Armor,
            allowed_kinds: vec![ItemKind::Shield],
            required_sockets: 3,
            mods: vec![
                RunewordMod { stat: StatKind::FireResist, modifier: Modifier::Flat(43) },
                RunewordMod { stat: StatKind::ColdResist, modifier: Modifier::Flat(48) },
                RunewordMod { stat: StatKind::LightningResist, modifier: Modifier::Flat(48) },
                RunewordMod { stat: StatKind::PoisonResist, modifier: Modifier::Flat(48) },
            ],
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steel_runeword_valid_on_sword() {
        let rws = mvp_runewords();
        let steel = &rws[0];
        let result = steel.validate(
            ItemFamily::Weapon,
            ItemKind::Sword,
            2,
            &[RuneKind::Tir, RuneKind::El],
            true,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn steel_wrong_rune_sequence_fails() {
        let rws = mvp_runewords();
        let steel = &rws[0];
        let result = steel.validate(
            ItemFamily::Weapon,
            ItemKind::Sword,
            2,
            &[RuneKind::El, RuneKind::Tir], // reversed
            true,
        );
        assert_eq!(result, Err(RunewordError::WrongRuneSequence));
    }

    #[test]
    fn steel_fails_on_armor_family() {
        let rws = mvp_runewords();
        let steel = &rws[0];
        let result = steel.validate(
            ItemFamily::Armor,
            ItemKind::Helm,
            2,
            &[RuneKind::Tir, RuneKind::El],
            true,
        );
        assert_eq!(result, Err(RunewordError::WrongFamily));
    }

    #[test]
    fn runeword_fails_on_non_normal_item() {
        let rws = mvp_runewords();
        let steel = &rws[0];
        let result = steel.validate(
            ItemFamily::Weapon,
            ItemKind::Sword,
            2,
            &[RuneKind::Tir, RuneKind::El],
            false,
        );
        assert_eq!(result, Err(RunewordError::ItemNotNormal));
    }
}
