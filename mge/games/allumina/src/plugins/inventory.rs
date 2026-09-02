//! @id     games_allumina_plugins_inventory
//! @do     ground_item_decay_system
//! @role   orchestration
//! @layer  core
//! @human  games allumina — module « plugins inventory ».

//! Inventory & equipment operations plus ground-item lifetime management.

use mge_items::affixes::ItemAffixes;
use mge_items::item::{EquipSlot, Item, ItemKind};
use mge_items::rarity::Rarity;

use crate::app::{App, PhaseId, Plugin};
use crate::components::item::{is_one_handed_melee, ItemInstance, SocketedGem};
use crate::data::items::base_item;
use crate::ecs::{Entity, World};
use crate::systems::mercenary::recompute_merc_stats;
use crate::systems::stats::{meets_requirements, recompute_player_stats};

/// Decrement ground-item lifetimes and despawn expired drops.
pub fn ground_item_decay_system(world: &mut World) {
    for e in world.ground_item_entities() {
        let expired = match world.ground_items.get_mut(&e) {
            Some(g) => {
                if g.life > 0 {
                    g.life -= 1;
                }
                g.life == 0
            }
            None => false,
        };
        if expired {
            world.despawn(e);
        }
    }
}

/// Equip the item at `inv_index`, swapping any previously worn piece back into
/// the backpack and updating the wearer's stat modifiers.
///
/// Refuses (returns `false`, leaves the inventory untouched) if the wearer's
/// effective attributes — base + any flat gear modifiers already worn —
/// don't meet the item's [`EquipRequirements`].
/// Resolve which slot a piece actually goes in. Rings prefer the first empty
/// finger (Ring1 then Ring2) so a second ring no longer overwrites the first;
/// everything else takes its natural slot.
fn free_ring_slot(world: &World, player: Entity, natural: EquipSlot) -> EquipSlot {
    if natural != EquipSlot::Ring1 {
        return natural;
    }
    let occupied = |s| world.equipment.get(&player).is_some_and(|eq| eq.get(s).is_some());
    if !occupied(EquipSlot::Ring1) {
        EquipSlot::Ring1
    } else if !occupied(EquipSlot::Ring2) {
        EquipSlot::Ring2
    } else {
        EquipSlot::Ring1
    }
}

pub fn equip_from_inventory(world: &mut World, player: Entity, inv_index: usize) -> bool {
    let Some(instance) = world.inventory.get(&player).and_then(|inv| inv.get(inv_index).cloned())
    else {
        return false;
    };
    let Some(natural) = instance.slot() else { return false };
    // Rings: fill the first free finger so a second ring goes to Ring2 instead
    // of overwriting Ring1.
    let slot = free_ring_slot(world, player, natural);

    // Gate the equip on STR/DEX/INT/level requirements, using *effective*
    // attributes so already-worn +STR gear can help meet the threshold. The
    // item's own Zirconia gems ease its requirements first.
    if !meets_requirements(world, player, &reduced_requirements(&instance)) {
        world.log_event(format!("Stats insuffisantes pour {}.", instance.name));
        return false;
    }

    world.inventory.get_mut(&player).and_then(|inv| inv.remove(inv_index));

    let old = world.equipment.get_mut(&player).and_then(|eq| eq.equip_to(slot, instance.clone()));

    recompute_player_stats(world, player);

    if let Some(old) = old {
        let _ = world.inventory.get_mut(&player).map(|inv| inv.add(old));
    }
    world.log_event(format!("Equipped {} ({:?}).", instance.name, slot));
    true
}

/// Equip the backpack item at `inv_index` into an explicit `target` slot,
/// regardless of its natural slot. Used to dual-wield a one-handed melee weapon
/// into the off-hand. Still honours the item's STR/DEX/level requirements, and
/// returns whatever was in `target` to the backpack.
pub fn equip_from_inventory_to(
    world: &mut World,
    player: Entity,
    inv_index: usize,
    target: mge_items::item::EquipSlot,
) -> bool {
    let Some(instance) = world.inventory.get(&player).and_then(|inv| inv.get(inv_index).cloned())
    else {
        return false;
    };
    if !meets_requirements(world, player, &reduced_requirements(&instance)) {
        world.log_event(format!("Stats insuffisantes pour {}.", instance.name));
        return false;
    }

    world.inventory.get_mut(&player).and_then(|inv| inv.remove(inv_index));
    let old = world.equipment.get_mut(&player).map(|eq| eq.equip_to(target, instance.clone())).flatten();
    recompute_player_stats(world, player);
    if let Some(old) = old {
        let _ = world.inventory.get_mut(&player).map(|inv| inv.add(old));
    }
    world.log_event(format!("Equipped {} ({:?}).", instance.name, target));
    true
}

/// Hand the backpack item at `inv_index` to `merc`, equipping it into the right
/// slot of its `merc_inventory`. A one-handed melee weapon dropped onto a merc
/// that already wields one goes to the off-hand (dual-wield); anything else
/// takes its natural slot. Whatever the merc was wearing returns to the pack.
/// Merc gear ignores attribute requirements (companions aren't stat-gated).
pub fn equip_inventory_item_to_merc(
    world: &mut World,
    player: Entity,
    inv_index: usize,
    merc: Entity,
) -> bool {
    let Some(instance) = world.inventory.get(&player).and_then(|inv| inv.get(inv_index).cloned())
    else {
        return false;
    };
    let Some(natural) = instance.slot() else { return false };

    // Dual-wield routing: a one-handed melee weapon goes to the off-hand when the
    // merc's main hand already holds a one-handed melee weapon.
    let main_one_h = world
        .merc_inventory
        .get(&merc)
        .and_then(|inv| inv.get(EquipSlot::MainHand))
        .is_some_and(|w| is_one_handed_melee(w.item.base.kind));
    let target = if is_one_handed_melee(instance.item.base.kind) && main_one_h {
        EquipSlot::OffHand
    } else {
        natural
    };

    world.inventory.get_mut(&player).and_then(|inv| inv.remove(inv_index));
    let old = world
        .merc_inventory
        .get_mut(&merc)
        .and_then(|inv| inv.equip_to(target, instance.clone()));
    recompute_merc_stats(world, merc);
    if let Some(old) = old {
        let _ = world.inventory.get_mut(&player).map(|inv| inv.add(old));
    }
    world.log_event(format!("Gave {} to the mercenary ({:?}).", instance.name, target));
    true
}

/// Take an item off a mercenary's `slot` back into the player's backpack.
pub fn unequip_merc_slot(world: &mut World, player: Entity, merc: Entity, slot: EquipSlot) -> bool {
    let Some(item) = world.merc_inventory.get_mut(&merc).and_then(|inv| inv.unequip(slot)) else {
        return false;
    };
    let name = item.name.clone();
    match world.inventory.get_mut(&player).map(|inv| inv.add(item)) {
        Some(Ok(_)) => {
            recompute_merc_stats(world, merc);
            world.log_event(format!("Took {name} back from the mercenary."));
            true
        }
        // No room (or no inventory): put it back on the merc so nothing is lost.
        Some(Err(item)) => {
            world.merc_inventory.get_mut(&merc).map(|inv| inv.equip_to(slot, item));
            world.log_event("No room to unequip the mercenary!");
            false
        }
        None => false,
    }
}

/// Socket the gem at `gem_index` into a worn (or, failing that, a backpack)
/// item that has a free socket: the gem's mods move into the item's `socketed`
/// list and the gem is consumed. Returns whether it found a slot.
pub fn socket_gem_from_inventory(world: &mut World, player: Entity, gem_index: usize) -> bool {
    let gem = match world.inventory.get(&player).and_then(|inv| inv.get(gem_index)) {
        Some(g) if g.is_gem() && g.gem.is_some() => g.clone(),
        _ => return false,
    };
    // Prefer a worn item with an open socket, else a backpack item.
    let inserted = world
        .equipment
        .get_mut(&player)
        .and_then(|eq| eq.slots.values_mut().find(|i| i.open_sockets() > 0))
        .map(|item| item.socket(&gem))
        .or_else(|| {
            world
                .inventory
                .get_mut(&player)
                .and_then(|inv| inv.slots.iter_mut().flatten().find(|i| i.open_sockets() > 0))
                .map(|item| item.socket(&gem))
        })
        .is_some();
    if !inserted {
        world.log_event("Aucun emplacement de gemme libre.");
        return false;
    }
    world.inventory.get_mut(&player).and_then(|inv| inv.remove(gem_index));
    recompute_player_stats(world, player);
    world.log_event(format!("{} sertie.", gem.name));
    true
}

/// Socket the gem at `gem_index` into the *specific* worn item in `slot` (drag
/// targeting). Returns false if it's not a gem or the item has no free socket.
pub fn socket_gem_into_equipped(world: &mut World, player: Entity, gem_index: usize, slot: EquipSlot) -> bool {
    let gem = match world.inventory.get(&player).and_then(|inv| inv.get(gem_index)) {
        Some(g) if g.is_gem() && g.gem.is_some() => g.clone(),
        _ => return false,
    };
    let ok = world
        .equipment
        .get_mut(&player)
        .and_then(|eq| eq.slots.get_mut(&slot))
        .is_some_and(|item| {
            if item.open_sockets() > 0 {
                item.socket(&gem);
                true
            } else {
                false
            }
        });
    if !ok {
        return false;
    }
    world.inventory.get_mut(&player).and_then(|inv| inv.remove(gem_index));
    recompute_player_stats(world, player);
    world.log_event(format!("{} sertie.", gem.name));
    true
}

/// Socket the gem at `gem_index` into the *specific* backpack item at
/// `target_index` (drag targeting). Returns false if it can't (caller may then
/// fall back to a normal move).
pub fn socket_gem_into_inventory_item(world: &mut World, player: Entity, gem_index: usize, target_index: usize) -> bool {
    if gem_index == target_index {
        return false;
    }
    let gem = match world.inventory.get(&player).and_then(|inv| inv.get(gem_index)) {
        Some(g) if g.is_gem() && g.gem.is_some() => g.clone(),
        _ => return false,
    };
    let ok = world.inventory.get_mut(&player).is_some_and(|inv| {
        match inv.slots.get_mut(target_index).and_then(|s| s.as_mut()) {
            Some(item) if item.open_sockets() > 0 => {
                item.socket(&gem);
                true
            }
            _ => false,
        }
    });
    if !ok {
        return false;
    }
    world.inventory.get_mut(&player).and_then(|inv| inv.remove(gem_index));
    recompute_player_stats(world, player);
    world.log_event(format!("{} sertie.", gem.name));
    true
}

/// Gold cost to pry a gem out of an item (scales with its item level).
#[must_use]
pub fn unsocket_cost(item: &crate::components::item::ItemInstance) -> u64 {
    200 + u64::from(item.ilvl()) * 40
}

/// Remove the last gem from a worn (`slot`) item, refunding it to the backpack
/// for a gold fee. Returns whether it happened.
pub fn unsocket_equipped(world: &mut World, player: Entity, slot: mge_items::item::EquipSlot) -> bool {
    let cost = match world.equipment.get(&player).and_then(|eq| eq.get(slot)) {
        Some(i) if !i.socketed.is_empty() => unsocket_cost(i),
        _ => return false,
    };
    unsocket_apply(world, player, cost, |w, p| {
        w.equipment.get_mut(&p).and_then(|eq| eq.slots.get_mut(&slot)).and_then(ItemInstance::unsocket)
    })
}

/// Same, for a backpack item at `inv_index`.
pub fn unsocket_in_inventory(world: &mut World, player: Entity, inv_index: usize) -> bool {
    let cost = match world.inventory.get(&player).and_then(|inv| inv.get(inv_index)) {
        Some(i) if !i.socketed.is_empty() => unsocket_cost(i),
        _ => return false,
    };
    unsocket_apply(world, player, cost, move |w, p| {
        w.inventory
            .get_mut(&p)
            .and_then(|inv| inv.slots.get_mut(inv_index).and_then(|s| s.as_mut()))
            .and_then(ItemInstance::unsocket)
    })
}

/// The item's equip requirements after its own Zirconia gems ease them.
fn reduced_requirements(instance: &ItemInstance) -> mge_items::item::EquipRequirements {
    let mut req = instance.item.base.requirements.clone();
    let cut = u64::from(instance.req_reduction_pct());
    if cut > 0 {
        let scale = |v: u32| ((u64::from(v) * (100 - cut)) / 100) as u32;
        req.min_strength = scale(req.min_strength);
        req.min_dexterity = scale(req.min_dexterity);
        req.min_intelligence = scale(req.min_intelligence);
        req.min_level = scale(req.min_level);
    }
    req
}

/// Rebuild a droppable gem item from a removed [`SocketedGem`].
fn gem_item_from(world: &mut World, gem: SocketedGem) -> ItemInstance {
    let uid = world.spawn().0;
    let item = Item::new(uid, base_item(ItemKind::Gem, 1), Rarity::Magic, 1);
    let mut instance = ItemInstance::new(item, ItemAffixes::new(), Vec::new(), gem.display_name());
    instance.gem = Some(gem);
    instance
}

/// Shared unsocket flow: charge `cost`, pop a gem via `pop`, rebuild it into the
/// backpack, and refresh stats.
fn unsocket_apply(
    world: &mut World,
    player: Entity,
    cost: u64,
    pop: impl FnOnce(&mut World, Entity) -> Option<SocketedGem>,
) -> bool {
    let gold = world.inventory.get(&player).map_or(0, |i| i.gold);
    if gold < cost {
        world.log_event(format!("Il faut {cost} or pour extraire la gemme."));
        return false;
    }
    let Some(socketed) = pop(world, player) else { return false };
    if let Some(inv) = world.inventory.get_mut(&player) {
        inv.gold -= cost;
    }
    let gem = gem_item_from(world, socketed);
    let name = gem.name.clone();
    let returned = world.inventory.get_mut(&player).map_or(false, |inv| inv.add(gem).is_ok());
    recompute_player_stats(world, player);
    if returned {
        world.log_event(format!("{name} extraite ({cost} or)."));
    } else {
        world.log_event("Sac plein : gemme extraite perdue.");
    }
    true
}

/// Unequip a slot back into the backpack.
pub fn unequip_to_inventory(
    world: &mut World,
    player: Entity,
    slot: mge_items::item::EquipSlot,
) -> bool {
    let Some(item) = world.equipment.get_mut(&player).and_then(|eq| eq.unequip(slot)) else {
        return false;
    };
    let name = item.name.clone();
    let returned = world.inventory.get_mut(&player).map_or(false, |inv| inv.add(item).is_ok());
    // Either way the slot is now empty, so the stat sheet has to be
    // resynced. (Pre-existing behaviour: a full backpack drops the item
    // on the floor; we honour that, but the worn-piece stats still go.)
    recompute_player_stats(world, player);
    if returned {
        world.log_event(format!("Unequipped {name}."));
        true
    } else {
        world.log_event("No room to unequip!");
        false
    }
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(PhaseId::Loot, ground_item_decay_system);
    }
}
