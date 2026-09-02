//! @id     games_allumina_data_items
//! @do     kind_id
//! @role   donnee
//! @layer  core
//! @human  games allumina — module « data items ».

//! Base item templates and unique definitions.

use mge_items::item::{BaseItemDef, EquipRequirements, ItemKind};
use mge_items::uniques::UniqueDef;

use crate::data::uniques::uniques_table;

/// Stable template id for an item kind.
fn kind_id(kind: ItemKind) -> u32 {
    match kind {
        ItemKind::Sword => 1,
        ItemKind::Axe => 2,
        ItemKind::Mace => 3,
        ItemKind::Dagger => 4,
        ItemKind::Bow => 5,
        ItemKind::Crossbow => 6,
        ItemKind::Staff => 7,
        ItemKind::Wand => 8,
        ItemKind::Scepter => 9,
        ItemKind::Helm => 10,
        ItemKind::BodyArmor => 11,
        ItemKind::Belt => 12,
        ItemKind::Gloves => 13,
        ItemKind::Boots => 14,
        ItemKind::Shield => 15,
        ItemKind::Ring => 16,
        ItemKind::Amulet => 17,
        ItemKind::Gem => 18,
        ItemKind::Rune => 19,
        ItemKind::Jewel => 20,
        ItemKind::SmallCharm => 21,
        ItemKind::LargeCharm => 22,
        ItemKind::GrandCharm => 23,
        ItemKind::Gold => 24,
        ItemKind::Key => 25,
        ItemKind::ScrollOfIdentify => 26,
        ItemKind::TomeOfIdentify => 27,
        ItemKind::TownPortalScroll => 28,
    }
}

fn kind_name(kind: ItemKind) -> &'static str {
    match kind {
        ItemKind::Sword => "Short Sword",
        ItemKind::Axe => "Hatchet",
        ItemKind::Mace => "Club",
        ItemKind::Dagger => "Dirk",
        ItemKind::Bow => "Short Bow",
        ItemKind::Crossbow => "Light Crossbow",
        ItemKind::Staff => "Quarterstaff",
        ItemKind::Wand => "Bone Wand",
        ItemKind::Scepter => "Scepter",
        ItemKind::Helm => "Cap",
        ItemKind::BodyArmor => "Quilted Armor",
        ItemKind::Belt => "Sash",
        ItemKind::Gloves => "Leather Gloves",
        ItemKind::Boots => "Leather Boots",
        ItemKind::Shield => "Buckler",
        ItemKind::Ring => "Ring",
        ItemKind::Amulet => "Amulet",
        ItemKind::Gem => "Gem",
        ItemKind::Rune => "Rune",
        ItemKind::Jewel => "Jewel",
        ItemKind::SmallCharm => "Small Charm",
        ItemKind::LargeCharm => "Large Charm",
        ItemKind::GrandCharm => "Grand Charm",
        ItemKind::Gold => "Gold",
        ItemKind::Key => "Key",
        ItemKind::ScrollOfIdentify => "Scroll of Identify",
        ItemKind::TomeOfIdentify => "Tome of Identify",
        ItemKind::TownPortalScroll => "Scroll of Town Portal",
    }
}

fn is_two_handed(kind: ItemKind) -> bool {
    matches!(kind, ItemKind::Bow | ItemKind::Crossbow | ItemKind::Staff)
}

fn max_sockets(kind: ItemKind) -> u8 {
    use mge_items::item::ItemFamily;
    match kind.family() {
        ItemFamily::Weapon | ItemFamily::Armor => 3,
        _ => 0,
    }
}

/// INT gating for caster gear (staves, wands, scepters). Other kinds get 0.
fn caster_int_requirement(kind: ItemKind, ilvl: u32) -> u32 {
    matches!(kind, ItemKind::Staff | ItemKind::Wand | ItemKind::Scepter)
        .then(|| 5 + ilvl * 2)
        .unwrap_or(0)
}

/// Build a base template for a kind scaled to an item level.
pub fn base_item(kind: ItemKind, ilvl: u32) -> BaseItemDef {
    let min_level = ilvl.saturating_sub(2);
    // Default requirements scale with ilvl; armour leans on STR/DEX, casters
    // pieces pile on INT instead (see kind matching below).
    let requirements = EquipRequirements {
        min_strength: 5 + ilvl * 2,
        min_dexterity: 5 + ilvl,
        min_intelligence: caster_int_requirement(kind, ilvl),
        min_level,
    };
    BaseItemDef {
        id: kind_id(kind),
        name: kind_name(kind).to_owned(),
        kind,
        max_sockets: max_sockets(kind),
        requirements,
        ilvl,
        two_handed: is_two_handed(kind),
    }
}

/// Unique item definitions — 425 named uniques (5 per (item type × 20-level
/// tier)) plus the original MVP entries kept for backwards compatibility with
/// older saves that reference `Wirt's Leg` / `Veil of Steel` by id.
pub fn uniques() -> Vec<UniqueDef> {
    let mut all = mge_items::uniques::mvp_uniques();
    all.extend(uniques_table());
    all
}
