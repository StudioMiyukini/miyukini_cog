//! Static game data: class/skill/item/affix/mob/map definitions.

pub mod affixes;
pub mod classes;
pub mod items;
pub mod maps;
pub mod mercenary;
pub mod mobs;
pub mod skills;
pub mod summon;
pub mod uniques;

use mge_items::uniques::UniqueDef;

use crate::rng::Rng;
use affixes::AffixPool;
use mobs::MobDef;

/// How far on either side of `ilvl` a unique drop is allowed to roll.
const UNIQUE_BAND: u32 = 5;

/// Runtime registries loaded once at startup and stored as a world resource.
pub struct GameData {
    pub affix_pool: AffixPool,
    pub mobs: Vec<MobDef>,
    pub uniques: Vec<UniqueDef>,
}

impl GameData {
    pub fn load() -> Self {
        Self {
            affix_pool: affixes::affix_pool(),
            mobs: mobs::mob_table(),
            uniques: items::uniques(),
        }
    }

    pub fn mob(&self, def_id: u32) -> Option<&MobDef> {
        self.mobs.iter().find(|m| m.id == def_id)
    }

    /// Random unique of `kind` whose `level_req` is in `[ilvl-5, ilvl+5]`.
    /// Returns `None` if no named unique exists in that band — the caller is
    /// expected to downgrade the drop to a Rare in that case.
    pub fn pick_unique(
        &self,
        kind: mge_items::item::ItemKind,
        ilvl: u32,
        rng: &mut Rng,
    ) -> Option<&UniqueDef> {
        let lo = ilvl.saturating_sub(UNIQUE_BAND);
        let hi = ilvl.saturating_add(UNIQUE_BAND);
        let candidates: Vec<&UniqueDef> = self
            .uniques
            .iter()
            .filter(|u| u.base_kind == kind && u.min_ilvl >= lo && u.min_ilvl <= hi)
            .collect();
        let idx = rng.pick_index(candidates.len())?;
        Some(candidates[idx])
    }

    /// Lookup by stable id (used at save-load time to reconstitute mods).
    pub fn unique_by_id(&self, id: u32) -> Option<&UniqueDef> {
        self.uniques.iter().find(|u| u.id == id)
    }
}
