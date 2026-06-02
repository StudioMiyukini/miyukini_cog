//! Per-class flavour and starting loadout.

use mge_items::item::ItemKind;

use crate::components::player::ClassType;

/// Static class metadata layered on top of the `mge-gameplay` base stats.
#[derive(Debug, Clone, Copy)]
pub struct ClassDef {
    pub class: ClassType,
    pub blurb: &'static str,
    /// Weapon the class starts equipped with.
    pub starting_weapon: ItemKind,
    /// Primary attribute that scales this class's attacks.
    pub primary: PrimaryStat,
}

/// Which attribute drives a class's damage scaling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryStat {
    Strength,
    Dexterity,
    Energy,
}

pub fn class_def(class: ClassType) -> ClassDef {
    match class {
        ClassType::Soldat => ClassDef {
            class,
            blurb: "Un meneur en première ligne qui encaisse les coups et rallie ses troupes.",
            starting_weapon: ItemKind::Sword,
            primary: PrimaryStat::Strength,
        },
        ClassType::Clerc => ClassDef {
            class,
            blurb: "Un soutien sacré qui soigne ses alliés et châtie le mal à distance.",
            starting_weapon: ItemKind::Scepter,
            primary: PrimaryStat::Energy,
        },
        ClassType::MageDeGuerre => ClassDef {
            class,
            blurb: "Un canalisateur élémentaire maniant feu, glace et foudre.",
            starting_weapon: ItemKind::Staff,
            primary: PrimaryStat::Energy,
        },
        ClassType::Archer => ClassDef {
            class,
            blurb: "Un tireur agile qui harcèle et démantèle ses cibles de loin.",
            starting_weapon: ItemKind::Bow,
            primary: PrimaryStat::Dexterity,
        },
    }
}
