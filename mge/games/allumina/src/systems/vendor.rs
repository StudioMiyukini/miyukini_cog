//! @id     games_allumina_systems_vendor
//! @do     price
//! @role   orchestration
//! @layer  core
//! @human  games allumina — module « systems vendor ».

//! Village NPCs: every non-combat villager (merchants, healer, blacksmith,
//! quest giver, recruiter) is spawned from the map's `npc_spawns` list.
//! Merchant kinds also get a buyable stock generated here.

use mge_items::item::ItemKind;
use mge_items::rarity::Rarity;

use crate::components::item::ItemInstance;
use crate::components::npc::{Merchant, NpcInfo, NpcKind};
use crate::components::world::NpcRole;
use crate::components::Position2D;
use crate::ecs::World;
use crate::systems::loot::generate_item;

/// Buy/sell value of an item (sell is half, floored at 1).
#[must_use]
pub fn price(item: &ItemInstance) -> u64 {
    let base = 10 + u64::from(item.ilvl()) * 5;
    let mult = match item.rarity() {
        Rarity::Normal => 1,
        Rarity::Magic => 3,
        Rarity::Rare => 8,
        Rarity::Crafted => 10,
        Rarity::Set => 15,
        Rarity::Unique => 25,
    };
    base * mult
}

/// Sell value the player receives.
#[must_use]
pub fn sell_value(item: &ItemInstance) -> u64 {
    (price(item) / 2).max(1)
}

/// Stock list for the legacy generic merchant.
const LEGACY_STOCK: [ItemKind; 6] = [
    ItemKind::Sword,
    ItemKind::BodyArmor,
    ItemKind::Helm,
    ItemKind::Boots,
    ItemKind::Ring,
    ItemKind::Wand,
];

/// Item kinds carried by each merchant role. Non-merchant roles return `&[]`.
fn stock_for(role: NpcRole) -> &'static [ItemKind] {
    match role {
        NpcRole::WeaponMerchant => &[
            ItemKind::Sword,
            ItemKind::Bow,
            ItemKind::Staff,
            ItemKind::Wand,
            ItemKind::Scepter,
        ],
        NpcRole::ArmorMerchant => &[
            ItemKind::Helm,
            ItemKind::BodyArmor,
            ItemKind::Boots,
            ItemKind::Gloves,
            ItemKind::Shield,
            ItemKind::Belt,
        ],
        // No `Potion` item kind exists yet — Lyra carries trinkets and a belt
        // as a stand-in until potions land.
        NpcRole::PotionMerchant => &[ItemKind::Ring, ItemKind::Amulet, ItemKind::Belt],
        _ => &[],
    }
}

/// Spawn every NPC declared by the current map, plus a legacy fallback
/// merchant when only the old `merchant_spawn` field is set.
pub fn spawn_npcs_system(world: &mut World) {
    if world.map.npc_spawns.is_empty() {
        spawn_legacy_merchant(world);
        return;
    }

    let ilvl = world.map.area_level.max(4);
    let spawns = world.map.npc_spawns.clone();
    for (i, spawn) in spawns.into_iter().enumerate() {
        let kinds = stock_for(spawn.kind);
        let mut stock = Vec::with_capacity(kinds.len());
        for (j, kind) in kinds.iter().copied().enumerate() {
            let uid = 900_000 + (i as u64) * 32 + j as u64;
            stock.push(generate_item(world, kind, ilvl, uid));
        }

        let e = world.spawn();
        world.positions.insert(e, Position2D::from_tile(spawn.x, spawn.y));
        world.npcs.insert(e, NpcKind::from_role(spawn.kind));
        world
            .npc_info
            .insert(e, NpcInfo { name: spawn.name.clone(), dialogue: spawn.dialogue.clone() });
        if !stock.is_empty() {
            world.merchants.insert(e, Merchant { stock });
        }
        world.log_event(format!("{} attends to villagers here.", spawn.name));
    }
}

/// Back-compat path for saves/tests that only declare `merchant_spawn`.
fn spawn_legacy_merchant(world: &mut World) {
    let Some((mx, my)) = world.map.merchant_spawn else { return };
    let ilvl = world.map.area_level.max(4);

    let mut stock = Vec::new();
    for (i, kind) in LEGACY_STOCK.into_iter().enumerate() {
        let uid = 900_000 + i as u64;
        stock.push(generate_item(world, kind, ilvl, uid));
    }

    let e = world.spawn();
    world.positions.insert(e, Position2D::from_tile(mx, my));
    world.npcs.insert(e, NpcKind::Merchant);
    world.merchants.insert(e, Merchant { stock });
    world.log_event("A merchant tends his wares here.".to_string());
}

/// Back-compat re-export: the old single-merchant spawner just dispatches to
/// the new NPC pipeline.
pub fn spawn_merchant_system(world: &mut World) {
    spawn_npcs_system(world);
}

/// The merchant nearest to `pos` within `radius` tiles.
#[must_use]
pub fn nearest_merchant(world: &World, pos: Position2D, radius: f32) -> Option<crate::ecs::Entity> {
    world
        .merchants
        .keys()
        .copied()
        .filter_map(|e| world.positions.get(&e).map(|p| (e, pos.distance(*p))))
        .filter(|(_, d)| *d <= radius)
        .min_by(|a, b| a.1.total_cmp(&b.1))
        .map(|(e, _)| e)
}

/// Buy stock item `idx` from `merchant` for `player`. Returns success.
pub fn buy(world: &mut World, player: crate::ecs::Entity, merchant: crate::ecs::Entity, idx: usize) -> bool {
    let Some(item) = world.merchants.get(&merchant).and_then(|m| m.stock.get(idx).cloned()) else {
        return false;
    };
    let cost = price(&item);
    let gold = world.inventory.get(&player).map_or(0, |i| i.gold);
    if gold < cost {
        world.log_event("Not enough gold.".to_string());
        return false;
    }
    let room = world.inventory.get(&player).is_some_and(|i| !i.is_full());
    if !room {
        world.log_event("Inventory full.".to_string());
        return false;
    }
    if let Some(inv) = world.inventory.get_mut(&player) {
        inv.gold -= cost;
        let _ = inv.add(item.clone());
    }
    if let Some(m) = world.merchants.get_mut(&merchant) {
        if idx < m.stock.len() {
            m.stock.remove(idx);
        }
    }
    world.log_event(format!("Bought {} for {cost} gold.", item.name));
    true
}

/// Sell inventory slot `idx` for `player`. Returns success.
pub fn sell(world: &mut World, player: crate::ecs::Entity, idx: usize) -> bool {
    let Some(item) = world.inventory.get(&player).and_then(|i| i.get(idx).cloned()) else {
        return false;
    };
    let value = sell_value(&item);
    if let Some(inv) = world.inventory.get_mut(&player) {
        inv.remove(idx);
        inv.add_gold(value);
    }
    world.log_event(format!("Sold {} for {value} gold.", item.name));
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::player::ClassType;
    use crate::data::maps::village_map;
    use crate::data::GameData;
    use crate::plugins::player::spawn_player_system;

    fn village_world() -> World {
        let mut world = World::new(village_map(), GameData::load(), 5);
        world.chosen_class = ClassType::Soldat;
        spawn_player_system(&mut world);
        spawn_merchant_system(&mut world);
        world
    }

    #[test]
    fn merchant_has_stock() {
        let world = village_world();
        // The village now declares three merchants (weapons, armor, potions);
        // every one of them must come pre-stocked.
        assert!(world.merchants.len() >= 3, "expected at least 3 merchants");
        for m in world.merchants.values() {
            assert!(!m.stock.is_empty(), "every merchant should ship with stock");
        }
    }

    #[test]
    fn buy_then_sell_roundtrips_gold_flow() {
        let mut world = village_world();
        let player = world.player_entity.unwrap();
        let merchant = *world.merchants.keys().next().unwrap();
        world.inventory.get_mut(&player).unwrap().add_gold(10_000);

        let before = world.inventory.get(&player).unwrap().gold;
        assert!(buy(&mut world, player, merchant, 0));
        let after_buy = world.inventory.get(&player).unwrap().gold;
        assert!(after_buy < before, "buying should cost gold");

        // Sell the item we just bought back.
        let slot = world
            .inventory
            .get(&player)
            .unwrap()
            .slots
            .iter()
            .position(Option::is_some)
            .unwrap();
        assert!(sell(&mut world, player, slot));
        assert!(world.inventory.get(&player).unwrap().gold > after_buy);
    }
}
