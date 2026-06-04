//! The blacksmith's forge: recycle (salvage) gear into crafting materials.
//! Recycling an item destroys it and credits the player's material stock,
//! scaled by the item's level, rarity, and what it's made of.

use serde::{Deserialize, Serialize};

use mge_items::item::ItemKind;
use mge_items::rarity::Rarity;

use crate::components::item::ItemInstance;
use crate::ecs::{Entity, World};

/// One of the seven crafting materials a salvaged item can yield.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Material {
    Fer,
    Bronze,
    Acier,
    Argent,
    Or,
    Bois,
    Gemmes,
}

impl Material {
    /// Every material, in stock-display order. Index matches the discriminant.
    pub const ALL: [Material; 7] = [
        Material::Fer,
        Material::Bronze,
        Material::Acier,
        Material::Argent,
        Material::Or,
        Material::Bois,
        Material::Gemmes,
    ];

    /// French display label.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Material::Fer => "Fer",
            Material::Bronze => "Bronze",
            Material::Acier => "Acier",
            Material::Argent => "Argent",
            Material::Or => "Or",
            Material::Bois => "Bois",
            Material::Gemmes => "Gemmes",
        }
    }
}

/// The player's running stock of crafting materials.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Materials {
    /// Counts indexed by `Material as usize`.
    pub counts: [u32; 7],
}

impl Materials {
    #[must_use]
    pub fn get(&self, m: Material) -> u32 {
        self.counts[m as usize]
    }

    pub fn add(&mut self, m: Material, n: u32) {
        self.counts[m as usize] = self.counts[m as usize].saturating_add(n);
    }

    /// Deduct `n` of material `m` if the stock allows it. Returns whether it did.
    pub fn take(&mut self, m: Material, n: u32) -> bool {
        let slot = &mut self.counts[m as usize];
        if *slot < n {
            return false;
        }
        *slot -= n;
        true
    }

    #[must_use]
    pub fn total(&self) -> u32 {
        self.counts.iter().sum()
    }
}

/// A metal item drops `Fer` plus, with rarity, finer alloys.
fn metal_yield(amt: u32, rarity: Rarity) -> Vec<(Material, u32)> {
    let mut out = vec![(Material::Fer, amt)];
    if rarity >= Rarity::Magic {
        out.push((Material::Bronze, amt / 2 + 1));
    }
    if rarity >= Rarity::Rare {
        out.push((Material::Acier, 1 + rarity_bonus(rarity)));
    }
    if rarity >= Rarity::Set {
        out.push((Material::Argent, 1));
    }
    out
}

/// Extra units higher rarities grant on their premium material.
fn rarity_bonus(rarity: Rarity) -> u32 {
    match rarity {
        Rarity::Normal => 0,
        Rarity::Magic => 1,
        Rarity::Rare | Rarity::Crafted => 2,
        Rarity::Set => 3,
        Rarity::Unique => 4,
    }
}

/// The materials recycling `item` yields. Deterministic: metal gear → iron and
/// finer alloys, wooden weapons → wood + fittings, jewellery → silver/gold,
/// gems → gem shards. Quantities climb with item level and rarity.
#[must_use]
pub fn salvage_yield(item: &ItemInstance) -> Vec<(Material, u32)> {
    let kind = item.item.base.kind;
    let rarity = item.rarity();
    // Base unit count from item level, with a rarity kicker.
    let amt = 1 + item.ilvl() / 6 + rarity_bonus(rarity);

    match kind {
        // Wooden-hafted weapons: mostly wood, with metal fittings.
        ItemKind::Bow | ItemKind::Staff | ItemKind::Wand | ItemKind::Scepter => {
            let mut out = vec![(Material::Bois, amt), (Material::Fer, amt / 2 + 1)];
            if rarity >= Rarity::Rare {
                out.push((Material::Acier, 1));
            }
            out
        }
        // Metal weapons and armour.
        ItemKind::Sword
        | ItemKind::Axe
        | ItemKind::Mace
        | ItemKind::Dagger
        | ItemKind::Crossbow
        | ItemKind::Helm
        | ItemKind::BodyArmor
        | ItemKind::Belt
        | ItemKind::Gloves
        | ItemKind::Boots
        | ItemKind::Shield => metal_yield(amt, rarity),
        // Jewellery: precious metals.
        ItemKind::Ring | ItemKind::Amulet => {
            let mut out = vec![(Material::Argent, 1 + rarity_bonus(rarity))];
            if rarity >= Rarity::Rare {
                out.push((Material::Or, 1 + rarity_bonus(rarity) / 2));
            }
            out
        }
        // Socketables crumble back into gem shards — a typed gem yields by tier.
        ItemKind::Gem | ItemKind::Rune | ItemKind::Jewel => {
            let n = item.gem.map_or(1, |g| u32::from(g.tier) + 1);
            vec![(Material::Gemmes, n)]
        }
        // Everything else is scrap iron.
        _ => vec![(Material::Fer, amt.max(1))],
    }
}

/// Recycle the backpack item at `inv_index` into materials. The item is
/// destroyed and the yield credited to the player's stock. Returns success.
pub fn recycle(world: &mut World, player: Entity, inv_index: usize) -> bool {
    let Some(item) = world.inventory.get(&player).and_then(|inv| inv.get(inv_index).cloned()) else {
        return false;
    };
    let yield_ = salvage_yield(&item);
    if let Some(inv) = world.inventory.get_mut(&player) {
        inv.remove(inv_index);
        for (mat, n) in &yield_ {
            inv.materials.add(*mat, *n);
        }
    }
    let parts: Vec<String> = yield_.iter().map(|(m, n)| format!("{n} {}", m.label())).collect();
    world.log_event(format!("{} recyclé : {}.", item.name, parts.join(", ")));
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::item::{GemKind, ItemInstance, SocketedGem};
    use crate::components::player::ClassType;
    use crate::data::items::base_item;
    use crate::data::maps::village_map;
    use crate::data::GameData;
    use crate::plugins::player::spawn_player_system;
    use mge_items::affixes::ItemAffixes;
    use mge_items::item::Item;

    fn mk(kind: ItemKind, ilvl: u32, rarity: Rarity) -> ItemInstance {
        let item = Item::new(1, base_item(kind, ilvl), rarity, ilvl);
        ItemInstance::new(item, ItemAffixes::new(), Vec::new(), "Test".into())
    }

    #[test]
    fn metal_gear_yields_iron_finer_with_rarity() {
        let plain = salvage_yield(&mk(ItemKind::Sword, 10, Rarity::Normal));
        assert!(plain.iter().any(|(m, _)| *m == Material::Fer));
        assert!(!plain.iter().any(|(m, _)| *m == Material::Acier), "normal gear has no steel");

        let rare = salvage_yield(&mk(ItemKind::Sword, 10, Rarity::Rare));
        assert!(rare.iter().any(|(m, _)| *m == Material::Acier), "rare gear yields steel");
    }

    #[test]
    fn wooden_weapons_yield_wood() {
        let y = salvage_yield(&mk(ItemKind::Bow, 12, Rarity::Magic));
        assert!(y.iter().any(|(m, _)| *m == Material::Bois));
    }

    #[test]
    fn jewellery_yields_silver_and_gold() {
        let y = salvage_yield(&mk(ItemKind::Amulet, 20, Rarity::Rare));
        assert!(y.iter().any(|(m, _)| *m == Material::Argent));
        assert!(y.iter().any(|(m, _)| *m == Material::Or));
    }

    #[test]
    fn a_perfect_gem_yields_more_shards() {
        let mut gem = mk(ItemKind::Gem, 1, Rarity::Magic);
        gem.gem = Some(SocketedGem { kind: GemKind::Rubis, tier: 4 });
        let y = salvage_yield(&gem);
        assert_eq!(y, vec![(Material::Gemmes, 5)], "tier-4 gem → 5 shards");
    }

    #[test]
    fn recycling_consumes_the_item_and_credits_materials() {
        let mut world = World::new(village_map(), GameData::load(), 1);
        world.chosen_class = ClassType::Soldat;
        spawn_player_system(&mut world);
        let player = world.player_entity.unwrap();

        world.inventory.get_mut(&player).unwrap().add(mk(ItemKind::Sword, 10, Rarity::Rare)).unwrap();
        let before = world.inventory.get(&player).unwrap().materials.total();
        assert!(recycle(&mut world, player, 0));
        assert_eq!(world.inventory.get(&player).unwrap().count(), 0, "the item is destroyed");
        assert!(
            world.inventory.get(&player).unwrap().materials.total() > before,
            "materials were credited",
        );
    }
}
