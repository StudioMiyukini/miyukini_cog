use serde::{Deserialize, Serialize};

use mge_gameplay::stats::{Modifier, StatKind};

use crate::item::ItemKind;

/// A fixed stat modifier on a unique or set item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedMod {
    pub stat: StatKind,
    pub modifier: Modifier,
}

/// Static definition of a unique item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniqueDef {
    pub id: u32,
    pub name: String,
    /// Base item kind this unique is built on.
    pub base_kind: ItemKind,
    pub min_ilvl: u32,
    pub mods: Vec<FixedMod>,
}

/// One piece within a set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetPieceDef {
    pub id: u32,
    pub set_id: u32,
    pub name: String,
    pub base_kind: ItemKind,
    pub min_ilvl: u32,
    pub item_mods: Vec<FixedMod>,
}

/// A partial-set bonus triggered by wearing N pieces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetBonus {
    pub pieces_required: u8,
    pub mods: Vec<FixedMod>,
}

/// Full set definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDef {
    pub id: u32,
    pub name: String,
    pub pieces: Vec<SetPieceDef>,
    pub bonuses: Vec<SetBonus>,
}

impl SetDef {
    /// Active bonuses given the number of pieces currently worn.
    pub fn active_bonuses(&self, pieces_worn: u8) -> Vec<&SetBonus> {
        self.bonuses.iter().filter(|b| pieces_worn >= b.pieces_required).collect()
    }
}

/// MVP unique items.
pub fn mvp_uniques() -> Vec<UniqueDef> {
    vec![
        UniqueDef {
            id: 1,
            name: "Wirt's Leg".into(),
            base_kind: ItemKind::Mace,
            min_ilvl: 1,
            mods: vec![
                FixedMod { stat: StatKind::Strength, modifier: Modifier::Flat(5) },
                FixedMod { stat: StatKind::Life, modifier: Modifier::Flat(25) },
            ],
        },
        UniqueDef {
            id: 2,
            name: "Veil of Steel".into(),
            base_kind: ItemKind::Helm,
            min_ilvl: 15,
            mods: vec![
                FixedMod { stat: StatKind::Defense, modifier: Modifier::Flat(120) },
                FixedMod { stat: StatKind::FireResist, modifier: Modifier::Flat(20) },
                FixedMod { stat: StatKind::ColdResist, modifier: Modifier::Flat(20) },
                FixedMod { stat: StatKind::LightningResist, modifier: Modifier::Flat(20) },
            ],
        },
    ]
}

/// MVP set: Orphan's Call (3-piece).
pub fn mvp_sets() -> Vec<SetDef> {
    vec![SetDef {
        id: 1,
        name: "Orphan's Call".into(),
        pieces: vec![
            SetPieceDef {
                id: 10,
                set_id: 1,
                name: "Guillaume's Face".into(),
                base_kind: ItemKind::Helm,
                min_ilvl: 12,
                item_mods: vec![FixedMod { stat: StatKind::Defense, modifier: Modifier::Flat(80) }],
            },
            SetPieceDef {
                id: 11,
                set_id: 1,
                name: "Whitstan's Guard".into(),
                base_kind: ItemKind::Shield,
                min_ilvl: 12,
                item_mods: vec![FixedMod {
                    stat: StatKind::BlockChance,
                    modifier: Modifier::Flat(15),
                }],
            },
            SetPieceDef {
                id: 12,
                set_id: 1,
                name: "Hsaru's Iron Stay".into(),
                base_kind: ItemKind::Belt,
                min_ilvl: 1,
                item_mods: vec![FixedMod { stat: StatKind::Life, modifier: Modifier::Flat(30) }],
            },
        ],
        bonuses: vec![
            SetBonus {
                pieces_required: 2,
                mods: vec![FixedMod { stat: StatKind::AttackRating, modifier: Modifier::Flat(50) }],
            },
            SetBonus {
                pieces_required: 3,
                mods: vec![FixedMod { stat: StatKind::Life, modifier: Modifier::Flat(100) }],
            },
        ],
    }]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mvp_uniques_non_empty() {
        assert!(!mvp_uniques().is_empty());
    }

    #[test]
    fn set_bonus_activates_at_two_pieces() {
        let sets = mvp_sets();
        let set = &sets[0];
        let bonuses = set.active_bonuses(2);
        assert_eq!(bonuses.len(), 1);
        assert_eq!(bonuses[0].pieces_required, 2);
    }

    #[test]
    fn set_full_bonus_at_three_pieces() {
        let sets = mvp_sets();
        let bonuses = sets[0].active_bonuses(3);
        assert_eq!(bonuses.len(), 2);
    }

    #[test]
    fn set_no_bonus_at_one_piece() {
        let sets = mvp_sets();
        assert!(sets[0].active_bonuses(1).is_empty());
    }
}
